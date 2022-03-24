#![allow(dead_code, non_snake_case)]

mod block;
mod blockchain;
mod cli;
mod transaction;

#[macro_use]
extern crate log;

pub type Result<T> = std::result::Result<T, failure::Error>;

// use std::pin::Pin;
// use blockchain::*;
// use std::thread::sleep;
// use std::time::Duration;
use crate::cli::Cli;
use env_logger::Env;
fn main() {
    // println!("Hello, world!");
    // let mut bc = Blockchain::new();
    // sleep(Duration::from_millis(10));
    // bc.add_block(String::from("Send 1 BTC to Ivan"))?;
    // sleep(Duration::from_millis(30));
    // bc.add_block(String::from("Send 2 more BTC to Ivan"))?;

    // println!("Blockchain: {:#?}", bc);
    // Ok(())
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    let mut cli = Cli::new();
    // cli.run()?;
    if let Err(e) = cli.run() {
        println!("Error: {}", e);
    }

    // Ok(())
    // println!("////////////////////////////////////////////////////////////");
    // let p = Person {
    //     name: String::from("xiaoming"),
    //     age: 22,
    // };
    // println!(
    //     "{} {} {} {} {}",
    //     p.get_age(),
    //     p.get_age1(),
    //     Person::get_age2(&p),
    //     Box::new(&p).get_age3(),
    //     Person::get_age3(Box::new(&p))
    //     // Person::get_age4(&p),
    //     // Person::get_age5(33),
    // );
}

// struct Person {
//     name: String,
//     age: i32,
// }

// impl Person {
//     fn get_age(&self) -> i32 {
//         self.age + 0
//     }

//     fn get_age1(&self) -> i32 {
//         self.age + 1
//     }

//     fn get_age2(&self) -> i32 {
//         self.age + 2
//     }

//     fn get_age3(self: Box<&Self>) -> i32 {
//         // match self {
//         //     Some(a) => a.age + 12,
//         //     _ => 101,
//         // }
//         self.age + 8
//     }

//     // fn get_age4(&self) -> i32 {
//     //     self.age + 4
//     // }

//     // fn get_age5(age: i32) -> i32 {
//     //     age
//     // }
// }

// // impl Future for Person {}
