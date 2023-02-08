///////////////////////////////////////////////////////////////////////////////////
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
///////////////////////////////////////////////////////////////////////////////////

use lady_deirdre::lexis::Token;

#[derive(Token, Debug, Clone)]
#[define(DEC = ['0'..'9'])]
#[define(HEX = DEC | ['A'..'F'])]
#[define(POSITIVE = ['1'..'9'] & DEC*)]
#[define(ESCAPE = '\\' & (
      ['"', '\\', '/', 'b', 'f', 'n', 'r', 't']
    | ('u' & HEX & HEX & HEX & HEX)
))]
#[define(LETTER = ^['@', '~', '$', '!', '#', '%', '^', '&', '*', '(', ')', '-',
'=', '+', '[', ']', '{', '}', '\\', '|', ';', ':', '\'', '"', ',', '<', '.', '>',
'/', '?', '\r', '\t', '\n', ' ', '\x0b', '\x0c'])]
pub enum XmlToken {
    #[rule(LETTER+)]
    Identifier,

    // BRACKETS
    #[rule('(')]
    ParenthesisOpen,

    #[rule(')')]
    ParenthesisClose,

    #[rule('<')]
    AngleBracketOpen,

    #[rule('>')]
    AngleBracketClose,

    #[rule('"' & (ESCAPE | ^['"', '\\'])* & '"')]
    String,

    #[rule('-'? & ('0' | POSITIVE) & ('.' & DEC+)? & (['e', 'E'] & ['-', '+']? & DEC+)?)]
    Number,

    #[rule([' ', '\t', '\n', '\x0c', '\r']+)]
    Whitespace,

    #[mismatch]
    Mismatch,
}
