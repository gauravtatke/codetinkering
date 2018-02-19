# Given preorder and inorder traversal of a tree, construct the binary tree.
# Note:
# You may assume that duplicates do not exist in the tree.
# For example, given
# preorder = [3,9,20,15,7]
# inorder = [9,3,15,20,7]
# Return the following binary tree:

#     3
#    / \
#   9  20
#     /  \
#    15   7


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def buildTree(self, preorder, inorder):
        """
        :type preorder: List[int]
        :type inorder: List[int]
        :rtype: TreeNode
        """
        # in preorder trversal root is at first. find root element in inorder list.
        # all elements to left in order will be in left subtree and all in the right will in rigt subtree
        return self.buildTreeRec(0, 0, len(inorder) - 1, preorder, inorder)

    def buildTreeRec(self, preStart, inStart, inEnd, preorder, inorder):
        # print('preStart = {}, inStart = {}, inEnd = {}'.format(
        #     preStart, inStart, inEnd))
        if preStart >= len(preorder) or inStart > inEnd:
            return None

        root = TreeNode(preorder[preStart])
        for i in range(inStart, inEnd + 1):
            if inorder[i] == root.val:
                inMid = i
                # print('inMid = {}'.format(inMid))
                break
        root.left = self.buildTreeRec(preStart + 1, inStart, inMid - 1,
                                      preorder, inorder)
        root.right = self.buildTreeRec(preStart + inMid - inStart + 1,
                                       inMid + 1, inEnd, preorder, inorder)
        # root.right = self.buildTreeRec(inMid + 1, inMid + 1, inEnd, preorder,
        #                                inorder)
        return root


def main():
    preorder = [1, 2, 3]
    inorder = [2, 3, 1]
    root = Solution().buildTree(preorder, inorder)


if __name__ == '__main__':
    main()