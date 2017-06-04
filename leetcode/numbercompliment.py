#!/usr/bin/env python3

#Given a positive integer, output its complement number. The complement strategy
#is to flip the bits of its binary representation.
#Note:
#The given integer is guaranteed to fit within the range of a 32-bit signed
#integer. You could assume no leading zero bit in the integer’s binary
#representation.

def findcompliment1(num):
    #function using string manupulation
    #num = bin(num).lstrip('0').lstrip('b')
    cnum = 0
    for i, nu in enumerate([1 if ch == '0' else 0 \
                            for ch in reversed(bin(num)[2:])]):
        cnum = cnum + nu*(2**i)
    return cnum

#if we use just ~num to flip the bits, it gives wrong result because BITWISE #NOT only flips the bits. Machine interprets it as negative number stored in #2's compliment and return something unexpected. For e.g. ~2 returns -3

#to just flip the bits as required by the problem, we have to create a mask of #1 bits of length = LEFTMOST 1 bit to RIGHTMOST bit. For e.g. for 5 (101) mask #is needed of length 3 (111), for 10 (1010), mask is needed for len 4 (1111).
#once mask is created, we can either do mask Bitwise XOR num (mask ^ num) or #Bitwise NOT num Bitwise AND mask (~num & mask)

#couple of ways to create mask of required length
#1. mask = 1
#mask = 1 << (len(bin(num)-2))

#2. mask = 1
#while mask <= num:
#    mask = mask << 1
#mask -= 1

#3. mask = 1
#while mask < num:
#    mask = mask << 1 | 1

def findcompliment2(num):
    mask = 1
    while mask < num:
        mask = mask << 1|1
    return num ^ mask

def main():
    print(findcompliment2(6))

if __name__ == '__main__':
    main()
