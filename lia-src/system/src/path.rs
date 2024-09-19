use std::{
    env, ffi::OsStr, fs, path, sync::{Mutex, MutexGuard, Once}
};

pub type SysPath = path::PathBuf;

static ROOT_NAME: &str = "lia-src";
static mut PATH: Option<Mutex<Path>> = None;
static SINGLETON: Once = Once::new();

pub struct Path {
    root: SysPath,
}

#[macro_export]
macro_rules! join_root {
    ($($arg:expr),*) => {
        Path::join_root(vec![$($arg),*])
    };
}
pub(crate) use join_root;

impl Path {
    pub fn new(path: &str) -> SysPath {
        let path: SysPath = path::PathBuf::from(path);

        path
    }

    pub fn get_migrations_path() -> SysPath {
        let migrations_path: SysPath = join_root!("back-end", "migrations");

        migrations_path
    }

    pub fn join_root(file_folder_names: Vec<&str>) -> SysPath {
        let path: MutexGuard<Path> = Path::get().lock().unwrap();
        let mut joined_path: SysPath = path.root.clone();

        for file_folder_name in file_folder_names {
            joined_path.push(file_folder_name);
        }

        joined_path
    }

    #[allow(dead_code)]
    fn join(path: &SysPath, file_folder_name:String) -> SysPath {
        let mut joined_path: SysPath = path.clone();
        joined_path.push(file_folder_name);

        joined_path
    }

    fn get<'a>() -> &'a Mutex<Path> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            let root: SysPath = Path::find_root();
            unsafe {
                PATH = Some(Mutex::new(Path { root }));
            }
        });

        unsafe {
            PATH.as_ref()
                .unwrap()

        }
    }

    fn find_root() -> SysPath {
        let mut current_path = env::current_exe().unwrap();
        let mut tries: u8 = 0;
    
        loop {
            let parent = match current_path.parent() {
                Some(p) => p,
                None => panic!("Reached the root of the filesystem without finding the root directory"),
            };
    
            if let Ok(entries) = fs::read_dir(parent) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        // Check if the entry is a directory named ROOT_NAME
                        if path.is_dir() && path.file_name() == Some(OsStr::new(ROOT_NAME)) {
                            return path;
                        }
                    }
                }
            }
    
            tries += 1;
            if tries > 10 {
                panic!("Could not find root directory after 10 attempts");
            }
    
            current_path = parent.to_path_buf();
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_root() {
        let executable_path: SysPath = env::current_exe().unwrap();

        let mut root: SysPath = executable_path.clone();
        for _ in 0..4 { // Goes back from target/debug/deps to the root directory
            root = root.parent().unwrap().to_path_buf();
        }

        let found_root: SysPath = Path::find_root();

        assert_eq!(root, found_root);
    }
}
