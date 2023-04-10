// TODO: Add support for more regex syntax
// ex. quantifiers ? + *, Dot character, Character classes [a-z] ...

/// (Incomplete) parser implementation for regular expressions.
/// Currently only support Concat and Literal expressions.
pub fn parse(pattern: &str) -> Ast {
    let mut concat = Concat { asts: Vec::new() };

    for c in pattern.chars() {
        let literal = Literal { c };
        concat.asts.push(Ast::Literal(literal));
    }

    Ast::Concat(concat)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Literal(Literal),
    Concat(Concat),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Concat {
    pub asts: Vec<Ast>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Literal {
    pub c: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literals() {
        const PATTERN: &str = "abcde";

        let ast = parse(PATTERN);

        dbg!(&ast);

        assert_eq!(
            ast,
            Ast::Concat(Concat {
                asts: vec![
                    Ast::Literal(Literal { c: 'a' }),
                    Ast::Literal(Literal { c: 'b' }),
                    Ast::Literal(Literal { c: 'c' }),
                    Ast::Literal(Literal { c: 'd' }),
                    Ast::Literal(Literal { c: 'e' })
                ]
            })
        )
    }
}
