# Nectar: The Human-Friendly Knowledge Graph

Hey, Repl.it-eers!

I'm proud to announce Nectar, a flexible, human-friendly language that helps you record any kind of knowledge straight from your stream of consciousness.

Here are just a few things that are possible with Nectar:
* Keep an address book of people you've met and their connections
* Write a content management system schema for your websites and apps
* Record and manage highly interconnected scientific datasets
* Augment your wiki by cataloging characters, places, objects, and events from your favorite franchise


## How does it work?

Build relationships between **@nouns** and **#categories** by writing regular English sentences:

```
@Nectar is a #human-friendly #language invented by @Darryl_Yeo and written with @Rust, @Web_Assembly, @TypeScript and @Deno.
```

This generates the following knowledge graph:

```
           Nouns: @Nectar
                  @Darryl_Yeo
                  @Rust
                  @Web_Assembly
                  @TypeScript
                  @Deno

      Categories: #human-friendly
                  #language

 Categorizations: @Nectar is #human-friendly
                  @Nectar is #language

       Relations: @Nectar is invented by @Darryl_Yeo
                  @Nectar written with @Rust
                  @Nectar written with @Web_Assembly
                  @Nectar written with @TypeScript
                  @Nectar written with @Deno
                  @Nectar invented by @Darryl_Yeo
                  @Nectar written with @Rust
                  @Nectar written with @Web_Assembly
                  @Nectar written with @TypeScript
                  @Nectar written with @Deno
```

@Nouns are also objects with **properties**:

```
@Earth is a #planet
	with name "Earth",
	color "blue"
	rank 3,
	and an equatorial_radius of 6378.1 km.
```

Nouns can be **aliased**. As long as a noun is aliased anywhere within a set of statements, any one of the aliases will refer to the same object.

```
@Earth (also known as @Terra or the @Blue_Marble)
	has an average_orbital_speed of 29.78 km/s
	and a mass of 5.97237e24 kg.

The #planet @Mars (aka the @Red_Planet)
	has name "Mars"
	has color "red"
	has rank 4
	has an equatorial_radius of 3396.2 km
	has an average_orbital_speed of 24.007 km/s
	has a mass of 6.4171e23 kg.
```

If several of your objects start looking the same, you can define a **rule** or **schema** (not yet implemented):

```
Every #planet has a numerical rank and a average_orbital_speed in km/s.
```

Finished brain dumping? You can **query** your knowledge graph within the same document.

```
Is @Nectar a #language?
Is @Nectar #human-friendly?
@Nectar is invented by $who?
What is @Earth?
Does @Earth have an equatorial_radius greater than 6000 km?
Is the rank of @Earth closer than the rank of @Mars?
```

A **`scope {}`** allows you to create and operate within a temporary graph fragment. Scopes are annotated with a label, and any statements, rules or aliases declared within it will only apply to the queries inside. This can be helpful for testing multiple scenarios on a common set of entities, especially if there are conflicts in terminology.

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

To make setting properties and hyper-relations easier, you can set up a **dimension** with **discrete** values or **ranges** of values. The value will be applied to all statements in the block.

```
a [
	
]
```

## Using Nectar

To run Nectar with a file e.g. hello.nectar, run the following command line:

```
deno run --allow-read --allow-env --allow-net --unstable src/hello.ts hello.nectar
```

## Using the REPL

To launch the Nectar REPL, run the following command line (or press "Run" on repl.it):

```
deno run --allow-read --allow-env --allow-net --unstable src/main.ts
```

Type a statement or query, then hit ENTER. If the line is empty or there are no query results to be shown, the REPL will show you all the entities (nouns, categories, categorizations, relations, and hyper-relations) that have been defined thus far.

The REPL can also create and exit scopes dynamically. Type `scope_name {` to enter a new scope. Type `}` to exit the scope.

## The Future of Nectar

Nectar strives to liberate your data from any one format or arbitrary structural limitation. With a hypergraph as its underlying, all-encompassing data structure, Nectar will be able to import from or output query results to CSV, JSON, YAML, SQL, GraphQL schemas, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, graph visualizations and much more in the future!

Features to add:
* Pronouns like "it", "this", "they" to reference subject of previous statements
* Modifier blocks that apply hyper-relations to a group of relations:
  * Timestamps and time ranges
  * Locations
* Automated reasoning/logical inferences
* Reactive properties
  * Bindings with JavaScript libraries like React, Vue, Svelte, RxJS
* Continue to make the grammar more comprehensive, robust, and closer to regular English
  * Add custom dictionaries of words to use within the grammar
* Intelligently apply contextual time ranges to statements based on past, present, and future tense

## The Making of Nectar

The Nectar parser is created with [pest](https://pest.rs), a Rust library for creating PEGs (parsing expression grammars). Nectar's grammar is defined in `src/grammar.pest`. I used the `pest_consume` crate to convert the AST into Rust structs.

Since this was my first time using Rust, I had a lot of trouble writing the interpreter (my attempt is found at `src/interpreter.ts`). I decided to start over using the more familiar TypeScript, and [attempted to port the pest grammar to `nearley.js`](https://repl.it/@nectarlang/nectar-lang-js). However, I ran into a lot of issues getting a `moo.js` lexer to work correctly.

I ultimately settled on a hybrid approach using WebAssembly. I created a Rust library that uses the `serde` crate to serialize the original Rust struct-AST into JSON. I then used Second State's [ssvmup](https://secondstate.io/ssvm) to compile the Rust library to a WebAssembly binary targeting the TypeScript runtime Deno. From there, I was able to write the Nectar interpreter and REPL comfortably in TypeScript. Whew!


## Why "Nectar"?

Nectar is fast as a hummingbird, resilient as a butterfly, and so fluid and rich in (syntactic) sugar that you can type it right from your stream of consciousness. Just as it's the definitive food source for bees, a foundational species within our biosphere, Nectar is the definitive source of all your data - the underlying con-nectar of everything within your tech ecosystem. How sweet is that?!