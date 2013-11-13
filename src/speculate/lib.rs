#[link(name = "speculate", vers = "0.0")];

extern mod extra;

use extra::future::Future;

/**
 * Speculatively execute consumer using the guessed value.
 */
pub fn spec<A: Eq + Send + Clone, B>(producer: ~fn() -> A,
                                     predictor: ~fn() -> A,
                                     consumer:  ~fn(A) -> B) -> B {

    let producer_result = Future::spawn(producer);
    let prediction = predictor();
    let speculative_result = consumer(prediction.clone());
    let real_value = producer_result.unwrap();

    if real_value == prediction {
        speculative_result
    } else {
        consumer(real_value)
    }
}

/**
 * Iteratively execute `loop_body` by guessing a value.
 *
 * the &fn() would close over the Arc, and then it would .clone it for each new
 * ~fn
 */
pub fn specfold<A: Eq + Clone + Send>(iters: uint,
                                      loop_body: &fn() -> ~fn(uint, A) -> A,
                                      predictor: &fn() -> ~fn(uint) -> A) {

    // The future is (prediction, result)
    let mut results: ~[Future<(A, A)>] = std::vec::with_capacity(iters);
    for i in range(0, iters) {
        let fut = do Future::spawn_with((predictor(), loop_body())) |(p,l)| {
            let prediction = p(i);
            let res = l(i, prediction.clone());
            (prediction, res)
        };
        results.push(fut);
    }

    // Wait for the first result. This is necessary in the case that `iters` is
    // 1, since then the validation loop will not run.
    if iters == 1 { results[0].get_ref(); }

    // Validate. Sequentially, for now
    for i in range(1, iters) {
        let (_, previous) = results[i - 1].get();
        let (prediction, _) = results[i].get();
        if previous != prediction {
            let res = loop_body()(i, previous.clone());
            results[i] = Future::from_value((previous, res));
        }
    }
}
