extern mod spec_css;
extern mod css_lex;
extern mod speculate;
extern mod extra;

use spec_css::*;
use css_lex::*;
use speculate::*;
use std::{num, task, vec};
use std::comm::{SharedPort, SharedChan, stream};
use extra::json::ToJson;

fn spawn_result_collector<T: Send + Clone + ToJson>(port: SharedPort<Option<(int, T)>>, chan: Chan<~[T]>, size: uint) {
    do task::spawn {
        let mut results = vec::from_elem::<~[T]>(size, Default::default());
        loop {
            match port.recv() {
                Some((idx, val)) => {
                    println!("worker {}: {}", idx, val.to_json().to_str());
                    results[idx].push(val)
                },
                None => break
            }
        }
        chan.send(results.flat_map(|v| v.clone()));
    }
}

// This could be replaced with JSON tests.
#[test]
fn test_next_token_start() {
    let css = "cls1 : cls2 {prop: val;}";

    assert!(next_token_start(css, 8) == 11);
    assert!(next_token_start(css, 4) == 4);
    assert!(next_token_start(css, 13) == 13);
    assert!(next_token_start(css, 14) == 17);
    assert!(next_token_start(css, 0) == 0);
}

#[test]
fn test_specfold_correct_prediction() {
    let css = "cls1 : cls2 {prop: val;}";
    let css_len = css.len();
    let iter_size: uint = 8;
    let num_iters: uint = 3;
    let (port, chan): (Port<Option<(int, Token)>>, Chan<Option<(int, Token)>>) = stream();
    let (res_port, res_chan) = stream();
    let shared_chan = SharedChan::new(chan);
    let shared_port = SharedPort::new(port);

    let loop_body: &fn() -> ~fn(int, uint) -> uint = || {
        let clone_chan = shared_chan.clone();
        |idx:int, token_start:uint| {
            // exclusive bound
            let upper = num::min((idx as uint + 1) * iter_size, css_len);
            let mut tokenizer = tokenize(css);
            tokenizer.position = token_start;
            while tokenizer.position < upper {
                match tokenizer.next() {
                    Some((t, _)) => clone_chan.send(Some((idx, t))),
                    None => break
                }
            }
            tokenizer.position
        }
    };

    let predictor: &fn() -> ~fn(int) -> uint = || {
        |idx| {
            let nts = next_token_start(css, idx as uint * iter_size);
            println!("prediction {}: {}", idx, nts);
            nts
        }
    };
    spawn_result_collector(shared_port.clone(), res_chan, num_iters);
    specfold(0, num_iters as int, loop_body, predictor);
    shared_chan.send(None);
    //println(list_to_json(&res_port.recv()).to_str())
}
