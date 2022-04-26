import enum

class TokenType(enum.Enum):
    CONSTANT = ["0", "1"]
    OPERATOR = "operator"
    VARIABLE = ["a", "b", "c", "d"]
    SPECIAL = [":=", ";"]

class Token:
    def __init__(self, s, id):
        self.text = s 
        self.id = id
        if self.text in TokenType.CONSTANT.value:
            self.token_type = "constant"
        elif self.text in TokenType.VARIABLE.value:
            self.token_type = "variable"
        elif self.text in TokenType.SPECIAL.value:
            self.token_type = "special symbol" 
        else:
            self.token_type = TokenType.OPERATOR.value

    def __repr__(self):
        return "Token " + str(self.id) + " = " + self.text + "\nToken type: " + self.token_type + "\n"
        


input = "c:=1 * 1 <= 0* 0 + 0*0;"
input = input.replace(" ", "")
id = 0
i = 0
while i < len(input):
    letter = input[i]
    if i < len(input) - 1 and input[i+1] == "=":
        letter = input[i] + input [i+1]
        i = i + 1
    tok = Token(letter, id)
    print(tok)
    id += 1
    i = i + 1

