import java.util.HashMap;

/**
 * given in_order and pre_order traversal for a tree, print post order traversal
 * 1. one way is to contruct tree and then do postOrderTraversal
 * 2. Second way to directly print postOrder because construction of tree is similar to postOrderTraversal
 */
public class PostOrderFromInorderPreOrder {
    public static int pre_count = 0;

    public static void printPostOrder(int[] inOrder, int[] preOrder) {
        HashMap<Integer, Integer> indexMap = new HashMap<>();

        // put node and its index in inOrder in hashmap for easy lookup
        for(int i = 0; i < inOrder.length; i++) {
            indexMap.put(inOrder[i], i);
        }

        postOrderTraversal(preOrder, 0, inOrder.length-1, indexMap);
    }

    public static void postOrderTraversal(int[] preOrder, int start, int end, HashMap<Integer, Integer> indexMap) {

    }

    public static void constructTreeAndPrintPostOrder(int[] inOrder, int[] preOrder) {
        pre_count = 0;
        HashMap<Integer, Integer> indexMap = new HashMap<>();

        // put node and its index in inOrder in hashmap for easy lookup
        for(int i = 0; i < inOrder.length; i++) {
            indexMap.put(inOrder[i], i);
        }

        TreeNode root = constructTree(preOrder, 0, inOrder.length-1, indexMap);
        TreeTraversal.postOrderTraversal(root);
    }

    public static TreeNode constructTree(int[] preOrder, int start, int end, HashMap<Integer, Integer> indexMap) {
        // root is always the first node of preOrder traversal
        TreeNode root = new TreeNode(preOrder[pre_count++]);
        if (start == end) {
            return root;
        }

        // find the location of root node in inOrder traversal.
        // all element before that index are part of left subtree
        // all element after that index are part of right subtree
        int rootLoc = indexMap.get(root.data);
        root.left = constructTree(preOrder, start, rootLoc-1, indexMap);
        root.right = constructTree(preOrder, rootLoc+1, end, indexMap);
        return root;
    }
    public static void main(String[] args) {
        int inOrder[] = {4, 2, 1, 7, 5, 8, 3, 6};
        int preOrder[] = {1, 2, 4, 3, 5, 7, 8, 6};

        PostOrderFromInorderPreOrder p = new PostOrderFromInorderPreOrder();
        // method 1: construct the tree and then print postOrderTraversal
        constructTreeAndPrintPostOrder(inOrder, preOrder);

        //method 2: directly print postOrder Traversal
        printPostOrder(inOrder, preOrder);
    }
}