#!/usr/bin/env python3

# Given an array of distinct integers, find if there are two pairs (a, b) and (c, d) such that a+b = c+d, and elements of array are distinct. Print "1" if pair exists else print "0".

def sumEqSum(arr):
    sumdict = {}
    for i, num1 in enumerate(arr):
        for j, num2 in enumerate(arr[i+1:], i+1):
            val = sumdict.get(num1+num2, None)
            if val:
                print(val, (num1, num2))
                return 1
            sumdict[num1+num2] = (num1, num2)
    return 0


def main():
    arr1 = [3, 4, 7, 1, 2, 9, 8]
    arr2 = [65, 30, 7, 90, 1, 9, 8]
    print(sumEqSum(arr2))

if __name__ == '__main__':
    main()

