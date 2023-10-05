use std::error::Error;
use std::process;
use std::fs;

/// pub struct Config
pub struct Config {
    pub query: String,
    pub file_path: String,
}
/// impl Config
impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

      //#[cfg(debug_assertions)]
      println!("args.len()={}",args.len());

      //let names = vec!["Bob", "Frank", "Ferris"];
      //for name in names.iter() {
      //    match name {
      //        &"Ferris" => println!("Ferris:There is a rustacean among us!"),
      //        &"Frank" => println!("Frank:There is a rustacean among us!"),
      //        &"Bob" => println!("Bob:There is a rustacean among us!"),
      //        // TODO ^ Try deleting the & and matching just "Ferris"
      //        _ => println!("Hello {}", name),
      //    }
      //}
      //// println!("names: {:?}", names);


    #[cfg(debug_assertions)]
    let apple: String = String::from("apple");
    #[cfg(debug_assertions)]
    let banana: String = String::from("banana");

    // true
    #[cfg(debug_assertions)]
    println!("{}", apple.ne(&banana));
    #[cfg(debug_assertions)]
    println!("{}", apple.eq(&banana));

    let _ferris: String = String::from("Ferris");
    let _frank:  String = String::from("Frank");
    let _bob:    String = String::from("Bob");

    if args.len() > 1 {
    let content = String::from(&args[1].clone());
      for arg in args.iter() {

          #[cfg(debug_assertions)]
          println!("arg=apple:{}", arg.eq(&apple));
          #[cfg(debug_assertions)]
          println!("arg=banana:{}", arg.eq(&banana));

          if content.eq(&_ferris) {
            println!("Matched {:?}!", arg);
            println!("Ferris:Hello {}", _ferris);
          }
          if content.eq(&_frank) {
            println!("Matched {:?}!", arg);
            println!("Frank:Hello {}", _frank);
          }
          if content.eq(&_bob) {
            println!("Matched {:?}!", arg);
            println!("Bob:Hello {}", _bob);
          }

      }//end for arg in args.iter()

    }

    if args.len() > 2 {
      let query = args[1].clone();
      let file_path = args[2].clone();
      Ok(Config { query, file_path })
    } else { process::exit(0); }

    }
}
/// pub fn run(config: Config) -> Result<(), Box\<dyn Error>>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    Ok(())
}
/// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            // do something with line
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
/// mod tests
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        /// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
