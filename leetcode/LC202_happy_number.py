# Write an algorithm to determine if a number is "happy".
# A happy number is a number defined by the following process: Starting with any positive integer, 
# replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), 
# or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.
# Example: 19 is a happy number

#     12 + 92 = 82
#     82 + 22 = 68
#     62 + 82 = 100
#     12 + 02 + 02 = 1


class Solution1:
    def isHappy(self, n):
        """
        :type n: int
        :rtype: bool
        """
        slow = fast = n
        while True:
            slow = self.squareSum(slow)
            fast = self.squareSum(fast)
            fast = self.squareSum(fast)
            if slow == fast:
                break
        if slow == 1:
            return True
        return False

    def squareSum(self, num):
        digitsum = 0
        while num:
            digit = num % 10
            digitsum += digit * digit
            num = num // 10
        return digitsum


class Solution2:
    def isHappy(self, n):
        """
        :type n: int
        :rtype: bool
        """
        uniqueset = set()
        while True:
            uniqueset.add(n)
            n = self.squareSum(n)
            if n == 1:
                return True
            if n in uniqueset:
                return False

    def squareSum(self, num):
        digitsum = 0
        while num:
            digit = num % 10
            digitsum += digit * digit
            num = num // 10
        return digitsum