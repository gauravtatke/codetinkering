# Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
#
# For example:
# Given binary tree [3,9,20,null,null,15,7],
#
#     3
#    / \
#   9  20
#     /  \
#    15   7
#
# return its level order traversal as:
#
# [
#   [3],
#   [9,20],
#   [15,7]
# ]

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def levelOrder(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """
        from collections import deque
        if root is None:
            return []
        dq = deque()
        dq.append(root)
        result = []
        while dq:
            level = []
            for i in range(len(dq)):
                temp = dq.popleft()
                if temp.left:
                    dq.append(temp.left)
                if temp.right:
                    dq.append(temp.right)
                level.append(temp.val)
            result.append(level)
        return result
