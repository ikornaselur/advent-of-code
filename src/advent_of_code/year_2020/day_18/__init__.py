from collections import deque
from enum import Enum
from typing import Deque, List, Optional, Tuple

from advent_of_code import Problem


class TokenType(Enum):
    left_bracket = "left_bracket"
    right_bracket = "right_bracket"
    plus = "plus"
    mult = "mult"
    number = "number"


class Token:
    value: Optional[int]
    token_type: TokenType
    _raw: str

    def __init__(self: "Token", token: str) -> None:
        if token.isdigit():
            self.value = int(token)
            self.token_type = TokenType.number
        elif token == "(":
            self.token_type = TokenType.left_bracket
        elif token == ")":
            self.token_type = TokenType.right_bracket
        elif token == "+":
            self.token_type = TokenType.plus
        elif token == "*":
            self.token_type = TokenType.mult
        else:
            raise Exception(f"Unknown token: {token}")
        self._raw = token

    def __repr__(self: "Token") -> str:
        return f"<Token {self.token_type.value}: {self._raw}>"


def tokenise(string: str) -> List[Token]:
    tokens = []
    buffer = ""
    for char in string:
        if char == " ":
            if buffer:
                tokens.append(Token(buffer))
            buffer = ""
        elif char == "(":
            tokens.append(Token(char))
        elif char == ")":
            if buffer:
                tokens.append(Token(buffer))
            tokens.append(Token(char))
            buffer = ""
        else:
            buffer += char
    if buffer:
        tokens.append(Token(buffer))

    return tokens


def calculate_no_precedence(tokens: List[Token]) -> int:
    stack: List[Tuple[int, Optional[TokenType]]] = [(0, None)]  # Value, Method

    last_method = None
    for token in tokens:
        if token.token_type == TokenType.left_bracket:
            stack.append((0, last_method))
            last_method = None
        elif token.token_type == TokenType.right_bracket:
            top, method = stack.pop()
            if method == TokenType.plus:
                stack[-1] = (stack[-1][0] + top, stack[-1][1])
            elif method == TokenType.mult:
                stack[-1] = (stack[-1][0] * top, stack[-1][1])
            else:
                stack[-1] = (top, stack[-1][1])
        elif token.token_type == TokenType.plus:
            last_method = TokenType.plus
        elif token.token_type == TokenType.mult:
            last_method = TokenType.mult
        elif token.token_type == TokenType.number:
            assert token.value is not None
            if last_method == TokenType.plus:
                stack[-1] = (stack[-1][0] + token.value, stack[-1][1])
            elif last_method == TokenType.mult:
                stack[-1] = (stack[-1][0] * token.value, stack[-1][1])
            else:
                stack[-1] = (token.value, stack[-1][1])

        else:
            raise Exception(f"Unknown token type: {token}")

    if len(stack) > 1:
        raise Exception(f"Stack not unwound: {stack}")

    return stack[0][0]


def calculate_reverse_precedence(tokens: List[Token]) -> int:
    # Convert to postfix notation
    postfix: List[Token] = []
    stack: Deque[Token] = deque()

    for token in tokens:
        if token.token_type == TokenType.number:
            postfix.append(token)
        elif token.token_type in (TokenType.plus, TokenType.mult):
            if not len(stack) or stack[-1].token_type == TokenType.left_bracket:
                stack.append(token)
            else:
                valid_operands = [token.token_type]
                if token.token_type == TokenType.mult:  # Reversed for this problem
                    valid_operands.append(TokenType.plus)
                while len(stack) and stack[-1].token_type in valid_operands:
                    postfix.append(stack.pop())
                stack.append(token)
        elif token.token_type == TokenType.left_bracket:
            stack.append(token)
        elif token.token_type == TokenType.right_bracket:
            while stack[-1].token_type != TokenType.left_bracket:
                postfix.append(stack.pop())
            stack.pop()  # Drop the bracket

    while len(stack) and stack[-1].token_type != TokenType.left_bracket:
        postfix.append(stack.pop())

    # Calculate the output
    output_stack: Deque[Token] = deque()
    for token in postfix[::-1]:
        output_stack.append(token)

        while (
            len(output_stack) > 2
            and output_stack[-1].token_type == output_stack[-2].token_type == TokenType.number
        ):
            a = output_stack.pop()
            b = output_stack.pop()
            op = output_stack.pop()

            assert a.value is not None
            assert b.value is not None

            if op.token_type == TokenType.plus:
                output_stack.append(Token(f"{a.value + b.value}"))
            elif op.token_type == TokenType.mult:
                output_stack.append(Token(f"{a.value * b.value}"))
            else:
                raise Exception(f"Invalid {a=} {b=} {op=}")

    assert len(output_stack) == 1
    assert output_stack[0].value is not None

    return output_stack[0].value


class Part1(Problem):
    def get_solution(self: "Part1") -> int:
        puzzle_input = self.get_input(__file__)

        return sum(calculate_no_precedence(tokenise(line)) for line in puzzle_input)


class Part2(Problem):
    def get_solution(self: "Part2") -> int:
        puzzle_input = self.get_input(__file__)

        return sum(calculate_reverse_precedence(tokenise(line)) for line in puzzle_input)
