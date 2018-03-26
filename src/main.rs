use std::collections::HashMap;
use std::env;
use std::process::exit;


fn print_usage() {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
}


fn args_to_environ(args_vec: &Vec<String>) -> Result<HashMap<&str, &str>, ()> {
    let mut environ = HashMap::<&str, &str>::new();

    for pair in args_vec {
        let splat: Vec<&str> = pair.splitn(2, '=').collect();
        if splat.len() != 2 {
            return Err(());
        }
        let (name, val) = (splat[0], splat[1]);
        environ.insert(name, val);
    }

    Ok(environ)
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
    let mut environ = match args_to_environ(&environ_args_vec) {
        Err(_) => { print_usage(); exit(2); },
        Ok(e) => e,
    };
    environ.insert("#top_name", &tmpl_name);

    for (name, val) in &environ {
        println!("{} = \"{}\"", name, val);
    }
}
