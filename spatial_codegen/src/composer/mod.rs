use crate::ast::ASTNode;
use crate::ast::SchemaFile;
use crate::ast::AST;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn generate_schema<P: AsRef<Path> + Clone>(
    path: P,
    schema: &SchemaFile,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path.clone()).map(|_| {
        let data = schema.generate();
        let mut file = File::create(path.clone().as_ref().join(schema.name + ".rs"))?;
        write!(file, "{}\n", data)?;
        Ok(())
    })?
}

fn generate_node<P: AsRef<Path> + Clone>(path: P, node: &ASTNode) -> Result<(), std::io::Error> {
    let path_clone = path.clone();
    match node {
        ASTNode::SchemaNode(schema) => generate_schema(path_clone, schema),
        ASTNode::PackageNode(pn) => {
            let name = pn.name.clone();
            for node in pn.inner {
                generate_node(path_clone.as_ref().join(&name), &*node)?;
            }
            generate_mod_rs_file(
                path_clone.as_ref().join(&name),
                ASTNode::get_exports(&pn.inner),
            )
        }
    }
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
    for node in &ast.inner {
        generate_node(path_clone.clone(), node)?;
    }
    generate_mod_rs_file(path_clone, ASTNode::get_exports(&ast.inner))
}
