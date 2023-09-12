use oxigraph::model::{Literal, Term};

pub trait AsLiteral {
    fn as_literal(&self) -> &Literal;
}

impl AsLiteral for Term {
    fn as_literal(&self) -> &Literal {
        match self {
            Term::Literal(literal) => literal,
            _ => {
                panic!("Expected the term to be the variant Literal");
            }
        }
    }
}
