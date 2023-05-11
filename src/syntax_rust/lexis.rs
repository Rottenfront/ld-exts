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
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE   //
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER        //
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, //
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE //
// SOFTWARE.                                                                     //
// ///////////////////////////////////////////////////////////////////////////// //

use core::fmt::Display;
use lady_deirdre::lexis::LexisSession;

#[derive(Clone, Copy, Debug)]
pub enum RustToken {
    As,
    Async,
    Await,
    Break,
    Const,
    Continue,
    Crate,
    Do,
    Dyn,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    LSelf,
    USelf,
    Static,
    Struct,
    Super,
    Trait,
    True,
    Try,
    Type,
    Union,
    Unsafe,
    Use,
    Where,
    While,
    Yield,

    BasicType,

    Number,

    Whitespace, // ' ' '\t' '\x0b' '\x0c'
    NewLine,    // '\n' '\r'

    Open,  // (
    Close, // )

    Less,    // <
    Greater, // >

    BraceOpen,  // {
    BraceClose, // }

    BracketOpen,  // [
    BracketClose, // ]

    Comma, // ,
    Point, // .
    Range, // ..

    Apostrophe, // '
    AsciiChar,  // b'

    Colon,       // :
    DoubleColon, // ::

    Dollar,        // $
    Semicolon,     // ;
    BinOp,         // % ^ && || << >> / >= == <=
    Add,           // +
    Sub,           // -
    Set,           // =
    Refer,         // & *
    Tilda,         // ~
    At,            // @
    Backslash,     // \\
    Bang,          // !
    Quest,         // ?
    Hash,          // #
    HashBang,      // #!
    Arrow,         // ->
    SetOp,         // += -= /= *= ^= |= %= &= <<= >>=
    SingleComment, // //
    CommentOpen,   // /*
    CommentClose,  // */

    Escape, // '\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0'] | ('x' & HEX & HEX) | ("u{" & HEX & HEX & HEX & HEX & '}'))

    Ident,

    String, // todo: implement String in lexer
}

macro_rules! advance {
    ($i:expr, $x:expr) => {
        for _ in 0..$x {
            $i.advance();
        }
    };
}

macro_rules! word {
    ($session:expr, $i:ident) => {
        if $session.character() == ' ' || $session.character() == '\t' || $session.character() == '\r' || $session.character() == '\x0b'
        || $session.character() == '\n'  || $session.character() == '('  || $session.character() == ')'  || $session.character() == '\x0c'
        || $session.character() == '<'   || $session.character() == '>'  || $session.character() == '{'  || $session.character() == '}'
        || $session.character() == '['   || $session.character() == ']'  || $session.character() == ','  || $session.character() == '|'
        || $session.character() == '.'   || $session.character() == '\'' || $session.character() == ':'  || $session.character() == '$'
        || $session.character() == ';'   || $session.character() == '-'  || $session.character() == '%'  || $session.character() == '^'
        || $session.character() == '='   || $session.character() == '+'  || $session.character() == '&'  || $session.character() == '*'
        || $session.character() == '/'   || $session.character() == '~'  || $session.character() == '@'  || $session.character() == '\\'
        || $session.character() == '!'   || $session.character() == '?'  || $session.character() == '#' {
            $session.submit();
            return Self::$i;
        }
    };
    /*
    ($session:expr, $i:ident, $f:expr, $( $o:expr ),*) => {
        if $session.character() == $f {
            $session.advance();
            word!($session, $i, $( $o, )*)
        }
    };*/
    ($session:expr, $i:ident, $f1:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i)
        }
    };
    ($session:expr, $i:ident, $f1:expr, $f2:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i, $f2)
        }
    };
    ($session:expr, $i:ident, $f1:expr, $f2:expr, $f3:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i, $f2, $f3)
        }
    };
    ($session:expr, $i:ident, $f1:expr, $f2:expr, $f3:expr, $f4:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i, $f2, $f3, $f4)
        }
    };
}

impl lady_deirdre::lexis::Token for RustToken {
    fn new(session: &mut impl LexisSession) -> Self {
        let mut state = 1usize;
        loop {
            let current = session.character();
            session.advance();
            let next = session.character();
            match (state, current, next) {
                (1usize, '0'..='9', _) => loop {
                    if session.character() == ' '
                        || session.character() == '\t'
                        || session.character() == '\r'
                        || session.character() == '\x0b'
                        || session.character() == '\n'
                        || session.character() == '('
                        || session.character() == ')'
                        || session.character() == '\x0c'
                        || session.character() == '<'
                        || session.character() == '>'
                        || session.character() == '{'
                        || session.character() == '}'
                        || session.character() == '['
                        || session.character() == ']'
                        || session.character() == ','
                        || session.character() == '|'
                        || session.character() == '\''
                        || session.character() == ':'
                        || session.character() == '$'
                        || session.character() == ';'
                        || session.character() == '-'
                        || session.character() == '%'
                        || session.character() == '^'
                        || session.character() == '='
                        || session.character() == '+'
                        || session.character() == '&'
                        || session.character() == '*'
                        || session.character() == '/'
                        || session.character() == '~'
                        || session.character() == '@'
                        || session.character() == '\\'
                        || session.character() == '!'
                        || session.character() == '?'
                        || session.character() == '#'
                    {
                        session.submit();
                        return Self::Number;
                    }
                },

                (1usize | 3usize, '\t' | '\u{b}'..='\r' | ' ', '\t' | '\u{b}'..='\r' | ' ') => {
                    state = 3usize
                }
                (1usize | 3usize, '\t' | '\u{b}'..='\r' | ' ', _) => {
                    session.submit();
                    return Self::Whitespace;
                }
                (1usize | 4usize, '\n', '\n') => state = 4usize,
                (1usize | 4usize, '\n', _) => {
                    session.submit();
                    return Self::NewLine;
                }
                (1usize, '.', '.') => {
                    session.advance();
                    session.submit();
                    return Self::Range;
                }
                (1usize, 'b', '\'') => {
                    session.advance();
                    session.submit();
                    return Self::AsciiChar;
                }
                (1usize, ':', ':') => {
                    session.advance();
                    session.submit();
                    return Self::DoubleColon;
                }
                (1usize, '&', '&') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '|', '|') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '#', '!') => {
                    session.advance();
                    session.submit();
                    return Self::HashBang;
                }
                (1usize, '-', '>') => {
                    session.advance();
                    session.submit();
                    return Self::Arrow;
                }
                (1usize, '+' | '-' | '/' | '*' | '^' | '|' | '%' | '&', '=') => {
                    session.advance();
                    session.submit();
                    return Self::SetOp;
                }
                (1usize, '>', '>') => {
                    session.advance();
                    if session.character() == '=' {
                        session.advance();
                        session.submit();
                        return Self::SetOp;
                    }
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '<', '<') => {
                    session.advance();
                    if session.character() == '=' {
                        session.advance();
                        session.submit();
                        return Self::SetOp;
                    }
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '>' | '=' | '<', '=') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '/', '/') => {
                    session.advance();
                    session.submit();
                    return Self::SingleComment;
                }
                (1usize, '/', '*') => {
                    session.advance();
                    session.submit();
                    return Self::CommentOpen;
                }
                (1usize, '*', '/') => {
                    session.advance();
                    session.submit();
                    return Self::CommentClose;
                }
                (1usize, '(', _) => {
                    session.submit();
                    return Self::Open;
                }
                (1usize, ')', _) => {
                    session.submit();
                    return Self::Close;
                }
                (1usize, '<', _) => {
                    session.submit();
                    return Self::Less;
                }
                (1usize, '>', _) => {
                    session.submit();
                    return Self::Greater;
                }
                (1usize, '{', _) => {
                    session.submit();
                    return Self::BraceOpen;
                }
                (1usize, '}', _) => {
                    session.submit();
                    return Self::BraceClose;
                }
                (1usize, '[', _) => {
                    session.submit();
                    return Self::BracketOpen;
                }
                (1usize, ']', _) => {
                    session.submit();
                    return Self::BracketClose;
                }
                (1usize, '.', _) => {
                    session.submit();
                    return Self::Point;
                }
                (1usize, '\'', _) => {
                    session.submit();
                    return Self::Apostrophe;
                }
                (1usize, ':', _) => {
                    session.submit();
                    return Self::Colon;
                }
                (1usize, '$', _) => {
                    session.submit();
                    return Self::Dollar;
                }
                (1usize, ',', _) => {
                    session.submit();
                    return Self::Comma;
                }
                (1usize, ';', _) => {
                    session.submit();
                    return Self::Semicolon;
                }
                (1usize, '+', _) => {
                    session.submit();
                    return Self::Add;
                }
                (1usize, '=', _) => {
                    session.submit();
                    return Self::Set;
                }
                (1usize, '&' | '*', _) => {
                    session.submit();
                    return Self::Refer;
                }
                (1usize, '/' | '%' | '^' | '|', _) => {
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '~', _) => {
                    session.submit();
                    return Self::Tilda;
                }
                (1usize, '@', _) => {
                    session.submit();
                    return Self::At;
                }
                (1usize, '!', _) => {
                    session.submit();
                    return Self::Bang;
                }
                (1usize, '?', _) => {
                    session.submit();
                    return Self::Quest;
                }
                (1usize, '#', _) => {
                    session.submit();
                    return Self::Hash;
                }
                (
                    1usize,
                    '\\',
                    '\'' | '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | '\n' | '0',
                ) => {
                    session.submit();
                    return Self::Escape;
                }
                (1usize, '\\', 'x') => {
                    advance!(session, 3);
                    session.submit();
                    return Self::Escape;
                }
                (1usize, '\\', 'u') => {
                    advance!(session, 7);
                    session.submit();
                    return Self::Escape;
                }

                (1usize, 'a', _) => {
                    // 'a' 's'
                    // 'a' 's' 'y' 'n' 'c'
                    // 'a' 'w' 'a' 'i' 't'
                    if session.character() == 's' {
                        session.advance();
                        word!(session, As);
                        word!(session, Async, 'y', 'n', 'c');
                    } else if session.character() == 'w' {
                        session.advance();
                        word!(session, Await, 'a', 'i', 't');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'b', _) => {
                    // 'b' 'r' 'e' 'a' 'k'
                    // 'b' 'o' 'o' 'l'
                    if session.character() == 'r' {
                        session.advance();
                        word!(session, Break, 'e', 'a', 'k');
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, BasicType, 'o', 'l');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'c', _) => {
                    // 'c' 'o' 'n' 's' 't'
                    // 'c' 'o' 'n' 't' 'i' 'n' 'u' 'e'
                    // 'c' 'r' 'a' 't' 'e'
                    // 'c' 'h' 'a' 'r'
                    if session.character() == 'o' {
                        session.advance();
                        if session.character() == 'n' {
                            session.advance();
                            if session.character() == 's' {
                                session.advance();
                                word!(session, Const, 't');
                            } else if session.character() == 't' {
                                word!(session, Continue, 'i', 'n', 'u', 'e');
                            }
                        }
                    } else if session.character() == 'r' {
                        session.advance();
                        word!(session, Crate, 'a', 't', 'e');
                    } else if session.character() == 'h' {
                        session.advance();
                        word!(session, BasicType, 'a', 'r');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'd', _) => {
                    // 'd' 'o'
                    // 'd' 'y' 'n'
                    if session.character() == 'o' {
                        session.advance();
                        word!(session, Do);
                    } else if session.character() == 'y' {
                        session.advance();
                        word!(session, Dyn, 'n');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'e', _) => {
                    // 'e' 'l' 's' 'e'
                    // 'e' 'n' 'u' 'm'
                    // 'e' 'x' 't' 'e' 'r' 'n'
                    if session.character() == 'l' {
                        session.advance();
                        word!(session, Else, 's', 'e');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, Enum, 'u', 'm');
                    } else if session.character() == 'x' {
                        session.advance();
                        word!(session, Extern, 't', 'e', 'r', 'n');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'f', _) => {
                    // 'f' 'a' 'l' 's' 'e'
                    // 'f' 'n'
                    // 'f' 'o' 'r'
                    // 'f' '3' '2'
                    // 'f' '6' '4'
                    if session.character() == 'a' {
                        session.advance();
                        word!(session, Else, 's', 'e');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, Fn);
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, For, 'r');
                    } else if session.character() == '3' {
                        session.advance();
                        word!(session, BasicType, '2');
                    } else if session.character() == '6' {
                        session.advance();
                        word!(session, BasicType, '4');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'i', _) => {
                    // 'i' 'f'
                    // 'i' 'm' 'p' 'l'
                    // 'i' 'n'
                    // 'i' 's' 'i' 'z' 'e'
                    // 'i' '1' '2' '8'
                    // 'i' '1' '6'
                    // 'i' '3' '2'
                    // 'i' '6' '4'
                    // 'i' '8'
                    if session.character() == 'f' {
                        session.advance();
                        word!(session, If);
                    } else if session.character() == 'm' {
                        session.advance();
                        word!(session, Impl, 'p', 'l');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, In);
                    } else if session.character() == 's' {
                        session.advance();
                        word!(session, BasicType, 'i', 'z', 'e');
                    } else if session.character() == '1' {
                        session.advance();
                        if session.character() == '2' {
                            session.advance();
                            word!(session, BasicType, '8');
                        } else if session.character() == '6' {
                            session.advance();
                            word!(session, BasicType);
                        }
                    } else if session.character() == '3' {
                        session.advance();
                        word!(session, BasicType, '2');
                    } else if session.character() == '6' {
                        session.advance();
                        word!(session, BasicType, '4');
                    } else if session.character() == '8' {
                        session.advance();
                        word!(session, BasicType, '8');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'l', _) => {
                    // 'l' 'e' 't'
                    // 'l' 'o' 'o' 'p'
                    if session.character() == 'e' {
                        session.advance();
                        word!(session, Let, 't');
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, Loop, 'o', 'p');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'm', _) => {
                    // 'm' 'a' 'c' 'r' 'o'
                    // 'm' 'a' 't' 'c' 'h'
                    // 'm' 'o' 'd'
                    // 'm' 'o' 'v' 'e'
                    // 'm' 'u' 't'
                    if session.character() == 'a' {
                        session.advance();
                        if session.character() == 'c' {
                            session.advance();
                            word!(session, Macro, 'r', 'o');
                        } else if session.character() == 'c' {
                            session.advance();
                            word!(session, Match, 'c', 'h');
                        }
                    } else if session.character() == 'o' {
                        session.advance();
                        if session.character() == 'd' {
                            session.advance();
                            word!(session, Mod);
                        } else if session.character() == 'v' {
                            session.advance();
                            word!(session, Move, 'e');
                        }
                    } else if session.character() == 'u' {
                        session.advance();
                        word!(session, Mut, 't');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'p', 'u') => {
                    // 'p' 'u' 'b'
                    session.advance();
                    word!(session, Pub, 'b');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'r', 'e') => {
                    // 'r' 'e' 'f'
                    // 'r' 'e' 't' 'u' 'r' 'n'
                    session.advance();
                    if session.character() == 'f' {
                        session.advance();
                        word!(session, Ref);
                    } else if session.character() == 't' {
                        session.advance();
                        word!(session, Return, 'u', 'r', 'n');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'S', 'e') => {
                    // 'S' 'e' 'l' 'f'
                    session.advance();
                    word!(session, USelf, 'l', 'f');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 's', _) => {
                    // 's' 'e' 'l' 'f'
                    // 's' 't' 'a' 't' 'i' 'c'
                    // 's' 't' 'r'
                    // 's' 't' 'r' 'u' 'c' 't'
                    // 's' 'u' 'p' 'e' 'r'
                    if session.character() == 'e' {
                        session.advance();
                        word!(session, LSelf, 'l', 'f');
                    } else if session.character() == 't' {
                        session.advance();
                        if session.character() == 'a' {
                            session.advance();
                            word!(session, Static, 't', 'i', 'c');
                        } else if session.character() == 'r' {
                            session.advance();
                            word!(session, BasicType);
                            word!(session, Struct, 'u', 'c', 't');
                        }
                    } else if session.character() == 'u' {
                        session.advance();
                        word!(session, Super, 'p', 'e', 'r');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 't', _) => {
                    // 't' 'r' 'a' 'i' 't'
                    // 't' 'r' 'u' 'e'
                    // 't' 'r' 'y'
                    // 't' 'y' 'p' 'e'
                    if session.character() == 'r' {
                        session.advance();
                        if session.character() == 'a' {
                            session.advance();
                            word!(session, Trait, 'i', 't');
                        } else if session.character() == 'u' {
                            session.advance();
                            word!(session, True, 'e');
                        } else if session.character() == 'y' {
                            session.advance();
                            word!(session, Try);
                        }
                    } else if session.character() == 'y' {
                        session.advance();
                        word!(session, Type, 'p', 'e');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'u', _) => {
                    // 'u' 'n' 'i' 'o' 'n'
                    // 'u' 'n' 's' 'a' 'f' 'e'
                    // 'u' 's' 'e'
                    // 'u' 's' 'i' 'z' 'e'
                    // 'u' '1' '6'
                    // 'u' '1' '2' '8'
                    // 'u' '3' '2'
                    // 'u' '6' '4'
                    // 'u' '8'
                    if session.character() == 'n' {
                        session.advance();
                        if session.character() == 'i' {
                            session.advance();
                            word!(session, Union, 'o', 'n');
                        } else if session.character() == 's' {
                            session.advance();
                            word!(session, Unsafe, 'a', 'f', 'e');
                        }
                    } else if session.character() == 's' {
                        session.advance();
                        if session.character() == 'e' {
                            session.advance();
                            word!(session, Use);
                        } else if session.character() == 'i' {
                            session.advance();
                            word!(session, BasicType, 'z', 'e');
                        }
                    } else if session.character() == '1' {
                        session.advance();
                        if session.character() == '2' {
                            session.advance();
                            word!(session, BasicType, '8');
                        } else if session.character() == '6' {
                            session.advance();
                            word!(session, BasicType);
                        }
                    } else if session.character() == '3' {
                        session.advance();
                        word!(session, BasicType, '2');
                    } else if session.character() == '6' {
                        session.advance();
                        word!(session, BasicType, '4');
                    } else if session.character() == '8' {
                        session.advance();
                        word!(session, BasicType, '8');
                    }
                    word!(session, Ident);

                    state = 5usize;
                }
                (1usize, 'w', 'h') => {
                    // 'w' 'h' 'e' 'r' 'e'
                    // 'w' 'h' 'i' 'l' 'e'
                    session.advance();
                    if session.character() == 'i' {
                        session.advance();
                        word!(session, While, 'l', 'e');
                    } else if session.character() == 'e' {
                        session.advance();
                        word!(session, Where, 'r', 'e');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'y', 'i') => {
                    // 'y' 'i' 'e' 'l' 'd'
                    session.advance();
                    word!(session, Yield, 'e', 'l', 'd');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, '"', ch) => {
                    let mut ch1 = ch;
                    session.advance();
                    if ch1 == '"' {
                        session.submit();
                        return Self::String;
                    }
                    let mut ch2 = session.character();
                    session.advance();
                    if ch1 != '\\' && ch2 == '"' {
                        session.submit();
                        return Self::String;
                    }
                    let mut ch3 = session.character();
                    while ch3 != '"' || (ch1 != '\\' && ch2 == '\\' && ch3 == '"') {
                        session.advance();
                        ch1 = ch2;
                        ch2 = ch3;
                        ch3 = session.character();
                    }
                    session.submit();
                    return Self::String;
                }
                (1usize | 5usize, _, ch) => {
                    if ch == ' '
                        || ch == '\t'
                        || ch == '\r'
                        || ch == '\x0b'
                        || ch == '\x0c'
                        || ch == '\n'
                        || ch == '('
                        || ch == ')'
                        || ch == '<'
                        || ch == '>'
                        || ch == '{'
                        || ch == '}'
                        || ch == '['
                        || ch == ']'
                        || ch == ','
                        || ch == '|'
                        || ch == '.'
                        || ch == '\''
                        || ch == ':'
                        || ch == '$'
                        || ch == ';'
                        || ch == '-'
                        || ch == '%'
                        || ch == '^'
                        || ch == '='
                        || ch == '+'
                        || ch == '&'
                        || ch == '*'
                        || ch == '/'
                        || ch == '~'
                        || ch == '@'
                        || ch == '\\'
                        || ch == '!'
                        || ch == '?'
                        || ch == '#'
                    {
                        session.submit();
                        return Self::Ident;
                    }
                }
                _ => break,
            }
        }
        Self::Ident
    }
}

impl Display for RustToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::As => "as",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Crate => "crate",
            Self::Do => "do",
            Self::Dyn => "dyn",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Extern => "extern",
            Self::False => "false",
            Self::Fn => "fn",
            Self::For => "for",
            Self::If => "if",
            Self::Impl => "impl",
            Self::In => "in",
            Self::Let => "let",
            Self::Loop => "loop",
            Self::Macro => "macro",
            Self::Match => "match",
            Self::Mod => "mod",
            Self::Move => "move",
            Self::Mut => "mut",
            Self::Pub => "pub",
            Self::Ref => "ref",
            Self::Return => "return",
            Self::LSelf => "self",
            Self::USelf => "Self",
            Self::Static => "static",
            Self::Struct => "struct",
            Self::Super => "super",
            Self::Trait => "trait",
            Self::True => "true",
            Self::Try => "try",
            Self::Type => "type",
            Self::Union => "union",
            Self::Unsafe => "unsafe",
            Self::Use => "use",
            Self::Where => "where",
            Self::While => "while",
            Self::Yield => "yield",
            Self::BasicType => "usize",
            Self::Number => "0x0abcd",
            Self::String => "STR",
            Self::Ident => "IDENT",
            Self::Open => "(",
            Self::Close => ")",
            Self::Less => "<",
            Self::Greater => ">",
            Self::BraceOpen => "{",
            Self::BraceClose => "}",
            Self::BracketOpen => "[",
            Self::BracketClose => "]",
            Self::Comma => ",",
            Self::Point => ".",
            Self::Range => "..",
            Self::Apostrophe => "'",
            Self::AsciiChar => "b'",
            Self::DoubleColon => "::",
            Self::Colon => ":",
            Self::Dollar => "$",
            Self::Semicolon => ";",
            Self::BinOp => "^",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Set => "=",
            Self::Refer => "&",
            Self::Tilda => "~",
            Self::At => "@",
            Self::Backslash => "\\",
            Self::Escape => "\\n",
            Self::Bang => "!",
            Self::Quest => "?",
            Self::Hash => "#",
            Self::HashBang => "#!",
            Self::Arrow => "->",
            Self::SetOp => "+=",
            Self::Whitespace => " ",
            Self::NewLine => "\n",
            Self::SingleComment => "//",
            Self::CommentOpen => "/*",
            Self::CommentClose => "*/",
        }
        .fmt(f)
    }
}
// ("r\"" & (ESCAPE | ^['"', '\\'])* & '\"')
// ("r#\"" & (ESCAPE | ^['"', '\\'])* & "\"#")
// ("r###\"" & (ESCAPE | ^['"', '\\'])* & "\"###")
