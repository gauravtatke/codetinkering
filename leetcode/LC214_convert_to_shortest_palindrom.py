import sys

#  Given a string S, you are allowed to convert it to a palindrome by adding
# characters in front of it. Find and return the shortest palindrome you can find by performing this transformation.
# For example:
# Given "aacecaaa", return "aaacecaaa".
# Given "abcd", return "dcbabcd".


def convertToShortestPalindrome(s):
    # find max length palindrom includes index 0 (start of s). get the other
    # end of the palindrom. reverse the string[end+:] and prefix it with s
    if len(s) == 1 or len(s) == 0:
        return s

    max_palin_len = 0
    max_rindex = 0
    for pos in range(len(s) // 2 + 1):
        # find max left and right index from pos where s[left:right] is a
        # palindrome with ch in the middle
        # find odd length palindrome
        plength, lindex, rindex = commonElements(s, pos, pos)
        if plength > max_palin_len and lindex == 0:
            max_palin_len = plength
            max_rindex = rindex

        # find even length palindrome
        plength, lindex, rindex = commonElements(s, pos, pos + 1)
        if plength > max_palin_len and lindex == 0:
            max_palin_len = plength
            max_rindex = rindex

    # prefix can be -> prefix = s[-1: max_rindex: -1]
    prefix = ''
    for ch in s[max_rindex + 1:]:
        prefix = ch + prefix

    return prefix + s


def commonElements(s, lpos, rpos):
    if lpos == rpos:
        # means odd length, initialize count to 1
        count = 1
        lpos = lpos - 1
        rpos = rpos + 1
    else:
        count = 0

    while lpos >= 0 and rpos < len(s):
        if s[lpos] == s[rpos]:
            count = count + 2
            lpos = lpos - 1
            rpos = rpos + 1
        else:
            break

    # lpos and rpos are one index too far so adjust
    return count, lpos + 1, rpos - 1


def shortestPalindrome(s):
    # another of finding a palindrome from starting is to reverse the original str
    # for i 0 to n if s[0:n-i] == rev[i:] then s[0:i] is the longest palindrome
    # from starting.
    n = len(s)
    if n == 0 or n == 1:
        return s
    rev = s[-1::-1]
    for i in range(n):
        if s[0:i] == rev[i:]:
            return rev[0:i] + s


def shortestPalindrome_Rec(s):
    # Let us consider 2 pointers i and j. Initialize i=0. Iterate over j from
    # n−1 to 0, incrementing i each time s[i]==s[j]. Now, we just need to
    # search in range [0,i). This way, we have reduced the size of string to
    # search for the largest palindrome substring from the beginning. The
    # range [0,i) must always contain the largest palindrome substring.
    i = 0
    for j in range(len(s) - 1, -1, -1):
        # print(j)
        if s[i] == s[j]:
            i = i + 1

    if i == len(s):
        # means s is a palindrome
        return s
    else:
        prefix = s[-1:i - 1:-1]
        suffix = s[i:]
        rem_str = s[0:i]
        # print('i={}, prefix={}, suffix={}, rem_str={}'.format(
        #     i, prefix, suffix, rem_str))
        return prefix + shortestPalindrome_Rec(rem_str) + suffix


def main():
    s = shortestPalindrome_Rec('aacecaaa')
    print(s)


if __name__ == '__main__':
    main()
