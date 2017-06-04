import pdb
from BinarySearchTree import BSTNode
from BinarySearchTree import BST

class AVLNode(BSTNode):
    def __init__(self, key):
        super().__init__(key)
        self.height = 1

    def __str__(self):
        str1 = super().__str__()
        hl = self.left.height if self.left is not None else 0
        hr = self.right.height if self.right is not None else 0
        return '{0}, height = {1:3d}, bal = {2:3d}'.format(str1, self.height,\
                                                           hl-hr)

class AVL(BST):
    def __init__(self, key):
        #super().__init__(key)
        self.root = AVLNode(key)

    @staticmethod
    def height(node):
        if node is None:
            return 0
        return node.height

    @classmethod
    def balance(cls,node):
        if node is None:
            return 0
        return cls.height(node.left) - cls.height(node.right)

    @classmethod
    def r_rotate(cls, ynode):
        '''rotates the subtree rooted at ynode towards right'''
        print('calling r_rot at: ', ynode)
        xnode = ynode.left
        znode = xnode.right
        #rotation
        xnode.right = ynode
        ynode.left = znode
        xnode.parent = ynode.parent
        ynode.parent = xnode
        if znode is not None:
            znode.parent = ynode
        ynode.height = max(cls.height(ynode.left), cls.height(ynode.right))+1
        xnode.height = max(cls.height(xnode.left), cls.height(xnode.right))+1
        return xnode

    @classmethod
    def l_rotate(cls, xnode):
        '''rotates subtree rooted at xnode towards left'''
        print('calling l_rot at: ', xnode)
        ynode = xnode.right
        znode = ynode.left
        #rotation
        ynode.left = xnode
        xnode.right = znode
        if znode is not None:
            znode.parent = xnode
        ynode.parent = xnode.parent
        xnode.parent = ynode
        xnode.height = max(cls.height(xnode.left), cls.height(xnode.right))+1
        ynode.height = max(cls.height(ynode.left), cls.height(ynode.right))+1
        return ynode

    def insert(self, key, tnode):
        #pdb.set_trace()
        if tnode is None:
            return AVLNode(key)
        if key < tnode.key:
            tnode.left = self.insert(key, tnode.left)
            tnode.left.parent = tnode
        else:
            tnode.right = self.insert(key, tnode.right)
            tnode.right.parent = tnode
        tnode.height = max(self.height(tnode.left), self.height(tnode.right))+1
        bal = self.balance(tnode)
        #if node is unbalanace, there are 4 cases
        #left-left case
        if bal > 1 and key < tnode.left.key:
            return self.r_rotate(tnode)
        #left-right case
        if bal > 1 and key > tnode.left.key:
            tnode.left = self.l_rotate(tnode.left)
            return self.r_rotate(tnode)
        #right-right case
        if bal < -1 and key > tnode.right.key:
            return self.l_rotate(tnode)
        #right-left case
        if bal < -1 and key < tnode.right.key:
            tnode.right = self.r_rotate(tnode.right)
            return self.l_rotate(tnode)
        return tnode

    def delete(self, key, tnode):
        if tnode is None:
            return tnode
        if key < tnode.key:
            tnode.left = self.delete(key, tnode.left)
        elif key > tnode.key:
            tnode.right = self.delete(key, tnode.right)
        else:#tnode has the key to be deleted
            if tnode.left is None:
                tnode = self.transplant(tnode, tnode.right)
            elif tnode.right is None:
                tnode = self.transplant(tnode, tnode.left)
            else:#tnode has both children
                minnode = self.tmin(tnode.right)
                if minnode.parent != tnode:
                    tnode.right = self.delete(minnode.key, tnode.right)
                    minnode.right = tnode.right
                    minnode.right.parent = minnode
                left = tnode.left
                tnode = self.transplant(tnode, minnode)
                tnode.left = left
                tnode.left.parent = tnode
        if tnode is None:
            return tnode
        tnode.height = max(self.height(tnode.left), self.height(tnode.right))+1

        #balance if it gets un-balanced
        #4 cases as in insert
        bal = self.balance(tnode)
        lbal = self.balance(tnode.left)
        rbal = self.balance(tnode.right)
        #left-left case
        if bal > 1 and lbal >= 0:
            return self.r_rotate(tnode)
        #left-right case
        if bal > 1 and lbal < 0:
            tnode.left = self.l_rotate(tnode.left)
            return self.r_rotate(tnode)
        #right-right case
        if bal < -1 and rbal <= 0:
            return self.l_rotate(tnode)
        #right-left case
        if bal < -1 and rbal > 0:
            tnode.right = self.r_rotate(tnode.right)
            return self.l_rotate(tnode)
        return tnode

if __name__ == '__main__':
    nlist = [5, 15, 2, 8, 1, 3, 6, 7, 13, 18, 16, 17, 20]
    atree = AVL(10)
    for key in nlist:
        #print('Inserting => ', key)
        atree.root = atree.insert(key, atree.root)
        #print('root: ', atree.root)
    print('\n TREE \n')
    atree.preorder_walk(atree.root)
    for key in [1,2,8,3]:
        print('Deleting => ', key)
        atree.root = atree.delete(key, atree.root)
        print('\n TREE \n')
        atree.preorder_walk(atree.root)
        print(' ################## ')


