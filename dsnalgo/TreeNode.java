public class TreeNode {
    int data;
    TreeNode left;
    TreeNode right;

    public TreeNode(int data) {
        this.data = data;
        this.left = null;
        this.right = null;
    }

    public TreeNode() {
        this.data = Integer.MIN_VALUE;
        this.left = null;
        this.right = null;
    }

    public String toString() {
        return String.valueOf(this.data);
    }
}