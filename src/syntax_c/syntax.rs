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

/*
 * Naming
 * 
 * Prefixes:
 * - Un = unary
 *
 * Suffixes:
 * - Op = operator
 * - Value = one of exression types
 * - Def - definition
 */

#![allow(unused_imports)]

use super::lexis::CToken;
use lady_deirdre::{
    lexis::{SiteRef, TokenRef, TokenSet},
    syntax::{Node, NodeRef, RuleSet, SyntaxError, SyntaxSession},
};

const VALUE_RULE: u16 = 10;
const IF_RULE: u16 = 11;
const SWITCH_RULE: u16 = 12;
const WHILE_RULE: u16 = 13;
const FOR_RULE: u16 = 14;
const DO_WHILE_RULE: u16 = 15;
const BASIC_TYPE_RULE: u16 = 30;

#[derive(Node, Debug, Clone)]
#[token(CToken)]
#[error(SyntaxError)]
#[trivia($Whitespace  | $NewLine           | $Unreachable   | $IfDirective
| $IfDefDirective     | $IfNotDefDirective | $ElseDirective | $ElseIfDirective
| $ElseIfDefDirective | $ElseIfNotDefDirective              | $EndIfDirective
| $PragmaDirective    | $ErrorDirective    | $WarnDirective | $LineDirective
| $Comment | $MlComment | MacroUse)]
pub enum CNode {
    #[rule(tokens: ($MacroName ($MacroPOpen ($MacroArg | $Comma)* $MacroPClose)?))]
    MacroUse {
        tokens: Vec<TokenRef>,
    },

    #[root]
    #[rule(items: RootItem*)]
    Root {
        items: Vec<NodeRef>,
    },

    #[rule(item: (Typedef | StructTypeDef | Definition))]
    RootItem {
        item: NodeRef,
    },

    // struct [ <struct-name> ] [ { <struct-items> } ]
    #[rule($Struct (
        ((name: $Ident) ($BcOpen (items: StructDefinition)* $BcClose)?)
        | ($BcOpen (items: StructDefinition)* $BcClose)
    ))]
    StructType {
        name: TokenRef,
        items: Vec<NodeRef>,
    },

    // чтобы упростить правило не учитываем порядок слов
    #[index(BASIC_TYPE_RULE)]
    #[rule(tokens: (
        ($BasicTypeMod+ $BasicType?)+
        | $BasicType
        | $Ident
    ))]
    BasicType {
        tokens: Vec<TokenRef>,
    },

    // enum [ <enum-name> ] [ { <enum-items> } ]
    #[rule($Enum (
        ((name: $Ident) ($BcOpen (items: Value) $BcClose)?)
        | ($BcOpen (items: Value) $BcClose)
    ))]
    EnumType {
        name: TokenRef,
        items: NodeRef, // Value = assignments divided by comma
    },

    #[rule($Union (
        ((name: $Ident) ($BcOpen (items: StructDefinition)* $BcClose)?)
        | ($BcOpen (items: StructDefinition)* $BcClose)
    ))]
    UnionType {
        name: TokenRef,
        items: Vec<NodeRef>,
    },

    #[rule((val: (StructType | BasicType | EnumType | UnionType))
    (ptr: ($Star | $Amp)*))]
    Type {
        val: NodeRef,
        ptr: Vec<TokenRef>,
    },

    #[rule($Typedef (type_: Type) (name: $Ident)?)]
    Typedef {
        type_: NodeRef,
        name: TokenRef,
    },

    #[rule(
        (type_: (StructType | BasicType | EnumType | UnionType))
        (ptr: ($Star | $Amp)*) (name: $Ident) $Semicolon
    )]
    StructDefinition {
        type_: NodeRef,
        ptr: Vec<TokenRef>,
        name: TokenRef,
    },

    #[rule(
        (type_: (StructType | UnionType))
        (ptr: $Star*)
        (
            ((name: $Ident) (next: (StructTypeDefValue | FuncDef)))
            | (
                $POpen ((func: $Star) (name: $Ident)) $PClose
                $POpen (args: FuncArg)*{$Comma} $PClose
                (next: DefinitionValue)
            )
        )
    )]
    StructTypeDef {
        type_: NodeRef,
        ptr: Vec<TokenRef>,
        func: TokenRef,
        name: TokenRef,
        args: Vec<NodeRef>,
        next: NodeRef,
    },

    #[rule(
        (array: ArrayIndex)? ($SetOp (value: (Value | StructValue)))?
        ($Comma (next: StructTypeDefValueNext))* $Semicolon
    )]
    StructTypeDefValue {
        array: NodeRef,
        value: NodeRef,
        next: Vec<NodeRef>,
    },

    #[rule((name: $Ident) (array: ArrayIndex)?
    ($SetOp (value: (Value | StructValue)))?)]
    StructTypeDefValueNext {
        name: TokenRef,
        array: NodeRef,
        value: NodeRef,
    },

    #[rule(
        (type_: (EnumType | BasicType))
        (ptr: $Star*) (
            ((name: $Ident) (next: (DefinitionValue | FuncDef)))
            | (
                $POpen ((func: $Star) (name: $Ident)) $PClose
                $POpen (args: FuncArg)*{$Comma} $PClose
                (next: DefinitionValue)
            )
        )
    )]
    Definition {
        type_: NodeRef,
        ptr: Vec<TokenRef>,
        func: TokenRef,
        name: TokenRef,
        args: Vec<NodeRef>,
        next: NodeRef,
    },
    #[rule(
        (array: ArrayIndex)? ($SetOp (value: Value))?
        ($Comma (next: DefinitionValueNext))* $Semicolon
    )]
    DefinitionValue {
        array: NodeRef,
        value: NodeRef,
        next: Vec<NodeRef>,
    },

    #[rule((name: $Ident) (array: ArrayIndex)?
    ($SetOp (value: Value))?)]
    DefinitionValueNext {
        name: TokenRef,
        array: NodeRef,
        value: NodeRef,
    },

    #[rule(
        $POpen (args: FuncArg)*{$Comma} $PClose
        ((block: CodeBlock) | $Semicolon)
    )]
    FuncDef {
        args: Vec<NodeRef>,
        block: NodeRef,
    },

    #[rule(
        (type_: Type) (
            (name: $Ident)
            | ($POpen ((func: $Star) (name: $Ident)?) $PClose
               $POpen (args: FuncArg)*{$Comma} $PClose)
        )?
    )]
    FuncArg {
        type_: NodeRef,
        name: TokenRef,
        func: TokenRef,
        args: Vec<NodeRef>,
    },

    #[rule($BcOpen (items: Action*) $BcClose)]
    CodeBlock {
        items: Vec<NodeRef>,
    },

    #[parser(parse_action)]
    #[rule((val: Value) $Semicolon)]
    Action {
        val: NodeRef,
    },

    Label {
        name: TokenRef,
    },
    #[index(IF_RULE)]
    #[rule(
        $If $POpen (cond: Value) $PClose (action: Action)
        ($Else (next: Action))?
    )]
    IfStatement {
        cond: NodeRef,
        action: NodeRef,
        next: NodeRef,
    },
    #[index(SWITCH_RULE)]
    #[rule($Switch $POpen (val: Value) $PClose (block: CodeBlock))]
    SwitchStatement {
        val: NodeRef,
        block: NodeRef,
    },
    #[index(WHILE_RULE)]
    #[rule($While $POpen (cond: Value) $PClose (action: Action))]
    WhileStatement {
        cond: NodeRef,
        action: NodeRef,
    },
    #[index(DO_WHILE_RULE)]
    #[rule(
        $Do (action: Action)
        $While $POpen (cond: Value) $PClose $Semicolon
    )]
    DoWhileStatement {
        action: NodeRef,
        cond: NodeRef,
    },
    #[index(FOR_RULE)]
    #[rule(
        $For $POpen
        (def: Action)
        (cond: Value) $Semicolon
        (inn_action: Value)
        $PClose (action: Action)
    )]
    ForStatement {
        def: NodeRef,
        cond: NodeRef,
        inn_action: NodeRef,
        action: NodeRef,
    },

    #[rule($BkOpen (value: AssignValue)? $BkClose)]
    ArrayIndex {
        value: NodeRef,
    },

    #[rule($BcOpen $BcClose)]
    StructValue {},

    #[rule(token: ($Number | $HexNumber | $DotNumber))]
    Number {
        token: TokenRef,
    },

    #[rule(tokens: $String+)]
    String {
        tokens: Vec<TokenRef>,
    },

    #[rule(token: $Character)]
    Char {
        token: TokenRef,
    },

    #[rule(first: $Ident)]
    IdentVal {
        first: TokenRef,
    },

    #[rule($POpen (val: (Value | CodeBlock))? $PClose)]
    ParentVal {
        val: NodeRef,
    },

    #[rule($KwGeneric $POpen (first: Value) ($Comma (items: GenericSectionItem))* $PClose)]
    GenericSection {
        first: NodeRef,
        items: Vec<NodeRef>,
    },

    #[rule(((type_: Type) | $Default) $Colon (val: Value))]
    GenericSectionItem {
        type_: NodeRef,
        val: NodeRef,
    },

    #[rule(
        (ops: ($Tilde | $Bang | $Inc | $Dec | $Sub | $Add | $SizeOf | $Star
             | $Amp | $GnuReal | $GnuImaginary | $TypeOf))*
        (val: (IdentVal | ParentVal | String | Number | Char | GenericSection))
        (post_ops: PostOperator)*
    )]
    UnVal {
        val: NodeRef,
        ops: Vec<TokenRef>,
        post_ops: Vec<NodeRef>,
    },

    #[rule($Dot (id: $Ident))]
    PostPath {
        id: TokenRef,
    },

    #[rule($Arrow (id: $Ident))]
    PostPtrPath {
        id: TokenRef,
    },

    // ParentVal == function call
    #[rule(val: (PostPath | IncDec | ArrayIndex | ParentVal | PostPtrPath))]
    PostOperator {
        val: NodeRef,
    },

    #[rule(tokens: ($Inc | $Dec)+)]
    IncDec {
        tokens: Vec<TokenRef>,
    },

    #[rule((values: UnVal)+{ops: ($Star | $Slash | $Percent)})]
    Value1 {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule((values: Value1)+{ops: ($Add | $Sub)})]
    Value2 {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule((values: Value2)+{ops: $BitShift})]
    Value3 {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule((values: Value3)+{ops: $BoolOp})]
    Value4 {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule((values: Value4)+{ops: $EqOrNot})]
    Value5 {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[rule((values: Value5)+{$Amp})]
    BitAndOpValue {
        values: Vec<NodeRef>,
    },

    #[rule((values: BitAndOpValue)+{$Xor})]
    BitXorOpValue {
        values: Vec<NodeRef>,
    },

    #[rule((values: BitXorOpValue)+{$Pipe})]
    BitOrOpValue {
        values: Vec<NodeRef>,
    },

    #[rule((values: BitOrOpValue)+{$LogicalAnd})]
    LogicAndOpValue {
        values: Vec<NodeRef>,
    },

    #[rule((values: LogicAndOpValue)+{$LogicalOr})]
    LogicOrOpValue {
        values: Vec<NodeRef>,
    },

    #[rule((cond: LogicOrOpValue) ($Quest (value1: LogicOrOpValue) $Colon (value2: LogicOrOpValue))?)]
    TernaryOpValue {
        cond: NodeRef,
        value1: NodeRef,
        value2: NodeRef,
    },

    #[rule((values: TernaryOpValue)+{ops: ($SetOp | $Set)})]
    AssignValue {
        values: Vec<NodeRef>,
        ops: Vec<TokenRef>,
    },

    #[index(VALUE_RULE)]
    #[rule((values: AssignValue)+{$Comma})]
    Value {
        values: Vec<NodeRef>,
    },

    Mismatch {
        tokens: Vec<TokenRef>,
        nodes: Vec<NodeRef>,
    },
}

/*
 * 1) Unary ops: ~ ! ++ -- - + sizeof * &
 * 2) Binary ops:
 *     1. * / %
 *     2. + -
 *     3. << >>
 *     4. < > <= >=
 *     5. == !=
 *     6. &
 *     7. ^
 *     8. |
 *     9. &&
 *     10. ||
 *     11. ?:
 *     12. = += -= *= &= |= ^= %= /= >>= <<=
 *     13. ,
 */

// TODO: add attributes and other modifiers to definitions

impl CNode {
    fn parse_action<'code>(session: &mut impl SyntaxSession<'code, Node = Self>) -> Self {
        loop {
            match session.token(0) {
                CToken::Ident => {
                    let mut i = 1;
                    while session.token(i) != CToken::EOI
                        || session.token(i) == CToken::Whitespace
                        || session.token(i) == CToken::NewLine
                    {
                        i += 1;
                    }
                    break match session.token(i) {
                        CToken::Colon => {
                            for _ in 0..i {
                                session.advance();
                            }
                            Self::Label {
                                name: session.token_ref(0),
                            }
                        }
                        CToken::EOI => Self::Action {
                            val: session.descend(VALUE_RULE),
                        },
                        CToken::Ident => Self::parse_definition(session),
                        _ => unreachable!(),
                    };
                }
                _ => {}
            }
        }
    }
    fn parse_definition<'code>(session: &mut impl SyntaxSession<'code, Node = CNode>) -> CNode {
        static RULES_1: RuleSet = RuleSet::new(&[32u16]);
        static RULES_2: RuleSet = RuleSet::new(&[36u16, 38u16]);
        static RULES_3: RuleSet = RuleSet::new(&[4u16]);
        static RULES_4: RuleSet = RuleSet::new(&[4u16, 31u16]);
        static TOKENS_1: TokenSet = TokenSet::inclusive(&[CToken::PClose as u8]);
        static TOKENS_2: TokenSet =
            TokenSet::inclusive(&[CToken::Ident as u8, CToken::POpen as u8, CToken::Star as u8]);
        static TOKENS_3: TokenSet = TokenSet::inclusive(&[
            CToken::BasicType as u8,
            CToken::BasicTypeMod as u8,
            CToken::Enum as u8,
            CToken::Ident as u8,
            CToken::Struct as u8,
            CToken::Union as u8,
        ]);
        static TOKENS_4: TokenSet =
            TokenSet::inclusive(&[CToken::Comma as u8, CToken::PClose as u8]);
        static TOKENS_5: TokenSet = TokenSet::inclusive(&[
            CToken::BasicType as u8,
            CToken::BasicTypeMod as u8,
            CToken::Ident as u8,
        ]);
        static TOKENS_6: TokenSet = TokenSet::inclusive(&[
            CToken::BasicType as u8,
            CToken::BasicTypeMod as u8,
            CToken::Enum as u8,
            CToken::Ident as u8,
        ]);
        static TOKENS_7: TokenSet = TokenSet::inclusive(&[CToken::Ident as u8]);
        static TOKENS_8: TokenSet = TokenSet::inclusive(&[CToken::POpen as u8]);
        static TOKENS_9: TokenSet = TokenSet::inclusive(&[
            CToken::BasicType as u8,
            CToken::BasicTypeMod as u8,
            CToken::Enum as u8,
            CToken::Ident as u8,
            CToken::PClose as u8,
            CToken::Struct as u8,
            CToken::Union as u8,
        ]);
        static TOKENS_10: TokenSet = TokenSet::inclusive(&[
            CToken::BkOpen as u8,
            CToken::Comma as u8,
            CToken::Semicolon as u8,
            CToken::SetOp as u8,
        ]);
        static TOKENS_11: TokenSet = TokenSet::inclusive(&[CToken::Star as u8]);
        static TOKENS_12: TokenSet = TokenSet::inclusive(&[
            CToken::BkOpen as u8,
            CToken::Comma as u8,
            CToken::POpen as u8,
            CToken::Semicolon as u8,
            CToken::SetOp as u8,
        ]);
        let mut state = 1usize;
        let mut site;
        let mut first = true;
        let mut capture_name = TokenRef::nil();
        let mut capture_args = Vec::<NodeRef>::with_capacity(1);
        let mut capture_next = NodeRef::nil();
        let mut capture_ptr = Vec::<TokenRef>::with_capacity(1);
        let mut capture_type_ = NodeRef::nil();
        let mut capture_func = TokenRef::nil();
        loop {
            match first {
                true => first = false,
                false => {
                    let mut token;
                    loop {
                        token = session.token(0);
                        if token == CToken::EOI
                            || token == CToken::Whitespace
                            || token == CToken::NewLine
                        {
                            break;
                        }
                    }
                }
            }
            match state {
                1usize => {
                    let token = session.token(0);
                    if TokenSet::contains(&TOKENS_5, token as u8) {
                        capture_type_ =
                            session.descend(36);
                        state = 2;
                        continue;
                    }
                    if token == CToken::Enum {
                        capture_type_ =
                            session.descend(38);
                        state = 2usize;
                        continue;
                    }
                    site = session.site_ref(0);
                    let recovered = lady_deirdre::syntax::Recovery::recover(
                        &lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_6,
                    );
                    let end_site = session.site_ref(0);
                    session.error(
                        SyntaxError {
                            span: site..end_site,
                            context: 3,
                            expected_tokens: &lady_deirdre::lexis::EMPTY_TOKEN_SET,
                            expected_rules: &RULES_2,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                2usize => {
                    let token = session.token(0);
                    if token == CToken::Star {
                        capture_ptr.push(session.token_ref(0));
                        session.advance();
                        continue;
                    }
                    if token == CToken::Ident {
                        capture_name = session.token_ref(0);
                        session.advance();
                        state = 11usize;
                        continue;
                    }
                    if token == CToken::POpen {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 12usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_2,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_2,
                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                3usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if token == CToken::Ident {
                        capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0);
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 4usize;
                        continue;
                    }
                    if token == CToken::PClose {
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &TOKENS_7,
                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 8usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_7,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_7,
                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                4usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if token == CToken::POpen {
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &TOKENS_1,
                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 5usize;
                        continue;
                    }
                    if token == CToken::PClose {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 8usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_1,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_1,
                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                5usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if TokenSet::contains(&TOKENS_3, token as u8) {
                        ::std::vec::Vec::push(
                            &mut capture_args,
                            ::lady_deirdre::syntax::SyntaxSession::descend(session, 32u16),
                        );
                        state = 6usize;
                        continue;
                    }
                    if token == CToken::PClose {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 7usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_9,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_1,
                            expected_rules: &RULES_1,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                6usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if token == CToken::PClose {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 7usize;
                        continue;
                    }
                    if token == CToken::Comma {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 10usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_4,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_4,
                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                7usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if TokenSet::contains(&TOKENS_10, token as u8) {
                        capture_next =
                            ::lady_deirdre::syntax::SyntaxSession::descend(session, 4u16);
                        break;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_10,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                            expected_rules: &RULES_3,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                8usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if token == CToken::POpen {
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 5usize;
                        continue;
                    }
                    if TokenSet::contains(&TOKENS_3, token as u8) {
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &TOKENS_8,
                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                        ::std::vec::Vec::push(
                            &mut capture_args,
                            ::lady_deirdre::syntax::SyntaxSession::descend(session, 32u16),
                        );
                        state = 6usize;
                        continue;
                    }
                    if token == CToken::PClose {
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &TOKENS_8,
                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 7usize;
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_8,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_8,
                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                10usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if TokenSet::contains(&TOKENS_3, token as u8) {
                        ::std::vec::Vec::push(
                            &mut capture_args,
                            ::lady_deirdre::syntax::SyntaxSession::descend(session, 32u16),
                        );
                        state = 6usize;
                        continue;
                    }
                    if token == CToken::PClose {
                        ::std::vec::Vec::push(
                            &mut capture_args,
                            ::lady_deirdre::syntax::NodeRef::nil(),
                        );
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                expected_rules: &RULES_1,
                            },
                        );
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        state = 7usize;
                        continue;
                    }
                    if token == CToken::Comma {
                        ::std::vec::Vec::push(
                            &mut capture_args,
                            ::lady_deirdre::syntax::NodeRef::nil(),
                        );
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                expected_rules: &RULES_1,
                            },
                        );
                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                        continue;
                    }
                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                        &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_3,
                    );
                    let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                    ::lady_deirdre::syntax::SyntaxSession::error(
                        session,
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                            expected_rules: &RULES_1,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                11usize => {
                    let token = ::lady_deirdre::lexis::TokenCursor::token(session, 0);
                    if TokenSet::contains(&TOKENS_10, token as u8) {
                        capture_next =
                            ::lady_deirdre::syntax::SyntaxSession::descend(session, 4u16);
                        break;
                    }
                    if token == CToken::POpen {
                        capture_next =
                            session.descend(31u16);
                        break;
                    }
                    site = session.site_ref(0);
                    let mut recovered = lady_deirdre::syntax::Recovery::recover(
                        &lady_deirdre::syntax::UNLIMITED_RECOVERY,
                        session,
                        &TOKENS_12,
                    );
                    let end_site = session.site_ref(0);
                        session.error(
                        SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &lady_deirdre::lexis::EMPTY_TOKEN_SET,
                            expected_rules: &RULES_4,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                12 => {
                    let token = session.token(0);
                    if token == CToken::Star {
                        capture_func = session.token_ref(0);
                        session.advance();
                        state = 3;
                        continue;
                    }
                    if token == CToken::Ident {
                        site = session.site_ref(0);
                            session.error(
                            SyntaxError {
                                span: site..site,
                                context: 3u16,
                                expected_tokens: &TOKENS_11,
                                expected_rules: &lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                        capture_name = session.token_ref(0);
                        session.advance();
                        state = 4;
                        continue;
                    }
                    site = session.site_ref(0);
                    let mut recovered = 
                        lady_deirdre::syntax::UNLIMITED_RECOVERY.recover(
                        session,
                        &TOKENS_11,
                    );
                    let end_site = session.site_ref(0);
                
                        session.error(
                        ::lady_deirdre::syntax::SyntaxError {
                            span: site..end_site,
                            context: 3u16,
                            expected_tokens: &TOKENS_11,
                            expected_rules: &lady_deirdre::syntax::EMPTY_RULE_SET,
                        },
                    );
                    if !recovered {
                        break;
                    }
                }
                other => unreachable!("Unknown state {other}."),
            }
        }
        CNode::Definition {
            type_: capture_type_,
            ptr: capture_ptr,
            func: capture_func,
            name: capture_name,
            args: capture_args,
            next: capture_next,
        }
    }
}
