/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

/* Given a binary search tree, write a function kthSmallest to find the kth smallest element in it.
Example 1:

Input: root = [3,1,4,null,2], k = 1
   3
  / \
 1   4
  \
   2
Output: 1

Example 2:

Input: root = [5,3,6,2,4,null,null,1], k = 3
       5
      / \
     3   6
    / \
   2   4
  /
 1
Output: 3
 */

import java.util.*;

public class LC230_kth_smallest_BST {
    public int kthSmallest(TreeNode root, int k) {
        return kthSmallestSol1(root, k);
    }
    
    public int kthSmallestSol1(TreeNode root, int k) {
        // recursive solution
        // first collect all the items doing inorder traversal
        // so that its a sorted list
        ArrayList<Integer> nums = new ArrayList<Integer>();
        inOrderCollect(root, nums);
        return nums.get(k-1);
    }
    
    public void inOrderCollect(TreeNode root, ArrayList<Integer> nums) {
        if (root == null) {
            return;
        }
        
        if (root.left == null && root.right == null) {
            nums.add(root.val);
            return;
        }
        
        inOrderCollect(root.left, nums);
        nums.add(root.val);
        inOrderCollect(root.right, nums);
    }
    
    public int kthSmallestSol2(TreeNode root, int k) {
        // iterative inorder traversal.
        // so we don't need to traverse complete tree
        Stack<TreeNode> stack = new Stack<TreeNode>();
        TreeNode curr = root;
        // keep traversing to left and push the curr into stack
        // left most node is smallest
        while (curr != null || !stack.isEmpty()) {
            while (curr != null) {
                stack.push(curr);
                curr = curr.left;
            }
            // curr is null and time to pop item from stack
            curr = stack.pop();
            // reduce k count because everytime we pop
            // we will get smallest element so far         
            k--;
            if (k == 0) {
                // means what we popped is kth smallest element
                return curr.val;
            }
            curr = curr.right;
        }
        return -1;
    }
}