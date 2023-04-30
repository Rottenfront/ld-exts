use core::fmt::Display;
use lady_deirdre::lexis::LexisSession;

#[derive(Clone, Copy, Debug)]
pub enum CToken {
    Auto,

    Break,

    Case,
    Const,
    Continue,

    Default,
    Do,

    Else,
    Enum,
    Extern,

    For,

    Goto,

    If,

    Long,

    Register,
    Return,

    Short,
    Signed,
    Static,
    Struct,
    Switch,

    Typedef,

    Union,
    Unsigned,

    Volatile,

    While,

    Number,

    Whitespace,
    NewLine,

    ParenthO,
    ParenthC,
    BraceO,
    BraceC,
    BracketO,
    BracketC,

    Comma,
    Point,
    Colon,
    Semicolon,

    Char,
    String,

    BinOp,
    UnOp,
    IncDec,
    BoolOp,
    LogicOp,
    Set,

    Amp,
    Star,
    Backslash,
    Bang,
    QuestMark,
    Tilda,
    Hash,
    At,
    Dollar,

    OpWithSet,

    SingleComment,
    MlCommentO,
    MlCommentC,

    Ident,
    Other,
    Mismatch,
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
    ($session:expr, $i:ident, $f1:expr, $f2:expr, $f3:expr, $f4:expr, $f5:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i, $f2, $f3, $f4, $f5)
        }
    };
    ($session:expr, $i:ident, $f1:expr, $f2:expr, $f3:expr, $f4:expr, $f5:expr, $f6:expr) => {
        if $session.character() == $f1 {
            $session.advance();
            word!($session, $i, $f2, $f3, $f4, $f5, $f6)
        }
    };
}

impl lady_deirdre::lexis::Token for CToken {
    fn new(session: &mut impl LexisSession) -> Self {
        let mut state = 1usize;
        loop {
            let current = session.character();
            session.advance();
            let next = session.character();
            match (state, current, next) {
                (1usize, '0', 'b' | 'o' | 'x') => {
                    session.advance();
                    while (session.character() >= '0' && session.character() <= '9')
                        || (session.character() >= 'a' && session.character() <= 'f')
                        || (session.character() >= 'A' && session.character() <= 'F')
                        || session.character() == '_' {
                        session.advance();
                    }
                    if session.character() == 'l' || session.character() == 'u' {
                        while (session.character() >= '0' && session.character() <= '9')
                            || (session.character() >= 'a' && session.character() <= 'z') {
                            session.advance();
                        }
                    }
                    session.submit();
                    return Self::Number;
                }

                (1usize | 2usize, '0'..='9', '0'..='9' | '.') => state = 2usize,
                (1usize, '0'..='9', _) => {
                    session.submit();
                    return Self::Number;
                }
                (2usize, '0'..='9' | '.', ch) => {
                    if ch == 'f' || ch == 'F' {
                        session.advance();
                    }
                    session.submit();
                    return Self::Number;
                }

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
                (1usize, '+' | '-' | '/' | '*' | '^' | '|' | '%' | '&', '=') => {
                    session.advance();
                    session.submit();
                    return Self::OpWithSet;
                }
                (1usize, '+', '+') => {
                    session.advance();
                    session.submit();
                    return Self::IncDec;
                }
                (1usize, '-', '-') => {
                    session.advance();
                    session.submit();
                    return Self::IncDec;
                }
                (1usize, '&', '&') => {
                    session.advance();
                    session.submit();
                    return Self::LogicOp;
                }
                (1usize, '|', '|') => {
                    session.advance();
                    session.submit();
                    return Self::LogicOp;
                }
                (1usize, '>' | '=' | '<', '=') => {
                    session.advance();
                    session.submit();
                    return Self::BoolOp;
                }
                (1usize, '>' | '<', _) => {
                    session.submit();
                    return Self::BoolOp;
                }
                (1usize, '/', '/') => {
                    session.advance();
                    session.submit();
                    return Self::SingleComment;
                }
                (1usize, '/', '*') => {
                    session.advance();
                    session.submit();
                    return Self::MlCommentO;
                }
                (1usize, '*', '/') => {
                    session.advance();
                    session.submit();
                    return Self::MlCommentC;
                }
                (1usize, '(', _) => {
                    session.submit();
                    return Self::ParenthO;
                }
                (1usize, ')', _) => {
                    session.submit();
                    return Self::ParenthC;
                }
                (1usize, '{', _) => {
                    session.submit();
                    return Self::BraceO;
                }
                (1usize, '}', _) => {
                    session.submit();
                    return Self::BraceC;
                }
                (1usize, '[', _) => {
                    session.submit();
                    return Self::BracketO;
                }
                (1usize, ']', _) => {
                    session.submit();
                    return Self::BracketC;
                }
                (1usize, '.', _) => {
                    session.submit();
                    return Self::Point;
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
                (1usize, '=', _) => {
                    session.submit();
                    return Self::Set;
                }
                (1usize, '&', _) => {
                    session.submit();
                    return Self::Amp;
                }
                (1usize, '*', _) => {
                    session.submit();
                    return Self::Star;
                }
                (1usize, '/' | '|' | '^' | '%', _) => {
                    session.submit();
                    return Self::BinOp;
                }
                (1usize, '!' | '+' | '-', _) => {
                    session.submit();
                    return Self::UnOp;
                }
                (1usize, '~', _) => {
                    session.submit();
                    return Self::Tilda;
                }
                (1usize, '@', _) => {
                    session.submit();
                    return Self::At;
                }
                (1usize, '?', _) => {
                    session.submit();
                    return Self::QuestMark;
                }
                (1usize, '#', _) => {
                    session.submit();
                    return Self::Hash;
                }
                (1usize, 'a', 'u') => {
                    session.advance();
                    word!(session, Auto, 't', 'o');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'b', 'r') => {
                    // 'b' 'r' 'e' 'a' 'k'
                    session.advance();
                    word!(session, Break, 'e', 'a', 'k');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'c', _) => {
                    // 'c' 'a' 's' 'e'
                    // 'c' 'o' 'n' 's' 't'
                    // 'c' 'o' 'n' 't' 'i' 'n' 'u' 'e'

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
                    } else if session.character() == 'a' {
                        session.advance();
                        word!(session, Case, 's', 'e');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'd', _) => {
                    // 'd' 'o'
                    if session.character() == 'o' {
                        session.advance();
                        word!(session, Do);
                    } else if session.character() == 'e' {
                        session.advance();
                        word!(session, Default, 'f', 'a', 'u', 'l', 't');
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
                (1usize, 'f', 'o') => {
                    // 'f' 'o' 'r'
                    session.advance();
                    word!(session, For, 'r');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'g', 'o') => {
                    // 'g' 'o' 't' 'o'
                    session.advance();
                    word!(session, Goto, 't', 'o');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'i', 'f') => {
                    // 'i' 'f'
                    session.advance();
                    word!(session, If);
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'l', 'o') => {
                    // 'l' 'o' 'n' 'g'
                    session.advance();
                    word!(session, Long, 'o', 'p');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'r', 'e') => {
                    // 'r' 'e' 'g' 'i' 's' 't' 'e' 'r'
                    // 'r' 'e' 't' 'u' 'r' 'n'
                    session.advance();
                    if session.character() == 'f' {
                        session.advance();
                        word!(session, Register, 'i', 's', 't', 'e', 'r');
                    } else if session.character() == 't' {
                        session.advance();
                        word!(session, Return, 'u', 'r', 'n');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 's', _) => {
                    // 's' 'h' 'o' 'r' 't'
                    // 's' 'i' 'g' 'n' 'e' 'd'
                    // 's' 't' 'a' 't' 'i' 'c'
                    // 's' 't' 'r' 'u' 'c' 't'
                    // 's' 'w' 'i' 't' 'c' 'h'
                    if session.character() == 'h' {
                        session.advance();
                        word!(session, Short, 'o', 'r', 't');
                    } else if session.character() == 't' {
                        session.advance();
                        if session.character() == 'a' {
                            session.advance();
                            word!(session, Static, 't', 'i', 'c');
                        } else if session.character() == 'r' {
                            session.advance();
                            word!(session, Struct, 'u', 'c', 't');
                        }
                    } else if session.character() == 'w' {
                        session.advance();
                        word!(session, Switch, 'i', 't', 'c', 'h');
                    }
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 't', 'y') => {
                    // 't' 'y' 'p' 'e' 'd' 'e' 'f'
                    session.advance();
                    word!(session, Typedef, 'p', 'e', 'd', 'e', 'f');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'u', _) => {
                    // 'u' 'n' 'i' 'o' 'n'
                    // 'u' 'n' 's' 'i' 'g' 'n' 'e' 'd'
                    session.advance();
                    if session.character() == 'i' {
                        session.advance();
                        word!(session, Union, 'o', 'n');
                    } else if session.character() == 's' {
                        session.advance();
                        word!(session, Unsigned, 'i', 'g', 'n', 'e', 'd');
                    }
                    word!(session, Ident);

                    state = 5usize;
                }
                (1usize, 'v', 'o') => {
                    // 'v' 'o' 'l' 'a' 't' 'i' 'l' 'e'
                    session.advance();
                    word!(session, Volatile, 'l', 'a', 't', 'i', 'l', 'e');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, 'w', 'h') => {
                    // 'w' 'h' 'i' 'l' 'e'
                    session.advance();
                    word!(session, While, 'i', 'l', 'e');
                    word!(session, Ident);
                    state = 5usize;
                }
                (1usize, '"', ch) => {
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
                (1usize, '\'', ch) => {
                    if ch == '\'' {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    state = 8;
                }
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
                (9, _, ch2) => {
                    if ch2 == '\'' {
                        session.advance();
                        session.submit();
                        return Self::Char;
                    }
                    state = 8;
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
        Self::Mismatch
    }
}
impl Display for CToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => "Auto",
            Self::Const => "Const",
            Self::Extern => "Extern",
            Self::Register => "Register",
            Self::Static => "Static",
            Self::Volatile => "Volatile",
            Self::For => "For",
            Self::Do => "Do",
            Self::While => "While",
            Self::Switch => "Switch",
            Self::Case => "Case",
            Self::Default => "Default",
            Self::If => "If",
            Self::Else => "Else",
            Self::Break => "Break",
            Self::Continue => "Continue",
            Self::Goto => "Goto",
            Self::Return => "Return",
            Self::Enum => "Enum",
            Self::Struct => "Struct",
            Self::Typedef => "Typedef",
            Self::Union => "Union",
            Self::Short => "Short",
            Self::Long => "Long",
            Self::Signed => "Signed",
            Self::Unsigned => "Unsigned",
            Self::Number => "NUM",
            Self::Whitespace => " ",
            Self::NewLine => " ",
            Self::ParenthO => "(",
            Self::ParenthC => ")",
            Self::BraceO => "{",
            Self::BraceC => "}",
            Self::BracketO => "[",
            Self::BracketC => "]",
            Self::Comma => ",",
            Self::Point => ".",
            Self::Char => "CHAR",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::BinOp => "*",
            Self::UnOp => "+",
            Self::IncDec => "++",
            Self::BoolOp => "==",
            Self::LogicOp => "&&",
            Self::Set => "=",
            Self::Amp => "&",
            Self::Star => "*",
            Self::Backslash => "\\",
            Self::Bang => "!",
            Self::QuestMark => "?",
            Self::Hash => "#",
            Self::Tilda => "~",
            Self::At => "@",
            Self::Dollar => "$",
            Self::OpWithSet => "+=",
            Self::SingleComment => "//",
            Self::MlCommentO => "/*",
            Self::MlCommentC => "*/",
            Self::Ident => "IDENT",
            Self::String => "STR",
            Self::Other => "OTHER",
            Self::Mismatch => "MISS",
        }.fmt(f)
    }
}
