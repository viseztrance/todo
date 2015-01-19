use std::io::fs::PathExtensions;
use std::io::{File, Open, ReadWrite};

pub struct List {
    file: File,
    pub entries: String
}

impl List {
    pub fn new(path: Path) -> List {
        let file = if path.exists() {
            match File::open_mode(&path, Open, ReadWrite) {
                Ok(file) => file,
                Err(reason) => panic!("Couldn't open file `{}` for writing: {}", path.display(), reason.desc)
            }
        } else {
            File::create(&path).unwrap()
        };

        List {
            file: file,
            entries: "Testing".to_string()
        }
    }

    pub fn index(&self, context: Option<String>) {
        println!("listing!");
    }

    pub fn add(&self, context: Option<String>) {
        println!("adding!");
    }

    pub fn edit(&self, context: Option<String>) {
        println!("editting!");
    }

    pub fn remove(&self, context: Option<String>) {
        println!("removing!");
    }

    pub fn finish(&self, context: Option<String>) {
        println!("finishing!");
    }

    pub fn count(&self, context: Option<String>) {
        println!("counting!");
    }
}
