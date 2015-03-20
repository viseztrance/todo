use std::str::FromStr;
use std::ascii::AsciiExt;

#[derive(PartialEq, Debug)]
pub enum State { PENDING, COMPLETED }

impl FromStr for State {
    type Err = ();

    fn from_str(value: &str) -> Result<State, ()> {
        match value.to_ascii_lowercase().as_slice() {
            "pending" => Ok(State::PENDING),
            _ => Ok(State::COMPLETED),
        }
    }
}

impl State {
    pub fn to_string(&self) -> String {
        match self {
            &State::COMPLETED => "completed".to_string(),
            &State::PENDING => "pending".to_string()
        }
    }
}

pub struct Entry {
    pub id: usize,
    pub content: String,
    pub status: State,
    pub color: Option<String>
}

impl Entry {
    pub fn new(id: usize, data: String) -> Entry {
        let expr    = regex!(r"(^\[(\w+)\]? (.*?)(\s+)?(#([:xdigit:]+))?$)?");
        let capture = expr.captures(data.as_slice()).unwrap();
        let status  = capture.at(2).unwrap_or("pending").to_string().parse::<State>().unwrap();
        let content = capture.at(3).unwrap_or(data.as_slice()).to_string();
        let color   = match capture.at(6) {
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

    pub fn update(&mut self, content: String) {
        self.content = content;
    }

    pub fn display(&self) -> String {
        format!("({}) {}", &self.id, &self.content)
    }

    pub fn to_data(&self) -> String {
        let mut buffer = format!("[{}] {}", &self.status.to_string(), &self.content);
        if (&self.color).is_some() {
            let color = &self.color.clone().unwrap();
            buffer.push_str(format!(" #{}", color).as_slice());
        }
        buffer
    }
}