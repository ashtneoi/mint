use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn take2<I, T>(x: &mut I) -> Option<(T, T)>
where
    I: Iterator<Item = T>,
{
    let x1 = x.next()?;
    let x2 = x.next()?;
    Some((x1, x2))
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

pub trait SliceAsStrs {
    fn as_strs(&self) -> Vec<&str>;
}

// TODO: Is this actually useful?
impl SliceAsStrs for [String] {
    fn as_strs(&self) -> Vec<&str> {
        self.iter().map(|s| &s[..]).collect()
    }
}

pub fn do_file(tmpl_name: &str, environ: &HashMap<&str, &str>)
    -> Result<Vec<String>, String>
{
    let tf = File::open(tmpl_name).map_err(|e| e.to_string())?;
    let tb = io::BufReader::new(tf);
    let lines: Vec<String> = tb.lines()
        .collect::<Result<_, _>>() // TODO: understand this magic
        .map_err(|e| e.to_string())?; // TODO: make this friendlier
    do_lines(&lines, environ)
}

pub fn do_lines(lines: &Vec<String>, environ: &HashMap<&str, &str>)
    -> Result<Vec<String>, String>
{
    static OPEN_PAT: &str = "{{";
    static OPEN_ESC: &str = "{{!";
    static CLOSE_PAT: &str = "}}";

    let mut lines2 = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        // Some((to, from, val)) or None (end)
        let mut replace = vec![Some((0, 0, ""))];
        for open_pos in str_find_all(line, OPEN_PAT) {
            let name_start = line[open_pos + OPEN_PAT.len() ..].chars().next();
            if name_start == Some('!') {
                replace.push(
                    Some((open_pos, open_pos + OPEN_ESC.len(), "{{"))
                );
            } else {
                let close_pos = str_find_at(line, open_pos, CLOSE_PAT)
                    .ok_or(
                        format!("{}: Missing \"{}\"", row, CLOSE_PAT)
                    )?;
                let name_from = open_pos + OPEN_PAT.len();
                let name_to = close_pos;
                let name = &line[name_from..name_to];
                let val = environ.get(name)
                    .ok_or(
                        format!(
                            "{},{}: {} is not defined", row, open_pos, name
                        )
                    )?;
                // TODO: use ok_or_else instead
                replace.push(
                    Some((open_pos, close_pos + CLOSE_PAT.len(), val))
                );
            }
        }
        replace.push(None);

        let mut line2 = "".to_string();
        for window in replace.windows(2) {
            let (prev, this) = (window[0], window[1]);
            let (_, prev_to, _) = prev.unwrap();
            if let Some((from, _, val)) = this {
                line2.push_str(&line[prev_to..from]);
                line2.push_str(val);
            } else {
                line2.push_str(&line[prev_to..]);
            }
        }
        lines2.push(line2);
    }
    Ok(lines2)
}

trait InvertOption<T: Default> {
    fn invert(self) -> Option<T>;
}

// TODO: Is this useful?
impl<T: Default> InvertOption<T> for Option<T> {
    fn invert(self) -> Option<T> {
        match self {
            Some(_) => None,
            None => Some(T::default()),
        }
    }
}

fn args_to_environ<'a>(args: &[&'a str]) -> Option<HashMap<&'a str, &'a str>> {
    let mut environ = HashMap::<&str, &str>::new();
    for pair in args {
        let (name, val) = take2(&mut pair.splitn(2, '='))?;
        environ.insert(name, val).invert()?; // TODO: need an error message
    }
    Some(environ)
}

#[derive(Debug, PartialEq)]
pub struct Mint<'a> {
    pub tmpl_name: &'a str,
    pub environ: HashMap<&'a str, &'a str>,
}

impl<'a> Mint<'a> {
    pub fn with_args(args: &[&'a str]) -> Option<Mint<'a>> {
        let tmpl_name = args.get(0)?;
        let environ_args: Vec<&str> = args[1..].to_vec();
        let environ = args_to_environ(&environ_args)?;
        Some(Mint { tmpl_name, environ })
    }
}
