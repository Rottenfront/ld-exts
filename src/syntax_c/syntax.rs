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
    syntax::{Node, NodeRef, SyntaxError},
};
use std::vec::Vec;
use super::lexis::CToken;

#[derive(Node, Debug, Clone)]
#[token(CToken)]
#[error(SyntaxError)]
#[skip($Whitespace | $NewLine)]
#[define(ACTION = (Block | ActionDef | ActionIdent | ActionOther | Return | Semicolon
    | Continue | Goto | Break | DoWhile | While | For | If | Switch | Default))]
pub enum CNode {
    #[root]
    #[rule(items: (BasicDefinition | StructDef)*)]
    Root { items: Vec<NodeRef> },
    
    #[rule((((tokens: $TypeMod)+ & (tokens: $BasicType)?) | (tokens: $BasicType)) & (tokens: $Star)*)]
    BasicType {
        tokens: Vec<TokenRef>,
    },

    #[rule((name: $Ident) & (ptr: $Star)*)]
    Type {
        name: TokenRef,
        ptr: Vec<TokenRef>,
    },
    
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

    #[rule($BracketOpen & (num: $Number)? & $BracketClose)]
    ArrayDef {
        num: TokenRef,
    },

    #[rule($ParenthesisOpen & (
        ((type_: BasicType) & (
            (name: $Ident) & ($Comma & (args: Arg))*
            | ($Comma & (types: (BasicType | Type)))*
        ))
        | ((first: $Ident) & (
            ((ptr: $Star)+ & (
                (name: $Ident) & ($Comma & (args: Arg))*
                | ($Comma & (types: (BasicType | Type)))*
            ))
            | ((name: $Ident) & ($Comma & (args: Arg))*)
            | ($Comma & (types: (BasicType | Type)))
        ))
    ) & $ParenthesisClose & (defs: BasicDefinition)* & ((block: Block) | $Semicolon))]
    FnDef {
        type_: NodeRef,
        name: TokenRef,
        first: TokenRef,
        ptr: Vec<TokenRef>,
        types: Vec<NodeRef>,
        args: Vec<NodeRef>,
        block: NodeRef,
        defs: Vec<NodeRef>,
    },

    #[rule((type_: (Type | BasicType)) & (name: $Ident))]
    Arg {
        type_: NodeRef,
        name: TokenRef,
    },

    /* Structs */
    #[rule($Struct & name: $Ident & ($BraceO & (fields: (StructDef | BasicDefinition))* & $BraceC)?
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

    #[rule((name: $Ident) & (next: (Call | Field))? & (($Set | $OpWithSet) & (assign: Value))?)]
    IdentVal { name: TokenRef, next: NodeRef, assign: NodeRef },
    #[rule($ParenthO & ((values: Value) & ($Comma & (values: Value))*)? & $ParenthC)]
    Call { values: Vec<NodeRef> },
    #[rule(($Point & (path: $Ident))+)]
    Field { path: Vec<TokenRef> },

    #[rule(val: $Number)]
    Number { val: TokenRef },
    #[rule((val: $String)+)]
    String { val: Vec<TokenRef> },
    #[rule((val: $Char))]
    Char { val: TokenRef },
    #[rule($BraceO & (values: StructValField)*{$Comma} & $BraceC)]
    StructVal { values: Vec<NodeRef> },
    #[rule(($Point & (name: $Ident) & $Set)? & (value: Value))]
    StructValField {
        name: TokenRef,
        value: NodeRef,
    },

    #[rule(
        (value: (String | Number | Char | IdentVal | StructVal))
        | ((prefix: $UnOp) & (value: OneValue))
        | ($ParenthO & (
            ((type_: BasicType) & $ParenthC & (value: OneValue))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis) & $ParenthC)
            | ((first: $Ident) & (
                ((ptr: $Star)+ & $ParenthC & (value: OneValue))
                | ((value: (Call | Field)) & ((op: ($BinOp | $UnOp | $LogicOp | $BoolOp))
                    & (next: Value))? & $ParenthC)
                | ($ParenthC & (op: ($BinOp | $LogicOp | $BoolOp))? & (value: OneValue))
                | ((op: ($BinOp | $UnOp | $LogicOp | $BoolOp)) & (next: Value) & $ParenthC)
            )))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis))
        )
    )]
    OneValue {
        prefix: Vec<TokenRef>,
        first: TokenRef,
        type_: NodeRef,
        ptr: Vec<TokenRef>,
        op: TokenRef,
        value: NodeRef,
        next: NodeRef,
    },

    #[rule($ParenthO & (value: (FieldFull | IdentInParenthesis)) & $ParenthC)]
    IdentInParenthesis {
        value: NodeRef
    },

    #[rule((path: $Ident)+{$Point})]
    FieldFull { path: Vec<TokenRef> },

    #[rule((values: OneValue)+{ops: ($BinOp | $UnOp | $BoolOp | $LogicOp)})]
    Value {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule($BraceO & (actions: ACTION)* & $BraceC)]
    Block {
        actions: Vec<NodeRef>,
    },

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

    #[rule(
        ((tokens: $Const) & (tokens: ($Long | $Short | $Signed | $Unsigned))*
        & (tokens: $Star)* & (tokens: $Ident))
        | ((tokens: ($Long | $Short | $Signed | $Unsigned)+) & (
            ((tokens: $Ident) & (tokens: ($Star* & $Ident))?)
            | ((tokens: $Star+) & (tokens: $Ident))
        )) & (arr: ($BracketO & $Number? & $BracketC))?
        & ($Set & (value: Value))? & $Semicolon
    )]
    ActionDef {
        tokens: Vec<TokenRef>,
        arr: Vec<TokenRef>,
        value: NodeRef,
    },

    #[rule((value: StartValue) & ((op: ($BinOp | $UnOp | $BoolOp | $LogicOp)) & (next: Value))? & $Semicolon)]
    ActionOther {
        value: NodeRef,
        op: TokenRef,
        next: NodeRef,
    },

    #[rule(
        (value: (String | Number | Char))
        | ((prefix: $UnOp) & (value: OneValue))
        | ($ParenthO & 
            (((_type: ($Long | $Short | $Signed | $Unsigned)+)
                & (_type: $Ident)? & (_type: $Star)* & $ParenthC & (value: OneValue)
            )
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis) & $ParenthC)
            | ((first: $Ident) & (
                ((_type: $Star)+ & $ParenthC & (value: OneValue))
                | ((value: (Call | Field)) & ((op: ($BinOp | $UnOp | $LogicOp | $BoolOp))
                    & (next: Value))? & $ParenthC)
                | ($ParenthC & (op: ($BinOp | $LogicOp | $BoolOp))? & (value: OneValue))
                | ((op: ($BinOp | $UnOp | $LogicOp | $BoolOp)) & (next: Value) & $ParenthC)
            )))
            | ((prefix: ($IncDec)+) & (value: IdentInParenthesis))
        )
    )]
    StartValue {
        prefix: Vec<TokenRef>,
        first: TokenRef,
        _type: Vec<TokenRef>,
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
    & $ParenthO & (cond: Value) & $ParenthC & $Semicolon)]
    DoWhile { cond: NodeRef, action: NodeRef },
    #[rule($While & $ParenthO & (cond: Value) & $ParenthC & (action: ACTION))]
    While { cond: NodeRef, action: NodeRef },

    #[rule(((tokens: $Const)? & (
        (tokens: ($Long | $Short | $Signed | $Unsigned))+ & (
            (tokens: $Star+) & (tokens: $Ident)
            | (tokens: $Ident & ((tokens: $Star*) & (tokens: $Ident))?)
        )
        | ((tokens: $Ident) & (tokens: $Star*) & (tokens: $Ident))
    ) & (arr: ($BracketO & $Number? & $BracketC))? & ($Set & (value: Value))?
    & ($Comma & (nexts: NextVar)+{$Comma})?)? & $Semicolon)]
    DefFor {
        tokens: Vec<TokenRef>,
        arr: Vec<TokenRef>,
        value: NodeRef,
        nexts: Vec<NodeRef>,
    },

    #[rule($For & $ParenthO & (def: DefFor) & (cond: Value) & $Semicolon
    & (actions: ACTION*) & $ParenthC & $Semicolon)]
    For {
        def: NodeRef,
        cond: NodeRef,
        actions: Vec<NodeRef>,
    },
    #[rule($If & $ParenthO & (cond: Value) & $ParenthC & (action: ACTION))]
    If {
        cond: NodeRef,
        action: NodeRef,
    },
    #[rule($Switch & $ParenthO & (value: Value) & $ParenthC & $BraceO & (actions: (ACTION | Case)*) & $BraceC)]
    Switch { value: NodeRef, actions: Vec<NodeRef> },
    #[rule($Case & (cond: Value) & $Colon)]
    Case { cond: NodeRef },
    #[rule($Default & $Semicolon)]
    Default,

    #[rule($Semicolon)]
    Semicolon,
}

/*



 */
