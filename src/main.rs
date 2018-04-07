use std::collections::HashMap;
use std::env;
use std::process::exit;

mod lib;
use lib::{do_file, SliceAsStrs, take2};

fn exit_with_usage() -> ! {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
    exit(2);
}

fn args_to_environ<'a>(args: &[&'a str]) -> Option<HashMap<&'a str, &'a str>> {
    let mut environ = HashMap::<&str, &str>::new();
    for pair in args {
        let (name, val) = take2(&mut pair.splitn(2, '='))?;
        environ.insert(name, val);
    }
    Some(environ)
}

struct Mint<'a> {
    tmpl_name: &'a str,
    environ: HashMap<&'a str, &'a str>,
}

impl<'a> Mint<'a> {
    fn with_args(args: &[&'a str]) -> Mint<'a> {
        let tmpl_name = args.get(0).unwrap_or_else(|| exit_with_usage());
        let environ_args: Vec<&str> = args[1..].to_vec();
        let environ = args_to_environ(&environ_args).unwrap_or_else(
            || exit_with_usage()
        );
        Mint { tmpl_name, environ }
    }
}

fn main() {
    let mut args_iter = env::args();
    if args_iter.next().is_none() {
        eprintln!("What?");
        exit(1);
    }
    let args: Vec<String> = args_iter.collect();
    let mut m = Mint::with_args(&args.as_strs());

    match do_file(m.tmpl_name, &mut m.environ) {
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
