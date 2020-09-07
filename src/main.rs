use std::io;
use json;
use std::fs;
use regex::Regex;

fn main() {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Err(error) => {
            println!("error: {}", error);
            return;
        },
        _=>()
    };
    let str_config = match fs::read_to_string("config.json") {
        Ok(s) => s,
        Err(e) => panic!("unable to read config.json")
    };

    let js_config = match json::parse(&str_config) {
        Ok(j) => j,
        Err(e) => panic!("unable to parse config.json tp json")
    };
    let height:usize = js_config["spec"]["height"].as_usize().expect("unable to get parameter config.spec.height");
    let new_line = js_config["spec"]["new_line"].as_str().expect("unable to get parameter config.spec.new_line");
    let mut lines=Vec::<String>::new();
    lines.resize_with(height, Default::default);
    for ch in input.chars() {
        for seq in js_config["sequences"].entries() {
            let r = Regex::new(seq.0).unwrap();
            if r.is_match(&String::from(ch)) {
                let letter = (seq.1).as_str().unwrap().to_string();
                let ls_letter: Vec<&str> = letter.split(new_line).collect();
                for (i, line) in ls_letter.iter().enumerate() {
                    lines[i]+=line;
                }
                // for ls_letter.1
                
                
                break;
            }
        }
    }
    for line in &mut lines {
        for ch_conv in js_config["char_conv"].entries() {
            let r = Regex::new(ch_conv.0).unwrap();
            *line = r.replace_all(&line, ch_conv.1.as_str().unwrap()).to_string();
        }
        println!("{}",line);
    }
}
