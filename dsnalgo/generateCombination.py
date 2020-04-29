#/usr/bin/env python3

# genarate 'nc' combination from arr

import sys


def combination1(arr, data, start, end, index, nc):
    # method 1: fix elem and recurse
    if index == nc:
        print(data)
        return 0
    i = start
    while i <= end and end + 1 - i >= nc - index:
        data[index] = arr[i]
        combination1(arr, data, i + 1, end, index + 1, nc)
        i += 1


def combination2(arr, data, index, nc, iarr):
    if index == nc:
        print(data)
        return

    if iarr >= len(arr):
        # no more items to put in data
        return

    data[index] = arr[iarr]
    combination2(arr, data, index + 1, nc, iarr + 1)
    combination2(arr, data, index, nc, iarr + 1)


def main(argv):
    arr = list(argv[0])
    nc = int(argv[1])
    data = [float('inf') for i in range(nc)]
    combination2(arr, data, 0, nc, 0)
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
