namespace Identifier {
	export type Entity = Noun | Category

	export type Noun = string
	export type Category = string
	export type Relation = string

	export type Unit = string
	export type Property = string
}

namespace Raw {
	export type NounEntity = Identifier.Noun[]

	export type CategoryEntity = Identifier.Category[]

	export type Quantity = {
		number: number,
		unit: Identifier.Unit
	}

	export type Value = number | string | Quantity

	export type PropertyExpression = string
	// export enum PropertyExpression {}

	export type AkaPredicate = {
		nouns: NounEntity
	}
	export type HasPropertyPredicate = {
		property: Identifier.Property,
		expression: PropertyExpression
	}
	export type CategoryPredicate = {
		categories: CategoryEntity[]
	}
	export type RelationPredicate = {
		relation: Identifier.Relation,
		objects: NounEntity[]
	}
	export type HyperRelationPredicate = {
		relation: Identifier.Relation,
		objectCategories: CategoryEntity[]
	}
	export type Predicate = HasPropertyPredicate | CategoryPredicate | RelationPredicate | HyperRelationPredicate

	export type CompoundStatement = {
		subjects: NounEntity[],
		predicates: [string, Predicate][]
	}
}