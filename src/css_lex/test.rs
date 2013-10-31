extern mod css_lex;
extern mod extra;

use extra::json;
use extra::json::ToJson;
use css_lex::*;

fn almost_equals(a: &json::Json, b: &json::Json) -> bool {
    match (a, b) {
        (&json::Number(a), &json::Number(b)) => (a - b).abs() < 1e-6,
        (&json::String(ref a), &json::String(ref b)) => a == b,
        (&json::Boolean(a), &json::Boolean(b)) => a == b,
        (&json::List(ref a), &json::List(ref b))
            => a.iter().zip(b.iter()).all(|(ref a, ref b)| almost_equals(*a, *b)),
        (&json::Object(_), &json::Object(_)) => fail!(~"Not implemented"),
        (&json::Null, &json::Null) => true,
        _ => false,
    }
}

fn run_json_tests<T: ToJson>(json_data: &str, parse: &fn (input: ~str) -> T) {
    let items = match json::from_str(json_data) {
        Ok(json::List(items)) => items,
        _ => fail!("Invalid JSON")
    };
    assert!(items.len() % 2 == 0);
    let mut input: Option<~str> = None;
    for item in items.move_iter() {
        match (&input, item) {
            (&None, json::String(string)) => input = Some(string),
            (&Some(_), expected) => {
                let css = input.take_unwrap();
                let result = parse(css.to_owned()).to_json();
                if !almost_equals(&result, &expected) {
                    fail!(css);
                }
            },
            _ => fail!("Unexpected JSON")
        };
    }
}

fn list_to_json(list: &~[(Token, SourceLocation)]) -> ~[json::Json] {
    list.map(|tuple| {
        match *tuple {
            (ref c, _) => c.to_json()
        }
    })
}

#[test]
fn tokenize_simple() {
    let mut t = tokenize("a");
    assert!(t.next() == Some((Ident(~"a"), SourceLocation{ line:1, column: 1})));
}

#[test]
fn test_tokenize_json() {
    do run_json_tests(include_str!("css-lexing-tests/tokens.json")) |input| {
        list_to_json(&tokenize(input).to_owned_vec())
    }
}
