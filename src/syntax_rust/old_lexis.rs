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
#[define(IDENT = (^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
'}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', '0'..'9', ' ',
'\t', '\r', '\x0b', '\x0c', '\n']
& ^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
'}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', ' ', '\t', '\r',
'\x0b', '\x0c', '\n']*))]
#[define(HEX = ['0'..'9', 'A'..'F', 'a'..'f'])]
pub enum RustToken {
    #[precedence(2)]
    #[rule("as")]
    As,
    #[precedence(3)]
    #[rule("async")]
    Async,
    #[precedence(2)]
    #[rule("await")]
    Await,
    #[precedence(2)]
    #[rule("break")]
    Break,
    #[precedence(2)]
    #[rule("const")]
    Const,
    #[precedence(2)]
    #[rule("continue")]
    Continue,
    #[precedence(2)]
    #[rule("crate")]
    Crate,
    #[precedence(2)]
    #[rule("do")]
    Do,
    #[precedence(2)]
    #[rule("dyn")]
    Dyn,
    #[precedence(2)]
    #[rule("else")]
    Else,
    #[precedence(2)]
    #[rule("enum")]
    Enum,
    #[precedence(2)]
    #[rule("extern")]
    Extern,
    #[precedence(2)]
    #[rule("fn")]
    Fn,
    #[precedence(2)]
    #[rule("for")]
    For,
    #[precedence(2)]
    #[rule("if")]
    If,
    #[precedence(2)]
    #[rule("impl")]
    Impl,
    #[precedence(2)]
    #[rule("in")]
    In,
    #[precedence(2)]
    #[rule("let")]
    Let,
    #[precedence(2)]
    #[rule("loop")]
    Loop,
    #[precedence(2)]
    #[rule("macro")]
    Macro,
    #[precedence(2)]
    #[rule("match")]
    Match,
    #[precedence(2)]
    #[rule("mod")]
    Mod,
    #[precedence(2)]
    #[rule("move")]
    Move,
    #[precedence(2)]
    #[rule("mut")]
    Mut,
    #[precedence(2)]
    #[rule("pub")]
    Pub,
    #[precedence(2)]
    #[rule("ref")]
    Ref,
    #[precedence(2)]
    #[rule("return")]
    Return,
    #[precedence(2)]
    #[rule("self")]
    LSelf,
    #[precedence(2)]
    #[rule("Self")]
    USelf,
    #[precedence(2)]
    #[rule("static")]
    Static,
    #[precedence(2)]
    #[rule("struct")]
    Struct,
    #[precedence(2)]
    #[rule("super")]
    Super,
    #[precedence(2)]
    #[rule("trait")]
    Trait,
    #[precedence(2)]
    #[rule("try")]
    Try,
    #[precedence(2)]
    #[rule("type")]
    Type,
    #[precedence(2)]
    #[rule("union")]
    Union,
    #[precedence(2)]
    #[rule("unsafe")]
    Unsafe,
    #[precedence(2)]
    #[rule("use")]
    Use,
    #[precedence(2)]
    #[rule("where")]
    Where,
    #[precedence(2)]
    #[rule("while")]
    While,
    #[precedence(2)]
    #[rule("yield")]
    Yield,

    #[precedence(2)]
    #[rule("u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8"
    | "i16" | "i32" | "i64" | "i128" | "isize" | "f32" | "f64")]
    BasicType,

    #[precedence(3)]
    #[rule("0b" & ['0', '1']+ & IDENT?)]
    BinNumber,
    #[precedence(3)]
    #[rule("0o" & ['0'..'7']+ & IDENT?)]
    OctNumber,
    #[rule(['0'..'9']+ & ('.' & ['0'..'9']+)?
    & IDENT?)]
    DecNumber,
    #[precedence(3)]
    #[rule("0x" & HEX+ & IDENT?)]
    HexNumber,

    #[rule([' ', '\t', '\x0b', '\x0c']+)]
    Whitespace,
    #[rule(['\n', '\r']+)]
    NewLine,

    #[rule('(')]
    Open,
    #[rule(')')]
    Close,

    #[rule('<')]
    Less,
    #[rule('>')]
    Greater,

    #[rule('{')]
    BraceOpen,
    #[rule('}')]
    BraceClose,

    #[rule('[')]
    BracketOpen,
    #[rule(']')]
    BracketClose,

    #[rule(',')]
    Comma,
    #[rule('|')]
    Pipe,

    #[rule('.')]
    Point,
    #[rule("..")]
    Range,

    #[rule('\'')]
    Apostr,
    #[precedence(2)]
    #[rule("b'")]
    AsciiChar,

    #[rule(':')]
    Colon,
    #[precedence(2)]
    #[rule("::")]
    DoubleColon,

    #[rule('$')]
    Dollar,
    #[rule(';')]
    Semicolon,
    #[precedence(2)]
    #[rule('%' | '^' | "<<" | ">>")]
    Op, 
    #[precedence(2)]
    #[rule(['>', '<', '='] & '=')]
    BoolOp,
    #[rule('+')]
    Add,
    #[rule('-')]
    Sub,
    #[rule('=')]
    Assign,
    #[rule('&')]
    Amp,
    #[rule('*')]
    Star,
    #[rule('/')]
    Slash,
    #[rule('~')]
    Tilda,
    #[rule('@')]
    At,
    #[rule('\\')]
    Backslash,
    #[rule('!')]
    Bang,
    #[rule('?')]
    Quest,
    #[rule("#")]
    Hash,
    #[precedence(2)]
    #[rule("#!")]
    HashBang,
    #[precedence(2)]
    #[rule("->")]
    Arrow,
    #[precedence(3)]
    #[rule((['+', '-', '/', '*', '^', '|', '%', '&'] & '=') | "<<=" | ">>=")]
    AssignWithOperation,
    #[precedence(2)]
    #[rule("//")]
    SingleComment,
    #[precedence(2)]
    #[rule("/*")]
    MultilineCommentOpen,
    #[precedence(2)]
    #[rule("*/")]
    MultilineCommentClose,

    #[precedence(2)]
    #[rule('\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0']
    | ('x' & HEX & HEX) | ("u{" & HEX & HEX & HEX & HEX & '}')))]
    Escape,
    
    // WHITESPACES
    #[rule([' ', '\t', '\r', '\x0b', '\x0c']+)]
    Whitespace,
    #[rule('\n'+)]
    NewLine,




    // BRACKETS
    #[precedence(3)]
    #[rule('(')]
    ParenthesisOpen,
    #[precedence(3)]
    #[rule(')')]
    ParenthesisClose,
    #[precedence(3)]
    #[rule('{')]
    BraceOpen,
    #[rule('}')]
    BraceClose,
    #[rule('[')]
    BracketOpen,
    #[rule(']')]
    BracketClose,

    // DELIMETERS
    #[rule(',')]
    Comma,
    #[rule('.')]
    Point,
    #[rule(':')]
    Colon,
    #[rule(';')]
    Semicolon,

    #[rule('"' & (^['\\', '"'] | ('\\' & ^['x'])
    | ("\\x" & HEX & HEX))* & '"'?)]
    String,

    // OPERATORS
    #[precedence(2)]
    #[rule(['-', '+', '&', '!'] | "<<" | ">>")]
    UnOp,
    #[rule(['%', '^', '|', '/'])]
    BinOp,
    #[precedence(3)]
    #[rule(['>', '<', '='] & '=')]
    BoolOp1,
    #[rule(['>', '<'])]
    BoolOp2,
    #[rule('=')]
    Set,
    #[precedence(3)]
    #[rule("++" | "--")]
    IncDec,

    // OTHER
    #[rule('*')]
    Star,
    #[rule('\\')]
    Backslash,
    #[rule('?')]
    QuestMark,

    #[rule('#')]
    Hash,

    #[precedence(3)]
    #[rule((['+', '-', '/', '*', '^', '|', '%', '&'] & '=') | "<<=" | ">>=")]
    SetOp,

    // COMMENTS
    #[precedence(5)]
    #[rule("//")]
    SingleComment,
    #[precedence(5)]
    #[rule("/*")]
    MultilineCommentOpen,
    #[precedence(5)]
    #[rule("*/")]
    MultilineCommentClose,

    #[rule(IDENT)]
    Ident,

    #[rule(['`', '@', '$', '~'])]
    Other,

    #[mismatch]
    Mismatch,
}

impl Display for RustToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::As => "As",
            Self::Async => "Async",
            Self::Await => "Await",
            Self::Break => "Break",
            Self::Const => "Const",
            Self::Continue => "Continue",
            Self::Crate => "Crate",
            Self::Do => "Do",
            Self::Dyn => "Dyn",
            Self::Else => "Else",
            Self::Enum => "Enum",
            Self::Extern => "Extern",
            Self::False => "False",
            Self::Fn => "Fn",
            Self::For => "For",
            Self::If => "If",
            Self::Impl => "Impl",
            Self::In => "In",
            Self::Let => "Let",
            Self::Loop => "Loop",
            Self::Macro => "Macro",
            Self::Match => "Match",
            Self::Mod => "Mod",
            Self::Move => "Move",
            Self::Mut => "Mut",
            Self::Pub => "Pub",
            Self::Ref => "Ref",
            Self::Return => "Return",
            Self::LSelf => "LSelf",
            Self::USelf => "USelf",
            Self::Static => "Static",
            Self::Struct => "Struct",
            Self::Super => "Super",
            Self::Trait => "Trait",
            Self::True => "True",
            Self::Try => "Try",
            Self::Type => "Type",
            Self::Union => "Union",
            Self::Unsafe => "Unsafe",
            Self::Use => "Use",
            Self::Where => "Where",
            Self::While => "While",
            Self::Yield => "Yield",
            Self::NumType => "NumType",
            Self::BasicType => "BasicType",
            Self::BinNumber => "BinNumber",
            Self::OctNumber => "OctNumber",
            Self::DecNumber => "DecNumber",
            Self::HexNumber => "HexNumber",
            Self::String => "String",
            Self::Identifier => "Identifier",
            Self::ParenthesisOpen => "ParenthesisOpen",
            Self::ParenthesisClose => "ParenthesisClose",
            Self::AngleBracketOpen => "AngleBracketOpen",
            Self::AngleBracketClose => "AngleBracketClose",
            Self::BraceOpen => "BraceOpen",
            Self::BraceClose => "BraceClose",
            Self::BracketOpen => "BracketOpen",
            Self::BracketClose => "BracketClose",
            Self::Underscore => "Underscore",
            Self::Comma => "Comma",
            Self::Point => "Point",
            Self::Pipe => "Pipe",
            Self::BoolOperator => "BoolOperator",
            Self::Range => "Range",
            Self::Apostrophe => "Apostrophe",
            Self::AsciiChar => "AsciiChar",
            Self::DoubleColon => "DoubleColon",
            Self::Colon => "Colon",
            Self::Dollar => "Dollar",
            Self::Semicolon => "Semicolon",
            Self::Operator => "Operator",
            Self::Add => "Add",
            Self::Assign => "Assign",
            Self::Amp => "Amp",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Tilda => "Tilda",
            Self::At => "At",
            Self::Backslash => "Backslash",
            Self::Escape => "Escape",
            Self::Bang => "Bang",
            Self::QuestMark => "QuestMark",
            Self::Hash => "Hash",
            Self::HashBang => "HashBang",
            Self::Arrow => "Arrow",
            Self::AssignWithOperation => "AssignWithOperation",
            Self::Whitespace => "Whitespace",
            Self::NewLine => "NewLine",
            Self::SingleComment => "SingleComment",
            Self::MultilineCommentOpen => "MultilineCommentOpen",
            Self::MultilineCommentClose => "MultilineCommentClose",
        }
        .fmt(f)
    }
}
// ("r\"" & (ESCAPE | ^['"', '\\'])* & '\"')
// ("r#\"" & (ESCAPE | ^['"', '\\'])* & "\"#")
// ("r###\"" & (ESCAPE | ^['"', '\\'])* & "\"###")