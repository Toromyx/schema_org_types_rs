use std::{str::FromStr, sync::OnceLock};

use oxigraph::model::{GraphName, NamedNode, Quad, Subject, Term};

static INSERT_QUADS: OnceLock<Vec<Quad>> = OnceLock::new();

pub fn insert_quads() -> &'static [Quad] {
	INSERT_QUADS.get_or_init(|| {
		vec![
			Quad::new(
				Subject::NamedNode(
					NamedNode::from_str("<https://schema.org/recipeInstructions>").unwrap(),
				),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToSection>").unwrap()),
				GraphName::DefaultGraph,
			),
			Quad::new(
				Subject::NamedNode(
					NamedNode::from_str("<https://schema.org/recipeInstructions>").unwrap(),
				),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToStep>").unwrap()),
				GraphName::DefaultGraph,
			),
			Quad::new(
				Subject::NamedNode(
					NamedNode::from_str("<https://schema.org/itemListElement>").unwrap(),
				),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToSection>").unwrap()),
				GraphName::DefaultGraph,
			),
			Quad::new(
				Subject::NamedNode(
					NamedNode::from_str("<https://schema.org/itemListElement>").unwrap(),
				),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToStep>").unwrap()),
				GraphName::DefaultGraph,
			),
			Quad::new(
				Subject::NamedNode(NamedNode::from_str("<https://schema.org/item>").unwrap()),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToSection>").unwrap()),
				GraphName::DefaultGraph,
			),
			Quad::new(
				Subject::NamedNode(NamedNode::from_str("<https://schema.org/item>").unwrap()),
				NamedNode::from_str("<https://schema.org/rangeIncludes>").unwrap(),
				Term::NamedNode(NamedNode::from_str("<https://schema.org/HowToStep>").unwrap()),
				GraphName::DefaultGraph,
			),
		]
	})
}
