# Two elements of a binary search tree (BST) are swapped by mistake.
# Recover the tree without changing its structure.

# Note:
# A solution using O(n) space is pretty straight forward. Could you devise
# a constant space solution?


class Solution1:
    def recoverTree(self, root):
        """
        :type root: TreeNode
        :rtype: void Do not return anything, modify root in-place instead.
        """
        nlist = []
        self.inorder_traverse(root, nlist)
        node1, node2 = self.find_node_to_swap(nlist)
        node1.val, node2.val = node2.val, node1.val

    def find_node_to_swap(self, nlist):
        # assume nlist should be always sorted.
        # find nodes which are not sorted.
        i = j = None
        left = 0
        for right in range(1, len(nlist)):
            if nlist[left].val > nlist[right].val:
                if i is None:
                    # first time, set i
                    i = left
                else:
                    # second time set j
                    j = right
                    break
            left += 1
        if j is None:
            # means node is swapped with its parent
            j = i + 1
        return nlist[i], nlist[j]

    def inorder_traverse(self, root, nlist):
        if root is None:
            return
        self.inorder_traverse(root.left, nlist)
        nlist.append(root)
        self.inorder_traverse(root.right, nlist)


class Solution2:
    def recoverTree(self, root):
        """
        :type root: TreeNode
        :rtype: void Do not return anything, modify root in-place instead.
        """
        self.first_node = None
        self.second_node = None
        self.prev_node = TreeNode(float('-inf'))
        self.inorder_traverse(root)
        self.first_node.val, self.second_node.val = self.second_node.val, self.first_node.val

    def inorder_traverse(self, root):
        if root is None:
            return
        # visit the left node
        self.inorder_traverse(root.left)

        # process current node
        if self.first_node is None and self.prev_node.val > root.val:
            self.first_node = self.prev_node
        if self.first_node is not None and self.prev_node.val > root.val:
            self.second_node = root

        self.prev_node = root

        # process the right subtree
        self.inorder_traverse(root.right)


class Solution3:
    # using morris trversal
    def recoverTree(self, root):
        """
        :type root: TreeNode
        :rtype: void Do not return anything, modify root in-place instead.
        """
        first = second = None
        prev = None
        curr = root
        while curr is not None:
            if curr.left is None:
                # process curr
                if prev is not None and prev.val > curr.val:
                    if first is None:
                        first = prev
                    second = curr
                prev = curr
                curr = curr.right
            else:
                rightmost = curr.left
                while rightmost.right is not None and rightmost.right != curr:
                    rightmost = rightmost.right
                if rightmost.right is None:
                    rightmost.right = curr
                    curr = curr.left
                else:
                    if prev is not None and prev.val > curr.val:
                        if first is None:
                            first = prev
                        second = curr
                    prev = curr
                    rightmost.right = None
                    curr = curr.right

        first.val, second.val = second.val, first.val
