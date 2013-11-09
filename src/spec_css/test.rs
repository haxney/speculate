extern mod spec_css;
extern mod css_lex;
extern mod speculate;
extern mod extra;

use spec_css::*;
use css_lex::*;
use extra::arc::Arc;
use extra::json;
use extra::json::ToJson;

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

// This could be replaced with JSON tests.
#[test]
fn test_next_token_start() {
    let css = Arc::new(~"cls1 : cls2 {prop: val;}");

    assert!(next_token_start(css.clone(), 8) == 11);
    assert!(next_token_start(css.clone(), 4) == 4);
    assert!(next_token_start(css.clone(), 13) == 13);
    assert!(next_token_start(css.clone(), 14) == 17);
    assert!(next_token_start(css.clone(), 0) == 0);
}

#[test]
fn test_spec_token_json() {
    // Test different number of parallel tasks
    do run_json_tests(include_str!("../css_lex/css-lexing-tests/tokens.json")) |input| {
        list_to_json(&spec_tokenize(input, 1))
    }
    do run_json_tests(include_str!("../css_lex/css-lexing-tests/tokens.json")) |input| {
        list_to_json(&spec_tokenize(input, 2))
    }
    do run_json_tests(include_str!("../css_lex/css-lexing-tests/tokens.json")) |input| {
        list_to_json(&spec_tokenize(input, 3))
    }
}
