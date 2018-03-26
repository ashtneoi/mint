use std::collections::HashMap;
use std::env;
use std::process::exit;


fn exit_with_usage() -> ! {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
    exit(2);
}


fn take2<I, T>(x: &mut I) -> Option<(T, T)>
where I: Iterator<Item = T> {
    let x1 = x.next()?;
    let x2 = x.next()?;
    Some((x1, x2))
}


fn args_to_environ(args_vec: &Vec<String>) -> Result<HashMap<&str, &str>, ()> {
    let mut environ = HashMap::<&str, &str>::new();

    for pair in args_vec {
        let (name, val) = take2(&mut pair.splitn(2, '=')).ok_or(())?;
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

    let tmpl_name = args.next().unwrap_or_else(|| exit_with_usage());

    let environ_args_vec: Vec<String> = args.collect();
    let mut environ = args_to_environ(&environ_args_vec).unwrap_or_else(
        |_| exit_with_usage()
    );
    environ.insert("#top_name", &tmpl_name);

    for (name, val) in &environ {
        println!("{} = \"{}\"", name, val);
    }
}
