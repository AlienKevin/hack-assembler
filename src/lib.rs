/// This module parses Hack assembly and produces a vector of instructions as output on success,
/// or a detailed error message with error location on parse failure.
mod hack_parser;
/// This module converts the vector of instructions outputed by hack_parser to binary Hack code.
mod hack_emitter;

pub fn assemble(source: &str) -> Result<String, String>
{
  hack_parser::parse(source).map(
      |ast| hack_emitter::emit(ast)
  )
}
