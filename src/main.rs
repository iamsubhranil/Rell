extern crate colored;
extern crate rell;

use colored::Color;
use rell::rell::Rell;
use std::error::Error;
use std::io;
use std::result::Result;

fn myfn(r: &mut Rell) -> Result<(), Box<Error>> {
    print!("Given arguments : ");
    for (_i, item) in r.line.split_whitespace().enumerate() {
        print!("{} ", item);
    }

    Ok(())
}

fn efn(_r: &mut Rell) -> Result<(), Box<Error>> {
    Err(Box::from(io::Error::from(io::ErrorKind::InvalidInput)))
}

fn main() {
    let mut r = Rell::new(">>");
    r.add_keyword(
        String::from("help"),
        Color::Green,
        &Rell::def_help,
        String::from("Show help"),
    );
    r.add_keyword(
        String::from("load"),
        Color::Blue,
        &myfn,
        String::from("Load a program from memory"),
    );
    r.add_keyword(
        String::from("exit"),
        Color::Yellow,
        &Rell::def_exit,
        String::from("Exit from the shell"),
    );
    r.add_keyword(
        String::from("error"),
        Color::Green,
        &efn,
        String::from("Throw an error"),
    );
    r.add_keyword(
        String::from("run"),
        Color::Green,
        &Rell::def_unimpl,
        String::from("Test an unimplemented function"),
    );
    if let Err(e) = r.input() {
        println!("REPL closed due to the following error :\n{}", e);
    }
}
