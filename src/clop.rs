use std::env;

pub struct Opts {
    pub long: Vec<(String, Option<String>)>,
    pub short: Vec<(String, Option<String>)>,
    pub leftover: Vec<String>,
}

pub fn get_opts() -> Opts {
    let mut options = Opts {
        long: vec![],
        short: vec![],
        leftover: vec![],
    };
    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter().skip(1).peekable();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--" => break,
            _ => {
                if arg.starts_with("--") {
                    let option = arg[2..].to_string();
                    let value = iter.peek().filter(|v| !v.starts_with("-")).cloned();
                    
                    if value.is_some() {
                        iter.next();
                    }

                    if !options.long.iter().any(|(o, _)| o == &option) {
                        options.long.push((option, value.cloned()));
                    }
                } else if arg.starts_with("-") {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let last_char = chars.last().unwrap().to_string();
                    let value = iter.peek().filter(|v| !v.starts_with("-")).cloned();

                    if value.is_some() {
                        iter.next();
                    }

                    for c in &chars[..chars.len() - 1] {
                        if !options.short.iter().any(|(o, _)| *o == c.to_string()) {
                            options.short.push((c.to_string(), None));
                        }
                    }

                    if !options.short.iter().any(|(o, _)| *o == last_char) {
                        options.short.push((last_char, value.cloned()));
                    }
                } else {
                    options.leftover.push(arg.to_string());
                }
            }
        }
    }

    options
}

impl Opts {
    pub fn has(&self, options: &[&str], argument: Option<&str>) -> bool {
        for option in options {
            if option.len() > 1 {
                if self.long.iter().any(|(o, a)| o == option && a.as_deref() == argument) {
                    return true;
                }
            } else {
                if self.short.iter().any(|(o, a)| o == option && a.as_deref() == argument) {
                    return true;
                }
            }
        }
        false
    }
}
