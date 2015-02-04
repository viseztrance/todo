pub struct Query {
    pub action: String,
    pub scope: QueryScope
}

impl Query {
    pub fn new(mut args: Vec<String>) -> Query {
        let action = args.remove(0);
        Query { action: action, scope: QueryScope::new(args) }
    }
}

pub struct QueryScope {
    value: Option<Vec<String>>
}

impl QueryScope {
    pub fn new(args: Vec<String>) -> QueryScope {
        let value = if args.len() == 0 {
            None
        } else {
            Some(args)
        };
        QueryScope { value: value }
    }

    pub fn to_string(&self) -> Option<String> {
        match (&self).value.clone() {
            Some(val) => {
                let parsed_value = val.iter()
                                      .fold(String::new(), |a, b| a + b.as_slice() + " ")
                                      .trim_right()
                                      .to_string();
                Some(parsed_value)
            },
            None => None
        }
    }

    pub fn to_vec_int(&self) -> Option<Vec<usize>> {
        match (&self).value.clone() {
            Some(val) => {
                let parsed_value: Vec<usize> = val.iter()
                                                  .filter_map(|e| e.parse::<usize>())
                                                  .collect();
                if parsed_value.len() > 0 {
                    Some(parsed_value)
                } else {
                    None
                }
            },
            None => None
        }
    }
}