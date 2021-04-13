pub mod ast;

use crate::runtime::Value;

peg::parser! {
    pub grammar context() for str {
        pub rule parse() -> Value
            = precedence! {
                dlm()? expr: expr0() dlm()? {Value::Unevaluted(expr)}
                dlm()? "`" dlm()? expr: expr0() dlm()? "`" dlm()? {Value::Unevaluted(expr)}
                dlm()? "```" dlm()? expr: expr0() dlm()? "```" dlm()? {Value::Unevaluted(expr)}
            }

        rule expr0() -> ast::Expr0
            = precedence! {
                left:(@) dlm()? "#" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("#")}}
                --
                left:(@) dlm()? "==" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("==")}}
                left:(@) dlm()? "!=" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("!=")}}
                left:(@) dlm()? "<=" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("<=")}}
                left:(@) dlm()? ">=" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from(">=")}}
                --
                left:(@) dlm()? "<" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("<")}}
                left:(@) dlm()? ">" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from(">")}}
                --
                left:(@) dlm()? "+" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("+")}}
                left:(@) dlm()? "-" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("-")}}
                --
                left:(@) dlm()? "*" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("*")}}
                left:(@) dlm()? "/" dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("/")}}
                --
                left:(@) dlm()? ("d"/"D") dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("d")}}
                left:(@) dlm()? ("b"/"B") dlm()? right:@ {ast::Expr0::Expr0{left: Box::new(left), right: Box::new(right), operator: String::from("b")}}
                --
                term:term() {ast::Expr0::Term(term)}
            }

        rule term() -> ast::Term
            = precedence! {
                "(" dlm()? expr:expr0() dlm()? ")" {ast::Term::Expr0(Box::new(expr))}
                "[" dlm()? exprs:expr0() ** ("," dlm()?) dlm()? "]" {ast::Term::Array(exprs)}
                "{" dlm()? pairs:key_value() ** ("," dlm()?) dlm()? "}" {ast::Term::Record(pairs.into_iter().collect())}
                --
                literal:literal() {ast::Term::Literal(literal)}
            }

        rule key_value() -> (String, ast::Expr0)
            = ident:ident() dlm()? ":" dlm()? expr:expr0() { (ident, expr) }

        rule literal() -> ast::Literal
            = precedence! {
                n:$(['0'..='9']+) { ast::Literal::Integer(n.parse().unwrap()) }
            }

        rule ident() -> String
            = x:$(['A'..='Z' | 'a'..='z']) xs:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']*) { String::from(x) + xs }

        rule dlm() = quiet!{[' ' | '\n' | '\t']+}
    }
}
