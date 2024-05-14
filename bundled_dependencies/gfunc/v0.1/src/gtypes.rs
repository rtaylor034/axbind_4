use std::sync::Arc;
#[derive(Debug, PartialEq)]
///Cheaply clonable (implemented with Arc)
pub struct Brancher<T> {
    pub root: Option<Arc<Brancher<T>>>,
    pub branch: Arc<T>,
}
impl<T> Brancher<T> {
    pub fn with(&self, branch: T) -> Brancher<T> {
        Brancher {
            root: Some(Arc::new(Brancher::clone(self))),
            branch: Arc::new(branch),
        }
    }
}
impl<T> Clone for Brancher<T> {
    fn clone(&self) -> Self {
        Brancher {
            root: self.root.clone(),
            branch: Arc::clone(&self.branch),
        }
    }
}
impl<T> From<T> for Brancher<T> {
    fn from(value: T) -> Self {
        Brancher {
            root: None,
            branch: Arc::new(value),
        }
    }
}

impl<T> std::fmt::Display for Brancher<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            Some(r) => write!(f, "{} -> {}", r, self.branch),
            None => write!(f, "{}", self.branch),
        }
    }
}
