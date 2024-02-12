use std::io;
use std::env;

pub struct Opts {
    pub long: Vec<(String, Option<String>)>, // holds our long options; implemented by has()
    pub short: Vec<(String, Option<String>)>, // ^ but short
    pub scrap: Vec<String> // leftover, because we don't parse arguments; we parse options
}

impl Opts {
    pub fn has(&mut self, options: &[&str], arg: bool) -> io::Result<String> {
        for option in options {
            // long option; we don't specifiy multiple character short options (-ab)
            if option.len() > 1 {
                for (o, a) in &self.long {
                    if o == option && a.is_some() && arg {
                        // remove long option and arg from scrap
                        self.scrap.retain(|s| *s != format!("--{}", o) && *s != a.clone().unwrap());
                        return Ok(a.clone().unwrap())
                    } else if o == option {
                        if arg {
                            return Err(io::Error::new(io::ErrorKind::Other, ""))
                        }
                        
                        // remove long option from scrap
                        self.scrap.retain(|s| *s != format!("--{}", o));
                        return Ok(String::new())
                    }
                }
            // short options
            } else {
                for (o, a) in &self.short {
                    if o == option && a.is_some() && arg {
                        // remove short option and arg from scrap
                        self.scrap.retain(|s| *s != format!("-{}", o) && *s != a.clone().unwrap());
                        return Ok(a.clone().unwrap())
                    } else if o == option {
                        if arg {
                            return Err(io::Error::new(io::ErrorKind::Other, ""))
                        }
                        
                        // remove short option from scrap
                        self.scrap.retain(|s| *s != format!("-{}", o));
                        return Ok(String::new())
                    }
                }
            }
        }

        return Err(io::Error::new(io::ErrorKind::Other, ""))
    }
}

pub fn get_opts() -> Opts {
    let mut options = Opts {
        long: vec![],
        short: vec![],
        scrap: vec![]
    };
    // get command line arguments without program invocation
    let args: Vec<String> = env::args().skip(1).collect();
    options.scrap = args.clone(); // populate scrap (arguments) to be stripped of options
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() { // a fancy iterator so we can skip iterations
        match arg.as_str() {
            "--" => {
                options.scrap.retain(|s| s != "--"); // remove -- from scrap
                break;
            },
            _ => {
                if arg.starts_with("--") {
                    let option = arg[2..].to_string(); // strip -- from option
                    // if next argument isn't an option set it to argument
                    let argument = iter.peek().filter(|a| !a.starts_with("-")).cloned();
                    
                    if argument.is_some() {
                        iter.next();
                    }

                    // if option not already in long push into long with argument
                    if !options.long.iter().any(|(o, _)| o == &option) {
                        options.long.push((option, argument.cloned()));
                    }
                } else if arg.starts_with("-") {
                    let chars: Vec<char> = arg[1..].chars().collect(); // stip - from option
                    // get last char from combined for argument
                    let last_char = chars.last().unwrap().to_string();
                    // if next argument isn't an option set it to argument
                    let argument = iter.peek().filter(|v| !v.starts_with("-")).cloned();

                    if argument.is_some() {
                        iter.next();
                    }

                    // if short option is combined, iterate the character except last and push them into short without argument
                    for c in &chars[..chars.len() - 1] {
                        if !options.short.iter().any(|(o, _)| *o == c.to_string()) {
                            options.short.push((c.to_string(), None));
                        }
                    }

                    // if last char or only char if not combined; push option and argument if exists
                    if !options.short.iter().any(|(o, _)| *o == last_char) {
                        options.short.push((last_char, argument.cloned()));
                    }
                }
            }
        }
    }

    options
}