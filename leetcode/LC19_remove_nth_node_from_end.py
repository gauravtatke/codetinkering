# Given a linked list, remove the n-th node from the end of list and return its head.

# Example:
# Given linked list: 1->2->3->4->5, and n = 2.
# After removing the second node from the end, the linked list becomes 1->2->3->5.

# Note:
# Given n will always be valid.

# Follow up:
# Could you do this in one pass?



# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
        return self.removeNthFromEnd_sol2(head, n)
    
    def removeNthFromEnd_sol1(self, head: ListNode, n: int) -> ListNode:
        nth_node = head
        curr = head
        prev = None
        count = 1
        while count < n:
            curr = curr.next
            count = count+1
        
        # after this, nth_node and curr are n nodes apart
        # if curr reaches to last node then nth_node is the one
        # to be removed. we also need a pointer which runs behind
        # nth_node so that it can be removed
        while curr.next is not None:
            curr = curr.next
            prev = nth_node
            nth_node = nth_node.next
        
        # here nth_node points to exact node that needs removal
        if nth_node == head:
            # could happen if n is length of list which means
            # n from last is head node
            head = head.next
        else:
            prev.next = nth_node.next
            
        return head
    
    def removeNthFromEnd_sol2(self, head: ListNode, n: int) -> ListNode:
        # basically same idea but using a dummy node in front of head
        # also instead of reaching at nth node to delete, we reach to 
        # node whose next is nth node to delete. So instead of 3 pointers
        # only use 2
        dummy = ListNode(None)
        dummy.next = head
        curr = prev = dummy
        
        # now maintain exact n node distance between prev and curr
        while n >= 0:
            curr = curr.next
            n = n-1
        
        # now just iterate till curr is None
        while curr is not None:
            curr = curr.next
            prev = prev.next
        
        # now prev.next is the node to be deleted
        prev.next = prev.next.next
        return dummy.next
        
    def removeNthFromEnd_sol3(self, head: ListNode, n: int) -> ListNode:
        # using dictionary to store nodes and node index
        curr = head
        my_list = []
        while curr is not None:
            my_list.append(curr)
            curr = curr.next
            
        # zero based index. each index corresponds to node at that location
        # in 0-based index, n from last is L-n from start. L is length
        length = len(my_list)
        if (length - n) > 0:
            my_list[length-n-1].next = my_list[length-n-1].next.next
        else:
            # head is to be removed
            head = head.next
        return head
        
            