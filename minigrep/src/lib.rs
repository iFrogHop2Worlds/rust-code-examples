use std::error::Error;
use std::fs;

pub struct Config {
    pub q: String,
    pub fp: String,
}

impl Config {
    pub fn build(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let q = &args[1].clone();
        let fp = &args[2].clone();
    
        Ok(Config { q: q.to_string(), fp: fp.to_string() })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.fp)?;

        for line in search(&config.q, &contents) {
            println!("{line}");
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
