extern mod speculate;

use speculate::*;

#[test]
fn simple() {
    assert!(spec(|| 2 + 2, || 4, |x:&int| x + 2) == 6);
    assert!(spec(|| 2 + 2, || 1, |x:&int| x + 2) == 6);
}
