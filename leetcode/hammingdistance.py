#!/usr/bin/env python3

#The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
#Given two integers x and y, calculate the Hamming distance.

import sys

def hammingdist1(arg1, arg2):
    try:
        arg1 = int(arg1)
        arg2 = int(arg2)
    except ValueError as ve:
        print('cannot convert to integer:', ve)
        sys.exit(-1)
    else:
        count = 0
        bin_num1 = bin(arg1)
        bin_num2 = bin(arg2)
        if len(bin_num1) != len(bin_num2):
            print('len of binary string is not equal')
            sys.exit(-1)
        for el1, el2 in zip(bin_num1, bin_num2):
            if el1 != el2:
                count += 1
        return count

def hammingdist2(arg1, arg2):
    try:
        arg1 = int(arg1)
        arg2 = int(arg2)
    except ValueError as ve:
        print('cannot convert to integer:', ve)
        sys.exit(-1)
    else:
        dist = 0
        val = arg1 ^ arg2 # doing XOR
        while val:
            val = val & (val-1)  #if we AND any number with number-1 its lower
                                #order 1 bit is unset
            dist += 1
        return dist

def hammingdist3(arg1, arg2):
    return bin(arg1 ^ arg2).count('1')

def main(argv):
    print('dist1 = ', hammingdist1('9','8'))
    print('dist2 = ', hammingdist2('9','8'))
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
