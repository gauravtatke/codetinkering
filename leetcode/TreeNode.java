// Definition for a binary tree node.
 public class TreeNode {
 int val;
 TreeNode left;
      TreeNode right;
      TreeNode() {}
      TreeNode(int val) { this.val = val; }
      TreeNode(int val, TreeNode left, TreeNode right) {
          this.val = val;
          this.left = left;
          this.right = right;
    }

    public static TreeNode constructTree(Integer[] arr) {
        // construct tree from Integer array and returns the root
        // simple helper method
        if (arr == null || arr.length == 0) {
            return null;
        }
        TreeNode root = null;
        return insertLevelOrder(arr, root, 0);
    }

    public static TreeNode insertLevelOrder(Integer[] array, TreeNode root, int index) {
        // for any node at index i, left child is at location 2*i + 1 and right child is at 2*i + 2
        // this considering index starting from zero for root
        if (index < array.length) {
            if (array[index] == null) {
                return root;
            }
            root = new TreeNode(array[index]);
            root.left = insertLevelOrder(array, root.left, 2*index + 1);
            root.right = insertLevelOrder(array, root.right, 2*index + 2);
            return root;
        }
        return null;
    }
 }
 