
use regex::{Regex, Captures};
use std::{fmt, collections::HashMap};


// an outline of our expected render function
fn render(mut template: String, mut data: HashMap<&str, Data>) -> String {
    /// render function that renders template using HashMap data
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
