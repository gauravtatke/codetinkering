# Given inorder and postorder traversal of a tree, construct the binary tree.
# Note:
# You may assume that duplicates do not exist in the tree.
# For example, given
# inorder = [9,3,15,20,7]
# postorder = [9,15,7,20,3]
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
    def buildTree(self, inorder, postorder):
        """
        :type inorder: List[int]
        :type postorder: List[int]
        :rtype: TreeNode
        """
        return self.buildTreeRec(0,
                                 len(postorder) - 1, 0,
                                 len(inorder) - 1, inorder, postorder)

    def buildTreeRec(self, postStart, postEnd, inStart, inEnd, inorder,
                     postorder):
        # print('inStart = {}, inEnd = {}, postEnd = {}'.format(
        #     inStart, inEnd, postEnd))
        if postStart > postEnd or inStart > inEnd:
            return None

        root = TreeNode(postorder[postEnd])
        for i in range(inStart, inEnd + 1):
            if root.val == inorder[i]:
                inMid = i
                # print('inMid = ', inMid)
                break

        # postStart remains same, inMid-inStart gives number of elem in left subtree
        # postEnd index should be one less than the number of items in left subtree such that 
        # postStart..postEnd index correctly represent left subtree in post order traversal.
        # postEnd = postStart + inMid - inStart -1
        root.left = self.buildTreeRec(postStart,
                                      postStart + inMid - inStart - 1, inStart,
                                      inMid - 1, inorder, postorder)

        # postEnd is reduced by one because that's the end of the right subtree. To find the start of the right subtree
        # postStart is incremented by number of items in between inMid and inStart i.e. 
        # postStart = postStart + inMid - inStart
        root.right = self.buildTreeRec(postStart + inMid - inStart,
                                       postEnd - 1, inMid + 1, inEnd, inorder,
                                       postorder)
        return root


def main():
    inorder = [9, 3, 15, 20, 7]
    postorder = [9, 15, 7, 20, 3]
    root = Solution().buildTree(inorder, postorder)


if __name__ == '__main__':
    main()