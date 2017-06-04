#!/usr/bin/env python3

import sys
class ListNode:
    def __init__(self, val):
        self.val = val
        self.next = None

    def __str__(self):
        return str(self.val)

class mylist:
    def __init__(self, *args):
        self.head = None
        if args:
            self.insert(*args)

    def insert(self, *args):
        for val in args:
            if self.head is None:
                self.head = ListNode(val)
            else:
                temp = ListNode(val)
                temp.next = self.head
                self.head = temp

    def __iter__(self):
        self.it = self.head
        return self

    def __next__(self):
        #self.it = self.head
        if self.it is not None:
            temp = self.it
            self.it = self.it.next
            return temp
        else:
            self.it = self.head  #reset val for subsequent iteration
            raise StopIteration()

    def __str__(self):
        strlist =  ' > '.join(str(node) for node in self)
        return strlist.strip(' >')


def printlist(head):
    curr = head
    while curr is not None:
        print(curr)
        curr = curr.next

def removeDuplicateVal(head):
    dummy = ListNode(None)
    dummy.next = head
    prev = None
    curr = dummy
    temp = curr
    while curr is not None:
        curr = curr.next
        if curr is not None and curr.val == temp.val:
            while curr is not None and curr.val == temp.val:
                curr = curr.next
            if curr is None:
                #means all remaining are duplicates
                if prev == dummy:
                    return None  #means no unique numbers
                prev.next = None
                return dummy.next
            prev.next = curr
            temp = curr
        else:
            temp = temp.next
            if prev is not None:
                prev = prev.next
            else:
                prev = dummy
    return dummy.next


def removeDuplicateVal_2(head):
    #using less loops, condition and variables. Basic algo remains same
    dummy = ListNode(None)
    dummy.next = head
    prev = dummy
    curr = head
    while curr is not None:
        while curr.next is not None and curr.val == curr.next.val:
            curr = curr.next
        if prev.next == curr:
            prev = prev.next
        else:
            prev.next = curr.next
        curr = curr.next
    return dummy.next

def removeDuplicateVal_3(head):
    dummy = ListNode(None)
    dummy.next = head
    prev = dummy
    while head is not None and head.next is not None:
        if head.val == head.next.val:
            while head.next is not None and head.val == head.next.val:
                head = head.next
            head = head.next
            prev.next = head
        else:
            prev = prev.next
            head = head.next
    return dummy.next


def main(arg):
    arg.reverse()
    mlist = mylist(*arg)
    print(mlist)
    #printlist(mlist.head)
    #for node in mlist:
    #    print(node)
    mlist.head = removeDuplicateVal_3(mlist.head)
    print(mlist)
    return 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
