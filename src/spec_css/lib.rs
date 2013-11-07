#[link(name = "spec_css", vers = "0.0")];

extern mod extra;
extern mod css_lex;
extern mod speculate;

static LOOKBACK: uint = 10;

/**
 * Find the start of the next token at or after `start`.
 *
 * Backs up `LOOKBACK` characters and begins lexing until reaching or passing
 * `start`.
 *
 * This is somewhat inefficient, since it makes a copy of `input`.
 */
pub fn next_token_start(input: &str, start: uint) -> uint {
    let mut tokenizer = css_lex::tokenize(input);
    tokenizer.position = if start < LOOKBACK {0} else {start - LOOKBACK};
    while tokenizer.position < start && tokenizer.next().is_some() {}
    tokenizer.position
}
