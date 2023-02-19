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
    KeywordAs,
    KeywordAsync,
    KeywordAwait,
    KeywordBreak,
    KeywordConst,
    KeywordContinue,
    KeywordCrate,
    KeywordDo,
    KeywordDyn,
    KeywordElse,
    KeywordEnum,
    KeywordExtern,
    KeywordFalse,
    KeywordFn,
    KeywordFor,
    KeywordIf,
    KeywordImpl,
    KeywordIn,
    KeywordLet,
    KeywordLoop,
    KeywordMacro,
    KeywordMatch,
    KeywordMod,
    KeywordMove,
    KeywordMut,
    KeywordPub,
    KeywordRef,
    KeywordReturn,
    KeywordSelf,
    KeywordUSelf,
    KeywordStatic,
    KeywordStruct,
    KeywordSuper,
    KeywordTrait,
    KeywordTrue,
    KeywordTry,
    KeywordType,
    KeywordUnion,
    KeywordUnsafe,
    KeywordUse,
    KeywordWhere,
    KeywordWhile,
    KeywordYield,

    NumType,
    BasicType,

    BinNumber,
    OctNumber,
    DecNumber,
    HexNumber,

    Whitespace,             // ' ' | '\t' | '\r' | '\x0b' | '\x0c'
    NewLine,                // '\n'

    ParenthesisOpen,        // (
    ParenthesisClose,       // )

    AngleBracketOpen,       // <
    AngleBracketClose,      // >

    BraceOpen,              // {
    BraceClose,             // }

    BracketOpen,            // [
    BracketClose,           // ]

    Comma,                  // ,
    Pipe,                   // |

    Point,                  // .
    Range,                  // ..

    Apostrophe,             // '
    AsciiChar,              // b'

    Colon,                  // :
    DoubleColon,            // ::

    Dollar,                 // $
    Semicolon,              // ;
    Operator,               // - % ^
    BoolOperator,           // >= == <=
    Add,                    // +
    Assign,                 // =
    Amp,                    // &
    Star,                   // *
    Slash,                  // /
    Tilda,                  // ~
    At,                     // @
    Backslash,              // \\
    Bang,                   // !
    QuestMark,              // ?
    Hash,                   // #
    HashBang,               // #!
    Arrow,                  // ->
    AssignWithOperation,    // += -= /= *= ^= |= %= &=
    SingleComment,          // //
    MultilineCommentOpen,   // /*
    MultilineCommentClose,  // */

    Escape, // '\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0'] | ('x' & HEX & HEX) | ("u{" & HEX & HEX & HEX & HEX & '}'))

    Identifier,

    String, // todo: implement String in lexer

    Underscore,
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
                (1usize, '0', 'b') => {
                    session.advance();
                    while session.character() == '0'
                        || session.character() == '1'
                        || session.character() == '_' {
                        session.advance();
                    }
                    if session.character() == 'i'
                        || session.character() == 'u'
                        || session.character() == 'f' {
                        while (session.character() >= '0'
                            && session.character() <= '9')
                            || (session.character() >= 'a'
                            && session.character() <= 'z') {
                            session.advance();
                        }
                    }
                    session.submit();
                    return Self::BinNumber;
                }
                (1usize, '0', 'o') => {
                    session.advance();
                    while (session.character() >= '0'
                        && session.character() <= '7')
                        || session.character() == '_' {
                        session.advance();
                    }
                    if session.character() == 'i'
                        || session.character() == 'u'
                        || session.character() == 'f' {
                        while (session.character() >= '0'
                            && session.character() <= '9')
                            || (session.character() >= 'a'
                            && session.character() <= 'z') {
                            session.advance();
                        }
                    }
                    session.submit();
                    return Self::OctNumber;
                }
                (1usize, '0', 'x') => {
                    session.advance();
                    while (session.character() >= '0'
                        && session.character() <= '9')
                        || (session.character() >= 'a'
                        && session.character() <= 'f')
                        || (session.character() >= 'A'
                        && session.character() <= 'F')
                        || session.character() == '_' {
                        session.advance();
                    }
                    if session.character() == 'i'
                        || session.character() == 'u'
                        || session.character() == 'f' {
                        while (session.character() >= '0'
                            && session.character() <= '9')
                            || (session.character() >= 'a'
                            && session.character() <= 'z') {
                            session.advance();
                        }
                    }
                    session.submit();
                    return Self::HexNumber;
                }

                (1usize | 2usize, '0'..='9', '0'..='9' | '.') => state = 2usize,
                (1usize, '0'..='9', _) => {
                    session.submit();
                    return Self::DecNumber;
                }
                (2usize, '0'..='9' | '.', _) => {
                    session.submit();
                    return Self::DecNumber;
                }

                (1usize | 3usize, '\t' | '\u{b}'..='\r' | ' ', '\t' | '\u{b}'..='\r' | ' ') => state = 3usize,
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
                    return Self::AssignWithOperation;
                }
                (1usize, '>' | '=' | '<', '=') => {
                    session.advance();
                    session.submit();
                    return Self::BoolOperator;
                }
                (1usize, '/', '/') => {
                    session.advance();
                    session.submit();
                    return Self::SingleComment;
                }
                (1usize, '/', '*') => {
                    session.advance();
                    session.submit();
                    return Self::MultilineCommentOpen;
                }
                (1usize, '*', '/') => {
                    session.advance();
                    session.submit();
                    return Self::MultilineCommentClose;
                }
                (1usize, '(', _) => {
                    session.submit();
                    return Self::ParenthesisOpen;
                }
                (1usize, ')', _) => {
                    session.submit();
                    return Self::ParenthesisClose;
                }
                (1usize, '<', _) => {
                    session.submit();
                    return Self::AngleBracketOpen;
                }
                (1usize, '>', _) => {
                    session.submit();
                    return Self::AngleBracketClose;
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
                (1usize, '|', _) => {
                    session.submit();
                    return Self::Pipe;
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
                    return Self::Assign;
                }
                (1usize, '&', _) => {
                    session.submit();
                    return Self::Amp;
                }
                (1usize, '*', _) => {
                    session.submit();
                    return Self::Star;
                }
                (1usize, '/', _) => {
                    session.submit();
                    return Self::Slash;
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
                    return Self::QuestMark;
                }
                (1usize, '#', _) => {
                    session.submit();
                    return Self::Hash;
                }
                (1usize, '\\', '\'' | '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | '\n' | '0') => {
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
                        word!(session, KeywordAs);
                        word!(session, KeywordAsync, 'y', 'n', 'c');
                    } else if session.character() == 'w' {
                        session.advance();
                        word!(session, KeywordAwait, 'a', 'i', 't');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'b', _) => {
                    // 'b' 'r' 'e' 'a' 'k'
                    // 'b' 'o' 'o' 'l'
                    if session.character() == 'r' {
                        session.advance();
                        word!(session, KeywordBreak, 'e', 'a', 'k');
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, BasicType, 'o', 'l');
                    }
                    word!(session, Identifier);
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
                                word!(session, KeywordConst, 't');
                            } else if session.character() == 't' {
                                word!(session, KeywordContinue, 'i', 'n', 'u', 'e');
                            }
                        }
                    } else if session.character() == 'r' {
                        session.advance();
                        word!(session, KeywordCrate, 'a', 't', 'e');
                    } else if session.character() == 'h' {
                        session.advance();
                        word!(session, BasicType, 'a', 'r');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'd', _) => {
                    // 'd' 'o'
                    // 'd' 'y' 'n'
                    if session.character() == 'o' {
                        session.advance();
                        word!(session, KeywordDo);
                    } else if session.character() == 'y' {
                        session.advance();
                        word!(session, KeywordDyn, 'n');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'e', _) => {
                    // 'e' 'l' 's' 'e'
                    // 'e' 'n' 'u' 'm'
                    // 'e' 'x' 't' 'e' 'r' 'n'
                    if session.character() == 'l' {
                        session.advance();
                        word!(session, KeywordElse, 's', 'e');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, KeywordEnum, 'u', 'm');
                    } else if session.character() == 'x' {
                        session.advance();
                        word!(session, KeywordExtern, 't', 'e', 'r', 'n');
                    }
                    word!(session, Identifier);
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
                        word!(session, KeywordElse, 's', 'e');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, KeywordFn);
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, KeywordFor, 'r');
                    } else if session.character() == '3' {
                        session.advance();
                        word!(session, BasicType, '2');
                    } else if session.character() == '6' {
                        session.advance();
                        word!(session, BasicType, '4');
                    }
                    word!(session, Identifier);
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
                        word!(session, KeywordIf);
                    } else if session.character() == 'm' {
                        session.advance();
                        word!(session, KeywordImpl, 'p', 'l');
                    } else if session.character() == 'n' {
                        session.advance();
                        word!(session, KeywordIn);
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
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'l', _) => {
                    // 'l' 'e' 't'
                    // 'l' 'o' 'o' 'p'
                    if session.character() == 'e' {
                        session.advance();
                        word!(session, KeywordLet, 't');
                    } else if session.character() == 'o' {
                        session.advance();
                        word!(session, KeywordLoop, 'o', 'p');
                    }
                    word!(session, Identifier);
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
                            word!(session, KeywordMacro, 'r', 'o');
                        } else if session.character() == 'c' {
                            session.advance();
                            word!(session, KeywordMatch, 'c', 'h');
                        }
                    } else if session.character() == 'o' {
                        session.advance();
                        if session.character() == 'd' {
                            session.advance();
                            word!(session, KeywordMod);
                        } else if session.character() == 'v' {
                            session.advance();
                            word!(session, KeywordMove, 'e');
                        }
                    } else if session.character() == 'u' {
                        session.advance();
                        word!(session, KeywordMut, 't');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'p', 'u') => {
                    // 'p' 'u' 'b'
                    session.advance();
                    word!(session, KeywordPub, 'b');
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'r', 'e') => {
                    // 'r' 'e' 'f'
                    // 'r' 'e' 't' 'u' 'r' 'n'
                    session.advance();
                    if session.character() == 'f' {
                        session.advance();
                        word!(session, KeywordRef);
                    } else if session.character() == 't' {
                        session.advance();
                        word!(session, KeywordReturn, 'u', 'r', 'n');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'S', 'e') => {
                    // 'S' 'e' 'l' 'f'
                    session.advance();
                    word!(session, KeywordUSelf, 'l', 'f');
                    word!(session, Identifier);
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
                        word!(session, KeywordSelf, 'l', 'f');
                    } else if session.character() == 't' {
                        session.advance();
                        if session.character() == 'a' {
                            session.advance();
                            word!(session, KeywordStatic, 't', 'i', 'c');
                        } else if session.character() == 'r' {
                            session.advance();
                            word!(session, BasicType);
                            word!(session, KeywordStruct, 'u', 'c', 't');
                        }
                    } else if session.character() == 'u' {
                        session.advance();
                        word!(session, KeywordSuper, 'p', 'e', 'r');
                    }
                    word!(session, Identifier);
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
                            word!(session, KeywordTrait, 'i', 't');
                        } else if session.character() == 'u' {
                            session.advance();
                            word!(session, KeywordTrue, 'e');
                        } else if session.character() == 'y' {
                            session.advance();
                            word!(session, KeywordTry);
                        }
                    } else if session.character() == 'y' {
                        session.advance();
                        word!(session, KeywordType, 'p', 'e');
                    }
                    word!(session, Identifier);
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
                            word!(session, KeywordUnion, 'o', 'n');
                        } else if session.character() == 's' {
                            session.advance();
                            word!(session, KeywordUnsafe, 'a', 'f', 'e');
                        }
                    } else if session.character() == 's' {
                        session.advance();
                        if session.character() == 'e' {
                            session.advance();
                            word!(session, KeywordUse);
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
                    word!(session, Identifier);

                    state = 5usize;
                }
                (1usize, 'w', 'h') => {
                    // 'w' 'h' 'e' 'r' 'e'
                    // 'w' 'h' 'i' 'l' 'e'
                    session.advance();
                    if session.character() == 'i' {
                        session.advance();
                        word!(session, KeywordWhile, 'l', 'e');
                    } else if session.character() == 'e' {
                        session.advance();
                        word!(session, KeywordWhere, 'r', 'e');
                    }
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, 'y', 'i') => {
                    // 'y' 'i' 'e' 'l' 'd'
                    session.advance();
                    word!(session, KeywordYield, 'e', 'l', 'd');
                    word!(session, Identifier);
                    state = 5usize;
                }
                (1usize, '_', _) => {
                    word!(session, Underscore);
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
                    if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\x0b' || ch == '\x0c' || ch == '\n' || ch == '(' || ch == ')'
                        || ch == '<' || ch == '>'  || ch == '{' || ch == '}' || ch == '[' || ch == ']' || ch == ',' || ch == '|'
                        || ch == '.' || ch == '\'' || ch == ':' || ch == '$' || ch == ';' || ch == '-' || ch == '%' || ch == '^'
                        || ch == '=' || ch == '+'  || ch == '&' || ch == '*' || ch == '/' || ch == '~' || ch == '@' || ch == '\\'
                        || ch == '!' || ch == '?'  || ch == '#' {
                        session.submit();
                        return Self::Identifier;
                    }
                }
                _ => break,
            }
        }
        Self::Identifier
    }
}

impl Display for RustToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::KeywordAs => "KeywordAs",
            Self::KeywordAsync => "KeywordAsync",
            Self::KeywordAwait => "KeywordAwait",
            Self::KeywordBreak => "KeywordBreak",
            Self::KeywordConst => "KeywordConst",
            Self::KeywordContinue => "KeywordContinue",
            Self::KeywordCrate => "KeywordCrate",
            Self::KeywordDo => "KeywordDo",
            Self::KeywordDyn => "KeywordDyn",
            Self::KeywordElse => "KeywordElse",
            Self::KeywordEnum => "KeywordEnum",
            Self::KeywordExtern => "KeywordExtern",
            Self::KeywordFalse => "KeywordFalse",
            Self::KeywordFn => "KeywordFn",
            Self::KeywordFor => "KeywordFor",
            Self::KeywordIf => "KeywordIf",
            Self::KeywordImpl => "KeywordImpl",
            Self::KeywordIn => "KeywordIn",
            Self::KeywordLet => "KeywordLet",
            Self::KeywordLoop => "KeywordLoop",
            Self::KeywordMacro => "KeywordMacro",
            Self::KeywordMatch => "KeywordMatch",
            Self::KeywordMod => "KeywordMod",
            Self::KeywordMove => "KeywordMove",
            Self::KeywordMut => "KeywordMut",
            Self::KeywordPub => "KeywordPub",
            Self::KeywordRef => "KeywordRef",
            Self::KeywordReturn => "KeywordReturn",
            Self::KeywordSelf => "KeywordSelf",
            Self::KeywordUSelf => "KeywordUSelf",
            Self::KeywordStatic => "KeywordStatic",
            Self::KeywordStruct => "KeywordStruct",
            Self::KeywordSuper => "KeywordSuper",
            Self::KeywordTrait => "KeywordTrait",
            Self::KeywordTrue => "KeywordTrue",
            Self::KeywordTry => "KeywordTry",
            Self::KeywordType => "KeywordType",
            Self::KeywordUnion => "KeywordUnion",
            Self::KeywordUnsafe => "KeywordUnsafe",
            Self::KeywordUse => "KeywordUse",
            Self::KeywordWhere => "KeywordWhere",
            Self::KeywordWhile => "KeywordWhile",
            Self::KeywordYield => "KeywordYield",
            Self::NumType => "NumType",
            Self::BasicType => "BasicType",
            Self::BinNumber => "BinNumber",
            Self::OctNumber => "OctNumber",
            Self::DecNumber => "DecNumber",
            Self::HexNumber => "HexNumber",
            Self::String => "String",
            Self::Identifier => "Identifier",
            Self::ParenthesisOpen => "ParenthesisOpen",
            Self::ParenthesisClose => "ParenthesisClose",
            Self::AngleBracketOpen => "AngleBracketOpen",
            Self::AngleBracketClose => "AngleBracketClose",
            Self::BraceOpen => "BraceOpen",
            Self::BraceClose => "BraceClose",
            Self::BracketOpen => "BracketOpen",
            Self::BracketClose => "BracketClose",
            Self::Underscore => "Underscore",
            Self::Comma => "Comma",
            Self::Point => "Point",
            Self::Pipe => "Pipe",
            Self::BoolOperator => "BoolOperator",
            Self::Range => "Range",
            Self::Apostrophe => "Apostrophe",
            Self::AsciiChar => "AsciiChar",
            Self::DoubleColon => "DoubleColon",
            Self::Colon => "Colon",
            Self::Dollar => "Dollar",
            Self::Semicolon => "Semicolon",
            Self::Operator => "Operator",
            Self::Add => "Add",
            Self::Assign => "Assign",
            Self::Amp => "Amp",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Tilda => "Tilda",
            Self::At => "At",
            Self::Backslash => "Backslash",
            Self::Escape => "Escape",
            Self::Bang => "Bang",
            Self::QuestMark => "QuestMark",
            Self::Hash => "Hash",
            Self::HashBang => "HashBang",
            Self::Arrow => "Arrow",
            Self::AssignWithOperation => "AssignWithOperation",
            Self::Whitespace => "Whitespace",
            Self::NewLine => "NewLine",
            Self::SingleComment => "SingleComment",
            Self::MultilineCommentOpen => "MultilineCommentOpen",
            Self::MultilineCommentClose => "MultilineCommentClose",
        }.fmt(f)
    }
}
// ("r\"" & (ESCAPE | ^['"', '\\'])* & '\"')
// ("r#\"" & (ESCAPE | ^['"', '\\'])* & "\"#")
// ("r###\"" & (ESCAPE | ^['"', '\\'])* & "\"###")

/*
struct S {}                  done
     struct S { x: T }       done
     struct S(T);            done
     struct S;               done
enum E {}                    done
     enum E { A, B(), C {} } done
     enum E { A = 1 }        done
union U {}                   todo
static X: T = T();           todo
const X: T = T();            todo
let x: T (= ...);            todo
let mut x: T (= ...);        todo

S { x: y }
S { x }
S { ..s }
S { 0: x }
S(x)
E::C { x: y }
()	Empty tuple
(x)	Parenthesized expression.
(x,)
(S,)
[S]
[S; n]
[x; n]
[x, y]
x[0]
     x[..]
a..b
..b
..=b
a..=b
a..
..
s.x	Named field access
s.0	Numbered field access

trait T {}
trait T : R {}
impl S {}
impl T for S {}
impl !T for S {}
fn f() {}
     fn f() -> S {}
     fn f(&self) {}
struct S(T);
const fn f() {}
async fn f() {}
     async fn f() -> S {}
     async { x }
fn() -> S
Fn() -> S
|| {}
     |x| {}
     |x| x + x
     move |x| x + y
     return || true
unsafe
     unsafe fn f() {}
     unsafe trait T {}
     unsafe { f(); }
     unsafe impl T for S {}

while x {}
loop {}
for x in collection {}
     collection.into_iter()
     iterator.next()
if x {} else {}
'label: {}
'label: loop {}
break
     break 'label x
     break 'label
     break x
continue
continue 'label
x?
x.await
     x.into_future()
     future.poll()
return x
     { return }
     || { return }
     async { return }
f()
x.f()
     X::f(x)
     X::f(&x)
     X::f(&mut x)
     S::f(&x)
     T::f(&x)
X::f()
     <X as T>::f()

a::b
     ::b
     crate::b
     self::b
     super::b
use a::b;
use a::{b, c};
use a::b as x;
use a::b as _;
use a::*;
pub use a::b;
pub T
     pub(crate) T
     pub(super) T
     pub(self) T
     pub(in a::b) T
extern crate a;
     self: Box<Self>
<S as T>
a::b as c
x as u32

m!()
$x:ty
$x
$(x),*
     $(x),?
     $(x),+
     $(x)<<+

match m {}
let S(x) = get();
     let S { x } = s;
     let (_, b, _) = abc;
     let (a, ..) = abc;
     let (.., a, b) = (1, 2);
     let s @ S { x } = get();
     let w @ t @ f = get();
     let (|x| x) = get();
let Some(x) = get();
let Some(x) = get() else {};
if let Some(x) = get() {}
while let Some(x) = get() {}
fn f(S { x }: S)

E::A => {}
E::B ( .. ) => {}
E::C { .. } => {}
S { x: 0, y: 1 } => {}
S { x: a, y: b } => {}
     S { x, y } => {}
S { .. } => {}
D => {}
D => {}
_ => {}
0 | 1 => {}
     E::A | E::Z => {}
     E::C {x} | E::D {x} => {}
     Some(A | B) => {}
     |x| x => {}
(a, 0) => {}
[a, 0] => {}
     [1, ..] => {}
     [1, .., 5] => {}
     [1, x @ .., 5] => {}
     [a, x @ .., b] => {}
1 .. 3 => {}
     1 ..= 3 => {}
     1 .. => {}
x @ 1..=5 => {}
     Err(x @ Error {..}) => {}
S { x } if x > 10 => {}

     where T: R + 'a
     where T: ?Sized
     where T: 'a
     where T: 'static
     where 'b: 'a
     where u8: R<T>
S<const N: usize>
     S<10>
     S<{5+5}>
S<T = R>
     S<const N: u8 = 0>
     S<T = u8>
S<'_>
S<_>
S::<T>
trait T { type X<'a>; }
impl<T> S<T> {}
impl S<T> {}
fn f(x: &impl T)
fn f(x: &dyn T)
fn f<X: T>(x: X)
fn f() where Self: R;
     fn f() where Self: Sized;
     fn f() where Self: R {}

for<'a>
     trait T: for<'a> R<'a> {}
fn(&'a u8)
for<'a> fn(&'a u8)
     fn(&'_ u8)
     fn(&u8)
dyn for<'a> Fn(&'a u8)
     dyn Fn(&'_ u8)
     dyn Fn(&u8)


impl<'a> T for fn(&'a u8) {}
impl T for for<'a> fn(&'a u8) {}
     impl T for fn(&u8) {}
r#"..."#
!
     fn f() -> ! {}
     fn f() -> Result<(), !> {}
     fn f(x: !) {}
_
     let _ = x;
     _ = x;
x;
*/
