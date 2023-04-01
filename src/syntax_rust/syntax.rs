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
// FITNESS FOR A PARTICULAR PURPOSE AN D NONINFRINGEMENT. IN NO EVENT SHALL THE  //
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        //
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, //
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE //
// SOFTWARE.                                                                     //
// ///////////////////////////////////////////////////////////////////////////// //

use lady_deirdre::{
    lexis::TokenRef,
    syntax::{Node, NodeRef},
};
use std::vec::Vec;

#[derive(Node, Debug, Clone)]
#[token(super::lexis::RustToken)]
#[error(lady_deirdre::syntax::SyntaxError)]
#[skip($Whitespace | $NewLine)]
/*#[define(ANY = ($As | $Async | $Await | $Break | $Const | $Continue
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
| $SingleComment | $MultilineCommentOpen | $Mismatch | $Bang | $QuestMark))]*/
#[define(ANY = ($Identifier | $Comma | $Point | $Range | $Apostrophe))]
/*#[define(ATTR_ITEM = ($As | $Async | $Await | $Break | $Const | $Continue
| $Crate | $Do | $Dyn | $Else | $Enum | $Extern | $False
| $Fn | $For | $If | $Impl | $In | $Let | $Loop | $Macro
| $Match | $Mod | $Move | $Mut | $Pub | $Ref | $Return
| $LSelf | $USelf | $Static | $Struct | $Super | $Trait | $True
| $Try | $Type | $Union | $Unsafe | $Use | $Where | $While
| $Yield | $MacroRules | $NumType | $BasicType | $BinNumber | $OctNumber | $DecNumber | $HexNumber
| $String | $Identifier | $Underscore | $Comma | $Point | $Range | $Apostrophe | $AsciiChar | $DoubleColon | $Colon
| $Dollar | $Semicolon | $Operator | $Add | $Assign | $Amp | $Star | $Slash | $Tilda | $At | $Backslash | $Escape
| $Bang | $QuestMark | $Hash | $HashBang | $Arrow | $AssignWithOperation | $Mismatch))]*/
#[define(ATTR_ITEM = ($Identifier))]
#[define(PATH_ITEM = ($Identifier | $LSelf | $USelf | $Super))]
pub enum RustNode {
    // Root
    #[root]
    #[rule((items: RootItem)*)]
    Root { items: Vec<NodeRef> },

    #[rule((attrs: AttrOuter)* & (mods: PubConstruct)? & (mods: (Extern | Unsafe))?
    & (value: (StructDefConstruct | AttrInner | EnumDefConstruct | UseConstruct | FnDefConstruct | TraitDef
    | ImplStatement | TypeDef | ModuleDef | TraitDef | ImplStatement)))]
    RootItem {
        attrs: Vec<NodeRef>,
        mods: Vec<NodeRef>,
        value: NodeRef,
    },

    // A
    #[rule($HashBang & $BracketOpen & (items: (($ParenthesisOpen & ATTR_ITEM* & $ParenthesisClose)
    | ($BracketOpen & ATTR_ITEM* & $BracketClose) | ($AngleBracketOpen & ATTR_ITEM* & $AngleBracketClose)
    | ($BraceOpen & ATTR_ITEM* & $BraceClose) | ATTR_ITEM))* & $BracketClose)]
    AttrInner { items: Vec<TokenRef> },

    #[rule($Hash & $BracketOpen & (items: (ATTR_ITEM | ($ParenthesisOpen & ATTR_ITEM* & $ParenthesisClose)
    | ($BracketOpen & ATTR_ITEM* & $BracketClose) | ($BraceOpen & ATTR_ITEM* & $BraceClose)))*
    & $BracketClose)]
    AttrOuter { items: Vec<TokenRef> },

    // B
    #[rule(value: ($BasicType | $NumType))]
    BasicType { value: TokenRef },

    // C
    // #[rule(value: (($AsciiChar | $Apostrophe) & ANY? & $Apostrophe))]
    // Char {
    //     value: Vec<TokenRef>,
    // },
    #[rule($BraceOpen & $BraceClose)]
    CodeBlock,

    #[comment]
    #[rule($SingleComment & (value: (ANY | $MultilineCommentClose | ($Backslash & $NewLine))*))]
    CommentSingle { value: Vec<TokenRef> },

    #[comment]
    #[rule($MultilineCommentOpen & (value: (ANY | $NewLine)*) & $MultilineCommentClose)]
    CommentMultiline { value: Vec<TokenRef> },

    // D

    // E
    #[rule((attrs: AttrOuter)* & (name: $Identifier) & (additional: (EnumItemAnonFields | EnumItemFields))?
    & ($Assign & (num: Number))?)]
    EnumItem {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        additional: NodeRef,
        num: NodeRef,
    },

    #[rule($ParenthesisOpen & (items: Type)*{$Comma} & $ParenthesisClose)]
    EnumItemAnonFields { items: Vec<NodeRef> },

    #[rule($BraceOpen & (items: StructItem)*{$Comma} & $BraceClose)]
    EnumItemFields { items: Vec<NodeRef> },

    #[rule($Enum & (name: $Identifier) & (generic: GenericDef)? & (where_cond: Where)?
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
    #[rule((name: $Identifier) & $Colon & (_type: Type))]
    FnParameter {
        name: TokenRef,
        _type: NodeRef,
    },
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

    #[rule($Arrow & (impl_kw: $Impl)? & (_type: Type))]
    FnTyping { impl_kw: TokenRef, _type: NodeRef },

    #[rule($Fn & (name: $Identifier) & (generic: GenericDef)?
    & ($ParenthesisOpen & (params: FnParameterConstruct)*{$Comma} & $ParenthesisClose) & (_type: FnTyping)?
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
    #[rule($AngleBracketOpen & (items: TypeForGeneric)+{$Comma} & $AngleBracketOpen)]
    GenericDef { items: Vec<NodeRef> },

    #[rule($AngleBracketOpen & (items: GenericUseType)+{$Comma} & $AngleBracketOpen)]
    GenericUse { items: Vec<NodeRef> },

    #[rule(((is_absolute: $DoubleColon)? & (path: ($LSelf | $USelf | $Crate | $Super)) & $DoubleColon
    & ((path: PATH_ITEM) & $DoubleColon)* & (path: PATH_ITEM)) | (((path: $Identifier) & $DoubleColon)* & (path: $Identifier) & $Assign & (_type: Type)))]
    GenericUseType {
        is_absolute: TokenRef,
        path: Vec<TokenRef>,
        _type: NodeRef,
    },

    // H

    // I
    #[rule($Impl & (_type: Path) & ($For & (for_t: Path))? & (code: TraitBlock))]
    ImplStatement {
        _type: NodeRef,
        for_t: NodeRef,
        code: NodeRef,
    },

    // J

    // K

    // L
    #[rule($Apostrophe & (value: $Identifier))]
    Lifetime { value: TokenRef },

    // M
    #[rule($BraceOpen & (items: RootItem)* & $BraceClose)]
    ModuleBlock { items: Vec<NodeRef> },

    #[rule($Mod & (name: $Identifier) & (code: (Semicolon | ModuleBlock)))]
    ModuleDef { name: TokenRef, code: NodeRef },

    // N
    #[rule(value: ($BinNumber | $OctNumber | $DecNumber | $HexNumber))]
    Number { value: TokenRef },

    // O

    // P
    #[rule((prefix: (($Crate | $Super+{$DoubleColon} | $LSelf | $USelf)? & $DoubleColon))?
    & (path: $Identifier)+{$DoubleColon})]
    Path {
        prefix: Vec<TokenRef>,
        path: Vec<TokenRef>,
    },

    #[rule($Pub & ($ParenthesisOpen & (pub_for: PubFor | PubIn) & $ParenthesisClose)?)]
    PubConstruct { pub_for: NodeRef },

    #[rule(path: ($Super | $Crate))]
    PubFor { path: TokenRef },

    #[rule($In & (path: Path))]
    PubIn { path: NodeRef },

    // Q

    // R
    #[rule((refer: ($Amp | $Star)) & (lf: Lifetime)? & (mutability: $Mut)?)]
    Reference {
        refer: TokenRef,
        lf: NodeRef,
        mutability: TokenRef,
    },

    // S
    #[rule(value: $String)]
    String { value: TokenRef },

    #[rule($Semicolon)]
    Semicolon,

    #[rule((attrs: AttrOuter)* & (name: $Identifier) & $Colon & (_type: Type))]
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

    #[rule(($ParenthesisOpen & (items: Type)*{$Comma} & $ParenthesisClose)? & $Semicolon)]
    ShortStructDefStatement { items: Vec<NodeRef> },

    #[rule($Struct & (name: $Identifier) & (generic: GenericDef)?
    & (where_cond: Where)? & (value: (ShortStructDefStatement | StructDefStatement)))]
    StructDefConstruct {
        generic: NodeRef,
        where_cond: NodeRef,
        name: TokenRef,
        value: NodeRef,
    },

    // T
    // #[rule($True)]
    // True,
    #[rule((refer: Reference)? & (value: (BasicType | UseType)) & (generic: GenericUse)?)]
    Type {
        refer: NodeRef,
        value: NodeRef,
        generic: NodeRef,
    },

    #[rule((value: (BasicType | UseType)) & (generic: GenericUse)?)]
    TypeNoRefer { value: NodeRef, generic: NodeRef },

    #[rule((attrs: AttrOuter)* & (name: $Identifier) & ($Colon & ((value: (TypeNoRefer | Lifetime)) & $Add)* & (value: (TypeNoRefer | Lifetime)))?)]
    TypeForGeneric {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        value: Vec<NodeRef>,
    },

    #[rule((attrs: AttrOuter)* & (name: Path) & $Colon & ((value: (TypeNoRefer | Lifetime)) & $Add)* & (value: (TypeNoRefer | Lifetime)))]
    TypeForWhere {
        attrs: Vec<NodeRef>,
        name: NodeRef,
        value: Vec<NodeRef>,
    },

    #[rule($Colon & ((traits: TypeNoRefer) & $Add)* & (traits: TypeNoRefer))]
    TraitInherit { traits: Vec<NodeRef> },

    #[rule($Trait & (name: $Identifier) & (generic: GenericDef)?
        & (inherit: TraitInherit)? & (where_cond: Where)? & (code: (TraitBlock | Semicolon)))]
    TraitDef {
        name: TokenRef,
        generic: NodeRef,
        inherit: NodeRef,
        where_cond: NodeRef,
        code: NodeRef,
    },

    #[rule($Type & (name: $Identifier) & (generic: GenericDef)? & ($Assign & set: Type)? & $Semicolon)]
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

    #[rule(path: Path)]
    UseType { path: NodeRef },

    #[rule($BraceOpen & (inner: UseStatementConstruct)*{$Comma} & $BraceClose)]
    UseBlock { inner: Vec<NodeRef> },

    #[rule($DoubleColon & block: UseBlock?)]
    UseStatementBlock { block: NodeRef },

    #[rule($As & (name: $Identifier))]
    UseStatementAs { name: TokenRef },

    #[rule((path: Path) & (additional: UseStatementBlock | UseStatementAs)?)]
    UseStatementConstruct { path: NodeRef, additional: NodeRef },

    #[rule($Use & (st: UseStatementConstruct) & $Semicolon)]
    UseConstruct { st: NodeRef },

    // V

    // W
    #[rule($Where & (items: TypeForWhere)+{$Comma})]
    Where { items: Vec<NodeRef> },
    // X

    // Y

    // Z
}
