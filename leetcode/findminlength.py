#!/usr/bin/env python3

import pdb

def minlength_brute(a, i, j, m, n):
    if m < i or n < j:
        return float('inf')
    if i == m and j == n:
        return a[i][j]
    return min(minlength(a, i, j, m-1, n), minlength(a, i,j, m, n-1)) + a[m][n]


def minlength_memoized(a, memo, i, j, m, n):
    #pdb.set_trace()
    if m < i or n < j:
        return float('inf')
    if memo[m][n] != float('inf'):
        return memo[m][n]
    if i == m and j == n:
        memo[m][n] = a[m][n]
        return memo[m][n]
    memo[m][n] = min(minlength_memoized(a, memo, i, j, m-1, n),\
                        minlength_memoized(a, memo, i, j, m, n-1)) + a[m][n]

    return memo[m][n]

def minlength_memoized_withsol(a, memo, sol, i, j, m, n):
    if m < i or n < j:
        return float('inf')
    if memo[m][n] != float('inf'):
        return memo[m][n]
    if i == m and j == n:
        memo[m][n] = a[m][n]
        sol[m][n] = m, n
        return memo[m][n]
    len_top = minlength_memoized_withsol(a, memo, sol, i, j, m-1, n) + a[m][n]
    len_left = minlength_memoized_withsol(a, memo, sol, i, j, m, n-1) + a[m][n]
    if len_top <= len_left:
        memo[m][n] = len_top
        sol[m][n] = (m-1, n)
    else:
        memo[m][n] = len_left
        sol[m][n] = (m, n-1)
    return memo[m][n]

def main():
    a = [
        [1, 3, 2],
        [2, 4, 3],
        [2, 1, 3]
        ]
    m, n = len(a)-1, len(a[0])-1
    memo = [[float('inf') for col in a[0]] for row in a]
    sol = [[None for col in a[0]] for row in a]
    print(minlength_memoized_withsol(a, memo, sol, 0, 0, m, n))
    while m != 0 or n != 0:
        print(sol[m][n])
        m, n = sol[m][n]

if __name__ == '__main__':
    main()
