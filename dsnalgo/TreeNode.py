#This file contains only class definition for a tree node

from functools import total_ordering

@total_ordering
class BinTreeNode(object):
    def __init__(self, key):
        self.key = key
        self.left = None
        self.right = None

    def __eq__(self, other):
        if other is None:
            return False
        return self.key == other.key

    def __lt__(self, other):
        if other is None:
            return False
        return self.key < other.key

    def __str__(self):
        if self is None:
            return str(None)
        lt = str(None) if self.left is None else self.left.key
        rt = str(None) if self.right is None else self.right.key
        str1 = "key = {0:5d}, left = {1:5}, right = {2:5}".format(self.key, lt, rt)
        return str1

