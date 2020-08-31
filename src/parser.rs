use pest_consume::{match_nodes, Error, Parser};

pub use super::types::*;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;


#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct NectarParser;

#[pest_consume::parser]
impl NectarParser {
	fn alpha(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn digit(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn identifier(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn string_content(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn string(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn integer(input: Node) -> Result<f64> {
		input
			.as_str()
			.parse::<f64>()
			.map_err(|e| input.error(e))
	}
	fn decimal(input: Node) -> Result<f64> {
		input
			.as_str()
			.parse::<f64>()
			.map_err(|e| input.error(e))
	}
	fn number(input: Node) -> Result<f64> {
		input
			.as_str()
			.parse::<f64>()
			.map_err(|e| input.error(e))
	}

	fn unit(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn quantity(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn property_expression(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn noun(input: Node) -> Result<NectarNoun> {
		Ok(input.as_str())
	}
	fn noun_entity(input: Node) -> Result<NectarNounEntity> {
		Ok(match_nodes!(input.into_children();
			[noun(nouns)..] =>
				nouns.collect()
		))
	}
	fn nouns(input: Node) -> Result<Vec<NectarNounEntity>> {
		Ok(match_nodes!(input.into_children();
			[noun_entity(nouns)..] =>
				nouns.collect()
		))
	}

	fn category(input: Node) -> Result<NectarCategory> {
		Ok(input.as_str())
	}
	fn category_entity(input: Node) -> Result<NectarCategoryEntity> {
		Ok(match_nodes!(input.into_children();
			[category(categories)..] =>
				categories.collect()
		))
	}
	fn categories(input: Node) -> Result<Vec<NectarCategoryEntity>> {
		Ok(match_nodes!(input.into_children();
			[category_entity(categories)..] =>
				categories.collect()
		))
	}

	fn property(input: Node) -> Result<NectarProperty> {
		Ok(input.as_str())
	}

	fn relation(input: Node) -> Result<NectarRelation> {
		Ok(input.as_str())
	}

	fn aka_predicate(input: Node) -> Result<NectarNounEntity> {
		Ok(match_nodes!(input.into_children();
			[noun_entity(noun_entity)] =>
				noun_entity
		))
	}
	fn categorization_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[categories(categories)] =>
				NectarPredicate::Categorization { categories }
		))
	}
	fn has_property_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[property(property), property_expression(expression)] =>
				NectarPredicate::HasProperty { property, expression }
		))
	}
	fn relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), nouns(objects)] =>
				NectarPredicate::Relation { relation, objects }
		))
	}
	fn hyper_relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), categories(categories)] =>
				NectarPredicate::HyperRelation { relation, categories }
		))
	}

	fn predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[categorization_predicate(categorization_predicate)] =>
				categorization_predicate,
			[has_property_predicate(has_property_predicate)] =>
				has_property_predicate,
			[relation_predicate(relation_predicate)] =>
				relation_predicate,
			[hyper_relation_predicate(hyper_relation_predicate)] =>
				hyper_relation_predicate
		))
	}
	fn predicates(input: Node) -> Result<Vec<NectarPredicate>> {
		Ok(match_nodes!(input.into_children();
			[predicate(predicates)..] =>
				predicates.collect()
		))
	}

	fn statement(input: Node) -> Result<NectarCompoundStatement> {
		Ok(match_nodes!(input.into_children();
			[nouns(subjects), predicates(predicates)] =>
				NectarCompoundStatement {subjects, predicates},
			[nouns(subjects)] =>
				NectarCompoundStatement {subjects, predicates: vec!()},
		))
	}
	fn statements(input: Node) -> Result<Vec<NectarCompoundStatement>> {
		Ok(match_nodes!(input.into_children();
			[statement(statements)..] =>
				statements.collect(),
		))
	}
}

pub fn parse(input_str: &str) -> Result<Vec<NectarCompoundStatement>> {
	// Parse the input into `Nodes`
	let inputs = NectarParser::parse(Rule::statements, input_str)?;
	// There should be a single root node in the parsed tree
	let input = inputs.single()?;
	// Consume the `Node` recursively into the final value
	NectarParser::statements(input)
}