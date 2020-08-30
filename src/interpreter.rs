use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::types::*;
use super::parser;

// Implement Hash for BTreeSet to make HashSet<Box<NectarEntity<'a>>> possible
// use std::hash::{Hash, Hasher, BuildHasher};
// impl<T, S> Hash for BTreeSet<T, S> where T: Eq + Hash, S: BuildHasher {
//     fn hash<H: Hasher>(&self, hasher: &mut H) {
//         self.map.hash(hasher);
// 	}
// }

pub type NectarNounAliases<'a> = HashSet<NectarNoun<'a>>;

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NectarEntity<'a> {
	// Entities referenced in ancestor scopes
	references: BTreeSet<&'a Box<NectarEntity<'a>>>,

	// aliases: NectarNounAliases<'a>,
	// parentAliases: NectarNounAliases<'a>
}
// impl NectarEntity {
// 	fn new() {
// 		NectarEntity { references: BTreeSet<Box<NectarEntity<'a>>>::new() }
// 	}
// 	fn addAlias(&self, alias: NectarNoun<'a>) {
// 		self.aliases.insert(alias)
// 	}
	
// 	fn mergeEntity(&self, entity: NectarEntity) -> self {
// 		self.aliases.insert(alias)
// 	}
// }

// pub type NectarCategoryAliases<'a> = BTreeSet<NectarCategory<'a>>;
// pub struct NectarCategorization<'a> {
// 	aliases: NectarCategoryAliases<'a>,
// 	parentAliases: NectarCategoryAliases<'a>
// }

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NectarScope<'a> {
	parent: Option<&'a Box<NectarScope<'a>>>,
	
	// Noun -> Entity
	lookupTable: BTreeMap<NectarNoun<'a>, &'a Box<NectarEntity<'a>>>,

	// Nouns declared in this scope
	// nouns: BTreeSet<NectarNoun<'a>>,

	// Entities defined in this scope
	entities: BTreeSet<Box<NectarEntity<'a>>>,

	// categories: HashMap<NectarCategory<'a>, NectarCategorization<'a>>,
}
impl<'a> NectarScope<'a> {
	fn lookup(&self, noun: &NectarNoun<'a>) -> Option<&&Box<NectarEntity<'a>>> {
		match self.lookupInternal(noun) {
			Some(entity) =>
				Some(entity),
			None => match &self.parent {
				Some(parent) =>
					parent.lookup(noun),
				None => None
			}
		}
	}
	
	fn lookupInternal(&self, noun: &NectarNoun<'a>) -> Option<&&Box<NectarEntity<'a>>> {
		self.lookupTable.get(noun)
	}

	fn evalSubject(&self, subject: &NectarSubject<'a>) -> Box<NectarEntity<'a>> {
		// let aliases: NectarCategoryAliases = subject
		// 	.filter(|noun| entities.contains_key(noun))
		// 	.flatMap(|noun| entities.get(noun))
		// 	.collect()
		// match subject.find(|noun| entities.contains_key(noun)){
		// 	Some(noun) =>
		// }
		// entities.entry(noun).or_insert(100);

		// let aliases: NectarCategoryAliases = subject
		// 	.filter(|noun| entities.contains_key(noun));

		// for noun in subject {
		// 	self.lookup(noun)
		// }
		// let aliases: NectarCategoryAliases = subject
		// 	.map(|noun| self.lookup(noun))


		let entity: Box<NectarEntity<'a>> = Box::new(Default::default());
		self.entities.insert(entity);

		// Find existing local entities for these nouns
		let entities: HashSet<&Box<NectarEntity<'a>>> = subject.iter()
			.filter_map(|noun| self.lookupTable.get(noun))
			.map(|entityRef| *entityRef)
			.collect();
		for noun in subject {
			
		}

		match &self.parent {
			Some(parent) => {
				for noun in subject {
					match (*parent).lookup(noun) {
						Some(parentEntity) => {
							entity.references.insert(*parentEntity);
						},
						None => {
							&self.lookupTable.insert(noun, &entity);
						}
					}
				}
			},
			None => {
				for noun in subject {
					&self.lookupTable.insert(noun, &entity);
				}
			}
		}

		// match entity.references.len() {
		// 	1 => entity.references.iter().next(),
		// 	_ => {
				// entityRef
				entity
		// 	}
		// }
	}

	fn evalSubjects(&self, subjects: Vec<NectarSubject<'a>>) -> BTreeSet<Box<NectarEntity<'a>>> {
		subjects.into_iter()
			.map(|subject| self.evalSubject(&subject))
			.collect()
	}

	fn evalCompoundStatement(&self, compoundStatement: NectarCompoundStatement<'a>){
		let entities = &self.evalSubjects(compoundStatement.subjects);
		// for predicate in compoundStatement.predicates {
		// 	match predicate {
		// 		IsA { categorizations } => NectarStatement,
		// 		HasProperty { property, expression } => ,
		// 		Relation { relation, object } => ,
		// 		HyperRelation { relation, categorizations } => 
		// 	}
		// }
	}
}


pub fn eval<'a>(contents: &str) {
	match parser::parse(&contents) {
		Ok(parsed) => {
			let globalScope: NectarScope = Default::default();

			for compoundStatement in parsed {
				println!("-> {:?}", compoundStatement);
				// println!("-> {}", json::stringify(compoundStatement));
				// print_type_of(&compoundStatement);

				globalScope.evalCompoundStatement(compoundStatement);
			}
		},
		Err(error) => {
			println!("{}", error);
		}
	}
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}