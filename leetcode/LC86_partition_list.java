// Given a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
// You should preserve the original relative order of the nodes in each of the two partitions.
// Example:
// Input: head = 1->4->3->2->5->2, x = 3
// Output: 1->2->2->4->3->5


  // Definition for singly-linked list.
class ListNode {
      int val;
     ListNode next;
     ListNode(int x) { val = x; }
 }
 

class LC86_partition_list {
    public ListNode partition(ListNode head, int x) {
        return partitionSolution2(head, x);
    }
    
    public ListNode removeNode(ListNode head, ListNode node) {
        ListNode curr = head;
        if(head == node) {
            head = head.next;
            return head;
        }
        
        while(curr.next != node) {
            curr = curr.next;
        }
        // curr.next is node
        curr.next = node.next;
        return head;      
    }
    
    public ListNode partitionSolution1(ListNode head, int x) {
        // if 0 or 1 items then just return
        if(head == null || head.next == null) {
            return head;
        }
        
        ListNode dummy = new ListNode(Integer.MIN_VALUE);
        dummy.next = head;
        //ListNode xloc = head;
        
        
        // find the location where nodes can be added
        // location is before the first node which is greater than x
        // one of the use case is x may not be a node val
        ListNode locBefore = dummy;
        while(locBefore.next != null && locBefore.next.val < x) {
            locBefore = locBefore.next;
        }
        
        if(locBefore.next == null) {
            // reached end so list is in correct order
            return head;
        }
        
        // start from locBefore because before that all items are less than x
        ListNode curr = locBefore.next;
        while(curr != null) {
            if(curr.val < x) {
                ListNode temp = curr;
                curr = curr.next;
                locBefore = removeNode(locBefore, temp);
                temp.next = locBefore.next;
                locBefore.next = temp;
                locBefore = locBefore.next;
            } else {
                curr = curr.next;
            }
        } 
        return dummy.next;
    }
    
    public ListNode partitionSolution2(ListNode head, int x) {
        // another solution to create 2 separate list for smaller and greater elements and join them
        if (head == null || head.next == null) {
            return head;
        }
        
        ListNode dummy_smaller = new ListNode(Integer.MIN_VALUE);
        ListNode small_curr = dummy_smaller;
        ListNode dummy_greater = new ListNode(Integer.MIN_VALUE);
        ListNode greater_curr = dummy_greater;
        
        ListNode curr = head;
        while(curr != null) {
            if(curr.val < x) {
                // remove from here and add to smaller list
                small_curr.next = curr;
                small_curr = small_curr.next;
            } else {
                greater_curr.next = curr;
                greater_curr = greater_curr.next;
            }
            curr = curr.next;
        }
        greater_curr.next = null;
        small_curr.next = dummy_greater.next;
        return dummy_smaller.next;
    }
}