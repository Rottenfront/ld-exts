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

use super::lexis::CppToken;
use lady_deirdre::{
    lexis::TokenRef,
    syntax::{Node, NodeRef, SyntaxError, SyntaxSession},
};
use std::vec::Vec;

#[derive(Node, Debug, Clone)]
#[token(CppToken)]
#[error(SyntaxError)]
#[trivia($Whitespace | Comment)]
#[define(NUMBER = ($DecNumber | $HexNumber | $BinNumber | $OctNumber))]
#[define(ACTION = (CodeBlock))]
pub enum CppNode {
    #[rule(val: $Comment)]
    Comment { val: TokenRef },

    #[root]
    #[rule(items: RootItem*)]
    Root { items: Vec<NodeRef> },

    #[rule(($Extern & (lang: $String))? & (template: TemplateDef)? &
    (item: (BasicDef | StructDef | EnumDef | ClassDef | NamespaceDef)))]
    RootItem { lang: TokenRef, template: NodeRef, item: NodeRef },

    #[rule($Template & $Less & (types: TemplateType)*{$Comma} & $Comma? & $Greater)]
    TemplateDef {
        types: Vec<NodeRef>,
    },

    #[rule((type_: ($Typename | $Class)) & (name: $Ident) & (recursive: $Point)*)]
    TemplateType {
        type_: TokenRef,
        name: TokenRef,
        recursive: Vec<TokenRef>,
    },
    
    #[rule(((mods: $TypeMod)+ & (type_: $BasicType)?) | (type_: $BasicType))]
    BasicType {
        mods: Vec<TokenRef>,
        type_: TokenRef,
    },

    #[rule((((mods: ($Const | $Auto | $Static | $Volatile)+) & (type_: (BasicValue | Path))?)
    | (type_: (BasicType | Path))) & (generic: GenericUse)? & (recursive: $Point*)
    & (refer: ($Amp | $Star)*))]
    Type {
        mods: Vec<TokenRef>,
        type_: NodeRef,
        refer: Vec<TokenRef>,
        generic: NodeRef,
        recursive: Vec<TokenRef>,
    },

    #[rule((path: $Ident)+{$DColon})]
    Path {
        path: Vec<TokenRef>,
    },

    #[rule((type_: Type) & ((path: Path)
        | ($Open & (refer: ($Amp | $Star)*) & (path: Path) & $Close))
    & (def: (FnDef | VarDef)))]
    BasicDef {
        type_: NodeRef,
        path: NodeRef,
        refer: Vec<TokenRef>,
        def: NodeRef,
    },

    #[rule($BracketOpen & (val: Value)? & $BracketClose)]
    ArrIndex { val: NodeRef },

    #[rule((arr: ArrIndex)? & ($Set & (val: Value))? & ($Comma & (nexts: StructVar))* & $Semicolon)]
    VarDef {
        arr: NodeRef,
        val: NodeRef,
        nexts: Vec<NodeRef>,
    },

    #[rule($Open & (params: FnParameter)*{$Comma} & $Close & (defs: BasicDef)*
        & ((($Set & (val: (NUMBER | $String | $Char | $Delete)))? & $Semicolon)
        | (block: CodeBlock)))]
    FnDef {
        params: Vec<NodeRef>,
        defs: Vec<NodeRef>,
        block: NodeRef,
        val: TokenRef,
    },

    #[rule((type_: Type) & (name: $Ident)?)]
    FnParameter {
        name: TokenRef,
        type_: NodeRef,
    },

    #[rule($Struct & (name: Path) & $BraceOpen & (fields: RootItem)* & $BraceClose
           & (defs: StructVar)*{$Comma} & $Semicolon)]
    StructDef {
        name: NodeRef,
        fields: Vec<NodeRef>,
        defs: Vec<NodeRef>,
    },

    #[rule((name: Path) & (add: ArrIndex)? & ($Set & (val: Value))?)]
    StructVar {
        name: NodeRef,
        val: NodeRef,
        add: NodeRef,
    },

    #[rule($Enum & (name: Path) & $BraceOpen & (items: EnumItem)*{$Comma} & $Comma? & $BraceClose)]
    EnumDef {
        name: NodeRef,
        items: Vec<NodeRef>,
    },

    #[rule((name: $Ident) & ($Set & (num: NUMBER))?)]
    EnumItem {
        name: TokenRef,
        num: TokenRef,
    },

    #[rule($Class & (name: Path) & $BraceOpen & (fields: (RootItem | VisibleLabel))* & $BraceClose)]
    ClassDef {
        name: NodeRef,
        fields: Vec<NodeRef>,
    },

    #[rule($BraceOpen & (actions: ACTION*) & $BraceClose)]
    CodeBlock {
        actions: Vec<NodeRef>,
    },

    #[rule((name: ($Public | $Protected | $Private)) & $Colon)]
    VisibleLabel { name: TokenRef },

    #[rule($Namespace & (name: Path) & $BraceOpen & (items: RootItem)* & $BraceClose)]
    NamespaceDef {
        name: NodeRef,
        items: Vec<NodeRef>,
    },

    #[rule((val: ($BinOp | $MultiOp | $Star | $Amp)) | ((val: $Less) & (add: $Less)?)
    | ((val: $Greater) & (add: $Greater)?))]
    BinOp {
        val: TokenRef,
        add: TokenRef,
    },

    #[rule((values: SingleValue)+{ops: BinOp})]
    Value {
        values: Vec<NodeRef>,
        ops: Vec<NodeRef>,
    },

    #[rule(val: (NUMBER | $String | $Char))]
    BasicValue { val: TokenRef },

    #[rule($Point & (name: $Ident) & ((generic: GenericUse)? & (call: Call))?)]
    Method {
        name: TokenRef,
        call: NodeRef,
        generic: NodeRef,
    },

    #[rule($Less & (types: Type)*{$Comma} & $Greater)]
    GenericUse {
        types: Vec<NodeRef>,
    },

    #[rule($Open & (args: Value)*{$Comma} & $Close)]
    Call { args: Vec<NodeRef> },

    #[rule(val: $IncDec)]
    IncDec { val: TokenRef },

    #[rule((un_op: ($UnOp | $MultiOp | $Star | $Amp | $IncDec)*)
    & (val: (BasicValue | ValueParenthesis)) & (methods: (Method | ArrIndex | IncDec))*)]
    SingleValue {
        un_op: Vec<TokenRef>,
        val: NodeRef,
        methods: Vec<NodeRef>,
    },

    #[rule($Open & (val: Value) & $Close)]
    ValueParenthesis { val: NodeRef },
}
/*
impl CppNode {
    fn parse_action<'code>(session: &mut impl SyntaxSession<'code, Node = Self>) -> Self {
        unimplemented!()
    }
}
*/
