#!/usr/bin/env python3

# generate all permutation of arr

import sys

def permutation(arr, start, end):
    if start == end:
        print(arr)
    else:
        i = start
        while i <= end:
            arr[start], arr[i] = arr[i], arr[start]
            permutation(arr, start+1, end)
            arr[start], arr[i] = arr[i], arr[start]  # backtrack. undo the swap done in prev step
            i += 1


def heapPermutation(arr, size):
    if size == 1:
        print(arr)
        return

    for i in range(size):
        heapPermutation(arr, size-1)
        if size % 2:  # size is even
            # swap last element with ith element
            arr[i], arr[size-1] = arr[size-1], arr[i]
        else:
            # swap first and last element
            arr[0], arr[size-1] = arr[size-1], arr[0]


def main(argv):
    arr = list(argv[0])
    permutation(arr, 0, len(arr)-1)
    print('#########')
    heapPermutation(arr, len(arr))
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
