use serde::{Serialize};


pub type NectarNoun<'a> = &'a str;
pub type NectarNounEntity<'a> = Vec<NectarNoun<'a>>;
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "items", rename_all = "camelCase")]
pub enum NectarNounJunction<'a> {
	Disjunction(Vec<NectarNoun<'a>>),
	Conjunction(Vec<NectarNoun<'a>>)
}

pub type NectarCategory<'a> = &'a str;
pub type NectarCategoryEntity<'a> = Vec<NectarCategory<'a>>;
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "items", rename_all = "camelCase")]
pub enum NectarCategoryJunction<'a> {
	Disjunction(Vec<NectarCategory<'a>>),
	Conjunction(Vec<NectarCategory<'a>>)
}

pub type NectarUnit<'a> = &'a str;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NectarComparator {
	EqualTo,
	LessThan,
	GreaterThan,
	LessThanOrEqualTo,
	GreaterThanOrEqualTo
}
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "items", rename_all = "camelCase")]
pub enum NectarComparatorJunction {
	Disjunction(Vec<NectarComparator>)
}

pub type NectarProperty<'a> = &'a str;
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "expression", rename_all = "camelCase")]
pub enum NectarPropertyExpression<'a> {
	Reference {
		noun: NectarNoun<'a>,
		property: NectarProperty<'a>
	},
	Number(f64),
	String(&'a str),
	Quantity {
		number: f64,
		unit: NectarUnit<'a>
	}
}
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "items", rename_all = "camelCase")]
pub enum NectarPropertyExpressionJunction<'a> {
	Disjunction(Vec<NectarPropertyExpression<'a>>),
	Conjunction(Vec<NectarPropertyExpression<'a>>)
}

pub type NectarRelation<'a> = &'a str;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "predicate", rename_all = "camelCase")]
pub enum NectarPredicate<'a> {
	Aka {
		nouns: NectarNounEntity<'a>
	},
	HasProperty {
		property: NectarProperty<'a>,
		expression: NectarPropertyExpression<'a>
	},
	Categorization {
		categories: Vec<NectarCategoryEntity<'a>>
	},
	Relation {
		relation: NectarRelation<'a>,
		objects: Vec<NectarNounEntity<'a>>
	},
	HyperRelation {
		relation: NectarRelation<'a>,
		categories: Vec<NectarCategoryEntity<'a>>
	}
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "query", rename_all = "camelCase")]
pub enum NectarQuery<'a> {
	#[serde(rename_all = "camelCase")]
	Aka {
		nouns_left: NectarNounJunction<'a>,
		nouns_right: NectarNounJunction<'a>
	},
	#[serde(rename_all = "camelCase")]
	PropertyComparison {
		property_expressions_left: NectarPropertyExpressionJunction<'a>,
		comparators: NectarComparatorJunction,
		property_expressions_right: NectarPropertyExpressionJunction<'a>
	},
	Categorization {
		nouns: NectarNounJunction<'a>,
		categories: NectarCategoryJunction<'a>
	},
	Relation {
		subjects: NectarNounJunction<'a>,
		relation: NectarRelation<'a>,
		objects: NectarNounJunction<'a>,
	}
}

pub type NectarScopeName<'a> = &'a str;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "statement", rename_all = "camelCase")]
pub enum NectarStatement<'a> {
	Declaration {
		subjects: Vec<NectarNounEntity<'a>>,
		predicates: Vec<NectarPredicate<'a>>
	},
	Query(NectarQuery<'a>),
	Scope {
		name: NectarScopeName<'a>,
		statements: Vec<NectarStatement<'a>>
	}
}