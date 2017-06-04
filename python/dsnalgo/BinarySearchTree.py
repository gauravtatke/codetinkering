import pdb
from TreeNode import BinTreeNode
from BinaryTree import BinaryTree

class BSTNode(BinTreeNode):
    def __init__(self, key):
        super().__init__(key)
        self.parent = None

    def __str__(self):
        str2 = super().__str__()
        pt = str(None) if self.parent is None else self.parent.key
        str3 = '{0}, parent = {1:3}'.format(str2, pt)
        return str3

class BST(BinaryTree):
    def __init__(self, key):
        self.root = BSTNode(key)
        #dont wanna call insert in init, let it only update values
        #if key is not None:
        #    self.insert_iter(key)

    def insert_iter(self, key):
        x = self.root
        y = None
        node = BSTNode(key)
        while x is not None:
            y = x
            if node.key < x.key:
                x = x.left
            else:
                x = x.right
        node.parent = y
        if y is None:
            #tree was empty
            self.root = node
        elif node.key < y.key:
            y.left = node
        else:
            y.right = node
        #returning root. its unnecessary but keeping it to make iter and rec
        #fucntion similar
        #return self.root

    def insert_rec(self, key, tnode):
        if tnode is None:
            return BSTNode(key)
        if key < tnode.key:
            tnode.left = self.insert_rec(key, tnode.left)
            tnode.left.parent = tnode
        else:
            tnode.right = self.insert_rec(key, tnode.right)
            tnode.right.parent = tnode
        return tnode

    def insert(self, key, tnode=None, flag='iter'):
        '''
        flag dictates whether iterative or recursive function is called
        only valid values are 'iter' and 'rec'
        '''
        if tnode is None:
            tnode = self.root
        if flag == 'iter':
            return self.insert_iter(key)
        elif flag == 'rec':
            #print('recursive')
            return self.insert_rec(key, tnode)
        else:
            raise Exception

    def search_iter(self, key, tnode):
        while tnode is not None and tnode.key != key:
            if key < tnode.key:
                tnode = tnode.left
            else:
                tnode = tnode.right
        return tnode

    def search_rec(self, key, tnode):
        if tnode is not None or tnode.key == key:
            return tnode
        if key < tnode.key:
            return self.search_rec(key, tnode.left)
        else:
            return self.search_rec(key, tnode.right)

    def search(self, key, tnode=None, flag='iter'):
        '''
        flag dictates whether iterative or recursive function is called
        only valid values are 'iter' and 'rec'
        '''
        if tnode is None:
            tnode = self.root
        if flag == 'iter':
            return self.search_iter(key, tnode)
        elif flag == 'rec':
            return self.search_rec(key, tnode)
        else:
            raise Exception

    def tmin(self, tnode):
        '''implements TREE-tminIMUM'''
        while tnode.left is not None:
            tnode = tnode.left
        return tnode

    def tmax(self, tnode):
        while tnode.right is not None:
            tnode = tnode.right
        return tnode

    def successor(self, tnode):
        '''returns smallest node having key greater than tnode'''
        if tnode.right is not None:
            return self.tmin(self, tnode.right)
        p = tnode.parent
        while p is not None and tnode == p.right:
            tnode = p
            p = p.parent
        return p

    def predecessor(self, tnode):
        '''returns largets node having key smaller than tnode'''
        if tnode.left is not None:
            return self.tmax(self, tnode.left)
        p = tnode.parent
        while p is not None and tnode == p.left:
            tnode = p
            p = p.parent
        return p

    def transplant(self, unode, vnode):
        '''replaces unode with vnode'''
        #pdb.set_trace()
        if unode.parent is None:
            self.root = vnode
        elif unode == unode.parent.left:
            unode.parent.left = vnode
        else:
            unode.parent.right = vnode

        if vnode is not None:
            vnode.parent = unode.parent
        return vnode

    def delete_iter(self, key):
        tnode = self.search(key)
        if tnode is None:
            print("key does not exist")
            return
        if tnode.left is None:
            __ = self.transplant(tnode, tnode.right)
        elif tnode.right is None:
            __ = self.transplant(tnode, tnode.left)
        else:
            ynode = self.tmin(tnode.right)
            if ynode.parent != tnode:
                __ = self.transplant(ynode, ynode.right)
                ynode.right = tnode.right
                ynode.right.parent = ynode
            __ = self.transplant(tnode, ynode)
            ynode.left = tnode.left
            ynode.left.parent = ynode

    def delete_rec(self, key, tnode):
            if tnode is None:
                return tnode
            print(tnode)
            if key < tnode.key:
                tnode.left = self.delete_rec(key, tnode.left)
                #tnode.left.parent = tnode
            elif key > tnode.key:
                tnode.right = self.delete_rec(key, tnode.right)
                #tnode.right.parent = tnode
            else:
                #tnode is to be deleted
                if tnode.left is None:
                    tnode = self.transplant(tnode, tnode.right)
                elif tnode.right is None:
                    tnode = self.transplant(tnode, tnode.left)
                else:
                    tminnode = self.tmin(tnode.right)
                    if tminnode.parent != tnode:
                        tnode.right = self.delete_rec(tminnode.key, tnode.right)
                        tminnode.right = tnode.right
                        tminnode.right.parent = tminnode
                    left = tnode.left
                    tnode = self.transplant(tnode, tminnode)
                    tnode.left = left
                    tnode.left.parent = tnode
            return tnode

    def delete(self, key, tnode=None, flag='iter'):
        '''
        flag dictates whether iterative or recursive function is called
        only valid values are 'iter' and 'rec'
        '''
        if tnode is None:
            tnode = self.root
        if flag == 'iter':
            return self.delete_iter(key)
        elif flag == 'rec':
            return self.delete_rec(key, tnode)
        else:
            raise Exception

    def __str__(self):
        pass

if __name__ == '__main__':
    nlist = [5, 15, 2, 8, 1, 3, 6, 7, 13, 18, 16, 17, 20]
    tree = BST(10)
    #print(tree.root)
    for key in nlist:
        tree.insert(key, flag='rec')
    print(tree.root)
    tree.preorder_walk(tree.root)
    #print("searching ", tree.search(10))
    #tree.delete(8, flag='rec')
    #tree.delete(6, flag='rec')
    #tree.delete(10, flag='rec')
    #print('\n\n###### AFTER DELETE ######\n\n')
    #tree.preorder_walk(tree.root)
