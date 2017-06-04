#!/usr/bin/env python3

# Invert a binary tree.
#
#      4
#    /   \
#   2     7
#  / \   / \
# 1   3 6   9

# to
#      4
#    /   \
#   7     2
#  / \   / \
# 9   6 3   1

from collections import deque

class TreeNode:
    def __init__(self, arg):
        self.val = arg
        self.left = None
        self.right = None

def invertTree(root):
    if root is None:
        return None
    if root.left is None and root.right is None:
        return root
    elif root.left is None:
        root.left = invertTree(root.right)
        root.right = None
    elif root.right is None:
        root.right = invertTree(root.left)
        root.left = None
    else:
        temp = invertTree(root.left)
        root.left = invertTree(root.right)
        root.right = temp
    return root

def invertTree_BFS(root):
        if root is None:
            return None
        dq = deque(root)
        while dq:
            node = dq.popleft()
            node.left, node.right = node.right, node.left
            if node.left is not None:
                dq.append(node.left)
            if node.right is not None:
                dq.append(node.right)
        return root
