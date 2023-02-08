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
#[define(ARRAY_ITEM = Preprocessor)]
#[define(ANY = $BasicType)]
pub enum RustNode {
    // Root
    #[root]
    #[rule(items: RootItem*)]
    Root { items: Vec<NodeRef> },

    #[rule((attrs: AttrOuter)* & (mods: KeywordPub)?
    & (value: (StructDefConstruct | AttrInner | EnumDefConstruct | UseConstruct | FnDefConstruct
        | TraitDef | ImplStatement | TypeDef | ModuleDef | TraitDef | ImplStatement)))]
    RootItem {
        attrs: Vec<NodeRef>,
        mods: NodeRef,
        value: NodeRef,
    },

    // A
    #[rule($HashBang & $BracketOpen & (items: (($ParenthesisOpen & ANY* & $ParenthesisClose) | ($BracketOpen & ANY* & $BracketClose)
    | ($BraceOpen & ANY* & $BraceClose) | ANY))* & $BracketClose)]
    AttrInner { items: Vec<TokenRef> },

    #[rule($Hash & $BracketOpen & (items: (($ParenthesisOpen & ANY* & $ParenthesisClose) | ($BracketOpen & ANY* & $BracketClose)
    | ($BraceOpen & ANY* & $BraceClose) | ANY))* & $BracketClose)]
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
    #[rule($SingleComment & (value: (ANY | $MultilineCommentClose)*) & $NewLine)]
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

    #[rule($KeywordEnum & (name: $Identifier) & (generic: GenericDef)?
    & (where_cond: Where)? & $BraceOpen & ((inner_attrs: AttrInner) | ((items: EnumItem) & $Comma))*
    & ((inner_attrs: AttrInner) | (items: EnumItem))? & $BraceClose)]
    EnumDefConstruct {
        name: TokenRef,
        generic: NodeRef,
        where_cond: NodeRef,
        inner_attrs: Vec<NodeRef>,
        items: Vec<NodeRef>,
    },


    // F
    // #[rule($KeywordFalse)]
    // False,

    #[rule((name: $Identifier) & $Colon & (_type: Type))]
    FnParameter { name: TokenRef, _type: NodeRef },

    #[rule((parent: Type) & $ParenthesisOpen & (name: $Identifier) & $ParenthesisClose & $Colon & (_type: Type))]
    FnParameterWithParent {
        parent: NodeRef,
        name: TokenRef,
        _type: NodeRef,
    },

    #[rule((attrs: AttrOuter)* & (value: (FnParameter | FnParameterWithParent | SelfUse)))]
    FnParameterConstruct { attrs: Vec<NodeRef>, value: NodeRef },

    #[rule($Arrow & (impl_kw: $KeywordImpl)? & (_type: Type))]
    FnTyping { impl_kw: TokenRef, _type: NodeRef },

    #[rule($KeywordFn & (name: $Identifier) & (generic: GenericDef)?
    & ($ParenthesisOpen & (params: FnParameterConstruct)*{$Comma} & $ParenthesisClose)
    & (_type: FnTyping)? & (where_cond: Where)? & (code: (CodeBlock | Semicolon)))]
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

    #[rule($AngleBracketOpen & (items: (Type | SetType))+{$Comma} & $AngleBracketOpen)]
    GenericUse { items: Vec<NodeRef> },


    // H


    // I
    #[rule($KeywordImpl & (_type: Type) & ($KeywordFor & (for_t: Type))?
    & (code: (TraitBlock | Semicolon)))]
    ImplStatement {
        _type: NodeRef,
        for_t: NodeRef,
        code: NodeRef,
    },


    // J


    // K
    #[rule($KeywordPub & ($ParenthesisOpen & (pub_for: ($KeywordSelf | $KeywordMod | $KeywordCrate))
    & $ParenthesisClose))]
    KeywordPub { pub_for: TokenRef },


    // L


    // M
    #[rule($BraceOpen & ((items: RootItem) | (inner_attrs: AttrInner))* & $BraceClose)]
    ModuleBlock {
        items: Vec<NodeRef>,
        inner_attrs: Vec<NodeRef>,
    },

    #[rule($KeywordMod & (name: $Identifier) & (code: (Semicolon | ModuleBlock)))]
    ModuleDef {
        name: TokenRef,
        code: NodeRef,
    },


    // N
    #[rule((value: ($BinNumber | $OctNumber | $DecNumber | $HexNumber)) & (_type: $NumType)?)]
    Number { value: TokenRef, _type: TokenRef },


    // O


    // P


    // Q


    // R
    #[rule(refer: (($Amp | $Star) & $KeywordMut?))]
    Reference { refer: Vec<TokenRef> },


    // S
    // #[rule(value: $String)]
    // String {
    //     value: TokenRef,
    // },

    #[rule($Semicolon)]
    Semicolon,

    #[rule((name: $Identifier) & $Assign & (_type: Type))]
    SetType { name: TokenRef, _type: NodeRef },

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

    #[rule($KeywordStruct & (name: $Identifier) & (generic: GenericDef)?
    & (where_cond: Where)? & (value: (ShortStructDefStatement | StructDefStatement)))]
    StructDefConstruct {
        generic: NodeRef,
        where_cond: NodeRef,
        name: TokenRef,
        value: NodeRef,
    },

    #[rule((refer: Reference)? & $KeywordSelf)]
    SelfUse { refer: NodeRef },


    // T
    // #[rule($KeywordTrue)]
    // True,

    #[rule((value: (BasicType | UseType)) & (generic: GenericUse)?)]
    Type { value: NodeRef, generic: NodeRef },

    #[rule((attrs: AttrOuter)* & (name: $Identifier) & ($Colon & (_type: Type)+{$Add})?)]
    TypeForGeneric {
        attrs: Vec<NodeRef>,
        name: TokenRef,
        _type: Vec<NodeRef>,
    },

    #[rule((attrs: AttrOuter)* & (name: $Identifier)+{$DoubleColon} & $Colon & (_type: Type)+{$Add})]
    TypeForWhere {
        attrs: Vec<NodeRef>,
        name: Vec<TokenRef>,
        _type: Vec<NodeRef>,
    },

    #[rule($Colon & (traits: Type)+{$Add})]
    TraitInherit { traits: Vec<NodeRef> },

    #[rule($KeywordTrait & (name: $Identifier) & (generic: GenericDef)?
        & (inherit: TraitInherit)? & (where_cond: Where)? & (code: (TraitBlock | Semicolon)))]
    TraitDef {
        name: TokenRef,
        generic: NodeRef,
        inherit: NodeRef,
        where_cond: NodeRef,
        code: NodeRef,
    },

    #[rule($KeywordType & (name: $Identifier) & (generic: GenericDef)
    & ($Assign & set: Type)?)]
    TypeDef {
        name: TokenRef,
        generic: NodeRef,
        set: NodeRef,
    },

    #[rule((attrs: AttrOuter)* & (kw_pub: KeywordPub)? & (value: (FnDefConstruct | TypeDef)))]
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
    #[rule((value: $DoubleColon)? & (value: (($KeywordCrate | $Identifier | $KeywordSuper) & $DoubleColon))*
    & (value: ($KeywordCrate | $Identifier | $KeywordSuper)))]
    UseType { value: Vec<TokenRef> },

    #[rule($BraceOpen & (inner: UseStatementConstruct)*{$Comma} & $BraceClose)]
    UseBlock { inner: Vec<NodeRef> },

    #[rule($DoubleColon & block: UseBlock?)]
    UseStatementBlock { block: NodeRef },

    #[rule($KeywordAs & (name: $Identifier))]
    UseStatementAs { name: TokenRef },

    #[rule(((prefix: ($Identifier | $KeywordCrate | $KeywordSuper)) & $DoubleColon)
    & (prefix: ($Identifier | $KeywordCrate | $KeywordSuper)) & (additional: UseStatementBlock | UseStatementAs)?)]
    UseStatementConstruct {
        prefix: Vec<TokenRef>,
        additional: NodeRef,
    },

    #[rule($KeywordUse & (st: UseStatementConstruct))]
    UseConstruct { st: NodeRef },


    // V


    // W
    #[rule($KeywordWhere & (items: TypeForWhere)+{$Comma})]
    Where { items: Vec<NodeRef> },


    // X


    // Y


    // Z
}
