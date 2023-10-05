use std::env;
use std::process;
use std::io::{Result};

/// time functions
extern crate time;
extern crate chrono;
use chrono::{DateTime,Utc};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(debug_assertions)]
use std::path::PathBuf;

/// shell commands


/// lib.rs
use gnostr_command::Config;

/// main.rs functions
/// get_epoch_ms
fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
#[cfg(debug_assertions)]
/// fn get_current_working_dir()
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

/// debug
/// #[cfg(debug_assertions)]
#[cfg(debug_assertions)]
/// fn example()
fn example() {

    //println!("Debugging enabled");
    //println!("cwd={:?}",get_current_working_dir());

}
/// #[cfg(not(debug_assertions))]
/// fn example()
#[cfg(not(debug_assertions))]
fn example() {

    //println!("Debugging disabled");
    //println!("cwd={:?}",get_current_working_dir());

}

/// fn main() -> Result<()>
fn main() -> Result<()> {

let start = time::get_time();
let _epoch = get_epoch_ms();
let _system_time = SystemTime::now();
let _datetime: DateTime<Utc> = _system_time.into();
let cwd = get_current_working_dir();

#[cfg(debug_assertions)]
        println!("start={:#?}", start);
        println!("_epoch={:#?}", _epoch);
        println!("_system_time={:#?}", _system_time);
        println!("_datetime={:#?}", _datetime);
        println!("cwd={:#?}", cwd);

let args: Vec<String> = env::args().collect();
let dirname = &args[0];

let config = Config::build(&args).unwrap_or_else(|_err| {
    println!("Usage: gnostr-command <string> <file>");
    process::exit(1);
});

if cfg!(debug_assertions) {

    #[cfg(debug_assertions)]
    println!("Debugging enabled");
    println!("dirname={}", dirname);
    println!("config.query={}", config.query);
    println!("config.file_path={}", config.file_path);

} else {

    //#[cfg(not(debug_assertions))]
    //println!("Debugging disabled");

}

example();

if let Err(e) = gnostr_command::run(config) {
    println!("Application error: {e}");
    process::exit(1);
}

let _duration = time::get_time() - start;
Ok(())
}//end main
