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

use std::fmt::{Display, Formatter};

use lady_deirdre::{
    lexis::{SourceCode, TokenRef},
    syntax::{Node, NodeRef, SyntaxTree, TreeContent},
};

use super::{lexis::JsonToken, syntax::JsonNode};

pub trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl<L: SourceCode<Token = JsonToken>> ToJsonString for L {
    fn to_json_string(&self) -> String {
        let syntax = JsonNode::parse(self.cursor(..));

        let formatter = JsonFormatter {
            lexis: self,
            syntax: &syntax,
        };

        formatter.to_string()
    }
}

pub struct JsonFormatter<'a, L: SourceCode<Token = JsonToken>, S: SyntaxTree<Node = JsonNode>> {
    pub lexis: &'a L,
    pub syntax: &'a S,
}

impl<'a, L, S> Display for JsonFormatter<'a, L, S>
where
    L: SourceCode<Token = JsonToken>,
    S: SyntaxTree<Node = JsonNode>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.format_node(&self.syntax.root_node_ref()))
    }
}

impl<'a, L, S> JsonFormatter<'a, L, S>
where
    L: SourceCode<Token = JsonToken>,
    S: SyntaxTree<Node = JsonNode>,
{
    fn format_node(&self, node_ref: &NodeRef) -> String {
        let node: &JsonNode = match node_ref.deref(self.syntax) {
            None => return String::from("?"),
            Some(node) => node,
        };

        match node {
            JsonNode::Root { object } => self.format_node(object),

            JsonNode::Object { entries } => {
                format!(
                    "{{{}}}",
                    entries
                        .into_iter()
                        .map(|entry| self.format_node(entry))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }

            JsonNode::Entry { key, value } => {
                format!("{:#}: {}", self.format_token(key), self.format_node(value),)
            }

            JsonNode::Array { items } => {
                format!(
                    "[{}]",
                    items
                        .into_iter()
                        .map(|item| self.format_node(item))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }

            JsonNode::String { value } | JsonNode::Number { value } => self.format_token(value),

            JsonNode::True => String::from("true"),

            JsonNode::False => String::from("false"),

            JsonNode::Null => String::from("null"),
        }
    }

    fn format_token(&self, token_ref: &TokenRef) -> String {
        token_ref.string(self.lexis).unwrap_or("?").to_string()
    }
}