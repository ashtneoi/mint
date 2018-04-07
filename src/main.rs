use std::env;
use std::process::exit;

mod lib;
use lib::{do_file, Mint, SliceAsStrs};

#[cfg(test)]
mod tests;

fn exit_with_usage() -> ! {
    println!("Usage: mint TMPLPATH [NAME=VAL ...]");
    exit(2);
}

fn main() {
    let mut args_iter = env::args();
    if args_iter.next().is_none() {
        eprintln!("What?");
        exit(1);
    }
    let args: Vec<String> = args_iter.collect();
    let m = Mint::with_args(&args.as_strs()).unwrap_or_else(
        || exit_with_usage()
    );

    match do_file(m.tmpl_name, &m.environ) {
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
