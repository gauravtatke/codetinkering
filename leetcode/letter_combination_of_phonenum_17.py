import sys

# Given a digit string, return all possible letter combinations that the number could represent.
#
# A mapping of digit to letters (just like on the telephone buttons) is given below.
#
# Input:Digit string "23"
# Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
#
# Note:
# Although the above answer is in lexicographical order, your answer could be in any order you want.


def letterCombinations(digits):
    mapping = [
        '0', '1', 'abc', 'def', 'ghi', 'jkl', 'mno', 'pqrs', 'tuv', 'wxyz'
    ]
    result = []
    bstr = ['']
    for digit in digits:
        astr = mapping[int(digit)]
        result = []
        for ch1 in bstr:
            for ch2 in astr:
                result.append(ch1 + ch2)
        bstr = result
    return result


def letterCombinations_recursive(digits):
    result = []
    keys = ['0', '1', 'abc', 'def', 'ghi', 'jkl', 'mno', 'pqrs', 'tuv', 'wxyz']
    if digits == '':
        return result
    generateCombination('', digits, 0, result, keys)
    return result


def generateCombination(prefix, digits, offset, result, keys):
    if offset >= len(digits):
        result.append(prefix)
        return
    astr = keys[int(digits[offset])]
    for ch in astr:
        generateCombination(prefix + ch, digits, offset + 1, result, keys)


def main():
    print(letterCombinations('234'))
    print(letterCombinations_recursive(''))


if __name__ == '__main__':
    main()
