# Given a linked list, swap every two adjacent nodes and return its head.
# You may not modify the values in the list's nodes, only nodes itself may be changed.

# Example:
# Given 1->2->3->4, you should return the list as 2->1->4->3.


# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def swapPairs(self, head: ListNode) -> ListNode:
        return self.swapPairs_sol1(head)
    
    def swapPairs_sol1(self, head: ListNode) -> ListNode:
        dummy = ListNode(None)
        dummy.next = head
        prev = dummy
        curr = head
        while curr is not None and curr.next is not None:
            temp = curr.next
            curr.next = temp.next
            temp.next = curr
            prev.next = temp
            
            prev = curr
            curr = curr.next
        return dummy.next