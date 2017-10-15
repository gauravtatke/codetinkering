# Implement an iterator over a binary search tree (BST). Your iterator will be initialized with the root node of a BST.
# Calling next() will return the next smallest number in the BST.
# Note: next() and hasNext() should run in average O(1) time and uses O(h)
# memory, where h is the height of the tree.


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class BSTIterator(object):
    def __init__(self, root):
        """
        :type root: TreeNode
        """
        self._stack = []
        if root is not None:
            self.insertNode(root)

    def insertNode(self, node):
        curr = node
        while curr is not None:
            self._stack.append(curr)
            curr = curr.leftnu

    def hasNext(self):
        """
        :rtype: bool
        """
        return len(self._stack) > 0

    def next(self):
        """
        :rtype: int
        """
        retnode = self._stack.pop()
        if retnode.right is not None:
            self.insertNode(retnode.right)
        return retnode.val
