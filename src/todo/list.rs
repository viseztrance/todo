use std::io;
use std::io::fs::PathExtensions;
use std::io::fs;
use std::io::File;
use std::ascii::AsciiExt;
use todo::entry::Entry;

pub struct List {
    path: Path,
    entries: Vec<Entry>
}

impl List {
    pub fn new(path: Path) -> List {
        List {
            path: path,
            entries: vec![]
        }
    }

    pub fn load(&mut self) {
        if !(&self.path).exists() {
            &self.touch();
        }
        let text = File::open(&self.path).read_to_string().unwrap();
        let mut i = 0;
        for line in text.lines() {
            i += 1;
            &self.entries.push(Entry::new(i, line.to_string()));
        }
    }

    pub fn save(&mut self) {
        let data = (&self.entries).iter()
                                  .map(|e| e.to_data())
                                  .fold(String::new(), |a, b| a + b.as_slice() + "\n");
        fs::unlink(&self.path);
        &self.touch().write(data.as_bytes());
    }

    pub fn index(&self, context: Option<String>) {
        for entry in (&self.entries).iter().filter(|current| List::filter(current, &context)) {
            println!("{}", entry.display());
        }
    }

    pub fn add(&mut self, context: Option<String>) {
        let content = match context {
            Some(val) => val,
            None => {
                println!("Write a description for your task:");
                io::stdin().read_line().unwrap()
            }
        };
        let entry = Entry::new((&self.entries).len() + 1, content);
        &self.entries.push(entry);
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

    fn touch(&self) -> File {
        File::create(&self.path).unwrap()
    }

    fn filter(entry: &&Entry, context: &Option<String>) -> bool {
        match context.clone() {
            Some(val) => val.eq_ignore_ascii_case(entry.status.as_slice()),
            None => true
        }
    }
}