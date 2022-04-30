// define the enums
enum TokenType {
    CONSTANT,
    OPERATOR,
    VARIABLE,
    SPECIAL,
}

// Assign values to the enumerated types
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
    text: String,
    id: i32,
    token_type: String
}

impl Token {
    fn new(u_input: &str, u_id: i32) -> Token {
        // Figure out which token type the input belongs to
        let temp_text;
        let constants = vec!["0", "1"];
        let variables = vec!["a", "b", "c", "d"];
        let specials = vec![":=", ";"];
        // Check if input belongs to constants
        if constants.contains(&(u_input)) {
            temp_text = TokenType::CONSTANT.as_str().to_string();
        }
        // Check if input belongs to variables
        else if variables.contains(&(u_input)) {
            temp_text = TokenType::VARIABLE.as_str().to_string();
        }
        // Check if input belongs to specials
        else if specials.contains(&(u_input)) {
            temp_text = TokenType::SPECIAL.as_str().to_string();
        }
        else {
            temp_text = TokenType::OPERATOR.as_str().to_string();
        }
        Token {
            text: u_input.to_string(),
            id: u_id,   // assign id to user assigned id
            token_type: temp_text.to_string() // assign token_type
        }
    }

    fn print_self(&self) {
        println!("Token {} = {}", self.id, self.text);
        println!("Token type: {}\n", self.token_type);
    }
}

fn parse_and_return(inp: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input:String = inp.to_string();
    // Potential operators with two chars
    let ops = vec!["=".to_string(), "<".to_string(), ">".to_string(), "!".to_string(), ":".to_string()];
    input = input.replace(" ", "");
    let len:i32 = input.chars().count() as i32;
    let mut  i:i32 = 0;
    let mut id:i32 = 0;
    // Go through each token
    while i < len {
        let mut letter:String = input.chars().nth(i as usize).unwrap().to_string();
        // Account for operators that have two chars
        if i < len - 1 && input.chars().nth((i + 1) as usize).unwrap().to_string() == "=" && ops.contains(&letter) {
            let b_letter:&str = "=";
            letter.push_str(b_letter);
            i += 1
        }
        // Create new Token object
        let tok = Token::new(&letter, id);
        // Append token to tokens vector
        tokens.push(tok);
        id += 1;
        i += 1;

    }

    return tokens;
}


fn main() {
    let inp:String = "b:= 1*1;".to_string();
    let tokens:Vec<Token> = parse_and_return(inp);
    for tok in &tokens {
        tok.print_self();
    }
    
   
}
