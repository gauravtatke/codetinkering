#!/usr/bin/env python3

def rotate(matrix):
    #we first rotate the outer square then mover to inner ones
    nrow = len(matrix)
    for eachcol in matrix:
        if nrow != len(eachcol):
            #order of matrix is not proper
            return None
    for i in range(nrow//2):
        for j in range(i, nrow-i-1):
            row = i
            col = j
            k = 0
            currval = matrix[row][col]
            while k < 4:
                #col becomes next row, next col = nrow-row-1 for rotation
                rown = col
                coln = nrow - row - 1
                nxval = matrix[rown][coln]
                matrix[rown][coln] = currval
                currval = nxval
                row = rown
                col = coln
                k += 1
    return matrix

def printmatrix(matrix):
    if matrix is not None:
        for row in matrix:
            for col in row:
                print('{:^4d}'.format(col), end=' ')
            print()
    else:
        print(None)


if __name__ == '__main__':
    matrix = [
                [1, 2, 3, 4, 5, 6, 7],
                [8, 9, 10, 11, 12, 13, 14],
                [15, 16, 17, 18, 19, 20, 21],
                [22, 23, 24, 25, 26, 27, 28],
                [29, 30, 31, 32, 33, 34, 35],
                [36, 37, 38, 39, 40, 41, 42],
                [43, 44, 45, 46, 47, 48, 49]
            ]
    print("matrix before rotation")
    printmatrix(matrix)
    print("after rotation")
    printmatrix(rotate(matrix))
