mod parse;
mod term;
mod token;
mod typecheck;

pub use self::parse::parse;
pub use self::term::Term;
pub use self::typecheck::typecheck;

#[cfg(test)]
mod tests {
    use super::*;

    // number 型同士以外の足し算をしないこと

    #[test]
    fn test_1_plus_2_is_ok() {
        typecheck(parse("1 + 2"));
    }

    #[should_panic]
    #[test]
    fn test_1_plus_true_is_ng() {
        typecheck(parse("1 + true"));
    }

    #[should_panic]
    #[test]
    fn test_false_plus_true_is_ng() {
        typecheck(parse("1 + true"));
    }

    // 条件演算子の条件式が boolean 型であること

    #[test]
    fn test_true_quest_is_ok() {
        typecheck(parse("true ? 0 : 0"));
    }

    #[should_panic]
    #[test]
    fn test_1_quest_is_ng() {
        typecheck(parse("1 ? 0 : 0"));
    }

    // 条件演算子の返す型が一致すること

    #[test]
    fn test_true_quest_true_colon_false_is_ok() {
        typecheck(parse("true ? true : false"));
    }

    #[test]
    fn test_true_quest_1_colon_2_is_ok() {
        typecheck(parse("true ? 1 : 2"));
    }

    #[should_panic]
    #[test]
    fn test_true_quest_true_colon_1_is_ok() {
        typecheck(parse("true ? true : 1"));
    }

    #[should_panic]
    #[test]
    fn test_true_quest_2_colon_false_is_ok() {
        typecheck(parse("true ? 2 : false"));
    }
}
