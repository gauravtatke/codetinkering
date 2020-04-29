#!/usr/bin/env python3

def cut_rod_brute(p, ln):
    #this is a brute force approach to rod cutting problem
    #ln is length not index in price array
    if ln == 0:
        return 0
    q = float('-inf')
    for i in range(ln):
        q = max(q, p[i]+cut_rod_brute(p, ln-i-1))
    return q

def memoized_cut_rod(p, ln):
    #let r is new list
    r = [float('-inf') for i in range(ln+1)]
    #for i in range(n+1):
    #    r.append(float('-inf'))
    return memoized_cut_rod_aux(p, ln, r)

def memoized_cut_rod_aux(p, ln, r):
    if r[ln] >= 0:
        return r[ln]
    if ln == 0:
        q = 0
    else:
        q = float('-inf')
        for i in range(ln):
            q = max(q, p[i]+memoized_cut_rod_aux(p, ln-i-1, r))
    r[ln] = q
    return q

def bottom_up_cut_rod(p, ln):
    r = [0 for x in range(ln+1)]
    for j in range(1, ln+1):
        q = float('-inf')
        for i in range(j):
            q = max(p[i]+r[j-i-1], q)
        r[j] = q
    return r[ln]

def extended_bottom_rod_cut(p, ln):
    r = [0 for i in range(ln+1)]
    s = [0 for i in range(ln+1)]
    for j in range(1, ln+1):
        q = float('-inf')
        for i in range(j):
            if q < p[i] + r[j-i-1]:
                q = p[i] + r[j-i-1]
                s[j] = i+1  #i is index,i+1 is actual length
        r[j] = q
    return r, s

def print_cut_rod_sol(s, ln):
    #r, s = extended_bottom_rod_cut(p, ln)
    while ln > 0:
        print(s[ln], end=', ')
        ln = ln - s[ln]
    print('\n')

def memoized_cut_rod_sol(p, ln):
    r = [float('-inf') for i in range(ln+1)]
    s = [0 for i in range(ln+1)]
    return mem_cut_rod_sol_aux(p, ln, r, s), s

def mem_cut_rod_sol_aux(p, ln, r, s):
    if r[ln] >= 0:
        return r[ln]
    if ln == 0:
        q = 0
    else:
        q = float('-inf')
        for i in range(ln):
            t = mem_cut_rod_sol_aux(p, ln-i-1, r, s)
            if q < p[i] + t:
                q = p[i] + t
                s[ln] = i+1
    r[ln] = q
    return q

if __name__ == '__main__':
    price1 = [1, 5, 8, 9, 10, 17, 17, 20]
    price2 = [2, 5, 8, 13, 15, 17, 22, 20]
    for ln, __ in enumerate(price2, start=1):
        print('len = {0}, price = {1}'.format(ln, memoized_cut_rod(price2,
        ln)))
    print('#'*20)
    #for ln,pr in enumerate(price1, start=1):
    #    print('len = {0}, price = {1}'.format(ln, cut_rod_brute(price1, ln)))
    #print('price = {0}, sol = {1}'.format(*extended_bottom_rod_cut(price1,
    #    len(price1))))
    #max_price, sol = extended_bottom_rod_cut(price2, len(price2))
    #print(max_price)
    for ln, __ in enumerate(price2, start=1):
        max_price, sol = memoized_cut_rod_sol(price2, ln)
        print(max_price)
        print('len = {0}, sol = '.format(ln), end='')
        print_cut_rod_sol(sol, ln)
        
