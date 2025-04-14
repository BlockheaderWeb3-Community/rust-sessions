use std::env;
use std::fs;

// struct to query a search into a file
struct Config {
    query: String,     // key
    file_name: String, // file to search from
    ignore_case: bool, // case sensitive search
}

fn main() {
    let user_arg: Vec<String> = env::args().collect();
    let config = Config::build(user_arg);
    run(config);
}

/// fn to run query search output
fn run(config: Result<Config, String>) {
    match config {
        Ok(data) => {
            let content = fs::read_to_string(&data.file_name).expect("data not foundd");

            //println!("env arg {}", data);
            let result = if data.ignore_case {
                case_sensitive_query_search(data.query, &content)
            } else {
                query_search(data.query, &content)
            };
            match result {
                Ok(value) => {
                    println!("{:#?}", value);
                }
                Err(e) => {
                    println!("{e}")
                }
            }
        }
        Err(error) => {
            println!("{error}")
        }
    }
}

impl Config {
    fn build(user_arg: Vec<String>) -> Result<Config, String> {
        if user_arg.len() < 3 {
            Err("argument not complete".to_string())
        } else {
            let query = &user_arg[1];
            let file_name = &user_arg[2];

            // to run a case sensitive search run IGNORE_CASE=1 cargo run -- to poem.txt
            // if IGNORE_CASE exist in env the variable will be true else false
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query: query.clone(),
                file_name: file_name.clone(),
                ignore_case,
            })
        }
    }
}

// non case sensitive search
fn query_search<'a>(key: String, content: &'a str) -> Result<Vec<&'a str>, String> {
    let mut storage = Vec::new();
    if content.contains(&key) {
        for line in content.lines() {
            if line.contains(&key) {
                storage.push(line);
            }
        }
        Ok(storage)
    } else {
        Err("Error: key does not exist".to_string())
    }
}

// case sensitive search
fn case_sensitive_query_search<'a>(key: String, content: &'a str) -> Result<Vec<&'a str>, String> {
    let mut storage = Vec::new();
    // println!("{} the key {}",content.to_lowercase(),&key.to_lowercase());
    if content.to_lowercase().contains(&key.to_lowercase()) {
        for line in content.lines() {
            if line.to_lowercase().contains(&key.to_lowercase()) {
                storage.push(line);
            }
        }
        Ok(storage)
    } else {
        //println!("hello");
        Err("Error: key does not exist".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_query_search() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        let search = query_search(query.to_string(), contents).unwrap();
        assert_eq!(vec!["safe, fast, productive."], search)
    }

    #[test]

    fn test_query_search_fail() {
        let query = "to";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        let search = query_search(query.to_string(), contents);
        assert_eq!(Err("Error: key does not exist".to_string()), search)
    }

    #[test]

    fn test_case_sensitive_query_search() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let search = case_sensitive_query_search(query.to_string(), contents).unwrap();
        assert_eq!(vec!["safe, fast, productive."], search)
    }

    #[test]

    fn test_case_sensitive_query_search_fail() {
        let query = "india";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let search = case_sensitive_query_search(query.to_string(), contents);
        assert_eq!(Err("Error: key does not exist".to_string()), search)
    }
}
