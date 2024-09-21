#[macro_export]
macro_rules! defer {
    ($($body:tt)*) => {
        use crate::defer::Defer;
        let _defer = Defer::new(|| {
            $($body)*
        });
    };
}

#[macro_export]
macro_rules! deferable {
    ($value:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($value))
    };
}

pub struct Defer<F: FnOnce()> {
    f: Option<F>,
}

#[allow(dead_code)]
impl<F: FnOnce()> Defer<F> {
    pub fn new(f: F) -> Self {
        Defer { f: Some(f) }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}

pub trait Deferable<T> {
    fn get(&self) -> std::cell::Ref<T>;
    fn get_mut(&self) -> std::cell::RefMut<T>;
    fn set(&self, value: T);
}

impl<T> Deferable<T> for std::rc::Rc<std::cell::RefCell<T>> {
    fn get(&self) -> std::cell::Ref<T> {
        self.borrow()
    }

    fn get_mut(&self) -> std::cell::RefMut<T> {
        self.borrow_mut()
    }

    fn set(&self, value: T) {
        *self.borrow_mut() = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defer;

    #[test]
    fn test_defer_with_deferable_i32() {

        let y = deferable!(0);
        defer!({
            println!("Defer block executed.");
            *y.get_mut() = 1;
            println!("y = {}", *y.get());
        });
        assert_eq!(*y.get(), 0);
        println!("Test block executed.");
        println!("y = {}", *y.get());
    }

    #[test]
    fn test_defer_with_deferable_string() {
        use std::rc::Rc;
        let x = deferable!(String::from("Hello"));
        let y = Rc::clone(&x);
        defer!({
            println!("Defer block executed.");
            y.get_mut().push_str(", world!");
            println!("y = {}", *y.get());
        });
        assert_eq!(*x.get(), "Hello");
        println!("Test block executed.");
        println!("x = {}", *x.get());
    }

    #[test]
    fn test_borrow_in_defer() {
        let x = deferable!(0);
        defer!({
            println!("Defer block executed.");
            *x.get_mut() = 1;
        });
        println!("Test block executed.");
        println!("x = {}", *x.get());
    }
}
