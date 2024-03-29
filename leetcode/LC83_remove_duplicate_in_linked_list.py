#Given a sorted linked list, delete all duplicates such that each element appear only once.
#
#Example 1:
#
#Input: 1->1->2
#Output: 1->2
#Example 2:
#
#Input: 1->1->2->3->3
#Output: 1->2->3
#


# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        if head is None:
            return None
        if head.next is None:
            return head
        prev = head
        curr = head.next
        while curr is not None:
            while curr is not None and curr.val == prev.val:
                curr = curr.next
            prev.next = curr
            prev = prev.next
            if curr is not None:
                curr = curr.next
        return head
        
