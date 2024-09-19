use std::sync::{Mutex, Once};
use serde::{Deserialize, Serialize};

pub(crate) use crate::env::config::{Config as Env, Profile};
use crate::join_root;
use crate::path::{Path, SysPath};

static SINGLETON: Once = Once::new();
static mut CONFIGS: Option<Mutex<Configs>> = None;

lazy_static::lazy_static! {
    pub static ref PROCESS_NAME: Mutex<String> = Mutex::new(String::from("RaTuS"));
}

pub fn set_process_name<T: AsRef<str>>(name: T) {
    let mut process_name = PROCESS_NAME.lock().unwrap();
    *process_name = name.as_ref().to_string();
}

pub fn get_process_name() -> String {
    let process_name = PROCESS_NAME.lock().unwrap();
    process_name.clone()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kinds {
    pub trace: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub on: bool,
    pub debug: bool,
    pub save: bool,
    pub kinds: Kinds,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configs {
    log: Log,
    profile: Option<Profile>,
}

impl Configs {
    pub fn open<'a>() -> &'a Mutex<Configs> { Self::get() }

    fn get<'a>() -> &'a Mutex<Configs> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            unsafe {
                CONFIGS = Some(Mutex::new(Configs::new()));
            }
        });

        unsafe {
            CONFIGS.as_ref()
                .unwrap()
        }
    }

    fn new() -> Configs {
        let config: SysPath= join_root!("configs.json");
        let content: String = std::fs::read_to_string(config).unwrap();
        let config: Configs = serde_json::from_str(&content).unwrap();
        let profile: Profile = Env::open().lock().unwrap().profile();

        Configs {
            profile: Some(profile),
            ..config
        }
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn save(&self) -> bool { self.log.save }

    pub fn debug(&self) -> bool { self.log.debug }

    pub fn profile(&self) -> &Profile {
        self.profile.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Configs;

    #[test]
    fn test_new() {
        drop(Configs::open().lock().unwrap());
    }
}
