use std::collections::HashMap;
use std::env;
use std::process::exit;

mod lib;
use lib::{do_file, take2};

fn exit_with_usage() -> ! {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
    exit(2);
}

fn args_to_environ(args_vec: &Vec<String>) -> Option<HashMap<&str, &str>> {
    let mut environ = HashMap::<&str, &str>::new();
    for pair in args_vec {
        let (name, val) = take2(&mut pair.splitn(2, '='))?;
        environ.insert(name, val);
    }
    Some(environ)
}

fn main() {
    let mut args = env::args();
    if args.next().is_none() {
        eprintln!("What?");
        exit(1);
    }
    let tmpl_name = &args.next().unwrap_or_else(|| exit_with_usage());
    let environ_args_vec: Vec<String> = args.collect();
    let environ = args_to_environ(&environ_args_vec).unwrap_or_else(
        || exit_with_usage()
    );

    match do_file(tmpl_name, &environ) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        },
    }
}
