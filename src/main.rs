// pub mod syntax_json;
pub mod syntax_rust;
// pub mod syntax_xml;

use lady_deirdre::lexis::{SourceCode, TokenBuffer, ToSpan};
use lady_deirdre::syntax::Node;
use lady_deirdre::syntax::SyntaxTree;

use std::fs;

fn main() {
    let code = TokenBuffer::<syntax_rust::lexis::RustToken>::from(fs::read_to_string("txt.rs")
        .expect("Should have been able to read the file"));
    let tree = syntax_rust::syntax::RustNode::parse(code.cursor(..));
    println!("{}", tree.errors()
            .map(|error| format!("{}: {}", error.span().format(&code), error))
            .collect::<Vec<_>>()
            .join("\n"))

}