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

    for line in do_file(tmpl_name, &environ) {
        println!("{}", line);
    }
}

fn do_file(tmpl_name: &str, environ: &HashMap<&str, &str>) -> Vec<String> {
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

    do_lines(&lines, environ)
}

fn do_lines(lines: &Vec<String>, environ: &HashMap<&str, &str>)
    -> Vec<String>
{
    static OPEN_PAT: &str = "{{";
    static CLOSE_PAT: &str = "}}";

    let mut lines2 = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        // Some(to, from) or None (end)
        let mut replace = vec![Some((0, 0))];

        for open_pos in str_find_all(line, OPEN_PAT) {
            let close_pos = str_find_at(line, open_pos, CLOSE_PAT)
                .unwrap_or_else(|| {
                    eprintln!("{}: Missing \"{}\"", row, CLOSE_PAT);
                    exit(1);
                });
            replace.push(
                Some((open_pos, close_pos + CLOSE_PAT.len()))
            );
        }

        replace.push(None);

        let mut line2 = "".to_string();

        for window in replace.windows(2) {
            let (prev, this) = (window[0], window[1]);
            let (_, prev_to) = prev.unwrap();

            if let Some((from, to)) = this {
                line2.push_str(&line[prev_to..from]);

                let name_from = from + OPEN_PAT.len();
                let name_to = to - CLOSE_PAT.len();
                let name = &line[name_from..name_to];
                let val = environ.get(name)
                    .unwrap_or_else(|| {
                        eprintln!("{},{}: {} is not defined", row, from, name);
                        exit(1);
                    });
                line2.push_str(val);
            } else {
                line2.push_str(&line[prev_to..]);
            }
        }

        lines2.push(line2);
    }

    lines2
}
