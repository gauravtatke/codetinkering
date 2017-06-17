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


def main(argv):
    arr = list(argv[0])
    permutation(arr, 0, len(arr)-1)
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

            
