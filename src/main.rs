mod tiny_ts;

use tiny_ts::{Term, parse};

fn main() {
    assert_eq!(parse("true"), Term::True);
}
