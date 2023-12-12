use std::collections::HashMap;

struct Database {
    data: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

fn main() {
    let mut db = Database::new();

    db.insert("key1".to_string(), "value1".to_string());
    db.insert("key2".to_string(), "value2".to_string());

    println!("{:?}", db.get("key1"));
    println!("{:?}", db.get("key2"));

    db.remove("key1");

    println!("{:?}", db.get("key1"));
    println!("{:?}", db.get("key2"));
}