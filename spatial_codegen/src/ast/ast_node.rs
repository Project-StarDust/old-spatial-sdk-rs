use crate::ast::PackageNode;
use crate::ast::SchemaFile;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum ASTNode {
    PackageNode(PackageNode),
    SchemaNode(SchemaFile),
}

impl ASTNode {
    fn get_export(&self) -> (String, Vec<String>) {
        match self {
            Self::PackageNode(pn) => (pn.name.clone(), Vec::new()),
            Self::SchemaNode(schema) => (schema.name.clone(), schema.get_exports()),
        }
    }

    fn get_exports(data: &[Self]) -> Vec<(String, Vec<String>)> {
        data.iter().map(Self::get_export).collect()
    }

    pub fn generate_mod_rs<P: AsRef<Path> + Clone>(
        nodes: &[Self],
        path: P,
    ) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(path.clone()).map(|_| {
            let mut file = File::create(path.clone().as_ref().join("mod.rs"))?;
            for module in Self::get_exports(nodes) {
                writeln!(
                    file,
                    "{}mod {};",
                    if !module.1.is_empty() { "" } else { "pub " },
                    module.0
                )?;
                for usage in module.1 {
                    writeln!(file, "pub use {}::{};", module.0, usage)?;
                }
            }
            Ok(())
        })?
    }

    pub fn generate_node<P: AsRef<Path> + Clone>(&self, path: P) -> Result<(), std::io::Error> {
        match self {
            Self::SchemaNode(node) => node.generate_schema(path),
            Self::PackageNode(node) => {
                let name = node.name.clone();
                for node in &node.inner {
                    Self::generate_node(node, path.as_ref().join(&name))?;
                }
                Self::generate_mod_rs(&node.inner, path.as_ref().join(&name))
            }
        }
    }

    pub fn merge_schema<T: AsRef<str>>(self, schema: &SchemaFile, path: &[T]) -> Self {
        match self {
            ASTNode::PackageNode(package_node) => {
                if package_node.name == path[0].as_ref() {
                    if path.len() > 1 {
                        if package_node.has_path(path[1].as_ref()) {
                            ASTNode::PackageNode(PackageNode {
                                name: package_node.name,
                                inner: package_node
                                    .inner
                                    .into_iter()
                                    .map(|n| n.merge_schema(schema, &path[1..]))
                                    .collect::<Vec<ASTNode>>(),
                            })
                        } else {
                            ASTNode::PackageNode(
                                package_node.add_node(Self::package_schema(schema, &path[1..])),
                            )
                        }
                    } else {
                        ASTNode::PackageNode(
                            package_node.add_node(ASTNode::SchemaNode(schema.clone())),
                        )
                    }
                } else {
                    ASTNode::PackageNode(package_node)
                }
            }
            ASTNode::SchemaNode(s) => ASTNode::SchemaNode(s),
        }
    }

    pub fn package_schema<T: AsRef<str>>(schema: &SchemaFile, path: &[T]) -> Self {
        if !path.is_empty() {
            ASTNode::PackageNode(PackageNode {
                name: path[0].as_ref().to_string(),
                inner: vec![Self::package_schema(schema, &path[1..])],
            })
        } else {
            ASTNode::SchemaNode(schema.clone())
        }
    }
}
