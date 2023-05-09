use core::fmt::Display;
use lady_deirdre::lexis::Token;

#[derive(Clone, Copy, Debug, Token)]
pub enum CToken {
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
    #[rule("const")]
    Const,
    #[precedence(2)]
    #[rule("continue")]
    Continue,
    #[precedence(2)]
    #[rule("default")]
    Default,
    #[precedence(2)]
    #[rule("do")]
    Do,
    #[precedence(2)]
    #[rule("char" | "double" | "float" | "int" | "void")]
    BasicType,
    #[precedence(2)]
    #[rule("else")]
    Else,
    #[precedence(2)]
    #[rule("enum")]
    Enum,
    #[precedence(2)]
    #[rule("extern")]
    Extern,
    #[precedence(2)]
    #[rule("for")]
    For,
    #[precedence(2)]
    #[rule("goto")]
    Goto,
    #[precedence(2)]
    #[rule("if")]
    If,
    #[precedence(2)]
    #[rule("long" | "short" | "signed" | "unsigned")]
    TypeMod,
    #[precedence(2)]
    #[rule("register")]
    Register,
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
    #[rule("typedef")]
    Typedef,
    #[precedence(2)]
    #[rule("union")]
    Union,
    #[precedence(2)]
    #[rule("volatile")]
    Volatile,
    #[precedence(2)]
    #[rule("while")]
    While,

    #[precedence(3)]
    #[rule("0b" & ['0', '1']+ & ('L' | 'U' | 'l' | 'u')*)]
    BinNumber,
    #[precedence(3)]
    #[rule("0o" & ['0'..'7']+ & ('L' | 'U' | 'l' | 'u')*)]
    OctNumber,
    #[rule(['0'..'9']+ & ('.' & ['0'..'9']+)?
    & ('L' | 'U' | 'l' | 'u' | 'f' | 'F' | 'd' | 'D')*)]
    DecNumber,
    #[precedence(3)]
    #[rule("0x" & ['0'..'9', 'A'..'F', 'a'..'f']+ & ('L' | 'U' | 'l' | 'u')*)]
    HexNumber,

    // WHITESPACES
    #[rule([' ', '\t', '\r', '\x0b', '\x0c']+)]
    Whitespace,
    #[rule('\n'+)]
    NewLine,

    // BRACKETS
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

    // DELIMETERS
    #[rule(',')]
    Comma,
    #[rule('.')]
    Point,
    #[rule(':')]
    Colon,
    #[rule(';')]
    Semicolon,

    // CHARACTER & STRING
    #[rule('\'' & (^['\\', '\''] | ('\\' & ^['x'])
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '\''?)]
    Char,
    #[rule('"' & (^['\\', '"'] | ('\\' & ^['x'])
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '"'?)]
    String,

    // OPERATORS
    #[precedence(2)]
    #[rule(['-', '+', '&', '!'] | "<<" | ">>")]
    UnOp,
    #[rule(['%', '^', '|', '/'])]
    BinOp,
    #[precedence(3)]
    #[rule(['>', '<', '='] & '=')]
    BoolOp1,
    #[rule(['>', '<'])]
    BoolOp2,
    #[precedence(3)]
    #[rule("&&" | "||")]
    LogicOp,
    #[rule('=')]
    Set,
    #[precedence(3)]
    #[rule("++" | "--")]
    IncDec,

    // OTHER
    #[rule('*')]
    Star,
    #[rule('\\')]
    Backslash,
    #[rule('?')]
    QuestMark,

    #[rule('#')]
    Hash,

    #[precedence(3)]
    #[rule((['+', '-', '/', '*', '^', '|', '%', '&'] & '=') | "<<=" | ">>=")]
    SetOp,

    // COMMENTS
    #[precedence(5)]
    #[rule("//")]
    SingleComment,
    #[precedence(5)]
    #[rule("/*")]
    MultilineCommentOpen,
    #[precedence(5)]
    #[rule("*/")]
    MultilineCommentClose,

    #[rule(^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
    '}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', '0'..'9', ' ',
    '\t', '\r', '\x0b', '\x0c', '\n']
    & ^['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{',
    '}', '\\', '|', '\'', '"', ';', ':', '/', '?', ',', '.', '<', '>', '`', '~', ' ', '\t', '\r',
    '\x0b', '\x0c', '\n']*)]
    Ident,

    #[rule(['`', '@', '$', '~'])]
    Other,

    #[mismatch]
    Mismatch,
}
impl Display for CToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Auto => "auto",
            Self::Const => "const",
            Self::Extern => "extern",
            Self::Register => "register",
            Self::Static => "static",
            Self::Volatile => "volatile",
            Self::For => "for",
            Self::Do => "do",
            Self::While => "while",
            Self::Switch => "switch",
            Self::Case => "case",
            Self::Default => "default",
            Self::If => "if",
            Self::Else => "else",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Goto => "goto",
            Self::Return => "return",
            Self::Enum => "enum",
            Self::Struct => "struct",
            Self::Typedef => "typedef",
            Self::Union => "Union",
            Self::TypeMod => "MOD",
            Self::BasicType => "TYPE",
            Self::BinNumber | Self::DecNumber | Self::HexNumber | Self::OctNumber => "NUM",
            Self::Whitespace => " ",
            Self::NewLine => "\n",
            Self::ParenthesisOpen => "(",
            Self::ParenthesisClose => ")",
            Self::BraceOpen => "{",
            Self::BraceClose => "}",
            Self::BracketOpen => "[",
            Self::BracketClose => "]",
            Self::Comma => ",",
            Self::Point => ".",
            Self::Char => "CHAR",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::UnOp => "-",
            Self::BinOp => "+",
            Self::BoolOp1 => "==",
            Self::BoolOp2 => ">",
            Self::LogicOp => "&&",
            Self::IncDec => "++",
            Self::Set => "=",
            Self::Star => "*",
            Self::Backslash => "\\",
            Self::QuestMark => "?",
            Self::Hash => "#",
            Self::SetOp => "+=",
            Self::SingleComment => "//",
            Self::MultilineCommentOpen => "/*",
            Self::MultilineCommentClose => "*/",
            Self::Ident => "IDENT",
            Self::String => "STR",
            Self::Other => "OTHER",
            Self::Mismatch => "MISS",
        }
        .fmt(f)
    }
}
