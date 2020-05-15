import java.util.LinkedList;
import java.util.Queue;
import java.util.Stack;

public class TreeTraversal {
    public static void inOrderTraversal(TreeNode root) {
        if (root == null) {
            return;
        }

        if (root.left == null && root.right == null) {
            System.out.print(root + ", ");
            return;
        }
        inOrderTraversal(root.left);
        System.out.print(root + ", ");
        inOrderTraversal(root.right);
    }

    public static void preOrderTraversal(TreeNode root) {
        if (root == null) {
            return;
        }
        if (root.left == null && root.right == null) {
            System.out.println(root);
            return;
        }

        System.out.println(root);
        preOrderTraversal(root.left);
        preOrderTraversal(root.right);
    }

    public static void postOrderTraversal(TreeNode root) {
        if (root == null) {
            return;
        }

        if (root.left == null && root.right == null) {
            System.out.print(root + ", ");
            return;
        }

        postOrderTraversal(root.left);
        postOrderTraversal(root.right);
        System.out.print(root + ", ");
    }

    public static void levelOrderTraversal(TreeNode root) {
        // also known as depth first search
        if (root == null || (root.left == null && root.right == null)) {
            System.out.println(root);
            return;
        }

        Queue<TreeNode> queue = new LinkedList<>();
        queue.add(root);
        TreeNode temp;
        while((temp = queue.poll()) != null) {
            System.out.println(temp);
            if (temp.left != null) {
                queue.add(temp.left);
            }

            if (temp.right != null) {
                queue.add(temp.right);
            }
        }
    }

    public static int height(TreeNode root) {
        int heightOfTree = 0;
        if (root == null) {
            return 0;
        }

        int leftHeightOfTree = 0;
        int rightHeightOfTree = 0;
        if (root.left != null) {
            leftHeightOfTree = height(root.left);
        }

        if (root.right != null) {
            rightHeightOfTree = height(root.left);
        }

        heightOfTree = Integer.max(leftHeightOfTree, rightHeightOfTree)+1;
        return heightOfTree;
    }

    public static void printGivenLevel(TreeNode root, int level) {
        if (root == null) {
            return;
        }

        if (level == 1) {
            System.out.print(root + " ");
        } else if (level > 1) {
            printGivenLevel(root.left, level-1);
            printGivenLevel(root.right, level-1);
        }
    }

    public static void levelOrderTraversalUsingHeight(TreeNode root) {
        if (root == null) {
            return;
        }

        for(int i = 1; i <= height(root); i++) {
            printGivenLevel(root, i);
            System.out.println("");
        }
    }

    public static void iterativeInOrderTraversal(TreeNode root) {
        Stack<TreeNode> stack1 = new Stack<TreeNode>();
        TreeNode current = root;
        while (current != null || !stack1.empty()) {
            if (current != null) {
                stack1.push(current);
                current = current.left;
            } else if (!stack1.empty()){
                current = stack1.pop();
                System.out.print(current + ", ");
                current = current.right;
            }
        }
        System.out.println();
    }

    public static void main(String[] args) {
        Tree tree = new Tree(1);
        tree.root.left = new TreeNode(2);
        tree.root.right = new TreeNode(3);
        tree.root.left.left = new TreeNode(4);
        tree.root.right.right = new TreeNode(5);
        TreeTraversal.inOrderTraversal(tree.root);
        //TreeTraversal.levelOrderTraversal(tree.root);
        // System.out.println(TreeTraversal.height(tree.root));
        // printGivenLevel(tree.root, 3);
        // levelOrderTraversalUsingHeight(tree.root);
        System.out.println();
        TreeTraversal.iterativeInOrderTraversal(tree.root);
    }
    
}