#!/usr/bin/env python3

# Given a single linked list and an integer x your task is to complete the
# function deleteAllOccurances  which deletes all occurences of a key x
# present in the linked list. The function takes two arguments: the head
# of the linked list and an integer x. The function should return the head
# of the modified linked list.


class SllNode:
    def __init__(self, val=None, np=None):
        self.key = val
        self.np = np

    def __str__(self):
        return str(self.key)


def createlist(alist):
    head = None
    tail = None
    for item in alist:
        if tail:
            tail.np = SllNode(item)
            tail = tail.np
        else:
            head = tail = SllNode(item)
    return head


def printlist(head):
    curr = head
    while curr:
        print(curr, end='->')
        curr = curr.np
    print(None)


def deleteAllOccurances(head, val):
    prev = None
    curr = head
    while curr:
        if curr.key == val:
            if prev:
                prev.np = curr.np
            else:
                head = curr.np
        else:
            prev = curr
        curr = curr.np
    return head


def main():
    alist = [1, 2, 3, 4, 3, 5, 6, 8, 3, 5, 3, 7, 1]
    head = createlist(alist)
    printlist(head)
    printlist(deleteAllOccurances(head, 1))


if __name__ == '__main__':
    main()
