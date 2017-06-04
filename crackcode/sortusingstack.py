#!/usr/bin/env python3

import random,sys
from listnstack import Stack

def sortstack(src):
    aux = Stack(len(src))
    while len(src):
        item = src.pop()
        while len(aux) and item < aux.peek():
            src.push(aux.pop())
        aux.push(item)
    return aux

def main(argv):
    slist = list(range(10))
    random.shuffle(slist)
    s = Stack(10)
    for i in slist:
        s.push(i)
    print(s)
    r = sortstack(s)
    print(r)
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

