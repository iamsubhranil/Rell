pub mod rell {
    extern crate colored;
    extern crate termios;

    pub use self::colored::*;
    use self::termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};
    use std::collections::HashMap;
    use std::error::Error;
    use std::io::*;
    use std::result::Result;

    type RellKeyFunction = Fn(&mut Rell);
    type RellKey<'a> = (Color, &'a RellKeyFunction, String);

    pub struct Rell<'a> {
        pub line: String,
        pub curpos: u64,
        pub prompt: String,
        pub renderer: &'a Fn(&mut Rell, &char) -> Result<(), Box<Error>>,
        pub keywords: HashMap<String, RellKey<'a>>,
        pub run: bool,
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

        pub fn def_exit(r: &mut Rell) {
            r.run = false;
        }

        pub fn def_help(r: &mut Rell) {
            println!("Welcome to help!");
            println!("{}", "Keywords".bold());
            print!("========");
            for (kw, ac) in r.keywords.iter() {
                print!("\n{}\t-- {}", kw, ac.2);
            }
        }

        pub fn def_unimpl(r: &mut Rell) {
            print!(
                "{} is not implemented yet!",
                r.line.split_whitespace().next().unwrap().bold()
            );
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

        pub fn add_keyword(
            &mut self,
            keyword: String,
            col: Color,
            func: &'a RellKeyFunction,
            desc: String,
        ) {
            self.keywords.insert(keyword, (col, func, desc));
        }

        fn def_func(_r: &mut Rell) {}

        fn get_func(&mut self) -> &'a RellKeyFunction {
            if self.line.eq("\n") {
                return &Rell::def_func;
            }
            let word = self.line.split_whitespace().next().unwrap_or_default();
            if word.len() == 0 {
                return &Rell::def_func;
            }
            match self.keywords.get(word) {
                Some(s) => return s.1,
                _ => {
                    print!(
                        "{} No such command : {}",
                        "[Error]".red().bold(),
                        (&word).bold()
                    );
                    return &Rell::def_func;
                }
            };
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
                    let func = Rell::get_func(self);
                    func(self);

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
