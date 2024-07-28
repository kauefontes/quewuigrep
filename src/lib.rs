use std::{env, error::Error, fs};

/// Runs the search based on the provided configuration.
/// 
/// # Arguments
/// 
/// * `config` - A `Config` struct containing the query, filename, and case sensitivity flag.
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if successful, or an error if something goes wrong.
/// 
/// # Examples
/// 
/// ```
/// use quewuigrep::{Config, run};
/// use std::env;
/// 
/// let args: Vec<String> = vec!["program".into(), "query".into(), "filename.txt".into()];
/// let config = Config::new(&args).unwrap();
/// run(config).unwrap();
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

/// Holds the configuration for the search.
/// 
/// # Fields
/// 
/// * `query` - The string to search for.
/// * `filename` - The name of the file to search in.
/// * `case_sensitive` - A flag indicating whether the search should be case-sensitive.
/// 
/// # Examples
/// 
/// ```
/// use quewuigrep::Config;
/// use std::env;
/// 
/// let args: Vec<String> = vec!["program".into(), "query".into(), "filename.txt".into()];
/// let config = Config::new(&args).unwrap();
/// assert_eq!(config.query, "query");
/// assert_eq!(config.filename, "filename.txt");
/// assert!(config.case_sensitive);
/// ```
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Creates a new `Config` instance from command-line arguments.
    /// 
    /// # Arguments
    /// 
    /// * `args` - An iterator over the command-line arguments.
    /// 
    /// # Returns
    /// 
    /// * `Result<Config, &'static str>` - Returns a `Config` instance if successful, or an error message if not.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use quewuigrep::Config;
    /// use std::env;
    /// 
    /// let args: Vec<String> = vec!["program".into(), "query".into(), "filename.txt".into()];
    /// let config = Config::new(&args).unwrap();
    /// assert_eq!(config.query, "query");
    /// assert_eq!(config.filename, "filename.txt");
    /// assert!(config.case_sensitive);
    /// ```
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename!"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Searches for the query string in the contents, case-sensitive.
/// 
/// # Arguments
/// 
/// * `query` - The string to search for.
/// * `contents` - The contents of the file to search in.
/// 
/// # Returns
/// 
/// * `Vec<&str>` - A vector of lines that contain the query string.
/// 
/// # Examples
/// 
/// ```
/// use quewuigrep::search;
/// 
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// 
/// let result = search(query, contents);
/// assert_eq!(result, vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for the query string in the contents, case-insensitive.
/// 
/// # Arguments
/// 
/// * `query` - The string to search for.
/// * `contents` - The contents of the file to search in.
/// 
/// # Returns
/// 
/// * `Vec<&str>` - A vector of lines that contain the query string, ignoring case.
/// 
/// # Examples
/// 
/// ```
/// use quewuigrep::search_case_insensitive;
/// 
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
/// 
/// let result = search_case_insensitive(query, contents);
/// assert_eq!(result, vec!["Rust:", "Trust me."]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape. ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}