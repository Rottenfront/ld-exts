use core::fmt::Display;
use lady_deirdre::lexis::LexisSession;
use lady_deirdre::lexis::Token;

#[derive(Clone, Copy, Debug, Token)]
pub enum CToken {
    // ATTRIBUTES
    #[precedence(2)]
    #[rule("auto")]
    Auto,
    #[precedence(2)]
    #[rule("const")]
    Const,
    #[precedence(2)]
    #[rule("extern")]
    Extern,
    #[precedence(2)]
    #[rule("register")]
    Register,
    #[precedence(2)]
    #[rule("static")]
    Static,
    #[precedence(2)]
    #[rule("volatile")]
    Volatile,

    // CYCLES
    #[precedence(2)]
    #[rule("for")]
    For,
    #[precedence(2)]
    #[rule("do")]
    Do,
    #[precedence(2)]
    #[rule("while")]
    While,

    // SWITCH
    #[precedence(2)]
    #[rule("switch")]
    Switch,
    #[precedence(2)]
    #[rule("case")]
    Case,
    #[precedence(2)]
    #[rule("default")]
    Default,

    // IF
    #[precedence(2)]
    #[rule("if")]
    If,
    #[precedence(2)]
    #[rule("else")]
    Else,


    #[precedence(2)]
    #[rule("break")]
    Break,
    #[precedence(2)]
    #[rule("continue")]
    Continue,
    #[precedence(2)]
    #[rule("goto")]
    Goto,
    #[precedence(2)]
    #[rule("return")]
    Return,

    // TYPES
    #[precedence(2)]
    #[rule("enum")]
    Enum,
    #[precedence(2)]
    #[rule("struct")]
    Struct,
    #[precedence(2)]
    #[rule("typedef")]
    Typedef,
    #[precedence(2)]
    #[rule("union")]
    Union,

    // TYPE MODIFIERS
    #[precedence(2)]
    #[rule("short")]
    Short,
    #[precedence(2)]
    #[rule("long")]
    Long,
    #[precedence(2)]
    #[rule("signed")]
    Signed,
    #[precedence(2)]
    #[rule("unsigned")]
    Unsigned,

    // NUMBERS
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
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '\'')]
    Char,
    #[rule('"' & (^['\\', '"'] | ('\\' & ^['x'])
    | ("\\x" & ['0'..'9', 'A'..'F', 'a'..'f'] & ['0'..'9', 'A'..'F', 'a'..'f']))* & '"')]
    String,

    // OPERATORS
    #[rule(['-', '%', '^', '|', '+'])]
    Operator,
    #[precedence(3)]
    #[rule((['>', '<'] & '='?) | "==")]
    BoolOperator,
    #[rule('=')]
    Assign,

    // OTHER
    #[rule('&')]
    Amp,
    #[rule('*')]
    Star,
    #[rule('\\')]
    Backslash,
    #[rule('!')]
    Bang,
    #[rule('?')]
    QuestMark,

    #[rule('#')]
    Hash,

    #[precedence(3)]
    #[rule('#' & (^['\n'] | "\\\n")+ & '\n')]
    Preprocessor,

    #[precedence(3)]
    #[rule(['+', '-', '/', '*', '^', '|', '%', '&'] & '=')]
    AssignWithOperation,

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
    Identifier,

    #[rule(['`', '@', '$', '~']+)]
    Other,

    #[mismatch]
    Mismatch,
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
            Self::BinNumber => "bNUM",
            Self::OctNumber => "oNUM",
            Self::DecNumber => "dNUM",
            Self::HexNumber => "xNUM",
            Self::Whitespace => " ",
            Self::NewLine => " ",
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
            Self::Operator => "+",
            Self::BoolOperator => "==",
            Self::Assign => "=",
            Self::Amp => "&",
            Self::Star => "*",
            Self::Backslash => "\\",
            Self::Bang => "!",
            Self::QuestMark => "?",
            Self::Hash => "#",
            Self::Preprocessor => "PREPROC",
            Self::AssignWithOperation => "+=",
            Self::SingleComment => "//",
            Self::MultilineCommentOpen => "/*",
            Self::MultilineCommentClose => "*/",
            Self::Identifier => "IDENT",
            Self::String => "STR",
            Self::Other => "OTHER",
            Self::Mismatch => "MISS",
        }.fmt(f)
    }
}
