mod benchmark;

mod path;
pub use path::{SysPath, Path};

mod logger;
pub use logger::Logger;

mod config;
pub use config::{set_process_name, get_process_name};

mod env;
pub use env::config::Config;

pub mod defer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defer::Deferable;

    #[test]
    fn test_defer() {
        let x = deferable!(0);
        defer!({
            x.set(1);
            assert_eq!(*x.get(), 1);
        });
        assert_eq!(*x.get(), 0);
    }

    #[test]
    fn test_deferable() {
        let x = deferable!(1);
        assert_eq!(*x.get(), 1);
        assert_eq!(*x.get_mut(), 1);
    }
}
