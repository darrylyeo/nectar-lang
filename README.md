# Nectar: The Human-Friendly Knowledge Graph 🐝

[Try Nectar on Repl.it!](https://repl.it/@nectarlang/nectar-lang#README.md)

**Nectar** is a flexible, human-friendly language that helps you record and analyze knowledge straight from your stream of consciousness. Build a knowledge graph quickly and query it as you go, simply by writing regular English sentences!

Here are just a few things that are possible with Nectar:
* Keep an address book of people you've met and their connections
* Write a content management system schema for your websites and apps
* Record and maintain highly interconnected scientific datasets
* Augment your wiki by cataloging characters, places, objects, and events from your favorite franchise


## Quickstart Guide

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

@Nouns are also objects that can be assigned any number of **properties** (like a key-value store). Property values can take the form of **strings**, **numbers**, or **quantities** (numbers with units).

```
@Earth is a #planet with color "blue", rank 3
and an equatorial_radius of 6378.1 km.
```

This results in the following:

```
           Nouns: @Earth {
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

### Aliases

Nouns can be **aliased** to multiple "@" identifiers. Any of the identifiers can be used to reference the same noun object, regardless of where the alias is declared within the current scope.

```
@Earth (aka @Terra or the @Blue_Marble)
has an average_orbital_speed of 29.78 km/s
and a mass of 5.97237e24 kg.

@Mars/the @Red_Planet is a #planet.
The @Red_Planet
has color "red"
has rank 4
has an equatorial_radius of 3396.2 km
has an average_orbital_speed of 24.007 km/s
has a mass of 6.4171e23 kg.
```

```
           Nouns: @Earth/@Terra/@Blue_Marble {
                    "color": "blue",
                    "rank": 3,
                    "equatorial_radius": {
                      "number": 6378.1,
                      "unit": "km"
                    },
                    "average_orbital_speed": {
                      "number": 29.78,
                      "unit": "km/s"
                    },
                    "mass": {
                      "number": 5.97237e24,
                      "unit": "kg"
                    }
                  }
                  @Mars/@Red_Planet {
                    "color": "red",
                    "rank": 4,
                    "equatorial_radius": {
                      "number": 3396.2,
                      "unit": "km"
                    },
                    "average_orbital_speed": {
                      "number": 24.007,
                      "unit": "km/s"
                    },
                    "mass": {
                      "number": 6.4171e23,
                      "unit": "kg"
                    }
                  }

      Categories: #planet

 Categorizations: @Earth/@Terra/@Blue_Marble is #planet
                  @Mars/@Red_Planet is #planet
```

### Complex declarations

Nectar excels when many nouns and categories share the same **relations** and/or **categorizations**. You can generate a complex knowledge graph really quickly using aliases, compound subjects, compound predicates, and compound sentences!

```
@Pac-Man is a #male #Pac-Person, and @Ms_Pac-Man is a #female #Pac-Person.
@Pac-Man is married to @Ms_Pac-Man.
@Pac-Man and @Ms_Pac-Man are enemies with @Blinky (also known as @Shadow or @Akabei),
@Pinky/@Speedy/@Pinki, @Inky (otherwise referred to as @Bashful or @Aosuke),
and @Clyde (@Pokey, @Guzuta).
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

### Querying the knowledge graph (partially implemented)

Finished brain dumping? You can **query** your knowledge graph directly within your Nectar document. Queries begin with "is" or "does", contain "what" or "who", use a **comparator** (e.g. "<", "greater than"), and/or end with a question mark.

```
Is @Nectar #human-friendly?
@Deno is a #language?
@Nectar is written with what?

What is @Mars?
Does @Earth have an equatorial_radius greater than 6000 km?
Is the rank of @Earth lower than the rank of @Mars?
@Earth mass < @Mars mass?
@Earth?

Who is #female?
Who is married to who?
```

These queries output the following:

```
true
false
[@Rust, @Web_Assembly, @TypeScript, @Deno]

[#planet]
true
true
false
@Earth/@Terra/@Blue_Marble {
  "color": "blue",
  "rank": 3,
  "equatorial_radius": {
    "number": 6378.1,
    "unit": "km"
  },
  "average_orbital_speed": {
    "number": 29.78,
    "unit": "km/s"
  },
  "mass": {
    "number": 5.97237e24,
    "unit": "kg"
  }
}

[@Ms_Pac-Man, @Pinky/@Speedy/@Pinki]
[@Pac-Man is married to @Ms_Pac-Man]
```

Regardless of how the statements are ordered, the Nectar interpreter will always build the graph from the declarations first, then run the queries and output their results. As such, queries can appear before, after, or even between the declarations they pertain to.

### Scopes

A **scope** allows you to create a temporary, isolated fragment of the knowledge graph. Scopes are written as a label followed by curly braces. Any statements, rules or aliases declared within a scope will only apply to the queries inside.

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

```
"pink"
false
true
"blue"
true
false
```

Scopes can be useful for testing multiple scenarios on an existing body of knowledge, or allowing conflicting but similarly named terminology to co-exist in the same Nectar document.


### Dimensions and slices (not yet implemented)

You can define a **dimension** with one or more **slices**. The value or range associated with a given slice will be associated with all statements inside (by assigning properties or generating hyper-relations). Dimensions and slices can be discrete (like countries and timestamps) or continuous (like GPS coordinates or time intervals).

```
in #city [
    @Chicago [
        @Harry meets @Sally
    ]
    @New_York_City [
        @Harry marries @Sally
    ]
]
```

```
in year (number) [
    2000 to 2005 [
        @Brad_Pitt was married to @Jennifer_Aniston
    ]

    2000 to 2003 [
        @Angelina_Jolie was married to @Billy_Bob_Thornton
    ]

    2005 to 2020 [
        @Brad_Pitt is divorced from @Jennifer_Aniston
    ]
    
    2014 to 2020 [
        @Brad_Pitt is married to @Angelina_Jolie
    ]
    
    2019 to 2020 [
        @Brad_Pitt is legally separated from @Angelina_Jolie
    ]
]
```

```
in #Middle_Earth_landmark [
    @Mount_Doom [
        @Sauron forges the @One_Ring
    ]
    @Anduin [
        @Isildur loses the @One_Ring
        @Deagol finds the @One_Ring
        @Smeagol murders @Deagol
        @Smeagol steals the @One_Ring
    ]
    the @Misty_Mountains [
        @Bilbo finds the @One_Ring
    ]
    the @Shire [
        @Frodo obtains the @One_Ring
    ]
    @Mount_Doom [
        @Smeagol steals the @One_Ring
        @Smeagol/@Gollum destroys the @One_Ring
    ]
]
```

Now you can write queries like this:

```
Did @Harry meet @Sally in @New_York_City?
Was @Brad_Pitt married to @Angelina_Jolie in the year 2015?
Did @Bilbo find the @One_Ring in @Anduin?
```

```
false
true
false
```

As you can see, dimensions are useful for defining properties of many objects at once, as well as describing relations with hyper-relations to aid in automated reasoning.

### Schemas and rules (not yet implemented)

If you start noticing similarities within your data, you can formalize these patterns as **schemas** and **rules** by using the word "every" or "all". For all nouns that are members of the given categories, schemas define a set of property names and types, while rules define a relation between members.

```
Every #planet has a numerical rank, a color string, and a average_orbital_speed in km/s.
Every #Pac-Person is enemies with every #ghost.
```

Schemas are useful for generating schemas for other data representations like SQL and GraphQL. Rules are useful for performing automated reasoning, allowing you make new discoveries and mine data out of your existing data.

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

Nectar strives to be as frictionless as possible and liberate your data from arbitrary formats or structural limitations. With a hypergraph as its underlying, all-encompassing data structure, Nectar will be able to import from or output query results to CSV, JSON, YAML, SQL, GraphQL schemas, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, graph visualizations and much more in the future!

### Features to add

* Higher-order relations ("hyper-relations") that relate a relation to a categorization, or another relation: `@Harry met @Sally in @Chicago`, `The @chicken crossed the @road because the @chicken was #bored`
  * Dimensions and slices
* Named capture variables in place of "what" or "who" in queries: `$a is married to $b?`
* Automated reasoning/logical inferences based on rules: `If $a is married to $b, $b is married to $a.`
  * Intelligently apply contextual time ranges to statements based on past, present, and future tense: `Was $a married to $b?`
  * Negative statements: `The @princess is not in the @castle`
* Create custom interpreters that export/transpile data, rules, and schemas to other common data formats
  * CSV, JSON, YAML, SQL, GraphQL, MongoDB, ArangoDB, GUN.js, Neo4J, Grakn.AI, etc.
* Computed/reactive properties
  * Generate mappings to reactive bindings in JavaScript libraries like React, Vue, and Svelte
* Continue to make the grammar more comprehensive, robust, and closer to regular English
  * Pronouns like "it", "this", "they" to reference the subject of a previous statement
  * Add custom dictionaries of words and phrases for easier aliasing of plural nouns and verb conjugations

## The Making of Nectar

Nectar was invented and developed in August 2020 during Repl.it's Programming Language Jam.

The Nectar parser is created with [pest](https://pest.rs), a Rust library for creating PEGs (parsing expression grammars). Nectar's grammar is defined in `src/grammar.pest`. The `pest_consume` crate was used to convert the AST into Rust structs, and the `serde` crate was used to serialize the AST into JSON.

The parser is compiled to a WebAssembly binary targeting the TypeScript runtime Deno via Second State's [ssvmup](https://secondstate.io/ssvm); the Nectar interpreter and REPL are written in TypeScript.

### Why "Nectar"?

Nectar is fast as a hummingbird, resilient as a butterfly, and so fluid and rich in (syntactic) sugar that you can type it right from your stream of consciousness. Just as it's the definitive food source for bees, a foundational species within our biosphere, Nectar is the definitive source of all your data - the underlying con-nectar of everything within your tech ecosystem. How sweet is that?!

## Try Nectar on Repl.it!

[https://repl.it/@nectarlang/nectar-lang](https://repl.it/@nectarlang/nectar-lang#README.md)

---

### Credits

@darrylyeo, @deebee