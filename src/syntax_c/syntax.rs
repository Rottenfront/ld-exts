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
#[token(super::lexis::CToken)]
#[error(lady_deirdre::syntax::SyntaxError)]
#[skip($Whitespace | $NewLine)]
pub enum CNode {
    #[root]
    #[rule((items: (DefWithType | StructDef))*)]
    Root { items: Vec<NodeRef> },

    #[rule((tokens: $Const)? & (
        (tokens: $Long | $Short | $Signed | $Unsigned)+ & (
            (ptr: $Star+) & (tokens: $Ident)
            | (tokens: $Ident & ((ptr: $Star*) & (tokens: $Ident))?)
        )
        | ((tokens: $Ident) & (ptr: $Star*) & (tokens: $Ident))
    ) & (arr: ($BracketO & $Number? & $BracketC))?)]
    Def {
        tokens: Vec<TokenRef>,
        arr: Vec<TokenRef>,
        ptr: Vec<TokenRef>,
    },

    /* Funcs/Vars */
    #[rule((modifier: ($Auto | $Static | $Register | $Volatile))? & (def: Def)
    & (next: (FnDef | VarDef)))]
    DefWithType {
        modifier: TokenRef,
        def: NodeRef,
        next: NodeRef,
    },

    #[rule(($ParenthO & (((first: $Ident)
    & (($Comma & (names: $Ident))*
    | ((ptr: $Star*) & (names: $Ident) & ($Comma & (params: Def))*)))
    | (
        (mod_first: $Const) & (
            (mod_first: $Long | $Short | $Signed | $Unsigned)+ & (
                (ptr: ($Star | $Amp)+) & (names: $Ident)
                | (first: $Ident & ((ptr: ($Star | $Amp)*) & (names: $Ident))?)
            )
            | ((first: $Ident) & (ptr: ($Star | $Amp)*) & (names: $Ident))
        ) & ($Comma & (params: Def))*
    )
    | ((mod_first: $Long | $Short | $Signed | $Unsigned)+ & (
        (ptr: ($Star | $Amp)+) & (names: $Ident)
        | (first: $Ident & ((ptr: ($Star | $Amp)*) & (names: $Ident))?)
    )
    | ((first: $Ident) & ((ptr: ($Star | $Amp)*) & (names: $Ident)))
    & ($Comma & (params: Def))*
    )
    & ($Comma & (params: Def))*)?
    & $ParenthC) & (defs: DefWithType)* & ((block: Block) | $Semicolon))]
    FnDef {
        mod_first: Vec<TokenRef>,
        first: TokenRef,
        ptr: Vec<TokenRef>,
        names: Vec<TokenRef>,
        params: Vec<NodeRef>,
        defs: Vec<NodeRef>,
        block: NodeRef,
    },

    #[rule((arr: ($BracketO & $Number? & $BracketC))? & ($Set & (value: Value))?
    & ($Comma & (nexts: NextVar)+{$Comma})? & $Semicolon)]
    VarDef {
        arr: Vec<TokenRef>,
        value: NodeRef,
        nexts: Vec<NodeRef>,
    },

    #[rule((name: $Ident) & (arr: ($BracketO & $Number? & $BracketC))? & ($Set & (value: Value))?)]
    NextVar {
        name: TokenRef,
        arr: Vec<TokenRef>,
        value: NodeRef,
    },
    /* */

    /* Typedef */
    // TypeDef,
    /* */

    /* Structs */
    #[rule($Struct & name: $Ident & ($BraceO & (fields: (StructDef | DefWithType))* & $BraceC)?
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

    #[rule((value: (String | Char | StructVal))
    | ((prefix: ($UnOp | $IncDec))* & (value: (IdentVal | Number | ValueInParenthesis) & (postfix: $IncDec)*)))]
    OneValue {
        prefix: Vec<TokenRef>,
        value: NodeRef,
        postfix: Vec<TokenRef>,
    },

    #[rule((prefix: ($UnOp | $IncDec))* & $ParenthO & (value: Value) & $ParenthC & (postfix: $IncDec)*)]
    ValueInParenthesis {
        prefix: Vec<TokenRef>,
        value: NodeRef,
        postfix: Vec<TokenRef>,
    },

    #[rule((values: OneValue)+{ops: ($BinOp | $UnOp | $BoolOp | $LogicOp)})]
    Value {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule($BraceO & ((actions: (ActionIdent)) & $Semicolon)* & $BraceC)]
    Block {
        actions: Vec<NodeRef>,
    },

    #[rule((first: $Ident) & ((call: Call) | ((field: Field)? & ($Set & (value: Value))?)
    | ((ptr: $Star*) & (name: $Ident) & ($Set & (value: Value))?)))]
    ActionIdent {
        first: TokenRef,
        call: NodeRef,
        field: NodeRef,
        ptr: Vec<TokenRef>,
        name: TokenRef,
        value: NodeRef,
    },
}

/*



 */
