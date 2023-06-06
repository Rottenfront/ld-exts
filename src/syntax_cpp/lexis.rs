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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use lady_deirdre::lexis::Token;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Token, Eq, PartialEq)]
pub enum CppToken {
    #[precedence(2)]
    #[rule("auto")]
    Auto,
    #[precedence(2)]
    #[rule("break")]
    Break,
    #[precedence(2)]
    #[rule("case")]
    Case,
    #[precedence(2)]
    #[rule("catch")]
    Catch,
    #[precedence(2)]
    #[rule("class")]
    Class,
    #[precedence(2)]
    #[rule("const")]
    Const,
    #[precedence(3)]
    #[rule("consteval")]
    ConstEval,
    #[precedence(3)]
    #[rule("constexpr")]
    ConstExpr,
    #[precedence(3)]
    #[rule("constinit")]
    ConstInit,
    #[precedence(2)]
    #[rule("continue")]
    Continue,
    #[precedence(2)]
    #[rule("decltype")]
    DeclType,
    #[precedence(2)]
    #[rule("default")]
    Default,
    #[precedence(2)]
    #[rule("delete")]
    Delete,
    #[precedence(2)]
    #[rule("do")]
    Do,
    #[precedence(3)]
    #[rule("char" | "double" | "float" | "int" | "void")]
    BasicType,
    #[precedence(2)]
    #[rule("else")]
    Else,
    #[precedence(2)]
    #[rule("enum")]
    Enum,
    #[precedence(2)]
    #[rule("explicit")]
    Explicit,
    #[precedence(2)]
    #[rule("export")]
    Export,
    #[precedence(2)]
    #[rule("extern")]
    Extern,
    #[precedence(2)]
    #[rule("final")]
    Final,
    #[precedence(2)]
    #[rule("for")]
    For,
    #[precedence(2)]
    #[rule("friend")]
    Friend,
    #[precedence(2)]
    #[rule("goto")]
    Goto,
    #[precedence(2)]
    #[rule("if")]
    If,
    #[precedence(2)]
    #[rule("inline")]
    Inline,
    #[precedence(2)]
    #[rule("namespace")]
    Namespace,
    #[precedence(2)]
    #[rule("new")]
    New,
    #[precedence(2)]
    #[rule("noexcept")]
    Noexpect,
    #[precedence(2)]
    #[rule("operator")]
    Operator,
    #[precedence(2)]
    #[rule("override")]
    Override,
    #[precedence(2)]
    #[rule("private")]
    Private,
    #[precedence(2)]
    #[rule("protected")]
    Protected,
    #[precedence(2)]
    #[rule("public")]
    Public,
    #[precedence(2)]
    #[rule("long" | "short" | "signed" | "unsigned")]
    TypeMod,
    #[precedence(2)]
    #[rule("register")]
    Register,
    #[precedence(2)]
    #[rule("requires")]
    Requires,
    #[precedence(2)]
    #[rule("return")]
    Return,
    #[precedence(2)]
    #[rule("static")]
    Static,
    #[precedence(2)]
    #[rule("struct")]
    Struct,
    #[precedence(2)]
    #[rule("switch")]
    Switch,
    #[precedence(2)]
    #[rule("template")]
    Template,
    #[precedence(2)]
    #[rule("throw")]
    Throw,
    #[precedence(2)]
    #[rule("try")]
    Try,
    #[precedence(2)]
    #[rule("typedef")]
    Typedef,
    #[precedence(2)]
    #[rule("typename")]
    Typename,
    #[precedence(2)]
    #[rule("union")]
    Union,
    #[precedence(2)]
    #[rule("using")]
    Using,
    #[precedence(2)]
    #[rule("virtual")]
    Virtual,
    #[precedence(2)]
    #[rule("volatile")]
    Volatile,
    #[precedence(2)]
    #[rule("while")]
    While,

    #[precedence(3)]
    #[rule("0b" & ['0', '1']+ & ('L' | 'U' | 'l' | 'u')*)]
    BinNumber,
    #[precedence(3)]
    #[rule("0o" & ['0'..'7']+ & ('L' | 'U' | 'l' | 'u')*)]
    OctNumber,
    #[rule(['0'..'9']+ & ('.' & ['0'..'9']+)?
    & ('L' | 'U' | 'l' | 'u' | 'f' | 'F' | 'd' | 'D')*)]
    DecNumber,
    #[precedence(3)]
    #[rule("0x" & ['0'..'9', 'A'..'F', 'a'..'f']+ & ('L' | 'U' | 'l' | 'u')*)]
    HexNumber,

    // WHITESPACES
    #[rule([' ', '\t', '\r', '\x0b', '\x0c', '\n']+)]
    Whitespace,

    // BRACKETS
    #[precedence(3)]
    #[rule('(')]
    Open,
    #[precedence(3)]
    #[rule(')')]
    Close,
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
    #[rule("::")]
    DColon,
    #[rule(';')]
    Semicolon,

    // CHARACTER & STRING
    #[rule('\'' & (^['\\', '\''] | ('\\' & ^['x'])
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '\''?)]
    Char,
    #[rule('"' & (^['\\', '"'] | ('\\' & ^['x'])
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '"'?)]
    String,

    // OPERATORS
    #[precedence(2)]
    #[rule('!' | "not" | "compl")]
    UnOp,
    #[rule(['-', '+'])]
    MultiOp,
    #[precedence(3)]
    #[rule(['%', '^', '|', '/'] | "bitand" | "bitor" | "xor" | "and" | "or"
    | "||" | "&&" | "==" | "!=" | ">=" | "<=")]
    BinOp,
    #[rule('<')]
    Less,
    #[rule('>')]
    Greater,
    #[rule('=')]
    Set,
    #[precedence(3)]
    #[rule("++" | "--")]
    IncDec,

    // OTHER
    #[rule('*')]
    Star,
    #[rule('&')]
    Amp,
    #[rule('\\')]
    Backslash,
    #[rule('?')]
    QuestMark,

    #[rule('#')]
    Hash,

    #[precedence(4)]
    #[rule((['+', '-', '/', '*', '^', '|', '%', '&'] & '=') | "<<=" | ">>=")]
    SetOp,

    // COMMENTS
    #[precedence(5)]
    #[rule(("//" & (^['\\', '\n'] | ("\\" & .?))* & '\n'?)
           | ("/*" & (^['*'] | ('*' & ^['/']?)) & "*/"?))]
    Comment,

    #[rule(^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
    '}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', '0'..'9', ' ',
    '\t', '\r', '\x0b', '\x0c', '\n']
    & ^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
    '}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', ' ', '\t', '\r',
    '\x0b', '\x0c', '\n']* /* ['A'..'Z', 'a'..'z', '_']+*/)]
    Ident,

    #[rule('~')]
    Tilde,
    #[rule('@')]
    At,

    #[rule(['`', '$'])]
    Other,

    #[mismatch]
    Mismatch,
}

