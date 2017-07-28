#!/usr/bin/env python3

# Given a string, find the first non-repeating character in it. For
# example, if the input string is “GeeksforGeeks”, then output should be
# ‘f’ and if input string is “GeeksQuiz”, then output should be ‘G’.


def findNonRepeatChar(stri):
    chlist = [0 for ch in range(256)]
    for ch in stri:
        chlist[ord(ch)] += 1

    for ch in stri:
        if chlist[ord(ch)] == 1:
            return ch
    return None


def findNonRepeatCharFromCount(str1):
    # store count of char and index at which it first occured in count list
    count = [[0, None] for i in range(256)]
    for i, ch in enumerate(str1):
        count[ord(ch)][0] += 1
        if not count[ord(ch)][1]:
            # if index is None, set it else leave as it is
            count[ord(ch)][1] = i

    # now only traverse count list i.e. 256 elements instead of whole string
    # again which could be very long
    maxi = 257
    result = None
    for cnt, indx in count:
        if cnt == 1 and indx < maxi:
            result = str1[indx]
            maxi = indx
    return result


def main(argv):
    print(findNonRepeatChar(argv[0]))
    print(findNonRepeatCharFromCount(argv[0]))
    return 0


if __name__ == '__main__':
    import sys
    sys.exit(main(sys.argv[1:]))
