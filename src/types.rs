pub type NectarNoun<'a> = &'a str;
pub type NectarSubject<'a> = Vec<NectarNoun<'a>>;

pub type NectarCategory<'a> = &'a str;
pub type NectarCategorization<'a> = Vec<NectarCategory<'a>>;

pub type NectarUnit<'a> = &'a str;

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

#[derive(Debug)]
pub enum NectarPredicate<'a> {
	IsA {
		categorizations: Vec<NectarCategorization<'a>>
	},
	HasProperty {
		property: NectarProperty<'a>,
		expression: &'a str,
		// expression: NectarExpression<'a>
	},
	Relation {
		relation: NectarRelation<'a>,
		object: NectarSubject<'a>
	},
	HyperRelation {
		relation: NectarRelation<'a>,
		categorizations: Vec<NectarCategorization<'a>>
	}
}

#[derive(Debug)]
pub struct NectarCompoundStatement<'a> {
	pub subjects: Vec<NectarSubject<'a>>,
	pub predicates: Vec<NectarPredicate<'a>>
}

pub struct NectarStatement<'a> {
	subject: NectarSubject<'a>,
	predicate: NectarPredicate<'a>
}