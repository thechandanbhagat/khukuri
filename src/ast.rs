#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    // Statements
    Program(Vec<Box<ASTNode>>),
    VarDeclaration {
        name: String,
        type_hint: Option<String>,
        value: Box<ASTNode>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_block: Vec<Box<ASTNode>>,
        else_block: Option<Vec<Box<ASTNode>>>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    },
    ForEachLoop {
        variable: String,
        iterable: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Box<ASTNode>>,
    },
    Return(Box<ASTNode>),
    Print(Box<ASTNode>),
    Break,
    Continue,
    Import {
        filename: String,
    },
    
    // Expressions
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    UnaryOp {
        operator: String,
        operand: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Box<ASTNode>>,
    },
    ListLiteral(Vec<Box<ASTNode>>),
    DictionaryLiteral(Vec<(String, Box<ASTNode>)>), // key-value pairs
    IndexAccess {
        object: Box<ASTNode>,
        index: Box<ASTNode>,
    },
    IndexAssignment {
        object: Box<ASTNode>,
        index: Box<ASTNode>,
        value: Box<ASTNode>,
    },
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
}

impl ASTNode {
    pub fn new_program(statements: Vec<Box<ASTNode>>) -> Self {
        ASTNode::Program(statements)
    }
    
    pub fn new_var_declaration(name: String, type_hint: Option<String>, value: Box<ASTNode>) -> Self {
        ASTNode::VarDeclaration { name, type_hint, value }
    }
    
    pub fn new_assignment(name: String, value: Box<ASTNode>) -> Self {
        ASTNode::Assignment { name, value }
    }
    
    pub fn new_if_statement(
        condition: Box<ASTNode>,
        then_block: Vec<Box<ASTNode>>,
        else_block: Option<Vec<Box<ASTNode>>>,
    ) -> Self {
        ASTNode::IfStatement {
            condition,
            then_block,
            else_block,
        }
    }
    
    pub fn new_while_loop(condition: Box<ASTNode>, body: Vec<Box<ASTNode>>) -> Self {
        ASTNode::WhileLoop { condition, body }
    }
    
    pub fn new_for_each_loop(
        variable: String,
        iterable: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    ) -> Self {
        ASTNode::ForEachLoop { variable, iterable, body }
    }
    
    pub fn new_function_declaration(
        name: String,
        parameters: Vec<String>,
        body: Vec<Box<ASTNode>>,
    ) -> Self {
        ASTNode::FunctionDeclaration {
            name,
            parameters,
            body,
        }
    }
    
    pub fn new_binary_op(left: Box<ASTNode>, operator: String, right: Box<ASTNode>) -> Self {
        ASTNode::BinaryOp {
            left,
            operator,
            right,
        }
    }
    
    pub fn new_unary_op(operator: String, operand: Box<ASTNode>) -> Self {
        ASTNode::UnaryOp { operator, operand }
    }
    
    pub fn new_function_call(name: String, arguments: Vec<Box<ASTNode>>) -> Self {
        ASTNode::FunctionCall { name, arguments }
    }
    
    pub fn new_list_literal(elements: Vec<Box<ASTNode>>) -> Self {
        ASTNode::ListLiteral(elements)
    }
    
    pub fn new_dictionary_literal(pairs: Vec<(String, Box<ASTNode>)>) -> Self {
        ASTNode::DictionaryLiteral(pairs)
    }
    
    pub fn new_index_access(object: Box<ASTNode>, index: Box<ASTNode>) -> Self {
        ASTNode::IndexAccess { object, index }
    }
    
    pub fn new_import(filename: String) -> Self {
        ASTNode::Import { filename }
    }
    
    pub fn new_index_assignment(object: Box<ASTNode>, index: Box<ASTNode>, value: Box<ASTNode>) -> Self {
        ASTNode::IndexAssignment { object, index, value }
    }
}