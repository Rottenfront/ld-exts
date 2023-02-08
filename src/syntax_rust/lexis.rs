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
#[define(HEX = DEC | ['A'..'F', 'a'..'f', '_'])]
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
    #[precedence(3)]
    #[rule(NUM_TYPES)]
    NumType,

    #[precedence(3)]
    #[rule("str" | "bool")]
    BasicType,

    #[precedence(3)]
    #[rule("0b" & BIN+ & ('.' & BIN+)? & NUM_TYPES?)]
    BinNumber,

    #[precedence(3)]
    #[rule("0o" & OCT+ & ('.' & OCT+)? & NUM_TYPES?)]
    OctNumber,

    #[precedence(2)]
    #[rule(DEC+ & ('.' & DEC+)? & NUM_TYPES?)]
    DecNumber,

    #[precedence(3)]
    #[rule("0x" & HEX+ & ('.' & HEX+)? & NUM_TYPES?)]
    HexNumber,

    #[rule(('"' & (ESCAPE | ^['"', '\\'])* & '"'))]
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
    #[precedence(5)]
    #[rule("//")]
    SingleComment,

    #[precedence(5)]
    #[rule("/*")]
    MultilineCommentOpen,

    #[precedence(5)]
    #[rule("*/")]
    MultilineCommentClose,

    #[mismatch]
    Mismatch,
}

impl RustToken {
    fn transform_ident(string: &str) -> Self {
        match string {
            "as" => Self::KeywordAs,
            "async" => Self::KeywordAsync,
            "await" => Self::KeywordAwait,
            "break" => Self::KeywordBreak,
            "const" => Self::KeywordConst,
            "continue" => Self::KeywordContinue,
            "crate" => Self::KeywordCrate,
            "do" => Self::KeywordDo,
            "dyn" => Self::KeywordDyn,
            "else" => Self::KeywordElse,
            "enum" => Self::KeywordEnum,
            "extern" => Self::KeywordExtern,
            "false" => Self::KeywordFalse,
            "fn" => Self::KeywordFn,
            "for" => Self::KeywordFor,
            "if" => Self::KeywordIf,
            "impl" => Self::KeywordImpl,
            "in" => Self::KeywordIn,
            "let" => Self::KeywordLet,
            "loop" => Self::KeywordLoop,
            "macro" => Self::KeywordMacro,
            "match" => Self::KeywordMatch,
            "mod" => Self::KeywordMod,
            "move" => Self::KeywordMove,
            "mut" => Self::KeywordMut,
            "pub" => Self::KeywordPub,
            "ref" => Self::KeywordRef,
            "return" => Self::KeywordReturn,
            "self" => Self::KeywordSelf,
            "Self" => Self::KeywordUSelf,
            "static" => Self::KeywordStatic,
            "struct" => Self::KeywordStruct,
            "super" => Self::KeywordSuper,
            "trait" => Self::KeywordTrait,
            "true" => Self::KeywordTrue,
            "try" => Self::KeywordTry,
            "type" => Self::KeywordType,
            "union" => Self::KeywordUnion,
            "unsafe" => Self::KeywordUnsafe,
            "use" => Self::KeywordUse,
            "where" => Self::KeywordWhere,
            "while" => Self::KeywordWhile,
            "yield" => Self::KeywordYield,
            "_" => Self::Underscore,
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
     struct S ​(T);            done
     struct S;               done
enum E {}                    done
     enum E { A, B​(), C {} } done
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
S​ (x)
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

trait T {}
trait T : R {}
impl S {}
impl T for S {}
impl !T for S {}
fn f() {}
     fn f() -> S {}
     fn f(&self) {}
struct S ​(T);
const fn f() {}
async fn f() {}
     async fn f() -> S {}
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
Control Flow
Control execution within a function.

Example	Explanation
while x {}	Loop, REF run while expression x is true.
loop {}	Loop indefinitely REF until break. Can yield value with break x.
for x in collection {}	Syntactic sugar to loop over iterators. BK STD REF
     collection.into_iter()	Effectively converts any IntoIterator STD type into proper iterator first.
     iterator.next()
if x {} else {}
'label: {}
'label: loop {}
break
     break 'label x
     break 'label
     break x
continue
continue 'label
x?
x.await
     x.into_future()
     future.poll()
return x
     { return }
     || { return }
     async { return }
f()
x.f()
     X::f(x)
     X::f(&x)
     X::f(&mut x)
     S::f(&x)
     T::f(&x)
X::f()
     <X as T>::f()

a::b
     ::b
     crate::b
     self::b
     super::b
use a::b;
use a::{b, c};
use a::b as x;
use a::b as _;
use a::*;
pub use a::b;
pub T
     pub(crate) T
     pub(super) T
     pub(self) T
     pub(in a::b) T
extern crate a;
extern "C" {}
extern "C" fn f() {}

type T = S;
Self
self
     &self
     &mut self
     self: Box<Self>
<S as T>
a::b as c
x as u32

m!()
#[attr]
#![attr]

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

struct S<T>
S<T> where T: R
     where T: R, P: S
     where T: R + S
     where T: R + 'a
     where T: ?Sized
     where T: 'a
     where T: 'static
     where 'b: 'a
     where u8: R<T>
S<T: R>
S<const N: usize>
     S<10>
     S<{5+5}>
S<T = R>
     S<const N: u8 = 0>
     S<T = u8>
S<'_>
S<_>
S::<T>
trait T<X> {}
trait T { type X; }
trait T { type X<G>; }
trait T { type X<'a>; }
     type X = R;
     type X<G> = R<G>;
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
!	Always empty never type. BK EX STD REF
     fn f() -> ! {}	Function that never returns; compat. with any type e.g., let x: u8 = f();
     fn f() -> Result<(), !> {}	Function that must return Result but signals it can never Err. 🚧
     fn f(x: !) {}	Function that exists, but can never be called. Not very useful. 🝖 🚧
_	Unnamed wildcard REF variable binding, e.g., |x, _| {}.
     let _ = x;	Unnamed assignment is no-op, does not 🛑 move out x or preserve scope!
     _ = x;	You can assign anything to _ without let, i.e., _ = ignore_error(); 1.59+ 🔥
x;	Statement REF terminator, c. expressions EX REF
*/

