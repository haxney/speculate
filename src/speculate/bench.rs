extern mod speculate;
extern mod extra;

use std::vec;
use std::comm::stream;
use speculate::*;
use extra::arc::Arc;
use extra::test;

#[bench]
fn bench_direct(b: &mut test::BenchHarness) {
    let v = vec::from_fn(4096, |n| n);
    do b.iter {
        let val = v.iter().fold(0, |old, new| old + *new);
        v.iter().fold(val, |old, new| old + *new);
    }
}

#[bench]
fn bench_spec_correct(b: &mut test::BenchHarness) {
    let v = vec::from_fn(4096, |n| n);
    let v_arc = Arc::new(v);

    do b.iter {
        let (port, chan)  = stream();
        let (port2, chan2)  = stream();
        chan.send(v_arc.clone());
        chan2.send(v_arc.clone());
        chan2.send(v_arc.clone());
        spec(|| {
                let local_arc = port.recv();
                local_arc.get().iter().fold(0, |old, new| old + *new)
            },
             || 8386560, // Result of `fold`
             |x| {
                let local_arc = port2.recv();
                local_arc.get().iter().fold(x, |old, new| old + *new)
            });
    }


}

#[bench]
fn bench_spec_wrong(b: &mut test::BenchHarness) {
    let v = vec::from_fn(4096, |n| n);
    let v_arc = Arc::new(v);

    do b.iter {
        let (port, chan)  = stream();
        let (port2, chan2)  = stream();
        chan.send(v_arc.clone());
        chan2.send(v_arc.clone());
        chan2.send(v_arc.clone());
        spec(|| {
                let local_arc = port.recv();
                local_arc.get().iter().fold(0, |old, new| old + *new)
            },
             || 0, // Incorrect result of `fold`
             |x| {
                let local_arc = port2.recv();
                local_arc.get().iter().fold(x, |old, new| old + *new)
            });
    }


}
