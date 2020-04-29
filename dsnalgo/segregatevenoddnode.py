#!/usr/bin/env python3

# Given a Linked List of integers, write a function to modify the linked
# list such that all even numbers appear before all the odd numbers in the
# modified linked list. Also, keep the order of even and odd numbers same.


class SllNode:
    def __init__(self, key, nextp=None):
        self.key = key
        self.nextp = nextp


def insert(tail, node):
    if tail:
        tail.nextp = node
        node.nextp = None
        tail = node
    else:
        tail = node
        tail.nextp = None
    return tail


def segregateEvenOdd(head):
    # get the tail of the list
    tail = head
    while tail.nextp != None:
        tail = tail.nextp

    origtail = tail
    # insert all odd key nodes after at tail, move tail
    # run the loop till original tail
    curr = head
    prev = None  # prev will always point to last even node
    while curr != origtail:
        if curr.key % 2:
            # odd key node. move head to next if curr is head, else
            # remove the node and insert at tail
            if curr == head:
                head = head.nextp
                tail = insert(tail, curr)
                curr = head
            else:
                prev.nextp = curr.nextp
                tail = insert(tail, curr)
                curr = prev.nextp
        else:
            prev = curr
            curr = curr.nextp

    # check whether origtail is odd or even
    # if odd, then add it back at tail
    if curr.key % 2:
        if prev and curr.nextp:
            # if it has atleast one even number, and curr != tail
            prev.nextp = curr.nextp
            insert(tail, curr)
        elif prev or curr.nextp is None:
            # this is odd node and it is last means curr == tail
            # like - [2,4,6,7]
            return head
        else:
            # there is no even node and more than one odd node
            head = curr.nextp
            insert(tail, curr)

    return head


def segregateEvenOdd_New(head):
    end = head
    while end.nextp:
        end = end.nextp

    new_end = end

    # move all odd nodes before first even node
    curr = head
    while curr != end and curr.key % 2 != 0:
        temp = curr.nextp
        new_end = insert(new_end, curr)
        curr = temp

    # execute below only if atleast one even node is present
    if curr.key % 2 == 0:
        # head points to even node
        head = curr
        while curr != end:
            if curr.key % 2 != 0:
                prev.nextp = curr.nextp
                new_end = insert(new_end, curr)
                curr = prev.nextp
            else:
                prev = curr
                curr = curr.nextp
    else:
        # means curr reached last node which is odd
        # prev need to be set before moving further
        prev = curr

    # below is to execute if list has more than one odd node and original list
    # ends with a odd node
    if new_end != end and (end.key % 2 != 0):
        prev.nextp = end.nextp
        new_end = insert(new_end, curr)

    return head


def segregateEvenOdd_ByCreatinSepList(head):
    even_head = None
    even_tail = None
    odd_head = None
    odd_tail = None
    curr = head
    while curr:
        if curr.key % 2:
            # odd list
            if odd_head:
                odd_tail.nextp = curr
                odd_tail = curr
            else:
                odd_head = curr
                odd_tail = curr
        else:
            # even list
            if even_head:
                even_tail.nextp = curr
                even_tail = curr
            else:
                even_head = even_tail = curr

        curr = curr.nextp

    # put odd head at even tail
    if odd_tail:
        odd_tail.nextp = None
    if even_tail:
        even_tail.nextp = odd_head

    return even_head if even_head else odd_head


def printlist(head):
    curr = head
    while curr:
        print(curr.key, end='->')
        curr = curr.nextp
    print(None)


def createlist(alist):
    head = None
    tail = head
    for key in alist:
        if not head:
            head = insert(tail, SllNode(key))
            tail = head
        else:
            tail = insert(tail, SllNode(key))
    return head


def main():
    alist = [9, 7, 2, 3]
    head = createlist(alist)
    print("before")
    printlist(head)
    head = segregateEvenOdd_ByCreatinSepList(head)
    print("after")
    printlist(head)


if __name__ == '__main__':
    main()
