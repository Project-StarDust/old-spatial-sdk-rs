use crate::ast::ASTNode;
use crate::ast::Component;
use crate::ast::Enum;
use crate::ast::SchemaFile;
use crate::ast::Type;
use crate::ast::AST;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_base_uses() -> String {
    format!(
        "{}\n{}\n{}\n{}\n",
        "#[allow(unused_imports)]",
        "use spatial_macro::spatial_enum;",
        "use spatial_macro::spatial_type;",
        "use spatial_macro::spatial_component;"
    )
}

fn generate_type(r#type: Type) -> String {
    format!(
        "{}\nstruct {} {{{}}}",
        "#[spatial_type]",
        r#type.name,
        if r#type.members.len() > 0 {
            let members = r#type
                .members
                .into_iter()
                .map(|m| format!("    {}: {}", m.name, m.m_type.rust_type()))
                .fold(String::new(), |acc, val| {
                    if acc.len() > 0 {
                        acc + ",\n" + &val
                    } else {
                        val
                    }
                });
            "\n".to_string() + &members + "\n"
        } else {
            "".to_string()
        }
    )
}

fn generate_enum(r#enum: Enum) -> String {
    format!(
        "{}\nenum {} {{{}}}",
        "#[spatial_enum]",
        r#enum.name,
        if r#enum.values.len() > 0 {
            let values = r#enum
                .values
                .into_iter()
                .map(|v| format!("    {}", v.name))
                .fold(String::new(), |acc, val| {
                    if acc.len() > 0 {
                        acc + ",\n" + &val
                    } else {
                        val
                    }
                });
            "\n".to_string() + &values + "\n"
        } else {
            "".to_string()
        }
    )
}

fn generate_component(component: Component) -> String {
    format!(
        "{}\nstruct {} {{{}}}",
        format!("#[spatial_component({})]", component.id),
        component.name,
        if component.members.len() > 0 {
            let members = component
                .members
                .into_iter()
                .map(|m| format!("    {}: {}", m.name, m.m_type.rust_type()))
                .fold(String::new(), |acc, val| {
                    if acc.len() > 0 {
                        acc + ",\n" + &val
                    } else {
                        val
                    }
                });
            "\n".to_string() + &members + "\n"
        } else {
            "".to_string()
        }
    )
}

fn write_to_schema_file<P: AsRef<Path> + Clone>(
    path: P,
    schema: SchemaFile,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path.clone()).map(|_| {
        let mut file = File::create(path.clone().as_ref().join(schema.name + ".rs"))?;
        write!(file, "{}\n", generate_base_uses())?;
        write!(
            file,
            "{}\n",
            schema
                .enums
                .into_iter()
                .map(generate_enum)
                .fold(String::new(), |acc, val| acc + "\n\n" + &val)
        )?;
        write!(
            file,
            "{}\n",
            schema
                .types
                .into_iter()
                .map(generate_type)
                .fold(String::new(), |acc, val| acc + "\n\n" + &val)
        )?;
        write!(
            file,
            "{}\n",
            schema
                .components
                .into_iter()
                .map(generate_component)
                .fold(String::new(), |acc, val| acc + "\n\n" + &val)
        )?;
        Ok(())
    })?
}

fn generate_schema<P: AsRef<Path> + Clone>(
    path: P,
    schema: SchemaFile,
) -> Result<(String, Vec<String>), std::io::Error> {
    let schema_name = schema.name.clone();
    let mut names = schema
        .types
        .iter()
        .map(|t| t.name.clone())
        .collect::<Vec<_>>();
    names.extend(schema.enums.iter().map(|e| e.name.clone()));
    names.extend(schema.components.iter().map(|c| c.name.clone()));
    write_to_schema_file(path, schema)?;
    Ok((schema_name, names))
}

fn generate_node<P: AsRef<Path> + Clone>(
    path: P,
    node: ASTNode,
) -> Result<(String, Vec<String>), std::io::Error> {
    let path_clone = path.clone();
    let module = match node {
        ASTNode::SchemaNode(schema) => generate_schema(path_clone, schema)?,
        ASTNode::PackageNode(pn) => {
            let name = pn.name.clone();
            let modules = pn
                .inner
                .into_iter()
                .map(|node| generate_node(path_clone.as_ref().join(&name), *node))
                .map(|res| match res {
                    Ok(d) => Ok(d),
                    Err(e) => {
                        eprint!("{}", e);
                        Err(())
                    }
                })
                .filter_map(Result::ok)
                .collect::<Vec<(String, Vec<String>)>>();
            generate_mod_rs_file(path_clone.as_ref().join(&name), modules)?;
            (name, Vec::new())
        }
    };
    Ok(module)
}

fn generate_mod_rs_file<P: AsRef<Path> + Clone>(
    path: P,
    modules: Vec<(String, Vec<String>)>,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path.clone()).map(|_| {
        let mut file = File::create(path.clone().as_ref().join("mod.rs"))?;
        for module in modules {
            write!(
                file,
                "{}mod {};\n",
                if module.1.len() > 0 { "" } else { "pub " },
                module.0
            )?;
            for usage in module.1 {
                write!(file, "pub use {}::{};\n", module.0, usage)?;
            }
        }
        Ok(())
    })?
}

pub fn generate_code<P: AsRef<Path> + Clone>(path: P, ast: AST) -> Result<(), std::io::Error> {
    let path_clone = path.clone();
    if path_clone.as_ref().exists() {
        std::fs::remove_dir_all(path)?;
    }
    let modules = ast
        .inner
        .into_iter()
        .map(|node| generate_node(path_clone.clone(), node))
        .map(|res| match res {
            Ok(d) => Ok(d),
            Err(e) => {
                eprint!("{}", e);
                Err(())
            }
        })
        .filter_map(Result::ok)
        .collect::<Vec<(String, Vec<String>)>>();
    generate_mod_rs_file(path_clone, modules)
}
