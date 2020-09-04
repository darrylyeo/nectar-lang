use pest_consume::{match_nodes, Error, Parser};

pub use super::types::*;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;


fn vecOfTupleToTupleOfVec<A, B>(v: std::vec::IntoIter<(A, B)>) -> (Vec<A>, Vec<B>){
	let a = v.map(|(a, _)| a);
	let b = v.map(|(_, b)| b);
	return (a.collect(), b.collect())
}


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
				nouns.collect(),
			[noun(nouns).., noun_disjunction(noun_disjunction)] =>
				match noun_disjunction {
					NectarNounJunction::Disjunction(nouns2) => {
						let mut nouns = nouns.collect::<NectarNounEntity>();
						nouns.extend(nouns2);
						nouns
					},
					_ =>
						nouns.collect()
				},
		))
	}
	fn noun_phrase(input: Node) -> Result<(NectarNoun, Option<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[category_entities(categories), noun(noun)] => (
				noun,
				Some(NectarCategorization {noun, categories})
			),
			[noun(noun)] => (
				noun,
				None
			)
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
	fn noun_entity(input: Node) -> Result<(NectarNounEntity, Vec<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[noun_phrase(data)..] => {
				let (noun_entity, categorizations) = vecOfTupleToTupleOfVec(data);
				(
					noun_entity,
					categorizations.iter().filter_map(|categorization| *categorization)
					.collect()
				)
			}
			// [noun_phrase(data)..] => (
			// 	data
			// 		.map(|(noun, _)| noun)
			// 		.collect(),
			// 	data
			// 		.filter_map(|(_, categorization)| categorization)
			// 		.collect()
			// )
		))
	}
	fn noun_entities(input: Node) -> Result<(Vec<NectarNounEntity>, Vec<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[noun_entity(data)..] => {
				let (noun_entity, categorizations) = vecOfTupleToTupleOfVec(data);
				(
					noun_entity,
					categorizations.iter()
						.flat_map(|categorization| *categorization)
						.collect()
				)
			}
			// [noun_entity(data)..] => (
			// 	data
			// 		.map(|(noun_entity, _)| noun_entity)
			// 		.collect(),
			// 	data
			// 		.flat_map(|(_, categorizations)| categorizations)
			// 		.collect()
			// )
		))
	}

	fn category(input: Node) -> Result<NectarCategory> {
		Ok(input.as_str())
	}
	fn category_entity(input: Node) -> Result<NectarCategoryEntity> {
		Ok(match_nodes!(input.into_children();
			[category(categories)..] =>
				categories.collect(),
			[category(categories).., category_disjunction(category_disjunction)] =>
				match category_disjunction {
					NectarCategoryJunction::Disjunction(categories2) => {
						let mut categories = categories.collect::<NectarCategoryEntity>();
						categories.extend(categories2);
						categories
					},
					_ =>
						categories.collect()
				},
		))
	}
	fn category_entities(input: Node) -> Result<Vec<NectarCategoryEntity>> {
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
			[category_entities(categories)] =>
				NectarPredicate::Categorization {categories}
		))
	}
	fn has_property_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[property(property), property_expression(expression)] =>
				NectarPredicate::HasProperty {property, expression}
		))
	}
	fn relation_predicate(input: Node) -> Result<(NectarPredicate, Vec<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), noun_entities((objects, categorizations))] => (
				NectarPredicate::Relation {relation, objects},
				categorizations
			)
		))
	}
	fn hyper_relation_predicate(input: Node) -> Result<NectarPredicate> {
		Ok(match_nodes!(input.into_children();
			[relation(relation), category_entities(categories)] =>
				NectarPredicate::HyperRelation {relation, categories}
		))
	}

	fn predicate(input: Node) -> Result<(NectarPredicate, Vec<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[aka_predicate(aka_predicate)] =>
				(aka_predicate, vec!()),
			[has_property_predicate(has_property_predicate)] =>
				(has_property_predicate, vec!()),
			[categorization_predicate(categorization_predicate)] =>
				(categorization_predicate, vec!()),
			[relation_predicate((relation_predicate, categorizations))] =>
				(relation_predicate, categorizations),
			[hyper_relation_predicate(hyper_relation_predicate)] =>
				(hyper_relation_predicate, vec!()),
		))
	}
	fn predicates(input: Node) -> Result<(Vec<NectarPredicate>, Vec<NectarCategorization>)> {
		Ok(match_nodes!(input.into_children();
			[predicate(data)..] => {
				let (predicates, categorizations) = vecOfTupleToTupleOfVec(data);
				(
					predicates,
					categorizations.iter()
						.flat_map(|categorizations| *categorizations)
						.collect()
				)
			}
			// [predicate(data)..] => (
			// 	data
			// 		.map(|(predicate, _)| predicate)
			// 		.collect(),
			// 	data
			// 		.flat_map(|(_, categorizations)| categorizations)
			// 		.collect()
			// )
		))
	}

	fn declaration(input: Node) -> Result<NectarStatement> {
		Ok(match_nodes!(input.into_children();
			[noun_entities((subjects, categorizations)), predicates((predicates, categorizations2))] => {
				// categorizations.extend(categorizations2);
				NectarStatement::Declaration {
					subjects,
					predicates,
					categorizations,
					categorizations2
				}
			},
			[noun_entities((subjects, categorizations))] =>
				NectarStatement::Declaration {
					subjects,
					predicates: vec!(),
					categorizations: categorizations,
					categorizations2: vec!()
				},
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