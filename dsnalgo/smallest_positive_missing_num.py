#!/usr/bin/env python3

# You are given an unsorted array with both positive and negative
# elements. You have to find the smallest positive number missing from the
# array in O(n) time using constant extra space.


def findSmallestPositiveMissingNum(arr):
    # this does take O(n) auxillary space
    adict = {}
    for i in arr:
        if i > 0:
            adict[i] = True

    smallest = 1
    while True:
        if adict.get(smallest, None):
            # smallest positive number exits, search for next smallest
            smallest = smallest + 1
        else:
            # smallest number is missing, return it
            return smallest


def segregate(arr):
    i = 0
    for j, num in enumerate(arr):
        # this loop just segregates -ve and +ve numbers
        # whenever we encounter -ve number, we swap it with a[i]
        # after this loop, all -ve num will be at start
        if num <= 0:
            # if num is -ve, swap with a[i] and increment i
            arr[i], arr[j] = arr[j], arr[i]
            i = i + 1
    return i  # return position of first positive element


def findSmallestPositiveNum_UsingIndex(arr):
    # whichever +ve number a[i], we go to a[a[i]] and mark it -ve. then
    # iterate over the array and whichever index has first positive number
    # that index is the smallest positive number
    for i, num in enumerate(arr):
        if abs(num) - 1 < len(arr) and arr[abs(num) - 1] > 0:
            # we are subtracting 1 because positive num start from 1 but index
            # start from 0. So to mark presence of 1 we update index 0 etc.
            arr[abs(num) - 1] = -arr[abs(num) - 1]

    for i, num in enumerate(arr):
        if num > 0:
            # means this index is missing number
            return i + 1  # index start from 0, num is i+1

    # if no +ver number found then len(arr)+1 is the only number missing
    return len(arr) + 1


def main():
    arr1 = [1, 2, 3, 4, 5]
    arr2 = [0, -10, 1, 3, -20]
    # print(findSmallestPositiveMissingNum(arr1))
    # print(findSmallestPositiveMissingNum(arr2))
    pos_index = segregate(arr1)
    print(findSmallestPositiveNum_UsingIndex(arr1[pos_index:]))


if __name__ == '__main__':
    main()
