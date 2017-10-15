# Write a function that takes a string as input and reverse only the vowels of a string.
#
# Example 1:
# Given s = "hello", return "holle".
#
# Example 2:
# Given s = "leetcode", return "leotcede".
#
# Note:
# The vowels does not include the letter "y".


def reverseVowels(s):
    lstr = list(s)
    i = 0
    j = len(lstr) - 1
    vowels = ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U')
    while i < j:
        while i < j and lstr[i] not in vowels:
            i = i + 1
        while i < j and lstr[j] not in vowels:
            j = j - 1
        lstr[i], lstr[j] = lstr[j], lstr[i]
        # print(i, j)
        i = i + 1
        j = j - 1
    return ''.join(lstr)


if __name__ == '__main__':
    print(reverseVowels('hello'))
