#!/usr/bin/env python3

# Given a non-negative integer num, repeatedly add all its digits until the
# result has only one digit.
#
# For example:
# Given num = 38, the process is like: 3 + 8 = 11, 1 + 1 = 2. Since 2 has only
# one digit, return it.

import sys

def addDigits(num):
    x = 0
    while num > 0:
        x = x + (num%10)
        num = num//10
    if x > 9:
        return addDigits(x)
    return x

def addDigits_2(num):
    while num >= 10:
        temp = 0
        while num > 0:
            temp = temp + (num % 10)
            num = num//10
        num = temp
    return num

def main(arg):
    print(addDigits_2(int(arg)))
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1]))
