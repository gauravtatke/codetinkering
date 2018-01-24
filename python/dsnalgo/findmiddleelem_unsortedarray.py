#!/usr/bin/env python3

# Given an unsorted array of size N. Find the first element in array such
# that all of its left elements are smaller and all right elements to it
# are greater than it.

# Note: Left and right side elements can be equal to required element. And
# extreme elements cannot be required element.


def findmiddleelem(arr):
    req_elem = -1
    curr_max = -1
    for i, num in enumerate(arr):
        if i == 0 or i == len(arr) - 1:
            # means first elem & last elem, cannot be req_elem
            # just update the curr_max
            curr_max = num
        else:
            if num >= curr_max:
                curr_max = num
                if req_elem == -1:
                    # means no req_elem is found yet so this becomes the req elem
                    # if req_elem not -1 then no need to do anything. just
                    # iterate next elem
                    req_elem = num
            else:
                # num is less than curr_max
                # check if is is less than req_elem. if so then update req_elem to -1
                # bcoz smaller elem can't appear on right of req_elem
                if num < req_elem:
                    req_elem = -1
    return req_elem


def main():
    arr1 = [2, 4, 1, 5, 6, 7]
    arr2 = [11, 9, 12]
    arr3 = [4, 3, 2, 7, 9, 8, 10]
    print(findmiddleelem(arr1))
    print(findmiddleelem(arr2))
    print(findmiddleelem(arr3))


if __name__ == '__main__':
    main()
