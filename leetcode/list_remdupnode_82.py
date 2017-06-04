#!/usr/bin/env python3

import sys
class lnode:
    def __init__(self, val=None):
        self.key = val
        self.nxt = None

    def __str__(self):
        return str(self.key)

class mylist:
    def __init__(self, *args):
        self.head = lnode()  #head is dummy node containing None
        self.it = self.head  #for iteration
        if args:
            self.insert(*args)

    def insert(self, *args):
        for val in args:
            temp = lnode(val)
            temp.nxt = self.head.nxt
            self.head.nxt = temp

    def __iter__(self):
        return self

    def __next__(self):
        #self.it = self.head
        if self.it is not None:
            temp = self.it
            self.it = self.it.nxt
            return temp
        else:
            self.it = self.head  #reset val for subsequent iteration
            raise StopIteration()

    def __str__(self):
        strlist =  ' > '.join(str(node) for node in self)
        return strlist.strip(' >')


def removeDuplicateKey(mlist):
    prev = None
    curr = mlist.head
    temp = curr
    while curr is not None:
        curr = curr.nxt
        if curr is not None and curr.key == temp.key:
            while curr is not None and curr.key == temp.key:
                curr = curr.nxt
            if curr is None:
                #means all remaining are duplicates
                prev.nxt = None
                return mlist
            prev.nxt = curr
            temp = curr
        else:
            temp = temp.nxt
            if prev is not None:
                prev = prev.nxt
            else:
                prev = mlist.head
    return mlist


def main(arg):
    mlist = mylist(*arg)
    print(mlist)
    print(removeDuplicateKey(mlist))
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
