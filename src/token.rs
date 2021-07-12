#[derive(Debug)]
pub enum TokenType {
    Undefined,

    // Commands
    IfCommand,
    ElseCommand,
    ForCommand,
    WhileCommand,
    DoCommand,
    SwitchCommand,
    CaseCommand,
    DefaultCommand,
    BreakCommand,
    ContinueCommand,
    ReturnCommand,
    GotoCommand,
    Label,
    TempVariable,
    
    // Boolean Operators
    AndOperator,
    OrOperator,
    NotOperator,

    // Comparsion Operators
    EqualToOperator,
    NotEqualToOperator,
    LessThanOperator,
    GreaterThanOperator,
    LessThanOrEqualToOperator,
    GreaterThanOrEqualToOperator,

    // Arithmetic Operators
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    ModuleOperator,
    AssignmentOperator,
    PowOperator,

    // Compound Operators
    IncrementOperator,
    DecrementOperator,
    CompoundAdditionOperator,
    CompoundSubtractionOperator,
    CompoundMultiplicationOperator,
    CompoundDivisionOperator,
    CompoundModuleOperator,

    // Identifier
    Identifier,

    // Syntax Symbols
    Colon,
    Semicolon,
    Comma,
    LeftBracers,
    RightBracers,
    LeftParenthesis,
    RightParenthesis,
    LeftBrackets,
    RightBrackets,

    // Types
    IntType,
    FloatType,
    LongType,
    ShortType,
    CharType,
    UnsignedType,
    VoidType,
    StructType,

    // Constants
    BooleanConstant,
    IntegerConstant,
    LongConstant,
    FloatingPointConstant,
    CharConstant,
    StringConstant
}

pub struct Pattern {
    pub value: Option<String>,
    pub token_type: Option<TokenType>
}

pub fn get_patterns() -> Vec<Pattern> {
    vec![
        Pattern {
            value: Some("([{])".to_string()),
            token_type: Some(TokenType::LeftBracers)
        },
        Pattern {
            value: Some("([}])".to_string()),
            token_type: Some(TokenType::RightBracers)
        },
        Pattern {
            value: Some("(\\d+[l])".to_string()),
            token_type: Some(TokenType::LongConstant)
        },
        Pattern {
            value: Some("([-+]?[0-9]*\\.?[0-9]*([.]|[E])[-+]?[0-9]*)".to_string()),
            token_type: Some(TokenType::FloatingPointConstant)
        },
        Pattern {
            value: Some("('(.)')".to_string()),
            token_type: Some(TokenType::CharConstant)
        },
        Pattern {
            value: Some("(\"([^\"]|\\.)*\")".to_string()),
            token_type: Some(TokenType::StringConstant)
        },
        Pattern {
            value: Some("([_]*[a-zA-Z][0-9]*)+".to_string()),
            token_type: Some(TokenType::Identifier)
        },
        Pattern {
            value: Some("(\\d+)".to_string()),
            token_type: Some(TokenType::IntegerConstant)
        },
        Pattern {
            value: Some("([(])".to_string()),
            token_type: Some(TokenType::LeftParenthesis)
        },
        Pattern {
            value: Some("([)])".to_string()),
            token_type: Some(TokenType::RightParenthesis)
        },
        Pattern {
            value: Some("([>][=])".to_string()),
            token_type: Some(TokenType::GreaterThanOrEqualToOperator)
        },
        Pattern {
            value: Some("([<][=])".to_string()),
            token_type: Some(TokenType::LessThanOrEqualToOperator)
        },
        Pattern {
            value: Some("([<])".to_string()),
            token_type: Some(TokenType::LessThanOperator)
        },
        Pattern {
            value: Some("([>])".to_string()),
            token_type: Some(TokenType::GreaterThanOperator)
        },
        Pattern {
            value: Some("([\\^])".to_string()),
            token_type: Some(TokenType::PowOperator)
        },
        Pattern {
            value: Some("([=][=])".to_string()),
            token_type: Some(TokenType::EqualToOperator)
        },
        Pattern {
            value: Some("([|][|])".to_string()),
            token_type: Some(TokenType::OrOperator)
        },
        Pattern {
            value: Some("([=])".to_string()),
            token_type: Some(TokenType::AssignmentOperator)
        },
        Pattern {
            value: Some("([+])".to_string()),
            token_type: Some(TokenType::AdditionOperator)
        },
        Pattern {
            value: Some("([-])".to_string()),
            token_type: Some(TokenType::SubtractionOperator)
        },
        Pattern {
            value: Some("([*])".to_string()),
            token_type: Some(TokenType::MultiplicationOperator)
        },
        Pattern {
            value: Some("([&][&])".to_string()),
            token_type: Some(TokenType::AndOperator)
        },
        Pattern {
            value: Some("([!][=])".to_string()),
            token_type: Some(TokenType::NotEqualToOperator)
        },
        Pattern {
            value: Some("([%])".to_string()),
            token_type: Some(TokenType::ModuleOperator)
        },
        Pattern {
            value: Some("([/])".to_string()),
            token_type: Some(TokenType::DivisionOperator)
        },
        Pattern {
            value: Some("([;])".to_string()),
            token_type: Some(TokenType::Semicolon)
        },
        Pattern {
            value: Some("([\\[])".to_string()),
            token_type: Some(TokenType::LeftBrackets)
        },
        Pattern {
            value: Some("([]])".to_string()),
            token_type: Some(TokenType::RightBrackets)
        },
        Pattern {
            value: Some("([,])".to_string()),
            token_type: Some(TokenType::Comma)
        }
    ]
}