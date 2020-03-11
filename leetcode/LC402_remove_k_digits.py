# Given a non-negative integer num represented as a string, remove k digits from the number so that the new number is the smallest possible.
# Note:

#     The length of num is less than 10002 and will be ≥ k.
#     The given num does not contain any leading zero.

# Example 1:
# Input: num = "1432219", k = 3
# Output: "1219"
# Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.

# Example 2:
# Input: num = "10200", k = 1
# Output: "200"
# Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.

# Example 3:
# Input: num = "10", k = 2
# Output: "0"
# Explanation: Remove all the digits from the number and it is left with nothing which is 0.

class Solution:
    def removeKdigits(self, num: str, k: int) -> str:
        return self.removeKdigits_sol1(num, k)
    
    def removeKdigits_sol1(self, num: str, k: int) -> str:
        # K digits can be consecutive or random but they have to be from left most side
        # because left most side is highest order
        # keep digits in stack. If new digit is smaller than top of stack then remove top 
        # and compare again. Remove top as long as k digits are not removed and new digit
        # is greater than top.
        if k >= len(num):
            # need to remove all the digits
            return "0"
        if k <= 0:
            return num
        my_stack = []
        for digit in num:
            if not my_stack:
                # stack is empty so digit should just be appended
                my_stack.append(digit)
            else:
                # as long as we have items to remove (k>0) and
                # stack is not empty then remove top of stack if
                # top > digit
                while k > 0 and my_stack and my_stack[-1] > digit:
                    my_stack.pop()
                    k = k-1
                # now digit is greater than top, so append it
                my_stack.append(digit)
        
        # if k > 0, means stack has all digits in increasing order
        # so remove last k digits
        while k > 0:
            my_stack.pop()
            k = k-1
        ret_string = "".join(my_stack).lstrip('0')
        return ret_string if ret_string else "0"