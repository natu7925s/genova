use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Gene {
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        logic: Vec<Statement>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Assignment { var: String, value: Expression },
    Express { func_name: String, target: String },
    Print { var: String },
}

#[derive(Debug)]
pub enum Expression {
    StringLiteral(String),
    Var(String),
    BinaryOp {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn eat(&mut self, expected: &Token) {
        if self.current() == expected {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current());
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut genes = vec![];
        while *self.current() != Token::Eof {
            genes.push(self.parse_gene());
        }
        genes
    }

    fn parse_gene(&mut self) -> ASTNode {
        self.eat(&Token::Gene);
        let name = if let Token::Identifier(n) = self.current() {
            let result = n.clone();
            self.advance();
            result
        } else {
            panic!("Expected gene name");
        };
        self.eat(&Token::LBrace);

        let mut inputs = vec![];
        let mut outputs = vec![];
        let mut logic = vec![];

        while *self.current() != Token::RBrace {
            match self.current() {
                Token::Inputs => {
                    self.advance();
                    self.eat(&Token::Colon);
                    inputs = self.parse_identifier_list();
                }
                Token::Outputs => {
                    self.advance();
                    self.eat(&Token::Colon);
                    outputs = self.parse_identifier_list();
                }
                Token::Logic => {
                    self.advance();
                    self.eat(&Token::Colon);
                    logic = self.parse_logic_block();
                }
                _ => panic!("Unexpected token in gene block: {:?}", self.current()),
            }
        }

        self.eat(&Token::RBrace);

        ASTNode::Gene {
            name,
            inputs,
            outputs,
            logic,
        }
    }

    fn parse_identifier_list(&mut self) -> Vec<String> {
        let mut list = vec![];
        self.eat(&Token::LBracket);
        loop {
            match self.current() {
                Token::Identifier(name) => {
                    list.push(name.clone());
                    self.advance();
                    if *self.current() == Token::Comma {
                        self.advance();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        self.eat(&Token::RBracket);
        list
    }

    fn parse_logic_block(&mut self) -> Vec<Statement> {
        let mut stmts = vec![];
        while !matches!(self.current(), Token::Gene | Token::RBrace | Token::Eof) {
            match self.current() {
                Token::Identifier(_) => stmts.push(self.parse_assignment()),
                Token::Express => stmts.push(self.parse_express()),
                Token::Print => stmts.push(self.parse_print()),
                _ => panic!("Unknown statement: {:?}", self.current()),
            }
        }
        stmts
    }

    fn parse_assignment(&mut self) -> Statement {
        let var = if let Token::Identifier(name) = self.current() {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected variable name");
        };

        self.eat(&Token::Equals);
        let expr = self.parse_expression();
        Statement::Assignment { var, value: expr }
    }

    fn parse_express(&mut self) -> Statement {
        self.eat(&Token::Express);
        let func_name = if let Token::Identifier(name) = self.current() {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected function name after 'express'");
        };
        self.eat(&Token::Arrow);
        let target = if let Token::Identifier(name) = self.current() {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected target variable after '=>' ");
        };
        Statement::Express { func_name, target }
    }

    fn parse_print(&mut self) -> Statement {
        self.eat(&Token::Print);
        self.eat(&Token::LParen);
        let var = if let Token::Identifier(name) = self.current() {
            let name = name.clone();
            self.advance();
            name
        } else {
            panic!("Expected variable inside print()");
        };
        self.eat(&Token::RParen);
        Statement::Print { var }
    }

    fn parse_expression(&mut self) -> Expression {
        let mut left = match self.current() {
            Token::StringLiteral(s) => {
                let value = s.clone();
                self.advance();
                Expression::StringLiteral(value)
            }
            Token::Identifier(name) => {
                let value = name.clone();
                self.advance();
                Expression::Var(value)
            }
            _ => panic!("Expected expression"),
        };

        while *self.current() == Token::Plus {
            self.advance();
            let right = self.parse_expression();
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: "+".to_string(),
                right: Box::new(right),
            };
        }

        left
    }
}
