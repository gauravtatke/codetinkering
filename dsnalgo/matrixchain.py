#!/usr/bin/env python3

'''
Matrix chain = <A1 A2 A3 .... Ai .... An>
Each Ai matrix has dimension = Pi-1 X Pi
So index chain = <P0 P1 P2...Pi-1 Pi....Pn>
cost[i, j] = array cost[1..n-1, 1..n-1], cost of multiplying Ai to Aj
sol[i, j] = array sol[1..n-1, 2...n] is array where value of 'K' is stored. K
is index where to parenthesize for a given i, j
P.length = n+1
'''

def recur_mat_chain_mult(p, i, j):
    #print(i, j)
    if i == j:
        return 0
    cost[i][j] = float('inf')
    for k in range(i, j):
        q = recur_mat_chain_mult(p, i, k) + \
                recur_mat_chain_mult(p, k+1, j) + p[i-1]*p[k]*p[j]
        if q < cost[i][j]:
            cost[i][j] = q
    return cost[i][j]

def matrix_chain_order(p):
    cost = [[0 for i in p] for j in p]
    sol = [[0 for i in p] for j in p]
    n = len(p)-1
    #for i in range(1, n+1):
    #    cost[i][i] = 0
    for length in range(2, n+1):
        #for len = 2, calculate cost[1, 2], cost[2, 3] ...
        #for len = 3, calculate cost[1, 3], cost[2, 4] ...
        for i in range(1, n-length+2):  #add xtra 1 cuz range excludes last num
            j = i+length-1
            cost[i][j] = float('inf')
            for k in range(i, j):
                min_val = cost[i][k] + cost[k+1][j] + p[i-1]*p[k]*p[j]
                if min_val < cost[i][j]:
                    cost[i][j] = min_val
                    sol[i][j] = k
    return cost, sol

def print_optimal_parens(sol, i, j):
    if i == j:
        print('A'+str(i), end='')
    else:
        print('(', end='')
        print_optimal_parens(sol, i, sol[i][j])
        print_optimal_parens(sol, sol[i][j]+1, j)
        print(')', end='')

def memoized_matrix_chain(p):
    n = len(p)-1
    cost = [[float('inf') for i in p] for j in p]
    return lookup_chain(p, cost, 1, n)

def lookup_chain(p, cost, i, j):
    if cost[i][j] < float('inf'):
        return cost[i][j]
    if i == j:
        cost[i][j] = 0
    else:
        for k in range(i, j):
            min_val = lookup_chain(p, cost, i, k) + \
                    lookup_chain(p, cost, k+1, j) + p[i-1]*p[k]*p[j]
            if min_val < cost[i][j]:
                cost[i][j] = min_val
    return cost[i][j]

if __name__ == '__main__':
    p1 = [10, 100, 5, 50]
    p2 = [30, 35, 15, 5, 10, 20, 25]
    cost = [[0 for i in p2] for j in p2]
    #print(cost)
    print(recur_mat_chain_mult(p2, 1, len(p2)-1))
    print(memoized_matrix_chain(p2))
    #print(cost)
    #cost, sol = matrix_chain_order(p1)
    #print('cost = ', cost[1][len(p1)-1])
    #print_optimal_parens(sol, 1, len(p1)-1)
    print()
