use crate::ast::ASTNode;
use crate::ast::PackageNode;
use crate::ast::SchemaFile;
use crate::ast::AST;
use crate::parser::schema_file::parse_schema_file;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub mod command;
pub mod component;
pub mod data_type;
pub mod event;
pub mod member;
pub mod package_name;
pub mod schema_file;
pub mod spatial_type;
pub mod utils;

fn package_schema<T: AsRef<str>>(schema: &SchemaFile, path: &[T]) -> ASTNode {
    if path.len() > 0 {
        ASTNode::PackageNode(PackageNode {
            name: path[0].as_ref().to_string(),
            inner: vec![Box::new(package_schema(schema, &path[1..]))],
        })
    } else {
        ASTNode::SchemaNode(schema.clone())
    }
}

fn merge_schema<T: AsRef<str>>(node: ASTNode, schema: &SchemaFile, path: &[T]) -> ASTNode {
    match node {
        ASTNode::PackageNode(package_node) => {
            if package_node.name == path[0].as_ref().to_string() {
                if path.len() > 1 {
                    if package_node.has_path(path[1].as_ref().to_string()) {
                        ASTNode::PackageNode(PackageNode {
                            name: package_node.name,
                            inner: package_node
                                .inner
                                .into_iter()
                                .map(|n| Box::new(merge_schema(*n, schema, &path[1..])))
                                .collect::<Vec<Box<ASTNode>>>(),
                        })
                    } else {
                        ASTNode::PackageNode(
                            package_node.add_node(package_schema(schema, &path[1..])),
                        )
                    }
                } else {
                    ASTNode::PackageNode(package_node.add_node(ASTNode::SchemaNode(schema.clone())))
                }
            } else {
                ASTNode::PackageNode(package_node)
            }
        }
        ASTNode::SchemaNode(s) => ASTNode::SchemaNode(s),
    }
}

fn merge_schema_root<T: AsRef<str>>(acc: AST, schema: &SchemaFile, path: &[T]) -> AST {
    let root = if let Some(root) = acc.root {
        merge_schema(root, schema, path)
    } else {
        package_schema(schema, path)
    };
    AST { root: Some(root) }
}

pub fn parse_folder<P: AsRef<Path>>(directory_path: P) -> AST {
    WalkDir::new(directory_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| {
            e.path()
                .to_str()
                .map(|s| s.to_string())
                .ok_or("Can't tranform into &str")
        })
        .filter_map(Result::ok)
        .map(PathBuf::from)
        .filter(|p| p.extension() == Some(OsStr::new("schema")))
        .map(parse_schema_file)
        .map(|schemas| match schemas {
            Ok(data) => Ok(data),
            Err(e) => {
                eprintln!("{}", e);
                Err(())
            }
        })
        .filter_map(Result::ok)
        .fold(AST::default(), |acc, val| {
            merge_schema_root(acc, &val.1, &val.0)
        })
}
