#!/usr/bin/env python3

'''
let -
X = <X1X2...Xm>
Y = <Y1Y2...Yn>
and Z = <Z1Z2...Zk> is any lcs of X and Y
1. If Xm == Yn, then Zk == Xm == Yn and Zk-1 is an lcs of Xm-1 and Yn-1
2. If Xm != Yn, then Zk != Xm implies that Z is an lcs of Xm-1 and Y
3. If Xm != Yn, then Zk != Yn implies that Z is an lcs of X and Yn-1
'''

def lcs_length(xseq, yseq):
    lcs = [[0 for col in range(len(yseq)+1)] for row in range(len(xseq)+1)]
    sol = [[0 for col in yseq] for row in xseq]
    for row, __ in enumerate(xseq):
        for col, __ in enumerate(yseq):
            if xseq[row] == yseq[col]:
                lcs[row+1][col+1] = lcs[row][col]+1
                #'l' points to left, 'u' points to up, 'd' points to diagnal
                sol[row][col] = 'd'
            elif lcs[row][col+1] >= lcs[row+1][col]:
                lcs[row+1][col+1] = lcs[row][col+1]
                sol[row][col] = 'u'
            else:
                lcs[row+1][col+1] = lcs[row+1][col]
                sol[row][col] = 'l'
    return lcs, sol

def print_lcs(sol, xseq, row, col):
    if row == -1 or col == -1:
        return
    if sol[row][col] == 'd':
        print_lcs(sol, xseq, row-1, col-1)
        print(xseq[row], end=' ')  
    elif sol[row][col] == 'u':
        print_lcs(sol, xseq, row-1, col)
    else:
        print_lcs(sol, xseq, row, col-1)

def print_lcs_nosol(lcs, xseq, yseq, row, col):
    if row == -1 or col == -1:
        return
    if xseq[row] == yseq[col]:
        print_lcs_nosol(lcs, xseq, yseq, row-1, col-1)
        print(xseq[row], end=' ')
    elif lcs[row][col+1] >= lcs[row+1][col]:
        print_lcs_nosol(lcs, xseq, yseq, row-1, col)
    else:
        print_lcs_nosol(lcs, xseq, yseq, row, col-1)

def rec_lcs(lcs, xseq, yseq, row, col):
    if row == -1 or col == -1:
        return 0
    elif xseq[row] == yseq[col]:
        lcs[row][col] = rec_lcs(lcs, xseq, yseq, row-1, col-1)+1
    else:
        lcs[row][col] = max(rec_lcs(lcs, xseq, yseq, row-1, col), \
                                rec_lcs(lcs, xseq, yseq, row, col-1))
    return lcs[row][col]

def memoized_lcs(xseq, yseq):
    lcs_mat = [[float('-inf') for col in yseq] \
                for row in xseq]
    return lcs_len_rec(xseq, yseq, len(xseq)-1, len(yseq)-1, lcs_mat)

def lcs_len_rec(xseq, yseq, row, col, lcs_mat):
    if lcs_mat[row][col] >= 0:
        return lcs_mat[row][col]
    if row == -1 or col == -1:
        return 0
    elif xseq[row] == yseq[col]:
        lcs_mat[row][col] = lcs_len_rec(xseq, yseq, row-1, col-1, lcs_mat) + 1
    else:
        lcs_mat[row][col] = max(lcs_len_rec(xseq, yseq, row-1, col, lcs_mat), \
                                lcs_len_rec(xseq, yseq, row, col-1, lcs_mat))
    return lcs_mat[row][col]

def lcs_len(xseq, yseq):
    '''
    this function uses 2*min(xseq.len, yseq.len) space to store the lcs
    matrix. Solution can't be build because of lack of information. It only
    returns max length
    '''
    (row, col) = (xseq, yseq) if len(xseq) >= len(yseq) else (yseq, xseq)
    lcs = [[0 for i in range(len(col)+1)] for j in range(2)]
    for rin, rel in enumerate(row):
        for cin, cel in enumerate(col):
            if rel == cel:
                lcs[1][cin+1] = lcs[0][cin] + 1
            elif lcs[0][cin+1] >= lcs[1][cin]:
                lcs[1][cin+1] = lcs[0][cin+1]
            else:
                lcs[1][cin+1] = lcs[1][cin]
        for k, __ in enumerate(col, start=1):
            lcs[0][k] = lcs[1][k]
    return lcs[0][len(col)]

def lcs_len2(xseq, yseq):
    '''
    This func() uses min(xseq.len, yseq.len) space & 2 extra variables to 
    calculate lcs. Solution can't be build. It only returns max length.
    '''
    (row, col)  = (xseq, yseq) if len(xseq) >= len(yseq) else (yseq, xseq)
    lcs = [0 for i in range(len(col)+1)]
    prev = 0
    for ri, rel in enumerate(row):
        for ci, cel in enumerate(col):
            if rel == cel:
                curr = lcs[ci]+1
            elif lcs[ci+1] >= prev:
                curr = lcs[ci+1]
            else:
                curr = prev
            lcs[ci] = prev
            prev = curr
        lcs[ci+1] = curr
        prev = 0
        #print(lcs)
    return lcs[ci+1]

if __name__ == '__main__':
    x1 = ['A', 'B', 'C', 'B', 'D', 'A', 'B']
    y1 = ['B', 'D', 'C', 'A', 'B', 'A']
    x2 = [1, 0, 0, 1, 0, 1, 0, 1]
    y2 = [0, 1, 0, 1, 1, 0, 1, 1, 0]
    x3 = [1, 0, 0, 1, 0]
    y3 = [1, 1, 0, 1, 0]
    lcs, sol = lcs_length(x3, y3)
    print(lcs)
    #print(sol)
    print_lcs(sol, x3, len(x3)-1, len(y3)-1)
    print()
    #lcs = [[0 for col in range(len(y3)] for row in range(len(x2))]
    lcslen = lcs_len2(x1, y1)
    print('lcs len = ', lcslen)
