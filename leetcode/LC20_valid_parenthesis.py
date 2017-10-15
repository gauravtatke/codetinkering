# Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
#
# The brackets must close in the correct order, "()" and "()[]{}" are all valid but "(]" and "([)]" are not.


def isValid(s):
    stack = []
    for parens in s:
        if parens in ('(', '{', '['):
            stack.append(parens)
        else:
            try:
                matching_parens = stack.pop()
            except IndexError:
                return False
            if parens == ')' and matching_parens != '(':
                return False
            elif parens == '}' and matching_parens != '{':
                return False
            elif parens == ']' and matching_parens != '[':
                return False
            else:
                continue
    return len(stack) == 0


def main():
    print(isValid('(){}[]'))
    print(isValid('(({})[])[]'))
    print(isValid('[{]}'))
    print(isValid(']'))


if __name__ == '__main__':
    main()
