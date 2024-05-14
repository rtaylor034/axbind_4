use std::path::PathBuf;

#[cfg(feature = "fnav2")]
pub mod fnav2;
#[cfg(feature = "fnav")]
pub mod fnav;
#[cfg(feature = "run")]
pub mod run;
#[cfg(feature = "tomlutil")]
pub mod tomlutil;
#[cfg(feature = "gtypes")]
pub mod gtypes;


///loops over iterator until Some is returned by body. if it is never returned, return None.
pub fn for_until<I, R, Item, Func>(iterator: I, mut body: Func) -> Option<R>
where
    I: IntoIterator<Item = Item>,
    Func: FnMut(Item) -> Option<R>,
{
    let mut i = iterator.into_iter();
    
    loop {
        match i.next() {
            None => break None,
            Some(item) => {
                if let Some(o) = body(item) {
                    break Some(o);
                }
            }
        }
    }
}
pub fn simple_envpath(string: &str) -> Result<PathBuf, std::env::VarError> {
    let mut o = PathBuf::new();
    for part in string.split(&['/']) {
        match part.strip_prefix('$') {
            Some(var) => o.push(&std::env::var(var)?),
            None => o.push(part),
        }
    }
    Ok(o)
}

