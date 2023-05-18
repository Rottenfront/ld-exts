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
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    MacroRules,
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

    Colon,       // :
    DoubleColon, // ::

    Dollar,     // $
    Semicolon,  // ;
    BinOp,      // % ^ && || << >> / >= == <=
    Add,        // +
    Sub,        // -
    Set,        // =
    Refer,      // & *
    Tilda,      // ~
    At,         // @
    Backslash,  // \\
    Bang,       // !
    Quest,      // ?
    Hash,       // #
    HashBang,   // #!
    Arrow,      // ->
    MatchArrow, // =>
    SetOp,      // += -= /= *= ^= |= %= &= <<= >>=
    Comment,    // /* ... */

    Escape, // '\\' & (['\'', '"', '\\', '/', 'b', 'f', 'n', 'r', 't', '\n', '0'] | ('x' & HEX & HEX) | ("u{" & HEX & HEX & HEX & HEX & '}'))

    Ident,

    String, // todo: implement String in lexer
    Char,
    Lable,
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
        match $session.character() {
            '\'' | ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
            | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
            | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => {
                $session.submit();
                return Self::$i;
            }
            _ => {}
        }
    };

    ($session:expr, $i:ident, $f:expr $(, $o:expr )*) => {
        if $session.character() == $f {
            $session.advance();
            word!($session, $i $(, $o )*)
        }
    };
}

impl lady_deirdre::lexis::Token for RustToken {
    fn new(session: &mut impl LexisSession) -> Self {
        let mut state = 1usize;
        let mut string_hash_count = 0usize;
        let mut tmp_string_hash_count = 0usize;
        let mut comment_nesting = 0usize;
        loop {
            let current = session.character();
            session.advance();
            let next = session.character();
            match (state, current, next) {
                (1, '0'..='9', ch) => match ch {
                    '\'' | ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
                    | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
                    | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => {
                        session.submit();
                        return Self::Number;
                    },
                    _ => state = 2
                },
                (2, _, ch) => match ch {
                    '\'' | ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
                    | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
                    | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => {
                        session.submit();
                        return Self::Number;
                    }
                    _ => {}
                },
                (1 | 3, '\t' | '\u{b}'..='\r' | ' ', '\t' | '\u{b}'..='\r' | ' ') => state = 3,
                (1 | 3, '\t' | '\u{b}'..='\r' | ' ', _) => {
                    session.submit();
                    return Self::Whitespace;
                }
                (1 | 4, '\n', '\n') => state = 4,
                (1 | 4, '\n', _) => {
                    session.submit();
                    return Self::NewLine;
                }
                (1, '.', '.') => {
                    session.advance();
                    if session.character() == '=' {
                        session.advance();
                    }
                    session.submit();
                    return Self::Range;
                }
                (1, ':', ':') => {
                    session.advance();
                    session.submit();
                    return Self::DoubleColon;
                }
                (1, '&', '&') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1, '|', '|') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1, '#', '!') => {
                    session.advance();
                    session.submit();
                    return Self::HashBang;
                }
                (1, '=', '>') => {
                    session.advance();
                    session.submit();
                    return Self::MatchArrow;
                }
                (1, '-', '>') => {
                    session.advance();
                    session.submit();
                    return Self::Arrow;
                }
                (1, '+' | '-' | '/' | '*' | '^' | '|' | '%' | '&', '=') => {
                    session.advance();
                    session.submit();
                    return Self::SetOp;
                }
                /*
                (1, '>', '>') => {
                    session.advance();
                    if session.character() == '=' {
                        session.advance();
                        session.submit();
                        return Self::SetOp;
                    }
                    session.submit();
                    return Self::BinOp;
                }
                (1, '<', '<') => {
                    session.advance();
                    if session.character() == '=' {
                        session.advance();
                        session.submit();
                        return Self::SetOp;
                    }
                    session.submit();
                    return Self::BinOp;
                }
                 */
                (1, '>' | '=' | '<' | '!', '=') => {
                    session.advance();
                    session.submit();
                    return Self::BinOp;
                }
                (1, '/', '/') => {
                    session.advance();
                    if session.character() == '\n' {
                        session.advance();
                        session.submit();
                        return Self::Comment;
                    }
                    state = 12;
                }
                (1, '/', '*') => {
                    state = 13;
                    session.advance();
                }
                (12, ch, _) => {
                    if ch == '\n' {
                        session.submit();
                        return Self::Comment;
                    }
                }
                (13, ch1, ch2) => {
                    if ch1 == '*' && ch2 == '/' {
                        if comment_nesting > 0 {
                            comment_nesting -= 1;
                            session.advance();
                        } else {
                            session.advance();
                            session.submit();
                            return Self::Comment;
                        }
                    } else if ch1 == '/' && ch2 == '*' {
                        comment_nesting += 1;
                        session.advance();
                    }
                }
                (1, '(', _) => {
                    session.submit();
                    return Self::Open;
                }
                (1, ')', _) => {
                    session.submit();
                    return Self::Close;
                }
                (1, '<', _) => {
                    session.submit();
                    return Self::Less;
                }
                (1, '>', _) => {
                    session.submit();
                    return Self::Greater;
                }
                (1, '{', _) => {
                    session.submit();
                    return Self::BraceOpen;
                }
                (1, '}', _) => {
                    session.submit();
                    return Self::BraceClose;
                }
                (1, '[', _) => {
                    session.submit();
                    return Self::BracketOpen;
                }
                (1, ']', _) => {
                    session.submit();
                    return Self::BracketClose;
                }
                (1, '.', _) => {
                    session.submit();
                    return Self::Point;
                }
                (1, ':', _) => {
                    session.submit();
                    return Self::Colon;
                }
                (1, '$', _) => {
                    session.submit();
                    return Self::Dollar;
                }
                (1, ',', _) => {
                    session.submit();
                    return Self::Comma;
                }
                (1, ';', _) => {
                    session.submit();
                    return Self::Semicolon;
                }
                (1, '+', _) => {
                    session.submit();
                    return Self::Add;
                }
                (1, '=', _) => {
                    session.submit();
                    return Self::Set;
                }
                (1, '&' | '*', _) => {
                    session.submit();
                    return Self::Refer;
                }
                (1, '/' | '%' | '^' | '|', _) => {
                    session.submit();
                    return Self::BinOp;
                }
                (1, '~', _) => {
                    session.submit();
                    return Self::Tilda;
                }
                (1, '@', _) => {
                    session.submit();
                    return Self::At;
                }
                (1, '!', _) => {
                    session.submit();
                    return Self::Bang;
                }
                (1, '?', _) => {
                    session.submit();
                    return Self::Quest;
                }
                (1, '#', _) => {
                    session.submit();
                    return Self::Hash;
                }
                (1, '\\', '\'' | '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | '\n' | '0') => {
                    session.submit();
                    return Self::Escape;
                }
                (1, '\\', 'x') => {
                    advance!(session, 3);
                    session.submit();
                    return Self::Escape;
                }
                (1, '\\', 'u') => {
                    session.advance();
                    if session.character() == '{' {
                        state = 14;
                    } else {
                        session.submit();
                        return Self::Escape;
                    }
                }
                (14, ch, _) => {
                    if ch == '}' {
                        session.submit();
                        return Self::Escape;
                    }
                }

                (1, 'a', ch) => {
                    // 'a' 's'
                    // 'a' 's' 'y' 'n' 'c'
                    // 'a' 'w' 'a' 'i' 't'
                    match ch {
                        's' => {
                            session.advance();
                            word!(session, As);
                            word!(session, Async, 'y', 'n', 'c');
                        }
                        'w' => {
                            session.advance();
                            word!(session, Await, 'a', 'i', 't');
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'b', ch) => {
                    // 'b' 'r' 'e' 'a' 'k'
                    // 'b' 'o' 'o' 'l'
                    match ch {
                        'r' => {
                            session.advance();
                            match session.character() {
                                'e' => {
                                    session.advance();
                                    word!(session, Break, 'a', 'k');
                                }
                                '"' => {
                                    session.advance();
                                    if session.character() == '"' {
                                        session.advance();
                                        session.submit();
                                        return Self::String;
                                    }
                                    state = 11;
                                    continue;
                                }
                                '#' => {
                                    while session.character() == '#' {
                                        session.advance();
                                        string_hash_count += 1;
                                    }
                                    if session.character() == '"' {
                                        tmp_string_hash_count = string_hash_count;
                                        state = 11;
                                        continue;
                                    } else {
                                        state = 5;
                                        string_hash_count = 0;
                                        continue;
                                    }
                                }
                                _ => {}
                            }
                        }
                        'o' => {
                            session.advance();
                            word!(session, BasicType, 'o', 'l');
                        }
                        '\'' => {
                            state = 8;
                            continue;
                        }
                        '"' => {
                            state = 6;
                            continue;
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'c', _) => {
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
                    state = 5;
                }
                (1, 'd', _) => {
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
                    state = 5;
                }
                (1, 'e', _) => {
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
                    state = 5;
                }
                (1, 'f', ch) => {
                    // 'f' 'n'
                    // 'f' 'o' 'r'
                    // 'f' '3' '2'
                    // 'f' '6' '4'
                    match ch {
                        'n' => {
                            session.advance();
                            word!(session, Fn);
                        }
                        'o' => {
                            session.advance();
                            word!(session, For, 'r');
                        }
                        '3' => {
                            session.advance();
                            word!(session, BasicType, '2');
                        }
                        '6' => {
                            session.advance();
                            word!(session, BasicType, '4');
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'i', ch) => {
                    // 'i' 'f'
                    // 'i' 'm' 'p' 'l'
                    // 'i' 'n'
                    // 'i' 's' 'i' 'z' 'e'
                    // 'i' '1' '2' '8'
                    // 'i' '1' '6'
                    // 'i' '3' '2'
                    // 'i' '6' '4'
                    // 'i' '8'
                    match ch {
                        'f' => {
                            session.advance();
                            word!(session, If);
                        }
                        'm' => {
                            session.advance();
                            word!(session, Impl, 'p', 'l');
                        }
                        'n' => {
                            session.advance();
                            word!(session, In);
                        }
                        's' => {
                            session.advance();
                            word!(session, BasicType, 'i', 'z', 'e');
                        }
                        '1' => {
                            session.advance();
                            match session.character() {
                                '2' => {
                                    session.advance();
                                    word!(session, BasicType, '8');
                                }
                                '6' => {
                                    session.advance();
                                    word!(session, BasicType);
                                }
                                _ => {}
                            }
                        }
                        '3' => {
                            session.advance();
                            word!(session, BasicType, '2');
                        }
                        '6' => {
                            session.advance();
                            word!(session, BasicType, '4');
                        }
                        '8' => {
                            session.advance();
                            word!(session, BasicType, '8');
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'l', ch) => {
                    // 'l' 'e' 't'
                    // 'l' 'o' 'o' 'p'
                    match ch {
                        'e' => {
                            session.advance();
                            word!(session, Let, 't');
                        }
                        'o' => {
                            session.advance();
                            word!(session, Loop, 'o', 'p');
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'm', _) => {
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
                            word!(session, MacroRules, '_', 'r', 'u', 'l', 'e', 's', '!');
                        } else if session.character() == 't' {
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
                    state = 5;
                }
                (1, 'p', 'u') => {
                    // 'p' 'u' 'b'
                    session.advance();
                    word!(session, Pub, 'b');
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'r', _) => {
                    // 'r' 'e' 'f'
                    // 'r' 'e' 't' 'u' 'r' 'n'
                    if session.character() == 'e' {
                        session.advance();
                        if session.character() == 'f' {
                            session.advance();
                            word!(session, Ref);
                        } else if session.character() == 't' {
                            session.advance();
                            word!(session, Return, 'u', 'r', 'n');
                        }
                    } else if session.character() == '"' {
                        println!("abc");
                        session.advance();
                        if session.character() == '"' {
                            session.advance();
                            session.submit();
                            return Self::String;
                        }
                        state = 11;
                        continue;
                    } else if session.character() == '#' {
                        while session.character() == '#' {
                            session.advance();
                            string_hash_count += 1;
                        }
                        if session.character() == '"' {
                            tmp_string_hash_count = string_hash_count;
                            state = 11;
                            continue;
                        } else {
                            state = 5;
                            string_hash_count = 0;
                            continue;
                        }
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (11, ch, _) => {
                    'b: {
                        if ch == '"' {
                            while tmp_string_hash_count > 0 {
                                if session.character() != '#' {
                                    break 'b;
                                }
                                session.advance();
                                tmp_string_hash_count -= 1;
                            }
                            session.submit();
                            return Self::String;
                        }
                    }
                    'a: {
                        if session.character() == '"' {
                            while tmp_string_hash_count > 0 {
                                session.advance();
                                if session.character() != '#' {
                                    println!("bac");
                                    break 'a;
                                }
                                tmp_string_hash_count -= 1;
                            }
                            session.submit();
                            return Self::String;
                        }
                    }
                    tmp_string_hash_count = string_hash_count;
                }
                (1, 'S', 'e') => {
                    // 'S' 'e' 'l' 'f'
                    session.advance();
                    word!(session, USelf, 'l', 'f');
                    word!(session, Ident);
                    state = 5;
                }
                (1, 's', ch) => {
                    // 's' 'e' 'l' 'f'
                    // 's' 't' 'a' 't' 'i' 'c'
                    // 's' 't' 'r'
                    // 's' 't' 'r' 'u' 'c' 't'
                    // 's' 'u' 'p' 'e' 'r'
                    match ch {
                        'e' => {
                            session.advance();
                            word!(session, LSelf, 'l', 'f');
                        }
                        't' => {
                            session.advance();
                            match session.character() {
                                'a' => {
                                    session.advance();
                                    word!(session, Static, 't', 'i', 'c');
                                }
                                'r' => {
                                    session.advance();
                                    word!(session, BasicType);
                                    word!(session, Struct, 'u', 'c', 't');
                                }
                                _ => {}
                            }
                        }
                        'u' => {
                            session.advance();
                            word!(session, Super, 'p', 'e', 'r');
                        }
                        _ => {}
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 't', _) => {
                    // 't' 'r' 'a' 'i' 't'
                    // 't' 'r' 'y'
                    // 't' 'y' 'p' 'e'
                    if session.character() == 'r' {
                        session.advance();
                        if session.character() == 'a' {
                            session.advance();
                            word!(session, Trait, 'i', 't');
                        } else if session.character() == 'y' {
                            session.advance();
                            word!(session, Try);
                        }
                    } else if session.character() == 'y' {
                        session.advance();
                        word!(session, Type, 'p', 'e');
                    }
                    word!(session, Ident);
                    state = 5;
                }
                (1, 'u', _) => {
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

                    state = 5;
                }
                (1, 'w', 'h') => {
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
                    state = 5;
                }
                (1, 'y', 'i') => {
                    // 'y' 'i' 'e' 'l' 'd'
                    session.advance();
                    word!(session, Yield, 'e', 'l', 'd');
                    word!(session, Ident);
                    state = 5;
                }
                (1, '"', ch) => {
                    if ch == '"' {
                        session.advance();
                        session.submit();
                        return Self::String;
                    }
                    state = 6;
                }
                (6, ch1, ch2) => {
                    if ch1 != '\\' && ch2 == '"' {
                        session.advance();
                        session.submit();
                        return Self::String;
                    }
                    if ch1 == '\\' && ch2 == '\\' {
                        state = 7;
                    }
                }
                (7, _, ch2) => {
                    if ch2 == '"' {
                        session.advance();
                        session.submit();
                        return Self::String;
                    }
                    state = 6;
                }
                (1, '\'', ch) => match ch {
                    '\'' => {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
                    | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
                    | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => state = 8,
                    _ => {
                        session.advance();
                        match session.character() {
                            '\'' => {
                                session.advance();
                                session.submit();
                                return Self::Char;
                            }
                            ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>'
                            | '{' | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-'
                            | '%' | '^' | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!'
                            | '?' | '#' => {
                                session.submit();
                                return Self::Lable;
                            }
                            _ => state = 10,
                        }
                    }
                },
                (8, ch1, ch2) => {
                    if ch1 != '\\' && ch2 == '\'' {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    if ch1 == '\\' && ch2 == '\\' {
                        state = 9;
                    }
                }
                (9, _, ch) => {
                    if ch == '\'' {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    state = 8;
                }
                (10, _, ch) => match ch {
                    '\'' => {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
                    | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
                    | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => {
                        session.submit();
                        return Self::Lable;
                    }
                    _ => {}
                },
                (1 | 5, _, ch) => match ch {
                    '\'' | ' ' | '\t' | '\r' | '\x0b' | '\x0c' | '\n' | '(' | ')' | '<' | '>' | '{'
                    | '}' | '[' | ']' | ',' | '|' | '.' | ':' | '$' | ';' | '-' | '%' | '^'
                    | '=' | '+' | '&' | '*' | '/' | '~' | '@' | '\\' | '!' | '?' | '#' => {
                        session.submit();
                        return Self::Ident;
                    },
                    _ => state = 5
                },
                _ => break,
            }
        }
        match state {
            2 => Self::Number,
            6 | 7 => Self::String,
            10 => Self::Lable,
            8 | 9 | 11 => Self::Char,
            12 | 13 => Self::Comment,
            14 => Self::Escape,
            _ => Self::Ident,
        }
    }
}

impl Display for RustToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::As => "as",
            Self::MatchArrow => "=>",
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
            Self::Fn => "fn",
            Self::For => "for",
            Self::If => "if",
            Self::Impl => "impl",
            Self::In => "in",
            Self::Let => "let",
            Self::Loop => "loop",
            Self::Macro => "macro",
            Self::MacroRules => "macro_rules!",
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
            Self::Char => "'a'",
            Self::Lable => "'static",
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
            Self::Comment => "/* */",
        }
        .fmt(f)
    }
}
// ("r\"" & (ESCAPE | ^['"', '\\'])* & '\"')
// ("r#\"" & (ESCAPE | ^['"', '\\'])* & "\"#")
// ("r###\"" & (ESCAPE | ^['"', '\\'])* & "\"###")
