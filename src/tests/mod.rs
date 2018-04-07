use std::collections::HashMap;

use lib::{do_file, do_lines, Mint};

#[test]
fn everything() {
    let args = &["src/tests/tmpl", "biz==_=", "foo=9*9/|"];
    let m = Mint::with_args(args);
    let out = do_file(m.tmpl_name, &m.environ);
    assert_eq!(
        out,
        Ok(vec![
            // TODO: can these be &str?
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
