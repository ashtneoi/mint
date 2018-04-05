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

    Ok(environ)
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

    let mut tags: Vec<(usize, usize, usize)> = Vec::new();

    let open_pat = "{{";
    let close_pat = "}}";
    for (row, line) in lines.iter().enumerate() {
        let mut first = true;
        let mut chunk_start: usize = 0;
        for chunk in line.split(open_pat) {
            if first {
                first = false;
            } else {
                if !chunk.starts_with('!') {
                    let tag_end_rel = chunk.find(close_pat).unwrap_or_else(|| {
                        eprintln!("{}: Missing \"{}\"", row + 1, close_pat);
                        exit(1);
                    });
                    let tag_start = chunk_start - open_pat.len();
                    let tag_end = chunk_start + tag_end_rel + close_pat.len();
                    tags.push((row, tag_start, tag_end));
                }
            }
            chunk_start += chunk.len() + open_pat.len();
        }
    }

    for (row, col_from, col_to) in tags {
        println!("{} {}", row, &lines[row][col_from..col_to]);
    }
}
