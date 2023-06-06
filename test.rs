#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod syntax_cpp {
    pub mod lexis {
        use lady_deirdre::lexis::Token;
        #[repr(u8)]
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
            #[rule("typename")]
            Typename,
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
            Open,
            #[precedence(3)]
            #[rule(')')]
            Close,
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
            #[rule(['A'..'Z', 'a'..'z', '_']+)]
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
                        CppToken::Typename => "Typename",
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
                        CppToken::Open => "Open",
                        CppToken::Close => "Close",
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
            fn parse(session: &mut impl ::lady_deirdre::lexis::LexisSession) -> Self {
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
                                'A'..='Z'
                                | '_'
                                | 'h'
                                | 'j'..='k'
                                | 'm'
                                | 'q'
                                | 'y'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                '%' | '^' => {
                                    state = 20usize;
                                    continue;
                                }
                                '\'' => {
                                    state = 34usize;
                                    continue;
                                }
                                '(' => {
                                    state = 38usize;
                                    continue;
                                }
                                '"' => {
                                    state = 41usize;
                                    continue;
                                }
                                '|' => {
                                    state = 47usize;
                                    continue;
                                }
                                '!' => {
                                    state = 50usize;
                                    continue;
                                }
                                '0' => {
                                    state = 56usize;
                                    continue;
                                }
                                '1'..='9' => {
                                    state = 57usize;
                                    continue;
                                }
                                'l' => {
                                    state = 72usize;
                                    continue;
                                }
                                '\t'..='\r' | ' ' => {
                                    state = 89usize;
                                    continue;
                                }
                                'o' => {
                                    state = 99usize;
                                    continue;
                                }
                                '#' => {
                                    state = 104usize;
                                    continue;
                                }
                                'f' => {
                                    state = 131usize;
                                    continue;
                                }
                                '{' => {
                                    state = 143usize;
                                    continue;
                                }
                                '~' => {
                                    state = 151usize;
                                    continue;
                                }
                                '-' => {
                                    state = 163usize;
                                    continue;
                                }
                                'c' => {
                                    state = 171usize;
                                    continue;
                                }
                                '}' => {
                                    state = 178usize;
                                    continue;
                                }
                                'n' => {
                                    state = 179usize;
                                    continue;
                                }
                                ';' => {
                                    state = 180usize;
                                    continue;
                                }
                                '>' => {
                                    state = 181usize;
                                    continue;
                                }
                                'e' => {
                                    state = 182usize;
                                    continue;
                                }
                                ']' => {
                                    state = 183usize;
                                    continue;
                                }
                                '&' => {
                                    state = 184usize;
                                    continue;
                                }
                                'x' => {
                                    state = 185usize;
                                    continue;
                                }
                                'd' => {
                                    state = 186usize;
                                    continue;
                                }
                                '?' => {
                                    state = 187usize;
                                    continue;
                                }
                                'a' => {
                                    state = 188usize;
                                    continue;
                                }
                                '@' => {
                                    state = 189usize;
                                    continue;
                                }
                                '[' => {
                                    state = 190usize;
                                    continue;
                                }
                                '$' | '`' => {
                                    state = 191usize;
                                    continue;
                                }
                                't' => {
                                    state = 192usize;
                                    continue;
                                }
                                '/' => {
                                    state = 193usize;
                                    continue;
                                }
                                'w' => {
                                    state = 194usize;
                                    continue;
                                }
                                'i' => {
                                    state = 195usize;
                                    continue;
                                }
                                '.' => {
                                    state = 196usize;
                                    continue;
                                }
                                '+' => {
                                    state = 197usize;
                                    continue;
                                }
                                'r' => {
                                    state = 198usize;
                                    continue;
                                }
                                'u' => {
                                    state = 199usize;
                                    continue;
                                }
                                ':' => {
                                    state = 200usize;
                                    continue;
                                }
                                '=' => {
                                    state = 201usize;
                                    continue;
                                }
                                's' => {
                                    state = 202usize;
                                    continue;
                                }
                                ')' => {
                                    state = 203usize;
                                    continue;
                                }
                                'b' => {
                                    state = 204usize;
                                    continue;
                                }
                                '<' => {
                                    state = 205usize;
                                    continue;
                                }
                                'p' => {
                                    state = 206usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 207usize;
                                    continue;
                                }
                                'g' => {
                                    state = 208usize;
                                    continue;
                                }
                                '*' => {
                                    state = 209usize;
                                    continue;
                                }
                                ',' => {
                                    state = 210usize;
                                    continue;
                                }
                                'v' => {
                                    state = 211usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        2usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 5usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        3usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        5usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 26usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        6usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 7usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        7usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 17usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        8usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 9usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        9usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 78usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        10usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 11usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        11usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 257usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        12usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 13usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        13usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='j' | 'l'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'k' => {
                                    state = 306usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        14usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 15usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        15usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 71usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        16usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 68usize;
                        }
                        17usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 29usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        18usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 19usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        19usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 124usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        20usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        21usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 82usize;
                        }
                        22usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 23usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        23usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 141usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        24usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 25usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        25usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 277usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        26usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 27usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        27usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 40usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        28usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 29usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        29usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 147usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        30usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 31usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        31usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 32usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        32usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 234usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        33usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 34usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        34usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 68usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\'' => {
                                    state = 16usize;
                                    continue;
                                }
                                '\t'..='\r' | ' '..='&' | '('..='[' | ']'..='~' => {
                                    state = 34usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 262usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 34usize;
                                    continue;
                                }
                            }
                        }
                        35usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 36usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        36usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 15usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        37usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 69usize;
                        }
                        38usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 57usize;
                        }
                        39usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 24usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        40usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r' | ' '..='w' | 'y'..='~' => {
                                    state = 41usize;
                                    continue;
                                }
                                'x' => {
                                    state = 42usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 41usize;
                                    continue;
                                }
                            }
                        }
                        41usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 69usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '"' => {
                                    state = 37usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 40usize;
                                    continue;
                                }
                                '\t'..='\r' | ' '..='!' | '#'..='[' | ']'..='~' => {
                                    state = 41usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 41usize;
                                    continue;
                                }
                            }
                        }
                        42usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 244usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        43usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 44usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        44usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 23usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        45usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='l' | 'n'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 46usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        46usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 39usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        47usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                '|' => {
                                    state = 48usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        48usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                        }
                        49usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 16usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        50usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 70usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 48usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        51usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r' | ' '..=')' | '+'..='.' | '0'..='~' => {
                                    state = 52usize;
                                    continue;
                                }
                                '*' => {
                                    state = 53usize;
                                    continue;
                                }
                                '\0' | '/' => break,
                                _ => {
                                    state = 52usize;
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
                                '*' => {
                                    state = 103usize;
                                    continue;
                                }
                                _ => break,
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
                                '*' => {
                                    state = 103usize;
                                    continue;
                                }
                                '/' => {
                                    state = 260usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        54usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 55usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        55usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='x' | 'z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'y' => {
                                    state = 28usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        56usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' => {
                                    state = 57usize;
                                    continue;
                                }
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 58usize;
                                    continue;
                                }
                                '.' => {
                                    state = 59usize;
                                    continue;
                                }
                                'o' => {
                                    state = 60usize;
                                    continue;
                                }
                                'x' => {
                                    state = 61usize;
                                    continue;
                                }
                                'b' => {
                                    state = 62usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        57usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' => {
                                    state = 57usize;
                                    continue;
                                }
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 58usize;
                                    continue;
                                }
                                '.' => {
                                    state = 59usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        58usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 58usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        59usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' => {
                                    state = 146usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        60usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='7' => {
                                    state = 328usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        61usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 95usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        62usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='1' => {
                                    state = 144usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        63usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 26usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        64usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 65usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        65usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 159usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        66usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 8usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        67usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 68usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        68usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 329usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        69usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 70usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        70usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 46usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        71usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 70usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        72usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 73usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        73usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 106usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        74usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 4usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        75usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 50usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        76usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 77usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        77usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 36usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        78usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 10usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        79usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 80usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        80usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 32usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        81usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 82usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        82usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='g' | 'i'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 314usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        83usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 84usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        84usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 81usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        85usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 86usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        86usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 13usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        87usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 66usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        88usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 21usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        89usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 56usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r' | ' ' => {
                                    state = 89usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        90usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 91usize;
                                    continue;
                                }
                                'o' => {
                                    state = 92usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        91usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 218usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        92usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 101usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        93usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 94usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        94usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='f' | 'h'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 114usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        95usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 55usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 95usize;
                                    continue;
                                }
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 96usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        96usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 55usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 96usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        97usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 98usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        98usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 311usize;
                                    continue;
                                }
                                'd' => {
                                    state = 338usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        99usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='o'
                                | 'q'
                                | 's'..='u'
                                | 'w'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 100usize;
                                    continue;
                                }
                                'r' => {
                                    state = 101usize;
                                    continue;
                                }
                                'v' => {
                                    state = 102usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        100usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 142usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        101usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        102usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 325usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        103usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '/' => {
                                    state = 260usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        104usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 81usize;
                        }
                        105usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 43usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        106usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='f' | 'h'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 107usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        107usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 35usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        108usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 109usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        109usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 36usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        110usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 111usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        111usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 216usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        112usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 14usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 113usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        113usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a' | 'c'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'b' => {
                                    state = 231usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        114usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 48usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        115usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 116usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        116usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 130usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        117usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 118usize;
                                    continue;
                                }
                                'p' => {
                                    state = 119usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        118usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 170usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        119usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 22usize;
                                    continue;
                                }
                                'l' => {
                                    state = 246usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        120usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='x' | 'z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'y' => {
                                    state = 121usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        121usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 44usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        122usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 123usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        123usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 291usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        124usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 125usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        125usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 75usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        126usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 127usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        127usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 280usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        128usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 129usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        129usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 34usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        130usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 49usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        131usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='h'
                                | 'j'..='k'
                                | 'm'..='n'
                                | 'p'..='q'
                                | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 132usize;
                                    continue;
                                }
                                'i' => {
                                    state = 133usize;
                                    continue;
                                }
                                'r' => {
                                    state = 134usize;
                                    continue;
                                }
                                'o' => {
                                    state = 135usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        132usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 335usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        133usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 150usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        134usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 321usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        135usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 258usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        136usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 97usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        137usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 138usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        138usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 237usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        139usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 7usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        140usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='v' | 'x'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'w' => {
                                    state = 105usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        141usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 19usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        142usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 157usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        143usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 59usize;
                        }
                        144usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 52usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='1' => {
                                    state = 144usize;
                                    continue;
                                }
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 145usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        145usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 52usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 145usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        146usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 54usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'D' | 'F' | 'L' | 'U' | 'd' | 'f' | 'l' | 'u' => {
                                    state = 58usize;
                                    continue;
                                }
                                '0'..='9' => {
                                    state = 146usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        147usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 11usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        148usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 149usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        149usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 303usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        150usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 268usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        151usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 85usize;
                        }
                        152usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='u' | 'w'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'v' => {
                                    state = 153usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        153usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 255usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        154usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 155usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        155usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 24usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        156usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 27usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        157usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 252usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        158usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 108usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        159usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 12usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        160usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 161usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        161usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 213usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        162usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 5usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        163usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                '-' => {
                                    state = 164usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        164usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 76usize;
                        }
                        165usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 6usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        166usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 167usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        167usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 322usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        168usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 169usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        169usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 126usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        170usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 238usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        171usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'b'..='g'
                                | 'i'..='k'
                                | 'm'..='n'
                                | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 76usize;
                                    continue;
                                }
                                'a' => {
                                    state = 172usize;
                                    continue;
                                }
                                'l' => {
                                    state = 173usize;
                                    continue;
                                }
                                'o' => {
                                    state = 174usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        172usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 300usize;
                                    continue;
                                }
                                's' => {
                                    state = 336usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        173usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 175usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        174usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='l' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 14usize;
                                    continue;
                                }
                                'n' => {
                                    state = 266usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        175usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 267usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        176usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 177usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        177usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 9usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        178usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 60usize;
                        }
                        179usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='d' | 'f'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 45usize;
                                    continue;
                                }
                                'o' => {
                                    state = 235usize;
                                    continue;
                                }
                                'e' => {
                                    state = 236usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        180usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 67usize;
                        }
                        181usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 74usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 48usize;
                                    continue;
                                }
                                '>' => {
                                    state = 212usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        182usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='k'
                                | 'm'
                                | 'o'..='w'
                                | 'y'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'x' => {
                                    state = 117usize;
                                    continue;
                                }
                                'l' => {
                                    state = 253usize;
                                    continue;
                                }
                                'n' => {
                                    state = 326usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        183usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 62usize;
                        }
                        184usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 78usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                '&' => {
                                    state = 48usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        185usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 92usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        186usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 112usize;
                                    continue;
                                }
                                'e' => {
                                    state = 339usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        187usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 80usize;
                        }
                        188usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 148usize;
                                    continue;
                                }
                                'n' => {
                                    state = 218usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        189usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 86usize;
                        }
                        190usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 61usize;
                        }
                        191usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 87usize;
                        }
                        192usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='d'
                                | 'f'..='g'
                                | 'i'..='q'
                                | 's'..='x'
                                | 'z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 120usize;
                                    continue;
                                }
                                'y' => {
                                    state = 136usize;
                                    continue;
                                }
                                'h' => {
                                    state = 221usize;
                                    continue;
                                }
                                'e' => {
                                    state = 264usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        193usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 72usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                '/' => {
                                    state = 240usize;
                                    continue;
                                }
                                '*' => {
                                    state = 241usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        194usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='g' | 'i'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 224usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        195usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='e' | 'g'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'f' => {
                                    state = 294usize;
                                    continue;
                                }
                                'n' => {
                                    state = 320usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        196usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 64usize;
                        }
                        197usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 71usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                '+' => {
                                    state = 164usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        198usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 297usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        199usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 271usize;
                                    continue;
                                }
                                'n' => {
                                    state = 272usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        200usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 65usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                ':' => {
                                    state = 256usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        201usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 75usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 48usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        202usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='g'
                                | 'j'..='s'
                                | 'u'..='v'
                                | 'x'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'w' => {
                                    state = 83usize;
                                    continue;
                                }
                                'h' => {
                                    state = 214usize;
                                    continue;
                                }
                                't' => {
                                    state = 229usize;
                                    continue;
                                }
                                'i' => {
                                    state = 230usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        203usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 58usize;
                        }
                        204usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 222usize;
                                    continue;
                                }
                                'r' => {
                                    state = 223usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        205usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 73usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 48usize;
                                    continue;
                                }
                                '<' => {
                                    state = 212usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        206usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 288usize;
                                    continue;
                                }
                                'r' => {
                                    state = 289usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        207usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 79usize;
                        }
                        208usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 319usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        209usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 77usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        210usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 63usize;
                        }
                        211usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 160usize;
                                    continue;
                                }
                                'o' => {
                                    state = 269usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        212usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '=' => {
                                    state = 21usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        213usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 115usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        214usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 215usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        215usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 286usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        216usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 39usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        217usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 140usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        218usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 101usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        219usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 220usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        220usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 8usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        221usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 217usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        222usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 90usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        223usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 12usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        224usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 225usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        225usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 316usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        226usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='g' | 'i'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'h' => {
                                    state = 74usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        227usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 228usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        228usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 85usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        229usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'r' => {
                                    state = 2usize;
                                    continue;
                                }
                                'A'..='Z' | '_' | 'b'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 232usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        230usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='f' | 'h'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'g' => {
                                    state = 334usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        231usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 242usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        232usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 110usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        233usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 230usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        234usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 38usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        235usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 71usize;
                                    continue;
                                }
                                'e' => {
                                    state = 282usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        236usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='v' | 'x'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'w' => {
                                    state = 245usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        237usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 42usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        238usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 239usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        239usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 20usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        240usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t' | '\u{b}'..='\r' | ' '..='[' | ']'..='~' => {
                                    state = 240usize;
                                    continue;
                                }
                                '\n' => {
                                    state = 260usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 327usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 240usize;
                                    continue;
                                }
                            }
                        }
                        241usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '*' => {
                                    state = 51usize;
                                    continue;
                                }
                                '\t'..='\r' | ' '..=')' | '+'..='~' => {
                                    state = 52usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 52usize;
                                    continue;
                                }
                            }
                        }
                        242usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 36usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        243usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 64usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        244usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 41usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        245usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 28usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        246usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 247usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        247usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 122usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        248usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 249usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        249usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 67usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        250usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'o' => {
                                    state = 251usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        251usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 290usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        252usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 250usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        253usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 254usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        254usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 49usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        255usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 79usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        256usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 66usize;
                        }
                        257usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 47usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        258usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 22usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        259usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 176usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        260usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 83usize;
                        }
                        261usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 243usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        262usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '\t'..='\r' | ' '..='w' | 'y'..='~' => {
                                    state = 34usize;
                                    continue;
                                }
                                'x' => {
                                    state = 308usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 34usize;
                                    continue;
                                }
                            }
                        }
                        263usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 158usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        264usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='l' | 'n'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 265usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        265usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 317usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        266usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 87usize;
                                    continue;
                                }
                                's' => {
                                    state = 331usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        267usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 162usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        268usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 88usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        269usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 35usize;
                                    continue;
                                }
                                'l' => {
                                    state = 270usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        270usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 18usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        271usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 93usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        272usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 10usize;
                                    continue;
                                }
                                's' => {
                                    state = 233usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        273usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='o' | 'q'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'p' => {
                                    state = 219usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        274usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='l' | 'n'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 69usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        275usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 156usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        276usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 3usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        277usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 275usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        278usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 279usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        279usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 128usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        280usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 281usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        281usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 292usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        282usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='w' | 'y'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'x' => {
                                    state = 283usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        283usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 165usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        284usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 45usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        285usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='c' | 'e'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'd' => {
                                    state = 107usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        286usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 107usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        287usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='e' | 'g'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'f' => {
                                    state = 284usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        288usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a' | 'c'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'b' => {
                                    state = 278usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        289usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='n' | 'p'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 152usize;
                                    continue;
                                }
                                'o' => {
                                    state = 168usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        290usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 30usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        291usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 18usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        292usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 33usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        293usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 293usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        294usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 25usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        295usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='u' | 'w' | 'y'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'x' => {
                                    state = 273usize;
                                    continue;
                                }
                                'v' => {
                                    state = 296usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        296usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 312usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        297usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='f'
                                | 'h'..='p'
                                | 'r'..='s'
                                | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 30usize;
                                    continue;
                                }
                                'q' => {
                                    state = 298usize;
                                    continue;
                                }
                                'g' => {
                                    state = 299usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        298usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 332usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        299usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 304usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        300usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='b' | 'd'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'c' => {
                                    state = 226usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        301usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='l' | 'n'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'm' => {
                                    state = 302usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        302usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 17usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        303usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 1usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        304usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='r' | 't'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                's' => {
                                    state = 263usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        305usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 54usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        306usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 2usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        307usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 259usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        308usize => {
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                                    state = 33usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        309usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 310usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        310usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 330usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        311usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 274usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        312usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 139usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        313usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 43usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        314usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 41usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        315usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 285usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        316usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 324usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        317usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 318usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        318usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 137usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        319usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 154usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        320usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='k' | 'm'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 36usize;
                                    continue;
                                }
                                'l' => {
                                    state = 309usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        321usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 313usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        322usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 37usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        323usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 6usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 295usize;
                                    continue;
                                }
                                'i' => {
                                    state = 307usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        324usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 51usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        325usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 248usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        326usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='t' | 'v'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'u' => {
                                    state = 301usize;
                                    continue;
                                }
                                _ => break,
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
                                '\t'..='\r' | ' '..='[' | ']'..='~' => {
                                    state = 240usize;
                                    continue;
                                }
                                '\\' => {
                                    state = 327usize;
                                    continue;
                                }
                                '\0' => break,
                                _ => {
                                    state = 240usize;
                                    continue;
                                }
                            }
                        }
                        328usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 53usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'L' | 'U' | 'l' | 'u' => {
                                    state = 293usize;
                                    continue;
                                }
                                '0'..='7' => {
                                    state = 328usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        329usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 31usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        330usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 63usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        331usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 323usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        332usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='h' | 'j'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'i' => {
                                    state = 333usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        333usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='q' | 's'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'r' => {
                                    state = 166usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        334usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='m' | 'o'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'n' => {
                                    state = 315usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        335usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'b'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'a' => {
                                    state = 337usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        336usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 276usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        337usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='s' | 'u'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                't' => {
                                    state = 36usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        338usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z' | '_' | 'a'..='d' | 'f'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'e' => {
                                    state = 287usize;
                                    continue;
                                }
                                _ => break,
                            }
                        }
                        339usize => {
                            ::lady_deirdre::lexis::LexisSession::submit(session);
                            token = 84usize;
                            let input = ::lady_deirdre::lexis::LexisSession::character(
                                session,
                            );
                            ::lady_deirdre::lexis::LexisSession::advance(session);
                            match input {
                                'A'..='Z'
                                | '_'
                                | 'a'..='b'
                                | 'd'..='e'
                                | 'g'..='k'
                                | 'm'..='z' => {
                                    state = 3usize;
                                    continue;
                                }
                                'l' => {
                                    state = 227usize;
                                    continue;
                                }
                                'f' => {
                                    state = 261usize;
                                    continue;
                                }
                                'c' => {
                                    state = 305usize;
                                    continue;
                                }
                                _ => break,
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
                    46usize => Self::Typename,
                    47usize => Self::Union,
                    48usize => Self::Using,
                    49usize => Self::Virtual,
                    50usize => Self::Volatile,
                    51usize => Self::While,
                    52usize => Self::BinNumber,
                    53usize => Self::OctNumber,
                    54usize => Self::DecNumber,
                    55usize => Self::HexNumber,
                    56usize => Self::Whitespace,
                    57usize => Self::Open,
                    58usize => Self::Close,
                    59usize => Self::BraceOpen,
                    60usize => Self::BraceClose,
                    61usize => Self::BracketOpen,
                    62usize => Self::BracketClose,
                    63usize => Self::Comma,
                    64usize => Self::Point,
                    65usize => Self::Colon,
                    66usize => Self::DColon,
                    67usize => Self::Semicolon,
                    68usize => Self::Char,
                    69usize => Self::String,
                    70usize => Self::UnOp,
                    71usize => Self::MultiOp,
                    72usize => Self::BinOp,
                    73usize => Self::Less,
                    74usize => Self::Greater,
                    75usize => Self::Set,
                    76usize => Self::IncDec,
                    77usize => Self::Star,
                    78usize => Self::Amp,
                    79usize => Self::Backslash,
                    80usize => Self::QuestMark,
                    81usize => Self::Hash,
                    82usize => Self::SetOp,
                    83usize => Self::Comment,
                    84usize => Self::Ident,
                    85usize => Self::Tilde,
                    86usize => Self::At,
                    87usize => Self::Other,
                    _ => Self::Mismatch,
                }
            }
            #[inline(always)]
            fn index(self) -> ::lady_deirdre::lexis::TokenIndex {
                self as u8
            }
            #[inline(always)]
            fn describe(
                token: ::lady_deirdre::lexis::TokenIndex,
            ) -> ::std::option::Option<&'static str> {
                if Self::Auto as u8 == token {
                    return ::std::option::Option::Some("auto");
                }
                if Self::Break as u8 == token {
                    return ::std::option::Option::Some("break");
                }
                if Self::Case as u8 == token {
                    return ::std::option::Option::Some("case");
                }
                if Self::Catch as u8 == token {
                    return ::std::option::Option::Some("catch");
                }
                if Self::Class as u8 == token {
                    return ::std::option::Option::Some("class");
                }
                if Self::Const as u8 == token {
                    return ::std::option::Option::Some("const");
                }
                if Self::ConstEval as u8 == token {
                    return ::std::option::Option::Some("consteval");
                }
                if Self::ConstExpr as u8 == token {
                    return ::std::option::Option::Some("constexpr");
                }
                if Self::ConstInit as u8 == token {
                    return ::std::option::Option::Some("constinit");
                }
                if Self::Continue as u8 == token {
                    return ::std::option::Option::Some("continue");
                }
                if Self::DeclType as u8 == token {
                    return ::std::option::Option::Some("decltype");
                }
                if Self::Default as u8 == token {
                    return ::std::option::Option::Some("default");
                }
                if Self::Delete as u8 == token {
                    return ::std::option::Option::Some("delete");
                }
                if Self::Do as u8 == token {
                    return ::std::option::Option::Some("do");
                }
                if Self::BasicType as u8 == token {
                    return ::std::option::Option::Some("<basic type>");
                }
                if Self::Else as u8 == token {
                    return ::std::option::Option::Some("else");
                }
                if Self::Enum as u8 == token {
                    return ::std::option::Option::Some("enum");
                }
                if Self::Explicit as u8 == token {
                    return ::std::option::Option::Some("explicit");
                }
                if Self::Export as u8 == token {
                    return ::std::option::Option::Some("export");
                }
                if Self::Extern as u8 == token {
                    return ::std::option::Option::Some("extern");
                }
                if Self::Final as u8 == token {
                    return ::std::option::Option::Some("final");
                }
                if Self::For as u8 == token {
                    return ::std::option::Option::Some("for");
                }
                if Self::Friend as u8 == token {
                    return ::std::option::Option::Some("friend");
                }
                if Self::Goto as u8 == token {
                    return ::std::option::Option::Some("goto");
                }
                if Self::If as u8 == token {
                    return ::std::option::Option::Some("if");
                }
                if Self::Inline as u8 == token {
                    return ::std::option::Option::Some("inline");
                }
                if Self::Namespace as u8 == token {
                    return ::std::option::Option::Some("namespace");
                }
                if Self::New as u8 == token {
                    return ::std::option::Option::Some("new");
                }
                if Self::Noexpect as u8 == token {
                    return ::std::option::Option::Some("noexcept");
                }
                if Self::Operator as u8 == token {
                    return ::std::option::Option::Some("operator");
                }
                if Self::Override as u8 == token {
                    return ::std::option::Option::Some("override");
                }
                if Self::Private as u8 == token {
                    return ::std::option::Option::Some("private");
                }
                if Self::Protected as u8 == token {
                    return ::std::option::Option::Some("protected");
                }
                if Self::Public as u8 == token {
                    return ::std::option::Option::Some("public");
                }
                if Self::TypeMod as u8 == token {
                    return ::std::option::Option::Some("<type mod>");
                }
                if Self::Register as u8 == token {
                    return ::std::option::Option::Some("register");
                }
                if Self::Requires as u8 == token {
                    return ::std::option::Option::Some("requires");
                }
                if Self::Return as u8 == token {
                    return ::std::option::Option::Some("return");
                }
                if Self::Static as u8 == token {
                    return ::std::option::Option::Some("static");
                }
                if Self::Struct as u8 == token {
                    return ::std::option::Option::Some("struct");
                }
                if Self::Switch as u8 == token {
                    return ::std::option::Option::Some("switch");
                }
                if Self::Template as u8 == token {
                    return ::std::option::Option::Some("template");
                }
                if Self::Throw as u8 == token {
                    return ::std::option::Option::Some("throw");
                }
                if Self::Try as u8 == token {
                    return ::std::option::Option::Some("try");
                }
                if Self::Typedef as u8 == token {
                    return ::std::option::Option::Some("typedef");
                }
                if Self::Typename as u8 == token {
                    return ::std::option::Option::Some("typename");
                }
                if Self::Union as u8 == token {
                    return ::std::option::Option::Some("union");
                }
                if Self::Using as u8 == token {
                    return ::std::option::Option::Some("using");
                }
                if Self::Virtual as u8 == token {
                    return ::std::option::Option::Some("virtual");
                }
                if Self::Volatile as u8 == token {
                    return ::std::option::Option::Some("volatile");
                }
                if Self::While as u8 == token {
                    return ::std::option::Option::Some("while");
                }
                if Self::BinNumber as u8 == token {
                    return ::std::option::Option::Some("<bin number>");
                }
                if Self::OctNumber as u8 == token {
                    return ::std::option::Option::Some("<oct number>");
                }
                if Self::DecNumber as u8 == token {
                    return ::std::option::Option::Some("<dec number>");
                }
                if Self::HexNumber as u8 == token {
                    return ::std::option::Option::Some("<hex number>");
                }
                if Self::Whitespace as u8 == token {
                    return ::std::option::Option::Some("<whitespace>");
                }
                if Self::Open as u8 == token {
                    return ::std::option::Option::Some("(");
                }
                if Self::Close as u8 == token {
                    return ::std::option::Option::Some(")");
                }
                if Self::BraceOpen as u8 == token {
                    return ::std::option::Option::Some("{");
                }
                if Self::BraceClose as u8 == token {
                    return ::std::option::Option::Some("}");
                }
                if Self::BracketOpen as u8 == token {
                    return ::std::option::Option::Some("[");
                }
                if Self::BracketClose as u8 == token {
                    return ::std::option::Option::Some("]");
                }
                if Self::Comma as u8 == token {
                    return ::std::option::Option::Some(",");
                }
                if Self::Point as u8 == token {
                    return ::std::option::Option::Some(".");
                }
                if Self::Colon as u8 == token {
                    return ::std::option::Option::Some(":");
                }
                if Self::DColon as u8 == token {
                    return ::std::option::Option::Some("::");
                }
                if Self::Semicolon as u8 == token {
                    return ::std::option::Option::Some(";");
                }
                if Self::Char as u8 == token {
                    return ::std::option::Option::Some("<char>");
                }
                if Self::String as u8 == token {
                    return ::std::option::Option::Some("<string>");
                }
                if Self::UnOp as u8 == token {
                    return ::std::option::Option::Some("<un op>");
                }
                if Self::MultiOp as u8 == token {
                    return ::std::option::Option::Some("<multi op>");
                }
                if Self::BinOp as u8 == token {
                    return ::std::option::Option::Some("<bin op>");
                }
                if Self::Less as u8 == token {
                    return ::std::option::Option::Some("<");
                }
                if Self::Greater as u8 == token {
                    return ::std::option::Option::Some(">");
                }
                if Self::Set as u8 == token {
                    return ::std::option::Option::Some("=");
                }
                if Self::IncDec as u8 == token {
                    return ::std::option::Option::Some("<inc dec>");
                }
                if Self::Star as u8 == token {
                    return ::std::option::Option::Some("*");
                }
                if Self::Amp as u8 == token {
                    return ::std::option::Option::Some("&");
                }
                if Self::Backslash as u8 == token {
                    return ::std::option::Option::Some("\\");
                }
                if Self::QuestMark as u8 == token {
                    return ::std::option::Option::Some("?");
                }
                if Self::Hash as u8 == token {
                    return ::std::option::Option::Some("#");
                }
                if Self::SetOp as u8 == token {
                    return ::std::option::Option::Some("<set op>");
                }
                if Self::Comment as u8 == token {
                    return ::std::option::Option::Some("<comment>");
                }
                if Self::Ident as u8 == token {
                    return ::std::option::Option::Some("<ident>");
                }
                if Self::Tilde as u8 == token {
                    return ::std::option::Option::Some("~");
                }
                if Self::At as u8 == token {
                    return ::std::option::Option::Some("@");
                }
                if Self::Other as u8 == token {
                    return ::std::option::Option::Some("<other>");
                }
                if Self::Mismatch as u8 == token {
                    return ::std::option::Option::Some("<mismatch>");
                }
                None
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CppToken {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CppToken {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CppToken {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CppToken {
            #[inline]
            fn eq(&self, other: &CppToken) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
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
        #[trivia($Whitespace|Comment)]
        #[define(NUMBER = ($DecNumber|$HexNumber|$BinNumber|$OctNumber))]
        #[define(ACTION = (CodeBlock))]
        pub enum CppNode {
            #[rule(val:$Comment)]
            Comment { val: TokenRef },
            #[root]
            #[rule(items:RootItem*)]
            Root { items: Vec<NodeRef> },
            #[rule(
                (
                    $Extern&(lang:$String)
                )?&(
                    template:TemplateDef
                )?&(item:(BasicDef|StructDef|EnumDef|ClassDef|NamespaceDef))
            )]
            RootItem { lang: TokenRef, template: NodeRef, item: NodeRef },
            #[rule($Template&$Less&(types:TemplateType)*{$Comma}&$Comma?&$Greater)]
            TemplateDef { types: Vec<NodeRef> },
            #[rule((type_:($Typename|$Class))&(name:$Ident)&(recursive:$Point)*)]
            TemplateType { type_: TokenRef, name: TokenRef, recursive: Vec<TokenRef> },
            #[rule(((mods:$TypeMod)+&(type_:$BasicType)?)|(type_:$BasicType))]
            BasicType { mods: Vec<TokenRef>, type_: TokenRef },
            #[rule(
                (
                    (
                        (
                            mods:($Const|$Auto|$Static|$Volatile)+
                        )&(type_:(BasicValue|Path))?
                    )|(type_:(BasicType|Path))
                )&(generic:GenericUse)?&(recursive:$Point*)&(refer:($Amp|$Star)*)
            )]
            Type {
                mods: Vec<TokenRef>,
                type_: NodeRef,
                refer: Vec<TokenRef>,
                generic: NodeRef,
                recursive: Vec<TokenRef>,
            },
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
            #[rule(
                (
                    arr:ArrIndex
                )?&($Set&(val:Value))?&($Comma&(nexts:StructVar))*&$Semicolon
            )]
            VarDef { arr: NodeRef, val: NodeRef, nexts: Vec<NodeRef> },
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
            #[rule(
                $Struct&(
                    name:Path
                )&$BraceOpen&(
                    fields:RootItem
                )*&$BraceClose&(defs:StructVar)*{$Comma}&$Semicolon
            )]
            StructDef { name: NodeRef, fields: Vec<NodeRef>, defs: Vec<NodeRef> },
            #[rule((name:Path)&(add:ArrIndex)?&($Set&(val:Value))?)]
            StructVar { name: NodeRef, val: NodeRef, add: NodeRef },
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
            #[rule((values:SingleValue)+{ops:BinOp})]
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
        impl ::lady_deirdre::syntax::Node for CppNode {
            type Token = CppToken;
            type Error = SyntaxError;
            #[inline(always)]
            fn parse<'code>(
                rule: ::lady_deirdre::syntax::RuleIndex,
                session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                    'code,
                    Node = Self,
                >,
            ) -> Self {
                static RULES_1: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[3u16, 18u16, 20u16, 22u16, 24u16, 25u16],
                );
                static RULES_2: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[3u16, 18u16, 22u16, 24u16, 25u16],
                );
                static RULES_3: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[4u16],
                );
                static RULES_4: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[5u16],
                );
                static RULES_5: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[2u16],
                );
                static RULES_6: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[6u16],
                );
                static RULES_7: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[7u16],
                );
                static RULES_8: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[8u16, 9u16],
                );
                static RULES_9: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[12u16],
                );
                static RULES_10: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[14u16],
                );
                static RULES_11: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[4u16, 15u16],
                );
                static RULES_12: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[2u16, 19u16],
                );
                static RULES_13: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[21u16],
                );
                static RULES_14: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[23u16],
                );
                static RULES_15: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[26u16, 27u16],
                );
                static RULES_16: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[10u16],
                );
                static RULES_17: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[28u16],
                );
                static RULES_18: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[25u16, 29u16],
                );
                static RULES_19: ::lady_deirdre::syntax::RuleSet = ::lady_deirdre::syntax::RuleSet::new(
                    &[29u16],
                );
                static TOKENS_1: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Extern as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::Template as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_2: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Comment as u8],
                );
                static TOKENS_3: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Extern as u8],
                );
                static TOKENS_4: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Const as u8,
                        CppToken::Ident as u8,
                        CppToken::Static as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_5: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::String as u8],
                );
                static TOKENS_6: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::Template as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_7: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_8: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Ident as u8, CppToken::Semicolon as u8],
                );
                static TOKENS_9: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Struct as u8],
                );
                static TOKENS_10: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Semicolon as u8, CppToken::Struct as u8],
                );
                static TOKENS_11: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceOpen as u8],
                );
                static TOKENS_12: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceOpen as u8, CppToken::Semicolon as u8],
                );
                static TOKENS_13: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Semicolon as u8],
                );
                static TOKENS_14: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Comma as u8, CppToken::Semicolon as u8],
                );
                static TOKENS_15: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Comma as u8],
                );
                static TOKENS_16: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8],
                );
                static TOKENS_17: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::BraceClose as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Extern as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Semicolon as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::Template as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_18: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Ident as u8],
                );
                static TOKENS_19: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::DColon as u8],
                );
                static TOKENS_20: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::BinNumber as u8,
                        CppToken::Char as u8,
                        CppToken::DecNumber as u8,
                        CppToken::HexNumber as u8,
                        CppToken::IncDec as u8,
                        CppToken::MultiOp as u8,
                        CppToken::OctNumber as u8,
                        CppToken::Open as u8,
                        CppToken::Star as u8,
                        CppToken::String as u8,
                        CppToken::UnOp as u8,
                    ],
                );
                static TOKENS_21: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Set as u8],
                );
                static TOKENS_22: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::BinOp as u8,
                        CppToken::Greater as u8,
                        CppToken::Less as u8,
                        CppToken::MultiOp as u8,
                        CppToken::Star as u8,
                    ],
                );
                static TOKENS_23: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::IncDec as u8,
                        CppToken::MultiOp as u8,
                        CppToken::Star as u8,
                        CppToken::UnOp as u8,
                    ],
                );
                static TOKENS_24: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BinNumber as u8,
                        CppToken::Char as u8,
                        CppToken::DecNumber as u8,
                        CppToken::HexNumber as u8,
                        CppToken::OctNumber as u8,
                        CppToken::String as u8,
                    ],
                );
                static TOKENS_25: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Open as u8],
                );
                static TOKENS_26: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Close as u8, CppToken::Open as u8],
                );
                static TOKENS_27: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::BinNumber as u8,
                        CppToken::Char as u8,
                        CppToken::Close as u8,
                        CppToken::DecNumber as u8,
                        CppToken::HexNumber as u8,
                        CppToken::IncDec as u8,
                        CppToken::MultiOp as u8,
                        CppToken::OctNumber as u8,
                        CppToken::Open as u8,
                        CppToken::Star as u8,
                        CppToken::String as u8,
                        CppToken::UnOp as u8,
                    ],
                );
                static TOKENS_28: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Close as u8],
                );
                static TOKENS_29: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BracketClose as u8],
                );
                static TOKENS_30: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::BinNumber as u8,
                        CppToken::BracketClose as u8,
                        CppToken::Char as u8,
                        CppToken::DecNumber as u8,
                        CppToken::HexNumber as u8,
                        CppToken::IncDec as u8,
                        CppToken::MultiOp as u8,
                        CppToken::OctNumber as u8,
                        CppToken::Open as u8,
                        CppToken::Star as u8,
                        CppToken::String as u8,
                        CppToken::UnOp as u8,
                    ],
                );
                static TOKENS_31: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BracketOpen as u8],
                );
                static TOKENS_32: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BracketClose as u8, CppToken::BracketOpen as u8],
                );
                static TOKENS_33: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Point as u8],
                );
                static TOKENS_34: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Close as u8, CppToken::Comma as u8],
                );
                static TOKENS_35: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Less as u8],
                );
                static TOKENS_36: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Greater as u8, CppToken::Less as u8],
                );
                static TOKENS_37: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Comma as u8, CppToken::Greater as u8],
                );
                static TOKENS_38: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Const as u8,
                        CppToken::Greater as u8,
                        CppToken::Ident as u8,
                        CppToken::Static as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_39: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Greater as u8],
                );
                static TOKENS_40: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::Const as u8,
                        CppToken::Static as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_41: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BasicType as u8, CppToken::TypeMod as u8],
                );
                static TOKENS_42: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Amp as u8, CppToken::Star as u8],
                );
                static TOKENS_43: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::IncDec as u8],
                );
                static TOKENS_44: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Amp as u8,
                        CppToken::BinOp as u8,
                        CppToken::MultiOp as u8,
                        CppToken::Star as u8,
                    ],
                );
                static TOKENS_45: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::Ident as u8],
                );
                static TOKENS_46: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Class as u8],
                );
                static TOKENS_47: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::Class as u8],
                );
                static TOKENS_48: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Private as u8,
                        CppToken::Protected as u8,
                        CppToken::Public as u8,
                    ],
                );
                static TOKENS_49: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::BraceOpen as u8],
                );
                static TOKENS_50: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::BraceClose as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Extern as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Private as u8,
                        CppToken::Protected as u8,
                        CppToken::Public as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::Template as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_51: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Colon as u8,
                        CppToken::Private as u8,
                        CppToken::Protected as u8,
                        CppToken::Public as u8,
                    ],
                );
                static TOKENS_52: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Colon as u8],
                );
                static TOKENS_53: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Template as u8],
                );
                static TOKENS_54: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Greater as u8, CppToken::Template as u8],
                );
                static TOKENS_55: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Class as u8, CppToken::Typename as u8],
                );
                static TOKENS_56: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Class as u8,
                        CppToken::Comma as u8,
                        CppToken::Greater as u8,
                        CppToken::Typename as u8,
                    ],
                );
                static TOKENS_57: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Class as u8,
                        CppToken::Greater as u8,
                        CppToken::Typename as u8,
                    ],
                );
                static TOKENS_58: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::Comma as u8],
                );
                static TOKENS_59: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BraceClose as u8,
                        CppToken::Comma as u8,
                        CppToken::Ident as u8,
                    ],
                );
                static TOKENS_60: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Enum as u8],
                );
                static TOKENS_61: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::Enum as u8],
                );
                static TOKENS_62: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BinNumber as u8,
                        CppToken::DecNumber as u8,
                        CppToken::HexNumber as u8,
                        CppToken::OctNumber as u8,
                    ],
                );
                static TOKENS_63: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Namespace as u8],
                );
                static TOKENS_64: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::BraceClose as u8, CppToken::Namespace as u8],
                );
                static TOKENS_65: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::BraceClose as u8,
                        CppToken::Class as u8,
                        CppToken::Const as u8,
                        CppToken::Enum as u8,
                        CppToken::Extern as u8,
                        CppToken::Ident as u8,
                        CppToken::Namespace as u8,
                        CppToken::Static as u8,
                        CppToken::Struct as u8,
                        CppToken::Template as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_66: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BracketOpen as u8,
                        CppToken::Comma as u8,
                        CppToken::Semicolon as u8,
                        CppToken::Set as u8,
                    ],
                );
                static TOKENS_67: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Ident as u8, CppToken::Open as u8],
                );
                static TOKENS_68: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Amp as u8, CppToken::Ident as u8, CppToken::Star as u8],
                );
                static TOKENS_69: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BracketOpen as u8,
                        CppToken::Comma as u8,
                        CppToken::Open as u8,
                        CppToken::Semicolon as u8,
                        CppToken::Set as u8,
                    ],
                );
                static TOKENS_70: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Comma as u8,
                        CppToken::Semicolon as u8,
                        CppToken::Set as u8,
                    ],
                );
                static TOKENS_71: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::Close as u8,
                        CppToken::Const as u8,
                        CppToken::Ident as u8,
                        CppToken::Static as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_72: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[CppToken::Semicolon as u8, CppToken::Set as u8],
                );
                static TOKENS_73: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::Auto as u8,
                        CppToken::BasicType as u8,
                        CppToken::BraceOpen as u8,
                        CppToken::Const as u8,
                        CppToken::Ident as u8,
                        CppToken::Semicolon as u8,
                        CppToken::Set as u8,
                        CppToken::Static as u8,
                        CppToken::TypeMod as u8,
                        CppToken::Volatile as u8,
                    ],
                );
                static TOKENS_74: ::lady_deirdre::lexis::TokenSet = ::lady_deirdre::lexis::TokenSet::inclusive(
                    &[
                        CppToken::BinNumber as u8,
                        CppToken::Char as u8,
                        CppToken::DecNumber as u8,
                        CppToken::Delete as u8,
                        CppToken::HexNumber as u8,
                        CppToken::OctNumber as u8,
                        CppToken::String as u8,
                    ],
                );
                #[allow(unused)]
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn skip_trivia<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    loop {
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Whitespace {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::Comment {
                                    ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        1u16,
                                    );
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Root<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut capture_items = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        skip_trivia(session);
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    skip_trivia(session);
                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0).is_some() {
                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(session, 0);
                        let end = ::lady_deirdre::lexis::TokenCursor::end_site_ref(
                            session,
                        );
                        ::lady_deirdre::syntax::SyntaxSession::error(
                            session,
                            ::lady_deirdre::syntax::SyntaxError {
                                span: site..end,
                                context: 0u16,
                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                            },
                        );
                    }
                    CppNode::Root {
                        items: capture_items,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Comment<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 1u16,
                                                expected_tokens: &TOKENS_2,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Comment {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_2,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 1u16,
                                        expected_tokens: &TOKENS_2,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Comment {
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_RootItem<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_item = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_template = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_lang = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 2u16,
                                                expected_tokens: &TOKENS_3,
                                                expected_rules: &RULES_1,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Template {
                                    capture_template = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        20u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        25u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Class {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        18u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Enum {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        22u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Namespace {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        24u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Struct {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        3u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Extern {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_1,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 2u16,
                                        expected_tokens: &TOKENS_3,
                                        expected_rules: &RULES_1,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 2u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_2,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        25u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Class {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        18u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Enum {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        22u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Namespace {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        24u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Struct {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        3u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_7,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 2u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_2,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 2u16,
                                                expected_tokens: &TOKENS_5,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Template {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_template = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        20u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        25u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Class {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        18u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Enum {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        22u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Namespace {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        24u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Struct {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 2u16,
                                            expected_tokens: &TOKENS_5,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        3u16,
                                    );
                                    break;
                                }
                                if token == CppToken::String {
                                    capture_lang = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_5,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 2u16,
                                        expected_tokens: &TOKENS_5,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 2u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_1,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Template {
                                    capture_template = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        20u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        25u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Class {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        18u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Enum {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        22u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Namespace {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        24u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Struct {
                                    capture_item = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        3u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_6,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 2u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_1,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::RootItem {
                        lang: capture_lang,
                        template: capture_template,
                        item: capture_item,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_StructDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_fields = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    let mut capture_defs = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &TOKENS_9,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Struct {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 6usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &TOKENS_9,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_10,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &TOKENS_9,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Semicolon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &RULES_5,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_17,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &RULES_5,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Semicolon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &TOKENS_13,
                                                expected_rules: &RULES_4,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_8,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &TOKENS_13,
                                        expected_rules: &RULES_4,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_4,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Comma {
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_4,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_4,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_8,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_4,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Semicolon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &TOKENS_14,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &TOKENS_15,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_14,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &TOKENS_14,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_8,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Semicolon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            7usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 3u16,
                                                expected_tokens: &TOKENS_11,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 3u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_12,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 3u16,
                                        expected_tokens: &TOKENS_11,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Semicolon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::StructDef {
                        name: capture_name,
                        fields: capture_fields,
                        defs: capture_defs,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Path<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_path = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 4u16,
                                                expected_tokens: &TOKENS_18,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::DColon {
                                    ::std::vec::Vec::push(
                                        &mut capture_path,
                                        ::lady_deirdre::lexis::TokenRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 4u16,
                                            expected_tokens: &TOKENS_18,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_path,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 4u16,
                                        expected_tokens: &TOKENS_18,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::DColon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 1usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 4u16,
                                            expected_tokens: &TOKENS_19,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_path,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Path {
                        path: capture_path,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_StructVar<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_add = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 5u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BracketOpen {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 5u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    capture_add = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        10u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Set {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 5u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 5u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 5u16,
                                            expected_tokens: &TOKENS_21,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    break;
                                }
                                break;
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 5u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_20,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 5u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::BracketOpen {
                                    capture_add = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        10u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::StructVar {
                        name: capture_name,
                        val: capture_val,
                        add: capture_add,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Value<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_ops = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    let mut capture_values = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 6u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_7,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_values,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            7u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_20,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 6u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_7,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_22,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_ops,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            17u16,
                                        ),
                                    );
                                    state = 1usize;
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Value {
                        values: capture_values,
                        ops: capture_ops,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_SingleValue<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_methods = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    let mut capture_un_op = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 7u16,
                                                expected_tokens: &TOKENS_23,
                                                expected_rules: &RULES_8,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_23,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_un_op,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_24,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        8u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Open {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        9u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_20,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 7u16,
                                        expected_tokens: &TOKENS_23,
                                        expected_rules: &RULES_8,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::BracketOpen {
                                    ::std::vec::Vec::push(
                                        &mut capture_methods,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            10u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::IncDec {
                                    ::std::vec::Vec::push(
                                        &mut capture_methods,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            16u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Point {
                                    ::std::vec::Vec::push(
                                        &mut capture_methods,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            11u16,
                                        ),
                                    );
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::SingleValue {
                        un_op: capture_un_op,
                        val: capture_val,
                        methods: capture_methods,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_BasicValue<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 8u16,
                                                expected_tokens: &TOKENS_24,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_24,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_24,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 8u16,
                                        expected_tokens: &TOKENS_24,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::BasicValue {
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_ValueParenthesis<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 9u16,
                                                expected_tokens: &TOKENS_25,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Open {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_26,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 9u16,
                                        expected_tokens: &TOKENS_25,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Close)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 9u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Close {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 9u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_6,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_27,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 9u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Close)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 9u16,
                                                expected_tokens: &TOKENS_28,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_28,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 9u16,
                                        expected_tokens: &TOKENS_28,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::ValueParenthesis {
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_ArrIndex<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 10u16,
                                                expected_tokens: &TOKENS_31,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 10u16,
                                            expected_tokens: &TOKENS_31,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BracketClose {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 10u16,
                                            expected_tokens: &TOKENS_31,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::BracketOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_32,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 10u16,
                                        expected_tokens: &TOKENS_31,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BracketClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 10u16,
                                                expected_tokens: &TOKENS_29,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BracketClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_29,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 10u16,
                                        expected_tokens: &TOKENS_29,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 10u16,
                                                expected_tokens: &TOKENS_29,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BracketClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_30,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 10u16,
                                        expected_tokens: &TOKENS_29,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::ArrIndex {
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Method<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_call = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_generic = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 11u16,
                                                expected_tokens: &TOKENS_33,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Point {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 11u16,
                                            expected_tokens: &TOKENS_33,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_33,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 11u16,
                                        expected_tokens: &TOKENS_33,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 11u16,
                                                expected_tokens: &TOKENS_18,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Open {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 11u16,
                                            expected_tokens: &TOKENS_18,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_call = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        12u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Less {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 11u16,
                                            expected_tokens: &TOKENS_18,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_generic = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        13u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 11u16,
                                        expected_tokens: &TOKENS_18,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Open {
                                    capture_call = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        12u16,
                                    );
                                    break;
                                }
                                if token == CppToken::Less {
                                    capture_generic = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        13u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                break;
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 11u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_9,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Open {
                                    capture_call = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        12u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_25,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 11u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_9,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Method {
                        name: capture_name,
                        call: capture_call,
                        generic: capture_generic,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Call<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_args = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 12u16,
                                                expected_tokens: &TOKENS_25,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Open {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_26,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 12u16,
                                        expected_tokens: &TOKENS_25,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Close)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 12u16,
                                                expected_tokens: &TOKENS_34,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 12u16,
                                            expected_tokens: &TOKENS_15,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_args,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            6u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_34,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 12u16,
                                        expected_tokens: &TOKENS_34,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 12u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_args,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            6u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::std::vec::Vec::push(
                                        &mut capture_args,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 12u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_6,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::Close {
                                    ::std::vec::Vec::push(
                                        &mut capture_args,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 12u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_6,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_27,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 12u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Close)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 12u16,
                                                expected_tokens: &TOKENS_28,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_args,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            6u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_27,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 12u16,
                                        expected_tokens: &TOKENS_28,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Call {
                        args: capture_args,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_GenericUse<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_types = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 13u16,
                                                expected_tokens: &TOKENS_35,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Less {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Greater {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 13u16,
                                            expected_tokens: &TOKENS_35,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 13u16,
                                            expected_tokens: &TOKENS_35,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            14u16,
                                        ),
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_36,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 13u16,
                                        expected_tokens: &TOKENS_35,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Greater)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 13u16,
                                                expected_tokens: &TOKENS_39,
                                                expected_rules: &RULES_10,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            14u16,
                                        ),
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_38,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 13u16,
                                        expected_tokens: &TOKENS_39,
                                        expected_rules: &RULES_10,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 13u16,
                                                expected_tokens: &TOKENS_37,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 13u16,
                                            expected_tokens: &TOKENS_15,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            14u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_37,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 13u16,
                                        expected_tokens: &TOKENS_37,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 13u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_10,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 13u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_10,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            14u16,
                                        ),
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 13u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_10,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_38,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 13u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_10,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Greater)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::GenericUse {
                        types: capture_types,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_Type<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_mods = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    let mut capture_refer = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    let mut capture_recursive = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    let mut capture_type_ = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_generic = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 14u16,
                                                expected_tokens: &TOKENS_40,
                                                expected_rules: &RULES_11,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_40,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_mods,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_41,
                                    token as u8,
                                ) {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        15u16,
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_4,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 14u16,
                                        expected_tokens: &TOKENS_40,
                                        expected_rules: &RULES_11,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_40,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_mods,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_24,
                                    token as u8,
                                ) {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        8u16,
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Point {
                                    ::std::vec::Vec::push(
                                        &mut capture_recursive,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Less {
                                    capture_generic = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        13u16,
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_42,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_refer,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                break;
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Point {
                                    ::std::vec::Vec::push(
                                        &mut capture_recursive,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Less {
                                    capture_generic = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        13u16,
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_42,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_refer,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                break;
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Point {
                                    ::std::vec::Vec::push(
                                        &mut capture_recursive,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_42,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_refer,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                break;
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_42,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_refer,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::Type {
                        mods: capture_mods,
                        type_: capture_type_,
                        refer: capture_refer,
                        generic: capture_generic,
                        recursive: capture_recursive,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_BasicType<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_mods = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    let mut capture_type_ = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 15u16,
                                                expected_tokens: &TOKENS_41,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::TypeMod {
                                    ::std::vec::Vec::push(
                                        &mut capture_mods,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BasicType {
                                    capture_type_ = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_41,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 15u16,
                                        expected_tokens: &TOKENS_41,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::TypeMod {
                                    ::std::vec::Vec::push(
                                        &mut capture_mods,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::BasicType {
                                    capture_type_ = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::BasicType {
                        mods: capture_mods,
                        type_: capture_type_,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_IncDec<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 16u16,
                                                expected_tokens: &TOKENS_43,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::IncDec {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_43,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 16u16,
                                        expected_tokens: &TOKENS_43,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::IncDec {
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_BinOp<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_val = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_add = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 17u16,
                                                expected_tokens: &TOKENS_22,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Less {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_44,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Greater {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_22,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 17u16,
                                        expected_tokens: &TOKENS_22,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Less {
                                    capture_add = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                break;
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Greater {
                                    capture_add = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::BinOp {
                        val: capture_val,
                        add: capture_add,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_ClassDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_fields = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 18u16,
                                                expected_tokens: &TOKENS_46,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Class {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 18u16,
                                            expected_tokens: &TOKENS_46,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_47,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 18u16,
                                        expected_tokens: &TOKENS_46,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 18u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &RULES_12,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_48,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            19u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_50,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 18u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &RULES_12,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 18u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 18u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_45,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 18u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 18u16,
                                                expected_tokens: &TOKENS_11,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 18u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_48,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 18u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_fields,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            19u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 18u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_49,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 18u16,
                                        expected_tokens: &TOKENS_11,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::ClassDef {
                        name: capture_name,
                        fields: capture_fields,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_VisibleLabel<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 19u16,
                                                expected_tokens: &TOKENS_48,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_48,
                                    token as u8,
                                ) {
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_51,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 19u16,
                                        expected_tokens: &TOKENS_48,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Colon)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 19u16,
                                                expected_tokens: &TOKENS_52,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Colon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_52,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 19u16,
                                        expected_tokens: &TOKENS_52,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::VisibleLabel {
                        name: capture_name,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_TemplateDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_types = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_53,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Template {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Less {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 20u16,
                                            expected_tokens: &TOKENS_53,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_54,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_53,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Greater)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_35,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Less {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 20u16,
                                            expected_tokens: &TOKENS_35,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Greater {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 20u16,
                                            expected_tokens: &TOKENS_35,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_55,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 20u16,
                                            expected_tokens: &TOKENS_35,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            21u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_36,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_35,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::Greater)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_37,
                                                expected_rules: &RULES_13,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_55,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            21u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_56,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_37,
                                        expected_rules: &RULES_13,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_39,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_39,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_39,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_37,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_37,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_37,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            7usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 20u16,
                                                expected_tokens: &TOKENS_39,
                                                expected_rules: &RULES_13,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Greater {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_55,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_types,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            21u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_57,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 20u16,
                                        expected_tokens: &TOKENS_39,
                                        expected_rules: &RULES_13,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::TemplateDef {
                        types: capture_types,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_TemplateType<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_type_ = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_recursive = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 21u16,
                                                expected_tokens: &TOKENS_55,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_55,
                                    token as u8,
                                ) {
                                    capture_type_ = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_55,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 21u16,
                                        expected_tokens: &TOKENS_55,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 21u16,
                                                expected_tokens: &TOKENS_18,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Point {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 21u16,
                                            expected_tokens: &TOKENS_18,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_recursive,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 21u16,
                                        expected_tokens: &TOKENS_18,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Point {
                                    ::std::vec::Vec::push(
                                        &mut capture_recursive,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::TemplateType {
                        type_: capture_type_,
                        name: capture_name,
                        recursive: capture_recursive,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_EnumDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_items = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_60,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 22u16,
                                            expected_tokens: &TOKENS_60,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::Enum {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 8usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_61,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_60,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_58,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_58,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_58,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &RULES_14,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            23u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_45,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &RULES_14,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_11,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 22u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            23u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 22u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::BraceOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 6usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 22u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_49,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_11,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_58,
                                                expected_rules: &RULES_14,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            23u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_59,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_58,
                                        expected_rules: &RULES_14,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            7usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_16,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            8usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 22u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::BraceOpen {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 22u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_45,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 22u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::EnumDef {
                        name: capture_name,
                        items: capture_items,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_EnumItem<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_num = ::lady_deirdre::lexis::TokenRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 23u16,
                                                expected_tokens: &TOKENS_18,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Set {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 23u16,
                                            expected_tokens: &TOKENS_18,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 23u16,
                                        expected_tokens: &TOKENS_18,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_62,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 23u16,
                                            expected_tokens: &TOKENS_21,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_num = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                break;
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 23u16,
                                                expected_tokens: &TOKENS_62,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_62,
                                    token as u8,
                                ) {
                                    capture_num = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_62,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 23u16,
                                        expected_tokens: &TOKENS_62,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::EnumItem {
                        name: capture_name,
                        num: capture_num,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_NamespaceDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_items = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 24u16,
                                                expected_tokens: &TOKENS_63,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Namespace {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 24u16,
                                            expected_tokens: &TOKENS_63,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_64,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 24u16,
                                        expected_tokens: &TOKENS_63,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 24u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 24u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_3,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_45,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 24u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            3usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 24u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &RULES_5,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_65,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 24u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &RULES_5,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 24u16,
                                                expected_tokens: &TOKENS_11,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 3usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_1,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 24u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_items,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            2u16,
                                        ),
                                    );
                                    state = 3usize;
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 24u16,
                                            expected_tokens: &TOKENS_11,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_49,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 24u16,
                                        expected_tokens: &TOKENS_11,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::NamespaceDef {
                        name: capture_name,
                        items: capture_items,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_BasicDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_refer = ::std::vec::Vec::<
                        ::lady_deirdre::lexis::TokenRef,
                    >::with_capacity(1);
                    let mut capture_def = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_path = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_type_ = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 25u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_10,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        14u16,
                                    );
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_4,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 25u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_10,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 25u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_15,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Open {
                                    capture_def = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        27u16,
                                    );
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_66,
                                    token as u8,
                                ) {
                                    capture_def = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        26u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_69,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 25u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_15,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 25u16,
                                                expected_tokens: &TOKENS_25,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    capture_path = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Open {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_67,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 25u16,
                                        expected_tokens: &TOKENS_25,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 25u16,
                                                expected_tokens: &TOKENS_42,
                                                expected_rules: &RULES_3,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_42,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_refer,
                                        ::lady_deirdre::lexis::TokenCursor::token_ref(session, 0),
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                if token == CppToken::Ident {
                                    capture_path = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        4u16,
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_68,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 25u16,
                                        expected_tokens: &TOKENS_42,
                                        expected_rules: &RULES_3,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 25u16,
                                                expected_tokens: &TOKENS_28,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Open {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 25u16,
                                            expected_tokens: &TOKENS_28,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_def = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        27u16,
                                    );
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_66,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 25u16,
                                            expected_tokens: &TOKENS_28,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_def = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        26u16,
                                    );
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_28,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 25u16,
                                        expected_tokens: &TOKENS_28,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::BasicDef {
                        type_: capture_type_,
                        path: capture_path,
                        refer: capture_refer,
                        def: capture_def,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_VarDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_nexts = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    let mut capture_val = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_arr = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 26u16,
                                                expected_tokens: &TOKENS_70,
                                                expected_rules: &RULES_16,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::BracketOpen {
                                    capture_arr = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        10u16,
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_66,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 26u16,
                                        expected_tokens: &TOKENS_70,
                                        expected_rules: &RULES_16,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 26u16,
                                                expected_tokens: &TOKENS_14,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &TOKENS_15,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_nexts,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_14,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 26u16,
                                        expected_tokens: &TOKENS_14,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 26u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_4,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    ::std::vec::Vec::push(
                                        &mut capture_nexts,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::std::vec::Vec::push(
                                        &mut capture_nexts,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_4,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::std::vec::Vec::push(
                                        &mut capture_nexts,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_4,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_18,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 26u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_4,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 26u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_6,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_6,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_6,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_20,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 26u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_6,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 26u16,
                                                expected_tokens: &TOKENS_70,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Ident {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &TOKENS_15,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_nexts,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            5u16,
                                        ),
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_20,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 26u16,
                                            expected_tokens: &TOKENS_21,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    capture_val = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        6u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_70,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 26u16,
                                        expected_tokens: &TOKENS_70,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::VarDef {
                        arr: capture_arr,
                        val: capture_val,
                        nexts: capture_nexts,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_FnDef<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_params = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    let mut capture_val = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_block = ::lady_deirdre::syntax::NodeRef::nil();
                    let mut capture_defs = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_25,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Open {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 4usize;
                                    continue;
                                }
                                if token == CppToken::Close {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 27u16,
                                            expected_tokens: &TOKENS_25,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 27u16,
                                            expected_tokens: &TOKENS_25,
                                            expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                        },
                                    );
                                    ::std::vec::Vec::push(
                                        &mut capture_params,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            28u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_25,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_25,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_13,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_13,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_13,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            4usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_28,
                                                expected_rules: &RULES_17,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_params,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            28u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_71,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_28,
                                        expected_rules: &RULES_17,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            5usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_72,
                                                expected_rules: &RULES_18,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Semicolon {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                if token == CppToken::BraceOpen {
                                    capture_block = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        29u16,
                                    );
                                    break;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_defs,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            25u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::Set {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 8usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_73,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_72,
                                        expected_rules: &RULES_18,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            6usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_34,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Close {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 7usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_34,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_34,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            7usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_17,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::Close {
                                    ::std::vec::Vec::push(
                                        &mut capture_params,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 27u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_17,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 5usize;
                                    continue;
                                }
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    ::std::vec::Vec::push(
                                        &mut capture_params,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            28u16,
                                        ),
                                    );
                                    state = 6usize;
                                    continue;
                                }
                                if token == CppToken::Comma {
                                    ::std::vec::Vec::push(
                                        &mut capture_params,
                                        ::lady_deirdre::syntax::NodeRef::nil(),
                                    );
                                    site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::syntax::SyntaxSession::error(
                                        session,
                                        ::lady_deirdre::syntax::SyntaxError {
                                            span: site..site,
                                            context: 27u16,
                                            expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                            expected_rules: &RULES_17,
                                        },
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_4,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_17,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            8usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 27u16,
                                                expected_tokens: &TOKENS_74,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_74,
                                    token as u8,
                                ) {
                                    capture_val = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_74,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 27u16,
                                        expected_tokens: &TOKENS_74,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::FnDef {
                        params: capture_params,
                        defs: capture_defs,
                        block: capture_block,
                        val: capture_val,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_FnParameter<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_name = ::lady_deirdre::lexis::TokenRef::nil();
                    let mut capture_type_ = ::lady_deirdre::syntax::NodeRef::nil();
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 28u16,
                                                expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                                expected_rules: &RULES_10,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if ::lady_deirdre::lexis::TokenSet::contains(
                                    &TOKENS_4,
                                    token as u8,
                                ) {
                                    capture_type_ = ::lady_deirdre::syntax::SyntaxSession::descend(
                                        session,
                                        14u16,
                                    );
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_4,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 28u16,
                                        expected_tokens: &::lady_deirdre::lexis::EMPTY_TOKEN_SET,
                                        expected_rules: &RULES_10,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => break,
                                };
                                if token == CppToken::Ident {
                                    capture_name = ::lady_deirdre::lexis::TokenCursor::token_ref(
                                        session,
                                        0,
                                    );
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                break;
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::FnParameter {
                        name: capture_name,
                        type_: capture_type_,
                    }
                }
                #[allow(unused_mut)]
                #[allow(unused_assignments)]
                #[allow(unused_variables)]
                #[allow(non_snake_case)]
                fn parse_CodeBlock<'code>(
                    session: &mut impl ::lady_deirdre::syntax::SyntaxSession<
                        'code,
                        Node = CppNode,
                    >,
                ) -> CppNode {
                    let mut state = 1usize;
                    let mut site = ::lady_deirdre::lexis::SiteRef::nil();
                    let mut first = true;
                    let mut capture_actions = ::std::vec::Vec::<
                        ::lady_deirdre::syntax::NodeRef,
                    >::with_capacity(1);
                    loop {
                        match first {
                            true => first = false,
                            false => skip_trivia(session),
                        }
                        match state {
                            1usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 29u16,
                                                expected_tokens: &TOKENS_11,
                                                expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    state = 2usize;
                                    continue;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_49,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 29u16,
                                        expected_tokens: &TOKENS_11,
                                        expected_rules: &::lady_deirdre::syntax::EMPTY_RULE_SET,
                                    },
                                );
                                if recovered {
                                    if ::lady_deirdre::lexis::TokenCursor::token(session, 0)
                                        == ::std::option::Option::Some(CppToken::BraceClose)
                                    {
                                        ::lady_deirdre::lexis::TokenCursor::advance(session);
                                        recovered = false;
                                    }
                                }
                                if !recovered {
                                    break;
                                }
                            }
                            2usize => {
                                let token = match ::lady_deirdre::lexis::TokenCursor::token(
                                    session,
                                    0,
                                ) {
                                    ::std::option::Option::Some(token) => token,
                                    ::std::option::Option::None => {
                                        site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                            session,
                                            0,
                                        );
                                        ::lady_deirdre::syntax::SyntaxSession::error(
                                            session,
                                            ::lady_deirdre::syntax::SyntaxError {
                                                span: site..site,
                                                context: 29u16,
                                                expected_tokens: &TOKENS_16,
                                                expected_rules: &RULES_19,
                                            },
                                        );
                                        break;
                                    }
                                };
                                if token == CppToken::BraceOpen {
                                    ::std::vec::Vec::push(
                                        &mut capture_actions,
                                        ::lady_deirdre::syntax::SyntaxSession::descend(
                                            session,
                                            29u16,
                                        ),
                                    );
                                    continue;
                                }
                                if token == CppToken::BraceClose {
                                    ::lady_deirdre::lexis::TokenCursor::advance(session);
                                    break;
                                }
                                site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                let mut recovered = ::lady_deirdre::syntax::Recovery::recover(
                                    &::lady_deirdre::syntax::UNLIMITED_RECOVERY,
                                    session,
                                    &TOKENS_49,
                                );
                                let end_site = ::lady_deirdre::lexis::TokenCursor::site_ref(
                                    session,
                                    0,
                                );
                                ::lady_deirdre::syntax::SyntaxSession::error(
                                    session,
                                    ::lady_deirdre::syntax::SyntaxError {
                                        span: site..end_site,
                                        context: 29u16,
                                        expected_tokens: &TOKENS_16,
                                        expected_rules: &RULES_19,
                                    },
                                );
                                if !recovered {
                                    break;
                                }
                            }
                            other => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("Unknown state {0}.", other)
                                    ),
                                );
                            }
                        }
                    }
                    CppNode::CodeBlock {
                        actions: capture_actions,
                    }
                }
                match rule {
                    0u16 => parse_Root(session),
                    1u16 => parse_Comment(session),
                    2u16 => parse_RootItem(session),
                    3u16 => parse_StructDef(session),
                    4u16 => parse_Path(session),
                    5u16 => parse_StructVar(session),
                    6u16 => parse_Value(session),
                    7u16 => parse_SingleValue(session),
                    8u16 => parse_BasicValue(session),
                    9u16 => parse_ValueParenthesis(session),
                    10u16 => parse_ArrIndex(session),
                    11u16 => parse_Method(session),
                    12u16 => parse_Call(session),
                    13u16 => parse_GenericUse(session),
                    14u16 => parse_Type(session),
                    15u16 => parse_BasicType(session),
                    16u16 => parse_IncDec(session),
                    17u16 => parse_BinOp(session),
                    18u16 => parse_ClassDef(session),
                    19u16 => parse_VisibleLabel(session),
                    20u16 => parse_TemplateDef(session),
                    21u16 => parse_TemplateType(session),
                    22u16 => parse_EnumDef(session),
                    23u16 => parse_EnumItem(session),
                    24u16 => parse_NamespaceDef(session),
                    25u16 => parse_BasicDef(session),
                    26u16 => parse_VarDef(session),
                    27u16 => parse_FnDef(session),
                    28u16 => parse_FnParameter(session),
                    29u16 => parse_CodeBlock(session),
                    other => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not implemented: {0}",
                                format_args!("Unsupported rule {0}.", other)
                            ),
                        );
                    }
                }
            }
            #[inline(always)]
            fn describe(
                rule: ::lady_deirdre::syntax::RuleIndex,
            ) -> ::std::option::Option<&'static str> {
                match rule {
                    0u16 => ::std::option::Option::Some("Root"),
                    1u16 => ::std::option::Option::Some("Comment"),
                    2u16 => ::std::option::Option::Some("Root Item"),
                    3u16 => ::std::option::Option::Some("Struct Def"),
                    4u16 => ::std::option::Option::Some("Path"),
                    5u16 => ::std::option::Option::Some("Struct Var"),
                    6u16 => ::std::option::Option::Some("Value"),
                    7u16 => ::std::option::Option::Some("Single Value"),
                    8u16 => ::std::option::Option::Some("Basic Value"),
                    9u16 => ::std::option::Option::Some("Value Parenthesis"),
                    10u16 => ::std::option::Option::Some("Arr Index"),
                    11u16 => ::std::option::Option::Some("Method"),
                    12u16 => ::std::option::Option::Some("Call"),
                    13u16 => ::std::option::Option::Some("Generic Use"),
                    14u16 => ::std::option::Option::Some("Type"),
                    15u16 => ::std::option::Option::Some("Basic Type"),
                    16u16 => ::std::option::Option::Some("Inc Dec"),
                    17u16 => ::std::option::Option::Some("Bin Op"),
                    18u16 => ::std::option::Option::Some("Class Def"),
                    19u16 => ::std::option::Option::Some("Visible Label"),
                    20u16 => ::std::option::Option::Some("Template Def"),
                    21u16 => ::std::option::Option::Some("Template Type"),
                    22u16 => ::std::option::Option::Some("Enum Def"),
                    23u16 => ::std::option::Option::Some("Enum Item"),
                    24u16 => ::std::option::Option::Some("Namespace Def"),
                    25u16 => ::std::option::Option::Some("Basic Def"),
                    26u16 => ::std::option::Option::Some("Var Def"),
                    27u16 => ::std::option::Option::Some("Fn Def"),
                    28u16 => ::std::option::Option::Some("Fn Parameter"),
                    29u16 => ::std::option::Option::Some("Code Block"),
                    _ => ::std::option::Option::None,
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CppNode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CppNode::Comment { val: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Comment",
                            "val",
                            &__self_0,
                        )
                    }
                    CppNode::Root { items: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Root",
                            "items",
                            &__self_0,
                        )
                    }
                    CppNode::RootItem {
                        lang: __self_0,
                        template: __self_1,
                        item: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "RootItem",
                            "lang",
                            __self_0,
                            "template",
                            __self_1,
                            "item",
                            &__self_2,
                        )
                    }
                    CppNode::TemplateDef { types: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "TemplateDef",
                            "types",
                            &__self_0,
                        )
                    }
                    CppNode::TemplateType {
                        type_: __self_0,
                        name: __self_1,
                        recursive: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "TemplateType",
                            "type_",
                            __self_0,
                            "name",
                            __self_1,
                            "recursive",
                            &__self_2,
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
                        mods: __self_0,
                        type_: __self_1,
                        refer: __self_2,
                        generic: __self_3,
                        recursive: __self_4,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "Type",
                            "mods",
                            __self_0,
                            "type_",
                            __self_1,
                            "refer",
                            __self_2,
                            "generic",
                            __self_3,
                            "recursive",
                            &__self_4,
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
                    CppNode::VarDef { arr: __self_0, val: __self_1, nexts: __self_2 } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "VarDef",
                            "arr",
                            __self_0,
                            "val",
                            __self_1,
                            "nexts",
                            &__self_2,
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
                    CppNode::StructDef {
                        name: __self_0,
                        fields: __self_1,
                        defs: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "StructDef",
                            "name",
                            __self_0,
                            "fields",
                            __self_1,
                            "defs",
                            &__self_2,
                        )
                    }
                    CppNode::StructVar {
                        name: __self_0,
                        val: __self_1,
                        add: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "StructVar",
                            "name",
                            __self_0,
                            "val",
                            __self_1,
                            "add",
                            &__self_2,
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
                    CppNode::Comment { val: __self_0 } => {
                        CppNode::Comment {
                            val: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::Root { items: __self_0 } => {
                        CppNode::Root {
                            items: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::RootItem {
                        lang: __self_0,
                        template: __self_1,
                        item: __self_2,
                    } => {
                        CppNode::RootItem {
                            lang: ::core::clone::Clone::clone(__self_0),
                            template: ::core::clone::Clone::clone(__self_1),
                            item: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::TemplateDef { types: __self_0 } => {
                        CppNode::TemplateDef {
                            types: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    CppNode::TemplateType {
                        type_: __self_0,
                        name: __self_1,
                        recursive: __self_2,
                    } => {
                        CppNode::TemplateType {
                            type_: ::core::clone::Clone::clone(__self_0),
                            name: ::core::clone::Clone::clone(__self_1),
                            recursive: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::BasicType { mods: __self_0, type_: __self_1 } => {
                        CppNode::BasicType {
                            mods: ::core::clone::Clone::clone(__self_0),
                            type_: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    CppNode::Type {
                        mods: __self_0,
                        type_: __self_1,
                        refer: __self_2,
                        generic: __self_3,
                        recursive: __self_4,
                    } => {
                        CppNode::Type {
                            mods: ::core::clone::Clone::clone(__self_0),
                            type_: ::core::clone::Clone::clone(__self_1),
                            refer: ::core::clone::Clone::clone(__self_2),
                            generic: ::core::clone::Clone::clone(__self_3),
                            recursive: ::core::clone::Clone::clone(__self_4),
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
                    CppNode::VarDef { arr: __self_0, val: __self_1, nexts: __self_2 } => {
                        CppNode::VarDef {
                            arr: ::core::clone::Clone::clone(__self_0),
                            val: ::core::clone::Clone::clone(__self_1),
                            nexts: ::core::clone::Clone::clone(__self_2),
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
                    CppNode::StructDef {
                        name: __self_0,
                        fields: __self_1,
                        defs: __self_2,
                    } => {
                        CppNode::StructDef {
                            name: ::core::clone::Clone::clone(__self_0),
                            fields: ::core::clone::Clone::clone(__self_1),
                            defs: ::core::clone::Clone::clone(__self_2),
                        }
                    }
                    CppNode::StructVar {
                        name: __self_0,
                        val: __self_1,
                        add: __self_2,
                    } => {
                        CppNode::StructVar {
                            name: ::core::clone::Clone::clone(__self_0),
                            val: ::core::clone::Clone::clone(__self_1),
                            add: ::core::clone::Clone::clone(__self_2),
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
    }
    use lady_deirdre::{
        lexis::{CodeContent, Token},
        syntax::TreeContent, Document,
    };
    use std::fs;
    pub fn main() {
        let tree = Document::<
            syntax::CppNode,
        >::from(
            fs::read_to_string("test.cpp")
                .expect("Should have been able to read the file"),
        );
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n{1}\n", tree.chunks(..).map(| ch | lexis::CppToken::describe(ch
                    .token as u8).unwrap()).collect::< Vec < _ >> ().join("|"), tree
                    .errors().map(| error | error.display(& tree).to_string()).collect::<
                    Vec < _ >> ().join("\n")
                ),
            );
        };
    }
}
fn main() {
    syntax_cpp::main();
}
