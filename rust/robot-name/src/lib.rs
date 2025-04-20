use std::cell::RefCell;
use std::collections::HashSet;

thread_local! (
    static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new())
);

pub struct Robot {
    name: String,
}

impl Robot {
    fn get_new_name() -> String {
        let new_name = format!(
            "{}{:03}",
            (0..2)
                .map(|_| {
                    let c = rand::random::<u8>() % 26 + b'A';
                    c as char
                })
                .collect::<String>(),
            rand::random::<u32>() % 1000
        );
        new_name
    }

    fn get_available_name() -> String {
        let mut name = Self::get_new_name();
        USED_NAMES.with(|used_names| {
            let mut used_names = used_names.borrow_mut();
            while used_names.contains(&name) {
                name = Self::get_new_name();
            }
            used_names.insert(name.clone());
        });
        name
    }

    pub fn new() -> Self {
        Self {
            name: Self::get_available_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::get_available_name();
    }
}
