extern mod css_lex;
extern mod extra;

use extra::json;
use extra::json::ToJson;
use css_lex::*;

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
                if !json_almost_equals(&result, &expected) {
                    fail!(format!("got: {}\nexpected: {}",
                                  result.to_str(),
                                  expected.to_str()));
                }
            },
            _ => fail!("Unexpected JSON")
        };
    }
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
