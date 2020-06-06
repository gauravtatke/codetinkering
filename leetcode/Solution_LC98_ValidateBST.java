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

/* Given a binary tree, determine if it is a valid binary search tree (BST).

Assume a BST is defined as follows:

    The left subtree of a node contains only nodes with keys less than the node's key.
    The right subtree of a node contains only nodes with keys greater than the node's key.
    Both the left and right subtrees must also be binary search trees.
 */
 
import java.util.*;

class Solution_LC98_ValidateBST {
    public boolean isValidBST(TreeNode root) {
        return isValidBSTSol1Recursive(root, null, null);
    }
    
    
    public boolean isValidBSTSol1Recursive(TreeNode root, TreeNode minNode, TreeNode maxNode) {
        if (root == null) {
            return true;
        }
        
        // if root val is greater or equal to minVal then false, similarly for right node
        if ((minNode != null && root.val >= minNode.val) || (maxNode != null && root.val <= maxNode.val)) {
            return false;
        }
        
        return isValidBSTSol1Recursive(root.left, root, null) && isValidBSTSol1Recursive(root.right, null, root);
    }
    
    public boolean isValidBSTSol2Iter(TreeNode root) {
        // we can do inorder traversal and store the result in array
        // then traverse the array and check if any prev value is greater than curr value
        // array should be sorted
        // But better way to do it is to not traverse complete tree. Instead every time we pop
        // from stack, we can compare it with prev value. if less than prev val then its NOT BST because its supposed to yield increasing values
        // Integer prev = Integer.MIN_VALUE; // this is not working bcoz a test case as [Integer.MIN_VALUE]
        // double prev = Double.MIN_VALUE; // not working bcoz this positive min value so small that 0 <= MIN_VALUE in jvm and a test case has [0]
        double prev = - Double.MAX_VALUE;
        TreeNode curr = root;
        Stack<TreeNode> s = new Stack<TreeNode>();
        while (curr != null || !s.isEmpty()) {
            while (curr != null) {
                s.push(curr);
                curr = curr.left;
            }
            
            curr = s.pop();
            if (curr.val <= prev) {
                System.out.println("Curr val: " + curr.val + ", prev val: " + prev);
                return false;
            }
            
            prev = curr.val;
            curr = curr.right;
        }
        return true;
    }

    public static void main(String args[]) {
        Integer[] nodes = {0};
        TreeNode root = TreeNode.constructTree(nodes);
        Solution_LC98_ValidateBST sol = new Solution_LC98_ValidateBST();
        System.out.println(sol.isValidBSTSol2Iter(root));
    }
}