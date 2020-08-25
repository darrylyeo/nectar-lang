use pest_consume::{match_nodes, Error, Parser};

use super::types::*;

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

	fn expression(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn noun(input: Node) -> Result<NectarNoun> {
		Ok(input.as_str())
	}
	fn subject(input: Node) -> Result<NectarSubject> {
		Ok(match_nodes!(input.into_children();
			[noun(nouns)..] => nouns.collect()
		))
	}
	fn subjects(input: Node) -> Result<Vec<NectarSubject>> {
		Ok(match_nodes!(input.into_children();
			[subject(subjects)..] => subjects.collect()
		))
	}

	fn category(input: Node) -> Result<NectarCategory> {
		Ok(input.as_str())
	}
	fn categorization(input: Node) -> Result<NectarCategorization> {
		Ok(match_nodes!(input.into_children();
			[category(categories)..] => categories.collect()
		))
	}
	fn categorizations(input: Node) -> Result<Vec<NectarCategorization>> {
		Ok(match_nodes!(input.into_children();
			[categorization(categorizations)..] => categorizations.collect()
		))
	}

	fn property(input: Node) -> Result<NectarProperty> {
		Ok(input.as_str())
	}

	fn relation(input: Node) -> Result<NectarRelation> {
		Ok(input.as_str())
	}

	fn is_a_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[categorizations(categorizations)] => NectarPredicate::IsA { categorizations }
		))
	}
	fn has_property_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[property(property), expression(expression)] => NectarPredicate::HasProperty { property, expression }
		))
	}
	fn relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), subject(object)] => NectarPredicate::Relation { relation, object }
		))
	}
	fn hyper_relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), categorizations(categorizations)] => NectarPredicate::HyperRelation { relation, categorizations }
		))
	}

	fn predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[is_a_predicate(is_a_predicate)] => is_a_predicate,
			[has_property_predicate(has_property_predicate)] => has_property_predicate,
			[relation_predicate(relation_predicate)] => relation_predicate,
			[hyper_relation_predicate(hyper_relation_predicate)] => hyper_relation_predicate
		))
	}
	fn predicates(input: Node) -> Result<Vec<NectarPredicate>> {
		Ok(match_nodes!(input.into_children();
			[predicate(predicates)..] => predicates.collect()
		))
	}

	fn compound_statement(input: Node) -> Result<NectarCompoundStatement> {
		Ok(match_nodes!(input.into_children();
			[subjects(subjects), predicates(predicates)] => NectarCompoundStatement {subjects, predicates},
		))
	}
	fn statements(input: Node) -> Result<Vec<NectarCompoundStatement>> {
		Ok(match_nodes!(input.into_children();
			[compound_statement(statements)..] => statements.collect(),
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