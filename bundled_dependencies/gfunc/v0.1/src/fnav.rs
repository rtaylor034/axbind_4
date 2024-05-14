use std::fs;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MetaType {
    File,
    Directory,
}
///Recursively searches [dir] for any paths that end with [path].
///# Errors
///- If [std::fs::read_dir()] of [dir] return Err, return it.
pub fn rsearch_dir<T1, T2>(
    dir: T1,
    path: T2,
    search_for: MetaType,
) -> Result<Vec<PathBuf>, std::io::Error>
where
    T1: AsRef<Path>,
    T2: AsRef<Path>,
{
    let mut o = Vec::<PathBuf>::new();

    for f in fs::read_dir(dir)? {
        if let Ok(f) = f {
            let fpath = f.path();
            if let Ok(data) = fs::metadata(&fpath) {
                let ftype = data.file_type();
                if ftype.is_dir() {
                    if let Ok(rsearch) = rsearch_dir(&fpath, path.as_ref(), search_for) {
                        for r in rsearch {
                            o.push(r);
                        }
                    }
                    if search_for == MetaType::Directory {
                        if fpath.ends_with(&path) {
                            o.push(fpath);
                        }
                    }
                } else if ftype.is_file() && search_for == MetaType::File {
                    if f.path().ends_with(&path) {
                        o.push(fpath);
                    }
                }
            }
        }
    }
    Ok(o)
}
pub fn rsearch_dir_pred<T1, F>(dir: T1, predicate: F) -> Result<Vec<PathBuf>, std::io::Error>
where
    T1: AsRef<Path>,
    F: Fn(&PathBuf) -> bool,
{
    rsearch_dir_pred_internal(dir, &predicate)
}
fn rsearch_dir_pred_internal<T1, F>(dir: T1, predicate: &F) -> Result<Vec<PathBuf>, std::io::Error>
where
    T1: AsRef<Path>,
    F: Fn(&PathBuf) -> bool,
{
    let mut o = Vec::<PathBuf>::new();

    for f in fs::read_dir(dir)? {
        if let Ok(f) = f {
            let fpath = f.path();
            if let Ok(data) = fs::metadata(&fpath) {
                let ftype = data.file_type();
                if ftype.is_dir() {
                    if let Ok(rsearch) = rsearch_dir_pred_internal(&fpath, predicate) {
                        for r in rsearch {
                            o.push(r);
                        }
                    }
                }
                if predicate(&fpath) {
                    o.push(fpath);
                }
            }
        }
    }
    Ok(o)
}
