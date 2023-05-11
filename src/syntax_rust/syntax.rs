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
 * FITNESS FOR A PARTICULAR PURPOSE AN D NONINFRINGEMENT. IN NO EVENT SHALL THE  
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, 
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE 
 * SOFTWARE.                                                                     
*/

use lady_deirdre::{
    lexis::TokenRef,
    syntax::{Node, NodeRef},
};
use std::vec::Vec;

#[derive(Node, Debug, Clone)]
#[token(super::lexis::RustToken)]
#[error(lady_deirdre::syntax::SyntaxError)]
#[skip($Whitespace | $NewLine)]
/*
#[define(ANY = ($As | $Async | $Await | $Break | $Const | $Continue
| $Crate | $Do | $Dyn | $Else | $Enum | $Extern | $False
| $Fn | $For | $If | $Impl | $In | $Let | $Loop | $Macro
| $Match | $Mod | $Move | $Mut | $Pub | $Ref | $Return
| $LSelf | $USelf | $Static | $Struct | $Super | $Trait | $True
| $Try | $Type | $Union | $Unsafe | $Use | $Where | $While
| $Yield | $MacroRules | $NumType | $BasicType | $BinNumber | $OctNumber | $DecNumber | $HexNumber
| $String | $Identifier | $ParenthesisOpen | $ParenthesisClose | $AngleBracketOpen | $AngleBracketClose | $BraceOpen
| $BraceClose | $BracketOpen | $BracketClose | $Underscore | $Comma | $Point | $Range | $Apostrophe | $AsciiChar
| $DoubleColon | $Colon | $Dollar | $Semicolon | $Operator | $Add | $Assign | $Amp | $Star | $Slash | $Tilda | $At
| $Backslash | $Escape | $Bang | $QuestMark | $Hash | $HashBang | $Arrow | $AssignWithOperation | $Whitespace
| $SingleComment | $MultilineCommentOpen | $Mismatch | $Bang | $QuestMark))]
*/
#[define(ANY = ($Ident | $Comma | $Point | $Range | $Apostrophe))]
/*
#[define(ATTR_ITEM = ($As | $Async | $Await | $Break | $Const | $Continue
| $Crate | $Do | $Dyn | $Else | $Enum | $Extern | $False
| $Fn | $For | $If | $Impl | $In | $Let | $Loop | $Macro
| $Match | $Mod | $Move | $Mut | $Pub | $Ref | $Return
| $LSelf | $USelf | $Static | $Struct | $Super | $Trait | $True
| $Try | $Type | $Union | $Unsafe | $Use | $Where | $While
| $Yield | $MacroRules | $NumType | $BasicType | $BinNumber | $OctNumber | $DecNumber | $HexNumber
| $String | $Identifier | $Underscore | $Comma | $Point | $Range | $Apostrophe | $AsciiChar | $DoubleColon | $Colon
| $Dollar | $Semicolon | $Operator | $Add | $Assign | $Amp | $Star | $Slash | $Tilda | $At | $Backslash | $Escape
| $Bang | $QuestMark | $Hash | $HashBang | $Arrow | $AssignWithOperation | $Mismatch))]
*/
#[define(ATTR_ITEM = ($Ident))]
#[define(PATH_ITEM = ($Ident | $LSelf | $USelf | $Super))]
pub enum RustNode {
    // Root
    #[root]
    #[rule((items: RootItem)*)]
    Root { items: Vec<NodeRef> },

    #[rule((attrs: AttrOuter)* & (mods: PubConstruct)? & (mods: (Extern | Unsafe))?
    & (value: (StructDefConstruct | AttrInner | EnumDefConstruct | UseConstruct | FnDefConstruct | TraitDef
    | ImplStatement | TypeDef | ModuleDef | TraitDef | ImplStatement | ConstDef | StaticDef)))]
    RootItem {
        attrs: Vec<NodeRef>,
        mods: Vec<NodeRef>,
        value: NodeRef,
    },

    // A
    #[rule($HashBang & $BracketOpen & (items: (($Open & ATTR_ITEM* & $Close)
    | ($BracketOpen & ATTR_ITEM* & $BracketClose) | ($Less & ATTR_ITEM* & $Greater)
    | ($BraceOpen & ATTR_ITEM* & $BraceClose) | ATTR_ITEM))* & $BracketClose)]
    AttrInner { items: Vec<TokenRef> },

    #[rule($Hash & $BracketOpen & (items: (ATTR_ITEM | ($Open & ATTR_ITEM* & $Close)
    | ($BracketOpen & ATTR_ITEM* & $BracketClose) | ($BraceOpen & ATTR_ITEM* & $BraceClose)))*
    & $BracketClose)]
    AttrOuter { items: Vec<TokenRef> },

    // B
    #[rule(value: $BasicType)]
    BasicType { value: TokenRef },

    // C
    #[rule(value: (($AsciiChar | $Apostrophe) & ANY? & $Apostrophe))]
    Char {
        value: Vec<TokenRef>,
    },
    #[rule($BraceOpen & $BraceClose)]
    CodeBlock,

    #[comment]
    #[rule($SingleComment & (value: (ANY | $CommentClose | ($Backslash & ($NewLine)? | $Whitespace))*))]
    CommentSingle { value: Vec<TokenRef> },

    #[comment]
    #[rule($CommentOpen & (value: (ANY | $NewLine | $Whitespace)*) & $CommentClose)]
    CommentMultiline { value: Vec<TokenRef> },

    // D

    // E
    #[rule((attrs: AttrOuter)* & (name: $Ident) & (additional: (EnumItemAnonFields | EnumItemFields))?
        & ($Set & (num: Number))?)]
    EnumItem {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        additional: NodeRef,
        num: NodeRef,
    },

    #[rule($Open & (items: Type)*{$Comma} & $Close)]
    EnumItemAnonFields { items: Vec<NodeRef> },

    #[rule($BraceOpen & (items: StructItem)*{$Comma} & $BraceClose)]
    EnumItemFields { items: Vec<NodeRef> },

    #[rule($Enum & (name: $Ident) & (generic: GenericDef)? & (where_cond: Where)?
        & $BraceOpen & ((inner_attrs: AttrInner) | ((items: EnumItem) & $Comma))* & ((inner_attrs: AttrInner)
        | (items: EnumItem))? & $BraceClose)]
    EnumDefConstruct {
        name: TokenRef,
        generic: NodeRef,
        where_cond: NodeRef,
        inner_attrs: Vec<NodeRef>,
        items: Vec<NodeRef>,
    },

    #[rule($Extern & (lang: String))]
    Extern { lang: NodeRef },

    // F
    // #[rule($False)]
    // False,
    #[rule((name: $Ident) & $Colon & (_type: Type))]
    FnParameter { name: TokenRef, _type: NodeRef },
    /*

    #[rule((parent: Path) & $ParenthesisOpen & (name: $Identifier) & $ParenthesisClose & $Colon & (_type: Type))]
    FnParameterWithParent {
        parent: NodeRef,
        name: TokenRef,
        _type: NodeRef,
    },

    #[rule((is_absolute: $DoubleColon)? & ((path: PATH_ITEM) & $DoubleColon)* & (path: PATH_ITEM) & ($DoubleColon & (path: $Star))?)]
    Path {
        is_absolute: TokenRef,
        path: Vec<TokenRef>,
    },
    */
    #[rule((refer: Reference)? & $LSelf)]
    SelfUse { refer: NodeRef },

    #[rule((attrs: AttrOuter)* & (value: (FnParameter | SelfUse)))]
    FnParameterConstruct { attrs: Vec<NodeRef>, value: NodeRef },

    #[rule($Arrow & (_type: Type))]
    FnTyping { _type: NodeRef },

    #[rule($Fn & (name: $Ident) & (generic: GenericDef)?
    & ($Open & (params: FnParameterConstruct)*{$Comma} & $Close) & (_type: FnTyping)?
    & (where_cond: Where)? & (code: (CodeBlock | Semicolon)))]
    FnDefConstruct {
        name: TokenRef,
        generic: NodeRef,
        params: Vec<NodeRef>,
        _type: NodeRef,
        where_cond: NodeRef,
        code: NodeRef,
    },

    // G
    #[rule($Less & (items: TypeForGeneric)+{$Comma} & $Greater)]
    GenericDef { items: Vec<NodeRef> },

    #[rule($Less & (items: GenericUseType)+{$Comma} & $Greater)]
    GenericUse { items: Vec<NodeRef> },

    #[rule(((is_absolute: $DoubleColon)? & (path: ($LSelf | $USelf | $Crate | $Super)) & $DoubleColon
    & ((path: PATH_ITEM) & $DoubleColon)* & (path: PATH_ITEM)) | (((path: $Ident) & $DoubleColon)*
    & (path: $Ident) & $Set & (_type: Type)))]
    GenericUseType {
        is_absolute: TokenRef,
        path: Vec<TokenRef>,
        _type: NodeRef,
    },

    // H

    // I
    #[rule($Impl & (bang: $Bang)? & (_type: Path) & ($For & (for_t: Path))? & (code: TraitBlock))]
    ImplStatement {
        _type: NodeRef,
        bang: TokenRef,
        for_t: NodeRef,
        code: NodeRef,
    },

    // J

    // K

    // L
    #[rule($Apostrophe & (value: ($Ident | $Static)))]
    Lifetime { value: TokenRef },

    // M
    #[rule($BraceOpen & (items: RootItem)* & $BraceClose)]
    ModuleBlock { items: Vec<NodeRef> },

    #[rule($Mod & (name: $Ident) & (code: (Semicolon | ModuleBlock)))]
    ModuleDef { name: TokenRef, code: NodeRef },

    // N
    #[rule(value: $Number)]
    Number { value: TokenRef },

    // O

    // P
    #[rule((prefix: (($Crate | $Super+{$DoubleColon} | $LSelf | $USelf)? & $DoubleColon))?
        & (path: $Ident)+{$DoubleColon})]
    Path {
        prefix: Vec<TokenRef>,
        path: Vec<TokenRef>,
    },

    #[rule($Pub & ($Open & (pub_for: PubFor | PubIn) & $Close)?)]
    PubConstruct { pub_for: NodeRef },

    #[rule(path: ($Super | $Crate))]
    PubFor { path: TokenRef },

    #[rule($In & (path: Path))]
    PubIn { path: NodeRef },

    // Q

    // R
    #[rule(((refer: $Refer)+ & (lf: Lifetime)? & (mutability: $Mut)?) | (mutability: $Mut))]
    Reference {
        refer: Vec<TokenRef>,
        lf: NodeRef,
        mutability: TokenRef,
    },

    // S
    #[rule(value: $String)]
    String { value: TokenRef },

    #[rule($Semicolon)]
    Semicolon,

    #[rule((attrs: AttrOuter)* & (name: $Ident) & $Colon & (_type: Type))]
    StructItem {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        _type: NodeRef,
    },

    #[rule($BraceOpen & ((inner_attrs: AttrInner) | ((items: StructItem) & $Comma))*
        & ((inner_attrs: AttrInner) | (items: StructItem))? & $BraceClose)]
    StructDefStatement {
        inner_attrs: Vec<NodeRef>,
        items: Vec<NodeRef>,
    },

    #[rule(($Open & (items: Type)*{$Comma} & $Close)? & $Semicolon)]
    ShortStructDefStatement { items: Vec<NodeRef> },

    #[rule($Struct & (name: $Ident) & (generic: GenericDef)?
        & (where_cond: Where)? & (value: (ShortStructDefStatement | StructDefStatement)))]
    StructDefConstruct {
        generic: NodeRef,
        where_cond: NodeRef,
        name: TokenRef,
        value: NodeRef,
    },

    // T
    #[rule((refer: Reference)? & (value: TypeNoRefer))]
    Type { refer: NodeRef, value: NodeRef },

    #[rule((modifier: ($Impl | $Dyn | $Quest))? & (value: (BasicType | Path)) & (generic: GenericUse)?)]
    TypeNoRefer { modifier: TokenRef, value: NodeRef, generic: NodeRef },

    #[rule((attrs: AttrOuter)* & (name: $Ident) & ($Colon
        & (value: (TypeNoRefer | Lifetime))+{$Add})?)]
    TypeForGeneric {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        value: Vec<NodeRef>,
    },

    #[rule((attrs: AttrOuter)* & (name: Path) & $Colon & ((value: (TypeNoRefer | Lifetime)) & $Add)*
    & (value: (TypeNoRefer | Lifetime)))]
    TypeForWhere {
        attrs: Vec<NodeRef>,
        name: NodeRef,
        value: Vec<NodeRef>,
    },

    #[rule($Colon & ((traits: TypeNoRefer) & $Add)* & (traits: TypeNoRefer))]
    TraitInherit { traits: Vec<NodeRef> },

    #[rule($Trait & (name: $Ident) & (generic: GenericDef)?
        & (inherit: TraitInherit)? & (where_cond: Where)? & (code: (TraitBlock | Semicolon)))]
    TraitDef {
        name: TokenRef,
        generic: NodeRef,
        inherit: NodeRef,
        where_cond: NodeRef,
        code: NodeRef,
    },

    #[rule($Type & (name: $Ident) & (generic: GenericDef)? & ($Set & set: Type)? & $Semicolon)]
    TypeDef {
        name: TokenRef,
        generic: NodeRef,
        set: NodeRef,
    },

    #[rule((attrs: AttrOuter)* & (kw_pub: PubConstruct)? & (value: (FnDefConstruct | TypeDef)))]
    TraitItem {
        attrs: Vec<NodeRef>,
        kw_pub: NodeRef,
        value: NodeRef,
    },

    #[rule($BraceOpen & ((items: TraitItem) | (inner_attrs: AttrInner))* & $BraceClose)]
    TraitBlock {
        items: Vec<NodeRef>,
        inner_attrs: Vec<NodeRef>,
    },

    // U
    #[rule($Unsafe)]
    Unsafe,

    #[rule($BraceOpen & (inner: UseStatementConstruct)*{$Comma} & $BraceClose)]
    UseBlock { inner: Vec<NodeRef> },

    #[rule($DoubleColon & block: UseBlock?)]
    UseStatementBlock { block: NodeRef },

    #[rule($As & (name: $Ident))]
    UseStatementAs { name: TokenRef },

    #[rule((path: Path) & (additional: UseStatementBlock | UseStatementAs)?)]
    UseStatementConstruct { path: NodeRef, additional: NodeRef },

    #[rule($Use & (st: UseStatementConstruct) & $Semicolon)]
    UseConstruct { st: NodeRef },

    #[rule($Const & (name: $Ident) & $Colon & (type_: Type) & $Set & (value: Value) & $Semicolon)]
    ConstDef {
        name: TokenRef,
        type_: NodeRef,
        value: NodeRef,
    },

    #[rule($Static & (name: $Ident) & $Colon & (type_: Type) & $Set & (value: Value) & $Semicolon)]
    StaticDef {
        name: TokenRef,
        type_: NodeRef,
        value: NodeRef,
    },

    // V
    #[rule((value: SingleVal)+{op: ($BinOp | $Refer | $Less | $Greater)})]
    Value {
        value: Vec<NodeRef>,
        op: Vec<TokenRef>,
    },
    
    #[rule(
        (prefix: ($Sub | $Add | $Refer | $Bang))?
        & (value: (ValueParenthesis | ValueIdent | String | Char | Number))
    )]
    SingleVal {
        prefix: TokenRef,
        value: NodeRef,
    },

    #[rule($Open & (value: Value) & $Close)]
    ValueParenthesis { value: NodeRef },

    #[rule(
        (
            (((prefix: $DoubleColon) | ((prefix: ($Crate | $USelf)) & $DoubleColon))
            & (path: $Ident)+{$DoubleColon}
            & ($DoubleColon & (generic: GenericUse) & $DoubleColon)?)
            | ((prefix: ($LSelf | $Ident)) & (
                (((path_t: $DoubleColon) & (path: $Ident))+
                & ($DoubleColon & (generic: GenericUse) & $DoubleColon & ((add_path: $Ident)+{$DoubleColon})?)?)
                | ((path_t: $Point) & (path: $Ident))+
            )?)
        )
    )]
    ValPath {
        prefix: TokenRef,
        path: Vec<TokenRef>,
        generic: NodeRef,
        // add_path: Vec<TokenRef>,
        path_t: Vec<TokenRef>,
    },

    #[rule($Less & (type_: Type) & $As & (as_type: Type) & $Greater
    & $DoubleColon & ((add_path: $Ident)+{$DoubleColon})?)]
    AsTrait {
        type_: NodeRef,
        as_type: NodeRef,
        add_path: Vec<TokenRef>,
    },

    #[rule((path: (ValPath | AsTrait)) & (val: Call)?)]
    ValueIdent {
        path: NodeRef,
        val: NodeRef,
    },

    #[rule($Open & (values: Value)*{$Comma} & $Close)]
    Call {
        values: Vec<NodeRef>,
    },

    // W
    #[rule($Where & (items: TypeForWhere)+{$Comma})]
    Where { items: Vec<NodeRef> },
    // X

    // Y

    // Z
}


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

trait T {}
trait T : R {}                done
impl S {}                     done
impl T for S {}
impl !T for S {}
fn f() {}
    fn f() -> S {}
    fn f(&self) {}
struct S(T);
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
    unsafe impl T for S {}

while x {}
loop {}
for x in collection {}
    collection.into_iter()
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
S<T = R>
    S<const N: u8 = 0>
    S<T = u8>
S<'_>
S<_>
S::<T>
trait T { type X<'a>; }
impl<T> S<T> {}
impl S<T> {}
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
r#"..."#
    fn f() -> ! {}
    fn f() -> Result<(), !> {}
    fn f(x: !) {}
_
    let _ = x;
    _ = x;
x;
*/