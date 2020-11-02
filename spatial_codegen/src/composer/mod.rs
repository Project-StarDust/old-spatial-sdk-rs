use crate::SchemaAST;
use crate::Type;
use crate::{Component, ComponentMember};
use std::path::{Path, PathBuf};
use std::{fs::File, io::Write};

enum SchemaNode {
    Root(Vec<Box<SchemaNode>>),
    PackageNode(String, Vec<Box<SchemaNode>>),
    ComponentNode(Component),
}

fn create_schema_graph(ast: SchemaAST) -> SchemaNode {
    let components = ast
        .components
        .into_iter()
        .map(SchemaNode::ComponentNode)
        .map(Box::new)
        .collect::<Vec<Box<SchemaNode>>>();
    let mut package_components = ast.package_name.0.into_iter().map(|p| p.0).rev();
    let first_item = package_components
        .next()
        .expect("Component doesn't have a package");
    let node = package_components.fold(
        SchemaNode::PackageNode(first_item, components),
        |acc, val| SchemaNode::PackageNode(val, vec![Box::new(acc)]),
    );
    SchemaNode::Root(vec![Box::new(node)])
}

fn get_id(component: &Component) -> Option<u32> {
    match component.body.iter().find(|e| match e {
        ComponentMember::ID(_) => true,
        _ => false,
    })? {
        ComponentMember::ID(id) => Some(*id),
        _ => None,
    }
}

fn generate_component(component: Component) -> Option<String> {
    let id = get_id(&component)?;
    let mut component_str = format!("use spatial_macro::component;\n\n");
    let mut members = component
        .body
        .into_iter()
        .filter_map(|mb| match mb {
            ComponentMember::Property(mtype, name, id) => Some((mtype, name, id)),
            _ => None,
        })
        .collect::<Vec<(Type, String, u32)>>();
    members.sort_by(|a, b| a.2.cmp(&b.2));
    component_str += &format!("#[component({})]\n", id);
    component_str += &format!("pub struct {} {{\n", component.name.camel_case());
    members.iter().for_each(|m| {
        component_str += &format!("    {}: {},\n", m.1, m.0.rust_type());
    });
    component_str += &format!("}}\n\n");
    Some(component_str)
}

fn write_component<P: AsRef<Path>>(path: P, component: Component) -> Result<(), std::io::Error> {
    if let Some(parent_path) = path.as_ref().parent() {
        std::fs::create_dir_all(parent_path)?;
    };
    let mut file = File::create(path)?;
    let component = generate_component(component).unwrap();
    file.write_all(component.as_bytes())
}

fn write_module_file<P: AsRef<Path>>(path: P, nodes: &Vec<Box<SchemaNode>>) -> Result<(), std::io::Error> {
    let new_path = path.as_ref().join("mod.rs");
    let mut file = File::create(new_path)?;
    for n in nodes {
        match n.as_ref() {
            SchemaNode::PackageNode(name, _) => {
                write!(file, "pub mod {};\n", name)?;
            },
            SchemaNode::ComponentNode(c) => {
                write!(file, "pub mod {};\n", c.name.snake_case())?;
                write!(file, "pub use {}::{};\n", c.name.snake_case(), c.name.camel_case())?;
            }
            _ => {}
        }
    };
    Ok(())
}

fn process_schema_node<P: AsRef<Path> + Clone>(path: P, node: SchemaNode) -> Result<(), std::io::Error> {
    Ok(match node {
        SchemaNode::Root(nodes) => {
            std::fs::create_dir_all(path.clone())?;
            write_module_file(path.clone(), &nodes)?;
            for node in nodes {
                process_schema_node(path.clone(), *node)?;
            }
        }
        SchemaNode::ComponentNode(component) => {
            let new_path = path.as_ref().join(component.name.snake_case() + ".rs");
            write_component(new_path, component)?;
        }
        SchemaNode::PackageNode(name, nodes) => {
            let new_path = path.as_ref().join(name);
            std::fs::create_dir_all(new_path.clone())?;
            write_module_file(new_path.clone(), &nodes)?;
            for node in nodes {
                process_schema_node(new_path.clone(), *node)?;
            }
        }
    })
}

pub fn generate_code<P: AsRef<Path> + Clone>(
    path: P,
    ast: SchemaAST,
) -> Result<(), std::io::Error> {
    let path_clone = path.clone();
    if path_clone.as_ref().exists() {
        std::fs::remove_dir_all(path)?;
    }

    let schema_graph = create_schema_graph(ast);
    process_schema_node(path_clone, schema_graph)
}
