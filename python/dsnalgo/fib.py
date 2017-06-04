#!/usr/bin/env python3

def fib(n):
    #let r[0..n] be new array
    r = [-1 for i in range(n+1)]
    return memoized_fib(n, r)

def memoized_fib(n, r):
    if r[n] >= 0:
        return r[n]
    if n == 1:
        return 0
    elif n == 2:
        return 1
    else:
        r[n] = memoized_fib(n-1, r) + memoized_fib(n-2, r)
        return r[n]

if __name__ == '__main__':
    for i in range(1, 10):
        print(fib(i), end=', ')
    print()
