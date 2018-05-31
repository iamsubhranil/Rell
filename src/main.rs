extern crate rell;
extern crate colored;

use rell::rell::Rell;
use colored::Color;
use std::result::Result;
use std::error::Error;

fn myfn(line: &String) -> Result<(), Box<Error>> {
    print!("\nGiven arguments : ");
    for (_i, item) in line.split_whitespace().enumerate() {
        print!("{} ", item);
    };

    Ok(())
}

fn main(){
    
    let mut r = Rell::new(">>");
    r.add_keyword(&String::from("help"), Color::Green, &Rell::def_help);
    r.add_keyword(&String::from("load"), Color::Blue, &myfn);
    if let Err(e) = r.input() {
        println!("REPL closed due to the following error :\n{}", e);
    }

}
