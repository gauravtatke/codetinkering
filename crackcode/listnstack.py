#!/usr/bin/env python3
import pdb

class slnode():
    def __init__(self, key=None):
        self.key = key
        self.nxt = None

    def __str__(self):
        s = '{0}'.format(self.key)
        return s

class sllist():
    def __init__(self, *keys):
        #self.slist = []
        self.head = None
        self.ln = 0
        if keys:
            self.insert(*keys)

    def insert(self, *keys):
        indx = 0
        if self.head is None:
            hd = slnode(keys[0])
            hd.nxt = None
            self.head = hd
            self.ln += 1
            indx += 1
        for key in keys[indx:]:
            node = slnode(key)
            node.nxt = self.head
            self.head = node
            self.ln += 1

    def deletebykey(self, key):
        if self.head is not None and self.head.key == key:
            self.head = self.head.nxt
        else:
            for node in self:
                if node.nxt is not None and node.nxt.key == key:
                    node.nxt = node.nxt.nxt
                    break

    def deletebyloc(self, dnode):
        if self.head == dnode:
            self.head = self.head.nxt
        else:
            for node in self:
                if node.nxt is not None and node.nxt == dnode:
                    node.nxt = dnode.nxt

    def search(self, key):
        curr = self.head
        while curr is not None:
            if curr.key == key:
                return curr
            curr = curr.nxt

    def __iter__(self):
        return sllistiter(self)

    def __len__(self):
        return self.ln

    def __getitem__(self, sliced):
        items = []
        try:
            start, stop, step = sliced.indices(self.ln)
        except AttributeError:
            #means 'int' is passed instead of 'slice' obj
            step = 1 if sliced >= 0 else -1
            start = sliced
            stop = sliced+1 if sliced >= 0 else sliced-1

        if (start > stop and step > 0) or (start < stop and step < 0) \
           or (start == stop):
            return []
        
        curr = self.head
        backwards = True
        if start > stop:
            #we have to convert start and stop such that start < stop, as this
            #is singly linked list and we cannot traverse backwards
            step = abs(step)
            start, stop = stop + step, start+1
            backwards = False
        
        j = start
        while start > 0:
            curr = curr.nxt
            start -= 1
        while j < stop:
            items.append(curr.key)
            for k in range(step):
                if curr is not None:
                    curr = curr.nxt
                j += 1
        if backwards:
            #we have to reverse the list cuz we will make return likedlist
            #instead of simple list. LinkedList inserts at head so items will
            #be reversed
            #reverse order
            items.reverse()
        return sllist(*items) 

class sllistiter():
    '''
    This class only serves as purpose of iteration so that sllist can be
    iterated over simultaneosly
    '''
    def __init__(self, slobj):
        self.currobj = slobj.head

    def __next__(self):
        if self.currobj is not None:
            cnode = self.currobj
            self.currobj = self.currobj.nxt
            return cnode
        else:
            raise StopIteration()

def printlist(ll):
    curr = ll.head
    while curr is not None:
        print(curr, end='>')
        curr = curr.nxt
    print(None)


class Stack():
    def __init__(self, threshold):
        self.sarray = []
        self.threshold = threshold

    def push(self, val):
        if len(self.sarray) >= self.threshold:
            raise IndexError
        self.sarray.append(val)

    def pop(self, remtop=True):
        if remtop:
            return self.sarray.pop()
        else:
            return self.sarray.pop(0)

    def peek(self):
        return self.sarray[-1]

    def __str__(self):
        st = ' '.join(str(i) for i in self.sarray)
        return st

    def __len__(self):
        return len(self.sarray)

if __name__ == '__main__':
    l1 = sllist(2,3,6,4,7,9)
    printlist(l1)
    #for node in l2:
    #    print(node, end='>')
    #print(None)

