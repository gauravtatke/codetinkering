#!/usr/bin/env python3

from rotate import printmatrix

def makezero(arr):
    zeropos = []
    for nrow, row  in enumerate(arr):
        for ncol, col in enumerate(row):
            if col == 0:
                zeropos.append((nrow,ncol))
    for nrow,ncol in zeropos:
        for i in range(len(arr)):
            arr[i][ncol] = 0
        for j in range(len(arr[nrow])):
            arr[nrow][j] = 0
    return arr

if __name__ == '__main__':
    array = [
            [1, 2, 4, 0],
            [3, 0, 5, 6],
            [7, 0, 0, 9],
            [8, 4, 3, 1]
            ]
    print('before zero')
    printmatrix(array)
    print('after zero')
    printmatrix(makezero(array))




