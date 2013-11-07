extern mod spec_css;

use spec_css::*;

#[test]
fn test_next_token_start() {
    let css = "cls1 : cls2 {prop: val;}";

    assert!(next_token_start(css, 8) == 11);
    assert!(next_token_start(css, 4) == 4);
    assert!(next_token_start(css, 13) == 13);
    assert!(next_token_start(css, 14) == 17);
    assert!(next_token_start(css, 0) == 0);
}
