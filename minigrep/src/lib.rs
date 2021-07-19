use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_unsensitive(&config.query, &content)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|lines| lines.contains(query))
        .collect()
}

pub fn search_case_unsensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|lines| lines.to_lowercase().contains(&query))
        .collect()

}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
pick three.
Ductape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_unsensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_unsensitive(query, content)
        );
    }
}
