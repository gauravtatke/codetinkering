#  A message containing letters from A-Z is being encoded to numbers using the following mapping:
# 'A' -> 1
# 'B' -> 2
# ...
# 'Z' -> 26

# Given an encoded message containing digits, determine the total number of ways to decode it.
# For example,
# Given encoded message "12", it could be decoded as "AB" (1 2) or "L" (12).
# The number of ways decoding "12" is 2.


class Solution1(object):
    def numDecodings(self, s):
        """
        :type s: str
        :rtype: int
        """
        if not s:
            return 0
        n = len(s)
        memo = [0 for i in range(n + 1)]
        memo[0] = 1  # for empty string, just an initialization
        if s[0] == '0':
            memo[1] = 0  # we dont know how to interpret '0' in string
        else:
            memo[1] = 1  # only one way to interpret one character
        i = 1
        while i < n:
            first = int(s[i])
            second = int(s[i - 1] + s[i])
            if 1 <= first <= 9:
                memo[i + 1] += memo[i]
            if 10 <= second <= 26:
                # if curr char and prev char form a valid number
                # for e.g. s[0..1] = '12', s[2] = '3', if '23' is valid then we
                # an either combine '2' & '3' i.e. number of ways '1' can be
                # represented OR 2 & 3 separated i.e. number of ways '12' can be
                # represented
                memo[i + 1] += memo[i - 1]
            i += 1
        return memo[n]
