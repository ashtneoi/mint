use std::collections::HashMap;
use std::env;
use std::process::exit;


fn print_usage() {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
}


fn take2<I, T>(x: &mut I) -> Option<(T, T)>
where I: Iterator<Item = T> {
    let x1 = x.next()?;
    let x2 = x.next()?;
    Some((x1, x2))
}


fn unwrap_else<T, E>(p: Option<T>, e: E) -> Result<T, E> {
    match p {
        None => Err(e),
        Some(s) => Ok(s),
    }
}


fn args_to_environ(args_vec: &Vec<String>) -> Result<HashMap<&str, &str>, ()> {
    let mut environ = HashMap::<&str, &str>::new();

    for pair in args_vec {
        let (name, val) = unwrap_else(take2(&mut pair.splitn(2, '=')), ())?;
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
