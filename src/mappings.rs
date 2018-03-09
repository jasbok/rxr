use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let key = String::from(key.as_ref()).replace("\"", "");
        let val = self.replace_all(val.as_ref());
        self.mappings.insert(key, val);

        self
    }

    pub fn replace_all<T: ?Sized>(&self, val: &T) -> String
    where
        T: AsRef<str>,
    {
        let mut result = String::from(val.as_ref()).replace("\"", "");

        for (key, val) in &self.mappings {
            result = result.replace(&format!("{{{}}}", key), val);
        }

        result
    }

    pub fn replace(&self, val: &mut String) -> () {
        for (mkey, mval) in &self.mappings {
            *val = val.replace(&format!("{{{}}}", mkey), mval);
        }
    }

    pub fn replace_vec(&self, values: &mut Vec<String>) -> () {
        for val in values.iter_mut() {
            for (mkey, mval) in &self.mappings {
                *val = val.replace(&format!("{{{}}}", mkey), mval);
            }
        }
    }

    pub fn replace_map(&self, values: &mut HashMap<String, String>) -> () {
        for val in values.values_mut() {
            for (mkey, mval) in &self.mappings {
                *val = val.replace(&format!("{{{}}}", mkey), mval);
            }
        }
    }
}
