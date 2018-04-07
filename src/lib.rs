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
    static CLOSE_PAT: &str = "}}";

    let mut lines2 = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let mut replace = vec![Some((0, 0))]; // Some(to, from) or None (end)
        for open_pos in str_find_all(line, OPEN_PAT) {
            let close_pos = str_find_at(line, open_pos, CLOSE_PAT)
                .ok_or(
                    format!("{}: Missing \"{}\"", row, CLOSE_PAT)
                )?;
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
                let name_from = from + OPEN_PAT.len();
                let name_to = to - CLOSE_PAT.len();
                let name = &line[name_from..name_to];
                let val = environ.get(name)
                    .ok_or(
                        format!("{},{}: {} is not defined", row, from, name)
                    )?;
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
