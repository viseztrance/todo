use regex::Regex;

pub struct Entry {
    id: isize,
    content: String,
    pub status: String,
    pub color: Option<String>
}

impl Entry {
    pub fn new(id: isize, data: String) -> Entry {
        let expr    = regex!(r"(^\[(\w+)\]? (.*?) ?#?([:xdigit:]+)?$)?");
        let capture = expr.captures(data.as_slice()).unwrap();
        let status  = capture.at(2).unwrap_or("PENDING").to_string();
        let content = capture.at(3).unwrap_or(data.as_slice()).to_string();
        let color   = match capture.at(4) {
            Some(val) => Some(val.to_string()),
            None => None
        };

        Entry {
            id: id,
            content: content,
            status: status,
            color: color
        }
    }

    pub fn display(&self) -> String {
        format!("({}) {}", &self.id, &self.content)
    }
}