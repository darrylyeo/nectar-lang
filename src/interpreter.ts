import { parse_to_json } from "../pkg/nectar_lib.js"
import { Identifier, Raw } from "./types.ts"

class Entity {
	static uid = 0;
	static generateID(){
		return ++this.uid
	}

	id = Entity.generateID();

	// Canonical name for the entity (defaults to first)
	constructor(
		public name?: Identifier.Entity
	){}

	// Ids this entity is known by
	aliases = new Set<Identifier.Entity>();

	// Entities referenced in ancestor scopes
	references = new Set<Entity>();

	// Properties
	properties: Record<Identifier.Property, Raw.Value> = {};
	// properties = new Map<PropertyEntity, Nectar.Value>();

	toString(){
		const aliases = [...this.aliases].filter(alias => alias != this.name)
		return [
			this.name ?? this.id,
			aliases.length && `(${aliases.join("/")})`,
			Object.keys(this.properties).length && this.toJSON("    ")
		].filter(Boolean).join(' ')
	}

	toJSON(space?: string){
		return JSON.stringify(this.properties, null, space)
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
	references = new Set<RelationEntity>();

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
	references = new Set<HyperRelationEntity>();
	
	constructor(
		public subjectCategory: CategoryEntity,
		public relation: Identifier.Relation,
		public objectCategory: CategoryEntity
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
	
	lookup(type: Function, id: Identifier.Entity): Entity | undefined {
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
		const [entity = new Entity(ids[0]), ...otherEntities] = existingLocalEntities
		for(const otherEntity of otherEntities){
			for(const reference of otherEntity.references)
				entity.references.add(reference)
			for(const alias of otherEntity.aliases)
				entity.aliases.add(alias)
			this.entities.delete(otherEntity)
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

	private declareProperty(property: Identifier.Property){

	}

	private evalExpression(thisEntity: Entity, expression: string): string {
		return expression
	}

	private declarePredicate(type: string, predicate: Raw.Predicate) {
		switch(type){
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
					subject.properties[property] = this.evalExpression(subject, expression)
				for(const subject of subjects)
					console.log(subject, subject.properties)
				return
			}
			case "categorization": {
				const categories = this.lookupCategoryEntities(predicate.categories)
				for(const subject of subjects)
					for(const category of categories)
						this.entities.add(new CategorizationEntity(
							subject,
							category
						))
				return
			}
			case "relation": {
				const objects = this.lookupNounEntities(predicate.objects)
				for(const subject of subjects)
					for(const object of objects)
						this.entities.add(new RelationEntity(
							subject,
							predicate.relation,
							object
						))
				return
			}
			case "hyperRelation": {
				const categories = this.lookupCategoryEntities(predicate.categories)
				for(const subjectCategory of subjects)
					for(const objectCategory of categories)
						this.entities.add(new HyperRelationEntity(
							subjectCategory,
							predicate.relation,
							objectCategory
						))
				return
			}
		}
	}

	private evalCompoundStatement(subjects: Entity[], predicates: {type: string, predicate: Raw.Predicate}[]) {
		for(const {type, predicate} of predicates)
			this.evalPredicate(subjects, type, predicate)
	}

	evalProgram(program: Raw.CompoundStatement[]) {
		// Declare all noun entities and category entities
		for(const compoundStatement of program){
			this.declareNounEntities(compoundStatement.subjects)
			this.declarePredicates(compoundStatement.predicates)
		}

		// Transform statements into relation, categorization, and rule entities
		for(const compoundStatement of program){
			const subjects = this.lookupNounEntities(compoundStatement.subjects)
			this.evalCompoundStatement(subjects, compoundStatement.predicates)

			// for(const entity of subjects)
			// 	console.log(entity)
			// console.log("nouns", this.nouns)
			// console.log("categories", this.categories)
		}

		// for(const entity of this.entities)
		// 	console.log(entity.toString())
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