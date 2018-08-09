use std::i64;

use combine::char::{char as c, digit, hex_digit, letter, spaces, string};
use combine::{between, choice, eof, many, many1, one_of, try, ParseError, Parser, Stream, optional};

use expr::Expr::{self, *};

fn int<I>() -> impl Parser<Input = I, Output = Expr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let binary = (string("0b"), many1(one_of("01".chars())))
        .map(|(_, s): (_, String)| i64::from_str_radix(&s, 2).unwrap());

    let hex = (string("0x"), many1(hex_digit()))
        .map(|(_, s): (_, String)| i64::from_str_radix(&s, 16).unwrap());

    let decimal = many1(digit()).map(|s: String| s.parse::<i64>().unwrap());

    choice((try(binary), try(hex), try(decimal))).skip(spaces()).map(Int)
}

fn float<I>() -> impl Parser<Input = I, Output = Expr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (many1(digit()), string("."), many1(digit())).map(|(a, b, c): (String, &str, String)| {
        Float([a, b.to_string(), c].join("").parse::<f64>().unwrap())
    })
}

fn boolean<I>() -> impl Parser<Input = I, Output = Expr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let t = || string("true").map(|_| Bool(true));
    let f = || string("false").map(|_| Bool(false));
    (try(choice((t(), f()))), spaces()).map(|(b, _)| b)
}

fn symbol<I>() -> impl Parser<Input = I, Output = Expr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many1(letter().or(one_of("+-*/^&|%!=".chars())))
        .skip(spaces())
        .map(Symbol)
}

fn nil<I>() -> impl Parser<Input = I, Output = Expr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    string("nil").skip(spaces()).map(|_| Nil)
}

parser!{
    #[inline(always)]
    fn expr[I]()(I) -> Expr
    where [I: Stream<Item = char>]
    {
        let empty_list = try((c('('), spaces(), c(')'), spaces())).map(|_| Nil);
        let list = between(c('(').skip(spaces()), c(')'), many(expr()))
            .skip(spaces())
            .map(Expr::List);

        let quote = (c('\''), expr()).map(|(_, e)| Quote(Box::new(e)));

        choice((
            boolean(),
            int(),
            float(),
            nil(),
            symbol(),
            empty_list,
            list,
            quote
        ))
    }
}

pub fn parse(input: &str) -> Result<Expr, String> {
    match (spaces(), optional(expr()).skip(eof())).easy_parse(input) {
        Ok(((_, Some(e)), _)) => Ok(e),
        Ok(((_, None), _)) => Ok(Nil),
        Err(err) => Err(format!("{}", err)),
    }
}
