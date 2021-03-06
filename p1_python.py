import enum

class TokenType(enum.Enum):
    # Assign enumerated types to their associated values
    CONSTANT = ["0", "1"]
    OPERATOR = "operator"
    VARIABLE = ["a", "b", "c", "d"]
    SPECIAL = [":=", ";"]

class Token:
    def __init__(self, s, id):
        self.text = s   # Assign self.text to input string
        self.id = id    # Assign self.id to user assigned id
        # Compare text to values associated to enums, assigns to matching one
        if self.text in TokenType.CONSTANT.value:
            self.token_type = "constant"
        elif self.text in TokenType.VARIABLE.value:
            self.token_type = "variable"
        elif self.text in TokenType.SPECIAL.value:
            self.token_type = "special symbol" 
        else:
            self.token_type = TokenType.OPERATOR.value

    def __repr__(self):
        # Create custom print function for object as required
        return "Token " + str(self.id) + " = " + self.text + "\nToken type: " + self.token_type + "\n"
        

def parse_and_return(inp):
    input = inp.replace(" ", "")
    id = 0
    i = 0
    tokens = []
    # Parsing through input string
    while i < len(input):
        letter = input[i]
        # Accounting for operators with two characters
        if i < len(input) - 1 and input[i+1] == "=" and input [i] in ["=", "<", ">", "!", ":"]:
            letter = input[i] + input [i+1]
            i = i + 1
        # Create Token obj for each token
        tok = Token(letter, id)
        # Append tok obj
        tokens.append(tok)
        id += 1
        i = i + 1

    return tokens

if __name__ == "__main__":
    input = "b:= 1*1;"
    list_of_tokens = parse_and_return(input)
    for tok in list_of_tokens:
        print(tok)





