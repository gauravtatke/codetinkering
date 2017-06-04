#!/usr/bin/env python3
from listnstack import sllist, printlist

def findcorrupt(lst):
    curr = lst.head
    while curr is not None:
        run = lst.head
        while run.nxt != curr.nxt:
            run = run.nxt
        if run != curr:
            #means run and curr are different but point to same object
            return run.nxt
        curr = curr.nxt
    return None

if __name__ == '__main__':
    alist = sllist(2,4,6,1,8,5,9)
    alist.search(2).nxt = alist.search(1)
    #printlist(alist)
    cnode = findcorrupt(alist)
    print(cnode)

