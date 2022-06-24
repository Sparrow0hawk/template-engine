use regex::{Captures, Regex};
use std::{collections::HashMap, fmt};

// a custom type for our hashmap
// essentially specifying what types the `value` section of
// the HashMap `key`:`value` can have
// we specify rust types that these custom types map too
enum Data {
    Number(i32),
    Boolean(bool),
    Text(String),
}

// here we outline how the data types can be represented as strings
// to aid with rendering our template
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Text(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Number(x) => write!(f, "{}", x),
        }
    }
}

// an outline of our expected render function
/// render function that renders template using HashMap data
fn render(mut template: String, mut data: HashMap<&str, Data>) -> String {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
