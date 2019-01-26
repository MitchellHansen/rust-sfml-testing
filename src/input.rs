use std::collections::HashSet;
use sfml::window::{Key, Event};


pub struct Input {
    held_keys: HashSet<Key>
}

impl Input {
    pub fn new() -> Input {

        let mut container = HashSet::new();

        Input {
            held_keys: container,
        }
    }

    pub fn is_held(&self, key: Key) -> bool{
        self.held_keys.contains(&key)
    }

    pub fn ingest(&mut self, event: &Event) {
        match event {
            Event::KeyPressed { code, .. } => {
                self.held_keys.insert(code.clone());
            }
            Event::KeyReleased { code, .. } => {
                self.held_keys.remove(code);
            }
            _ => {}
        }
    }
}
