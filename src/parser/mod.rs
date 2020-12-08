pub mod ast;

use crate::runtime::Value;

peg::parser! {
    pub grammar context() for str {
        pub rule parse() -> Value
            = precedence! {
                dlm() expr: expr0() {Value::Unevaluted(expr)}
            }

        rule expr0() -> ast::Expr0
            = precedence! {
                left:(@) ("d"/"D") right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("d")}}
                --
                term:term() {ast::Expr0::Term(term)}
            }

        rule term() -> ast::Term
            = precedence! {
                "(" expr:expr0() ")" {ast::Term::Expr0(Box::new(expr))}
                --
                literal:literal() {ast::Term::Literal(literal)}
            }

        rule literal() -> ast::Literal
            = precedence! {
                n:$(['0'..='9']+) { ast::Literal::Integer(n.parse().unwrap()) }
            }

        rule ident() -> String
            = x:$(['A'..='Z' | 'a'..='z']) xs:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']) { String::from(x) + xs }

        rule dlm() = quiet!{[' ' | '\n' | '\t']+}
    }
}
