pub mod ast;
pub mod composer;
pub mod parser;
pub mod std_library_ast;

use crate::std_library_ast::generate_standard_library;
use ast::ASTNode;
use ast::PackageNode;
use ast::SchemaFile;
use ast::AST;
use parser::schema_file::parse_schema_file;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

fn package_schema<T: AsRef<str>>(schema: &SchemaFile, path: &[T]) -> ASTNode {
    if path.len() > 0 {
        ASTNode::PackageNode(PackageNode {
            name: path[0].as_ref().to_string(),
            inner: vec![package_schema(schema, &path[1..])],
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
                                .map(|n| merge_schema(n, schema, &path[1..]))
                                .collect::<Vec<ASTNode>>(),
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
    if path.len() > 0 {
        let is_path_present = acc
            .inner
            .iter()
            .map(|n| match n {
                ASTNode::SchemaNode(_) => panic!("SchemaFile shouldn't be at the root of AST"),
                ASTNode::PackageNode(pn) => pn.name == path[0].as_ref().to_string(),
            })
            .fold(false, |acc, val| acc | val);
        if is_path_present {
            AST {
                inner: acc
                    .inner
                    .into_iter()
                    .map(|n| merge_schema(n, schema, path))
                    .collect::<Vec<ASTNode>>(),
            }
        } else {
            let mut inner = acc.inner;
            inner.push(package_schema(schema, path));
            AST { inner }
        }
    } else {
        panic!("SchemaFile does not have a package name");
    }
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
        .fold(generate_standard_library(), |acc, val| {
            merge_schema_root(acc, &val.1, &val.0)
        })
}
