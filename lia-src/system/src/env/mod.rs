pub mod config;

use crate::path::SysPath;
use std::sync::Mutex;

trait Env {
    fn get<'a>() -> &'a Mutex<Self>;
    fn new(path: SysPath) -> Self;
    fn set_env(path: &SysPath) -> ();
    fn read_env() -> Self;
}
