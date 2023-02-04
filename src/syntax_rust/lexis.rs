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

use lady_deirdre::lexis::Token;

#[derive(Token, Debug, Clone)]
#[define(BIN = ['0', '1', '_'])]
#[define(OCT = ['0'..'7', '_'])]
#[define(DEC = ['0'..'9', '_'])]
#[define(HEX = DEC | ['A'..'F', 'a'..'f'])]
#[define(NUM_TYPES = ("i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64"))]
#[define(ESCAPE = '\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0'] | ('x' & HEX & HEX)
| ("u{" & HEX & HEX & HEX & HEX & '}')))]
#[define(LETTER = ^['@', '~', '$', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '!', '#', '%', '^', '&',
'*', '(', ')', '-', '=', '+', '[', ']', '{', '}', '\\', '|', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?',
'\r', '\t', '\n', ' ', '\x0b', '\x0c'])]
pub enum RustToken {
    // KEYWORDS
    KeywordAs,
    KeywordAsync,
    KeywordAwait,
    KeywordBreak,
    KeywordConst,
    KeywordContinue,
    KeywordCrate,
    KeywordDo,
    KeywordDyn,
    KeywordElse,
    KeywordEnum,
    KeywordExtern,
    KeywordFalse,
    KeywordFn,
    KeywordFor,
    KeywordIf,
    KeywordImpl,
    KeywordIn,
    KeywordLet,
    KeywordLoop,
    KeywordMacro,
    KeywordMatch,
    KeywordMod,
    KeywordMove,
    KeywordMut,
    KeywordPub,
    KeywordRef,
    KeywordReturn,
    KeywordSelf,
    KeywordUSelf,
    KeywordStatic,
    KeywordStruct,
    KeywordSuper,
    KeywordTrait,
    KeywordTrue,
    KeywordTry,
    KeywordType,
    KeywordUnion,
    KeywordUnsafe,
    KeywordUse,
    KeywordWhere,
    KeywordWhile,
    KeywordYield,

    #[precedence(10)]
    #[rule("macro_rules!")]
    KeywordMacroRules,

    // IDENTIFIERS

    NumType,
    BasicType,

    #[precedence(3)]
    #[rule("0b" & BIN+ & ('.' & BIN*)? & NUM_TYPES?)]
    BinNumber,

    #[precedence(3)]
    #[rule("0o" & OCT+ & ('.' & OCT*)? & NUM_TYPES?)]
    OctNumber,

    #[precedence(2)]
    #[rule(DEC+ & ('.' & DEC*)? & NUM_TYPES?)]
    DecNumber,

    #[precedence(3)]
    #[rule("0x" & HEX+ & ('.' & HEX*)? & NUM_TYPES?)]
    HexNumber,

    #[rule(('b'? & '"' & (ESCAPE | ^['"', '\\'])* & '"'))]
    String,

    #[constructor(transform_ident)]
    #[rule("r#"? & LETTER & (LETTER | DEC)*)]
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

    #[rule('{')]
    BraceOpen,

    #[rule('}')]
    BraceClose,

    #[rule('[')]
    BracketOpen,

    #[rule(']')]
    BracketClose,

    // CHARACTERS

    Underscore,

    #[rule(',')]
    Comma,

    #[rule('.')]
    Point,

    #[precedence(2)]
    #[rule("..")]
    Range,

    #[rule('\'')]
    Apostrophe,

    #[precedence(2)]
    #[rule("b'")]
    AsciiChar,

    #[precedence(2)]
    #[rule("::")]
    DoubleColon,

    #[rule(':')]
    Colon,

    #[rule('$')]
    Dollar,

    #[rule(';')]
    Semicolon,

    #[rule("-" | "%" | "|" | "^" | "<<" | ">>" | "==" | "!=" | "<=" | ">=" | "&&" | "||")]
    Operator,

    #[rule('+')] // operator
    Add,

    #[rule('=')] // operator
    Assign,

    #[rule('&')] // operator
    Amp,

    #[rule('*')] // operator
    Star,

    #[rule('/')] // operator
    Slash,

    #[rule('~')]
    Tilda,

    #[rule('@')]
    At,

    #[rule('\\')]
    Backslash,

    #[precedence(2)]
    #[rule(ESCAPE)]
    Escape,

    #[rule('!')]
    Bang,

    #[rule('?')]
    QuestMark,

    #[rule('#')]
    Hash,

    #[precedence(2)]
    #[rule("#!")]
    HashBang,

    #[precedence(5)]
    #[rule("->")]
    Arrow,

    #[precedence(2)]
    #[rule("+=" | "-=" | "*=" | "/=" | "%=" | "|=" | "&=" | "^=" | "<<=" | ">>=")]
    AssignWithOperation,

    #[rule((' ' | '\t' | '\x0b' | '\x0c' | '\r')+)]
    Whitespace,

    #[rule('\n')]
    NewLine,

    // COMMENTS

    #[precedence(4)]
    #[rule("//")]
    SingleComment,

    #[precedence(5)]
    #[rule("//!")]
    InnerSlComment,

    #[precedence(5)]
    #[rule("///")]
    OuterSlComment,

    #[precedence(4)]
    #[rule("/*")]
    MultilineCommentOpen,

    #[precedence(5)]
    #[rule("/*!")]
    InnerMlCommentOpen,

    #[precedence(5)]
    #[rule("/**")]
    OuterMlCommentOpen,

    #[precedence(5)]
    #[rule("*/")]
    MultilineCommentClose,

    #[mismatch]
    Mismatch,
}

impl RustToken {
    fn transform_ident(string: &str) -> Self {
        match string {
            "as"       => Self::KeywordAs,
            "async"    => Self::KeywordAsync,
            "await"    => Self::KeywordAwait,
            "break"    => Self::KeywordBreak,
            "const"    => Self::KeywordConst,
            "continue" => Self::KeywordContinue,
            "crate"    => Self::KeywordCrate,
            "do"       => Self::KeywordDo,
            "dyn"      => Self::KeywordDyn,
            "else"     => Self::KeywordElse,
            "enum"     => Self::KeywordEnum,
            "extern"   => Self::KeywordExtern,
            "false"    => Self::KeywordFalse,
            "fn"       => Self::KeywordFn,
            "for"      => Self::KeywordFor,
            "if"       => Self::KeywordIf,
            "impl"     => Self::KeywordImpl,
            "in"       => Self::KeywordIn,
            "let"      => Self::KeywordLet,
            "loop"     => Self::KeywordLoop,
            "macro"    => Self::KeywordMacro,
            "match"    => Self::KeywordMatch,
            "mod"      => Self::KeywordMod,
            "move"     => Self::KeywordMove,
            "mut"      => Self::KeywordMut,
            "pub"      => Self::KeywordPub,
            "ref"      => Self::KeywordRef,
            "return"   => Self::KeywordReturn,
            "self"     => Self::KeywordSelf,
            "Self"     => Self::KeywordUSelf,
            "static"   => Self::KeywordStatic,
            "struct"   => Self::KeywordStruct,
            "super"    => Self::KeywordSuper,
            "trait"    => Self::KeywordTrait,
            "true"     => Self::KeywordTrue,
            "try"      => Self::KeywordTry,
            "type"     => Self::KeywordType,
            "union"    => Self::KeywordUnion,
            "unsafe"   => Self::KeywordUnsafe,
            "use"      => Self::KeywordUse,
            "where"    => Self::KeywordWhere,
            "while"    => Self::KeywordWhile,
            "yield"    => Self::KeywordYield,
            "_"        => Self::Underscore,
            "str" | "bool" => Self::BasicType,
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" => Self::NumType,
            _ => Self::Identifier,
        }
    }
}

// ("r\"" & (ESCAPE | ^['"', '\\'])* & '\"')
// ("r#\"" & (ESCAPE | ^['"', '\\'])* & "\"#")
// ("r###\"" & (ESCAPE | ^['"', '\\'])* & "\"###")

/*
struct S {}                  done
     struct S { x: T }       done
     struct S(T);            done
     struct S;               done
enum E {}                    done
     enum E { A, B(), C {} } done
     enum E { A = 1 }        done
union U {}                   todo
static X: T = T();           todo
const X: T = T();            todo
let x: T (= ...);            todo
let mut x: T (= ...);        todo

S { x: y }
S { x }
S { ..s }
S { 0: x }
S(x)
E::C { x: y }
()	Empty tuple
(x)	Parenthesized expression.
(x,)
(S,)
[S]
[S; n]
[x; n]
[x, y]
x[0]
     x[..]
a..b
..b
..=b
a..=b
a..
..
s.x	Named field access
s.0	Numbered field access

impl S {}
impl T for S {}
impl !T for S {}
const fn f() {}
     async { x }
fn() -> S
Fn() -> S
|| {}
     |x| {}
     |x| x + x
     move |x| x + y
     return || true
unsafe
     unsafe fn f() {}
     unsafe trait T {}
     unsafe { f(); }
     unsafe impl T for S {}	Guarantees S is well-behaved w.r.t T; people may use T on S safely.

while x {}	Loop, REF run while expression x is true.
loop {}	Loop indefinitely REF until break. Can yield value with break x.
for x in collection {}	Syntactic sugar to loop over iterators. BK STD REF
     collection.into_iter()	Effectively converts any IntoIterator STD type into proper iterator first.
     iterator.next()	On proper Iterator STD then x = next() until exhausted (first None).
if x {} else {}	Conditional branch REF if expression is true.
'label: {}	Block label, RFC can be used with break to exit out of this block. 1.65+
'label: loop {}	Similar loop label, EX REF useful for flow control in nested loops.
break	Break expression REF to exit a labelled block or loop.
     break 'label x	Break out of block or loop named 'label and make x its value.
     break 'label	Same, but don't produce any value.
     break x	Make x value of the innermost loop (only in actual loop).
continue	Continue expression REF to the next loop iteration of this loop.
continue 'label	Same but instead of this loop, enclosing loop marked with 'label.
x?	If x is Err or None, return and propagate. BK EX STD REF
x.await	Syntactic sugar to get future, poll, yield. REF '18 Only works inside async.
     x.into_future()	Effectively converts any IntoFuture STD type into proper future first.
     future.poll()	On proper Future STD then poll() and yield flow if Poll::Pending. STD
return x	Early return REF from function. More idiomatic is to end with expression.
     { return }	Inside normal {}-blocks return exits surrounding function.
     || { return }	Within closures return exits that closure only, i.e., closure is s. function.
     async { return }	Inside async a return only REF 🛑 exits that {}, i.e., async {} is s. function.
f()	Invoke callable f (e.g., a function, closure, function pointer, Fn, …).
x.f()	Call member function, requires f takes self, &self, … as first argument.
     X::f(x)	Same as x.f(). Unless impl Copy for X {}, f can only be called once.
     X::f(&x)	Same as x.f().
     X::f(&mut x)	Same as x.f().
     S::f(&x)	Same as x.f() if X derefs to S, i.e., x.f() finds methods of S.
     T::f(&x)	Same as x.f() if X impl T, i.e., x.f() finds methods of T if in scope.
X::f()	Call associated function, e.g., X::new().
     <X as T>::f()	Call trait method T::f() implemented for X.

a::b
     ::b
     crate::b
     self::b
     super::b
use a::b as _;
     pub(in a::b) T
extern crate a;
extern "C" {}
extern "C" fn f() {}
     self: Box<Self>
<S as T>
a::b as c
x as u32

m!()

$x:ty
$x
$(x),*
     $(x),?
     $(x),+
     $(x)<<+

match m {}
let S(x) = get();
     let S { x } = s;
     let (_, b, _) = abc;
     let (a, ..) = abc;
     let (.., a, b) = (1, 2);
     let s @ S { x } = get();
     let w @ t @ f = get();
     let (|x| x) = get();
let Some(x) = get();
let Some(x) = get() else {};
if let Some(x) = get() {}
while let Some(x) = get() {}
fn f(S { x }: S)

E::A => {}
E::B ( .. ) => {}
E::C { .. } => {}
S { x: 0, y: 1 } => {}
S { x: a, y: b } => {}
     S { x, y } => {}
S { .. } => {}
D => {}
D => {}
_ => {}
0 | 1 => {}
     E::A | E::Z => {}
     E::C {x} | E::D {x} => {}
     Some(A | B) => {}
     |x| x => {}
(a, 0) => {}
[a, 0] => {}
     [1, ..] => {}
     [1, .., 5] => {}
     [1, x @ .., 5] => {}
     [a, x @ .., b] => {}
1 .. 3 => {}
     1 ..= 3 => {}
     1 .. => {}
x @ 1..=5 => {}
     Err(x @ Error {..}) => {}
S { x } if x > 10 => {}
     where T: R + 'a
     where T: ?Sized
     where T: 'a
     where T: 'static
     where 'b: 'a
     where u8: R<T>
S<const N: usize>
     S<10>
     S<{5+5}>
     S<const N: u8 = 0>
     S<T = u8>
S<'_>
S<_>
S::<T>
impl<T> S<T> {}
impl S<T> {}
fn f() -> impl T
fn f(x: &impl T)
fn f(x: &dyn T)
fn f<X: T>(x: X)
fn f() where Self: R;
     fn f() where Self: Sized;
     fn f() where Self: R {}
for<'a>
     trait T: for<'a> R<'a> {}
fn(&'a u8)
for<'a> fn(&'a u8)
     fn(&'_ u8)
     fn(&u8)
dyn for<'a> Fn(&'a u8)
     dyn Fn(&'_ u8)
     dyn Fn(&u8)
impl<'a> T for fn(&'a u8) {}
impl T for for<'a> fn(&'a u8) {}
     impl T for fn(&u8) {}
r#"..."#	Raw string literal
!
     fn f() -> ! {}
     fn f() -> Result<(), !> {}
     fn f(x: !) {}
_
     let _ = x;
     _ = x;
*/
