pub mod syntax_c;
// pub mod syntax_json;
// pub mod syntax_rust;
// pub mod syntax_xml;

use lady_deirdre::lexis::{CodeContent, SourceCode, TokenBuffer, ToSpan};
use lady_deirdre::syntax::Node;
use lady_deirdre::syntax::SyntaxTree;

use std::fs;

fn main() {
    let code = TokenBuffer::<syntax_c::lexis::CToken>::from(
        fs::read_to_string("txt.c").expect("Should have been able to read the file"),
    );

    let tree = syntax_c::syntax::CNode::parse(code.cursor(..));

    println!(
        "{}",
        tree.errors()
            .map(|error| format!("{}: {}", error.span().format(&code), error))
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("{}",
             code
                 .chunks(..)
                 .map(|chunk| chunk.token.to_string())
                 .collect::<Vec<_>>()
                 .join("|"))
}
