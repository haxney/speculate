extern mod speculate;

use std::task;
use std::vec;
use std::comm::{SharedPort, SharedChan, stream};
use speculate::*;

#[test]
fn test_spec() {
    assert!(spec(|| 2 + 2, || 4, |x| x + 2) == 6);
    assert!(spec(|| 2 + 2, || 1, |x| x + 2) == 6);
}

fn spawn_result_collector<T: Send + Default + Clone>(port: SharedPort<Option<(int, T)>>, chan: Chan<~[T]>, size: uint) {
    do task::spawn {
        let mut results = vec::from_elem::<T>(size, Default::default());
        loop {
            match port.recv() {
                Some((idx, val)) => results[idx] = val,
                None => break
            }
        }
        chan.send(results.clone());
    }
}


#[test]
fn test_specfold_correct_prediction() {
    let (port, chan): (Port<Option<(int, int)>>, Chan<Option<(int, int)>>) = stream();
    let (res_port, res_chan) = stream();
    let shared_chan = SharedChan::new(chan);
    let shared_port = SharedPort::new(port);

    let loop_body: &fn() -> ~fn(int, int) -> int = || {
        let clone_chan = shared_chan.clone();
        |idx:int, val:int| {
            let res = idx + val;
            clone_chan.send(Some((idx, res)));
            res
        }
    };

    let loop_results = [0, 0, 1, 3, 6];
    let predictor: &fn() -> ~fn(int) -> int = || { |idx| loop_results[idx] };
    spawn_result_collector(shared_port.clone(), res_chan, 5);
    specfold(0, 5, loop_body, predictor);
    shared_chan.send(None);
    assert!(res_port.recv() == ~[0, 1, 3, 6, 10]);
}

#[test]
fn test_specfold_incorrect_prediction() {
    let (port, chan): (Port<Option<(int, int)>>, Chan<Option<(int, int)>>) = stream();
    let (res_port, res_chan) = stream();
    let shared_chan = SharedChan::new(chan);
    let shared_port = SharedPort::new(port);

    let loop_body: &fn() -> ~fn(int, int) -> int = || {
        let clone_chan = shared_chan.clone();
        |idx:int, val:int| {
            let res = idx + val;
            clone_chan.send(Some((idx, res)));
            res
        }
    };

    let predictor: &fn() -> ~fn(int) -> int = || { |_| 0 };
    spawn_result_collector(shared_port.clone(), res_chan, 5);
    specfold(0, 5, loop_body, predictor);
    shared_chan.send(None);
    assert!(res_port.recv() == ~[0, 1, 3, 6, 10]);

}
