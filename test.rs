#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod syntax_cpp {
    pub mod lexis {
        use core::fmt::Display;
        use lady_deirdre::lexis::Token;
        pub enum CppToken {
            #[precedence(2)]
            #[rule("auto")]
            Auto,
            #[precedence(2)]
            #[rule("break")]
            Break,
            #[precedence(2)]
            #[rule("case")]
            Case,
            #[precedence(2)]
            #[rule("catch")]
            Catch,
            #[precedence(2)]
            #[rule("class")]
            Class,
            #[precedence(2)]
            #[rule("const")]
            Const,
            #[precedence(3)]
            #[rule("consteval")]
            ConstEval,
            #[precedence(3)]
            #[rule("constexpr")]
            ConstExpr,
            #[precedence(3)]
            #[rule("constinit")]
            ConstInit,
            #[precedence(2)]
            #[rule("continue")]
            Continue,
            #[precedence(2)]
            #[rule("decltype")]
            DeclType,
            #[precedence(2)]
            #[rule("default")]
            Default,
            #[precedence(2)]
            #[rule("delete")]
            Delete,
            #[precedence(2)]
            #[rule("do")]
            Do,
            #[precedence(3)]
            #[rule("char"|"double"|"float"|"int"|"void")]
            BasicType,
            #[precedence(2)]
            #[rule("else")]
            Else,
            #[precedence(2)]
            #[rule("enum")]
            Enum,
            #[precedence(2)]
            #[rule("explicit")]
            Explicit,
            #[precedence(2)]
            #[rule("export")]
            Export,
            #[precedence(2)]
            #[rule("extern")]
            Extern,
            #[precedence(2)]
            #[rule("final")]
            Final,
            #[precedence(2)]
            #[rule("for")]
            For,
            #[precedence(2)]
            #[rule("friend")]
            Friend,
            #[precedence(2)]
            #[rule("goto")]
            Goto,
            #[precedence(2)]
            #[rule("if")]
            If,
            #[precedence(2)]
            #[rule("inline")]
            Inline,
            #[precedence(2)]
            #[rule("namespace")]
            Namespace,
            #[precedence(2)]
            #[rule("new")]
            New,
            #[precedence(2)]
            #[rule("noexcept")]
            Noexpect,
            #[precedence(2)]
            #[rule("operator")]
            Operator,
            #[precedence(2)]
            #[rule("override")]
            Override,
            #[precedence(2)]
            #[rule("private")]
            Private,
            #[precedence(2)]
            #[rule("protected")]
            Protected,
            #[precedence(2)]
            #[rule("public")]
            Public,
            #[precedence(2)]
            #[rule("long"|"short"|"signed"|"unsigned")]
            TypeMod,
            #[precedence(2)]
            #[rule("register")]
            Register,
            #[precedence(2)]
            #[rule("requires")]
            Requires,
            #[precedence(2)]
            #[rule("return")]
            Return,
            #[precedence(2)]
            #[rule("static")]
            Static,
            #[precedence(2)]
            #[rule("struct")]
            Struct,
            #[precedence(2)]
            #[rule("switch")]
            Switch,
            #[precedence(2)]
            #[rule("template")]
            Template,
            #[precedence(2)]
            #[rule("throw")]
            Throw,
            #[precedence(2)]
            #[rule("try")]
            Try,
            #[precedence(2)]
            #[rule("typedef")]
            Typedef,
            #[precedence(2)]
            #[rule("union")]
            Union,
            #[precedence(2)]
            #[rule("using")]
            Using,
            #[precedence(2)]
            #[rule("virtual")]
            Virtual,
            #[precedence(2)]
            #[rule("volatile")]
            Volatile,
            #[precedence(2)]
            #[rule("while")]
            While,
            #[precedence(3)]
            #[rule("0b"&['0', '1']+&('L'|'U'|'l'|'u')*)]
            BinNumber,
            #[precedence(3)]
            #[rule("0o"&['0'..'7']+&('L'|'U'|'l'|'u')*)]
            OctNumber,
            #[rule(['0'..'9']+&('.'&['0'..'9']+)?&('L'|'U'|'l'|'u'|'f'|'F'|'d'|'D')*)]
            DecNumber,
            #[precedence(3)]
            #[rule("0x"&['0'..'9', 'A'..'F', 'a'..'f']+&('L'|'U'|'l'|'u')*)]
            HexNumber,
            #[rule([' ', '\t', '\r', '\x0b', '\x0c', '\n']+)]
            Whitespace,
            #[precedence(3)]
            #[rule('(')]
            ParenthesisOpen,
            #[precedence(3)]
            #[rule(')')]
            ParenthesisClose,
            #[precedence(3)]
            #[rule('{')]
            BraceOpen,
            #[rule('}')]
            BraceClose,
            #[rule('[')]
            BracketOpen,
            #[rule(']')]
            BracketClose,
            #[rule(',')]
            Comma,
            #[rule('.')]
            Point,
            #[rule(':')]
            Colon,
            #[rule("::")]
            DColon,
            #[rule(';')]
            Semicolon,
            #[rule(
                '\''&(
                    ^['\\',
                    '\'']|(
                        '\\'&^['x']
                    )|(
                        "\\x"&['0'..'9',
                        'A'..'F',
                        'a'..'f']&['0'..'9',
                        'A'..'F',
                        'a'..'f']
                    )
                )*&'\''?
            )]
            Char,
            #[rule(
                '"'&(
                    ^['\\',
                    '"']|(
                        '\\'&^['x']
                    )|(
                        "\\x"&['0'..'9',
                        'A'..'F',
                        'a'..'f']&['0'..'9',
                        'A'..'F',
                        'a'..'f']
                    )
                )*&'"'?
            )]
            String,
            #[precedence(2)]
            #[rule('!'|"not"|"compl")]
            UnOp,
            #[rule(['-', '+'])]
            MultiOp,
            #[precedence(3)]
            #[rule(
                ['%',
                '^',
                '|',
                '/']|"bitand"|"bitor"|"xor"|"and"|"or"|"||"|"&&"|"=="|"!="|">="|"<="
            )]
            BinOp,
            #[rule('<')]
            Less,
            #[rule('>')]
            Greater,
            #[rule('=')]
            Set,
            #[precedence(3)]
            #[rule("++"|"--")]
            IncDec,
            #[rule('*')]
            Star,
            #[rule('&')]
            Amp,
            #[rule('\\')]
            Backslash,
            #[rule('?')]
            QuestMark,
            #[rule('#')]
            Hash,
            #[precedence(4)]
            #[rule((['+', '-', '/', '*', '^', '|', '%', '&']&'=')|"<<="|">>=")]
            SetOp,
            #[precedence(5)]
            #[rule(
                (
                    "//"&(^['\\', '\n']|("\\"&.?))*&'\n'?
                )|("/*"&(^['*']|('*'&^['/']?))&"*/"?)
            )]
            Comment,
            #[rule(
                ^['!',
                '@',
                '#',
                '$',
                '%',
                '^',
                '&',
                '*',
                '(',
                ')',
                '-',
                '+',
                '=',
                '[',
                ']',
                '{',
                '}',
                '\\',
                '|',
                '\'',
                '"',
                ';',
                ':',
                '/',
                '?',
                ',',
                '.',
                '<',
                '>',
                '`',
                '~',
                '0'..'9',
                ' ',
                '\t',
                '\r',
                '\x0b',
                '\x0c',
                '\n']&^['!',
                '@',
                '#',
                '$',
                '%',
                '^',
                '&',
                '*',
                '(',
                ')',
                '-',
                '+',
                '=',
                '[',
                ']',
                '{',
                '}',
                '\\',
                '|',
                '\'',
                '"',
                ';',
                ':',
                '/',
                '?',
                ',',
                '.',
                '<',
                '>',
                '`',
                '~',
                ' ',
                '\t',
                '\r',
                '\x0b',
                '\x0c',
                '\n']*
            )]
            Ident,
            #[rule('~')]
            Tilde,
            #[rule('@')]
            At,
            #[rule(['`', '$'])]
            Other,
            #[mismatch]
            Mismatch,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CppToken {
            #[inline]
            fn clone(&self) -> CppToken {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CppToken {}
        #[automatically_derived]
        impl ::core::fmt::Debug for CppToken {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        CppToken::Auto => "Auto",
                        CppToken::Break => "Break",
                        CppToken::Case => "Case",
                        CppToken::Catch => "Catch",
                        CppToken::Class => "Class",
                        CppToken::Const => "Const",
                        CppToken::ConstEval => "ConstEval",
                        CppToken::ConstExpr => "ConstExpr",
                        CppToken::ConstInit => "ConstInit",
                        CppToken::Continue => "Continue",
                        CppToken::DeclType => "DeclType",
                        CppToken::Default => "Default",
                        CppToken::Delete => "Delete",
                        CppToken::Do => "Do",
                        CppToken::BasicType => "BasicType",
                        CppToken::Else => "Else",
                        CppToken::Enum => "Enum",
                        CppToken::Explicit => "Explicit",
                        CppToken::Export => "Export",
                        CppToken::Extern => "Extern",
                        CppToken::Final => "Final",
                        CppToken::For => "For",
                        CppToken::Friend => "Friend",
                        CppToken::Goto => "Goto",
                        CppToken::If => "If",
                        CppToken::Inline => "Inline",
                        CppToken::Namespace => "Namespace",
                        CppToken::New => "New",
                        CppToken::Noexpect => "Noexpect",
                        CppToken::Operator => "Operator",
                        CppToken::Override => "Override",
                        CppToken::Private => "Private",
                        CppToken::Protected => "Protected",
                        CppToken::Public => "Public",
                        CppToken::TypeMod => "TypeMod",
                        CppToken::Register => "Register",
                        CppToken::Requires => "Requires",
                        CppToken::Return => "Return",
                        CppToken::Static => "Static",
                        CppToken::Struct => "Struct",
                        CppToken::Switch => "Switch",
                        CppToken::Template => "Template",
                        CppToken::Throw => "Throw",
                        CppToken::Try => "Try",
                        CppToken::Typedef => "Typedef",
                        CppToken::Union => "Union",
                        CppToken::Using => "Using",
                        CppToken::Virtual => "Virtual",
                        CppToken::Volatile => "Volatile",
                        CppToken::While => "While",
                        CppToken::BinNumber => "BinNumber",
                        CppToken::OctNumber => "OctNumber",
                        CppToken::DecNumber => "DecNumber",
                        CppToken::HexNumber => "HexNumber",
                        CppToken::Whitespace => "Whitespace",
                        CppToken::ParenthesisOpen => "ParenthesisOpen",
                        CppToken::ParenthesisClose => "ParenthesisClose",
                        CppToken::BraceOpen => "BraceOpen",
                        CppToken::BraceClose => "BraceClose",
                        CppToken::BracketOpen => "BracketOpen",
                        CppToken::BracketClose => "BracketClose",
                        CppToken::Comma => "Comma",
                        CppToken::Point => "Point",
                        CppToken::Colon => "Colon",
                        CppToken::DColon => "DColon",
                        CppToken::Semicolon => "Semicolon",
                        CppToken::Char => "Char",
                        CppToken::String => "String",
                        CppToken::UnOp => "UnOp",
                        CppToken::MultiOp => "MultiOp",
                        CppToken::BinOp => "BinOp",
                        CppToken::Less => "Less",
                        CppToken::Greater => "Greater",
                        CppToken::Set => "Set",
                        CppToken::IncDec => "IncDec",
                        CppToken::Star => "Star",
                        CppToken::Amp => "Amp",
                        CppToken::Backslash => "Backslash",
                        CppToken::QuestMark => "QuestMark",
                        CppToken::Hash => "Hash",
                        CppToken::SetOp => "SetOp",
                        CppToken::Comment => "Comment",
                        CppToken::Ident => "Ident",
                        CppToken::Tilde => "Tilde",
                        CppToken::At => "At",
                        CppToken::Other => "Other",
                        CppToken::Mismatch => "Mismatch",
                    },
                )
            }
        }
        impl ::lady_deirdre::lexis::Token for CppToken {
            fn new(session: &mut impl ::lady_deirdre::lexis::LexisSession) -> Self {
                #[allow(unused_mut)]
                let mut state = 1usize;
                #[allow(unused_mut)]
                let mut token = 0usize;
                loop {
                    match state {
                        1usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='F' | 'L' | 'U' | 'h' | 'k' | 'm' | 'q' | 'y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '+' => {
                                    state = 12usize;
                                    continue;
                                }
                                '"' => {
                                    state = 23usize;
                                    continue;
                                }
                                's' => {
                                    state = 44usize;
                                    continue;
                                }
                                '|' => {
                                    state = 49usize;
                                    continue;
                                }
                                'i' => {
                                    state = 57usize;
                                    continue;
                                }
                                't' => {
                                    state = 60usize;
                                    continue;
                                }
                                'o' => {
                                    state = 89usize;
                                    continue;
                                }
                                '{' => {
                                    state = 98usize;
                                    continue;
                                }
                                'u' => {
                                    state = 102usize;
                                    continue;
                                }
                                '}' => {
                                    state = 129usize;
                                    continue;
                                }
                                '%' | '^' => {
                                    state = 134usize;
                                    continue;
                                }
                                '@' => {
                                    state = 149usize;
                                    continue;
                                }
                                '&' => {
                                    state = 163usize;
                                    continue;
                                }
                                '*' => {
                                    state = 180usize;
                                    continue;
                                }
                                'b' => {
                                    state = 182usize;
                                    continue;
                                }
                                '1'..='9' => {
                                    state = 185usize;
                                    continue;
                                }
                                ')' => {
                                    state = 190usize;
                                    continue;
                                }
                                'n' => {
                                    state = 195usize;
                                    continue;
                                }
                                '\t'..='\r' | ' ' => {
                                    state = 196usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 197usize;
                                    continue;
                                }
                                ';' => {
                                    state = 198usize;
                                    continue;
                                }
                                '!' => {
                                    state = 199usize;
                                    continue;
                                }
                                '.' => {
                                    state = 200usize;
                                    continue;
                                }
                                '[' => {
                                    state = 201usize;
                                    continue;
                                }
                                'd' => {
                                    state = 202usize;
                                    continue;
                                }
                                'v' => {
                                    state = 203usize;
                                    continue;
                                }
                                '=' => {
                                    state = 204usize;
                                    continue;
                                }
                                ']' => {
                                    state = 205usize;
                                    continue;
                                }
                                '(' => {
                                    state = 206usize;
                                    continue;
                                }
                                'f' => {
                                    state = 207usize;
                                    continue;
                                }
                                '#' => {
                                    state = 208usize;
                                    continue;
                                }
                                'a' => {
                                    state = 209usize;
                                    continue;
                                }
                                '~' => {
                                    state = 210usize;
                                    continue;
                                }
                                '>' => {
                                    state = 211usize;
                                    continue;
                                }
                                ',' => {
                                    state = 212usize;
                                    continue;
                                }
                                'c' => {
                                    state = 213usize;
                                    continue;
                                }
                                '-' => {
                                    state = 214usize;
                                    continue;
                                }
                                '$' | '`' => {
                                    state = 215usize;
                                    continue;
                                }
                                'x' => {
                                    state = 216usize;
                                    continue;
                                }
                                'l' => {
                                    state = 217usize;
                                    continue;
                                }
                                '?' => {
                                    state = 218usize;
                                    continue;
                                }
                                '/' => {
                                    state = 219usize;
                                    continue;
                                }
                                'r' => {
                                    state = 220usize;
                                    continue;
                                }
                                '<' => {
                                    state = 221usize;
                                    continue;
                                }
                                ':' => {
                                    state = 222usize;
                                    continue;
                                }
                                'w' => {
                                    state = 223usize;
                                    continue;
                                }
                                'e' => {
                                    state = 224usize;
                                    continue;
                                }
                                '\'' => {
                                    state = 225usize;
                                    continue;
                                }
                                '0' => {
                                    state = 226usize;
                                    continue;
                                }
                                'g' => {
                                    state = 227usize;
                                    continue;
                                }
                                'p' => {
                                    state = 228usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        2usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 4usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        3usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        4usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='f'
                                | 'h'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 112usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        6usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 7usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        7usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 18usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        8usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 9usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        9usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 79usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        10usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 11usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        11usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 241usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        12usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 70usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                '+' => {
                                    state = 14usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        13usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 81usize;
                        }
                        14usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 75usize;
                        }
                        15usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 3usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        16usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 17usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        17usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 8usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        18usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 19usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        19usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 162usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        20usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r'
                                | ' '..=')'
                                | '+'..='.'
                                | '0'..='F'
                                | 'L'
                                | 'U'
                                | '['..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 21usize;
                                    continue;
                                }
                                '*' => {
                                    state = 22usize;
                                    continue;
                                }
                                '\0' | '/' => break,
                                _ => {
                                    state = 21usize;
                                    continue;
                                }
                            }
                        }
                        21usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '*' => {
                                    state = 280usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        22usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '/' => {
                                    state = 275usize;
                                    continue;
                                }
                                '*' => {
                                    state = 280usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        23usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 68usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r'
                                | ' '..='!'
                                | '#'..='F'
                                | 'L'
                                | 'U'
                                | '['
                                | ']'..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 23usize;
                                    continue;
                                }
                                '"' => {
                                    state = 24usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 25usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 23usize;
                                    continue;
                                }
                            }
                        }
                        24usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 68usize;
                        }
                        25usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r'
                                | ' '..='F'
                                | 'L'
                                | 'U'
                                | '['..='^'
                                | '`'..='i'
                                | 'k'..='w'
                                | 'y'
                                | '{'..='~' => {
                                    state = 23usize;
                                    continue;
                                }
                                'x' => {
                                    state = 235usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 23usize;
                                    continue;
                                }
                            }
                        }
                        26usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='l'
                                | 'n'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 27usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        27usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 17usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        28usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 29usize;
                                    continue;
                                }
                                'i' => {
                                    state = 30usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        29usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 279usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        30usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 140usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        31usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 46usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        32usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 2usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        33usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 34usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        34usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 308usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        35usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 36usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        36usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 242usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        37usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 38usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        38usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 35usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        39usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '*' => {
                                    state = 20usize;
                                    continue;
                                }
                                '\t'..='\r'
                                | ' '..=')'
                                | '+'..='F'
                                | 'L'
                                | 'U'
                                | '['..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 21usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 21usize;
                                    continue;
                                }
                            }
                        }
                        40usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 6usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        41usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 42usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        42usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 101usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        43usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 29usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        44usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='g'
                                | 'k'..='s'
                                | 'u'..='v'
                                | 'x'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 45usize;
                                    continue;
                                }
                                'w' => {
                                    state = 46usize;
                                    continue;
                                }
                                't' => {
                                    state = 47usize;
                                    continue;
                                }
                                'i' => {
                                    state = 48usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        45usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 328usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        46usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 229usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        47usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 191usize;
                                    continue;
                                }
                                'a' => {
                                    state = 239usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        48usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='f'
                                | 'h'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 267usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        49usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                '|' => {
                                    state = 50usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        50usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                        }
                        51usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 52usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        52usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 306usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        53usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='e'
                                | 'g'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'f' => {
                                    state = 54usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        54usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 45usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        55usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 56usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        56usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'l'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'k' => {
                                    state = 32usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        57usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='e'
                                | 'g'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'f' => {
                                    state = 58usize;
                                    continue;
                                }
                                'n' => {
                                    state = 59usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        58usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 25usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        59usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 139usize;
                                    continue;
                                }
                                't' => {
                                    state = 140usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        60usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='g'
                                | 'i'
                                | 'k'..='q'
                                | 's'..='x' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 61usize;
                                    continue;
                                }
                                'r' => {
                                    state = 62usize;
                                    continue;
                                }
                                'y' => {
                                    state = 63usize;
                                    continue;
                                }
                                'h' => {
                                    state = 64usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        61usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='l'
                                | 'n'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 335usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        62usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='x' => {
                                    state = 3usize;
                                    continue;
                                }
                                'y' => {
                                    state = 188usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        63usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 133usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        64usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 119usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        65usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 19usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        66usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 50usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        67usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 8usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        68usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 69usize;
                                    continue;
                                }
                                'o' => {
                                    state = 70usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        69usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 333usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        70usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 325usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        71usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 72usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        72usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 295usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        73usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 74usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        74usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 271usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        75usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 51usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='1' => {
                                    state = 75usize;
                                    continue;
                                }
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 76usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        76usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 51usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 76usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        77usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 78usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        78usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 181usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        79usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 284usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        80usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 81usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        81usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 177usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        82usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 39usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        83usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 84usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        84usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 82usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        85usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 86usize;
                                    continue;
                                }
                                'a' => {
                                    state = 87usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        86usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 92usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        87usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 234usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        88usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 7usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        89usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'
                                | 's'..='u'
                                | 'w'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 90usize;
                                    continue;
                                }
                                'v' => {
                                    state = 91usize;
                                    continue;
                                }
                                'r' => {
                                    state = 92usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        90usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 327usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        91usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 67usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        92usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        93usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 94usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        94usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='g'
                                | 'i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 309usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        95usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 34usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        96usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 97usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        97usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 27usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        98usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 58usize;
                        }
                        99usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 100usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        100usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 141usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        101usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 254usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        102usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 103usize;
                                    continue;
                                }
                                's' => {
                                    state = 104usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        103usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 194usize;
                                    continue;
                                }
                                'i' => {
                                    state = 318usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        104usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'i' => {
                                    state = 2usize;
                                    continue;
                                }
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        105usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        106usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 53usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        107usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 43usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        108usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 109usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        109usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='x' => {
                                    state = 3usize;
                                    continue;
                                }
                                'y' => {
                                    state = 115usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        110usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 111usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        111usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 264usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        112usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 47usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        113usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 114usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        114usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 225usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        115usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 116usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        116usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 240usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        117usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='u'
                                | 'w'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'v' => {
                                    state = 118usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        118usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 287usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        119usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 135usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        120usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 20usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        121usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 122usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        122usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 42usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        123usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='f'
                                | 'h'..='i'
                                | 'k'..='p'
                                | 'r'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 124usize;
                                    continue;
                                }
                                'g' => {
                                    state = 125usize;
                                    continue;
                                }
                                'q' => {
                                    state = 126usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        124usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 33usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        125usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 137usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        126usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 51usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        127usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\\' => {
                                    state = 127usize;
                                    continue;
                                }
                                '\t'..='\r'
                                | ' '..='F'
                                | 'L'
                                | 'U'
                                | '['
                                | ']'..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 128usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 128usize;
                                    continue;
                                }
                            }
                        }
                        128usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\\' => {
                                    state = 127usize;
                                    continue;
                                }
                                '\t'
                                | '\u{b}'..='\r'
                                | ' '..='F'
                                | 'L'
                                | 'U'
                                | '['
                                | ']'..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 128usize;
                                    continue;
                                }
                                '\n' => {
                                    state = 275usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 128usize;
                                    continue;
                                }
                            }
                        }
                        129usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 59usize;
                        }
                        130usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'
                                | 'c'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'b' => {
                                    state = 131usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        131usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 289usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        132usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 32usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        133usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 326usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        134usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        135usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='v'
                                | 'x'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'w' => {
                                    state = 107usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        136usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 30usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        137usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 138usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        138usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 245usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        139usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 258usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        140usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 15usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        141usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 121usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        142usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 143usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        143usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 66usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        144usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 145usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        145usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='g'
                                | 'i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 307usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        146usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='v'
                                | 'x'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'w' => {
                                    state = 147usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        147usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 28usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        148usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 142usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        149usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 85usize;
                        }
                        150usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='f'
                                | 'h'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 38usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        151usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='7' => {
                                    state = 152usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        152usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 52usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='7' => {
                                    state = 152usize;
                                    continue;
                                }
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 189usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        153usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'
                                | 'c'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'b' => {
                                    state = 154usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        154usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 276usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        155usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 156usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        156usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 290usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        157usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 158usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        158usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 22usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        159usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 24usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        160usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='l'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 77usize;
                                    continue;
                                }
                                'n' => {
                                    state = 161usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        161usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 304usize;
                                    continue;
                                }
                                't' => {
                                    state = 323usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        162usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 48usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        163usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 77usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                '&' => {
                                    state = 50usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        164usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 165usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        165usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 88usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        166usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 167usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        167usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 140usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        168usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 169usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        169usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 33usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        170usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 49usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        171usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 172usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        172usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 319usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        173usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='e'
                                | 'g'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 174usize;
                                    continue;
                                }
                                'l' => {
                                    state = 175usize;
                                    continue;
                                }
                                'f' => {
                                    state = 176usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        174usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 108usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        175usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 80usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        176usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 251usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        177usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 13usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        178usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 179usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        179usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 16usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        180usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 76usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        181usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 69usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        182usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 183usize;
                                    continue;
                                }
                                'r' => {
                                    state = 184usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        183usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 85usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        184usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 55usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        185usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' => {
                                    state = 185usize;
                                    continue;
                                }
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 186usize;
                                    continue;
                                }
                                '.' => {
                                    state = 187usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        186usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 186usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        187usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' => {
                                    state = 274usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        188usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 44usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        189usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 52usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 189usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        190usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 57usize;
                        }
                        191usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 192usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        192usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 315usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        193usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 14usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 130usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        194usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 48usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        195usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='d'
                                | 'f'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 146usize;
                                    continue;
                                }
                                'o' => {
                                    state = 302usize;
                                    continue;
                                }
                                'a' => {
                                    state = 330usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        196usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 55usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r' | ' ' => {
                                    state = 196usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        197usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 78usize;
                        }
                        198usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 66usize;
                        }
                        199usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 69usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 50usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        200usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 63usize;
                        }
                        201usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 60usize;
                        }
                        202usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 173usize;
                                    continue;
                                }
                                'o' => {
                                    state = 193usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        203usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 28usize;
                                    continue;
                                }
                                'i' => {
                                    state = 40usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        204usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 74usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 50usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        205usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 61usize;
                        }
                        206usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 56usize;
                        }
                        207usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'
                                | 'm'..='n'
                                | 'p'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 110usize;
                                    continue;
                                }
                                'o' => {
                                    state = 157usize;
                                    continue;
                                }
                                'i' => {
                                    state = 270usize;
                                    continue;
                                }
                                'r' => {
                                    state = 314usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        208usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 80usize;
                        }
                        209usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 71usize;
                                    continue;
                                }
                                'n' => {
                                    state = 234usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        210usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                        }
                        211usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 73usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 50usize;
                                    continue;
                                }
                                '>' => {
                                    state = 105usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        212usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 62usize;
                        }
                        213usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='g'
                                | 'i'
                                | 'k'
                                | 'm'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 160usize;
                                    continue;
                                }
                                'h' => {
                                    state = 166usize;
                                    continue;
                                }
                                'l' => {
                                    state = 249usize;
                                    continue;
                                }
                                'a' => {
                                    state = 250usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        214usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 70usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                '-' => {
                                    state = 14usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        215usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 86usize;
                        }
                        216usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 86usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        217usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 310usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        218usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 79usize;
                        }
                        219usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 13usize;
                                    continue;
                                }
                                '*' => {
                                    state = 39usize;
                                    continue;
                                }
                                '/' => {
                                    state = 128usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        220usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 123usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        221usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 50usize;
                                    continue;
                                }
                                '<' => {
                                    state = 105usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        222usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 64usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                ':' => {
                                    state = 293usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        223usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='g'
                                | 'i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 148usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        224usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'
                                | 'o'..='w'
                                | 'y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'x' => {
                                    state = 255usize;
                                    continue;
                                }
                                'n' => {
                                    state = 268usize;
                                    continue;
                                }
                                'l' => {
                                    state = 269usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        225usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 67usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r'
                                | ' '..='&'
                                | '('..='F'
                                | 'L'
                                | 'U'
                                | '['
                                | ']'..='^'
                                | '`'..='i'
                                | 'k'..='y'
                                | '{'..='~' => {
                                    state = 225usize;
                                    continue;
                                }
                                '\'' => {
                                    state = 321usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 324usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 225usize;
                                    continue;
                                }
                            }
                        }
                        226usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'o' => {
                                    state = 151usize;
                                    continue;
                                }
                                '0'..='9' => {
                                    state = 185usize;
                                    continue;
                                }
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 186usize;
                                    continue;
                                }
                                '.' => {
                                    state = 187usize;
                                    continue;
                                }
                                'b' => {
                                    state = 253usize;
                                    continue;
                                }
                                'x' => {
                                    state = 286usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        227usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 244usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        228usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 153usize;
                                    continue;
                                }
                                'r' => {
                                    state = 278usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        229usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 144usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        230usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 231usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        231usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 313usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        232usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 232usize;
                                    continue;
                                }
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 233usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        233usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 233usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        234usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 92usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        235usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 248usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        236usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 237usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        237usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 317usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        238usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 73usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        239usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 83usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        240usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 11usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        241usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 261usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        242usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 243usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        243usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 23usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        244usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 291usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        245usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 246usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        246usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 294usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        247usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 10usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        248usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 23usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        249usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 236usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        250usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 93usize;
                                    continue;
                                }
                                's' => {
                                    state = 320usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        251usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 252usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        252usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'
                                | 'm'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 265usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        253usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='1' => {
                                    state = 75usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        254usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 43usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        255usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 68usize;
                                    continue;
                                }
                                't' => {
                                    state = 256usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        256usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 334usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        257usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 120usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        258usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 259usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        259usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 260usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        260usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 26usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        261usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 168usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        262usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 263usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        263usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 272usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        264usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 140usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        265usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 266usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        266usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 12usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        267usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 311usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        268usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 26usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        269usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 178usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        270usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 171usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        271usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 9usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        272usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 96usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        273usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 38usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        274usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 186usize;
                                    continue;
                                }
                                '0'..='9' => {
                                    state = 274usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        275usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                        }
                        276usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 292usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        277usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'b'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 230usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        278usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 117usize;
                                    continue;
                                }
                                'o' => {
                                    state = 247usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        279usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 155usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        280usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '/' => {
                                    state = 275usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        281usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 31usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        282usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='u'
                                | 'w'
                                | 'y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'v' => {
                                    state = 164usize;
                                    continue;
                                }
                                'x' => {
                                    state = 283usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        283usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 16usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        284usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 285usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        285usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 31usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        286usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 232usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        287usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 288usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        288usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 132usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        289usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 140usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        290usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 170usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        291usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 159usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        292usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 95usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        293usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 65usize;
                        }
                        294usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 36usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        295usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 1usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        296usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 297usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        297usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 37usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        298usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 299usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        299usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 312usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        300usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 301usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        301usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 10usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        302usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 181usize;
                                    continue;
                                }
                                'e' => {
                                    state = 303usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        303usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='w'
                                | 'y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'x' => {
                                    state = 41usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        304usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 305usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        305usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 6usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 238usize;
                                    continue;
                                }
                                'e' => {
                                    state = 282usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        306usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 296usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        307usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 41usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        308usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 38usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        309usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 4usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        310usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 150usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        311usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 37usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        312usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 18usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        313usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 136usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        314usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 35usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        315usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 316usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        316usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 40usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        317usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 5usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        318usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='n'
                                | 'p'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 281usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        319usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 21usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        320usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 15usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        321usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 67usize;
                        }
                        322usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='t'
                                | 'v'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 300usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        323usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='h'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 329usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        324usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'x' => {
                                    state = 113usize;
                                    continue;
                                }
                                '\t'..='\r'
                                | ' '..='F'
                                | 'L'
                                | 'U'
                                | '['..='^'
                                | '`'..='i'
                                | 'k'..='w'
                                | 'y'
                                | '{'..='~' => {
                                    state = 225usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 225usize;
                                    continue;
                                }
                            }
                        }
                        325usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='s'
                                | 'u'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 65usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        326usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='c'
                                | 'e'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 106usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        327usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 277usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        328usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 273usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        329usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='m'
                                | 'o'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 322usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        330usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='l'
                                | 'n'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 331usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        331usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='d'
                                | 'f'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 332usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        332usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='r'
                                | 't'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 262usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        333usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='b'
                                | 'd'..='i'
                                | 'k'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 298usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        334usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='q'
                                | 's'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 257usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        335usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9'
                                | 'A'..='F'
                                | 'L'
                                | 'U'
                                | 'a'..='i'
                                | 'k'..='o'
                                | 'q'..='y' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 99usize;
                                    continue;
                                }
                                '\0'
                                | '\t'..='\r'
                                | ' '..='/'
                                | ':'..='@'
                                | '['..='^'
                                | '`'
                                | '{'..='~' => break,
                                _ => {
                                    state = 3usize;
                                    continue;
                                }
                            }
                        }
                        _ => {}
                    }
                    break;
                }
                match token {
                    1usize => Self::Auto,
                    2usize => Self::Break,
                    3usize => Self::Case,
                    4usize => Self::Catch,
                    5usize => Self::Class,
                    6usize => Self::Const,
                    7usize => Self::ConstEval,
                    8usize => Self::ConstExpr,
                    9usize => Self::ConstInit,
                    10usize => Self::Continue,
                    11usize => Self::DeclType,
                    12usize => Self::Default,
                    13usize => Self::Delete,
                    14usize => Self::Do,
                    15usize => Self::BasicType,
                    16usize => Self::Else,
                    17usize => Self::Enum,
                    18usize => Self::Explicit,
                    19usize => Self::Export,
                    20usize => Self::Extern,
                    21usize => Self::Final,
                    22usize => Self::For,
                    23usize => Self::Friend,
                    24usize => Self::Goto,
                    25usize => Self::If,
                    26usize => Self::Inline,
                    27usize => Self::Namespace,
                    28usize => Self::New,
                    29usize => Self::Noexpect,
                    30usize => Self::Operator,
                    31usize => Self::Override,
                    32usize => Self::Private,
                    33usize => Self::Protected,
                    34usize => Self::Public,
                    35usize => Self::TypeMod,
                    36usize => Self::Register,
                    37usize => Self::Requires,
                    38usize => Self::Return,
                    39usize => Self::Static,
                    40usize => Self::Struct,
                    41usize => Self::Switch,
                    42usize => Self::Template,
                    43usize => Self::Throw,
                    44usize => Self::Try,
                    45usize => Self::Typedef,
                    46usize => Self::Union,
                    47usize => Self::Using,
                    48usize => Self::Virtual,
                    49usize => Self::Volatile,
                    50usize => Self::While,
                    51usize => Self::BinNumber,
                    52usize => Self::OctNumber,
                    53usize => Self::DecNumber,
                    54usize => Self::HexNumber,
                    55usize => Self::Whitespace,
                    56usize => Self::ParenthesisOpen,
                    57usize => Self::ParenthesisClose,
                    58usize => Self::BraceOpen,
                    59usize => Self::BraceClose,
                    60usize => Self::BracketOpen,
                    61usize => Self::BracketClose,
                    62usize => Self::Comma,
                    63usize => Self::Point,
                    64usize => Self::Colon,
                    65usize => Self::DColon,
                    66usize => Self::Semicolon,
                    67usize => Self::Char,
                    68usize => Self::String,
                    69usize => Self::UnOp,
                    70usize => Self::MultiOp,
                    71usize => Self::BinOp,
                    72usize => Self::Less,
                    73usize => Self::Greater,
                    74usize => Self::Set,
                    75usize => Self::IncDec,
                    76usize => Self::Star,
                    77usize => Self::Amp,
                    78usize => Self::Backslash,
                    79usize => Self::QuestMark,
                    80usize => Self::Hash,
                    81usize => Self::SetOp,
                    82usize => Self::Comment,
                    83usize => Self::Ident,
                    84usize => Self::Tilde,
                    85usize => Self::At,
                    86usize => Self::Other,
                    _ => Self::Mismatch,
                }
            }
        }
    }
    pub mod syntax {
        use super::lexis::CppToken;
        use lady_deirdre::{
            lexis::TokenRef, syntax::{Node, NodeRef, SyntaxError, SyntaxSession},
        };
        use std::vec::Vec;
        #[token(CppToken)]
        #[error(SyntaxError)]
        #[skip($Whitespace)]
        #[define(NUMBER = ($DecNumber|$HexNumber|$BinNumber|$OctNumber))]
        #[define(ACTION = (CodeBlock))]
        pub enum CppNode {
            #[root]
            #[rule(items:RootItem*)]
            Root { items: Vec<NodeRef> },
            #[rule(item:(BasicDef|StructDef|EnumDef|ClassDef|NamespaceDef))]
            RootItem { item: NodeRef },
            #[rule(((mods:$TypeMod)+&(type_:$BasicType)?)|(type_:$BasicType))]
            BasicType { mods: Vec<TokenRef>, type_: TokenRef },
            #[rule(
                (
                    (
                        (
                            mods:($Const|$Auto|$Static|$Volatile)
                        )&(type_:(BasicValue|Path))?
                    )|(type_:(BasicType|Path))
                )&(generic:GenericUse)?&(refer:($Amp|$Star)*)
            )]
            Type { auto: TokenRef, type_: NodeRef, refer: Vec<NodeRef> },
            #[rule((path:$Ident)+{$DColon})]
            Path { path: Vec<TokenRef> },
            #[rule(
                (
                    type_:Type
                )&(
                    (path:Path)|($Open&(refer:($Amp|$Star)*)&(path:Path)&$Close)
                )&(def:(FnDef|VarDef))
            )]
            BasicDef {
                type_: NodeRef,
                path: NodeRef,
                refer: Vec<TokenRef>,
                def: NodeRef,
            },
            #[rule($BracketOpen&(val:Value)?&$BracketClose)]
            ArrIndex { val: NodeRef },
            #[rule((arr:ArrIndex)?&($Set&(val:Value))?&$Semicolon)]
            VarDef { arr: NodeRef, value: NodeRef },
            #[rule(
                $Open&(
                    params:FnParameter
                )*{$Comma}&$Close&(
                    defs:BasicDef
                )*&(
                    (
                        ($Set&(val:(NUMBER|$String|$Char|$Delete)))?&$Semicolon
                    )|(block:CodeBlock)
                )
            )]
            FnDef {
                params: Vec<NodeRef>,
                defs: Vec<NodeRef>,
                block: NodeRef,
                val: TokenRef,
            },
            #[rule((type_:Type)&(name:$Ident)?)]
            FnParameter { name: TokenRef, type_: NodeRef },
            #[rule($Struct&(name:Path)&$BraceOpen&(fields:RootItem)*&$BraceClose)]
            StructDef { name: NodeRef, fields: Vec<NodeRef> },
            #[rule(
                $Enum&(
                    name:Path
                )&$BraceOpen&(items:EnumItem)*{$Comma}&$Comma?&$BraceClose
            )]
            EnumDef { name: NodeRef, items: Vec<NodeRef> },
            #[rule((name:$Ident)&($Set&(num:NUMBER))?)]
            EnumItem { name: TokenRef, num: TokenRef },
            #[rule(
                $Class&(
                    name:Path
                )&$BraceOpen&(fields:(RootItem|VisibleLabel))*&$BraceClose
            )]
            ClassDef { name: NodeRef, fields: Vec<NodeRef> },
            #[rule($BraceOpen&(actions:ACTION*)&$BraceClose)]
            CodeBlock { actions: Vec<NodeRef> },
            #[rule((name:($Public|$Protected|$Private))&$Colon)]
            VisibleLabel { name: TokenRef },
            #[rule($Namespace&(name:Path)&$BraceOpen&(items:RootItem)*&$BraceClose)]
            NamespaceDef { name: NodeRef, items: Vec<NodeRef> },
            #[rule(
                (
                    val:($BinOp|$MultiOp|$Star|$Amp)
                )|((val:$Less)&(add:$Less)?)|((val:$Greater)&(add:$Greater)?)
            )]
            BinOp { val: TokenRef, add: TokenRef },
            #[rule((values:SingleValue)*{ops:BinOp})]
            Value { values: Vec<NodeRef>, ops: Vec<NodeRef> },
            #[rule(val:(NUMBER|$String|$Char))]
            BasicValue { val: TokenRef },
            #[rule($Point&(name:$Ident)&((generic:GenericUse)?&(call:Call))?)]
            Method { name: TokenRef, call: NodeRef, generic: NodeRef },
            #[rule($Less&(types:Type)*{$Comma}&$Greater)]
            GenericUse { types: Vec<NodeRef> },
            #[rule($Open&(args:Value)*{$Comma}&$Close)]
            Call { args: Vec<NodeRef> },
            #[rule(val:$IncDec)]
            IncDec { val: TokenRef },
            #[rule(
                (
                    un_op:($UnOp|$MultiOp|$Star|$Amp|$IncDec)*
                )&(val:(BasicValue|ValueParenthesis))&(methods:(Method|ArrIndex|IncDec))*
            )]
            SingleValue { un_op: Vec<TokenRef>, val: NodeRef, methods: Vec<NodeRef> },
            #[rule($Open&(val:Value)&$Close)]
            ValueParenthesis { val: NodeRef },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CppNode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CppNode::Root { items: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Root",
                            "items",
                            &__self_0,
                        )
                    }
                    CppNode::RootItem { item: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "RootItem",
                            "item",
                            &__self_0,
                        )
                    }
                    CppNode::BasicType { mods: __self_0, type_: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "BasicType",
                            "mods",
                            __self_0,
                            "type_",
                            &__self_1,
                        )
                    }
                    CppNode::Type {
                        auto: __self_0,
                        type_: __self_1,
                        refer: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "Type",
                            "auto",
                            __self_0,
                            "type_",
                            __self_1,
                            "refer",
                            &__self_2,
                        )
                    }
                    CppNode::Path { path: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Path",
                            "path",
                            &__self_0,
                        )
                    }
                    CppNode::BasicDef {
                        type_: __self_0,
                        path: __self_1,
                        refer: __self_2,
                        def: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "BasicDef",
                            "type_",
                            __self_0,
                            "path",
                            __self_1,
                            "refer",
                            __self_2,
                            "def",
                            &__self_3,
                        )
                    }
                    CppNode::ArrIndex { val: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ArrIndex",
                            "val",
                            &__self_0,
                        )
                    }
                    CppNode::VarDef { arr: __self_0, value: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "VarDef",
                            "arr",
                            __self_0,
                            "value",
                            &__self_1,
                        )
                    }
                    CppNode::FnDef {
                        params: __self_0,
                        defs: __self_1,
                        block: __self_2,
                        val: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "FnDef",
                            "params",
                            __self_0,
                            "defs",
                            __self_1,
                            "block",
                            __self_2,
                            "val",
                            &__self_3,
                        )
                    }
                    CppNode::FnParameter { name: __self_0, type_: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "FnParameter",
                            "name",
                            __self_0,
                            "type_",
                            &__self_1,
                        )
                    }
                    CppNode::StructDef { name: __self_0, fields: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "StructDef",
                            "name",
                            __self_0,
                            "fields",
                            &__self_1,
                        )
                    }
                    CppNode::EnumDef { name: __self_0, items: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "EnumDef",
                            "name",
                            __self_0,
                            "items",
                            &__self_1,
                        )
                    }
                    CppNode::EnumItem { name: __self_0, num: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "EnumItem",
                            "name",
                            __self_0,
                            "num",
                            &__self_1,
                        )
                    }
                    CppNode::ClassDef { name: __self_0, fields: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "ClassDef",
                            "name",
                            __self_0,
                            "fields",
                            &__self_1,
                        )
                    }
                    CppNode::CodeBlock { actions: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "CodeBlock",
                            "actions",
                            &__self_0,
                        )
                    }
                    CppNode::VisibleLabel { name: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "VisibleLabel",
                            "name",
                            &__self_0,
                        )
                    }
                    CppNode::NamespaceDef { name: __self_0, items: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "NamespaceDef",
                            "name",
                            __self_0,
                            "items",
                            &__self_1,
                        )
                    }
                    CppNode::BinOp { val: __self_0, add: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "BinOp",
                            "val",
                            __self_0,
                            "add",
                            &__self_1,
                        )
                    }
                    CppNode::Value { values: __self_0, ops: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Value",
                            "values",
                            __self_0,
                            "ops",
                            &__self_1,
                        )
                    }
                    CppNode::BasicValue { val: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "BasicValue",
                            "val",
                            &__self_0,
                        )
                    }
                    CppNode::Method {
                        name: __self_0,
                        call: __self_1,
                        generic: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "Method",
                            "name",
                            __self_0,
                            "call",
                            __self_1,
                            "generic",
                            &__self_2,
                        )
                    }
                    CppNode::GenericUse { types: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "GenericUse",
                            "types",
                            &__self_0,
                        )
                    }
                    CppNode::Call { args: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Call",
                            "args",
                            &__self_0,
                        )
                    }
                    CppNode::IncDec { val: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "IncDec",
                            "val",
                            &__self_0,
                        )
                    }
                    CppNode::SingleValue {
                        un_op: __self_0,
                        val: __self_1,
                        methods: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "SingleValue",
                            "un_op",
                            __self_0,
                            "val",
                            __self_1,
                            "methods",
                            &__self_2,
                        )
                    }
                    CppNode::ValueParenthesis { val: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ValueParenthesis",
                            "val",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CppNode {
            #[inline]
            fn clone(&self) -> CppNode {
                match self {
                    CppNode::Root { items: __self_0 } => {
                        CppNode::Root {
                            items: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::RootItem { item: __self_0 } => {
                        CppNode::RootItem {
                            item: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::BasicType { mods: __self_0, type_: __self_1 } => {
                        CppNode::BasicType {
                            mods: ::core::clone::Clone::clone(__self_0),
                            type_: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::Type {
                        auto: __self_0,
                        type_: __self_1,
                        refer: __self_2,
                    } => {
                        CppNode::Type {
                            auto: ::core::clone::Clone::clone(__self_0),
                            type_: ::core::clone::Clone::clone(__self_1),
                            refer: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::Path { path: __self_0 } => {
                        CppNode::Path {
                            path: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::BasicDef {
                        type_: __self_0,
                        path: __self_1,
                        refer: __self_2,
                        def: __self_3,
                    } => {
                        CppNode::BasicDef {
                            type_: ::core::clone::Clone::clone(__self_0),
                            path: ::core::clone::Clone::clone(__self_1),
                            refer: ::core::clone::Clone::clone(__self_2),
                            def: ::core::clone::Clone::clone(__self_3),
                        }
                    }
                    CppNode::ArrIndex { val: __self_0 } => {
                        CppNode::ArrIndex {
                            val: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::VarDef { arr: __self_0, value: __self_1 } => {
                        CppNode::VarDef {
                            arr: ::core::clone::Clone::clone(__self_0),
                            value: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::FnDef {
                        params: __self_0,
                        defs: __self_1,
                        block: __self_2,
                        val: __self_3,
                    } => {
                        CppNode::FnDef {
                            params: ::core::clone::Clone::clone(__self_0),
                            defs: ::core::clone::Clone::clone(__self_1),
                            block: ::core::clone::Clone::clone(__self_2),
                            val: ::core::clone::Clone::clone(__self_3),
                        }
                    }
                    CppNode::FnParameter { name: __self_0, type_: __self_1 } => {
                        CppNode::FnParameter {
                            name: ::core::clone::Clone::clone(__self_0),
                            type_: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::StructDef { name: __self_0, fields: __self_1 } => {
                        CppNode::StructDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            fields: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::EnumDef { name: __self_0, items: __self_1 } => {
                        CppNode::EnumDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            items: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::EnumItem { name: __self_0, num: __self_1 } => {
                        CppNode::EnumItem {
                            name: ::core::clone::Clone::clone(__self_0),
                            num: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::ClassDef { name: __self_0, fields: __self_1 } => {
                        CppNode::ClassDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            fields: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::CodeBlock { actions: __self_0 } => {
                        CppNode::CodeBlock {
                            actions: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::VisibleLabel { name: __self_0 } => {
                        CppNode::VisibleLabel {
                            name: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::NamespaceDef { name: __self_0, items: __self_1 } => {
                        CppNode::NamespaceDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            items: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::BinOp { val: __self_0, add: __self_1 } => {
                        CppNode::BinOp {
                            val: ::core::clone::Clone::clone(__self_0),
                            add: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::Value { values: __self_0, ops: __self_1 } => {
                        CppNode::Value {
                            values: ::core::clone::Clone::clone(__self_0),
                            ops: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::BasicValue { val: __self_0 } => {
                        CppNode::BasicValue {
                            val: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::Method {
                        name: __self_0,
                        call: __self_1,
                        generic: __self_2,
                    } => {
                        CppNode::Method {
                            name: ::core::clone::Clone::clone(__self_0),
                            call: ::core::clone::Clone::clone(__self_1),
                            generic: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::GenericUse { types: __self_0 } => {
                        CppNode::GenericUse {
                            types: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::Call { args: __self_0 } => {
                        CppNode::Call {
                            args: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::IncDec { val: __self_0 } => {
                        CppNode::IncDec {
                            val: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::SingleValue {
                        un_op: __self_0,
                        val: __self_1,
                        methods: __self_2,
                    } => {
                        CppNode::SingleValue {
                            un_op: ::core::clone::Clone::clone(__self_0),
                            val: ::core::clone::Clone::clone(__self_1),
                            methods: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::ValueParenthesis { val: __self_0 } => {
                        CppNode::ValueParenthesis {
                            val: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        impl CppNode {
            fn parse_action<'code>(
                session: &mut impl SyntaxSession<'code, Node = Self>,
            ) -> Self {
                ::core::panicking::panic("not implemented")
            }
        }
    }
    use lady_deirdre::lexis::{CodeContent, SourceCode, ToSpan, TokenBuffer};
    use lady_deirdre::syntax::Node;
    use std::fs;
    pub fn main() {
        let code = TokenBuffer::<
            lexis::CppToken,
        >::from(
            fs::read_to_string("txt.c").expect("Should have been able to read the file"),
        );
        let tree = syntax::CppNode::parse(code.cursor(..));
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n", tree.errors().map(| error | { let res =
                    ::alloc::fmt::format(format_args!("{0}: {1}", error.span().format(&
                    code), error)); res }).collect::< Vec < _ >> ().join("\n")
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n", code.chunks(..).map(| chunk | format_args!("{0:?}", chunk
                    .token)).collect::< Vec < _ >> ().join("|")
                ),
            );
        }
    }
}
fn main() {
    syntax_cpp::main();
}
