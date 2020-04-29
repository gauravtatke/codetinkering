#!/usr/bin/env python3

# Given a singly linked list of characters, write a function that returns true
# if the given list is palindrome, else false.


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


def comparelist(head1, head2):
    curr1 = head1
    curr2 = head2
    while curr1 and curr2:
        if curr1.key != curr2.key:
            return False
        curr1 = curr1.np
        curr2 = curr2.np
    if curr1 or curr2:
        # atleast one of them is not None.
        # return False
        return False
    return True


def reverselist(head):
    curr = head
    prev = None
    nxt = head.np
    while curr:
        curr.np = prev
        prev = curr
        curr = nxt
        nxt = nxt.np if nxt else None
    return prev


def isPalindrome(head):
    # find the mid. reverse the list from mid to end and compare the 2 list
    slow = fast = head
    prev_slow = None
    mid = None

    if head and head.np:
        while fast and fast.np:
            fast = fast.np.np
            prev_slow = slow
            slow = slow.np

        if fast is not None:
            # means it is odd size list
            # save the mid so that we can make original list back after
            # reversing half list
            mid = slow
            slow = slow.np
        # now we will reverse the list from slow to end
        slow = reverselist(slow)
        prev_slow.np = None
        if comparelist(head, slow):
            result = True
        else:
            result = False

        # Now make the original list back
        slow = reverselist(slow)
        if mid:
            # if there was a mid element i.e. odd size list
            mid.np = slow
            prev_slow = mid
        else:
            prev_slow.np = slow

        return result


def isPalindrome_UsingStack(head):
    # this method uses stack. push all nodes into stack.
    # then traverse the list again and pop the items if it matches.
    # list is palindrome if the stack becomes empty after second traversal
    stack = []
    curr = head
    while curr:
        stack.append(curr)
        curr = curr.np

    curr = head
    while curr:
        if curr.key == stack[-1].key:
            stack.pop()
            curr = curr.np
        else:
            return False

    return not len(stack)


def main():
    alist1 = [1, 2]
    alist2 = [1, 2, 3, 3, 2, 1]
    head1 = createlist(alist1)
    head2 = createlist(alist2)
    print(isPalindrome_UsingStack(head1))


if __name__ == '__main__':
    main()
