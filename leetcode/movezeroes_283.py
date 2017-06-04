#!/usr/bin/env python3

# Given an numsay nums, write a function to move all 0's to the end of it while
# maintaining the relative order of the non-zero elements.
#
# For example, given nums = [0, 1, 0, 3, 12], after calling your function, nums
# should be [1, 3, 12, 0, 0].
#
# Note:
# You must do this in-place without making a copy of the numsay.
# Minimize the total number of operations.

def moveZeroes(nums):
    i = -1
    j = -1
    for x, n in enumerate(nums):
        if n == 0:
            if i == -1 and j == -1:
                i = j = x
            k = j+1
            while k < len(nums) and nums[k] == 0:
                k = k+1
            if k >= len(nums):
                break
            nums[i], nums[k] = nums[k], nums[i]
            i += 1
            j = k


def moveZeroes_2(nums):
    i = j = -1
    for indx, num in enumerate(nums):
        if not num:
            if i == -1:
                i = indx
            #j = indx
        elif i != -1:
            nums[indx], nums[i] = nums[i], nums[indx]
            #j = indx
            i += 1
        else:
            continue

def moveZeroes_3(nums):
    insertpos = 0
    for num in nums:
        if num:
            nums[insertpos] = num
            insertpos += 1
    while insertpos < len(nums):
        nums[insertpos] = 0
        insertpos += 1



def main(arg):
    moveZeroes_3(arg)
    print(arg)
    return 0

if __name__ == '__main__':
    main([0,1,0,0,12,0,6,0])
