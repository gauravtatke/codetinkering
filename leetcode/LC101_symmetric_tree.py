# Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
#
# For example, this binary tree [1,2,2,3,4,4,3] is symmetric:
#
#     1
#    / \
#   2   2
#  / \ / \
# 3  4 4  3


class Solution1(object):
    # recursive solution
    def isSymmetric(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        def isEqual(left, right):
            if left is None and right is None:
                return True
            elif left is not None and right is not None:
                if left.val != right.val:
                    return False
                else:
                    return isEqual(left.left, right.right) and isEqual(left.right, right.left)
            else:
                return False

        if root is None:
            return True
        return isEqual(root.left, root.right)


class Solution2(object):
    # iterative solution
    def isSymmetric(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        stack = []
        if root is None:
            return True
        if root.left is not None:
            if root.right is None:
                return False
            stack.append(root.left)
            stack.append(root.right)
        elif root.right is not None:
            return False

        while len(stack):
            if len(stack) % 2:
                return False
            right = stack.pop()
            left = stack.pop()
            if left.val != right.val:
                return False
            if left.left is not None:
                if right.right is None:
                    return False
                stack.append(left.left)
                stack.append(right.right)
            elif right.right is not None:
                return False

            if left.right is not None:
                if right.left is None:
                    return False
                stack.append(left.right)
                stack.append(right.left)
            elif right.left is not None:
                return False
        return True
