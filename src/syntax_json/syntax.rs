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
    lexis::TokenRef,
    syntax::{Node, NodeRef, SyntaxError},
};
use std::vec::Vec;

use super::lexis::JsonToken;

#[derive(Node, Clone)]
#[token(JsonToken)]
#[error(SyntaxError)]
#[skip($Whitespace)]
#[define(ANY = Object | Array | True | False | String | Number | Null)]
pub enum JsonNode {
    #[root]
    #[rule(object: Object)]
    Root { object: NodeRef },

    #[rule($BraceOpen & (entries: Entry)*{$Comma} & $BraceClose)]
    #[synchronization]
    Object { entries: Vec<NodeRef> },

    #[rule(key: $String & $Colon & value: ANY)]
    Entry { key: TokenRef, value: NodeRef },

    #[rule($BracketOpen & (items: ANY)*{$Comma} & $BracketClose)]
    #[synchronization]
    Array { items: Vec<NodeRef> },

    #[rule(value: $String)]
    String { value: TokenRef },

    #[rule(value: $Number)]
    Number { value: TokenRef },

    #[rule($True)]
    True,

    #[rule($False)]
    False,

    #[rule($Null)]
    Null,
}
