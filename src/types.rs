use serde::{Serialize};


pub type NectarNoun<'a> = &'a str;
pub type NectarNounEntity<'a> = Vec<NectarNoun<'a>>;

pub type NectarCategory<'a> = &'a str;
pub type NectarCategoryEntity<'a> = Vec<NectarCategory<'a>>;

pub type NectarUnit<'a> = &'a str;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "predicate", rename_all = "camelCase")]
pub enum NectarValue<'a> {
	Number(f64),
	String(&'a str),
	Quantity {
		number: f64,
		unit: NectarUnit<'a>
	}
}

pub enum NectarExpression<'a> {
	Value(NectarValue<'a>)
}

pub type NectarProperty<'a> = &'a str;

pub type NectarRelation<'a> = &'a str;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "predicate", rename_all = "camelCase")]
pub enum NectarPredicate<'a> {
	Aka {
		nouns: NectarNounEntity<'a>
	},
	HasProperty {
		property: NectarProperty<'a>,
		expression: &'a str,
		// expression: NectarExpression<'a>
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
pub struct NectarCompoundStatement<'a> {
    pub subjects: Vec<NectarNounEntity<'a>>,
    pub predicates: Vec<NectarPredicate<'a>>
}

// pub struct NectarStatement<'a> {
// 	subject: NectarNounEntity<'a>,
// 	predicate: NectarPredicate<'a>
// }