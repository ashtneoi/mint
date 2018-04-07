use lib::{do_file, Mint};

#[test]
fn everything() {
    let args = &["src/tests/tmpl", "biz==_=", "foo=9*9/|"];
    let m = Mint::with_args(args);
    let maybe_output = do_file(m.tmpl_name, &m.environ);
    assert_eq!(
        maybe_output,
        Ok(vec![
            "9*9/|bar=_=".to_string(),
            "x9*9/|=_=x".to_string(),
        ])
    );
}
