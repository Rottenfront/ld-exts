/*
 * Copyright (c) 2023 Bankov Andrey "Rottenfront"
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AN D NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
*/

pub mod lexis;
pub mod syntax;

use std::{fs, thread, println};

use lady_deirdre::{
    lexis::{SourceCode, ToSpan, TokenBuffer, CodeContent},
    syntax::{Node, TreeContent},
};

pub fn main() {
    // let handle = thread::Builder::new()
    //     .stack_size(1024 * 1024 * 1024 * 10)
    //     .spawn(|| {
            let code = TokenBuffer::<lexis::RustToken>::from(
                fs::read_to_string(
                    "txt.rs",
                )
                .expect("Should have been able to read the file"),
                // fs::read_to_string("src/syntax_rust/mod.rs")
                //     .expect("Should have been able to read the file"),
            );

            println!("lexis");

            let tree = syntax::RustNode::parse(code.cursor(..));
            println!("{}", code.chunks(..)
                     .map(|ch| ch.token.to_string())
                     .collect::<Vec<_>>()
                     .join("|"));
            println!("syntax");

            for error in tree.errors() {
                println!("{}: {}", error.span().format(&code), error);
            }
    //     })
    //     .unwrap();

    // handle.join().unwrap();
}
