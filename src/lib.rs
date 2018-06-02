pub mod rell {
    extern crate colored;
    extern crate termios;

    pub use self::colored::*;
    use self::termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};
    use std::collections::HashMap;
    use std::error::Error;
    use std::io::*;
    use std::result::Result;

    type KeyFunction = Fn(&String, &mut bool) -> Result<(), Box<Error>>;
    type Key<'a> = (Color, &'a KeyFunction);

    pub struct Rell<'a> {
        line: String,
        curpos: u64,
        prompt: String,
        renderer: &'a Fn(&mut Rell, &char) -> Result<(), Box<Error>>,
        keywords: HashMap<String, Key<'a>>,
        run: bool,
    }

    impl<'a> Rell<'a> {
        pub fn def_render(r: &mut Rell, buf: &char) -> Result<(), Box<Error>> {
            print!("\r{} ", r.prompt);
            r.line.push(*buf);

            let mut oldi = 0;
            let mut coldone = 0;
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
                            print!("{}", part.red());
                        }
                    }
                    oldi = i;
                    cache.push(b' ');
                    coldone = 1;
                    break;
                }
            }

            let mut part = &r.line[oldi..];
            if cache.len() > 0 {
                print!(" ");
                part = &r.line[oldi + 1..];
            }

            let word = part.trim();

            if coldone == 0 {
                match r.keywords.get(word) {
                    Some(s) => {
                        print!("{}", part.color(s.0));
                    }
                    _ => {
                        print!("{}", part.red());
                    }
                }
            } else {
                print!("{}", &part);
            }

            stdout().flush()?;
            Ok(())
        }

        pub fn def_exit(_line: &String, r: &mut bool) -> Result<(), Box<Error>> {
            *r = false;
            Ok(())
        }

        pub fn def_help(_line: &String, _r: &mut bool) -> Result<(), Box<Error>> {
            println!("Welcome to help!");
            Ok(())
        }

        pub fn def_unimpl(line: &String, _r: &mut bool) -> Result<(), Box<Error>> {
            println!(
                "{} not implemented yet!",
                line.split_whitespace().next().unwrap().bold()
            );
            Ok(())
        }

        pub fn new(prompt: &str) -> Rell {
            Rell {
                line: String::new(),
                curpos: 0,
                prompt: String::from(prompt),
                renderer: &Rell::def_render,
                keywords: HashMap::new(),
                run: true,
            }
        }

        pub fn add_keyword(&mut self, keyword: &String, col: Color, func: &'a KeyFunction) {
            self.keywords.insert(keyword.clone(), (col, func));
        }

        pub fn input(&mut self) -> Result<(), Box<Error>> {
            print!("\n{} ", self.prompt);
            stdout().flush()?;

            let termios = Termios::from_fd(0).unwrap();
            let mut new_termios = termios.clone(); // make a mutable copy of termios
                                                   // that we will modify
            new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
            tcsetattr(0, TCSANOW, &mut new_termios).unwrap();

            while self.run {
                let mut buf = [0; 1];
                stdin().read_exact(&mut buf).unwrap();

                //print!("{}", buf[0] as char);
                //stdout().flush();

                (self.renderer)(self, &(buf[0] as char))?;

                if buf[0] == '\n' as u8 {
                    let c = self.line.clone();
                    let word = c.split_whitespace().next().unwrap();

                    match self.keywords.get(word) {
                        Some(s) => (s.1)(&self.line, &mut self.run),
                        _ => {
                            print!(
                                "{} No such command : {}",
                                "[Error]".red().bold(),
                                word.bold()
                            );
                            Ok(())
                        }
                    }?;

                    if self.run {
                        print!("\n{} ", self.prompt);
                        stdout().flush()?;
                        self.line.clear();
                    }
                }

                //self.size = 0;
                //println!("Looping..");
            }

            tcsetattr(0, TCSANOW, &termios).unwrap();

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
