import { parse_to_json } from "../pkg/nectar_lib.js"
import { Identifier, Raw } from "./types.ts"

class Entity {
	static uid = 0
	static generateID(){
		return ++this.uid
	}

	id = Entity.generateID()

	// Canonical name for the entity (defaults to first)
	constructor(
		public name?: Identifier.Entity
	){}

	// Ids this entity is known by
	aliases = new Set<Identifier.Entity>()

	// Entities referenced in ancestor scopes
	references = new Set<Entity>()

	// Properties
	properties: Record<Identifier.Property, Raw.Value> = {}
	// properties = new Map<PropertyEntity, Nectar.Value>()

	toString(includeProperties = false){
		return [
			this.aliases.size ? [...this.aliases].join("/") : this.name ?? this.id,
			includeProperties && Object.keys(this.properties).length && this.toJSON("  ")
		].filter(Boolean).join(" ")

		// const aliases = [...this.aliases].filter(alias => alias != this.name)
		// return [
		// 	this.name ?? this.id,
		// 	aliases.length && `(aka ${aliases.join(", ")})`,
		// 	Object.keys(this.properties).length && this.toJSON("    ")
		// ].filter(Boolean).join(" ")
	}

	toJSON(space?: string){
		return JSON.stringify(this.properties, null, space)
	}
}

class NounEntity extends Entity {
	references = new Set<NounEntity>()
}

class CategoryEntity extends Entity {
	references = new Set<CategoryEntity>()
}

class RelationEntity extends Entity {
	references = new Set<RelationEntity>()

	constructor(
		public subject: NounEntity,
		public relation: Identifier.Relation,
		public object: NounEntity
	){
		super()
	}

	toString(){
		return `${this.subject} ${this.relation} ${this.object}`
	}
}

class CategorizationEntity extends Entity {
	references = new Set<RelationEntity>()

	constructor(
		public subject: NounEntity,
		public category: CategoryEntity
	){
		super()
	}

	toString(){
		return `${this.subject} is ${this.category}`
	}
}

class HyperRelationEntity extends Entity {
	references = new Set<HyperRelationEntity>()
	
	constructor(
		public subjectCategory: CategoryEntity,
		public relation: Identifier.Relation,
		public objectCategory: CategoryEntity
	){
		super()
	}
}

class RuleEntity extends Entity {
	references = new Set<HyperRelationEntity>()
	
	constructor(
	){
		super()
	}
}

type PropertyExpression =
	(() => PropertyExpression) |
	string |
	number |
	PropertyExpressionQuantity

class PropertyExpressionQuantity {
	constructor(
		public number: number,
		public unit: Identifier.Unit
	){}
}

class Scope {
	parent?: Scope
	
	// Table of nouns declared in this scope
	nounLookup: Record<Identifier.Noun, NounEntity> = {}
	// Table of categories declared in this scope
	categoryLookup: Record<Identifier.Category, CategoryEntity> = {}

	nouns = new Set<Entity>()
	categories = new Set<Entity>()
	categorizations = new Set<CategorizationEntity>()
	relations = new Set<RelationEntity>()
	hyperRelations = new Set<HyperRelationEntity>()
	
	lookup(type: Function, id: Identifier.Entity): Entity | undefined {
		const lookupTable = this.getLookupTable(type)
		return lookupTable[id] ?? this.parent?.lookup(type, id)
	}

	private getLookupTable(type: Function): Record<string, Entity> {
		switch(type){
			case NounEntity: return this.nounLookup
			case CategoryEntity: return this.categoryLookup
			default: return {}
		}
	}
	private getEntitySet(type: Function): Set<Entity> {
		switch(type){
			case NounEntity: return this.nouns
			case CategoryEntity: return this.categories
			default: return new Set()
		}
	}

	private declareEntity(type: Function, ids: Raw.NounEntity | Raw.CategoryEntity) {
		const lookupTable = this.getLookupTable(type)
		const entities = this.getEntitySet(type)

		// Find existing local entities for these nouns
		const existingLocalEntities = ids
			.map((id: Identifier.Noun) => lookupTable[id])
			.filter(Boolean)
		
		// Merge entities or create a new one
		const [entity = new Entity(ids[0]), ...otherEntities] = existingLocalEntities
		for(const otherEntity of otherEntities){
			for(const reference of otherEntity.references)
				entity.references.add(reference)
			for(const alias of otherEntity.aliases)
				entity.aliases.add(alias)
			entities.delete(otherEntity)
		}

		// Declare references to the merged entity
		for(const id of ids){
			lookupTable[id] = entity
			entity.aliases.add(id)
		}
		
		// If parent entities exist, reference them
		for(const id of ids)
			if(this.parent){
				const parentEntity = this.parent.lookup(type, id)
				if(parentEntity){
					entity.references.add(parentEntity)
					for(const alias of parentEntity.aliases)
						entity.aliases.add(alias)
				}
			}

		entities.add(entity)
	}

	private declareNounEntities(nouns: Raw.NounEntity[]){
		for(const noun of nouns)
			this.declareEntity(NounEntity, noun)
	}

	private declareCategoryEntities(categories: Raw.CategoryEntity[]){
		for(const category of categories)
			this.declareEntity(CategoryEntity, category)
	}

	private lookupNounEntities(nounEntities: Raw.NounEntity[]): Entity[] {
		// @ts-ignore
		// Lookup by the first identifier
		return nounEntities.map(([noun]) => this.lookup(NounEntity, noun))
	}

	private lookupCategoryEntities(categoryEntities: Raw.CategoryEntity[]): Entity[] {
		// @ts-ignore
		// Lookup by the first identifier
		return categoryEntities.map(([category]) => this.lookup(CategoryEntity, category))
	}

	private declareProperty(property: Identifier.Property){

	}

	private evalExpression({type, expression}: Raw.PropertyExpression, thisEntity?: Entity): PropertyExpression {
		switch(type){
			case "reference":
				return () => this.lookup(NounEntity, expression.noun)?.properties[expression.property] ?? ""
			case "number":
				return Number(expression)
			case "string":
				return expression
			case "quantity":
				return new PropertyExpressionQuantity(expression.number, expression.unit)
			default:
				return ""
		}
	}

	private declarePredicate(type: string, predicate: Raw.Predicate) {
		switch(type){
			case "aka":
				this.declareEntity(NounEntity, predicate.nouns)
				return
			case "hasProperty":
				this.declareProperty(predicate.property)
				return
			case "categorization":
				this.declareCategoryEntities(predicate.categories)
				return
			case "relation":
				this.declareNounEntities(predicate.objects)
				return
			case "hyperRelation":
				this.declareCategoryEntities(predicate.categories)
				return
		}
	}
	private declarePredicates(predicates: {type: string, predicate: Raw.Predicate}[]) {
		for(const {type, predicate} of predicates)
			this.declarePredicate(type, predicate)
	}

	private evalPredicate(subjects: Entity[], type: string, predicate: Raw.Predicate) {
		switch(type){
			case "hasProperty": {
				const {property, expression} = predicate
				for(const subject of subjects)
					subject.properties[property] = this.evalExpression(expression, subject)
				return
			}
			case "categorization": {
				const categories = this.lookupCategoryEntities(predicate.categories)
				for(const subject of subjects)
					for(const category of categories){
						const entity = new CategorizationEntity(
							subject,
							category
						)
						this.categorizations.add(entity)
					}
				return
			}
			case "relation": {
				const objects = this.lookupNounEntities(predicate.objects)
				for(const subject of subjects)
					for(const object of objects){
						const entity = new RelationEntity(
							subject,
							predicate.relation,
							object
						)
						this.relations.add(entity)
					}
				return
			}
			case "hyperRelation": {
				const categories = this.lookupCategoryEntities(predicate.categories)
				for(const subjectCategory of subjects)
					for(const objectCategory of categories){
						const entity = new HyperRelationEntity(
							subjectCategory,
							predicate.relation,
							objectCategory
						)
						this.hyperRelations.add(entity)
					}
				return
			}
		}
	}

	private evalDeclaration(subjects: Entity[], predicates: {type: string, predicate: Raw.Predicate}[]) {
		for(const {type, predicate} of predicates)
			this.evalPredicate(subjects, type, predicate)
	}

	private evalJunction<T>({type, items}: {type: string, items: T[]}, callback: (item: T) => boolean): boolean {
		if(type === "disjunction")
			return items.some(callback)
		else if(type === "conjunction")
			return items.every(callback)
		else
			return false
	}

	private evalQuery({type, query}: {type: string, query: Raw.Query}){
		switch(type){
			case "aka": {
				const {nounsLeft, nounsRight} = query
				return this.evalJunction<Raw.Noun>(nounsLeft, nounLeft =>
					this.evalJunction<Raw.Noun>(nounsRight, nounRight =>
						this.lookup(NounEntity, nounLeft) === this.lookup(NounEntity, nounRight)
					)
				)
			}
			case "propertyComparison": {
				const {propertyExpressionsLeft, comparators, propertyExpressionsRight} = query
				return this.evalJunction<Raw.PropertyExpression>(propertyExpressionsLeft, leftExp => {
					const leftValue = this.evalExpression(leftExp)
					return this.evalJunction<Raw.PropertyExpression>(propertyExpressionsRight, rightExp => {
						const rightValue = this.evalExpression(rightExp)
						return this.evalJunction<Raw.Comparator>(comparators, comparator => {
							switch(comparator){
								case "equalTo":
									return leftValue === rightValue
								case "lessThan":
									return leftValue < rightValue
								case "greaterThan":
									return leftValue > rightValue
								case "lessThanOrEqualTo":
									return leftValue <= rightValue
								case "greaterThanOrEqualTo":
									return leftValue >= rightValue
								default:
									return false 
							}
						})
					})
				})
			}
			case "categorization": {
				const {nouns, categories} = query
				return this.evalJunction<Raw.Noun>(nouns, noun =>
					this.evalJunction<Raw.Category>(categories, category => {
						// TODO: Optimize
						for(const categorization of this.categorizations)
							if(noun === categorization.subject && category === categorization.category)
								return true
						return false
					})
				)
			}
			case "relation": {
				const {subjects, relation, objects} = query
				return this.evalJunction<Raw.Noun>(subjects, subject =>
					this.evalJunction<Raw.Noun>(objects, object => {
						// TODO: Optimize
						for(const relation of this.relations)
							if(subject === relation.subject && object === relation.object)
								return true
						return false
					})
				)
			}
		}
	}

	evalProgram(program: Raw.Declaration[]) {
		// Distinguish declarations and queries
		const declarations = []
		const queries = []
		for(const {type, statement} of program)
			if(type === "declaration")
				declarations.push(statement)
			else if(type === "query")
				queries.push(statement)
		
		// Declare all noun entities and category entities
		for(const declaration of declarations){
			this.declareNounEntities(declaration.subjects)
			this.declarePredicates(declaration.predicates)
		}

		// Transform declarations into relation, categorization, and rule entities
		for(const declaration of declarations){
			const subjects = this.lookupNounEntities(declaration.subjects)
			this.evalDeclaration(subjects, declaration.predicates)

			// for(const entity of subjects)
			// 	console.log(entity)
			// console.log("nouns", this.nouns)
			// console.log("categories", this.categories)
		}

		// Perform queries
		const results = queries.map(query => this.evalQuery(query))
		return results
	}

	debug(){
		return [
			["Nouns", Array.from(this.nouns, noun => noun.toString(true))],
			["Categories", Array.from(this.categories, String)],
			["Categorizations", Array.from(this.categorizations, String)],
			["Relations", Array.from(this.relations, String)],
			["Hyper-Relations", Array.from(this.hyperRelations, String)],
		].filter(x => x[1].length)

		// console.log("Entities:")
		// for(const entity of this.entities)
		// 	console.log(entity.toString())
	}
}

export class NectarInterpreter {
	scope = new Scope()

	evaluate(contents: string){
		const program = JSON.parse(parse_to_json(contents))
		if(program.type === "error")
			throw program.message

		// console.log("->", program)
		return this.scope.evalProgram(program)
	}

	debug(){
		// Print a table of the current state
		console.log(
			// @ts-ignore
			"\n" + this.scope.debug().map(([k, [v, ...vs]]) =>
				// [(k + ": ").padStart(18) + v, ...vs].join("\n" + " ".repeat(18))
				[(k + ": ").padStart(18) + v, ...vs].join("\n").replace(/\n/g, "\n" + " ".repeat(18))
			).join("\n\n")
		)
	}
}