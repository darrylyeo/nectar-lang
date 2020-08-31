# Nectar: The Human-Friendly Knowledge Graph

Hey, Repl.it-eers!

I'm proud to announce Nectar, a flexible, human-friendly language that helps you encode any kind of knowledge straight from your stream of consciousness.

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
	has rank 3
	has an equatorial_radius of 6378.1 km
	has an average_orbital_speed of 29.78 km/s
	has a mass of 5.97237e24 kg.
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
Is the rank of @Earth higher than the rank of @Mars?
```

A scope {} allows you to deal with distinct fragments of graphs. Scopes can have any label, and any statements, rules or aliases declared within will only apply to the queries inside.

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

# Using Nectar

To run Nectar with a file e.g. hello.nectar, run the following command line:

```
deno run --allow-read --allow-env --allow-net --unstable src/hello.ts hello.nectar
```

# Using the REPL

To launch the Nectar REPL, run the following command line (or press "Run" on repl.it):

```
deno run --allow-read --allow-env --allow-net --unstable src/main.ts
```

Type a statement or query, then hit ENTER. The REPL will show the query result or the current state

# The Future of Nectar

Since hypergraphs are an all-encompassing data structure, Nectar will be able to output query results to CSV, JSON, YAML, SQL, GraphQL schemas, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, graph visualizations and much more in the future!

Features to add:
* Pronouns like "it", "this", "they" to reference subject of previous statements
* Modifier blocks that apply hyper-relations to a group of relations:
  * Timestamps and time ranges
  * 
* Automated reasoning/logical inferences
* Reactive properties
  * Bindings with JavaScript libraries like React, Vue, Svelte, RxJS
* Jupyter
* Make the grammar even more robust, comprehensive and on par with regular English
* Add custom dictionaries of words to use within the grammar
* Intelligently apply contextual time ranges to statements based on past, present, and future tense


Why "Nectar"?

Nectar is fast as a hummingbird, resilient as a butterfly, and so fluid and rich in (syntactic) sugar that you can type it right from your stream of consciousness. Just as it's the definitive food source for bees, a foundational species within our biosphere, Nectar is the definitive source of all your data - the underlying con-nectar of everything within your tech ecosystem. How sweet is that?!