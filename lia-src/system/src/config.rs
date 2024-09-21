use std::sync::{Mutex, Once};
use serde::{Deserialize, Serialize};

pub(crate) use crate::env::config::{Config as Env, Profile};
use crate::join_root;
use crate::path::{Path, SysPath};

static SINGLETON: Once = Once::new();
static mut CONFIGS: Option<Mutex<Configs>> = None;

lazy_static::lazy_static! {
    pub static ref PROCESS_NAME: Mutex<String> = Mutex::new(String::from("LiA"));
}

pub fn set_process_name<T: AsRef<str>>(name: T) {
    let mut process_name = PROCESS_NAME.lock().unwrap();
    *process_name = name.as_ref().to_string();
}

pub fn get_process_name() -> String {
    let process_name = PROCESS_NAME.lock().unwrap();
    process_name.clone()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kinds {
    pub trace: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub on: bool,
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
        let profile: Profile = Env::get_profile();

        Configs {
            profile: Some(profile),
            ..config
        }
    }

    pub fn get_log() -> Log {
        let log = {
            let config = Configs::open().lock().unwrap();
            config.log().clone()
        };
        log.clone()
    }

    fn log(&self) -> &Log {
        &self.log
    }

    pub fn set_log(on: bool, save: bool, kinds: Option<Kinds>) {
        { // Unlock mutex guard before saving to file
            let mut config = Configs::open().lock().unwrap();
            config.log.on = on;
            config.log.save = save;
            if let  Some(kinds) = kinds {
                config.log.kinds = kinds;
            }
        }
        Configs::save_to_file();
    }

    fn save_to_file() {
        let config_path: SysPath = join_root!("configs.json");

        let config = Configs::open().lock().unwrap();
        let config_str = serde_json::to_string_pretty(&*config).unwrap();

        std::fs::write(config_path, config_str).unwrap();
    }

    pub fn get_save() -> bool {
        let save = {
            let config = Configs::open().lock().unwrap();
            config.save()
        };
        save
    }

    fn save(&self) -> bool { self.log.save }

    pub fn get_profile() -> Profile {
        let profile = {
            let config = Configs::open().lock().unwrap();
            config.profile().clone()
        };
        profile
    }

    fn profile(&self) -> &Profile {
        self.profile.as_ref().unwrap()
    }

    pub fn reload() {
        let config_path: SysPath = join_root!("configs.json");
        let content: String = std::fs::read_to_string(config_path).unwrap();
        let new_config: Configs = serde_json::from_str(&content).unwrap();

        let mut config_guard = Configs::open().lock().unwrap();
        *config_guard = new_config;
    }
    
}

#[cfg(test)]
mod tests {
    use crate::config::Configs;

    #[test]
    fn test_new() {
        drop(Configs::open().lock().unwrap());
    }

    #[test]
    fn test_get_log() {
        let _ = Configs::get_log();
        Configs::set_log(false, false, None);
        let log = Configs::get_log();
        assert_eq!(log.on, false);
        assert_eq!(log.save, false);
        // Reload from file
        Configs::reload();
        let log = Configs::get_log();
        assert_eq!(log.on, false);
        assert_eq!(log.save, false);
        Configs::set_log(true, true, None);
        assert!(Configs::get_log().on);
        assert!(Configs::get_log().save);
        // Reload from file
        Configs::reload();
        assert!(Configs::get_log().on);
        assert!(Configs::get_log().save);
    }
}
