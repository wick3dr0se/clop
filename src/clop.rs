use std::env;

pub struct Opts {
    long: Vec<(String, Option<String>)>,
    short: Vec<(char, Option<String>)>,
}

pub fn get_opts() -> Opts {
    let mut options = Opts {
        long: vec![],
        short: vec![],
    };
    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--" => break,
            _ => {
                if arg.starts_with("--") {
                    let option = arg[2..].to_string();
                    let value = iter.clone().next().filter(|v| !v.starts_with("-")).cloned();
                    
                    if !options.long.iter().any(|(o, _)| o == &option) {
                        options.long.push((option, value));
                    }
                } else if arg.starts_with("-") {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    let last_char = chars.last().unwrap();
                    let value = iter.clone().next().filter(|v| !v.starts_with("-")).cloned();

                    for c in &chars[..chars.len() - 1] {
                        if !options.short.iter().any(|(o, _)| o == c) {
                            options.short.push((*c, None));
                        }
                    }
                    
                    if !options.short.iter().any(|(o, _)| o == last_char) {
                        options.short.push((*last_char, value));
                    }
                }
            }
        }
    }

    options
}