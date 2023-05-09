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

use super::lexis::CToken;
use lady_deirdre::{
    lexis::TokenRef,
    syntax::{Node, NodeRef, SyntaxError},
};
use std::vec::Vec;

#[derive(Node, Debug, Clone)]
#[token(CToken)]
#[error(SyntaxError)]
#[skip($Whitespace | $NewLine)]
#[define(ACTION = (Block | ActionDef | ActionIdent | ActionOther | Return | Semicolon
    | Continue | Goto | Break | DoWhile | While | For | If | Switch | Default))]
#[define(UN_OP = ($UnOp | $Star))]
#[define(BIN_OP = ($UnOp | $Star | $BinOp | $BoolOp1 | $BoolOp2 | $LogicOp))]
#[define(NUMBER = ($BinNumber | $OctNumber | $HexNumber | $DecNumber))]
pub enum CNode {
    #[root]
    #[rule(items: (BasicDefinition | StructDef)*)]
    Root { items: Vec<NodeRef> },

    #[rule((((tokens: $TypeMod)+ & (tokens: $BasicType)?) | (tokens: $BasicType)) & (tokens: $Star)*)]
    BasicType { tokens: Vec<TokenRef> },

    #[rule((name: $Ident) & (ptr: $Star)*)]
    Type { name: TokenRef, ptr: Vec<TokenRef> },

    #[rule((mods: ($Const | $Register | $Volatile | $Auto | $Static))* & (type_: (Type | BasicType)))]
    ModType { mods: Vec<TokenRef>, type_: NodeRef },

    #[rule((prefix: ($Const | $Register | $Volatile | $Auto | $Static))* & (type_: (Type | BasicType))
    & (name: $Ident) & (next: (FnDef | VarDef)))]
    BasicDefinition {
        prefix: Vec<TokenRef>,
        type_: NodeRef,
        name: TokenRef,
        next: NodeRef,
    },

    #[rule((arr: ArrayDef)* & ($Set & (value: Value))? & ($Comma & (next: NextVar))* & $Semicolon)]
    VarDef {
        arr: Vec<NodeRef>,
        value: NodeRef,
        next: Vec<NodeRef>,
    },

    #[rule((name: $Ident) & (arr: ArrayDef)* & ($Set & (value: Value))?)]
    NextVar {
        name: TokenRef,
        arr: Vec<NodeRef>,
        value: NodeRef,
    },

    #[rule($BracketOpen & (num: NUMBER)? & $BracketClose)]
    ArrayDef { num: TokenRef },

    #[rule($ParenthesisOpen & ((mods: ($Const | $Register | $Volatile | $Auto | $Static))* &
        ((type_: BasicType) & (
            (name: $Ident) & ($Comma & (args: Arg))*
            | ($Comma & (types: ModType))*
        ))
        | ((first: $Ident) & (
            ((ptr: $Star)+ & (
                (name: $Ident) & ($Comma & (args: Arg))*
                | ($Comma & (types: ModType))*
            ))
            | ((name: $Ident) & ($Comma & (args: Arg))*)
            | ($Comma & (types: ModType))
        ))
    )? & $ParenthesisClose & (defs: BasicDefinition)* & ((block: Block) | $Semicolon))]
    FnDef {
        mods: Vec<TokenRef>,
        type_: NodeRef,
        name: TokenRef,
        first: TokenRef,
        ptr: Vec<TokenRef>,
        types: Vec<NodeRef>,
        args: Vec<NodeRef>,
        block: NodeRef,
        defs: Vec<NodeRef>,
    },

    #[rule((type_: ModType) & (name: $Ident))]
    Arg { type_: NodeRef, name: TokenRef },

    /* Structs */
    #[rule($Struct & name: $Ident & ($BraceOpen & (fields: (StructDef | BasicDefinition))* & $BraceClose)?
           & (defs: NextVar)* & $Semicolon)]
    StructDef {
        name: TokenRef,
        fields: Vec<NodeRef>,
        defs: Vec<NodeRef>,
    },
    /* */

    /* Enums */
    // Enum,
    /* */
    #[rule((name: $Ident) & (next: (Call | Field))? & (($Set | $SetOp) & (assign: Value))?)]
    IdentVal {
        name: TokenRef,
        next: NodeRef,
        assign: NodeRef,
    },
    #[rule($ParenthesisOpen & ((values: Value) & ($Comma & (values: Value))*)? & $ParenthesisClose)]
    Call { values: Vec<NodeRef> },
    #[rule(($Point & (path: $Ident))+)]
    Field { path: Vec<TokenRef> },

    #[rule(val: NUMBER)]
    Number { val: TokenRef },
    #[rule((val: $String)+)]
    String { val: Vec<TokenRef> },
    #[rule((val: $Char))]
    Char { val: TokenRef },
    #[rule($BraceOpen & (values: StructValField)*{$Comma} & $BraceClose)]
    StructVal { values: Vec<NodeRef> },
    #[rule(($Point & (name: $Ident) & $Set)? & (value: Value))]
    StructValField { name: TokenRef, value: NodeRef },

    #[rule(
        (value: (String | Number | Char | IdentVal | StructVal))
        | ((prefix: UN_OP) & (value: OneValue))
        | ($ParenthesisOpen & (
            ((type_: BasicType) & $ParenthesisClose & (value: OneValue))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis) & $ParenthesisClose)
            | ((first: $Ident) & (
                ((op: $Star) & (
                    ($ParenthesisClose & (value: OneValue))
                    /*| ((ptr: $Star)+ & ((next: Value) | ($ParenthesisClose & (value: OneValue))))*/
                ))
                | ((value: (Call | Field)) & ((op: ($BinOp | $BoolOp1 | $BoolOp2 | $LogicOp))
                    & (next: Value))? & $ParenthesisClose)
                | ($ParenthesisClose & (op: ($BinOp | $LogicOp | $BoolOp1 | $BoolOp2))?
                    & (value: OneValue))
                | ((op: ($UnOp | $BinOp | $BoolOp1 | $BoolOp2 | $LogicOp)) & (next: Value) & $ParenthesisClose)
            )))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis))
        )
    )]
    OneValue {
        prefix: Vec<TokenRef>,
        first: TokenRef,
        type_: NodeRef,
        // ptr: Vec<TokenRef>,
        op: TokenRef,
        value: NodeRef,
        next: NodeRef,
    },

    #[rule($ParenthesisOpen & (value: (FieldFull | IdentInParenthesis)) & $ParenthesisClose)]
    IdentInParenthesis { value: NodeRef },

    #[rule((path: $Ident)+{$Point})]
    FieldFull { path: Vec<TokenRef> },

    #[rule((values: OneValue)+{ops: BIN_OP})]
    Value {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule($BraceOpen & (actions: ACTION)* & $BraceClose)]
    Block { actions: Vec<NodeRef> },

    #[rule((first: $Ident) & (((call: Call) | ((field: Field)? & ($Set & (value: Value))?)
    | ((ptr: $Star*) & (name: $Ident) & ($Set & (value: Value))?) & $Semicolon) | (label: $Colon)))]
    ActionIdent {
        first: TokenRef,
        call: NodeRef,
        field: NodeRef,
        ptr: Vec<TokenRef>,
        name: TokenRef,
        value: NodeRef,
        label: TokenRef,
    },

    #[rule((((prefix: ($Const | $Register | $Volatile | $Auto | $Static))+ & (type_: (Type | BasicType)))
    | ((type_: BasicType))) & (name: $Ident) & (def: VarDef))]
    ActionDef {
        prefix: Vec<TokenRef>,
        type_: NodeRef,
        name: TokenRef,
        def: NodeRef,
    },

    #[rule((value: StartValue) & ((op: BIN_OP) & (next: Value))? & $Semicolon)]
    ActionOther {
        value: NodeRef,
        op: TokenRef,
        next: NodeRef,
    },

    #[rule(
        (value: (String | Number | Char))
        | ((prefix: UN_OP) & (value: OneValue))
        | ($ParenthesisOpen &
            (((_type: BasicType) & $ParenthesisClose & (value: OneValue)
            )
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis) & $ParenthesisClose)
            | ((first: $Ident) & (
                ((ptr: $Star)+ & $ParenthesisClose & (value: OneValue))
                | ((value: (Call | Field)) & ((op: ($BinOp | $LogicOp | $BoolOp1 | $BoolOp2))
                    & (next: Value))? & $ParenthesisClose)
                | ($ParenthesisClose & (op: ($BinOp | $LogicOp | $BoolOp1 | $BoolOp2))? & (value: OneValue))
                | ((op: ($BinOp | $LogicOp | $BoolOp1 | $BoolOp2)) & (next: Value) & $ParenthesisClose)
            )))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis))
        )
        | ((prefix: $IncDec)+ & (value: IdentInParenthesis) & (postfix: $IncDec)*)

    )]
    StartValue {
        prefix: Vec<TokenRef>,
        postfix: Vec<TokenRef>,
        first: TokenRef,
        _type: NodeRef,
        ptr: Vec<TokenRef>,
        op: TokenRef,
        value: NodeRef,
        next: NodeRef,
    },

    #[rule($Return & (value: Value) & $Semicolon)]
    Return { value: NodeRef },
    #[rule($Continue & $Semicolon)]
    Continue,
    #[rule($Goto & (label: $Ident) & $Semicolon)]
    Goto { label: TokenRef },
    #[rule($Break & $Semicolon)]
    Break,
    #[rule($Do & (action: ACTION) & $While
    & $ParenthesisOpen & (cond: Value) & $ParenthesisClose & $Semicolon)]
    DoWhile { cond: NodeRef, action: NodeRef },
    #[rule($While & $ParenthesisOpen & (cond: Value) & $ParenthesisClose & (action: ACTION))]
    While { cond: NodeRef, action: NodeRef },

    #[rule(((prefix: ($Const | $Register | $Volatile | $Auto | $Static))*
    & (type_: (Type | BasicType)) & (name: $Ident) & (def: VarDef)) | $Semicolon)]
    DefFor {
        prefix: Vec<TokenRef>,
        type_: NodeRef,
        name: TokenRef,
        def: NodeRef,
    },

    #[rule($For & $ParenthesisOpen & (def: DefFor) & (cond: Value) & $Semicolon
    & (actions: ACTION*) & $ParenthesisClose & $Semicolon)]
    For {
        def: NodeRef,
        cond: NodeRef,
        actions: Vec<NodeRef>,
    },
    #[rule($If & $ParenthesisOpen & (cond: Value) & $ParenthesisClose & (action: ACTION))]
    If { cond: NodeRef, action: NodeRef },
    #[rule($Switch & $ParenthesisOpen & (value: Value) & $ParenthesisClose
    & $BraceOpen & (actions: (ACTION | Case)*) & $BraceClose)]
    Switch {
        value: NodeRef,
        actions: Vec<NodeRef>,
    },
    #[rule($Case & (cond: Value) & $Colon)]
    Case { cond: NodeRef },
    #[rule($Default & $Semicolon)]
    Default,

    #[rule($Semicolon)]
    Semicolon,
}

/*



*/
