#!/usr/bin/env python3

# Given a linked list, the task is to complete the function maxPalindrome which
# returns an integer denoting  the length of the longest palindrome list that
# exist in the given linked list.
#
# Examples:
#
# Input  : List = 2->3->7->3->2->12->24
# Output : 5
# The longest palindrome list is 2->3->7->3->2
#
# Input  : List = 12->4->4->3->14
# Output : 2
# The longest palindrome list is 4->4


class SllNode:
    def __init__(self, val=None):
        self.key = val
        self.nextp = None

    def __str__(self):
        return str(self.key)


def createlist(alist):
    head = tail = None
    for val in alist:
        if tail:
            tail.nextp = SllNode(val)
            tail = tail.nextp
        else:
            head = tail = SllNode(val)
        tail.nextp = SllNode()  # just put dummy node at end so that comparisons are easy.
    return head


def printlist(head):
    curr = head
    while curr.key:
        print(curr, end='->')
        curr = curr.nextp
    print(None)



def findMaxPalindromeLen(head):
    # reverse the linked list at each node and traverse from that node in opp
    # directions.
    curr = head
    prev = SllNode()
    maxlen = 0
    while curr.key:
        nxt = curr.nextp
        curr.nextp = prev
        # now from curr traverse back and forward and check if palindrome exists. if exists then get the length.
        # first check for odd length palindrome from curr
        i = prev
        j = nxt
        currmax = 1
        while i.key == j.key:
            # print('i == j for odd')
            currmax += 2
            i = i.nextp if i else None
            j = j.nextp if j else None
        maxlen = max(maxlen, currmax)

        # now check for even length palindrome starting at curr and nxt
        i = curr
        j = nxt
        currmax = 0
        while i.key == j.key:
            currmax += 2
            i = i.nextp if i else None
            j = j.nextp if j else None

        maxlen = max(maxlen, currmax)

        prev = curr
        curr = nxt

    return maxlen


def main():
    alist1 = [2, 3, 7, 3, 2, 12, 24]
    alist2 = [12, 4, 4, 3, 14]
    alist3 = [1,2,3,6,3,9,6,6,9,3]

    head = createlist(alist3)
    printlist(head)
    print(findMaxPalindromeLen(head))


if __name__ == '__main__':
    main()
