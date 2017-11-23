# Given a binary tree, determine if it is a valid binary search tree (BST).
#
# Assume a BST is defined as follows:
#
#     The left subtree of a node contains only nodes with keys less than the node's key.
#     The right subtree of a node contains only nodes with keys greater than the node's key.
#     Both the left and right subtrees must also be binary search trees.


def isValidBST(root):
    def isValid(root, minNode, maxNode):
        # define the min and max between which the node val should be
        if root is None:
            return True
        if (minNode and minNode.val >= root.val) or (maxNode and maxNode.val <= root.val):
            return False
        return isValid(root.left, minNode, root) and isValid(root.right, root, maxNode)
    return isValid(root, None, None)
