from TreeNode import BinTreeNode

class BinaryTree(object):
    def __init__(self, key):
        self.root = BinTreeNode(key)
        #self.insert(tnode)

    def insert(self, node):
        raise NotImplementedError

    def delete(self, node):
        raise NotImplementedError

    def search(self, key):
        raise NotImplementedError

    def preorder_walk(self, tnode):
        if tnode is not None:
            print(tnode)
            self.preorder_walk(tnode.left)
            self.preorder_walk(tnode.right)

    def postorder_walk(self, tnode):
        if tnode is not None:
            self.postorder_walk(tnode.left)
            self.postorder_walk(tnode.right)
            print(tnode)

    def inorder_walk(self, tnode):
        if tnode is not None:
            self.inorder_walk(tnode.left)
            print(tnode)
            self.inorder_walk(tnode.right)

