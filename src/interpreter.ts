import { parse_to_json } from "../pkg/nectar_lib.js"
import { Identifier, Raw } from './types.ts'

class Entity {
	// Entities referenced in ancestor scopes
	references = new Set<Entity>();

	// Properties
	properties: Record<Identifier.Property, Raw.Value> = {};
	// properties = new Map<PropertyEntity, Nectar.Value>();

	id = Entity.generateID();

	toString(){
		return this.id
	}

	static id = 0;
	static generateID(){
		return ++this.id
	}
}

class NounEntity extends Entity {
	references = new Set<NounEntity>();
}

class CategoryEntity extends Entity {
	references = new Set<CategoryEntity>();
}

class RelationEntity extends Entity {
	references = new Set<RelationEntity>();

	constructor(
		subject: NounEntity,
		relation: Identifier.Relation,
		object: NounEntity
	){
		super()
	}
}

class CategorizationEntity extends Entity {
	references = new Set<RelationEntity>();

	constructor(
		subject: NounEntity,
		relation: Identifier.Relation,
		category: CategoryEntity
	){
		super()
	}
}

class HyperRelationEntity extends Entity {
	references = new Set<HyperRelationEntity>();
	
	constructor(
		subjectCategory: CategoryEntity,
		relation: Identifier.Relation,
		objectCategory: CategoryEntity
	){
		super()
	}
}

class RuleEntity extends Entity {
	references = new Set<HyperRelationEntity>();
	
	constructor(
	){
		super()
	}
}

class Scope {
	parent?: Scope;

	// Entities defined in this scope
	entities = new Set<Entity>();
	
	// Nouns declared in this scope
	nouns: Record<Identifier.Noun, NounEntity> = {};

	// Categories declared in this scope
	categories: Record<Identifier.Category, CategoryEntity> = {};
	
	lookup(type: Function, id: Identifier.Noun | Identifier.Category): Entity | undefined {
		const lookupTable = this.getLookupTable(type)
		return lookupTable[id] ?? this.parent?.lookup(type, id)
	}

	private getLookupTable(type: Function): Record<string, Entity> {
		switch(type){
			case NounEntity: return this.nouns
			case CategoryEntity: return this.categories
		}
		return {}
	}

	private declareEntity(type: Function, ids: Raw.NounEntity | Raw.CategoryEntity) {
		const lookupTable = this.getLookupTable(type)

		// Find existing local entities for these nouns
		const existingLocalEntities = ids
			.map((id: Identifier.Noun) => lookupTable[id])
			.filter(Boolean)
		
		// Merge entities or create a new one
		const [entity = new Entity(), ...otherEntities] = existingLocalEntities
		for(const {references} of otherEntities)
			for(const reference of references)
				entity.references.add(reference)

		// Declare references to the entity
		for(const id of ids)
			lookupTable[id] = entity
		
		// If parent entities exist, reference them
		for(const id of ids)
			if(this.parent){
				const parentEntity = this.parent.lookup(type, id)
				if(parentEntity)
					entity.references.add(parentEntity)
			}

		this.entities.add(entity)
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

	private declarePredicate(type: string, predicate: Raw.Predicate) {
		switch(type){
			case "hasProperty":
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

	// private evalPredicates(predicates: [string, Raw.Predicate][]): {[key: string]: Set<any>} {
	// 	const predicatesByType: {[key: string]: Set<Entity>} = {
	// 		is: new Set<CategoryEntity>(),
	// 		hasProperty: new Set<Raw.HasPropertyPredicate>(),
	// 		relation: new Set<Identifier.RelationPredicate>(),
	// 		hyperRelation: new Set<Raw.HyperRelationPredicate>(),
	// 	}
		
	// 	for(const [type, predicate] of predicates)
	// 		predicatesByType[type].add(this.evalPredicate(type, predicate))

	// 	return predicatesByType
	// }

	evalProgram(program: Raw.CompoundStatement[]) {
		// Declare all noun entities and category entities
		for(const compoundStatement of program){
			this.declareNounEntities(compoundStatement.subjects)
			this.declarePredicates(compoundStatement.predicates)
		}

		// Transform statements into relation, categorization, and rule entities
		for(const compoundStatement of program){
			const subjects = this.lookupNounEntities(compoundStatement.subjects)
			// const predicatesByType = this.evalPredicates(compoundStatement.predicates)
			// console.log(predicatesByType)

			for(const entity of subjects)
				console.log(entity)
			console.log(this.nouns)
			console.log(this.categories)
		}
	}
}

export class NectarInterpreter {
	scope = new Scope();

	evaluate(contents: string){
		try {
			const program = JSON.parse(parse_to_json(contents))
			console.log("->", program)

			this.scope.evalProgram(program)
		}catch(e){
			console.error(e)
		}
	}
}