mod tiny_ts;

use tiny_ts::arith::{Term, parse};

fn main() {
    assert_eq!(parse("true"), Term::True);
}
