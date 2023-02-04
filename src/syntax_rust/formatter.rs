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

use lady_deirdre::{
    lexis::SourceCode,
    syntax::{ParseContext, TransduceRef, Transducer},
};

use super::{lexis::RustToken, syntax::RustNode};
pub struct RustFormatter;

impl<S: SourceCode<Token = RustToken>> Transducer<RustNode, S, String> for RustFormatter {
    fn map(&mut self, context: &mut ParseContext<RustNode, S, String>) -> String {
        match context.node() {
            RustNode::Root { object } => object
                .get(context)
                .map(|string| string.as_str())
                .unwrap_or("?")
                .to_string(),

            RustNode::Object { entries } => {
                format!(
                    "{{{}}}",
                    entries
                        .into_iter()
                        .map(|node_ref| node_ref
                            .get(context)
                            .map(|string| string.as_str())
                            .unwrap_or("?")
                            .to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }

            RustNode::Entry { key, value } => {
                format!(
                    "{:#}: {}",
                    key.string(context).unwrap_or("?"),
                    value
                        .get(context)
                        .map(|string| string.as_str())
                        .unwrap_or("?"),
                )
            }

            RustNode::Array { items } => {
                format!(
                    "[{}]",
                    items
                        .into_iter()
                        .map(|node_ref| node_ref
                            .get(context)
                            .map(|string| string.as_str())
                            .unwrap_or("?")
                            .to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }

            RustNode::String { value } | RustNode::Number { value } => {
                value.string(context).unwrap_or("?").to_string()
            }

            RustNode::True => String::from("true"),

            RustNode::False => String::from("false"),

            RustNode::Null => String::from("null"),
        }
    }
}
