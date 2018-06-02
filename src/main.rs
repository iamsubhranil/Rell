extern crate colored;
extern crate rell;

use colored::*;
use rell::rell::Rell;

fn myfn(r: &mut Rell) {
    print!("Given arguments : ");
    for (_i, item) in r.line.split_whitespace().enumerate() {
        print!("{} ", item);
    }
}

fn efn(r: &mut Rell) {
    print!(
        "{} Terminating due to an unrecoverable error!\n",
        "[Error]".red().bold()
    );
    r.run = false;
}

fn change_prompt(r: &mut Rell) {
    let mut sws = r.line.split_whitespace();
    match sws.nth(1) {
        Some(s) => r.prompt = s.to_string(),
        _ => print!("{} Enter a new sign to set as prompt!", "Error".bold()),
    }
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
    r.add_keyword(
        String::from("prompt"),
        Color::Green,
        &change_prompt,
        String::from("Change the prompt"),
    );
    if let Err(e) = r.input() {
        println!("REPL closed due to the following error :\n{}", e);
    }
}
