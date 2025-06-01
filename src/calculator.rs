use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidNumber(String),
    InvalidCharacter(char),
    UnexpectedToken(String),
    UnexpectedEndOfExpression,
    MissingClosingParenthesis,
    EmptyExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::InvalidCharacter(c) => write!(f, "Invalid character: '{}'", c),
            ParseError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
            ParseError::UnexpectedEndOfExpression => write!(f, "Unexpected end of expression"),
            ParseError::MissingClosingParenthesis => write!(f, "Missing closing parenthesis"),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Number(f32),
    Operator(char),
    EOF,
}

pub struct Lexer {
    tokens: Vec<Token>,
    current_token: usize,
}

impl Lexer {
    pub fn new(expr: &str) -> Result<Lexer, ParseError> {
        let tokens = Lexer::tokenize(expr)?;
        Ok(Lexer {
            tokens,
            current_token: 0,
        })
    }

    fn tokenize(expr: &str) -> Result<Vec<Token>, ParseError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current_number = String::new();
        let mut chars = expr.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    current_number.push(c);
                    chars.next();
                    
                    // Continue collecting the number
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_digit(10) || (next_c == '.' && !current_number.contains('.')) {
                            current_number.push(next_c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    // Parse the collected number
                    match current_number.parse::<f32>() {
                        Ok(num) => {
                            tokens.push(Token::Number(num));
                            
                            // Check for implicit multiplication: number followed by '('
                            // Skip any whitespace first
                            while let Some(&next_c) = chars.peek() {
                                if next_c.is_whitespace() {
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            
                            // If next non-whitespace char is '(', insert multiplication
                            if let Some(&'(') = chars.peek() {
                                tokens.push(Token::Operator('*'));
                            }
                        }
                        Err(_) => return Err(ParseError::InvalidNumber(current_number)),
                    }
                    current_number.clear();
                }
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    // Handle ')' followed by '(' or number for implicit multiplication
                    if c == ')' {
                        tokens.push(Token::Operator(c));
                        chars.next();
                        
                        // Skip whitespace
                        while let Some(&next_c) = chars.peek() {
                            if next_c.is_whitespace() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        
                        // Check if next is '(' or a number
                        if let Some(&next_c) = chars.peek() {
                            if next_c == '(' || next_c.is_digit(10) {
                                tokens.push(Token::Operator('*'));
                            }
                        }
                    } else {
                        tokens.push(Token::Operator(c));
                        chars.next();
                    }
                }
                ' ' | '\t' | '\n' => {
                    chars.next(); // Skip whitespace
                }
                _ => {
                    return Err(ParseError::InvalidCharacter(c));
                }
            }
        }

        if tokens.is_empty() {
            return Err(ParseError::EmptyExpression);
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }

    pub fn next(&mut self) -> Token {
        if self.current_token < self.tokens.len() {
            let token = self.tokens[self.current_token].clone();
            self.current_token += 1;
            token
        } else {
            Token::EOF
        }
    }

    pub fn peek(&self) -> Token {
        if self.current_token < self.tokens.len() {
            self.tokens[self.current_token].clone()
        } else {
            Token::EOF
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f32),
    UnaryOp { op: char, operand: Box<Expr> },
    BinaryOp { op: char, left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    pub fn evaluate(&self) -> Result<f32, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::UnaryOp { op, operand } => {
                let val = operand.evaluate()?;
                match op {
                    '+' => Ok(val),
                    '-' => Ok(-val),
                    _ => Err(format!("Unknown unary operator: {}", op)),
                }
            }
            Expr::BinaryOp { op, left, right } => {
                let left_val = left.evaluate()?;
                let right_val = right.evaluate()?;
                match op {
                    '+' => Ok(left_val + right_val),
                    '-' => Ok(left_val - right_val),
                    '*' => Ok(left_val * right_val),
                    '/' => {
                        if right_val == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    _ => Err(format!("Unknown binary operator: {}", op)),
                }
            }
        }
    }
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(expr: &str) -> Result<Parser, ParseError> {
        let lexer = Lexer::new(expr)?;
        Ok(Parser { lexer })
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expression(0)?;
        
        // Make sure we consumed all tokens
        match self.lexer.peek() {
            Token::EOF => Ok(expr),
            token => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
        }
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;

        loop {
            let token = self.lexer.peek();
            
            // Check if we should continue parsing
            let bp = match &token {
                Token::Operator(op) => self.infix_binding_power(*op),
                _ => None,
            };

            match bp {
                Some((left_bp, _)) if left_bp >= min_bp => {
                    // Continue parsing
                    self.lexer.next(); // Consume the operator
                    left = self.parse_infix(left, token)?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let token = self.lexer.next();
        
        match token {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Operator(op) => match op {
                '+' | '-' => {
                    let (_, right_bp) = self.prefix_binding_power(op);
                    let operand = self.parse_expression(right_bp)?;
                    Ok(Expr::UnaryOp {
                        op,
                        operand: Box::new(operand),
                    })
                }
                '(' => {
                    let expr = self.parse_expression(0)?;
                    match self.lexer.next() {
                        Token::Operator(')') => Ok(expr),
                        _ => Err(ParseError::MissingClosingParenthesis),
                    }
                }
                _ => Err(ParseError::UnexpectedToken(format!("operator '{}'", op))),
            },
            Token::EOF => Err(ParseError::UnexpectedEndOfExpression),
        }
    }

    fn parse_infix(&mut self, left: Expr, op_token: Token) -> Result<Expr, ParseError> {
        match op_token {
            Token::Operator(op) => {
                let (_, right_bp) = self.infix_binding_power(op)
                    .ok_or_else(|| ParseError::UnexpectedToken(format!("operator '{}'", op)))?;
                
                let right = self.parse_expression(right_bp)?;
                
                Ok(Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => Err(ParseError::UnexpectedToken(format!("{:?}", op_token))),
        }
    }

    fn prefix_binding_power(&self, op: char) -> ((), u8) {
        match op {
            '+' | '-' => ((), 9),
            _ => ((), 0),
        }
    }

    fn infix_binding_power(&self, op: char) -> Option<(u8, u8)> {
        let res = match op {
            '+' | '-' => (5, 6),  // Left associative
            '*' | '/' => (7, 8),  // Left associative, higher precedence
            _ => return None,
        };
        Some(res)
    }
}

pub struct Calculator {}

impl Calculator {
    pub fn str_2_f(expr: &str) -> Result<f32, String> {
        let mut parser = Parser::new(expr)
            .map_err(|e| e.to_string())?;
        
        let ast = parser.parse()
            .map_err(|e| e.to_string())?;
        
        ast.evaluate()
    }

    pub fn weighted_sum(grades: &[f32], weights: &[f32]) -> Option<f32> {
        if grades.len() != weights.len() || grades.is_empty() {
            return None;
        }
        
        let sum: f32 = grades.iter()
            .zip(weights.iter())
            .map(|(g, w)| g * w)
            .sum();
        
        Some(sum)
    }
}

// Helper function to parse and print AST
pub fn print_parsed(expr: &str) {
    match Parser::new(expr) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => {
                    println!("Expression: {}", expr);
                    println!("Parsed AST: {:#?}", ast);
                    match ast.evaluate() {
                        Ok(result) => println!("Result: {}", result),
                        Err(e) => println!("Evaluation error: {}", e),
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Err(e) => println!("Lexer error: {}", e),
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(Calculator::str_2_f("2 + 3").unwrap(), 5.0);
        assert_eq!(Calculator::str_2_f("10 - 4").unwrap(), 6.0);
        assert_eq!(Calculator::str_2_f("3 * 4").unwrap(), 12.0);
        assert_eq!(Calculator::str_2_f("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(Calculator::str_2_f("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(Calculator::str_2_f("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(Calculator::str_2_f("10 - 6 / 2").unwrap(), 7.0);
    }

    #[test]
    fn test_unary_operators() {
        assert_eq!(Calculator::str_2_f("-5").unwrap(), -5.0);
        assert_eq!(Calculator::str_2_f("+5").unwrap(), 5.0);
        assert_eq!(Calculator::str_2_f("-5 + 3").unwrap(), -2.0);
        assert_eq!(Calculator::str_2_f("-(5 + 3)").unwrap(), -8.0);
    }

    #[test]
    fn test_complex_expressions() {
        assert_eq!(Calculator::str_2_f("2 * (3 + 4) - 5").unwrap(), 9.0);
        assert_eq!(Calculator::str_2_f("10 / (2 + 3) * 4").unwrap(), 8.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(Calculator::str_2_f("(2 + 3) * (4 - 1)").unwrap(), 15.0);
        assert_eq!(Calculator::str_2_f("((2 + 3) * 4) - 5").unwrap(), 15.0);
        assert_eq!(Calculator::str_2_f("10 / (2 + (3 - 1))").unwrap(), 2.5);
    }

    #[test]
    fn test_implicit_multiplication() {
        assert_eq!(Calculator::str_2_f("2(3 + 4)").unwrap(), 14.0);
        assert_eq!(Calculator::str_2_f("5(2 + 3) - 10").unwrap(), 15.0);
        assert_eq!(Calculator::str_2_f("3(4 - 1) + 2").unwrap(), 11.0);
    }

    #[test]
    fn test_errors() {
        assert!(Calculator::str_2_f("2 +").is_err());
        assert!(Calculator::str_2_f("2 + + 3").is_err());
        assert!(Calculator::str_2_f("(2 + 3").is_err());
        assert!(Calculator::str_2_f("2 $ 3").is_err());
        assert!(Calculator::str_2_f("").is_err());
    }

    #[test]
    fn test_weighted_sum() {
        let grades = vec![85.0, 90.0, 78.0];
        let weights = vec![0.3, 0.4, 0.3];
        assert_eq!(Calculator::weighted_sum(&grades, &weights), Some(84.9));
        
        // Test error cases
        assert_eq!(Calculator::weighted_sum(&vec![], &vec![]), None);
        assert_eq!(Calculator::weighted_sum(&vec![1.0], &vec![1.0, 2.0]), None);
    }
}