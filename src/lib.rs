

pub mod rell{
    extern crate termios;
    extern crate colored;

    use std::io::*; 
    use std::error::Error;
    use std::result::Result;
    use self::termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
    pub use self::colored::*;
    use std::collections::HashMap;

    type KeyFunction = Fn(&String) -> Result<(), Box<Error>>;
    type Key<'a> = (Color, &'a KeyFunction);

    pub struct Rell<'a>{
        line: String,
        curpos: u64,
        prompt: String,
        renderer: &'a Fn(&mut Rell, &char) -> Result<(), Box<Error>>,
        keywords: HashMap<String, Key<'a>>,
    }

    impl<'a> Rell<'a>{

        pub fn def_render(r: &mut Rell, buf: &char) -> Result<(), Box<Error>> {
            print!("\r{} ", r.prompt);

            r.line.push(*buf);

            let mut oldi = 0;
            let mut cache: Vec<u8> = Vec::new();
            cache.clear();

            for (i, part) in r.line.bytes().enumerate() {
                if part == b' ' {
                    cache.pop();
                    let part = &r.line[oldi..i];

                    let word = part.trim();

                    match r.keywords.get(word) {
                        Some(s) => {
                            print!("{}", part.color(s.0));
                        }
                        _ => {
                            print!("{}", &part);
                        }
                    }
                    oldi = i;
                    cache.push(b' ');
                }
            }

            let mut part = &r.line[oldi..];
            if cache.len() > 0 {
                print!(" ");
                part = &r.line[oldi+1..];
            }
            //let part = &line[oldi..];

            let word = part.trim();

            match r.keywords.get(word) {
                Some(s) => {
                    print!("{}", part.color(s.0));
                }
                _ => {
                    print!("{}", &part);
                }
            }

            stdout().flush()?;
            Ok(())
        }

        pub fn def_help(_line : &String) -> Result<(), Box<Error>>{
            println!("Welcome to help!");
            Ok(())
        }

        pub fn def_unimpl(line : &String) -> Result<(), Box<Error>>{
            println!("{} not implemented yet!", 
                     line.split_whitespace().next().unwrap()
                     .bold());
            Ok(())
        }

        pub fn new(prompt: &str) -> Rell {
            Rell { 
                line: String::new(), curpos: 0, 
                prompt: String::from(prompt), 
                renderer: &Rell::def_render,
                keywords: HashMap::new()
            }
        }

        pub fn add_keyword(&mut self, keyword: &String, col: Color, func: &'a KeyFunction){
            self.keywords.insert(keyword.clone(), (col, func));
        }

        pub fn input(&mut self) -> Result<(), Box<Error>>{
            print!("\n{} ", self.prompt);
            stdout().flush()?;

            let termios = Termios::from_fd(0).unwrap();
            let mut new_termios = termios.clone();  // make a mutable copy of termios
            // that we will modify
            new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
            tcsetattr(0, TCSANOW, &mut new_termios).unwrap();

            loop{

                let mut buf = [0;1];
                stdin().read_exact(&mut buf).unwrap();

                //print!("{}", buf[0] as char);
                //stdout().flush();

                (self.renderer)(self, &(buf[0] as char))?;

                if buf[0] == '\n' as u8 {
                    let c = self.line.clone();
                    let word = c.split_whitespace().next().unwrap();

                    match self.keywords.get(word) {
                        Some(s) => (s.1)(&self.line),
                        _ => break,
                    };

                    print!("\n{} ", self.prompt);
                    stdout().flush()?;
                    self.line.clear();
                }

                //self.size = 0;
                //println!("Looping..");
            }

            tcsetattr(0, TCSANOW, & termios).unwrap();

            Ok(())
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
