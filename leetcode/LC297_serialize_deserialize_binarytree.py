# Serialization is the process of converting a data structure or object into a sequence of bits
# so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
# Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work.
# You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
#
# For example, you may serialize the following tree
#
#     1
#    / \
#   2   3
#      / \
#     4   5
#
# as "[1,2,3,null,null,4,5]", just the same as how LeetCode OJ serializes
# a binary tree. You do not necessarily need to follow this format, so
# please be creative and come up with different approaches yourself.


class Codec:
    splitter = ','
    nullStr = 'NULL'

    def serialize(self, root):
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """
        def build_list(node, ln):
            if node is None:
                ln.append(self.nullStr)
                return
            # pre-order traversal
            ln.append(str(node.val))
            build_list(node.left, ln)
            build_list(node.right, ln)

        list_node = []
        build_list(root, list_node)
        return self.splitter.join(list_node)

    def deserialize(self, data):
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        def createTree(list_node):
            try:
                nval = list_node.pop()
            except IndexError:
                return

            if nval == self.nullStr:
                return None

            node = TreeNode(nval)
            node.left = createTree(list_node_val)
            node.right = createTree(list_node_val)
            return node

        list_node_val = data.split(self.splitter)
        list_node_val.reverse()
        root = createTree(list_node_val)
        return root
