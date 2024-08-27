use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
pub enum FilterMode {
    Include,
    Exclude,
}

pub struct TraverseIterator<'a, F>
where
    F: Fn(&Path) -> (FilterMode, bool),
{
    filter: &'a F,
    stack: Vec<PathBuf>,
}

impl<'a, F> TraverseIterator<'a, F>
where
    F: Fn(&Path) -> (FilterMode, bool),
{
    pub fn new(path: &'a str, filter: &'a F) -> Self {
        TraverseIterator {
            filter,
            stack: vec![Path::new(path).to_path_buf()],
        }
    }
}

impl<'a, F> Iterator for TraverseIterator<'a, F>
where
    F: Fn(&Path) -> (FilterMode, bool),
{
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.stack.pop() {
            if path.is_dir() {
                // println!("Folder: {}", path.display());
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            let (mode, result) = (self.filter)(&path);
                            if (mode == FilterMode::Include && result)
                                || (mode == FilterMode::Exclude && !result)
                            {
                                self.stack.push(path);
                            }
                        }
                    }
                }
            } else {
                // println!("File: {}", path.display());
                return Some(path);
            }
        }
        None
    }
}

pub fn default_filter(path: &Path) -> (FilterMode, bool) {
    if path.is_dir() {
        // 排除.git和target目录
        let re = Regex::new(r"\/\.\w+|target").unwrap();
        (FilterMode::Exclude, re.is_match(path.to_str().unwrap()))
    } else {
        // 只包含以.lox结尾文件
        let re = Regex::new(r"\.lox$").unwrap();
        (FilterMode::Include, re.is_match(path.to_str().unwrap()))
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_traverse() {
        let iterator = TraverseIterator::new("/Users/bytedance/Desktop/rlox", &default_filter);
        for path in iterator {
            println!("{}", path.display());
        }
    }
}
