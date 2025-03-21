#[derive(Clone)]
pub enum Query {
    Add { name: String, department: String },
    ListDepartment(String),
    ListCompany,
    Exit,
    Help,
    Invalid,
}

pub struct Parser(Query);

impl Parser {
    pub fn new(query: &str) -> Self {
        Parser(Parser::parse(query))
    }

    pub fn parsed(&self) -> Query {
        (self.0).clone()
    }

    fn parse(query: &str) -> Query {
        let tokens: Vec<&str> = query.split_whitespace().collect();
        match tokens.len() {
            1 => Parser::parse_single_token_command(tokens[0]),
            2 => Parser::parse_two_token_command((tokens[0], tokens[1])),
            4 => Parser::parse_quad_token_command((tokens[0], tokens[1], tokens[2], tokens[3])),
            _ => return Query::Invalid,
        }
    }

    fn parse_single_token_command(token: &str) -> Query {
        match token.to_lowercase().as_str() {
            "exit" => Query::Exit,
            "list" => Query::ListCompany,
            "help" => Query::Help,
            _ => Query::Invalid,
        }
    }

    fn parse_two_token_command((command, specifier): (&str, &str)) -> Query {
        match command.to_lowercase().as_str() {
            "list" => Query::ListDepartment(String::from(specifier)),
            _ => Query::Invalid,
        }
    }

    fn parse_quad_token_command(tokens: (&str, &str, &str, &str)) -> Query {
        match tokens.0.to_lowercase().as_str() {
            "add" => Query::Add {
                name: String::from(tokens.1),
                department: String::from(tokens.3),
            },
            _ => Query::Invalid,
        }
    }
}
#[cfg(test)]

mod parser_tests {
    use super::*;

    #[test]
    fn test_add_query() {
        let parser = Parser::new("Add Sally to Engineering");
        match parser.parsed() {
            Query::Add { name, department } => {
                assert_eq!(name, "Sally");
                assert_eq!(department, "Engineering");
            }
            _ => panic!("Expected Add query"),
        }
    }

    #[test]
    fn test_list_department() {
        let parser = Parser::new("List Engineering");
        match parser.parsed() {
            Query::ListDepartment(dept) => {
                assert_eq!(dept, "Engineering");
            }
            _ => panic!("Expected ListDepartment query"),
        }
    }

    #[test]
    fn test_list_company() {
        let parser = Parser::new("List");
        match parser.parsed() {
            Query::ListCompany => (),
            _ => panic!("Expected ListCompany query"),
        }
    }

    #[test]
    fn test_exit() {
        let parser = Parser::new("Exit");
        match parser.parsed() {
            Query::Exit => (),
            _ => panic!("Expected Exit query"),
        }
    }

    #[test]
    fn test_invalid_query() {
        let parser = Parser::new("invalid command");
        match parser.parsed() {
            Query::Invalid => (),
            _ => panic!("Expected Invalid query"),
        }
    }
    #[test]
    fn test_case_insensitive_add() {
        let parser = Parser::new("ADD John TO Sales");
        match parser.parsed() {
            Query::Add { name, department } => {
                assert_eq!(name, "John");
                assert_eq!(department, "Sales");
            }
            _ => panic!("Expected Add query"),
        }
    }

    #[test]
    fn test_case_insensitive_list_department() {
        let parser = Parser::new("LIST Sales");
        match parser.parsed() {
            Query::ListDepartment(dept) => {
                assert_eq!(dept, "Sales");
            }
            _ => panic!("Expected ListDepartment query"),
        }
    }

    #[test]
    fn test_case_insensitive_list_company() {
        let parser = Parser::new("LIST");
        match parser.parsed() {
            Query::ListCompany => (),
            _ => panic!("Expected ListCompany query"),
        }
    }

    #[test]
    fn test_case_insensitive_exit() {
        let parser = Parser::new("EXIT");
        match parser.parsed() {
            Query::Exit => (),
            _ => panic!("Expected Exit query"),
        }
    }
    #[test]
    fn test_lowercase_add() {
        let parser = Parser::new("add Bob to Marketing");
        match parser.parsed() {
            Query::Add { name, department } => {
                assert_eq!(name, "Bob");
                assert_eq!(department, "Marketing");
            }
            _ => panic!("Expected Add query"),
        }
    }

    #[test]
    fn test_lowercase_list_department() {
        let parser = Parser::new("list Marketing");
        match parser.parsed() {
            Query::ListDepartment(dept) => {
                assert_eq!(dept, "Marketing");
            }
            _ => panic!("Expected ListDepartment query"),
        }
    }

    #[test]
    fn test_lowercase_list() {
        let parser = Parser::new("list");
        match parser.parsed() {
            Query::ListCompany => (),
            _ => panic!("Expected ListCompany query"),
        }
    }

    #[test]
    fn test_lowercase_exit() {
        let parser = Parser::new("exit");
        match parser.parsed() {
            Query::Exit => (),
            _ => panic!("Expected Exit query"),
        }
    }

    #[test]
    fn test_mixed_case_add() {
        let parser = Parser::new("AdD Alice tO Finance");
        match parser.parsed() {
            Query::Add { name, department } => {
                assert_eq!(name, "Alice");
                assert_eq!(department, "Finance");
            }
            _ => panic!("Expected Add query"),
        }
    }
}
