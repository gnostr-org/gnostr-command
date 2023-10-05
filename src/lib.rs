//! ## Introduction
//!
//! Hypercore protocol is a streaming, message based protocol. This is a rust port of the wire
//! protocol implementation in [the original Javascript version][holepunch-hypercore] aiming
//! for interoperability with LTS version.
//!
//! This crate is built on top of the [hypercore] crate, which defines some structs used here.
//!
//! ## Design
//!
//! This crate does not include any IO related code, it is up to the user to supply a streaming IO
//! handler that implements the [AsyncRead] and [AsyncWrite] traits.
//!
//! When opening a Hypercore protocol stream on an IO handler, the protocol will perform a Noise
//! handshake followed by libsodium's [crypto_secretstream] to setup a secure and authenticated
//! connection. After that, each side can request any number of channels on the protocol. A
//! channel is opened with a [Key], a 32 byte buffer. Channels are only opened if both peers
//! opened a channel for the same key. It is automatically verified that both parties know the
//! key without transmitting the key itself.
//!
//! On a channel, the predefined messages, including a custom Extension message, of the Hypercore
//! protocol can be sent and received.
//!
//! ## Features
//!
//! ### `sparse` (default)
//!
//! When using disk storage for hypercore, clearing values may create sparse files. On by default.
//!
//! ### `async-std` (default)
//!
//! Use the async-std runtime, on by default. Either this or `tokio` is mandatory.
//!
//! ### `tokio`
//!
//! Use the tokio runtime. Either this or `async_std` is mandatory.
//!
//! ### `wasm-bindgen`
//!
//! Enable for WASM runtime support.
//!
//! ### `cache`
//!
//! Use a moka cache for hypercore's merkle tree nodes to speed-up reading.
//!
//! ## Example
//!
//! The following example opens a TCP server on localhost and connects to that server. Both ends
//! then open a channel with the same key and exchange a message.
//!
//! ```no_run
//! # async_std::task::block_on(async {
//! use hypercore_protocol::{ProtocolBuilder, Event, Message};
//! use hypercore_protocol::schema::*;
//! use async_std::prelude::*;
//! // Start a tcp server.
//! let listener = async_std::net::TcpListener::bind("localhost:8000").await.unwrap();
//! async_std::task::spawn(async move {
//!     let mut incoming = listener.incoming();
//!     while let Some(Ok(stream)) = incoming.next().await {
//!         async_std::task::spawn(async move {
//!             onconnection(stream, false).await
//!         });
//!     }
//! });
//!
//! // Connect a client.
//! let stream = async_std::net::TcpStream::connect("localhost:8000").await.unwrap();
//! onconnection(stream, true).await;
//!
//! /// Start Hypercore protocol on a TcpStream.
//! async fn onconnection (stream: async_std::net::TcpStream, is_initiator: bool) {
//!     // A peer either is the initiator or a connection or is being connected to.
//!     let name = if is_initiator { "dialer" } else { "listener" };
//!     // A key for the channel we want to open. Usually, this is a pre-shared key that both peers
//!     // know about.
//!     let key = [3u8; 32];
//!     // Create the protocol.
//!     let mut protocol = ProtocolBuilder::new(is_initiator).connect(stream);
//!
//!     // Iterate over the protocol events. This is required to "drive" the protocol.
//!
//!     while let Some(Ok(event)) = protocol.next().await {
//!         eprintln!("{} received event {:?}", name, event);
//!         match event {
//!             // The handshake event is emitted after the protocol is fully established.
//!             Event::Handshake(_remote_key) => {
//!                 protocol.open(key.clone()).await;
//!             },
//!             // A Channel event is emitted for each established channel.
//!             Event::Channel(mut channel) => {
//!                 // A Channel can be sent to other tasks.
//!                 async_std::task::spawn(async move {
//!                     // A Channel can both send messages and is a stream of incoming messages.
//!                     channel.send(Message::Want(Want { start: 0, length: 1 })).await;
//!                     while let Some(message) = channel.next().await {
//!                         eprintln!("{} received message: {:?}", name, message);
//!                     }
//!                 });
//!             },
//!             _ => {}
//!         }
//!     }
//! }
//! # })
//! ```
//!
//! Find more examples in the [Github repository][examples].
//!
//! [holepunch-hypercore]: https://github.com/holepunchto/hypercore
//! [datrs-hypercore]: https://github.com/datrs/hypercore
//! [AsyncRead]: futures_lite::AsyncRead
//! [AsyncWrite]: futures_lite::AsyncWrite
//! [examples]: https://github.com/datrs/hypercore-protocol-rs#examples

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

mod builder;
mod channels;
mod constants;
mod crypto;
mod duplex;
mod message;
mod protocol;
mod reader;
mod util;
mod writer;

/// The wire messages used by the protocol.
pub mod schema;

pub use builder::Builder as ProtocolBuilder;
pub use channels::Channel;
// Export the needed types for Channel::take_receiver, and Channel::local_sender()
pub use async_channel::{
    Receiver as ChannelReceiver, SendError as ChannelSendError, Sender as ChannelSender,
};
pub use duplex::Duplex;
pub use hypercore; // Re-export hypercore
pub use message::Message;
pub use protocol::{Command, CommandTx, DiscoveryKey, Event, Key, Protocol};
pub use util::discovery_key;
use std::error::Error;
use std::process;
use std::fs;

#[derive(Debug)]
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
