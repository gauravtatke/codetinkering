#!/usr/bin/env python3

# Given a binary tree, find its maximum depth.
#
# The maximum depth is the number of nodes along the longest path from the root
# node down to the farthest leaf node.

from collections import deque

class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None

def maxDepth(root):
    if root is None:
        return 0
    if root.left is None and root.right is None:
        return 1
    return max(maxDepth(root.left), maxDepth(root.right)) + 1

def maxDepth_2(root):
    if root is None:
        return 0
    elif root.left is None and root.right is not None:
        return maxDepth_2(root.right)+1
    elif root.left is not None and root.right is None:
        return maxDepth_2(root.left)+1
    elif root.left is None and root.right is None:
        return 1
    else:
        return max(maxDepth_2(root.left), maxDepth_2(root.right))+1

def maxDepth_BFS(root):
    dq = deque()
    if root is None:
        return 0
    dq.append(root)
    count = 0
    while dq:
        for i in range(len(dq)):
            node = dq.popleft()
            if node.left is not None:
                dq.append(node.left)
            if node.right is not None:
                dq.append(node.right)
        count += 1
    return count
