import { parse_to_json } from "../pkg/nectar_lib.js"
import Nectar from './types.ts'

class Entity {
	// Entities referenced in ancestor scopes
	references = new Set<Entity>();

	// aliases: Nectar.NounAliases,
	// parentAliases: Nectar.NounAliases
}

interface EntityTable {
	[key: string]: Entity
}

class Scope {
	parent?: Scope;
	
	// Nouns declared in this scope
	lookupTable: EntityTable = {};

	// Entities defined in this scope
	entities = new Set<Entity>();

	// Categories defined in this scope
	// categories: new Set<Category>();
	
	lookup(noun: Nectar.Noun): Entity | undefined {
		return this.lookupInternal(noun) ?? this.parent?.lookup(noun)
	}
	
	lookupInternal(noun: Nectar.Noun): Entity | undefined {
		return this.lookupTable[noun]
	}

	evalSubject(subject: Nectar.Subject): Entity {
		// Find existing local entities for these nouns
		const existingLocalEntities = subject
			.map((noun: Nectar.Noun) => this.lookupInternal(noun))
			.filter(Boolean)
		
		// Merge entities or create a new one
		const [entity = new Entity(), ...otherEntities] = existingLocalEntities
		for(const {references} of otherEntities)
			for(const reference of references)
				entity.references.add(reference)

		// Declare references to the entity
		for(const noun of subject)
			this.lookupTable[noun] = entity
		
		// If parent entities exist, reference them
		for(const noun of subject)
			if(this.parent){
				const parentEntity = this.parent.lookup(noun)
				if(parentEntity)
					entity.references.add(parentEntity)
			}

		// if(entity.references.size === 1)
		// 	return [...entity.references][0]
		// else {
			this.entities.add(entity)
			return entity
		// }
	}

	evalSubjects(subjects: Nectar.Subject[]): Set<Entity> {
		return new Set(
			subjects
				.map(subject => this.evalSubject(subject))
		)
	}

	evalCompoundStatement({subjects, predicates}: Nectar.CompoundStatement){
		const entities = this.evalSubjects(subjects)

		for(const predicate of predicates)
			switch(predicate.type){

			}

		// for(const entity of entities)
		// 	console.log(entity)
		// console.log(this.lookupTable)
	}
}

export class NectarInterpreter {
	scope = new Scope();

	evaluate(contents: string){
		try {
			const statements = JSON.parse(parse_to_json(contents))
			console.log("->", statements)

			for(const statement of statements)
				this.scope.evalCompoundStatement(statement)
		}catch(e){
			console.error(e)
		}
	}
}