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
    syntax::{Node, NodeRef, SyntaxSession, SyntaxError},
};
use super::lexis::RustToken;

#[derive(Node, Debug, Clone)]
#[token(RustToken)]
#[error(SyntaxError)]
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
    | $Label
    | $Colon
    | $DoubleColon
    | $Dollar
    | $Semicolon
    | $BinOp
    | $Pipe
    | $Add
    | $Sub
    | $Set
    | $Refer
    | $Tilde
    | $At
    | $Backslash
    | $Bang
    | $Quest
    | $Hash
    | $HashBang
    | $Arrow
    | $SetOp
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

    #[rule((attrs: AttrOuter)* & (async_: $Async)? & (value: RootItemVal))]
    RootItem {
        attrs: Vec<NodeRef>,
        async_: TokenRef,
        value: NodeRef,
    },

    #[rule((mods: PubConstruct)? & (mods: (Extern | Unsafe))?
    & (value: (StructDefConstruct | AttrInner | EnumDefConstruct | UseConstruct | FnDefConstruct | TraitDef
    | ImplStatement | TypeDef | ModuleDef | TraitDef | ImplStatement | ConstDef | StaticDef | MacroRules)))]
    RootItemVal {
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
    #[rule((name: TypeNoRefer) & ($Colon & (_type: Type))?)]
    FnParameter { name: NodeRef, _type: NodeRef },

    /*
    #[rule((refer: Reference)? & $LSelf)]
    SelfUse { refer: NodeRef },
    */

    #[rule((attrs: AttrOuter)* & (value: (FnParameter/* | SelfUse*/)))]
    FnParameterConstruct { attrs: Vec<NodeRef>, value: NodeRef },

    #[rule($Arrow & (_type: Type))]
    FnTyping { _type: NodeRef },

    #[rule($Fn & (name: $Ident) & (generic: GenericDef)?
    & ($Open & (params: FnParameterConstruct)*{$Comma} & $Comma? & $Close) & (_type: FnTyping)?
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

    #[rule((lifetime: $Label) & ($Colon & (val: $Label)+{$Comma})?)]
    LifetimeForGeneric {
        lifetime: TokenRef,
        val: Vec<TokenRef>,
    },

    #[rule($Less & (items: (GenericUseType | Lifetime))*{$Comma} & $Comma? & $Greater)]
    GenericUse { items: Vec<NodeRef> },

    #[rule((name: Type) & ($Set & (_type: Type))?)]
    GenericUseType {
        name: NodeRef,
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

    #[rule(value: $Label)]
    Lifetime { value: TokenRef },

    // M
    #[rule($BraceOpen & (items: RootItem)* & $BraceClose)]
    ModuleBlock { items: Vec<NodeRef> },

    #[rule($Mod & (name: $Ident) & (code: (Semicolon | ModuleBlock)))]
    ModuleDef { name: TokenRef, code: NodeRef },

    #[rule((((prefix: $DoubleColon) | ((prefix: ($Crate)) & $DoubleColon)
        | ((path: $Super) & $DoubleColon)+)? & (path: $Ident)+{$DoubleColon})
        | ((path: ($USelf | $LSelf)) & ($DoubleColon & (path: $Ident))*))]
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

    #[rule(val: $Refer)]
    Star { val: TokenRef },

    #[rule(((prefix: $DoubleColon) | ((prefix: ($Crate | $LSelf | $USelf)) & $DoubleColon)
    | ((prefix: $Super) & $DoubleColon)+)? & (path: (Ident | UseBlock | Star))+{$DoubleColon}
    & (additional: UseStatementAs)?)]
    UseStatementConstruct {
        prefix: Vec<TokenRef>,
        path: Vec<NodeRef>,
        additional: NodeRef,
    },

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
    #[rule((value: SingleVal)+{op: ($BinOp | $Refer | $Less | $Greater)} | (clojure: Clojure))]
    Value {
        value: Vec<NodeRef>,
        op: Vec<TokenRef>,
        clojure: NodeRef,
    },

    #[rule((mods: $Move)? & $Pipe & (params: FnParameter)* & $Pipe & (action: CodeBlockItem))]
    Clojure {
        mods: TokenRef,
        params: Vec<NodeRef>,
        action: NodeRef,
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

    #[rule($BracketOpen & ((value: Value) & ((($Comma & (values: Value))*
        & $Comma?) | ($Semicolon & (len: Value)))?)? & $BracketClose)]
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

    #[rule($BraceOpen & (items: ConstructItem)*{$Comma} & $Comma? & ($Range & (default: Value)?)? & $BraceClose)]
    Constructor {
        items: Vec<NodeRef>,
        default: NodeRef,
    },

    #[rule($Open & (values: Value)*{$Comma} & $Comma? & $Close)]
    Call { values: Vec<NodeRef> },

    #[rule($Point & (name: ($Ident | $Await)) & (call: Call)?)]
    Method { name: TokenRef, call: NodeRef },

    #[rule($BraceOpen & (actions: CodeBlockItem)* & $BraceClose)]
    CodeBlock { actions: Vec<NodeRef> },

    #[rule((attrs: AttrOuter)* & (async_: $Async)? &
           (action: (Action | Let | Continue | Break | Return | Label | RootItemVal)))]
    CodeBlockItem {
        attrs: Vec<NodeRef>,
        async_: TokenRef,
        action: NodeRef,
    },

    #[rule((lhs: Value) & (((op: ($SetOp | $Set)) & (rhs: Value) & $Semicolon?) | (end: $Semicolon))?)]
    Action {
        lhs: NodeRef,
        op: TokenRef,
        rhs: NodeRef,
        end: TokenRef,
    },

    #[index(12)]
    #[rule((value: SingleValNoConstruct)+{op: ($BinOp | $Refer | $Less | $Greater)})]
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
    SingleValNoConstruct {
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

    #[rule((label: Lifetime) & $Colon & (val: (While | For | CodeBlock | Loop)))]
    Label { label: NodeRef, val: NodeRef },

    // #[parser(parse_match)]
    // #[leftmost($Match)]
    #[rule($Match & (value: ValNoConstruct) & $BraceOpen & (items: MatchItem)* & $BraceClose)]
    Match { value: NodeRef, /* constructor: Option<NodeRef>,*/ items: Vec<NodeRef> },

    #[index(10)]
    #[rule((val: Value) & $MatchArrow & (ret: (Action | Let | Continue | Break | Return | Label)) & $Comma?)]
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
/*
impl RustNode {
    fn skip<'code>(session: &mut impl SyntaxSession<'code, Node = RustNode>) {
        loop {
            match session.token(0) {
                Some(RustToken::Whitespace) | Some(RustToken::NewLine) => _ = session.advance(),
                _ => break,
            }
        }
    }
    fn parse_MatchItem<'code>(
        session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
            'code,
            Node = RustNode,
        >,
    ) -> RustNode {
        let mut state = 1usize;
        let mut start = ::lady_deirdre::lexis::TokenCursor::site_ref(
            session,
            0,
        );
        let mut capture_ret = ::lady_deirdre::syntax::NodeRef::nil();
        let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
        'outer: loop {
            match (
                state,
                ::lady_deirdre::lexis::TokenCursor::token(session, 0),
            ) {
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Range { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    state = 2usize;
                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        60usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::MatchArrow { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingRule {
                            span: start..start,
                            context: "MatchItem",
                            rule: "Value",
                        }),
                    );
                    state = 3usize;
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (1usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(RustToken::USelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Char { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ref { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Loop { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BracketOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Add { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Less { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::If { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Sub { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::String { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Refer { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Range { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Crate { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::DoubleColon { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Bang { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ident { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::While { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::For { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Number { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Super { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Mut { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Match { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Open { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::LSelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Value"]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::MatchArrow { .. }),
                ) => {
                    state = 3usize;
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Range { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Break { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        70usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Continue { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        68usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Label { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        92usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Let { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        48usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Return { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "MatchItem",
                            token: "MatchArrow",
                        }),
                    );
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        18usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (2usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(
                                RustToken::MatchArrow { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchArrow"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchArrow"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Range { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        13usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Break { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        70usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Continue { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        68usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Label { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        92usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Let { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        48usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Return { .. }),
                ) => {
                    state = 4usize;
                    capture_ret = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        18usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (3usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(RustToken::LSelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Char { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ref { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Loop { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BracketOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Add { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Less { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::If { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Sub { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::String { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Refer { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Range { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Crate { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::DoubleColon { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Bang { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ident { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::While { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::For { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Number { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Super { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Mut { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Match { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::USelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Open { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Break { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Continue { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Label { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Let { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Return { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "MatchItem",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 6usize],
                                        >>::from([
                                            "Action",
                                            "Break",
                                            "Continue",
                                            "Label",
                                            "Let",
                                            "Return",
                                        ]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Comma { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    break;
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (4usize, _) => {
                    break;
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("Unknown state.")
                        ),
                    );
                }
            }
        }
        RustNode::MatchItem {
            val: capture_val,
            ret: capture_ret,
        }
    }
    
    
    
    fn parse_Match<'code>(
        session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
            'code,
            Node = RustNode,
        >,
    ) -> RustNode {
        let mut state = 1usize;
        let mut start = ::lady_deirdre::lexis::TokenCursor::site_ref(
            session,
            0,
        );
        let mut capture_value = ::lady_deirdre::syntax::NodeRef::nil();
        let mut capture_items = ::std::vec::Vec::<
            ::lady_deirdre::syntax::NodeRef,
        >::with_capacity(1);
        'outer: loop {
            match (
                state,
                ::lady_deirdre::lexis::TokenCursor::token(session, 0),
            ) {
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    state = 2usize;
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    1usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "Match",
                        }),
                    );
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (1usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(RustToken::Match { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Match"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceClose { .. },
                            ) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Match"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break 'outer;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["Match"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    state = 3usize;
                    capture_value = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        12usize,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    2usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (2usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(RustToken::USelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Char { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ref { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Loop { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BracketOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Add { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Less { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::If { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Sub { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::String { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Refer { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Crate { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::DoubleColon { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Bang { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ident { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::While { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::LSelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::For { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Super { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Mut { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Open { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Match { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Number { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceClose { .. },
                            ) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break 'outer;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 0usize],
                                        >>::from([]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["ValNoConstruct"]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    state = 4usize;
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::BraceClose { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    break;
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Range { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    3usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        <SyntaxError as ::std::convert::From<
                            ::lady_deirdre::syntax::SyntaxError,
                        >>::from(::lady_deirdre::syntax::SyntaxError::MissingToken {
                            span: start..start,
                            context: "Match",
                            token: "BraceOpen",
                        }),
                    );
                    state = 4usize;
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (3usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(
                                RustToken::BraceOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceOpen"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceClose { .. },
                            ) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceOpen"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break 'outer;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceOpen"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<[&'static str; 0usize]>>::from([]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::BraceClose { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                    break;
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::NewLine { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Whitespace { .. }),
                ) => {
                    let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                        session,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Comment { .. }),
                ) => {
                    let _ = ::lady_deirdre::syntax::SyntaxSession::descend(
                        session,
                        8usize,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::LSelf { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Char { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Ref { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Loop { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::BracketOpen { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Add { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Less { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::If { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Sub { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::String { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::BraceOpen { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Refer { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Range { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Crate { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::DoubleColon { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Bang { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Ident { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::While { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::For { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Number { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Super { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Mut { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Match { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::USelf { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (
                    4usize,
                    ::std::option::Option::Some(RustToken::Open { .. }),
                ) => {
                    ::std::vec::Vec::push(
                        &mut capture_items,
                        ::lady_deirdre::syntax::SyntaxSession::descend(
                            session,
                            10usize,
                        ),
                    );
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                }
                (4usize, _) => {
                    start = ::lady_deirdre::lexis::TokenCursor::site_ref(
                        session,
                        0,
                    );
                    let mut end = start;
                    loop {
                        match ::lady_deirdre::lexis::TokenCursor::token(
                            session,
                            0,
                        ) {
                            ::std::option::Option::Some(
                                RustToken::BraceClose { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::LSelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Char { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ref { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Loop { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BracketOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Add { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Less { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::If { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Sub { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::String { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceOpen { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Refer { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Range { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Crate { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::DoubleColon { .. },
                            ) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Bang { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Ident { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::While { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::For { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Number { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Super { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Mut { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Match { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::USelf { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(RustToken::Open { .. }) => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break;
                            }
                            ::std::option::Option::Some(
                                RustToken::BraceClose { .. },
                            ) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break 'outer;
                            }
                            ::std::option::Option::Some(_) => {
                                let _ = ::lady_deirdre::lexis::TokenCursor::advance(
                                    session,
                                );
                                end = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                            }
                            ::std::option::Option::None => {
                                let _ = ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    <SyntaxError as ::std::convert::From<
                                        ::lady_deirdre::syntax::SyntaxError,
                                    >>::from(::lady_deirdre::syntax::SyntaxError::Mismatch {
                                        span: start..end,
                                        context: "Match",
                                        expected_tokens: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["BraceClose"]),
                                        expected_rules: <::std::vec::Vec<
                                            &'static str,
                                        > as ::std::convert::From<
                                            [&'static str; 1usize],
                                        >>::from(["MatchItem"]),
                                    }),
                                );
                                break 'outer;
                            }
                        }
                        skip(session);
                    }
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("Unknown state.")
                        ),
                    );
                }
            }
        }
        RustNode::Match {
            value: capture_value,
            items: capture_items,
        }
    }
}
*/
// $Match & (value: ValNoConstruct) & $BraceOpen & (items: MatchItem)* & $BraceClose
