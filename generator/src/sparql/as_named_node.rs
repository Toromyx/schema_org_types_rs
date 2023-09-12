use oxigraph::model::{NamedNode, Term};

pub trait AsNamedNode {
    fn as_named_node(&self) -> &NamedNode;
}

impl AsNamedNode for Term {
    fn as_named_node(&self) -> &NamedNode {
        match self {
            Term::NamedNode(named_node) => named_node,
            _ => {
                panic!("Expected the term to be the variant NamedNode");
            }
        }
    }
}
