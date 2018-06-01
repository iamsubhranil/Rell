extern crate rell;
extern crate colored;

use rell::rell::Rell;
use colored::Color;
use std::result::Result;
use std::error::Error;
use std::io;

fn myfn(line: &String, _r: &mut bool) -> Result<(), Box<Error>>  {
    print!("\nGiven arguments : ");
    for (_i, item) in line.split_whitespace().enumerate() {
        print!("{} ", item);
    };

    Ok(())
}

fn efn(_line: &String, _r: &mut bool) -> Result<(), Box<Error>> {
    Err(Box::from(io::Error::from(io::ErrorKind::InvalidInput)))
}

fn main(){
    
    let mut r = Rell::new(">>");
    r.add_keyword(&String::from("help"), Color::Green, &Rell::def_help);
    r.add_keyword(&String::from("load"), Color::Blue, &myfn);
    r.add_keyword(&String::from("exit"), Color::BrightRed, &Rell::def_exit);
    r.add_keyword(&String::from("error"), Color::Red, &efn);
    if let Err(e) = r.input() {
        println!("REPL closed due to the following error :\n{}", e);
    }

}
