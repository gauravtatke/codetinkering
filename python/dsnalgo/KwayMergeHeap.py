'''Solution to excercise 6.5-9 (clrs)'''

class KwayHeap:
    def __init__(self, alist):
        self.aList = alist
        self.kHeap = []
        self.heapsize = 0
        for indx, elem in enumerate(self.aList):
            first = elem.pop(0) #pop the first item out of every sublist
            self.insert_min_heap((indx, first)) #inserting first item and index of each list in heap

    @staticmethod
    def left(index):
        return 2*index+1

    @staticmethod
    def right(index):
        return 2*index+2

    @staticmethod
    def parent(index):
        return index//2

    def extract_min(self):
        min_el = self.kHeap[0]
        self.kHeap[0] = self.kHeap.pop()
        self.heapsize -= 1
        self.min_heapify(0)
        return min_el

    def insert_min_heap(self, key):
        self.heapsize += 1
        self.kHeap.append(key)
        idx = self.heapsize-1 #index of last element
        while idx > 0 and self.kHeap[KwayHeap.parent(idx)][1] > key[1]:
            self.kHeap[idx] = self.kHeap[KwayHeap.parent(idx)]
            idx = KwayHeap.parent(idx)
        self.kHeap[idx] = key

    def min_heapify(self, index):
        l = KwayHeap.left(index)
        r = KwayHeap.right(index)
        if l <= self.heapsize-1 and \
                self.kHeap[l][1] <= self.kHeap[index][1]:
            small = l
        else:
            small = index
        #print('small {0}, heap {1}'.format(small, self.kHeap))
        if r <= self.heapsize-1 and \
                self.kHeap[r][1] <= self.kHeap[small][1]:
            small = r
        if small != index:
            self.kHeap[index], self.kHeap[small] = \
                    self.kHeap[small], self.kHeap[index]
            self.min_heapify(small)

    def __str__(self):
        return str(self.kHeap)

    def __len__(self):
        return len(self.kHeap)

def getnext(heap, index):
    next_el = heap.aList[index].pop(0)
    return index, next_el

def kwaymergesort(aList):
    hp = KwayHeap(aList)
    res = []
    no_items = [el for sublist in aList for el in sublist]
    for _ in no_items:
        idx, num = hp.extract_min()
        res.append(num) #adding smallest number
        if hp.aList[idx]:
            hp.insert_min_heap(getnext(hp, idx))
    print(res)


if __name__ == '__main__':
    aList = [
            [5, 6, 7],
            [1, 10, 12, 13],
            [3, 4, 17, 18, 19],
            [2, 8, 11, 15, 21]
            ]
    kwaymergesort(aList)
