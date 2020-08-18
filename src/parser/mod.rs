use pest_consume::{Error, Parser};


type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;


#[derive(Parser)]
#[grammar = "parser/nectar-parser.pest"]
pub struct NectarParser;

#[pest_consume::parser]
impl NectarParser {
	fn alpha(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn digit(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn identifier(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn string_content(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn string(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn integer(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn decimal(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn number(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn unit(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn quantity(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn noun(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn category(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn property(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn expression(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn noun_disjunction(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn noun_conjunction(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn noun_phrase(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn category_disjunction(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn category_conjunction(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn category_phrase(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn subject(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn relation(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn is_a_predicate(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn has_property_predicate(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn relation_predicate(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn hyper_relation_predicate(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}

	fn statement(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
	fn statements(input: Node) -> Result<&str> {
		Ok(input.as_str())
	}
}

pub fn parse(input_str: &str) -> Result<&str> {
	// Parse the input into `Nodes`
	let inputs = NectarParser::parse(Rule::statements, input_str)?;
	// There should be a single root node in the parsed tree
	let input = inputs.single()?;
	// Consume the `Node` recursively into the final value
	NectarParser::statements(input)
}