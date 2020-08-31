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

	export type PropertyExpression = {type: string, expression: any}

	export type Comparator = string

	// export type AkaPredicate = {
	// 	nouns: NounEntity
	// }
	// export type HasPropertyPredicate = {
	// 	property: Identifier.Property,
	// 	expression: PropertyExpression
	// }
	// export type CategoryPredicate = {
	// 	categories: CategoryEntity[]
	// }
	// export type RelationPredicate = {
	// 	relation: Identifier.Relation,
	// 	objects: NounEntity[]
	// }
	// export type HyperRelationPredicate = {
	// 	relation: Identifier.Relation,
	// 	objectCategories: CategoryEntity[]
	// }
	// export type Predicate = HasPropertyPredicate | CategoryPredicate | RelationPredicate | HyperRelationPredicate
	export type Predicate = any

	export type Declaration = {
		subjects: NounEntity[],
		predicates: [string, Predicate][]
	}

	export type Query = any
}