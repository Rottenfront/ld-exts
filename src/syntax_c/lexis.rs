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
pub enum CToken {
    EOI = 0,
    Mismatch = 1,

    #[priority(3)]
    #[rule("alignas" | "_Alignas")]
    AlignAs, // TODO: implement alignas (IDK what is it)
    #[priority(3)]
    #[rule("alignof" | "_Alignof")]
    AlignOf,
    #[priority(3)]
    #[rule("auto")]
    Auto, // TODO: implement auto (the definition modifier)
    #[priority(3)]
    #[rule("break")]
    Break, // TODO: implement break (action `break;`)
    #[priority(3)]
    #[rule("case")]
    Case, // TODO: implement case (action `case VALUE:`)
    #[priority(3)]
    #[rule("const")]
    Const, // TODO: implement const (the definition modifier)
    #[priority(3)]
    #[rule("constexpr")]
    ConstExpr, // TODO: implement constexpr (IDK what is it)
    #[priority(3)]
    #[rule("continue")]
    Continue, // TODO: implement continue (action `continue;`)
    #[priority(3)]
    #[rule("default")]
    Default, // TODO: implement default (action `default:` or default case in _Generic construction)
    #[priority(3)]
    #[rule("do")]
    Do, // TODO: implement do (action `do ACTION while (CONDITION);`)
    #[priority(3)]
    #[rule("else")]
    Else, // TODO: implement else (action `if ACTION else ACTION`)
    #[priority(3)]
    #[rule("enum")]
    Enum, // TODO: implement enum (type definition)
    #[priority(3)]
    #[rule("extern")]
    Extern, // TODO: implement extern (root item modifier)
    #[priority(3)]
    #[rule("for")]
    For, // TODO: implement for (action `for (INIT; CONDITION; STEP) ACTION`)
    #[priority(3)]
    #[rule("goto")]
    Goto, // TODO: implement goto (action `goto LABEL;`)
    #[priority(3)]
    #[rule("if")]
    If, // TODO: implement if (action `if ACTION else ACTION`)
    #[priority(3)]
    #[rule("inline")]
    Inline, // TODO: implement inline (definition modifier)
    #[priority(3)]
    #[rule("register")]
    Register, // TODO: implement register (definition modifier)
    #[priority(3)]
    #[rule("restrict")]
    Restrict, // TODO: implement restrict (IDK what is it)
    #[priority(3)]
    #[rule("return")]
    Return, // TODO: implement return (action `return VALUE?;`)
    #[priority(3)]
    #[rule("sizeof")]
    SizeOf,
    #[priority(3)]
    #[rule("static")]
    Static, // TODO: implement static (definition modifier)
    #[priority(3)]
    #[rule("static_assert" | "_Static_assert")]
    StaticAssert, // TODO: implement static_assert (IDK what is it)
    #[priority(3)]
    #[rule("struct")]
    Struct, // TODO: implement struct (type definition)
    #[priority(3)]
    #[rule("switch")]
    Switch, // TODO: implement switch (action `switch VALUE CODE_BLOCK`)
    #[priority(3)]
    #[rule("thread_local" | "_Thread_local")]
    ThreadLocal, // TODO: implement thread_local (IDK what is it)
    #[priority(3)]
    #[rule("typedef")]
    Typedef, // TODO: implement typedef (type definition)
    #[priority(3)]
    #[rule("typeof")]
    TypeOf,
    #[priority(3)]
    #[rule("typeof_unqual")]
    TypeofUnqual, // TODO: implement typeof_unqual (IDK what is it)
    #[priority(3)]
    #[rule("union")]
    Union, // TODO: implement union (type definition)
    #[priority(3)]
    #[rule("volatile")]
    Volatile, // TODO: implement volatile (definition modifier)
    #[priority(3)]
    #[rule("while")]
    While, // TODO: implement while (action `while (CONDITION) ACTION`)

    #[priority(3)]
    #[rule("true" | "false" | "nullptr")]
    ValueConst,

    #[priority(3)]
    #[rule(
        "bool"
        | "_Bool"
        | "void"
        | "char"
        | "double"
        | "float"
        | "int"
        | "_Noreturn"
        | "_Decimal128"
        | "_Decimal32"
        | "_Decimal64"
        | "_BitInt" // What is this?
    )]
    BasicType,
    #[priority(3)]
    #[rule(
        "long"
        | "short"
        | "signed"
        | "unsigned"
        | "_Complex"
        | "_Imaginary"
        | "__complex__" // GNU C Extension
    )]
    BasicTypeMod,

    #[priority(3)]
    #[rule("_Generic")] // very strange thing
    KwGeneric, // TODO: implement _Generic (ISO C2x pg 72)

    // GNU C Extensions
    // #[priority(3)]
    // #[rule("__FUNCTION__")]
    // GnuFunction,
    // #[priority(3)]
    // #[rule("__PRETTY_FUNCTION__")]
    // GnuPrettyFunction,
    // #[priority(3)]
    // #[rule("__alignof" "__"?)]
    // GnuAlignOf,
    // #[priority(3)]
    // #[rule("__asm" "__"?)]
    // GnuAsm,
    #[priority(3)]
    #[rule("__attribute" "__"?)]
    GnuAttribute,
    // #[priority(3)]
    // #[rule("__builtin_offsetof")]
    // GnuBuiltInOffsetOf,
    // #[priority(3)]
    // #[rule("__builtin_va_arg")]
    // GnuBuiltInVaArg,
    // #[priority(3)]
    // #[rule("__complex__")]
    // GnuComplex,
    // #[priority(3)]
    // #[rule("__const")]
    // GnuConst,
    // #[priority(3)]
    // #[rule("__extension__")]
    // GnuExtension,
    // #[priority(3)]
    // #[rule("__func__")]
    // GnuFunc,
    #[priority(3)]
    #[rule("__imag" "__"?)]
    GnuImaginary,
    // #[priority(3)]
    // #[rule("__inline" "__"?)]
    // GnuInline,
    // #[priority(3)]
    // #[rule("__label__")]
    // GnuLabel,
    // #[priority(3)]
    // #[rule("__null")]
    // GnuNull,
    #[priority(3)]
    #[rule("__real" "__"?)]
    GnuReal,
    // #[priority(3)]
    // #[rule("__restrict" "__"?)]
    // GnuRestrict,
    // #[priority(3)]
    // #[rule("__signed" "__"?)]
    // GnuSigned,
    // #[priority(3)]
    // #[rule("__thread")]
    // GnuThread,
    // #[priority(3)]
    // #[rule("__typeof")]
    // GnuTypeof,
    // #[priority(3)]
    // #[rule("__volatile" "__"?)]
    // GnuVolatile,
    #[priority(2)]
    #[rule((('u' '8'?) | 'U' | 'L')? '\'' (
        ^['\\', '\'']
        | ('\\' (
            ^['0'..'7', 'x']
            | ('x' ['0'..'9', 'a'..'f', 'A'..'F']*)
            | (['0'..'7']+))
        )
    ) '\''?)]
    Character,
    #[priority(2)]
    #[rule((('u' '8'?) | 'U' | 'L')? '"' (
        ^['\\', '"']
        | ('\\' (
            ^['0'..'7', 'x']
            | ('x' ['0'..'9', 'a'..'f', 'A'..'F']*)
            | (['0'..'7']+))
        )
    )* '"'?)]
    String,
    #[priority(3)]
    #[rule('0' ['x', 'X'] ['0'..'9', 'a'..'f', 'A'..'F', '\'', '_']+ ['u', 'l', 'U', 'L', 'w', 'W', 'B', 'b']*)]
    HexNumber,
    #[priority(3)]
    #[rule('0' ['0'..'7', '\'', '_']+ ['u', 'l', 'U', 'L', 'w', 'W', 'B', 'b']*)]
    OctNumber,
    #[rule(['1'..'9'] ['0'..'9', '\'', '_'] ('.' ['0'..'9', '\'', '_']*)? ('e' '-'? ['0'..'9', '\'', '_']+)?
    ['l', 'L', 'f', 'F', 'i', 'u', 'U', 'w', 'W', 'B', 'b', 'd', 'D']*)]
    Number,
    #[priority(2)]
    #[rule('.' ['0'..'9', '\'', '_']+ ('e' '-'? ['0'..'9', '\'', '_']+)? ['l', 'L', 'f', 'F', 'i', 'd', 'D']*)]
    DotNumber,
    #[priority(3)]
    #[rule('0' ['b', 'B'] ['0', '1', '\'', '_']+ ['u', 'l', 'U', 'L', 'w', 'W', 'B', 'b']*)]
    BinNumber,

    #[rule('\\' & .)]
    Backslash,

    #[rule(($alpha | '_')+)]
    Ident,

    // Preprocessor
    #[priority(2)]
    #[rule('#' $space* "include" $space* (
        ('<' (^['\n', '>', '\\'] | ('\\' .))* '>'?)
      | ('"' (^['\n', '"', '\\'] | ('\\' .))* '"'?)
    ))]
    IncludeDirective,
    #[priority(2)]
    #[rule('#' $space* "define")]
    DefineDirective,
    #[priority(2)]
    #[rule('#' $space* "undef")]
    UnDefDirective,
    #[priority(2)]
    #[rule('#' $space* "if")]
    IfDirective,
    #[priority(3)]
    #[rule('#' $space* "ifdef")]
    IfDefDirective,
    #[priority(3)]
    #[rule('#' $space* "ifndef")]
    IfNotDefDirective,
    #[priority(2)]
    #[rule('#' $space* "else")]
    ElseDirective,
    #[priority(2)]
    #[rule('#' $space* "elif")]
    ElseIfDirective,
    #[priority(3)]
    #[rule('#' $space* "elifdef")]
    ElseIfDefDirective,
    #[priority(3)]
    #[rule('#' $space* "elifndef")]
    ElseIfNotDefDirective,
    #[priority(2)]
    #[rule('#' $space* "endif")]
    EndIfDirective,
    #[priority(2)]
    #[rule('#' $space* "pragma" (^['\\', '\n'] | ('\\' .))* '\n'?)]
    PragmaDirective,
    #[priority(2)]
    #[rule('#' $space* "error" (^['\\', '\n'] | ('\\' .))* '\n'?)]
    ErrorDirective,
    #[priority(2)]
    #[rule('#' $space* "warning" (^['\\', '\n'] | ('\\' .))* '\n'?)]
    WarnDirective,
    #[priority(2)]
    #[rule('#' $space* "line" (^['\\', '\n'] | ('\\' .))* '\n'?)]
    LineDirective,

    #[rule($space+)]
    Whitespace,
    #[priority(2)]
    #[rule(['\n', '\r']+)]
    NewLine,

    #[priority(5)]
    #[rule("/*" (^['*'] | ('*' ^['/'])* "*/"?))]
    MlComment,
    #[priority(5)]
    #[rule("//" (^['\n', '\\'] | ('\\' .))* '\n'?)]
    Comment,

    // Operators
    #[rule('~')]
    Tilde,
    #[rule('!')]
    Bang,
    #[priority(2)]
    #[rule("++")]
    Inc,
    #[priority(2)]
    #[rule("--")]
    Dec,
    #[rule('+')]
    Add,
    #[rule('-')]
    Sub,

    #[rule('*')]
    Star,
    #[rule('/')]
    Slash,
    #[rule('%')]
    Percent,

    #[priority(2)]
    #[rule("<<" | ">>")]
    BitShift,

    #[rule('&')]
    Amp,
    #[rule('|')]
    Pipe,
    #[rule('^')]
    Xor,
    #[rule(('<' | '>') '='?)]
    BoolOp,
    #[priority(2)]
    #[rule(('=' | '!') '=')]
    EqOrNot,

    #[priority(2)]
    #[rule("&&")]
    LogicalAnd,
    #[priority(2)]
    #[rule("||")]
    LogicalOr,

    #[rule('?')]
    Quest,

    #[priority(3)]
    #[rule((['*', '/', '%', '+', '-', '&', '^', '|'] | "<<" | ">>") '=')]
    SetOp,

    #[rule('=')]
    Set,

    #[rule('(')]
    POpen,
    #[rule(')')]
    PClose,
    #[priority(2)]
    #[rule('[' | "<:")]
    BkOpen,
    #[priority(2)]
    #[rule(']' | ":>")]
    BkClose,
    #[priority(2)]
    #[rule('{' | "<%")]
    BcOpen,
    #[priority(2)]
    #[rule('}' | "%>")]
    BcClose,

    #[rule(';')]
    Semicolon,
    #[rule(',')]
    Comma,
    #[rule('.')]
    Dot,
    #[rule(':')]
    Colon,
    #[priority(2)]
    #[rule("::")]
    DColon,
    #[priority(2)]
    #[rule("...")]
    Variadic,

    #[priority(2)]
    #[rule("->")]
    Arrow,

    #[priority(2)]
    #[rule('#' | "%:")]
    Hash,
    #[priority(3)]
    #[rule("##" | "%:%:")]
    DHash,

    // After-preprocessor section
    Unreachable,
    MacroName,
    MacroPOpen,
    MacroArg,
    MacroPClose,
}
