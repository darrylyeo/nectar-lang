namespace Nectar {
	export type Noun = string
	export type Subject = Noun[]

	export type Category = string
	export type Categorization = Category[]

	export type Unit = string
	export type Quantity = {
		number: number,
		unit: Unit
	}

	export type Value = number | string | Quantity

	export type Expression = {
		
	}

	export type Property = string

	export type Relation = string

	export type IsAPredicate = {
		categorizations: Categorization[]
	}
	export type HasPropertyPredicate = {
		property: Property,
		expression: string,
		// expression: Expression
	}
	export type RelationPredicate = {
		relation: Relation,
		object: Subject
	}
	export type HyperRelationPredicate = {
		relation: Relation,
		categorizations: Categorization[]
	}
	export type Predicate = IsAPredicate | HasPropertyPredicate | RelationPredicate | HyperRelationPredicate

	export type CompoundStatement = {
		subjects: Subject[],
		predicates: Predicate[]
	}

	export type Statement = {
		subject: Subject,
		predicate: Predicate
	}
}