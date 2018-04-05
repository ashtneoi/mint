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

fn str_find_at(s: &str, start: usize, pat: &str) -> Option<usize> {
    s[start..].find(pat).and_then(|i| Some(start + i))
}

struct StrFindAll<'a, 'b> {
    s: &'a str,
    pat: &'b str,
    start: usize,
}

impl<'a, 'b> Iterator for StrFindAll<'a, 'b> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        str_find_at(self.s, self.start, self.pat)
            .and_then(|pos| {
                self.start = pos + self.pat.len();
                Some(pos)
            })
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

    do_file(tmpl_name, &environ);
}

fn do_file(tmpl_name: &str, environ: &HashMap<&str, &str>) {
    let tf = File::open(tmpl_name).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });
    let tb = io::BufReader::new(tf);

    let lines: Vec<String> = tb.lines()
        .collect::<Result<_, _>>() // TODO: understand this magic
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });

    do_lines(&lines, environ);
}

fn do_lines(lines: &Vec<String>, environ: &HashMap<&str, &str>) {
    // (row, from, to)
    let mut replace = Vec::new();

    let open_pat = "{{";
    let close_pat = "}}";
    for (row, line) in lines.iter().enumerate() {
        let first = true;
        for open_pos in str_find_all(line, open_pat) {
            let close_pos = str_find_at(line, open_pos, close_pat)
                .unwrap_or_else(|| {
                    eprintln!("{}: Missing \"{}\"", row, close_pat);
                    exit(1);
                });
            replace.push((row, open_pos, close_pos + close_pat.len()));
        }
    }

    for (row, from, to) in replace {
        let name_from = from + open_pat.len();
        let name_to = to - close_pat.len();
        let name = &lines[row][name_from..name_to];
        if let Some(val) = environ.get(name) {
            println!("{}: {}-{} ({} = {:?})", row, from, to, name, val);
        } else {
            eprintln!("{},{}: {} is not defined", row, from, name);
        }
    }
}
