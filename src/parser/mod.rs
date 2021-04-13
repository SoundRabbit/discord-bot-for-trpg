use async_std::sync::Arc;

pub mod ast;

peg::parser! {
    pub grammar context() for str {
        pub rule parse() ->  ast::Proc
            = precedence! {
                dlm()? p: proc() dlm()? { p }
                dlm()? "`" dlm()? p: proc() dlm()? "`" dlm()? { p }
                dlm()? "```" dlm()? p: proc() dlm()? "```" dlm()? { p }
            }

        rule proc() -> ast::Proc
            = exprs: expr0() ** (dlm()? ";" dlm()?) { ast::Proc::new(exprs.into_iter().map(|expr| Arc::new(expr)).collect())}

        rule expr0() -> ast::Expr0
            = precedence! {
                i:ident() dlm()? ":=" dlm()? value: expr0() {ast::Expr0::Def {ident:i, value: Arc::new(value)}}
                --
                left:(@) dlm() right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from(" ")}}
                --
                arg:ident() dlm()? "=>" dlm()? value:expr0() {ast::Expr0::Fn {arg, value: Arc::new(value)}}
                --
                left:(@) dlm()? "#" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("#")}}
                --
                left:(@) dlm()? "==" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("==")}}
                left:(@) dlm()? "!=" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("!=")}}
                left:(@) dlm()? "<=" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("<=")}}
                left:(@) dlm()? ">=" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from(">=")}}
                --
                left:(@) dlm()? "<" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("<")}}
                left:(@) dlm()? ">" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from(">")}}
                --
                left:(@) dlm()? "+" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("+")}}
                left:(@) dlm()? "-" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("-")}}
                --
                left:(@) dlm()? "*" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("*")}}
                left:(@) dlm()? "/" dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("/")}}
                --
                left:(@) dlm()? "." dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from(".")}}
                --
                left:(@) dlm()? ("d"/"D") dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("d")}}
                left:(@) dlm()? ("b"/"B") dlm()? right:@ {ast::Expr0::Expr0{left: Arc::new(left), right: Arc::new(right), operator: String::from("b")}}
                --
                term:term() {ast::Expr0::Term(term)}
            }

        rule term() -> ast::Term
            = precedence! {
                "(" dlm()? expr:expr0() dlm()? ")" {ast::Term::Expr0(Arc::new(expr))}
                "{" dlm()? p:proc() dlm()? "}" {ast::Term::Proc(p)}
                "[" dlm()? exprs:expr0() ** (dlm()? "," dlm()?) dlm()? "]" {ast::Term::Array(exprs.into_iter().map(|exp0| Arc::new(exp0)).collect())}
                "{" dlm()? pairs:key_value() ** (dlm()? "," dlm()?) dlm()? "}" {ast::Term::Record(pairs.into_iter().map(|(key, exp0)| (key, Arc::new(exp0))).collect())}
                --
                literal:literal() {ast::Term::Literal(literal)}
            }

        rule key_value() -> (Arc<String>, ast::Expr0)
            = ident:ident() dlm()? ":" dlm()? expr:expr0() { (ident, expr) }

        rule literal() -> ast::Literal
            = precedence! {
                n:$(['0'..='9']+) { ast::Literal::Integer(n.parse().unwrap()) }
                i:ident() {ast::Literal::Ident(i)}
            }

        rule ident() -> Arc<String>
            = x:$(['A'..='Z' | 'a'..='z']) xs:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']*) { Arc::new(String::from(x) + xs) }

        rule dlm() = quiet!{[' ' | '\n' | '\t']+}
    }
}
