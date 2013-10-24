#[link(name = "speculate", vers = "0.0")];

extern mod extra;

use extra::future::Future;
use std::num;

/**
 * Speculatively execute consumer using the guessed value.
 */
pub fn spec<A: Eq + Send, B>(producer: ~fn() -> A,
                             predictor: ~fn() -> A,
                             consumer:  ~fn(x: &A) -> B) -> B {

    let producer_result = Future::spawn(producer);
    let prediction = predictor();
    let speculative_result = consumer(&prediction);
    let real_value = producer_result.unwrap();

    if real_value == prediction {
        speculative_result
    } else {
        consumer(&real_value)
    }
}

/**
 * Iteratively execute `loop_body` by guessing a value.
 *
 * the &fn() would close over the Arc, and then it would .clone it for each new
 * ~fn
 */
pub fn specfold<A: Eq + Clone + Send>(low: int, high: int,
                                      loop_body: &fn() -> ~fn(int, A) -> A,
                                      predictor: &fn() -> ~fn(int) -> A) {

    let len = num::abs(high - low) as uint;
    // The future is (prediction, result)
    let mut results: ~[Future<(A, A)>] = std::vec::with_capacity(len);
    for i in range(low, high) {
        let fut = do Future::spawn_with((predictor(), loop_body())) |(p,l)| {
            let prediction = p(i);
            let res = l(i, prediction.clone());
            (prediction, res)
        };
        results.push(fut);
    }

    // Validate. Sequentially, for now
    for i in range(low + 1, high) {
        let (_, previous) = results[(i - low) - 1].get();
        let (prediction, _) = results[i - low].get();
        if previous != prediction {
            let res = loop_body()(i, previous.clone());
            results[i - low] = Future::from_value((previous, res));
        }
    }
}
