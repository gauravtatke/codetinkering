import java.util.HashMap;

/**
 * given in_order and pre_order traversal for a tree, print post order traversal
 * 1. one way is to contruct tree and then do postOrderTraversal
 * 2. Second way to directly print postOrder because construction of tree is similar to postOrderTraversal
 */
public class PostOrderFromInorderPreOrder {
    public static int preCount;

    public static void printPostOrder(int[] inOrder, int[] preOrder) {
        HashMap<Integer, Integer> indexMap = new HashMap<>();

        // put node and its index in inOrder in hashmap for easy lookup
        for(int i = 0; i < inOrder.length; i++) {
            indexMap.put(inOrder[i], i);
        }
        preCount = 0;
        postOrderTraversal(preOrder, 0, inOrder.length-1, indexMap);
        System.out.println();
    }

    public static void postOrderTraversal(int[] preOrder, int start, int end, HashMap<Integer, Integer> indexMap) {
        // this is very similar to constructTree but instead of returning root node,
        // we will just print it
        if (start > end) {
            return;
        }

        int root = preOrder[preCount++];
        if (start == end) {
            // this is a leaf node so just print it
            System.out.print(root + ", ");
            return;
        }

        // if not a leaf node
        int rootLoc = indexMap.get(root);
        postOrderTraversal(preOrder, start, rootLoc-1, indexMap); // for left subtree
        postOrderTraversal(preOrder, rootLoc+1, end, indexMap); // for right subtree
        System.out.print(root + ", ");
        return;
    }

    public static void constructTreeAndPrintPostOrder(int[] inOrder, int[] preOrder) {
        preCount = 0;
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
        // System.out.println("pre count: " + preCount);
        if (start > end) {
            return null;
        }

        TreeNode root = new TreeNode(preOrder[preCount++]);
        // System.out.println("start: " + start);
        // System.out.println("end: " + end);
        
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
        // post order should be = 4, 2, 7, 8, 5, 6, 3, 1

        PostOrderFromInorderPreOrder p = new PostOrderFromInorderPreOrder();
        // method 1: construct the tree and then print postOrderTraversal
        constructTreeAndPrintPostOrder(inOrder, preOrder);
        System.out.println();
        
        //method 2: directly print postOrder Traversal
        printPostOrder(inOrder, preOrder);
    }
}