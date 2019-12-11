#Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
#
#Example:
#
#Input: 1->2->4, 1->3->4
#Output: 1->1->2->3->4->4

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        head = current = None
        if l1 is None and l2 is None:
            return None

        while l1 is not None and l2 is not None:
            if head == None:
                if l1.val <= l2.val:
                    head = current = l1
                    l1 = l1.next
                else:
                    head = current = l2
                    l2 = l2.next
            else:
                if l1.val <= l2.val:
                    current.next = l1
                    current = current.next
                    l1 = l1.next
                else:
                    current.next = l2
                    current = current.next
                    l2 = l2.next

        if l1 is None and l2 is not None:
            if head == None:
                head = l2
            else:
                current.next = l2
        elif l2 is None and l1 is not None:
            if head == None:
                head = l1
            else:
                current.next = l1
        return head
