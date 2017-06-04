#!/usr/bin/env python3

import listnstack

def remdup(sll):
    #removes any duplicate entries.
    #always removes the second (and so on) occurence of duplicate key
    for node in sll:
        currnode = sll.head
        while currnode is not None and currnode != node:
            if currnode.key == node.key:
                #node is duplicate
                sll.deletebyloc(node)
            currnode = currnode.nxt


if __name__ == '__main__':
    ls = listnstack.sllist(1,2,3,4,3,5,6,7,3,9)
    for nd in ls:
        print(nd)
    remdup(ls)
    print('#### after removing duplicate ####')
    for nd in ls:
        print(nd)


