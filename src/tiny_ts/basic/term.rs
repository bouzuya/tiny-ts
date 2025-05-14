use super::Type;

#[derive(Debug, PartialEq)]
pub enum Term {
    /// false リテラル (例: `false`)
    False,
    /// true リテラル (例: `true`)
    True,
    /// 数値リテラル (例: `1`, `2`, `100`)
    /// 実装の簡素化のため 0..=255 の範囲に制限
    Integer(u8),
    /// 足し算 (例: `1 + 2`)
    Add { left: Box<Term>, right: Box<Term> },
    /// 条件演算子 (例: `false ? 1 : 2`, `true ? false : true`)
    If {
        cond: Box<Term>,
        thn: Box<Term>,
        els: Box<Term>,
    },
    /// 変数参照 (例: `x`, `f`)
    Var { name: String },
    /// 無名関数 (例: `(x: number) => x`)
    Func { params: Vec<Param>, body: Box<Term> },
    /// 関数呼び出し (例: `f(1)`, `f(true)`)
    Call { func: Box<Term>, args: Vec<Term> },
    /// 逐次実行 (例: `f(1); f(2);`)
    Seq { body: Box<Term>, rest: Box<Term> },
    /// 変数定義 (例: `const x = 1; ...`)
    Const {
        name: String,
        init: Box<Term>,
        rest: Box<Term>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub typ: Type,
}
