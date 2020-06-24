# Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
#
# For example, given n = 3, a solution set is:
#
# [
#   "((()))",
#   "(()())",
#   "(())()",
#   "()(())",
#   "()()()"
# ]


def generateParenthesis_backtrack2(num):
    result = []
    backtrack(result, '', 0, 0, num)
    return result


def backtrack(result, res_str, open_paren, close_paren, max_paren):
    if len(res_str) == 2 * max_paren:
        result.append(res_str)
        return
    if open_paren < max_paren:
        backtrack(result, res_str + '(', open_paren + 1, close_paren,
                  max_paren)
    if close_paren < open_paren:
        backtrack(result, res_str + ')', open_paren, close_paren + 1,
                  max_paren)


def generateParenthesis_backtrack1(num):
    result = []
    addParens(result, "", num, 0)
    return result


def addParens(result, res_str, nopen, nclose):
    # use 2 ints to count number on open and close parens ar remaining to be added.
    # At each recursive call, add a open parens if nopen>0 and add a close parens if nclose>0
    # once both nopen=nclose=0, then add the string to result
    if nopen == 0 and nclose == 0:
        result.append(res_str)
        return
    if nopen > 0:
        addParens(result, res_str + '(', nopen - 1, nclose + 1)
    if nclose > 0:
        addParens(result, res_str + ')', nopen, nclose - 1)


def generateParenthesis_DP(n):
    # Each new set will introduce extra '()' in prev set. f(n) will having extra '()' to f(n-1)
    # for any n>=2, nth pair ca be added to f(n-1) such that i pairs are inside nth parens and n-1-i pairs are outside
    # f(0) = ''
    # f(1) = "('f(0)')"
    # f(2) = "('f(0)')"f(1), f(0)"('f(1)')"
    # f(3) = "('f(0)')"f(2), "(f(1))"f(1), f(0)"('f(2)')"
    comp_list = [['']]
    for num in range(1, n + 1):
        res = []
        # res stores 'num' set of parenthesis and it is later added to comp_list
        # thus comp_list has solution to all num=0 to num=n
        for i in range(num):
            for left in comp_list[num - 1 - i]:
                for right in comp_list[i]:
                    res.append('(' + left + ')' + right)
        comp_list.append(res)
    return comp_list[-1]


def main():
    # print(generateParenthesis_backtrack2(1))
    # print(generateParenthesis_backtrack2(2))
    print(generateParenthesis_backtrack2(3))

    # print(generateParenthesis_backtrack1(1))
    # print(generateParenthesis_backtrack1(2))
    # print(generateParenthesis_backtrack1(3))

    # print(generateParenthesis_DP(1))
    # print(generateParenthesis_DP(2))
    # print(generateParenthesis_DP(3))


if __name__ == '__main__':
    main()
