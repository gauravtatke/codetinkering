# Given a binary tree, determine if it is height-balanced.
# For this problem, a height-balanced binary tree is defined as:
#     a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

# Example 1:
# Given the following tree [3,9,20,null,null,15,7]:

#     3
#    / \
#   9  20
#     /  \
#    15   7

# Return true.

# Example 2:
# Given the following tree [1,2,2,3,3,null,null,4,4]:

#        1
#       / \
#      2   2
#     / \
#    3   3
#   / \
#  4   4

# Return false

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution1:
    def isBalanced(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        if root is None:
            return True
        hleft = self.height(root.left)
        hright = self.height(root.right)

        if abs(hleft - hright) > 1 and self.isBalanced(
                root.left) and self.isBalanced(root.right):
            return True
        return False

    def height(self, root):
        if root is None:
            return 0
        return max(self.height(root.left), self.height(root.right)) + 1


class Solution2:
    def isBalanced(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        # another way is to return height at each recursion and -1 if it is un-balanced.
        # no need to traverse tree twice, once for height and once for isBalance as in above solution
        height = self.heightBalance(root)
        if height == -1:
            return False
        return True

    def heightBalance(self, root):
        if root is None:
            return 0
        left_height = self.heightBalance(root.left)
        if left_height == -1:
            return -1

        right_height = self.heightBalance(root.right)
        if right_height == -1:
            return -1

        if abs(left_height - right_height) > 1:
            return -1
        return max(left_height, right_height) + 1
