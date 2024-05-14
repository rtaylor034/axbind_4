use serde::Deserialize;
use std::path::{PathBuf, Path};

pub trait FromFile {
    fn from_file(path: PathBuf) -> Result<Self, anyhow::Error> where Self: Sized;
    fn source(&self) -> &Path;
}
#[derive(Debug, Deserialize)]
pub struct BindFile {
    pub meta: Option<MetaOpts>,
    pub groups: Vec<Group>,
    #[serde(skip)]
    source_path: PathBuf,
}
#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub inclusions: Vec<String>,
    pub values: std::collections::HashMap<String, String>,
    #[serde(skip)]
    source_path: PathBuf,
}
#[derive(Debug, Deserialize)]
pub struct FunctionFile {
    pub meta: Option<MetaOpts>,
    pub function: Function,
    #[serde(skip)]
    source_path: PathBuf,
}
#[derive(Debug, Deserialize)]
pub struct MetaOpts {
    pub escape: Option<String>,
    pub proxy: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Function {
    pub parameters: Option<Vec<String>>,
    pub parameter_format: Option<String>,
    pub pipeline: FunctionPipeline,
}
#[derive(Debug, Deserialize)]
pub enum FunctionPipeline {
    Command(CommandFunction),
    Internal(InternalFunction),
}
#[derive(Debug, Deserialize)]
pub struct CommandFunction {
    pub binary: String,
    pub stdin: Option<String>,
    pub args: Option<Vec<String>>,
}
#[derive(Debug, Deserialize)]
pub struct InternalFunction {
    pub internal: String,
    pub args: Option<Vec<String>>
}
#[derive(Debug, Deserialize)]
pub struct Group {
    pub files: Vec<String>,
    pub axbind_filename: String,
    pub captures: Vec<Capture>,
}
#[derive(Debug, Deserialize)]
pub struct Capture {
    pub capture: String,
    pub escape: Option<String>,
    pub layers: Vec<Layer>,
}
#[derive(Debug, Deserialize)]
pub enum Layer {
    Map(MapLayer),
    Function(FunctionLayer),
}
#[derive(Debug, Deserialize)]
pub struct MapLayer {
    pub map: String,
}
#[derive(Debug, Deserialize)]
pub struct FunctionLayer{
    pub function: String,
    pub args: Option<Vec<String>>,
}

impl FromFile for BindFile {
    fn from_file(path: PathBuf) -> Result<Self, anyhow::Error> where Self: Sized {
        let mut o: BindFile = toml::from_str(&std::fs::read_to_string(&path)?)?;
        o.source_path = path;
        Ok(o)
    }
    fn source(&self) -> &Path {
        &self.source_path
    }
}
impl FromFile for FunctionFile {
    fn from_file(path: PathBuf) -> Result<Self, anyhow::Error> where Self: Sized {
        let mut o: FunctionFile = toml::from_str(&std::fs::read_to_string(&path)?)?;
        o.source_path = path;
        Ok(o)
    }
    fn source(&self) -> &Path {
        &self.source_path
    }
}
impl FromFile for MapFile {
    fn from_file(path: PathBuf) -> Result<Self, anyhow::Error> where Self: Sized {
        let mut o: MapFile = toml::from_str(&std::fs::read_to_string(&path)?)?;
        o.source_path = path;
        Ok(o)
    }
    fn source(&self) -> &Path {
        &self.source_path
    }
}
