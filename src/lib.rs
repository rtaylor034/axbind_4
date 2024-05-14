use std::path::{ PathBuf, Path };
use std::collections::HashMap;
use std::pin::*;
use glob::glob;
use anyhow::{Error, anyhow};
mod schema;
pub fn parse_glob(str: &str) -> Result<Vec<PathBuf>, String> {
    glob(str).map_err(|e| e.to_string())?
        .map(|r| r.map_err(|e| e.to_string()))
        .collect()
}
pub enum IdentifierType {
    Function,
    Map,
}
impl schema::BindFile {
    pub fn apply(&self, filebase: &AxbindFilebase) -> Result<(), Error> {
        todo!();
    }
}

pub struct AxbindFilebase {
    pub functions: Filebase<schema::FunctionFile>,
    pub maps: Filebase<schema::MapFile>,
}
pub struct Filebase<T: schema::FromFile> {
    files: HashMap<String, Pin<Box<T>>>,
    root: PathBuf,
}
impl AxbindFilebase {
    pub fn new<P: AsRef<Path>>(root: P) -> AxbindFilebase {
        AxbindFilebase {
            functions: Filebase::new(root.as_ref().join("functions")),
            maps: Filebase::new(root.as_ref().join("maps")),
        }
    }
}
impl<T: schema::FromFile> Filebase<T> {
    pub fn new(path_root: PathBuf) -> Filebase<T> {
        Filebase { files: HashMap::new(), root: path_root }
    }
    pub fn query<'s>(&'s self, identifier: &str) -> Result<&'s T, Error> {
        if let Some(stored) = self.files.get(identifier) { return Ok(stored); }
        // probably (definetely) not the correct way to do interior mutability, but it works.
        unsafe {
            let ptr: *const Filebase<T> = &*self;
            let mut_ptr = ptr.cast_mut();
            (*mut_ptr).files.insert(identifier.to_owned(), Box::pin(T::from_file(self.root.join(identifier).with_extension("toml"))?));
        }
        Ok(&self.files[identifier])
    }
}
