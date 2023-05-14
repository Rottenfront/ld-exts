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

use core::fmt::Display;
use lady_deirdre::lexis::LexisSession;

#[derive(Clone, Copy, Debug)]
pub enum RustToken {
    As,
    Async,
    Await,
    Break,
    Const,
    Continue,
    Crate,
    Do,
    Dyn,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    LSelf,
    USelf,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Try,
    Type,
    Union,
    Unsafe,
    Use,
    Where,
    While,
    Yield,

    BasicType,

    Number,

    Whitespace, // ' ' '\t' '\x0b' '\x0c'
    NewLine,    // '\n' '\r'

    Open,  // (
    Close, // )

    Less,    // <
    Greater, // >

    BraceOpen,  // {
    BraceClose, // }

    BracketOpen,  // [
    BracketClose, // ]

    Comma, // ,
    Point, // .
    Range, // ..

    Colon,       // :
    DoubleColon, // ::

    Dollar,        // $
    Semicolon,     // ;
    BinOp,         // % ^ && || << >> / >= == <=
    Add,           // +
    Sub,           // -
    Set,           // =
    Refer,         // & *
    Tilda,         // ~
    At,            // @
    Backslash,     // \\
    Bang,          // !
    Quest,         // ?
    Hash,          // #
    HashBang,      // #!
    Arrow,         // ->
    MatchArrow,    // =>
    SetOp,         // += -= /= *= ^= |= %= &= <<= >>=
    Comment,       // /* ... */

    Escape, // '\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0'] | ('x' & HEX & HEX) | ("u{" & HEX & HEX & HEX & HEX & '}'))

    Ident,

    String, // todo: implement String in lexer
    Char,
    Lable,
}

macro_rules! advance {
    ($i:expr, $x:expr) => {
        for _ in 0..$x {
            $i.advance();
        }
    };
}