# You are climbing a stair case. It takes n steps to reach to the top.
# Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
# Note: Given n will be a positive integer.
#
# Example 1:
#
# Input: 2
# Output:  2
# Explanation:  There are two ways to climb to the top.
#
# 1. 1 step + 1 step
# 2. 2 steps
#
# Example 2:
#
# Input: 3
# Output:  3
# Explanation:  There are three ways to climb to the top.
#
# 1. 1 step + 1 step + 1 step
# 2. 1 step + 2 steps
# 3. 2 steps + 1 step


class Solution1(object):
    # iterative solution
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        distinct = [0 for i in range(n + 1)]
        distinct[0] = 1
        distinct[1] = 1
        for i in range(2, n + 1):
            distinct[i] = distinct[i - 1] + distinct[i - 2]
        return distinct[n]


class Solution2(object):
    # above solution also forms a fibonacci series of n+1 elements
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        if n == 1:
            return 1
        if n == 2:
            return 2

        first = 1
        second = 2
        for i in range(3, n + 1):
            temp = first + second
            first = second
            second = temp
        return second


class Solution3(object):
    # recursive solution
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        dist = [0 for i in range(n + 1)]
        return self.climbRecursive(dist, n)

    def climbRecursive(self, dist, n):
        if n == 1 or n == 2:
            return n
        elif dist[n]:
            return dist[n]
        else:
            dist[n] = self.climbRecursive(
                dist, n - 1) + self.climbRecursive(dist, n - 2)
            return dist[n]
