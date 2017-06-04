#!/usr/bin/env python3

# Calculate the sum of two integers a and b, but you are not allowed to use the
# operator + and -.
#
# Example:
# Given a = 1 and b = 2, return 3.

def sum2int(a, b):
    a = bin(a).lstrip('b')
    b = bin(b).lstrip('b')

    
