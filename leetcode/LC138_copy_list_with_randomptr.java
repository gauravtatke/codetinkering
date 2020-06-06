// A linked list is given such that each node contains an additional random pointer which could point to any node in the list or null.

// Return a deep copy of the list.

// The Linked List is represented in the input/output as a list of n nodes. Each node is represented as a pair of [val, random_index] where:

//     val: an integer representing Node.val
//     random_index: the index of the node (range from 0 to n-1) where random pointer points to, or null if it does not point to any node.


// Definition for a Node.

import java.util.*;

class Node {
    int val;
    Node next;
    Node random;

    public Node(int val) {
        this.val = val;
        this.next = null;
        this.random = null;
    }
}


class Solution {
    
    class Pair {
        Node originalNode;
        Node nextNode;
        
        public Pair(Node node1, Node node2) {
            this.originalNode = node1;
            this.nextNode = node2;
        }
    }
    
    public Node copyRandomList(Node head) {
        return copyRandomListSol1(head);
    }
    
    public Node copyRandomListSol1(Node head) {
        if (head == null) {
            return head;
        }
        
        List<Pair> origListNodeAndNextNode = new ArrayList<Pair>();
        
        // first copy the original list as it is
        // also copy orignal list node and its next node in an array
        Node copyListHead = null;
        Node copyListCurr = null;
        Node curr = head;
        while(curr != null) {
            if(copyListHead == null) {
                copyListHead = new Node(curr.val);
                copyListCurr = copyListHead;
            } else {
                copyListCurr.next = new Node(curr.val);
                copyListCurr = copyListCurr.next;
            }
            origListNodeAndNextNode.add(new Pair(curr, curr.next));
            curr = curr.next;
        }
        
        // again traverse the original list and mark next pointer of original node
        // to corresponding node in copy list and point random pointer of the corresponding
        // copy node to original node
        curr = head;
        copyListCurr = copyListHead;
        while(curr != null) {
            Node temp = curr;
            curr = curr.next;
            temp.next = copyListCurr;
            copyListCurr.random = temp;
            copyListCurr = copyListCurr.next;
        }
        
        // now traverse copy list and its random pointer is set to random ptr =
        // rand ptr (orig node) -> rand ptr (orig node rand ptr) -> next (copy list node)
        copyListCurr = copyListHead;
        while (copyListCurr != null) {
            if (copyListCurr.random.random == null) {
                copyListCurr.random = null;
            } else {
                copyListCurr.random = copyListCurr.random.random.next;
            }
            
            copyListCurr = copyListCurr.next;
        }
        
        // revert the original list's next pointer
        for(Pair nodePair: origListNodeAndNextNode) {
            nodePair.originalNode.next = nodePair.nextNode;
        }
        
        return copyListHead;
        
    }
}