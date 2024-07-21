#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Scoped<T>(T);

impl<T> Scoped<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn with<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        f(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Scoped;
    use std::sync::Mutex;

    #[test]
    fn test_mutex() {
        let mu = Scoped::new(Mutex::new(1));
        let output = mu.with(|mutex| {
            let guard = mutex.lock().unwrap();
            *guard + 1
        });
        assert_eq!(output, 2);
    }
}
