use std::io::fs::PathExtensions;
use std::io::{File, Open, ReadWrite};
use std::ascii::AsciiExt;
use todo::entry::Entry;

pub struct List {
    file: File,
    entries: Vec<Entry>
}

impl List {
    pub fn new(path: Path) -> List {
        let file = if path.exists() {
            match File::open_mode(&path, Open, ReadWrite) {
                Ok(result) => result,
                Err(reason) => panic!("Couldn't open file `{}` for writing: {}", path.display(), reason.desc)
            }
        } else {
            File::create(&path).unwrap()
        };

        List {
            file: file,
            entries: vec![]
        }
    }

    pub fn load(&mut self) {
        let text = &self.file.read_to_string().unwrap();
        let mut i = 0;
        for line in text.lines() {
            i += 1;
            &self.entries.push(Entry::new(i, line.to_string()));
        }
    }

    pub fn save(&mut self) {
        &self.file.write("Hi!".as_bytes());
    }

    pub fn index(&self, context: Option<String>) {
        for entry in (&self.entries).iter().filter(|current| List::filter(current, &context)) {
            println!("({}) {}", entry.id, entry.content);
        }
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
        let total = (&self.entries).iter().filter(|current| List::filter(current, &context)).count();
        println!("{}", total);
    }

    fn filter(entry: &&Entry, context: &Option<String>) -> bool {
        match context.clone() {
            Some(val) => val.eq_ignore_ascii_case(entry.status.as_slice()),
            None => true
        }
    }
}