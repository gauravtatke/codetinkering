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

import java.util.*;

class LC129_sum_root_to_leaf {
    public int sumNumbers(TreeNode root) {
        return sumNumbers_sol2(root);
    }
    
    public int sumNumbers_sol1(TreeNode root) {
        ArrayList<Integer> sumList = new ArrayList<Integer>();
        if (root == null) {
            return 0;
        }
        
        sumRootToLeaf(root, root.val, sumList);
        Integer sum = 0;
        for(Integer num: sumList) {
            System.out.println(num);
            sum += num;
        }
        return sum;
    }
    
    public void sumRootToLeaf(TreeNode root, int val, ArrayList<Integer> sumList) {
        if (root.left == null && root.right == null) {
            // if no left & right child then add val to sumList
            // val represent the number from root to leaf of one branch
            sumList.add(val);
        }

        if (root.left != null) {
            // construct the number with left node
            int data = val*10 + root.left.val;
            sumRootToLeaf(root.left, data, sumList);
        }
        
        if (root.right != null) {
            int data = val*10 + root.right.val;
            sumRootToLeaf(root.right, data, sumList);
        }
    }
    
    public int sumNumbers_sol2(TreeNode root) {
        return getSum(root, 0);
    }
    
    public int getSum(TreeNode root, int sum) {
        if (root == null) {
            return 0;
        }
        
        // if root not null, add root's val to sum
        sum = sum*10 + root.val;
        if (root.left == null && root.right == null) {
            return sum;
        }
        return getSum(root.left, sum) +  getSum(root.right, sum);
        
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(4);
        root.left = new TreeNode(9);
        root.right = new TreeNode(0);
        root.left.left = new TreeNode(5);
        root.left.right = new TreeNode(1);
        LC129_sum_root_to_leaf sol = new LC129_sum_root_to_leaf();
        System.out.println("Sum root to leaf: " + sol.sumNumbers(root));
    }
}