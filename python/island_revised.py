#!/usr/bin/env python3

import sys

class UnionFind:
    #this is quickfind algo. `Find` is fast but union is very slow
    def __init__(self, size):
        #assume 2-d _iday is one big 1-d _iday.
        #each cell's (row,col) representative in matrix is stored at location
        #self._id[col*row + col]
        #p and q are connected if _id[p] == _id[q]
        #id         0   1   2   3   4   5   6   7   8   9
        #_id[id]    0   1   2   6   4   6   6   9   8   9
        #in above, 3,5,6 are connected, 7 and 9 are connected
        self._id = [None for i in range(size)]

    def find(self, elem):
        return self._id[elem]

    def union(self, elem1, elem2):
        parent = self.find(elem1)
        for index, elem in enumerate(self._id):
            if elem == parent:
                self._id[index] = self._id[elem2]

    def setVal(self, elem, value):
        self._id[elem] = value

    def getUnique(self):
        return len(set(self._id))-1  #Ignoring 'None' entry in set


class WQUPC(UnionFind):
    #This is Weighted Quick Union with Path Compression data structure
    def __init__(self, size):
        #_id[i] is parent of i
        #root of i is _id[_id[_id[_id[i] keep going until it doesnt change]]]
        super().__init__(size)
        self._sz = [0 for i in range(size)] #_sz[i] _iday holds num of elements
                                          #in tree rooted at i
    
    def setVal(self, elem, val):
        super().setVal(elem, val)
        self._sz[elem] = 1

    def find(self, elem):
        #returns root and in process flattens the tree
        while elem != self._id[elem]:
            #for path compression, set each root of each visited idx with its 
            #grandfather. This keeps tree near flat
            self._id[elem] = self._id[self._id[elem]]
            elem = self._id[elem]
        return elem

    def union(self, elem1, elem2):
        #union(p,q)-> set id of p's root to id of q's root or vice versa
        root1 = self.find(elem1)
        root2 = self.find(elem2)
        #merge smaller tree into larger one
        if self._sz[root1] > self._sz[root2]:
            self._id[root2] = root1
            self._sz[root1] += self._sz[root2]
        else:
            self._id[root1] = root2
            self._sz[root2] += self._sz[root1]

    def getUnique(self):
        return len(set([self.find(i) for i in self._id if i is not None])) 

def getIslands(M, i, j, uf):
    nCol = len(M[0])
    nRow = len(M)
    cell_num = i*nCol + j
    #uf = UnionFind(nRow*nCol)
    uf.setVal(cell_num, cell_num)
    M[i][j] = 1
    if i-1 >= 0 and M[i-1][j]: #if cell above is not island
        a_cell = (i-1)*nCol + j
        if uf.find(a_cell) != uf.find(cell_num):
            uf.union(a_cell, cell_num)
    if i+1 < nRow and M[i+1][j]: #if cell below is not island
        a_cell = nCol*(i+1) + j
        if uf.find(a_cell) != uf.find(cell_num):
            uf.union(a_cell, cell_num)
    if j-1 >= 0 and M[i][j-1]:
        a_cell = nCol*i + j-1
        if uf.find(a_cell) != uf.find(cell_num):
            uf.union(a_cell, cell_num)
    if j+1 < nCol and M[i][j+1]:
        a_cell = nCol*i + j+1
        if uf.find(a_cell) != uf.find(cell_num):
            uf.union(a_cell, cell_num)
    return uf.getUnique()

def printMatrix(matrix):
    for row in matrix:
        for col in row:
            print(col, end='\t')
        print()

def main(argv):
    nrow = 6
    ncol = 5
    uf = UnionFind(nrow*ncol)
    wqupc = WQUPC(nrow*ncol)
    matrix = [[0 for col in range(ncol)] for row in range(nrow)]
    cord = [(0,0), (0,4), (5,0), (5,4), (1,1), (1,3), (4,1), (4,3), (2,2),
            (3,2), (0,1), (0,2), (3,4), (4,2), (1,2)]
    # cord = [(0,0), (0,2), (1,0), (0,1), (2,2)]
    for i, j in cord:
        print('input = ({0},{1}), continents = '.format(i,j),
              getIslands(matrix, i, j, wqupc))
        # print('union find = ', wqupc._id)
        printMatrix(matrix)
        print()

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
