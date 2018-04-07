use std::env;
use std::process::exit;

mod lib;
use lib::{do_file, Mint, SliceAsStrs};


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
