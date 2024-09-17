use std::env;
use std::sync::{Mutex, Once};

use dotenv::from_path;
use serde::{Deserialize, Serialize};

use crate::path::{SysPath, join_root, Path};
use crate::env::Env;

static SINGLETON: Once = Once::new();
static mut CONFIG: Option<Mutex<Config>> = None;

pub(crate) struct Config {
    profile: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub(crate) enum Profile {
    DEBUG,
}

impl Profile {
    fn from_string(input: &String) -> Profile {
        match input.as_str() {
            "DEBUG" => Profile::DEBUG,
            _ => unreachable!("Invalid profile"), // CI/CD must enforce this with ENV vars
        }
    }
}

impl Env for Config {
    fn get<'a>() -> &'a Mutex<Config> { // Will be locked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            unsafe {
                let path: SysPath = join_root!(".env");
                CONFIG = Some(Mutex::new(Config::new(path)));
            }
        });

        unsafe {
            CONFIG.as_ref()
                .unwrap()
        }
    }

    fn new(path: SysPath) -> Self {
        Self::set_env(&path);
        let env: Self = Self::read_env();
        env
    }

    fn set_env(path: &SysPath) -> () {
        from_path(path.as_path()).expect("Failed to read .env file.");
    }

    fn read_env() -> Self {
        Config {
            profile: env::var("PROFILE").unwrap()
        }
    }
}

impl Config {
    pub fn open<'a>() -> &'a Mutex<Config> { Self::get() }

    pub fn profile(self: &Self) -> Profile {
        Profile::from_string(&self.profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::open().lock().unwrap();
        let prof_works: bool = config.profile() == Profile::DEBUG;
        assert_eq!(prof_works, true);
    }
}
