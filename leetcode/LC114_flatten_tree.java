import java.util.Stack;

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


public class LC114_flatten_tree {
    public void flatten(TreeNode root) {
        // flatten_iterative(root);
        root = flatten_recursive(root);
    }
    
    public void flatten_iterative(TreeNode root) {
        Stack<TreeNode> stack = new Stack<TreeNode>();
        stack.push(null);
        TreeNode curr = root;
        while (curr != null ) {
            if (curr.right != null) {
                stack.push(curr.right);
            }
            if (curr.left != null) {
                curr.right = curr.left;
                curr.left = null;
                curr = curr.right;
            } else {
                curr.right = stack.pop();
                curr = curr.right;
            }  
        }
    }
    
    public TreeNode flatten_recursive(TreeNode root) {
        if (root == null) {
            return null;
        }
        
        if (root.left == null && root.right == null) {
            return root;
        }
        
        TreeNode rootLeft = flatten_recursive(root.left);
        TreeNode rootRight = flatten_recursive(root.right);
        root.left = null;
        if (rootLeft == null) {
            root.right = rootRight;
            return root;
        }
        
        root.right = rootLeft;
        while (rootLeft.right != null) {
            rootLeft = rootLeft.right;
        }
        
        // rootLeft pointing to last node in chain
        // append rootRight here
        rootLeft.right = rootRight;
        return root;
    }
}