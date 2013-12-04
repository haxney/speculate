#[link(name = "spec_css", vers = "0.0")];

extern mod extra;
extern mod css_lex;
extern mod speculate;

use css_lex::*;
use speculate::*;
use std::{num, task, vec};
use std::comm::{SharedPort, SharedChan, stream};
use extra::arc::Arc;

static LOOKBACK: uint = 10;

/**
 * Collects the loop body results into a 2d vector.
 *
 * Each loop body sends its index and an optional result type. If the message
 * received over the `port` is `None`, then stop listening and push the result
 * over `chan`. If the message received is `Some(i, None)`, then clear the
 * `i`-th result vector. If the message is `Some(i, Some(t))`, then add `t` to
 * the `i`-th result vector.
 */
fn spawn_result_collector<T: Send + Clone>(port: SharedPort<Option<(uint, Option<~[T]>)>>,
                                           chan: Chan<~[T]>,
                                           size: uint) {
    do task::spawn {
        let mut results = vec::from_elem::<~[T]>(size, Default::default());
        loop {
            match port.recv() {
                Some((idx, Some(val))) => results[idx] = val,
                Some((idx, None)) => results[idx].clear(),
                None => break
            }
        }
        chan.send(results.move_iter().flat_map(|v| v.move_iter()).to_owned_vec());
    }
}

/**
 * Find the start of the next token at or after `start`.
 *
 * Backs up `LOOKBACK` characters and begins lexing until reaching or passing
 * `start`.
 *
 * Assumes `input` has already been preprocessed.
 */
pub fn next_token_start(input: Arc<~str>, start: uint) -> uint {
    let mut tokenizer = Tokenizer::new(input);
    tokenizer.position = if start < LOOKBACK {
        0
    } else {
        num::min(start - LOOKBACK, tokenizer.length)
    };
    while tokenizer.position < start && tokenizer.next().is_some() {}
    tokenizer.position
}

pub fn spec_tokenize(input: ~str, num_iters: uint) -> (SpecStats, ~[Node]) {
    let input = preprocess(input);
    let css_len = input.len();
    let str_arc = Arc::new(input);
    let iter_size: uint = (css_len + num_iters - 1) / num_iters; // round up
    let (port, chan): (Port<Option<(uint, Option<~[Node]>)>>,
                       Chan<Option<(uint, Option<~[Node]>)>>) = stream();
    let (res_port, res_chan) = stream();
    let body_chan = SharedChan::new(chan);
    let body_port = SharedPort::new(port);

    let loop_body: &fn() -> ~fn(uint, uint) -> uint = || {
        let (arc_port, arc_chan) = stream();
        arc_chan.send(str_arc.clone());
        let local_body_chan = body_chan.clone();

        |idx:uint, token_start:uint| {
            // exclusive bound
            let upper = num::min((idx + 1) * iter_size, css_len);
            let string = arc_port.recv();
            let mut tokenizer = Tokenizer::new(string);
            tokenizer.position = token_start;
            let mut results: ~[Node] = vec::with_capacity(10); // arbitrary starting number

            // Reset the vector for this loop iteration
            local_body_chan.send(Some((idx, None)));
            while tokenizer.position < upper {
                match tokenizer.next() {
                    Some(node) => results.push(node),
                    None => break
                }
            }
            local_body_chan.send(Some((idx, Some(results))));
            tokenizer.position
        }
    };

    let predictor: &fn() -> ~fn(uint) -> uint = || {
        let (arc_port, arc_chan) = stream();
        arc_chan.send(str_arc.clone());
        |idx| {
            next_token_start(arc_port.recv(), idx * iter_size)
        }
    };

    spawn_result_collector(body_port.clone(), res_chan, num_iters);
    let res = specfold(num_iters, loop_body, predictor);
    body_chan.send(None);
    (res, res_port.recv())
}
