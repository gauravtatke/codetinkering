# Given an integer n, generate all structurally unique BST's (binary search trees) that store values 1...n.
#
# For example,
# Given n = 3, your program should return all 5 unique BST's shown below.
#
#    1         3     3      2      1
#     \       /     /      / \      \
#      3     2     1      1   3      2
#     /     /       \                 \
#    2     1         2                 3


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


def unique_BST(num):
    def generate_tree(start, end):
        temp = []
        if start > end:
            temp.append([])
            return temp
        if start == end:
            temp.append(TreeNode(start))
            return temp
        for i in range(start, end + 1):
            left_list = generate_tree(start, i - 1)
            right_list = generate_tree(i + 1, end)
            for ltree in left_list:
                for rtree in right_list:
                    root = TreeNode(i)
                    root.left = ltree
                    root.right = rtree
                    temp.append(root)
        return temp
    return generate_tree(1, num)


def unique_BST_DP(n):
    # result[n] stores the result of unique bst with n elem
    # if (n+1)th elem is added, we have to generate tree with 1 to n+1 as root.
    # For n elem in BST, for every i there are i-1 nodes in left subtree and  n-i nodes in right subtree.
    # i.e. result[i-1] left subtrees and result[n-i] right subtrees. left
    # subtree will remains same because for new elem added, it will greater
    # than all prev elem. right subtree should be cloned with val offset with i.
    # for e.g. result[2] = [(1,2), (2,1)] only 2 ways to arrange two nodes.
    # When n = 5, with i=3 as root there will be 5-3 = 2 nodes in right
    # subtree. i.e. result[2] right subtrees but instead of [(1,2), (2,1)]
    # they will be offset by 3 giving [(4,5), (5,4)]
    def clone(root, offset):
        if root is None:
            return None
        temproot = TreeNode(root.val + offset)
        temproot.left = clone(root.left, offset)
        temproot.right = clone(root.right, offset)
        return temproot
    result = [[] for i in range(n + 1)]
    result[0].append(None)
    if not n:
        return result[n]
    for i in range(1, n + 1):
        for rootval in range(1, i + 1):
            for l_subtree in result[rootval - 1]:
                for rtree in result[i - rootval]:
                    root = TreeNode(rootval)
                    root.left = l_subtree
                    root.right = clone(rtree, rootval)
                    result[i].append(root)
    return result[n]


def main():
    pass


if __name__ == '__main__':
    main()
