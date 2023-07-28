use std::env;
use std::process;
use std::io::{Result};
use std::path::PathBuf;

//time functions
extern crate time;
extern crate chrono;
use chrono::{DateTime,Utc};
use std::time::{SystemTime, UNIX_EPOCH};


//shell commands


//lib.rs
use gnostr_command::Config;

//main.rs functions
fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

//debug
#[cfg(debug_assertions)]

fn example() {

    //println!("Debugging enabled");
    //println!("cwd={:?}",get_current_working_dir());

}
#[cfg(not(debug_assertions))]
fn example() {

    //println!("Debugging disabled");
    //println!("cwd={:?}",get_current_working_dir());

}

fn main() -> Result<()> {

    let start = time::get_time();
    let _epoch = get_epoch_ms();
    let system_time = SystemTime::now();
    let _datetime: DateTime<Utc> = system_time.into();

    let args: Vec<String> = env::args().collect();
    let dirname = &args[0];

    if cfg!(debug_assertions) {
        //println!("Debugging enabled");
    } else {
        //println!("Debugging disabled");
    }

    #[cfg(debug_assertions)]
    //println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    //println!("Debugging disabled");

    example();


    let config = Config::build(&args).unwrap_or_else(|_err| {
        println!("Usage: gnostr-command <string> <file>");
        process::exit(1);
    });

    println!("Searching in {}", dirname);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = gnostr_command::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    let _duration = time::get_time() - start;
    Ok(())

}
