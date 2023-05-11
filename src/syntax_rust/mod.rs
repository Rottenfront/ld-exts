// ///////////////////////////////////////////////////////////////////////////// //
// Copyright (c) 2023 Bankov Andrey "Rottenfront"                                //
//                                                                               //
// Permission is hereby granted, free of charge, to any person obtaining a copy  //
// of this software and associated documentation files (the "Software"), to deal //
// in the Software without restriction, including without limitation the rights  //
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell     //
// copies of the Software, and to permit persons to whom the Software is         //
// furnished to do so, subject to the following conditions:                      //
//                                                                               //
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR    //
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,      //
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE   //
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        //
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, //
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE //
// SOFTWARE.                                                                     //
// ///////////////////////////////////////////////////////////////////////////// //

// pub mod formatter;
pub mod lexis;
pub mod syntax;

use lady_deirdre::lexis::{CodeContent, SourceCode, ToSpan, TokenBuffer};
use lady_deirdre::syntax::Node;
use lady_deirdre::syntax::SyntaxTree;

use std::fs;

pub fn main() {
    let code = TokenBuffer::<lexis::RustToken>::from(
        fs::read_to_string("txt.rs").expect("Should have been able to read the file"),
    );

    let tree = syntax::RustNode::parse(code.cursor(..));

    println!(
        "{}",
        tree.errors()
            .map(|error| format!("{}: {}", error.span().format(&code), error))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    println!(
        "{}",
        code.chunks(..)
            .map(|chunk| chunk.token.to_string())
            .collect::<Vec<_>>()
            .join("|")
    )
}
