#!/usr/bin/env python3

# Given an array of integers, every element appears twice except for one. Find
# that single one.
#
# Note:
# Your algorithm should have a linear runtime complexity. Could you implement it
# without using extra memory?

def singleNumber(nums):
    nu = 0
    for n in nums:
        nu = nu ^ n
    return nu

def singleNumber2(nums):
    adict = {}
    for nu in nums:
        if nu in adict:
            del adict[nu]
        else:
            adict[nu] = None
    return adict.popitem()[0]

def main():
    print(singleNumber([4,5,6,7,3,2,1,4]))

if __name__ == '__main__':
    main()
