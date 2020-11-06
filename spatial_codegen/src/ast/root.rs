use crate::ast::std::generate_standard_library;
use crate::ast::ASTNode;
use crate::ast::SchemaFile;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq)]
pub struct AST {
    pub inner: Vec<ASTNode>,
}

impl Default for AST {
    fn default() -> Self {
        generate_standard_library()
    }
}

impl AST {
    pub fn generate<P: AsRef<Path> + Clone>(&self, path: P) -> Result<(), std::io::Error> {
        let path_clone = path.clone();
        if path_clone.as_ref().exists() {
            std::fs::remove_dir_all(path)?;
        }
        for node in &self.inner {
            node.generate_node(path_clone.clone())?;
        }
        ASTNode::generate_mod_rs(&self.inner, path_clone)
    }

    fn merge_schema<T: AsRef<str>>(self, schema: &SchemaFile, path: &[T]) -> Self {
        if !path.is_empty() {
            let is_path_present = self
                .inner
                .iter()
                .map(|n| match n {
                    ASTNode::SchemaNode(_) => panic!("SchemaFile shouldn't be at the root of AST"),
                    ASTNode::PackageNode(pn) => pn.name == *path[0].as_ref(),
                })
                .fold(false, |acc, val| acc | val);
            if is_path_present {
                AST {
                    inner: self
                        .inner
                        .into_iter()
                        .map(|n| n.merge_schema(schema, path))
                        .collect::<Vec<ASTNode>>(),
                }
            } else {
                let mut inner = self.inner;
                inner.push(ASTNode::package_schema(schema, path));
                AST { inner }
            }
        } else {
            panic!("SchemaFile does not have a package name");
        }
    }
}

impl<P: AsRef<Path>> From<P> for AST {
    fn from(path: P) -> Self {
        WalkDir::new(path)
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
            .map(SchemaFile::try_from)
            .map(|schemas| match schemas {
                Ok(data) => Ok(data),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(())
                }
            })
            .filter_map(Result::ok)
            .fold(Self::default(), |acc, val| {
                acc.merge_schema(&val, &val.package_name)
            })
    }
}
