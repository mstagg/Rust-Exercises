use std::env;
use std::error::Error;
use std::fs;

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.path)?;
    let results = if args.case_insensitive {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    for r in results {
        println!("{r}");
    }

    Ok(())
}

pub fn search<'a>(search_term: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|x| x.contains(search_term))
        .collect::<Vec<&str>>()
}

pub fn search_case_insensitive<'a>(search_term: &str, text: &'a str) -> Vec<&'a str> {
    let search_term_lower = search_term.to_lowercase();
    text.lines()
        .filter(|x| x.to_lowercase().contains(&search_term_lower))
        .collect::<Vec<&str>>()
}

pub struct Args {
    pub query: String,
    pub path: String,
    pub case_insensitive: bool,
}

impl Args {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        // discard first value, it is the program name, not a program argument
        args.next();

        let query = match args.next() {
            Some(a) => a,
            None => return Err("Invalid or Missing Query String"),
        };
        let path = match args.next() {
            Some(a) => a,
            None => return Err("Invalid or Missing Path String"),
        };
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Args {
            query,
            path,
            case_insensitive,
        })
    }
}

#[cfg(test)]
mod search {
    use super::*;

    #[test]
    fn happy_path() {
        let search_term = "duck";
        let file_contents = "\
My favorite animal is a duck.\nIt goes \"quack quack\"!";

        assert_eq!(
            search(search_term, file_contents),
            vec!["My favorite animal is a duck."]
        );
    }

    #[test]
    fn no_results() {
        let search_term = "duck";
        let file_contents = "\
My favorite animal is a dog.\nIt goes \"woof woof\"!";

        assert_eq!(search(search_term, file_contents), Vec::<&str>::new());
    }
}

#[cfg(test)]
mod search_case_insensitive {
    use super::*;

    #[test]
    fn happy_path() {
        let search_term = "Mario";
        let file_contents = "\
My favorite name is mario.\nIt is from a game on nintendo!";

        assert_eq!(
            search_case_insensitive(search_term, file_contents),
            vec!["My favorite name is mario."]
        );
    }

    #[test]
    fn no_results() {
        let search_term = "Mario";
        let file_contents = "\
My favorite name is luigi.\nIt is from a game on nintendo!";

        assert_eq!(search(search_term, file_contents), Vec::<&str>::new());
    }
}
