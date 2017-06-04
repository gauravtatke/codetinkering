#!/usr/bin/env python3

from listnstack import sllist,slnode
import pdb

def addlist(sllist1, sllist2):
    temp = None
    head = None
    s1 = sllist1.head
    s2 = sllist2.head
    carry = 0
    remain = 0
    while (s1 is not None) and (s2 is not None):
        #pdb.set_trace()
        remain = (s1.key + s2.key + carry)%10
        carry = (s1.key + s2.key + carry)//10
        if temp is None:
            temp = slnode(remain)
            head = temp
        else:
            temp.nxt = slnode(remain)
            temp = temp.nxt
        #sllist3.insert(remain)
        s1 = s1.nxt
        s2 = s2.nxt
    if s1 is None:
        #means s1 exhausted
        while s2 is not None:
            remain = (s2.key + carry)%10
            carry = (s2.key + carry)//10
            #sllist3.insert(remain)
            temp.nxt = slnode(remain)
            temp = temp.nxt
            s2 = s2.nxt
    elif s2 is None:
        while s1 is not None:
            remain = (s1.key + carry)%10
            carry = (s1.key + carry)//10
            #sllist3.insert(remain)
            temp.nxt = slnode(remain)
            temp = temp.nxt
            s1 = s1.nxt
    if carry > 0:
        #sllist3.insert(carry)
        temp.nxt = slnode(carry)
    
    #return sllist3
    return head

def addlistrec(node1, node2, carry):
    if node1 is None and node2 is None:
        return None

    result = slnode()
    value = carry
    if node1 is not None:
        value += node1.key
    if node2 is not None:
        value += node2.key
    result.key = value % 10

    more = addlistrec(None if node1 is None else node1.nxt,
                     None if node2 is None else node2.nxt,
                     1 if value >= 10 else 0)
    result.nxt = more
    return result



if __name__ == '__main__':
    l1 = sllist(5, 1, 3)
    l2 = sllist(2, 9, 5, 2, 1)
    head = addlist(l1, l2)
    head2 = addlistrec(l1.head, l2.head, 0)
    #for node in l3:
    #    print(node)
    while head is not None:
        print(head, end='>')
        head = head.nxt
    print(None)
    while head2 is not None:
        print(head2, end='>')
        head2 = head2.nxt
    print(None)
