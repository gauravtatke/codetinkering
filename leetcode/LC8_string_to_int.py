# Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

# The algorithm for myAtoi(string s) is as follows:

# Read in and ignore any leading whitespace.
# Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
# Read in next the characters until the next non-digit charcter or the end of the input is reached. The rest of the string is ignored.
# Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
# If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
# Return the integer as the final result.
# Note:

# Only the space character ' ' is considered a whitespace character.
# Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

class Solution:
    def myAtoi(self, s: str) -> int:
        return str_to_int(s)

MAX_INT = 2147483647
MIN_INT = -2147483648

def str_to_int(s: str) -> int:
    ascii_zero = ord('0')
    i = 0
    my_int = 0
    # ignore all whitespace
    while i < len(s) and s[i] == ' ':
        i = i + 1
    signed = True if i < len(s) and s[i] == '-' else False # initialize sign
    i = i + 1 if i < len(s) and s[i] in ('+', '-') else i # increment i if there is sign

    while i < len(s) and s[i].isdigit():
        digit = ord(s[i]) - ascii_zero
        # check for underflow
        if (my_int > MAX_INT // 10 or (my_int == MAX_INT // 10 and digit > 7)):
            # underflow
            return MIN_INT if signed else MAX_INT
        # no overflow or underflow
        my_int = my_int * 10 + digit
        i = i + 1
    if signed:
        my_int = my_int * -1
    return my_int
    
    
if __name__ == '__main__':
    sol = Solution()
    print(sol.myAtoi('-2147483648'))
