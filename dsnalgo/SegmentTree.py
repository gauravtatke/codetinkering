# leaf nodes are elements of array. Each internal node represent some kind
# of merging. Merging depends on the kind of problem. For e.g. it may
# store min in range (i, j) or sum of range (i,j)

import math


class SegTreeSumRange:
    def __init__(self, arr):
        self.arr = arr
        height = math.ceil(math.log2(len(arr)))
        max_nodes = pow(2, height + 1) - 1
        self.stree = [0 for i in range(max_nodes)]
        self.constructTree(0, len(arr) - 1, 0)

    def constructTree(self, ststart, stend, stindx):
        '''
        ststart, stend := start and end index of range for which segment tree is created
        stindex := index of current node in segment tree
        '''
        # for every node at index i, left child is at 2*i+1 and right child =
        # 2*i+2
        # print('stindx = {}, ststart = {}, stend = {}'.format(stindx, ststart, stend))
        if ststart == stend:
            self.stree[stindx] = self.arr[ststart]
            return self.stree[stindx]
        mid = (ststart + stend) // 2
        # below construct depends of the problem
        self.stree[stindx] = self.constructTree(
            ststart, mid, 2 * stindx + 1) + self.constructTree(mid + 1, stend, 2 * stindx + 2)
        return self.stree[stindx]

    def getVal(self, qstart, qend):
        '''
        qstart := start of query range
        qend := end of query range
        '''
        if qstart < 0 or qend >= len(self.arr) or qstart > qend:
            raise Exception
        return self.getSumUtil(0, len(self.arr) - 1, qstart, qend, 0)

    def getSumUtil(self, ststart, stend, qstart, qend, stindx):
        # if segment node is part of qstart, qend then return it
        if qstart <= ststart and qend >= stend:
            return self.stree[stindx]
        # if segment of node at stindex is outside of range then return 0
        if qstart > stend or qend < ststart:
            return 0
        # if part of the segment overlaps with the range then,
        mid = (ststart + stend) // 2
        return self.getSumUtil(ststart, mid, qstart, qend, 2 * stindx + 1) + \
            self.getSumUtil(mid + 1, stend, qstart, qend, 2 * stindx + 2)

    def updateValue(self, indx, newval):
        if indx < 0 or indx >= len(self.arr):
            raise Exception

        diff = newval - self.arr[indx]
        self.arr[indx] = newval
        self.updateValueUtil(0, len(self.arr) - 1, indx, diff, 0)

    def updateValueUtil(self, ststart, stend, indx, diff, stindx):
        if indx < ststart or indx > stend:
            # indx in not in range, do nothing and return
            return
        # indx lies in range
        self.stree[stindx] += diff

        if ststart != stend:
            # update left and right subtree accordingly if range is more than 1
            mid = (ststart + stend) // 2
            self.updateValueUtil(ststart, mid, indx, diff, 2 * stindx + 1)
            self.updateValueUtil(mid + 1, stend, indx, diff, 2 * stindx + 2)


class SegTreeMinInRange:
    def __init__(self, arr):
        self.arr = arr
        height = math.ceil(math.log2(len(arr)))
        max_nodes = pow(2, height + 1) - 1
        self.stree = [0 for i in range(max_nodes)]
        self.constructTree(0, len(arr) - 1, 0)

    def constructTree(self, ststart, stend, stindx):
        if ststart == stend:
            self.stree[stindx] = self.arr[ststart]
            return self.stree[stindx]
        mid = (ststart + stend) // 2
        leftmin = self.constructTree(ststart, mid, 2 * stindx + 1)
        rightmin = self.constructTree(mid + 1, stend, 2 * stindx + 2)
        if leftmin < rightmin:
            self.stree[stindx] = leftmin
        else:
            self.stree[stindx] = rightmin
        return self.stree[stindx]

    def getVal(self, qstart, qend):
        if qstart < 0 or qend >= len(self.arr) or qstart > qend:
            raise Exception
        return self.getMinUtil(0, len(self.arr) - 1, qstart, qend, 0)

    def getMinUtil(self, ststart, stend, qstart, qend, stindx):
        if qstart > stend or qend < ststart:
            # non overlapping
            return float('inf')
        if qstart <= ststart and qend >= stend:
            return self.stree[stindx]
        mid = (ststart + stend) // 2
        leftmin = self.getMinUtil(ststart, mid, qstart, qend, 2 * stindx + 1)
        rightmin = self.getMinUtil(mid + 1, stend, qstart, qend, 2 * stindx + 2)
        return leftmin if leftmin < rightmin else rightmin


def main():
    arr = [2, 8, 7, 4, 9, 10, 3, 1]
    st = SegTreeMinInRange(arr)
    print(st.getVal(1, 7))


if __name__ == '__main__':
    main()
