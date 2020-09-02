# Nectar: The Human-Friendly Knowledge Graph

**Nectar** is a flexible, human-friendly language that helps you record any kind of knowledge straight from your stream of consciousness.

Here are just a few things that are possible with Nectar:
* Keep an address book of people you've met and their connections
* Write a content management system schema for your websites and apps
* Record and manage highly interconnected scientific datasets
* Augment your wiki by cataloging characters, places, objects, and events from your favorite franchise


## How does it work?

### Declaring data

Build relationships between **@nouns** and **#categories** by writing regular English sentences:

```
@Nectar is a #human-friendly #language created for the @Repl_it_Language_Jam
and written with @Rust, @Web_Assembly, @TypeScript and @Deno.
```

This generates the following knowledge graph:

```
           Nouns: @Nectar
                  @Repl_it_Language_Jam
                  @Rust
                  @Web_Assembly
                  @TypeScript
                  @Deno

      Categories: #human-friendly
                  #language

 Categorizations: @Nectar is #human-friendly
                  @Nectar is #language

       Relations: @Nectar created for @Repl_it_Language_Jam
                  @Nectar written with @Rust
                  @Nectar written with @Web_Assembly
                  @Nectar written with @TypeScript
                  @Nectar written with @Deno
```

@Nouns are also objects that can be assigned any number of **properties** (like a key-value store). Property values can take the form of strings, numbers, and numbers with units.

```
@Earth is a #planet
	with name "Earth",
	color "blue"
	rank 3,
	and an equatorial_radius of 6378.1 km.
```

This results in the following data:

```
           Nouns: @Earth {
                    "name": "Earth",
                    "color": "blue",
                    "rank": 3,
                    "equatorial_radius": {
                      "number": 6378.1,
                      "unit": "km"
                    }
                  }

      Categories: #planet

 Categorizations: @Earth is #planet
```

### Aliases and complex declarations

Nouns can be **aliased** with multiple "@" identifiers. Any of the aliases can be used to reference the same noun object, regardless of where the alias is declared within a set of statements.

```
@Earth (aka @Terra or the @Blue_Marble)
	has an average_orbital_speed of 29.78 km/s
	and a mass of 5.97237e24 kg.

The #planet @Mars (also known as the @Red_Planet)
	has name "Mars"
	has color "red"
	has rank 4
	has an equatorial_radius of 3396.2 km
	has an average_orbital_speed of 24.007 km/s
	has a mass of 6.4171e23 kg.
```

Nectar excels when many nouns and categories share the same relations and/or categorizations. You can write a complex knowledge graph really quickly using aliases, compound subjects, compound predicates, and compound sentences!

```
@Pac-Man is a #male #Pac-Person, and @Ms_Pac-Man is a #female #Pac-Person.
@Pac-Man is married to @Ms_Pac-Man.
@Pac-Man and @Ms_Pac-Man are enemies with @Blinky (also known as @Shadow or @Akabei), @Pinky/@Speedy/@Pinki, @Inky (otherwise referred to as @Bashful or @Aosuke), and @Clyde (@Pokey, @Guzuta).
@Blinky, @Speedy, @Bashful, and @Guzuta are #ghosts (#ghost).
@Speedy is #female; @Shadow, @Aosuke, and @Pokey are #male.
```

```
           Nouns: @Pac-Man
                  @Ms_Pac-Man
                  @Blinky/@Shadow/@Akabei
                  @Pinky/@Speedy/@Pinki
                  @Inky/@Bashful/@Aosuke
                  @Clyde/@Pokey/@Guzuta

      Categories: #male
                  #Pac-Person
                  #female
                  #ghosts/#ghost

 Categorizations: @Pac-Man is #male
                  @Pac-Man is #Pac-Person
                  @Ms_Pac-Man is #female
                  @Ms_Pac-Man is #Pac-Person
                  @Blinky/@Shadow/@Akabei is #ghosts/#ghost
                  @Pinky/@Speedy/@Pinki is #ghosts/#ghost
                  @Inky/@Bashful/@Aosuke is #ghosts/#ghost
                  @Clyde/@Pokey/@Guzuta is #ghosts/#ghost
                  @Pinky/@Speedy/@Pinki is #female
                  @Blinky/@Shadow/@Akabei is #male
                  @Inky/@Bashful/@Aosuke is #male
                  @Clyde/@Pokey/@Guzuta is #male

       Relations: @Pac-Man married to @Ms_Pac-Man
                  @Pac-Man enemies with @Blinky/@Shadow/@Akabei
                  @Pac-Man enemies with @Pinky/@Speedy/@Pinki
                  @Pac-Man enemies with @Inky/@Bashful/@Aosuke
                  @Pac-Man enemies with @Clyde/@Pokey/@Guzuta
                  @Ms_Pac-Man enemies with @Blinky/@Shadow/@Akabei
                  @Ms_Pac-Man enemies with @Pinky/@Speedy/@Pinki
                  @Ms_Pac-Man enemies with @Inky/@Bashful/@Aosuke
                  @Ms_Pac-Man enemies with @Clyde/@Pokey/@Guzuta
```

### Schemas and rules (not yet implemented)

If you start noticing similarities within your data, you can formalize these patterns as **schemas** and **rules** by using the word "every". For all nouns that are members of the given categories, schemas define a set of property names and types, while rules define a relation between members.

```
Every #planet has a numerical rank, a color string, and a average_orbital_speed in km/s.
Every #Pac-Person is enemies with every #ghost.
```

Schemas are useful for generating schemas for other data representations like SQL and GraphQL. Rules are useful for performing automated reasoning, allowing you to mine data out of your existing data and make discoveries.

### Querying the knowledge graph (partially implemented)

Finished brain dumping? You can **query** your knowledge graph directly within your Nectar document. Queries begin with "is" or "does", contain "what" or "who", and/or end with a question mark.

```
Is @Nectar a #language?
@Nectar is #human-friendly?
@Nectar is invented by who?
What is @Earth?
Does @Earth have an equatorial_radius greater than 6000 km?
Is the rank of @Earth lower than the rank of @Mars?
@Earth mass < @Mars mass?
Who is #female?
Who is married to who?
```



The Nectar interpreter will return a list of results

A **`scope {}`** allows you to create a temporary graph fragment. Scopes are annotated with a label, and any statements, rules or aliases declared within it will only apply to the queries inside. This can be helpful for testing multiple scenarios on a common set of entities, especially if there are conflicts in terminology.

```
parallel_universe {
	@Earth has color "pink".

	Color of @Earth?
	Does @Earth have color "blue"?
	Does @Earth have color "pink"?
}

Color of @Earth?
Does @Earth have color "blue"?
Does @Earth have color "pink"?
```

(Not yet implemented.) You can define a **dimension** with **discrete** or **ranged** blocks, where the value of the block will be applied to all statements within. Possible dimensions include location, timestamps, and time intervals. This is useful for defining properties of many objects at once, as well as describing relations with hyper-relations to aid in automated reasoning.

```
Between year [
	2000 to 2005 [
		@Brad_Pitt is married to @Jennifer_Aniston
	]

	2000 to 2003 [
		@Angelina_Jolie is married to @Billy_Bob_Thornton
	]

	2005 to present [
		@Brad_Pitt is divorced from @Jennifer_Aniston
	]
	
	2014 to present [
		@Brad_Pitt is married to @Angelina_Jolie
	]
	
	2019 to present [
		@Brad_Pitt is legally separated from @Angelina_Jolie
	]
]

In year 2002 [
	Is @Brad_Pitt married to @Angelina_Jolie?
	Is @Brad_Pitt married to @Jennifer_Aniston?
]

In year present [
	Is @Brad_Pitt married to @Angelina_Jolie?
	Is @Brad_Pitt married to @Jennifer_Aniston?
]
```

## Using Nectar

To run Nectar with a file e.g. hello.nectar, run the following command line:

```
deno run --allow-read --allow-env --allow-net --unstable src/hello.ts hello.nectar
```

You can pass as many file arguments as you wish.

## Using the REPL

To launch the Nectar REPL, run the following command line (or press "Run" on repl.it):

```
deno run --allow-read --allow-env --allow-net --unstable src/main.ts
```

Type a statement or query, then hit ENTER. If the line is empty or there are no query results to be shown, the REPL will show you all the entities (nouns, categories, categorizations, relations, and hyper-relations) that have been defined thus far.

The REPL can also create and exit scopes dynamically. Type `scope_name {` to enter a new scope. Type `}` to exit the scope.

## The Future of Nectar

Nectar strives to liberate your data from arbitrary formats or structural limitations. With a hypergraph as its underlying, all-encompassing data structure, Nectar will be able to import from or output query results to CSV, JSON, YAML, SQL, GraphQL schemas, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, graph visualizations and much more in the future!

Features to add:
* Higher-order relations ("hyper-relations") that relate a relation to a categorization, or another relation: `@Harry met @Sally in @New_York_City`, `The @chicken crossed the @road because the @chicken was #bored`
  * Modifier blocks that apply hyper-relations to a group of relations:
    * Timestamps and time ranges
    * Locations
* Named capture variables in place of "what" or "who" in queries: "$a is married to $b?"
* Automated reasoning/logical inferences based on rules: "If $a is married to $b, $b is married to $a."
  * Intelligently apply contextual time ranges to statements based on past, present, and future tense: "A was a married to $b?"
  * Negative statements: "The @princess is not in the @castle"
* Create custom interpreters that export/transpile data, rules, and schemas to other common data formats
  * CSV, JSON, YAML, SQL, GraphQL, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, etc.
* Computed/reactive properties
  * Generate mappings to reactive bindings in JavaScript libraries like React, Vue, and Svelte
* Continue to make the grammar more comprehensive, robust, and closer to regular English
  * Pronouns like "it", "this", "they" to reference the subject of a previous statement
  * Add custom dictionaries of words and phrases for easier aliasing of plurals and verb conjugations

## The Making of Nectar

Nectar was created in August 2020 as part of the Repl.it Programming Language Jam.

The Nectar parser is created with [pest](https://pest.rs), a Rust library for creating PEGs (parsing expression grammars). Nectar's grammar is defined in `src/grammar.pest`. The `pest_consume` crate was used to convert the AST into Rust structs, and the `serde` crate was used to serialize the AST into JSON.

The parser is compiled to a WebAssembly binary targeting the TypeScript runtime Deno via Second State's [ssvmup](https://secondstate.io/ssvm); the Nectar interpreter and REPL are written in TypeScript.


## Why "Nectar"?

Nectar is fast as a hummingbird, resilient as a butterfly, and so fluid and rich in (syntactic) sugar that you can type it right from your stream of consciousness. Just as it's the definitive food source for bees, a foundational species within our biosphere, Nectar is the definitive source of all your data - the underlying con-nectar of everything within your tech ecosystem. How sweet is that?!