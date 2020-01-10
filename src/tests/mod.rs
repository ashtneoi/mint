use std::collections::HashMap;

use crate::lib::{do_file, do_lines, Mint};

#[test]
fn everything() {
    let args = &["src/tests/tmpl", "biz==_=", "foo=9*9/|"];
    let m = Mint::with_args(args).unwrap();
    let out = do_file(m.tmpl_name, &m.environ);
    assert_eq!(
        out,
        Ok(vec![
            "9*9/|bar=_=".to_string(),
            "x9*9/|=_=x".to_string(),
        ])
    );
}

#[test]
fn lines() {
    let lines = vec![
        "no {{bbb}} like {{aaa}}".to_string(),
        "{{bbb}} pasta +{{aaa}}+".to_string(),
        "{{ccc}}{{ccc}}{{ccc}}".to_string(),
    ];
    let mut environ: HashMap<&str, &str> = HashMap::new();
    environ.insert("aaa", "spinach");
    environ.insert("ccc", "1234");
    environ.insert("bbb", "()()");
    let out = do_lines(&lines, &environ);

    assert_eq!(
        out,
        Ok(vec![
            "no ()() like spinach".to_string(),
            "()() pasta +spinach+".to_string(),
            "123412341234".to_string(),
        ]),
    );
}

#[test]
fn cli_no_dup() {
    assert_eq!(
        Mint::with_args(&["nothing", "xyz=99", "xyz=pancakes"]),
        None
    );
    assert_eq!(
        Mint::with_args(&["nothing", "m=m", "oooo=!!", "u=u", "oooo=juice"]),
        None
    );
    assert_eq!(
        Mint::with_args(&["nothing", "wxyz=abcd", "m=m", "wxyz=abcd"]),
        None
    );
}

#[test]
fn no_replacements() {
    let lines = vec![
        "onward toward the rising sun".to_string(),
        "leave the cruel night behind".to_string(),
    ];
    let mut environ: HashMap<&str, &str> = HashMap::new();
    environ.insert("aaa", "spinach");
    environ.insert("ccc", "1234");
    environ.insert("bbb", "()()");

    let out = do_lines(&lines, &environ);
    assert_eq!(
        out,
        Ok(lines),
    );
}

#[test]
fn test_brace_escape() {
    let lines = vec![
        "{{!!foo}} {{!foo}} {{foo}} {{!".to_string(),
    ];
    let mut environ: HashMap<&str, &str> = HashMap::new();
    environ.insert("foo", "FOO");

    let out = do_lines(&lines, &environ);
    assert_eq!(
        out,
        Ok(vec![
            "{{!foo}} {{foo}} FOO {{".to_string(),
        ])
    );
}

#[test]
fn cli_disallowed_names() {
    assert_ne!(
        Mint::with_args(&["nothing"]),
        None
    );

    assert_eq!(
        Mint::with_args(&["nothing", "}}=foo"]),
        None
    );
    assert_eq!(
        Mint::with_args(&["nothing", "foo}}bar=biz"]),
        None
    );
    assert_eq!(
        Mint::with_args(&["nothing", "!=foo"]),
        None
    );
    assert_eq!(
        Mint::with_args(&["nothing", "!foo=bar"]),
        None
    );
}
