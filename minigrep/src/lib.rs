use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query : String,
    pub file_path: String,
    pub ignore_case : bool,
}


impl Config {
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, Usage: command <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config{
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config : &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{}", contents);
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            println!("{}" , line);
            results.push(line);
        }
    }
    return results;
}


// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config(){

    }

    #[test]
    fn test_search(){
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\n";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_vec_iter() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_vec_iter_sum(){
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| {return x + 1;}).collect();
        assert_eq!(v2, vec![2,3,4]);
    }

    #[test]
    fn test_iter_filter(){
        let v1 = vec![1,2, 3];
        let v2 : Vec<_> = v1.into_iter().filter(|&x| {return x > 1;}).collect();
        assert_eq!(vec![2,3], v2);
    }
}