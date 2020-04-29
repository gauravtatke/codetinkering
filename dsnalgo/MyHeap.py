class MyHeap:
    def __init__(self, alist=None):
        if alist == None:
            self.hitems = []
        else:
            self.hitems = alist
        self.heapsize = len(self.hitems)
        self.build_max_heap()

    def __left(self, indx):
        '''Returns left child index of indx arg'''
        return 2*indx + 1

    def __right(self, indx):
        '''Returns left child index of indx arg'''
        return 2*indx + 2

    def __parent(self, indx):
        '''Returns parent index of arg indx'''
        return indx//2

    def max_heapify(self, key):
        '''Heapifies at index "key". Assumes left and right subtrees are already max heaps'''
        l = self.__left(key)
        r = self.__right(key)
        if l <= self.heapsize-1 and self.hitems[l] > self.hitems[key]:
            largest = l
        else:
            largest = key
        if r <= self.heapsize-1 and self.hitems[r] > self.hitems[largest]:
            largest = r

        if largest != key:
            self.hitems[key], self.hitems[largest] = self.hitems[largest], self.hitems[key]
            self.max_heapify(largest)

    def build_max_heap(self):
        for key in range(self.heapsize//2, -1, -1):
            self.max_heapify(key)

    def __str__(self):
        return str(self.hitems)

    def pop(self):
        if self.heapsize < 1:
            raise Exception
        maxelem = self.hitems[0]
        self.hitems[0] = self.hitems.pop()
        self.heapsize -= 1
        self.max_heapify(0)
        return maxelem

    def insert(self, key):
        self.hitems.append(key)
        self.heapsize += 1
        indx = self.heapsize-1
        while indx > 0 and self.hitems[self.__parent(indx)] < key:
            self.hitems[indx] = self.hitems[self.__parent(indx)]
            indx = self.__parent(indx)
        self.hitems[indx] = key

    def delete(self, indx):
        if indx == self.heapsize - 1:#last node right most
            self.hitems.pop()
            self.heapsize -= 1
        else:
            #self.hitems[indx] = self.hitems[self.heapsize-1]
            self.hitems[indx] = self.hitems.pop()
            self.heapsize -= 1
            self.max_heapify(self.hitems[indx])

if __name__ == '__main__':
    alist = [27, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0]
    emplist = []
    myheap = MyHeap(alist)
    print('Heap : ', myheap)
    for i in range(5):
        print("Popping: ", myheap.pop())
        print(myheap)
    print(myheap.heapsize)
    for i in (43,23):
        print("Inserting: ", i)
        myheap.insert(i)
        print("Heap : ", myheap)
    print('Heapsize = ', myheap.heapsize)
    for i in (3, 7):
        print('deleting ', i)
        myheap.delete(i)
        print(myheap)

