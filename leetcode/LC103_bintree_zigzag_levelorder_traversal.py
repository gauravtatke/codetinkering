# Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
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
# return its zigzag level order traversal as:
#
# [
#   [3],
#   [20,9],
#   [15,7]
# ]

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution1:
    def zigzagLevelOrder(self, root):
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
        zigzag = 0  # 0 means left to right, 1 means right to left
        while dq:
            level = []
            if not zigzag:
                # left to right
                for i in range(len(dq)):
                    node = dq.popleft()
                    level.append(node.val)
                    if node.left:
                        dq.append(node.left)
                    if node.right:
                        dq.append(node.right)
            else:
                for i in range(len(dq)):
                    # take out in right to left
                    node = dq.pop()
                    level.append(node.val)
                    if node.right:
                        dq.appendleft(node.right)
                    if node.left:
                        dq.appendleft(node.left)
            result.append(level)
            zigzag = not zigzag
        return result


class Solution2:
    def zigzagLevelOrder(self, root):
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
        zigzag = 0  # 0 means left to right, 1 means right to left
        while dq:
            level = []
            for i in range(len(dq)):
                node = dq.popleft()
                level.append(node.val)
                if node.left:
                    dq.append(node.left)
                if node.right:
                    dq.append(node.right)
            if zigzag:
                # means right to left
                level.reverse()
            result.append(level)
            zigzag = not zigzag
        return result
