#Given a linked list, determine if it has a cycle in it.
#
#To represent a cycle in the given linked list, we use an integer pos which represents the position (0-indexed) in the linked list where tail connects to. If pos is -1, then there is no cycle in the linked list.
#
#Example 1:
#
#Input: head = [3,2,0,-4], pos = 1
#Output: true
#Explanation: There is a cycle in the linked list, where tail connects to the second node.

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        return self.hasCycle_sol3(head)
    
    def hasCycle_sol1(self, head: ListNode) -> bool:
        # brute force approach
        # in case cycle is at head, we need to create dummy node 
        # which points head and start from there
        curr = head
        while curr is not None:
            # if list has no cycle then curr reaches end
            # start a runner from head always. If there is a point
            # where runner.next == curr.next and runner != curr then
            # there is cycle
            runner = head
            while runner.next != head and runner.next != curr.next and runner.next != runner:
                runner = runner.next
            if runner.next == head or runner.next == runner or runner != curr:
                # cycle at head or tail
                return True
            else:
                # no cycle, just move curr and continue loop
                curr = curr.next                    
        return False # when curr reaches end, no cycle found
    
    def hasCycle_sol2(self, head: ListNode) -> bool:
        # using hash set
        visited = set()
        while head is not None:
            if head in visited:
                return True
            else:
                visited.add(head)
                head = head.next
        return False
    
    def hasCycle(self, head: ListNode) -> bool:
        # using 2 pointers
        # fast and slow pointer. If fast becomes None
        # then there is no cycle otherwise fast and slow
        # will meet somewhere
        if head is None or head.next is None:
            return False
        slow = head
        fast = head.next
        while (slow != fast):
            if fast is None or fast.next is None:
                return False
            slow = slow.next
            fast = fast.next.next
        #if loop finishes then slow == fast
        return True
