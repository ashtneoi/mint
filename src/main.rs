use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

fn exit_with_usage() -> ! {
    println!("Usage: mint TMPLNAME [NAME=VAL ...]");
    exit(2);
}

fn take2<I, T>(x: &mut I) -> Option<(T, T)>
where
    I: Iterator<Item = T>,
{
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

    println!("environ:");
    for (name, val) in environ.iter() {
        println!("{} = {:?}", name, val);
    }
    println!();

    Ok(environ)
}

struct StrFindAll<'a, 'b> {
    s: &'a str,
    pat: &'b str,
    start: usize,
}

impl<'a, 'b> Iterator for StrFindAll<'a, 'b> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.s[self.start..].find(self.pat) {
            Some(pos) => {
                let ret = Some(self.start + pos);
                self.start += pos + self.pat.len();
                ret
            },
            None => None,
        }
    }
}

fn str_find_all<'a, 'b>(s: &'a str, pat: &'b str) -> StrFindAll<'a, 'b> {
    StrFindAll { s, pat, start: 0 }
}

fn main() {
    let mut args = env::args();

    if args.next().is_none() {
        eprintln!("What?");
        exit(1);
    }

    let tmpl_name = &args.next().unwrap_or_else(|| exit_with_usage());

    let environ_args_vec: Vec<String> = args.collect();
    let mut environ = args_to_environ(&environ_args_vec).unwrap_or_else(
        |_| exit_with_usage()
    );
    environ.insert("#top_name", &tmpl_name);

    let tf = File::open(tmpl_name).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });
    let tb = io::BufReader::new(tf);

    let lines: Vec<String> = tb.lines()
        .collect::<Result<_, _>>()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });

    let open_pat = "{{";
    let close_pat = "}}";
    for (row, line) in lines.iter().enumerate() {
        let first = true;
        for open_pos in str_find_all(line, open_pat) {
            println!("{} {}", row, open_pos);
        }
    }
}
