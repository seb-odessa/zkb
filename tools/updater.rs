extern crate crossbeam;
use std::io;
use std::io::BufRead;
use std::{thread, time};
use crossbeam::crossbeam_channel::bounded;


fn main() {
    println!("Usage:");
    let (s, r) = bounded::<String>(4);
    for tid in 0..4 {
        let recv = r.clone();
        thread::spawn(move || {
            for line in recv.iter() {
                let timeout = time::Duration::from_millis(10);
                print!("{:?}: {}\n", &tid, line);
                thread::sleep(timeout);
            }
        }
        );
    }

    let reader = io::BufReader::new(io::stdin());
    for line in reader.lines() {
        s.send(line.unwrap()).unwrap();
    }

}
