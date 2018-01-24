#  Given two words word1 and word2, find the minimum number of steps required to convert word1 to word2. (each operation is counted as 1 step.)
#
# You have the following 3 operations permitted on a word:
#
# a) Insert a character
# b) Delete a character
# c) Replace a character


class Solution1(object):
    # this is my solution - DOES NOT WORK
    def minDistance(self, word1, word2):
        """
        :type word1: str
        :type word2: str
        :rtype: int
        """
        # calculate max index and length of maximum contiguous match in word1
        # and word2. Then calculate number of prefix and suffix operations
        # needed to convert. Total operation = prefix_op + suffix_op
        len1 = len(word1)
        len2 = len(word2)

        grid = [[0 for i in range(len1 + 1)] for j in range(len2 + 1)]
        maxi = maxj = 0
        max_cnt = 0
        for i, char2 in enumerate(word2):
            for j, char1 in enumerate(word1):
                if char1 == char2:
                    # match occurs. see if total match is greater than max_cnt
                    if grid[i][j] + 1 > max_cnt:  # grid[i][j] is prev row,col
                        max_cnt = grid[i][j] + 1
                        maxi, maxj = i, j
                        # grid has extra row and column
                        grid[i + 1][j + 1] = max_cnt
        if max_cnt:
            # if there is a match
            prefix1_char = maxj - max_cnt + 1  # num of characters before the start of match
            prefix2_char = maxi - max_cnt + 1
            prefix_op = max(prefix1_char, prefix2_char)

            suffix1_char = len1 - maxj - 1
            suffix2_char = len2 - maxi - 1
            suffix_op = max(suffix2_char, suffix1_char)

            return prefix_op + suffix_op
        # if no match, then largest length is ans
        return max(len1, len2)


class Solution2(object):
    # DP solution
    def minDistance(self, word1, word2):
        """
        :type word1: str
        :type word2: str
        :rtype: int
        """
        # let dp[i][j] is min step required to convert word1[0..i-1] to word2[0..j-1].
        # i, j start from 1 in this notation
        # boundary condition -
        # case 1: convert empty string to a string -> j insertions
        # case 2: convert string to an empty string -> i deletions
        # non-edge case - convert word1[0..i-1] to word2[0..j-1]. suppose we
        # know how to convert word1[0..i-2] to word2[0..j-2] i.e. dp[i-1][j-1]
        # then following cases arise
        # 1. if word1[i-1] == word2[j-1], then dp[i][j] = dp[i-1][j-1].
        # 2. if word1[i-1] != word2[j-1], then we can either -
        # 2a. replace word1[i-1] with word2[j-1], dp[i][j] = dp[i-1][j-1] + 1, or
        # 2b. delete word1[i-1] and just convert word1[0..i-2] to word2[0..j-1], dp[i][j] = dp[i-1][j] + 1, or
        # 2c. insert word2[j-1] to word1[0..i-1] such that word1[0..i-1] +
        # word2[j-1] = word2[0..j-1], dp[i][j] = dp[i][j-1] + 1
        m, n = len(word1), len(word2)
        dp = [[0 for j in range(n + 1)] for i in range(m + 1)]
        for i in range(m + 1):
            dp[i][0] = i  # boundary case 2
        for j in range(n + 1):
            dp[0][j] = j  # boundary case 1
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if word1[i - 1] == word2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1]
                else:
                    dp[i][j] = min(dp[i - 1][j - 1], dp[i - 1]
                                   [j], dp[i][j - 1]) + 1
        return dp[m][n]
