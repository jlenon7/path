use std::env;
use std::path::{Path as StdPath, PathBuf as StdPathBuf, MAIN_SEPARATOR};

struct Directories<'a> {
    bin: &'a str,
    src: &'a str,
}

const DIRECTORIES: Directories = Directories {
    bin: "bin",
    src: "src",
};

pub struct Path {}

impl Path {
    pub fn normalize<P: AsRef<StdPath>>(path: P) -> StdPathBuf {
        let mut components = path.as_ref().components().peekable();
        let mut result = StdPathBuf::new();

        while let Some(component) = components.next() {
            match component {
                std::path::Component::RootDir => {
                    result.push(component.as_os_str());
                }
                std::path::Component::Prefix(_) | std::path::Component::Normal(_) => {
                    result.push(component.as_os_str());
                }
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    result.pop();
                }
            }
        }

        result
    }

    fn remove_slashes(mut path: String) -> String {
        if path.ends_with(MAIN_SEPARATOR) {
            path.pop();
            Path::remove_slashes(path)
        } else {
            path
        }
    }

    pub fn pwd(sub: Option<String>) -> String {
        let pwd = format!(
            "{}{}{}",
            env::current_dir().unwrap().display(),
            MAIN_SEPARATOR,
            Path::normalize(sub.unwrap_or(String::from(""))).display()
        );

        Path::remove_slashes(pwd)
    }

    pub fn bin(sub: Option<String>) -> String {
        let path = format!(
            "{}{}{}",
            DIRECTORIES.bin.to_string(),
            MAIN_SEPARATOR,
            Path::normalize(sub.unwrap_or(String::from(""))).display()
        );

        Path::pwd(Some(path))
    }

    pub fn src(sub: Option<String>) -> String {
        let path = format!(
            "{}{}{}",
            DIRECTORIES.src.to_string(),
            MAIN_SEPARATOR,
            Path::normalize(sub.unwrap_or(String::from(""))).display()
        );

        Path::pwd(Some(path))
    }
}
