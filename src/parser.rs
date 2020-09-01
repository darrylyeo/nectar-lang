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
		Ok(match_nodes!(input.into_children();
			[string_content(string_content)] =>
				string_content
		))
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
	fn quantity(input: Node) -> Result<NectarPropertyExpression> {
		Ok(match_nodes!(input.into_children();
			[number(number), unit(unit)] =>
				NectarPropertyExpression::Quantity {number, unit}
		))
	}

	fn comparator(input: Node) -> Result<NectarComparator> {
		Ok(match_nodes!(input.into_children();
			[equal_to] =>
				NectarComparator::EqualTo,
			[less_than] =>
				NectarComparator::LessThan,
			[greater_than] =>
				NectarComparator::GreaterThan,
			[less_than_or_equal_to] =>
				NectarComparator::LessThanOrEqualTo,
			[greater_than_or_equal_to] =>
				NectarComparator::GreaterThanOrEqualTo
		))
	}
	fn comparator_disjunction(input: Node) -> Result<NectarComparatorJunction> {
		Ok(match_nodes!(input.into_children();
			[comparator(comparator)..] =>
				NectarComparatorJunction::Disjunction(comparator.collect())
		))
	}

	fn property(input: Node) -> Result<NectarProperty> {
		Ok(input.as_str())
	}
	fn property_expression(input: Node) -> Result<NectarPropertyExpression> {
		Ok(match_nodes!(input.into_children();
			[property(property), noun(noun)] =>
				NectarPropertyExpression::Reference {property, noun},
			[noun(noun), property(property)] =>
				NectarPropertyExpression::Reference {property, noun},
			[string(string)] =>
				NectarPropertyExpression::String(string),
			[quantity(quantity)] =>
				quantity,
			[number(number)] =>
				NectarPropertyExpression::Number(number)
		))
	}
	fn property_expression_disjunction(input: Node) -> Result<NectarPropertyExpressionJunction> {
		Ok(match_nodes!(input.into_children();
			[property_expression(property_expressions)..] =>
				NectarPropertyExpressionJunction::Disjunction(property_expressions.collect())
		))
	}
	fn property_expression_conjunction(input: Node) -> Result<NectarPropertyExpressionJunction> {
		Ok(match_nodes!(input.into_children();
			[property_expression(property_expressions)..] =>
				NectarPropertyExpressionJunction::Conjunction(property_expressions.collect())
		))
	}
	fn property_expression_junction(input: Node) -> Result<NectarPropertyExpressionJunction> {
		Ok(match_nodes!(input.into_children();
			[property_expression_disjunction(property_expression_disjunction)] =>
				property_expression_disjunction,
			[property_expression_conjunction(property_expression_conjunction)] =>
				property_expression_conjunction
		))
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
	fn noun_disjunction(input: Node) -> Result<NectarNounJunction> {
		Ok(match_nodes!(input.into_children();
			[noun(nouns)..] =>
				NectarNounJunction::Disjunction(nouns.collect())
		))
	}
	fn noun_conjunction(input: Node) -> Result<NectarNounJunction> {
		Ok(match_nodes!(input.into_children();
			[noun(nouns)..] =>
				NectarNounJunction::Conjunction(nouns.collect())
		))
	}
	fn noun_junction(input: Node) -> Result<NectarNounJunction> {
		Ok(match_nodes!(input.into_children();
			[noun_disjunction(noun_disjunction)] =>
				noun_disjunction,
			[noun_conjunction(noun_conjunction)] =>
				noun_conjunction
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
	fn category_disjunction(input: Node) -> Result<NectarCategoryJunction> {
		Ok(match_nodes!(input.into_children();
			[category(categories)..] =>
				NectarCategoryJunction::Disjunction(categories.collect())
		))
	}
	fn category_conjunction(input: Node) -> Result<NectarCategoryJunction> {
		Ok(match_nodes!(input.into_children();
			[category(categories)..] =>
				NectarCategoryJunction::Conjunction(categories.collect())
		))
	}
	fn category_junction(input: Node) -> Result<NectarCategoryJunction> {
		Ok(match_nodes!(input.into_children();
			[category_disjunction(category_disjunction)] =>
				category_disjunction,
			[category_conjunction(category_conjunction)] =>
				category_conjunction
		))
	}

	fn relation(input: Node) -> Result<NectarRelation> {
		Ok(input.as_str())
	}

	fn aka_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[noun_disjunction(noun_disjunction)] =>
				match noun_disjunction {
					NectarNounJunction::Disjunction(nouns) =>
						NectarPredicate::Aka {nouns},
					_ =>
						NectarPredicate::Aka {nouns: vec!()}
				}
		))
	}
	fn categorization_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[categories(categories)] =>
				NectarPredicate::Categorization {categories}
		))
	}
	fn has_property_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[property(property), property_expression(expression)] =>
				NectarPredicate::HasProperty {property, expression}
		))
	}
	fn relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), nouns(objects)] =>
				NectarPredicate::Relation {relation, objects}
		))
	}
	fn hyper_relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), categories(categories)] =>
				NectarPredicate::HyperRelation {relation, categories}
		))
	}

	fn predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[aka_predicate(aka_predicate)] =>
				aka_predicate,
			[has_property_predicate(has_property_predicate)] =>
				has_property_predicate,
			[categorization_predicate(categorization_predicate)] =>
				categorization_predicate,
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

	fn declaration(input: Node) -> Result<NectarStatement> {
		Ok(match_nodes!(input.into_children();
			[nouns(subjects), predicates(predicates)] =>
				NectarStatement::Declaration {subjects, predicates},
			[nouns(subjects)] =>
				NectarStatement::Declaration {subjects, predicates: vec!()},
		))
	}

	fn aka_query(input: Node) -> Result<NectarQuery> {
		Ok(match_nodes!(input.into_children();
			[noun_junction(nouns_left), noun_junction(nouns_right)] =>
				NectarQuery::Aka {nouns_left, nouns_right}
		))
	}
	fn property_comparison_query(input: Node) -> Result<NectarQuery> {
		Ok(match_nodes!(input.into_children();
			[property_expression_junction(property_expressions_left), comparator_disjunction(comparators), property_expression_junction(property_expressions_right)] =>
				NectarQuery::PropertyComparison {property_expressions_left, comparators, property_expressions_right},
			[noun_junction(noun_junction), property(property), comparator_disjunction(comparators), property_expression_junction(property_expressions_right)] =>
				match noun_junction {
					NectarNounJunction::Disjunction(nouns) =>
						NectarQuery::PropertyComparison {
							property_expressions_left: NectarPropertyExpressionJunction::Disjunction(
								nouns
									.iter()
									.map(|noun| NectarPropertyExpression::Reference {noun, property})
									.collect()
							),
							comparators,
							property_expressions_right
						},
					NectarNounJunction::Conjunction(nouns) =>
						NectarQuery::PropertyComparison {
							property_expressions_left: NectarPropertyExpressionJunction::Conjunction(
								nouns
									.iter()
									.map(|noun| NectarPropertyExpression::Reference {noun, property})
									.collect()
							),
							comparators,
							property_expressions_right
						}
				}
		))
	}
	fn categorization_query(input: Node) -> Result<NectarQuery> {
		Ok(match_nodes!(input.into_children();
			[noun_junction(nouns), category_junction(categories)] =>
				NectarQuery::Categorization {nouns, categories}
		))
	}
	fn relation_query(input: Node) -> Result<NectarQuery> {
		Ok(match_nodes!(input.into_children();
			[noun_junction(subjects), relation(relation), noun_junction(objects)] =>
				NectarQuery::Relation {subjects, relation, objects}
		))
	}

	fn query(input: Node) -> Result<NectarQuery> {
		Ok(match_nodes!(input.into_children();
			[aka_query(aka_query)] =>
				aka_query,
			[property_comparison_query(property_comparison_query)] =>
				property_comparison_query,
			[categorization_query(categorization_query)] =>
				categorization_query,
			[relation_query(relation_query)] =>
				relation_query
		))
	}

	fn scope(input: Node) -> Result<NectarStatement> {
		Ok(match_nodes!(input.into_children();
			[identifier(name), statements(statements)] =>
				NectarStatement::Scope {name, statements}
		))
	}

	fn statement(input: Node) -> Result<NectarStatement> {
		Ok(match_nodes!(input.into_children();
			[declaration(declaration)] =>
				declaration,
			[query(query)] =>
				NectarStatement::Query(query),
			[scope(scope)] =>
				scope
		))
	}
	fn statements(input: Node) -> Result<Vec<NectarStatement>> {
		Ok(match_nodes!(input.into_children();
			[statement(statements)..] =>
				statements.collect(),
		))
	}
}

pub fn parse(input_str: &str) -> Result<Vec<NectarStatement>> {
	// Parse the input into `Nodes`
	let inputs = NectarParser::parse(Rule::statements, input_str)?;
	// There should be a single root node in the parsed tree
	let input = inputs.single()?;
	// Consume the `Node` recursively into the final value
	NectarParser::statements(input)
}