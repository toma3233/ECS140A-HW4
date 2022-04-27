enum TokenType {
    CONSTANT,
    OPERATOR,
    VARIABLE,
    SPECIAL,
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match self {
            TokenType::CONSTANT => "constant",
            TokenType::OPERATOR => "operator",
            TokenType::VARIABLE => "variable",
            TokenType::SPECIAL => "special symbol"
        }
    }
}

struct Token {
    input: String,
    id: i32,
    text: String
}

impl Token {
    fn new(u_input: &str, u_id: i32) -> Token {

        let temp_text;
        let constants = vec!["0", "1"];
        let variables = vec!["a", "b", "c", "d"];
        let specials = vec![":=", ";"];
        if constants.contains(&(u_input)) {
            temp_text = TokenType::CONSTANT.as_str().to_string();
        }
        else if variables.contains(&(u_input)) {
            temp_text = TokenType::VARIABLE.as_str().to_string();
        }
        else if specials.contains(&(u_input)) {
            temp_text = TokenType::SPECIAL.as_str().to_string();
        }
        else {
            temp_text = TokenType::OPERATOR.as_str().to_string();
        }
        Token {
            input: u_input.to_string(),
            id: u_id,
            text: temp_text.to_string()
        }
    }

    fn print_self(&self) {
        println!("Token {} = {}", self.id, self.input);
        println!("Token type: {}\n", self.text);
    }
}


fn main() {
    let mut input:String = "d:= 1*1 / 1 * 0 == 0;".to_string();
    let ops = vec!["=".to_string(), "<".to_string(), ">".to_string(), "!".to_string(), ":".to_string()];
    input = input.replace(" ", "");
    let len:i32 = input.chars().count() as i32;
    let mut  i:i32 = 0;
    let mut id:i32 = 0;
    while i < len {
        let mut letter:String = input.chars().nth(i as usize).unwrap().to_string();
        if i < len - 1 && input.chars().nth((i + 1) as usize).unwrap().to_string() == "=" && ops.contains(&letter) {
            let b_letter:&str = "=";
            letter.push_str(b_letter);
            i += 1
        }

        let tok = Token::new(&letter, id);
        tok.print_self();
        id += 1;
        i += 1;

    }
   
}
