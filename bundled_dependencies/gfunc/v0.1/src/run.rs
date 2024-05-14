use std::collections::HashMap;
use std::collections::HashSet;
use std::str;

#[derive(Debug)]
pub struct RunOptions {
    chars: String,
    longs: Vec<String>,
}
#[derive(Debug)]
pub struct RunValues {
    valued: Vec<(String, String)>,
}
#[derive(Debug)]
pub struct RunArguements {
    args: Vec<String>,
}
#[derive(Debug)]
#[allow(dead_code)]
pub enum ValidateError {
    InvalidChar(char),
    InvalidLong(String),
    DuplicateOption(String),
    ArguementCount(usize, usize),
    InvalidArguement(String),
}
pub type ValidateResult<T> = Result<T, ValidateError>;
pub trait ValidateExit<T> {
    fn auto_exit(self) -> T;
}
impl<T> ValidateExit<T> for ValidateResult<T> {
    fn auto_exit(self) -> T {
        match self {
            Self::Ok(v) => v,
            Self::Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
impl std::fmt::Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ValidateError::*;
        match self {
            InvalidChar(char) => write!(f, "Unknown option alias \'-{}\'", char),
            InvalidLong(long) => write!(f, "Unknown option \'--{}\'", long),
            DuplicateOption(opt) => write!(
                f,
                "Option \'--{}\' (and/or its alias) specified multiple times",
                opt
            ),
            ArguementCount(exp, got) => write!(f, "expected {} arguements, got {}", exp, got),
            InvalidArguement(arg) => write!(f, "arguement \'{}\' is invalid", arg),
        }
    }
}
impl RunArguements {
    pub fn validate_exact<I, F>(self, predicates: I) -> ValidateResult<Vec<String>>
    where
        F: FnOnce(&str) -> bool,
        I: IntoIterator<Item = F>,
    {
        let mut p_count: usize = 0;
        let mut o = Vec::<String>::new();
        let arg_count = self.args.len();
        let mut a_iter = self.args.into_iter();
        for pred in predicates {
            p_count += 1;
            match a_iter.next() {
                Some(arg) => match pred(arg.as_str()) {
                    true => o.push(arg),
                    false => return Err(ValidateError::InvalidArguement(arg)),
                },
                None => continue,
            };
        }
        if p_count != arg_count {
            return Err(ValidateError::ArguementCount(p_count, arg_count));
        }
        Ok(o)
    }
}
impl RunOptions {
    pub fn validate<'a, I, C>(self, valid_singlets: I) -> ValidateResult<HashSet<String>>
    where
        C: Iterator<Item = (&'a str, Option<char>)> + Clone,
        I: IntoIterator<IntoIter = C>,
    {
        let mut o = HashSet::<String>::new();
        let iter = valid_singlets.into_iter();
        let mut alias_map = HashMap::<char, &str>::new();
        for (val, key_opt) in iter.clone() {
            if let Some(key) = key_opt {
                alias_map.insert(key, val);
            }
        }
        let valid_names = iter.clone().map(|(name, _alias)| name);
        let valid_aliases = iter.clone().filter_map(|(_name, alias)| alias);
        for char in self.chars.chars() {
            if None == valid_aliases.clone().find(|v| *v == char) {
                return Err(ValidateError::InvalidChar(char));
            }
            if !o.insert(alias_map[&char].to_owned()) {
                return Err(ValidateError::DuplicateOption(alias_map[&char].to_owned()));
            }
        }
        for long in self.longs {
            if let None = valid_names.clone().find(|v| *v == long) {
                return Err(ValidateError::InvalidLong(long));
            }
            match o.contains(&long) {
                true => return Err(ValidateError::DuplicateOption(long)),
                false => {
                    o.insert(long);
                }
            }
        }
        Ok(o)
    }
}
impl RunValues {
    pub fn validate<'a, I, C>(self, valid_keys: I) -> ValidateResult<HashMap<String, String>>
    where
        C: Iterator<Item = &'a str> + Clone,
        I: IntoIterator<IntoIter = C>,
    {
        let mut o = HashMap::<String, String>::new();
        let mut iter = valid_keys.into_iter();
        for (key, val) in self.valued {
            if None == iter.find(|valid| *valid == key.as_str()) {
                return Err(ValidateError::InvalidLong(key));
            }
            match o.contains_key(&key) {
                true => return Err(ValidateError::DuplicateOption(key)),
                false => {
                    o.insert(key, val);
                }
            }
        }
        Ok(o)
    }
}
#[derive(Debug)]
pub struct RunInfo {
    pub binary_path: String,
    pub options: RunOptions,
    pub values: RunValues,
    pub arguements: RunArguements,
}
impl RunInfo {
    pub fn new<I, T>(args: I) -> Self
    where
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        let mut iter = args.into_iter();
        let mut chars = String::new();
        let mut longs = Vec::<String>::new();
        let mut valued = Vec::<(String, String)>::new();
        let mut args = Vec::<String>::new();
        let binary_path = iter
            .next()
            .expect("Binary path not found? (no implied first arg)")
            .as_ref()
            .to_owned();
        for raw in iter {
            let raw = raw.as_ref();
            if let Some(l_opt) = str::strip_prefix(raw, "--") {
                match str::split_once(l_opt, "=") {
                    Some((opt, val)) => valued.push((opt.to_owned(), val.to_owned())),
                    None => longs.push(l_opt.to_owned()),
                };
                continue;
            }
            if let Some(c_opt) = str::strip_prefix(raw, "-") {
                chars.push_str(c_opt);
                continue;
            }
            args.push(raw.to_owned());
        }
        let options = RunOptions { chars, longs };
        let arguements = RunArguements { args };
        let values = RunValues { valued };
        RunInfo {
            binary_path,
            options,
            values,
            arguements,
        }
    }
    pub fn get_from_env() -> Self {
        Self::from(std::env::args())
    }
}
impl From<std::env::Args> for RunInfo {
    fn from(args: std::env::Args) -> Self {
        RunInfo::new(&args.collect::<Vec<String>>())
    }
}
