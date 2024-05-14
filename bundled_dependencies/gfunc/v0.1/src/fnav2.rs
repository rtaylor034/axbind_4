use std::fs;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MetaType {
    File,
    Directory,
}
//im kinda retarded
///Recursively searches [dir] for any paths that end with [path], with [depth].
///elements of returned dir are (<found path>, <depth to spare>).
///- <depth to spare> meaning ([depth] - <depth found at>), this has a good reason to exist.
///# Errors
///- If [std::fs::read_dir()] of [dir] return Err, return it.
pub fn rsearch_dir<T1, T2>(
    dir: T1,
    path: T2,
    search_for: MetaType,
    mut depth: u8,
) -> Result<Vec<(PathBuf, u8)>, std::io::Error>
where
    T1: AsRef<Path>,
    T2: AsRef<Path>,
{
    let mut o = Vec::<(PathBuf, u8)>::new();
    if depth == 0 { return Ok(o) }
    depth -= 1;
    for f in fs::read_dir(dir)? {
        if let Ok(f) = f {
            let fpath = f.path();
            if let Ok(data) = fs::metadata(&fpath) {
                let ftype = data.file_type();
                if ftype.is_dir() {
                    if let Ok(rsearch) = rsearch_dir(&fpath, path.as_ref(), search_for, depth) {
                        for r in rsearch {
                            o.push(r);
                        }
                    }
                    if search_for == MetaType::Directory {
                        if fpath.ends_with(&path) {
                            o.push((fpath, depth));
                        }
                    }
                } else if ftype.is_file() && search_for == MetaType::File {
                    if f.path().ends_with(&path) {
                        o.push((fpath, depth));
                    }
                }
            }
        }
    }
    Ok(o)
}
pub fn rsearch_dir_pred<T1, F>(dir: T1, depth: u8, predicate: F) -> Result<Vec<(PathBuf, u8)>, std::io::Error>
where
    T1: AsRef<Path>,
    F: Fn(&PathBuf) -> bool,
{
    rsearch_dir_pred_internal(dir, depth, &predicate)
}
fn rsearch_dir_pred_internal<T1, F>(dir: T1, mut depth: u8, predicate: &F) -> Result<Vec<(PathBuf, u8)>, std::io::Error>
where
    T1: AsRef<Path>,
    F: Fn(&PathBuf) -> bool,
{
    let mut o = Vec::<(PathBuf, u8)>::new();
    if depth == 0 { return Ok(o) }
    depth -= 1;

    for f in fs::read_dir(dir)? {
        if let Ok(f) = f {
            let fpath = f.path();
            if let Ok(data) = fs::metadata(&fpath) {
                let ftype = data.file_type();
                if ftype.is_dir() {
                    if let Ok(rsearch) = rsearch_dir_pred_internal(&fpath, depth, predicate) {
                        for r in rsearch {
                            o.push(r);
                        }
                    }
                }
                if predicate(&fpath) {
                    o.push((fpath, depth));
                }
            }
        }
    }
    Ok(o)
}
