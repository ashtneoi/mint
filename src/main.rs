use std::collections::HashMap;
use std::env;
use std::process::exit;


fn print_usage() {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
}


fn main() {
    let mut args = env::args();

    if args.next().is_none() {
        eprintln!("What?");
        exit(1);
    }

    let tmpl_name = match args.next() {
        None => { print_usage(); exit(2); },
        Some(n) => n,
    };

    let mut environ = HashMap::<String, String>::new();
    environ.insert("#top_name".to_string(), tmpl_name.clone());
    loop {
        let mut pair = match args.next() {
            None => break,
            Some(p) => p,
        };
        let i = match pair.find('=') {
            None => { print_usage(); exit(2); },
            Some(i) => i,
        };
        let val = pair.split_off(i + 1);
        let mut name = pair;
        name.pop().unwrap();
        environ.insert(name, val);
    }

    for (name, val) in &environ {
        println!("{} = \"{}\"", name, val);
    }
}
