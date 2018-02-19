# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def reverseList(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        prev = None
        curr = head
        if head is None:
            return head
        nxt = curr.next
        while curr is not None:
            curr.next = prev
            prev = curr
            curr = nxt
            if nxt is not None:
                nxt = nxt.next
        return prev
