# Given a linked list, rotate the list to the right by k places, where k is non-negative.
# Example 1:

# Input: 1->2->3->4->5->NULL, k = 2
# Output: 4->5->1->2->3->NULL

# Explanation:
# rotate 1 steps to the right: 5->1->2->3->4->NULL
# rotate 2 steps to the right: 4->5->1->2->3->NULL

# Example 2:

# Input: 0->1->2->NULL, k = 4
# Output: 2->0->1->NULL

# Explanation:
# rotate 1 steps to the right: 2->0->1->NULL
# rotate 2 steps to the right: 1->2->0->NULL
# rotate 3 steps to the right: 0->1->2->NULL
# rotate 4 steps to the right: 2->0->1->NULL

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def rotateRight(self, head: ListNode, k: int) -> ListNode:
        return self.rotateRight_sol2(head, k)
    
    def rotateRight_sol1(self, head: ListNode, k: int) -> ListNode:
        # brute force solution
        # time limit exceeded
        count = 0
        while count < k:
            prevToLast = self.findPrevToLast(head)
            if prevToLast is None:
                # there is only one element
                return head
            last = prevToLast.next
            prevToLast.next = last.next
            last.next = head
            head = last
            count = count+1
        return head
    
    
    def findPrevToLast(self, head: ListNode) -> ListNode:
        curr = head
        if curr is None:
            return None
        while curr.next is not None and curr.next.next is not None:
            curr = curr.next
        # curr could be head if head is the only node. then curr.next is None
        # but if more then 1 node then curr is always second last node
        if curr.next is None:
            return None
        return curr
    
    def rotateRight_sol2(self, head: ListNode, k: int) -> ListNode:
        # k could be >= length of list. In that case effective rotation is only
        # k modulo (length of list). k==len(list) means after rotation its the same
        # list as original
        if head is None or head.next is None:
            return head
        length = 1
        last = head
        # find length of list
        while last.next is not None:
            last = last.next
            length += 1
        
        # re-calc k so that k < length
        k = k % length
        if k == 0:
            # same list as original
            return head
        
        # now right shifting k nodes also means cutting list at
        # length - k - 1 node from from and adding remaining nodes at
        # head. So get to the cutting point
        cutting_point = length - k - 1
        curr = head
        while cutting_point > 0:
            curr = curr.next
            cutting_point = cutting_point - 1
        
        # now list should end at curr node and remaining curr+1 to last node
        # should add to the head
        last.next = head
        head = curr.next
        curr.next = None
        
        return head

