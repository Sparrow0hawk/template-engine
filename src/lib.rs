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
fn render(mut template: String, mut data: HashMap<&str, Data>) -> String {
    // for the template engine we want to match items contained within
    // double curly braces `{{ item }}` in the template and replace it with
    // our data
    // we unwrap here to get the value from the Result
    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();

    let repeat_regex =
        Regex::new(r"\{% repeat (\d*?) times %\}((.|\n)*?)\{% endrepeat %\}").unwrap();

    let if_else_reg = Regex::new(
        r"\{% if (.*?) %\}((.|\n)*?)(\{% else %\}((.|\n)*?)\{% endif %\}|\{% endif %\})",
    )
    .unwrap();

    template = print_regex
        .replace_all(&template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();
            data[key].to_string()
        })
        .to_string();

    template = repeat_regex
        .replace_all(&template, |caps: &Captures| {
            // extract the number of times to repeat the `code`
            // the assumption here is that this value can
            // be coerced to a integer
            let times = caps.get(1).unwrap().as_str().trim();

            // the `code` within the repeat section
            let code = caps.get(2).unwrap().as_str().trim();

            // coerce times to usize type and repeat
            // `code` string `times` times
            code.repeat(times.parse::<usize>().unwrap())
        })
        .to_string();

    template = if_else_reg
        .replace_all(&template, |caps: &Captures| {
            let key = caps.get(1).unwrap().as_str().trim();

            let if_code = caps.get(2).unwrap().as_str().trim();

            let else_code = caps.get(5).map_or("", |m| m.as_str()).trim();

            if let Data::Boolean(exp) = data[key] {
                if exp {
                    if_code.to_string()
                } else {
                    else_code.to_string()
                }
            } else {
                "ERROR PARSING KEY".to_string()
            }
        })
        .to_string();

    template
}

#[cfg(test)]
mod tests {
    use crate::{render, Data};
    use assert_str::assert_str_trim_eq;
    use std::collections::HashMap;

    #[test]
    fn test_template() {
        let test_data = HashMap::from([("hello", Data::Text("Hello world!".to_string()))]);
        let input = std::fs::read_to_string("dist/hello.html").expect(
            "
        Something went wrong reading this file!",
        );

        let result = "<!DOCTYPE html>
        <html>
        
        <body>
            <h1>Hello world!</h1>
        </body>
        
        </html>
        "
        .to_string();

        assert_str_trim_eq!(render(input, test_data), result);
    }

    #[test]
    fn test_repeat() {
        let input = "{% repeat 4 times %}hello{% endrepeat %}".to_string();
        let data = HashMap::from([("hello", Data::Text("Hello world!".to_string()))]);

        assert_eq!(render(input, data), "hellohellohellohello".to_string())
    }

    #[test]
    fn test_if() {
        let input_if = "{% if test %}hello world!{% endif %}".to_string();
        let data = HashMap::from([
            ("test", Data::Boolean(true)),
            ("false_test", Data::Boolean(false)),
        ]);

        assert_eq!(render(input_if, data), "hello world!".to_string());
    }

    #[test]
    fn test_else() {
        let input_else = "{% if false_test %}Magic{% else %}Unicorn{% endif %}".to_string();
        let data = HashMap::from([
            ("test", Data::Boolean(true)),
            ("false_test", Data::Boolean(false)),
        ]);

        assert_eq!(render(input_else, data), "Unicorn".to_string());
    }

    #[test]
    fn test_nokey() {
        let input_nokey = "{% if badkey %}amazing{% endif %}".to_string();
        let data = HashMap::from([
            ("test", Data::Boolean(true)),
            ("false_test", Data::Boolean(false)),
        ]);

        assert_eq!(render(input_nokey, data), "ERROR PARSING KEY".to_string())
    }
}
