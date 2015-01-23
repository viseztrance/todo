pub struct Entry {
    pub id: isize,
    pub content: String
}

impl Entry {
    pub fn new(id: isize, content: String) -> Entry {
        Entry {
            id: id,
            content: content
        }
    }
}