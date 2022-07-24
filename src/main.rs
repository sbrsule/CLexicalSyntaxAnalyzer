use std::fs::read_to_string;
use std::path::Path;
use std::env::args;
use regex::{Regex, RegexSet};

fn main() {
    let args : Vec<String> = args().collect();
    if args.len() == 1 {
        panic!("Must include input file in arguments!");
    }
    let content = read_to_string(Path::new(&args[1]))
        .expect("Unable to read file!")
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();

    lexical_analyze(content);
}

fn lexical_analyze(content: Vec<String>) {
    let re = RegexSet::new(&[
        "^\"[^\"]*\"$",
        "^[a-zA-Z]*\\([^{}]*\\);?$",
        "^[\\{\\}:\\(\\)=<>\\+\\-\\*/]$",
        "^(==|<=|>=)$",
        "^[a-zA-Z][a-zA-Z0-9]*(=|<|>|==|<=|>=)[0-9]*;?$",   
        "^[a-zA-Z0-9]*;?$",
        "^[a-zA-Z][a-zA-Z0-9]*(--|\\+\\+);?$",
    ]).unwrap();
    for c in content.iter() {
        println!("{}", c);
        if !re.is_match(c) {
            panic!("Lexical Error in file!");
        }
    }
    println!("File is lexically correct!");
    syntax_analyse(content);
}

fn syntax_analyse(content: Vec<String>) {
    let mut current_index: usize = 0;
    if content[current_index] == "VOID" {
        current_index += 1;
        if content[current_index] == "MAIN()" {
            current_index += 1;
            if content[current_index] == "{" {
                current_index += 1;
                statements(content, current_index);
            }
        }
    }
}

fn statements(content: Vec<String>, mut current_index: usize) {
    match &content[current_index] as &str {
        "switch" => switch_statement(content, current_index+1),
        "for" => for_loop(content, current_index+1),
        "while" => while_loop(content, current_index+1),
        "if" => if_statement(content, current_index+1),
        "ASG" => assignment(content, current_index+1),
        "return" => return_statement(content, current_index+1),
        _ => panic!("Syntax Error, invalid statement!"),
    }
}

fn assignment(content: Vec<String>, mut current_index: usize) {
    let mut expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    if expression.is_match(&content[current_index]) {
        current_index += 1;
        if content[current_index] == "=" {
            current_index += 1;
            expression = Regex::new("^[0-9]*;$").unwrap();
            if expression.is_match(&content[current_index]) {
                current_index += 1;
                statements(content, current_index);
            } else {
                panic!("Syntax Error, invalid statement!");
            }
        } else {
            panic!("Syntax Error, invalid statement!");
        }
    } else {
        panic!("Syntax Error, invalid statement!");
    }
}

fn while_loop(content: Vec<String>, mut current_index: usize) {
    if content[current_index] == "(" {
        current_index += 1;
        let expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*(==|<|>|<=|>=)[0-9]*$").unwrap();
        if expression.is_match(&content[current_index]) {
            current_index += 1;
            if content[current_index] == ")" {
                current_index += 1;
                if content[current_index] == "{" {
                    current_index += 1;
                    while content[current_index] == "statement(s);"  {
                        current_index += 1;
                    }
                    if content[current_index] == "}" {
                        current_index +=1;
                        statements(content, current_index);
                    } else {
                        panic!("Syntax Error, invalid statement!");
                    }
                } else {
                    panic!("Syntax Error, invalid statement!");
                }
            } else {
                panic!("Syntax Error, invalid statement!");
            }
        } else {
            panic!("Syntax Error, invalid statement!");
        }
    } else {
        panic!("Syntax Error, invalid statement!");
    }
}

fn switch_statement(content: Vec<String>, mut current_index: usize) {
    let expression = Regex::new("^\\([a-zA-Z][a-zA-Z0-9]*\\)$").unwrap();
    if expression.is_match(&content[current_index]) {
        current_index += 1;
        if content[current_index] == "{" {
            current_index += 1;
            while content[current_index] == "case" {
                current_index += 1;
                if expression.is_match(&content[current_index]) {
                    current_index += 1;
                    if content[current_index] == ":" {
                        current_index += 1;
                        while content[current_index] == "statement(s);" {
                            current_index += 1;
                        }
                        if content[current_index] == "break;" {
                            current_index += 1;
                        }
                    } else {
                        panic!("Syntax Error, invalid statement!");
                    }
                } else {
                    panic!("Syntax Error, invalid statement!");
                }
            }
            if content[current_index] == "default" {
                current_index += 1;
                if content[current_index] == ":" {
                    current_index += 1;
                    while content[current_index] == "statement(s);" {
                        current_index += 1;
                    }
                    if content[current_index] == "break;" {
                        current_index += 1;
                    }
                } else {
                    panic!("Syntax Error, invalid statement!");
                }
            } 
        } else {
            panic!("Syntax Error, invalid statement!");
        }
    } else {
        panic!("Syntax Error, invalid statement!");
    }

    if content[current_index] == "}" {
        current_index += 1;
        statements(content, current_index);
    } else {
        panic!("Syntax Error, invalid statement!");
    }
}


fn for_loop(content: Vec<String>, mut current_index: usize) {
    if content[current_index] == "(" {
        current_index += 1;
        if content[current_index] == "int" {
            current_index += 1;
            let mut expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*=[0-9]*;$").unwrap();
            if expression.is_match(&content[current_index]) {
                current_index += 1;
                expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*(<|>|=|<=|>=)[0-9]*;$").unwrap();
                if expression.is_match(&content[current_index]) {
                    current_index += 1;
                    expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*(((\\+|-|\\*|/)[0-9]*)|(\\+\\+|--))$").unwrap();
                    if expression.is_match(&content[current_index]) {
                        current_index += 1;
                        if content[current_index] == ")" {
                            current_index += 1;
                            if content[current_index] == "{" {
                                current_index += 1;
                                while content[current_index] == "statement(s);" {
                                    current_index += 1;
                                }
                                if content[current_index] == "}" {
                                    current_index += 1;
                                    statements(content, current_index);
                                } else {
                                    panic!("Syntax Error, invalid statement!");
                                }
                            } else {
                                panic!("Syntax Error, invalid statement!");
                            }
                        } else {
                            panic!("Syntax Error, invalid statement!");
                        }
                    } else {
                        panic!("Syntax Error, invalid statement!");
                    }
                } else {
                   panic!("Syntax Error, invalid statement!");
                }
            } else {
                panic!("Syntax Error, invalid statement!");
            }
        } else {
            panic!("Syntax Error, invalid statement!");
        }
    } else {
        panic!("Syntax Error, invalid statement!");
    }
}

fn if_statement(content: Vec<String>, mut current_index: usize) {
    if content[current_index] == "(" {
        current_index += 1;
        let expression = Regex::new("^[a-zA-Z][a-zA-Z0-9]*((<|>|==|<=|>=)[0-9]*)?$").unwrap();
        if expression.is_match(&content[current_index]) {
            current_index += 1;
            if content[current_index] == ")" {
                current_index += 1;
                if content[current_index] == "{" {
                    current_index += 1;
                    while content[current_index] == "statement(s);" {
                        current_index += 1;
                    }
                    if content[current_index] == "}" {
                        current_index += 1;
                        statements(content, current_index);
                    } else {
                        panic!("Syntax Error, invalid statement!");
                    }
                } else {
                    panic!("Syntax Error, invalid statement!");
                }
            } else {
                panic!("Syntax Error, invalid statement!");
            }
        } else {
            panic!("Syntax Error, invalid statement!");
        }
    } else {
        panic!("Syntax Error, invalid statement!");
    }
}


fn return_statement(content: Vec<String>, mut current_index: usize) {
    let re = Regex::new("^[a-zA-Z0-9]+;$").unwrap();
    if re.is_match(&content[current_index]) {
        current_index += 1;
        if content[current_index] == "}" {
            println!("File is syntactically correct!");
        }
    }
}