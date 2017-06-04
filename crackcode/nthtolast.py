#!/usr/bin/env python3

from listnstack import sllist

def ntolast(sll, n):
    if len(sll) < n:
        return None

    curr = sll.head
    nobj = sll.head

    for i in range(n-1):
        #make distance between curr and nobj equal to n
        nobj = nobj.nxt

    while nobj.nxt is not None:
        nobj = nobj.nxt
        curr = curr.nxt
    
    #when loops is over, curr is nth to last, nobj is last
    return curr

if __name__ == '__main__':
    sll = sllist(1,3,4,2,6,7,4,3)
    print('{n}th to last is {ntolast}'.format(n=4, ntolast=ntolast(sll, 4)))


