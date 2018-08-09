extern crate libchao;

use libchao::Expr::*;

macro_rules! assert_parse {
    ($input:expr, $expected:expr) => (
        assert_eq!(libchao::parse($input), Ok($expected))
    )
}

macro_rules! assert_parse_err {
    ($input:expr) => (
        assert!(libchao::parse($input).is_err())
    )
}


#[test]
fn parses_whitespace() {
    assert_parse!("", Nil);
    assert_parse!(" \n\r\t   ", Nil);
}

#[test]
fn parses_nil() {
    assert_parse!("nil", Nil);
}

#[test]
fn parses_booleans() {
    assert_parse!("true", Bool(true));
    assert_parse!("false", Bool(false));
}

#[test]
fn parse_symbols() {
    assert_parse!("symbol", Symbol("symbol".to_string()));
    assert_parse_err!("sym bol");
    assert_parse!("+-/*%|&", Symbol("+-/*%|&".to_string()));
}

#[test]
fn parses_integers() {
    assert_parse!("42", Int(42));
    assert_parse!("0x2a", Int(42));
    assert_parse_err!("0xz2a");
    assert_parse!("0b101010", Int(42));
    assert_parse_err!("0b2101010");
}

#[test]
fn parses_lists() {
    assert_parse!("(+ 1 nil true)", List(vec![Symbol("+".to_string()), Int(1), Nil, Bool(true)]));
    assert_parse!("()", Nil);
    assert_parse!("(())", List(vec![Nil]));
    assert_parse!("(1 () 2)", List(vec![Int(1), Nil, Int(2)]));
    assert_parse!("((()))", List(vec![List(vec![Nil])]));
    assert_parse!("((() ()))", List(vec![List(vec![Nil, Nil])]));
    assert_parse!("((42))", List(vec![List(vec![Int(42)])]));
    assert_parse!("(+ 1 2 3)", List(vec![Symbol("+".to_string()), Int(1), Int(2), Int(3)]));
}

#[test]
fn parses_quotes() {
    assert_parse!("'1", Quote(Box::new(Int(1))));
    assert_parse!("''1", Quote(Box::new(Quote(Box::new(Int(1))))));
    assert_parse!("'()", Quote(Box::new(Nil)));
    assert_parse!("'(a b c)", Quote(Box::new(List(vec![
        Symbol("a".to_string()),
        Symbol("b".to_string()),
        Symbol("c".to_string())
    ]))));
    assert_parse!(
        "'('(1))",
                  Quote(
                      Box::new(
                          List(vec![
                              Quote(
                                  Box::new(
                                      List(vec![Int(1)])
                                  )
                              )
                          ])
                      )
                  )
    );
}

#[test]
fn parses_strings() {
    assert_parse!(r#""""#, Str("".to_string()));
    assert_parse!(r#""\"""#, Str("\"".to_string()));
    assert_parse!(r#""\n""#, Str("\n".to_string()));
    assert_parse!(r#""\r""#, Str("\r".to_string()));
    assert_parse!(r#""\t""#, Str("\t".to_string()));
    assert_parse!(r#""\\""#, Str("\\".to_string()));
    assert_parse!(r#"" a b c ""#, Str(" a b c ".to_string()));
    assert_parse!(r#""' \"abc\" '""#, Str("' \"abc\" '".to_string()));
    assert_parse_err!(r#"""#);
    assert_parse_err!(r#"\"#);
    assert_parse_err!(r#"\\"#);
    assert_parse_err!(r#"\"""#);
    assert_parse_err!(r#""\foo""#);
}
