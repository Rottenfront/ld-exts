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
#[define(ATTR_ITEM = (
    $As
    | $Async
    | $Await
    | $Break
    | $Const
    | $Continue
    | $Crate
    | $Do
    | $Dyn
    | $Else
    | $Enum
    | $Extern
    | $False
    | $Fn
    | $For
    | $If
    | $Impl
    | $In
    | $Let
    | $Loop
    | $Macro
    | $Match
    | $Mod
    | $Move
    | $Mut
    | $Pub
    | $Ref
    | $Return
    | $LSelf
    | $USelf
    | $Static
    | $Struct
    | $Super
    | $Trait
    | $True
    | $Try
    | $Type
    | $Union
    | $Unsafe
    | $Use
    | $Where
    | $While
    | $Yield
    | $BasicType
    | $Number
    | $Less
    | $Greater
    | $Comma
    | $Point
    | $Range
    | $Char
    | $Lable
    | $Colon
    | $DoubleColon
    | $Dollar
    | $Semicolon
    | $BinOp
    | $Add
    | $Sub
    | $Set
    | $Refer
    | $Tilda
    | $At
    | $Backslash
    | $Bang
    | $Quest
    | $Hash
    | $HashBang
    | $Arrow
    | $SetOp
    | $Escape
    | $Ident
    | $String
))]

// #[define(ATTR_ITEM = ($Ident | $LSelf | $USelf | $Super | $DoubleColon))]
#[define(PATH_ITEM = ($Ident | $LSelf | $USelf | $Super))]
pub enum RustNode {
    // Root
    #[root]
    #[rule((items: RootItem)*)]
    Root { items: Vec<NodeRef> },

    #[rule($Unsafe)]
    Unsafe,
    #[rule($Where & (items: TypeForWhere)+{$Comma})]
    Where { items: Vec<NodeRef> },

    #[rule((attrs: AttrOuter)* & (mods: PubConstruct)? & (mods: (Extern | Unsafe))?
    & (value: (StructDefConstruct | AttrInner | EnumDefConstruct | UseConstruct | FnDefConstruct | TraitDef
    | ImplStatement | TypeDef | ModuleDef | TraitDef | ImplStatement | ConstDef | StaticDef | MacroRules)))]
    RootItem {
        attrs: Vec<NodeRef>,
        mods: Vec<NodeRef>,
        value: NodeRef,
    },

    #[rule($MacroRules & (name: $Ident) & $BraceOpen & (items: MacroRulesItem)* & $BraceClose)]
    MacroRules {
        name: TokenRef,
        items: Vec<NodeRef>,
    },

    #[rule((template: MacroUse) & $MatchArrow & (code: MacroUse) & $Semicolon)]
    MacroRulesItem {
        template: NodeRef,
        code: NodeRef,
    },

    #[rule($HashBang & $BracketOpen & (name: Path) & (arg: MacroUse)? & $BracketClose)]
    AttrInner { name: NodeRef, arg: NodeRef },

    #[rule($Hash & $BracketOpen & (name: Path) & (arg: MacroUse)? & $BracketClose)]
    AttrOuter { name: NodeRef, arg: NodeRef },

    #[comment]
    #[rule(value: $Comment+)]
    Comment { value: Vec<TokenRef> },

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

    #[rule($BraceOpen & (items: StructItem)*{$Comma} & $Comma? & $BraceClose)]
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
    #[rule($Less & (items: (TypeForGeneric | LifetimeForGeneric))+{$Comma} & $Comma? & $Greater)]
    GenericDef { items: Vec<NodeRef> },

    #[rule((lifetime: $Lable) & ($Colon & (val: $Lable)+{$Comma})?)]
    LifetimeForGeneric {
        lifetime: TokenRef,
        val: Vec<TokenRef>,
    },

    #[rule($Less & (items: (GenericUseType | Lifetime))*{$Comma} & $Comma? & $Greater)]
    GenericUse { items: Vec<NodeRef> },

    #[rule((path: Path) & ($Set & (_type: Type))?)]
    GenericUseType {
        path: NodeRef,
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

    #[rule(value: $Lable)]
    Lifetime { value: TokenRef },

    // M
    #[rule($BraceOpen & (items: RootItem)* & $BraceClose)]
    ModuleBlock { items: Vec<NodeRef> },

    #[rule($Mod & (name: $Ident) & (code: (Semicolon | ModuleBlock)))]
    ModuleDef { name: TokenRef, code: NodeRef },

    #[rule((((prefix: $DoubleColon) | ((prefix: ($Crate | $LSelf)) & $DoubleColon)
        | ((path: $Super) & $DoubleColon)+)? & (path: $Ident)+{$DoubleColon})
        | ((path: $USelf) & ($DoubleColon & (path: $Ident))*))]
    Path {
        prefix: TokenRef,
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
    #[rule(((refer: $Refer)+ & (lf: Lifetime)? & (ref_: $Ref)? & (mutability: $Mut)?)
        | ((mutability: $Mut) & (ref_: $Ref)?) | (ref_: $Ref))]
    Reference {
        refer: Vec<TokenRef>,
        lf: NodeRef,
        mutability: TokenRef,
        ref_: TokenRef,
    },

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

    #[rule((refer: Reference)? & ((value: TypeNoRefer) | ($Open & (tuple: Type)* & $Close)
    | ($BracketOpen & (arr_type: Type) & ($Semicolon & (len: Value))? & $BracketClose)))]
    Type {
        refer: NodeRef,
        value: NodeRef,
        tuple: Vec<NodeRef>,
        arr_type: NodeRef,
        len: NodeRef,
    },

    #[rule((modifier: ($Impl | $Dyn | $Quest))? & (
            ((value: Path) & (generic: GenericUse)? & ($Open & (tuple: Type*) & $Close)?)
            | (value: BasicType)
    ))]
    TypeNoRefer {
        modifier: TokenRef,
        value: NodeRef,
        tuple: Vec<NodeRef>,
        generic: NodeRef,
    },

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

    #[rule(value: $BasicType)]
    BasicType { value: TokenRef },

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

    #[rule($BraceOpen & (inner: UseStatementConstruct)*{$Comma} & $Comma? & $BraceClose)]
    UseBlock { inner: Vec<NodeRef> },

    #[rule($As & (name: $Ident))]
    UseStatementAs { name: TokenRef },

    #[rule($Refer)]
    Star,

    #[rule(((prefix: $DoubleColon) | ((prefix: ($Crate | $LSelf | $USelf)) & $DoubleColon)
    | ((prefix: $Super) & $DoubleColon)+)? & (path: (Ident | UseBlock | Star))+{$DoubleColon} & (additional: UseStatementAs)?)]
    UseStatementConstruct { prefix: Vec<TokenRef>, path: Vec<NodeRef>, additional: NodeRef },

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

    #[rule(value: $Number)]
    Number { value: TokenRef },
    #[rule(value: $String)]
    String { value: TokenRef },
    #[rule(value: $Char)]
    Char { value: TokenRef },
    #[rule((value: SingleVal)+{op: ($BinOp | $Refer | $Less | $Greater)})]
    Value {
        value: Vec<NodeRef>,
        op: Vec<TokenRef>,
    },

    #[rule(((prefix: (Reference | UnOp)*)
    & ((value: (ValueParenthesis | ValueIdent | String | Char | Number | ValueBrackets
        | CodeBlock | Match | If | For | While | Loop))
    & ((range: $Range) & (next: Value))?
    & (methods: (Method | Index))*) | ((range: $Range) & (next: Value)?)) & ($As & (as_type: Type))*)]
    SingleVal {
        prefix: Vec<NodeRef>,
        value: NodeRef,
        range: TokenRef,
        next: NodeRef,
        methods: Vec<NodeRef>,
        as_type: Vec<NodeRef>,
    },

    #[rule(val: ($Sub | $Add | $Bang)+)]
    UnOp { val: Vec<TokenRef> },

    #[rule($BracketOpen & (val: Value) & $BracketClose)]
    Index { val: NodeRef },

    #[rule($Open & (values: Value)*{$Comma} & $Comma? & $Close)]
    ValueParenthesis { values: Vec<NodeRef> },

    #[rule($BracketOpen & (value: Value) & (($Comma & (values: Value))*
        | ($Semicolon & (len: Value))) & $BracketClose)]
    ValueBrackets {
        value: NodeRef,
        len: NodeRef,
        values: Vec<NodeRef>,
    },

    #[rule(val: $Ident)]
    Ident { val: TokenRef },

    #[rule(
    (((prefix: $DoubleColon) | ((prefix: ($Crate | $USelf)) & $DoubleColon)
    | ((prefix: $Super) & $DoubleColon)+)
        & (path: (Ident | GenericUse))+{$DoubleColon})
    | ((prefix: ($LSelf | $Ident)) & (
        ((path_t: $DoubleColon) & (path: (Ident | GenericUse)))+
        | ((path_t: $Point) & (path: Ident))+
    )?))]
    ValPath {
        prefix: Vec<TokenRef>,
        path: Vec<NodeRef>,
        path_t: Vec<TokenRef>,
    },

    #[rule($Less & (type_: Type) & $As & (as_type: Type) & $Greater
    & $DoubleColon & ((add_path: $Ident)+{$DoubleColon})?)]
    AsTrait {
        type_: NodeRef,
        as_type: NodeRef,
        add_path: Vec<TokenRef>,
    },

    #[rule(val: ATTR_ITEM+)]
    AnyTokens { val: Vec<TokenRef> },

    #[rule(((brace_type: $Open) & (tokens: (MacroUse | AnyTokens))* & $Close)
        | ((brace_type: $BraceOpen) & (tokens: (MacroUse | AnyTokens))* & $BraceClose)
        | ((brace_type: $BracketOpen) & (tokens: (MacroUse | AnyTokens))* & $BracketClose))]
    MacroUse {
        brace_type: TokenRef,
        tokens: Vec<NodeRef>,
    },

    #[rule((path: (ValPath | AsTrait)) & ((val: Call) | ($Bang & (macro_: MacroUse))
        | (val: Constructor))?)]
    ValueIdent {
        path: NodeRef,
        val: NodeRef,
        macro_: NodeRef,
    },

    #[rule((name: $Ident) & ($Colon & (val: Value))?)]
    ConstructItem { name: TokenRef, val: NodeRef },

    #[rule($BraceOpen & (items: ConstructItem)*{$Comma} & ($Range & (default: Value))? & $BraceClose)]
    Constructor {
        items: Vec<NodeRef>,
        default: NodeRef,
    },

    #[rule($Open & (values: Value)*{$Comma} & $Comma? & $Close)]
    Call { values: Vec<NodeRef> },

    #[rule($Point & (name: $Ident) & (call: Call)?)]
    Method { name: TokenRef, call: NodeRef },

    #[rule($BraceOpen & (actions: (Action | Let | Continue | Break | Return | Lable))* & $BraceClose)]
    CodeBlock { actions: Vec<NodeRef> },

    #[rule((lhs: Value) & (((op: ($SetOp | $Set)) & (rhs: Value) & $Semicolon?) | (end: $Semicolon))?)]
    Action {
        lhs: NodeRef,
        op: TokenRef,
        rhs: NodeRef,
        end: TokenRef,
    },

    #[rule((value: SingleValNoCostruct)+{op: ($BinOp | $Refer | $Less | $Greater)})]
    ValNoConstruct {
        value: Vec<NodeRef>,
        op: Vec<TokenRef>,
    },

    #[rule((path: (ValPath | AsTrait)) & ((val: Call) | ($Bang & (macro_: MacroUse)))?)]
    ValNoConstructIdent {
        path: NodeRef,
        val: NodeRef,
        macro_: NodeRef,
    },

    #[rule((prefix: (Reference | UnOp)*)
    & (value: (ValueParenthesis | ValNoConstructIdent | String | Char | Number | ValueBrackets
        | CodeBlock | Match | If | For | While | Loop))
    & ((range: $Range) & (next: ValNoConstruct))?
    & (methods: (Method | Index | Number))* & ($As & (as_type: Type))*)]
    SingleValNoCostruct {
        prefix: Vec<NodeRef>,
        value: NodeRef,
        range: TokenRef,
        next: NodeRef,
        methods: Vec<NodeRef>,
        as_type: Vec<NodeRef>,
    },

    #[rule((value: Ident) | ((refer: Reference) & (value: TupleFor))
        | ($Open & (values: TupleFor)*{$Comma} & $Close))]
    TupleFor {
        refer: NodeRef,
        value: NodeRef,
        values: Vec<NodeRef>,
    },

    #[rule((lable: Lifetime) & $Colon & (val: (While | For | CodeBlock | Loop)))]
    Lable { lable: NodeRef, val: NodeRef },

    #[rule($Match & (val: ValNoConstruct) & $BraceOpen & (items: MatchItem*) & $BraceClose)]
    Match { val: NodeRef, items: Vec<NodeRef> },

    #[rule((val: Value) & $MatchArrow & (ret: (Action | Let | Continue | Break | Return | Lable)) & $Comma?)]
    MatchItem { val: NodeRef, ret: NodeRef },

    #[rule($For & (val: TupleFor) & $In & (arr: ValNoConstruct) & (block: CodeBlock))]
    For {
        val: NodeRef,
        arr: NodeRef,
        block: NodeRef,
    },

    #[rule($While & (cond: (ValNoConstruct | Let) & (block: CodeBlock)))]
    While { cond: NodeRef, block: NodeRef },

    #[rule($Loop & (block: CodeBlock))]
    Loop { block: NodeRef },

    #[rule($If & (cond: (ValNoConstruct | Let)) & (block: CodeBlock) & ($Else & (next: (CodeBlock | If)))?)]
    If {
        cond: NodeRef,
        block: NodeRef,
        next: NodeRef,
    },

    #[rule($Continue & (label: Lifetime)? & (val: Value)? & $Semicolon?)]
    Continue { label: NodeRef, val: NodeRef },

    #[rule($Break & (label: Lifetime)? & (val: Value)? & $Semicolon?)]
    Break { label: NodeRef, val: NodeRef },

    #[rule($Return & (val: Value)? & $Semicolon?)]
    Return { val: NodeRef },

    #[rule(((prefix: $DoubleColon) | ((prefix: ($Crate | $LSelf | $USelf)) & $DoubleColon)
        | ((path: $Super) & $DoubleColon)+)
        & (path: $Ident)+{$DoubleColon})]
    PathNoIdent {
        prefix: TokenRef,
        path: Vec<TokenRef>,
    },

    #[rule($Let & (((path: PathNoIdent) & ($Open & (values: TupleFor)*{$Comma} & $Close)?)
    | (names: TupleFor)) & ($Colon & (type_: Type))? & ($Set & (val: Value))? & $Semicolon?)]
    Let {
        path: NodeRef,
        names: NodeRef,
        values: Vec<NodeRef>,
        type_: NodeRef,
        val: NodeRef,
    },
}
