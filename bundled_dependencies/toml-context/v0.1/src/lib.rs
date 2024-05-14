use gfunc::gtypes::Brancher;
use toml;
use std::fs;
use std::path::PathBuf;

//the use of an owned-String brancher means that there is a duplicate String for every toml-key
//that is read and put into Context. (assuming there is no redundant usage of 'with()')
pub type Context = Brancher<String>;
#[derive(Debug, Clone, PartialEq)]
pub struct TableHandle<'t> {
    pub context: Context,
    pub table: &'t toml::Table,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PotentialValueHandle<'t> {
    pub value: Option<&'t toml::Value>,
    pub context: Context,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ValueHandle<'t> {
    pub value: &'t toml::Value,
    pub context: Context,
}
#[derive(Debug, PartialEq)]
pub struct TableRoot {
    pub table: toml::Table,
    pub context: Context,
}
#[derive(Debug)]
pub enum RootErr {
    ReadFile(PathBuf, std::io::Error),
    Parse(PathBuf, <toml::Table as core::str::FromStr>::Err),
}
impl std::error::Error for RootErr {}
impl TableRoot {
    pub fn handle(&self) -> TableHandle {
        TableHandle {
            table: &self.table,
            context: self.context.clone(),
        }
    }
    pub fn from_file_path<P>(path: P) -> Result<TableRoot, RootErr>
    where P: AsRef<std::path::Path> {
        Ok(TableRoot {
            table: fs::read_to_string(&path)
                .map_err(|e| RootErr::ReadFile(path.as_ref().to_path_buf(), e))?
                .parse()
                .map_err(|e| RootErr::Parse(path.as_ref().to_path_buf(), e))?,
            context: Context::from(String::from(path.as_ref().to_string_lossy())),
        })
    }
}
impl<'st> TableHandle<'st> {
    pub fn new_root<'s>(table: &'s toml::Table, root_context: String) -> TableHandle<'s> {
        TableHandle {
            table,
            context: root_context.into(),
        }
    }
    pub fn get(&self, key: &str) -> PotentialValueHandle<'st> {
        PotentialValueHandle {
            value: self.table.get(key),
            context: self.context.with(key.to_owned()),
        }
    }
    pub fn traverse<I, S>(&self, path: I) -> TableResult<TableHandle<'st>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut o = self.clone();
        for key in path {
            o = extract_value!(Table, o.get(key.as_ref()))?;
        }
        Ok(o)
    }
}
impl<'st> IntoIterator for TableHandle<'st> {
    type Item = (&'st String, ValueHandle<'st>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    //I wish I knew how to implement this without collecting it into a Vec.
    //bit of a major redundancy
    fn into_iter(self) -> Self::IntoIter {
        self.table.iter().map(|(k, val)| (k, ValueHandle { value: val, context: self.context.with(k.to_owned()) })).collect::<Vec<(&String, ValueHandle)>>().into_iter()
    }
}
impl<'st> From<ValueHandle<'st>> for PotentialValueHandle<'st> {
    fn from(handle: ValueHandle<'st>) -> Self {
        PotentialValueHandle {
            value: Some(handle.value),
            context: handle.context,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TableGetError {
    pub context: Context,
    pub error: TableGetErr,
}
impl std::error::Error for TableGetError {}
#[derive(Debug, Clone)]
pub enum TableGetErr {
    NoKey,
    WrongType(&'static str),
}
impl std::fmt::Display for TableGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TableGetErr::*;
        match self.error {
            WrongType(t) => write!(
                f,
                "toml::Value for key '{}' is of wrong type. (expected {})",
                self.context, t
            ),
            NoKey => write!(f, "Expected key at '{}', no such key exists.", self.context),
        }
    }
}
//TODO: make actually useful thanks
impl std::fmt::Display for RootErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(file, e) => {
                writeln!(f, "Error while parsing file {:?} to toml", file)?;
                writeln!(f, " - {}", e)
            }
            Self::ReadFile(file, e) => {
                writeln!(f, "Error reading file {:?} while trying to parse to toml", file)?;
                writeln!(f, " - {}", e)
            }
        }
    }
}
pub type TableResult<T> = Result<T, TableGetError>;
pub trait TableResultOptional<T> {
    fn optional(self) -> TableResult<Option<T>>;
}
impl<T> TableResultOptional<T> for TableResult<T> {
    fn optional(self) -> TableResult<Option<T>> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e.error {
                TableGetErr::NoKey => Ok(None),
                _ => Err(e),
            },
        }
    }
}
#[macro_export]
///(expected_value_type: toml::Value::$, value_handle: (Potential)ValueHandle) -> Result<X, TableGetError>
///- extracts the value held in 'value_handle'
///- if 'expected_value_type' is 'Table' or 'Array', table/element references are returned as handles for convenience
//DEV - perhaps take a reference to the PotentialValueHandle instead of owned.
macro_rules! extract_value {
    (Table, $val_handle:expr) => {{
        let handle: $crate::PotentialValueHandle = $val_handle.into();
        match handle.value {
            None => Err($crate::TableGetError {
                error: $crate::TableGetErr::NoKey,
                context: handle.context.clone(),
            }),
            Some(val) => match val {
                toml::Value::Table(o) => Ok($crate::TableHandle {
                    table: o,
                    context: handle.context.clone(),
                }),
                _ => Err($crate::TableGetError {
                    error: $crate::TableGetErr::WrongType(stringify!(Table)),
                    context: handle.context.clone(),
                }),
            },
        }
    }};
    (Array, $val_handle:expr) => {{
        let handle: $crate::PotentialValueHandle = $val_handle.into();
        match handle.value {
            None => Err($crate::TableGetError {
                error: $crate::TableGetErr::NoKey,
                context: handle.context.clone(),
            }),
            Some(val) => match val {
                toml::Value::Array(o) => Ok(o
                    .into_iter()
                    .map(|tval| $crate::ValueHandle {
                        value: tval,
                        context: handle.context.clone(),
                    })
                    .collect::<Vec<$crate::ValueHandle>>()),
                _ => Err($crate::TableGetError {
                    error: $crate::TableGetErr::WrongType(stringify!(Array)),
                    context: handle.context.clone(),
                }),
            },
        }
    }};
    ($expected_type:tt, $val_handle:expr) => {{
        let handle: $crate::PotentialValueHandle = $val_handle.into();
        match handle.value {
            None => Err($crate::TableGetError {
                error: $crate::TableGetErr::NoKey,
                context: handle.context.clone(),
            }),
            Some(val) => match val {
                toml::Value::$expected_type(o) => Ok(o),
                _ => Err($crate::TableGetError {
                    error: $crate::TableGetErr::WrongType(stringify!($expected_type)),
                    context: handle.context.clone(),
                }),
            },
        }
    }};
}
