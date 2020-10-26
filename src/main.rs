use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use chrono::Utc;
use std::{io, thread};
use std::time::Duration;
use fs2::FileExt;
use std::io::Error;

fn main() {
    println!("Starting");

    let path = "/tmp/testfile.tst";
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .create(true)
    //     .open(path)
    //     .unwrap();

    let file = File::create(path).unwrap();

    println!("Trying to get lock - {}", Utc::now());


    for _ in 0..10 {
        match file.try_lock_exclusive() {
            Ok(_) => {
                println!("Got lock @ {}", Utc::now() );
                break;
            },
            Err(e) => {
                println!("failed to get lock @ {} - {:?}", Utc::now(), e);
                thread::sleep(Duration::from_secs(1));
            },
        }

    }

    println!("Finished lock - {}", Utc::now());

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    println!("Finished");
}
