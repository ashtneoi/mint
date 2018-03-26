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

    let environ_args_vec: Vec<String> = args.collect();
    let mut environ_args = environ_args_vec.iter();

    let mut environ = HashMap::<&str, &str>::new();
    environ.insert("#top_name", &tmpl_name);
    loop {
        let pair = match environ_args.next() {
            None => break,
            Some(p) => p,
        };

        let splat: Vec<&str> = pair.splitn(2, '=').collect();
        if splat.len() != 2 {
            print_usage();
            exit(2);
        }
        let (name, val) = (splat[0], splat[1]);
        environ.insert(name, val);
    }

    for (name, val) in &environ {
        println!("{} = \"{}\"", name, val);
    }
}
