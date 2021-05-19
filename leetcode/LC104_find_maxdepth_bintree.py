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
    
    def __str__(self):
        return f'{self.val}'

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

def max_depth_iter(root: TreeNode) -> int:
    if root is None:
        return 0
    level_list = [[root]]  # each level is new list in the level list
    for node_list in level_list:
        child_node_list = []
        for node in node_list:
            if node:
                child_node_list.append(node.left)
                child_node_list.append(node.right)
        # one level is completely added to child node
        if child_node_list:
            level_list.append(child_node_list)
    # level list len is the depth
    print(str(level_list))
    return len(level_list)

if __name__ == '__main__':
    root = TreeNode(3)
    root.left = TreeNode(9)
    root.right = TreeNode(7)
    root.right.left = TreeNode(20)
    root.right.right = TreeNode(11)
    depth = max_depth_iter(root)
    print(f'max depth = {depth}')
