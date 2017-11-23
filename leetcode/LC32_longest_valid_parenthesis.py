# Given a string containing just the characters '(' and ')', find the length of
# the longest valid (well-formed) parentheses substring.

# For "(()", the longest valid parentheses substring is "()", which has
# length = 2.

# Another example is ")()())", where the longest valid parentheses
# substring is "()()", which has length = 4.


class Solution1(object):
    # using stack
    # push each index in stack. If we find a pair, then discard it.
    # check the difference between current index and index of last unmatched
    # parens at stack top. If difference is greater then previous max, then
    # update max.
    def longestValidParentheses(self, s):
        """
        :type s: str
        :rtype: int
        """
        maxlen = 0
        stack = [-1]  # pushing -1 as initial index to cover edge case
        for i, ch in enumerate(s):
            if ch == ')' and len(stack) > 1 and s[stack[-1]] == '(':
                # found a pair, discard it and see max length.
                stack.pop()
                maxlen = max(maxlen, i - stack[-1])
            else:
                stack.append(i)
        return maxlen


class Solution2(object):
    # this is DP solution. go through the string and store the length of longest valid parens at that point.
    # count all '('. If we encounter ')' and count of '(' is non-zero then we have atleast one pair.
    # check for longest[i-1]. if it is non-zero than we have something like "(())".
    # we might have something before "(())". check for i - longest[i]. if
    # i-longest[i] is non-zero than we have something like "()(())"
    def longestValidParentheses(self, s):
        """
        :type s: str
        :rtype: int
        """
        open_parens = 0
        result = 0
        longest = [0 for i in s]
        for i, parens in enumerate(s):
            if parens == '(':
                open_parens += 1
            else:
                if open_parens:
                    longest[i] = longest[i - 1] + 2
                    if i - longest[i] > 0:
                        longest[i] += longest[i - longest[i]]
                    open_parens -= 1
                result = max(result, longest[i])
        return result
