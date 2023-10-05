use std::error::Error;
use std::process;
use std::process::Command;
use std::fs;

use crate::process::Output;

    static repo_root : String = std::env::args().nth(1).unwrap_or(".".to_string());
    const repo_path : Output  =
        if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "cd"])
                .output()
                .expect("failed to execute process")
        } else
        if cfg!(target_os = "macos"){
        Command::new("sh")
                .arg("-c")
                .arg("pwd")
                .output()
                .expect("failed to execute process")
        } else
        if cfg!(target_os = "linux"){
        Command::new("sh")
                .arg("-c")
                .arg("pwd")
                .output()
                .expect("failed to execute process")
        } else {
        Command::new("sh")
                .arg("-c")
                .arg("pwd")
                .output()
                .expect("failed to execute process")
        };

    static path : String = String::from_utf8(repo_path.stdout)
    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
    .unwrap();
    //println!("path={:?}", path);




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

    // Setup for cargo-binstall additional gnostr-command utilities
    let install:       String = String::from("install");// we dont require --install
                                                        // TODO: test gnostr command   install
                                                        // TODO: test gnostr command --install

    let gnostr_cat:       String = String::from("gnostr-cat");
    let gnostr_cli:       String = String::from("gnostr-cli");
    let gnostr_command:   String = String::from("gnostr-command"); //update gnostr-command
    let gnostr_grep:      String = String::from("gnostr-grep");
    let gnostr_legit:     String = String::from("gnostr-legit");
    let gnostr_sha256:    String = String::from("gnostr-sha256");

    if args.len() > 1 {
    let content = String::from(&args[1].clone());
      for arg in args.iter() {

          #[cfg(debug_assertions)]
          println!("arg=apple:{}", arg.eq(&apple));
          #[cfg(debug_assertions)]
          println!("arg=banana:{}", arg.eq(&banana));

          if args.len() == 3 {// gnostr-command install gnostr-*
                              // must be args.len() == 3
            if content.eq(&install) {
              #[cfg(debug_assertions)]
              println!("exec install sub argparse {:?}!", arg);
              let content = String::from(&args[2].clone());
              if content.eq(&gnostr_cat) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-cat install {:?}!", args);

              }
              if content.eq(&gnostr_cli) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-cli install {:?}!", args);

              }
              if content.eq(&gnostr_command) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-command install {:?}!", args);

              }
              if content.eq(&gnostr_grep) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-grep install {:?}!", args);

              }
              if content.eq(&gnostr_legit) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-legit install {:?}!", args);

              }
              if content.eq(&gnostr_sha256) {
                //#[cfg(debug_assertions)]
                println!("exec gnostr-sha256 install {:?}!", args);

              }
            }// end if content.eq(&install)
          }// end if args.len() == 3

          //reinitialize content as args[1]
          let content = String::from(&args[1].clone());

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

    if args.len() == 3 {

      // intercept return if
      // gnostr-comand install gnostr-*
      let query = args[1].clone();
      let file_path = args[2].clone();

      //if query.contains(&install) {
      if query.contains(&install) {

          //#[cfg(debug_assertions)]
          println!("arg=query:{}", query);
          //#[cfg(debug_assertions)]
          println!("arg=install:{}", install);
          //#[cfg(debug_assertions)]
          println!("query=install:{}", query.eq(&install));

          // intercept return if
          // gnostr-comand install gnostr-*
          Ok(Config { query, file_path })

      } else {

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })

      }

    } else { process::exit(0); }

    }
}
/// pub fn run(config: Config) -> Result<(), Box\<dyn Error>>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;
    // println!("config.file_path:contents={}", contents);
    if contents.contains("nostr") {

      // cargo install cargo-binstall
      // cargo-binstall contents string
      println!("___________gnostr___________:contents={}", contents);

      Ok(())

    } else {

        println!("contents={}", contents);
        println!("&config.query={}", &config.query);
        for line in search(&config.query, &contents) {

            println!("{line}");

        }

    Ok(())

    }

}



/// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let q = fs::read_to_string(query);
    println!("_________q________={:?}", q);

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
