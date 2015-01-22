use std::io::fs::PathExtensions;
use std::io::{File, Open, ReadWrite, BufferedReader};

pub struct List {
    file: File,
    entries: Vec<String>
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
            entries: vec!()
        }
    }

    pub fn load(&mut self) {
        let text = &self.file.read_to_string().unwrap();
        for line in text.lines() {
            println!("* {}", line);
        }
    }

    pub fn save(&mut self) {
        &self.file.write("Hi!".as_bytes());
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
