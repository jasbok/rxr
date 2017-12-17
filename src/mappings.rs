use serde::Serialize;
use serde_json;
use std::collections::HashMap;

pub struct Mappings {
    mappings: HashMap<String, String>,
}

impl Mappings {
    pub fn new() -> Mappings {
        Mappings {
            mappings: HashMap::new(),
        }
    }

    pub fn insert<K: ?Sized, V: ?Sized>(&mut self, key: &K, val: &V) -> &Mappings
    where
        K: Serialize,
        V: Serialize,
    {
        let key = serde_json::to_string(key).unwrap().replace("\"", "");
        let val = self.replace_all(val);
        self.mappings.insert(key, val);

        self
    }

    pub fn replace_all<T: ?Sized>(&self, val: &T) -> String
    where
        T: Serialize,
    {
        let mut result = serde_json::to_string(val).unwrap().replace("\"", "");

        for (key, val) in self.mappings.iter() {
            let key = String::from(format!("{{{}}}", key));
            result = result.replace(&key, &val);
        }

        result
    }
}
