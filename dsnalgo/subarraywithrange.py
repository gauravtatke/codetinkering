#/usr/bin/env python3

import sys

def printRange(arr, sum):
    # this generates all ranges. complexity is O(n2)
    for i, num in enumerate(arr):
        j = i+1
        s = num
        while j < len(arr):
            s = s + arr[j]
            if s == sum:
                print(i, j)
            j += 1


def subArraySumRange(arr, reqsum):
    # this uses a dictionary to store index as values and sum upto index as keys
    # this may not generate all possible solution but it is fast as it traverses
    # array only once. Since it uses a dictionary, if currsum key already exist
    # then value (index) is overwritten so only few solutions can be generated
    sumdict = {}
    currsum = 0
    for i, num in enumerate(arr):
        currsum += num
        if currsum == reqsum:
            # sum is from 0 index to ith index
            print(0, i)
        else:
            j = sumdict.get(currsum-reqsum, None)
            if j:
                # means subarray exist whose sum is reqsum
                # 0 to i is currsum, 0 to j is currsum-reqsum, means
                # j+1 to i is reqsum
                print(j+1, i)
        sumdict[currsum] = i


def main():
    arr = [10, 2, -2, -20, 10]
    arr2 = [1, 4, 20, 3, 10, 5]
    printRange(arr, -10)
    subArraySumRange(arr, -10)


if __name__ == '__main__':
    main()
