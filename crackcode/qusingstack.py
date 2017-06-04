#!/usr/bin/env python3

#implementing Queue using 2 stacks

from listnstack import Stack

class MyQueue():
    def __init__(self, *args):
        self.qstack = Stack(100)
        self.aux = Stack(100)
        if args:
            for item in args:
                self.qstack.push(item)

    def enqueue(self, item):
        if len(self.aux):
            while len(self.aux):
                self.qstack.push(self.aux.pop())
        self.qstack.push(item)

    def dequeue(self):
        if len(self.aux):
            return self.aux.pop()
        while len(self.qstack):
            self.aux.push(self.qstack.pop())
        return self.aux.pop()

    def __str__(self):
        if len(self.aux):
            while len(self.aux):
                self.qstack.push(self.aux.pop())
        return str(self.qstack)

if __name__ == '__main__':
    mq = MyQueue()
    for i in range(10):
        mq.enqueue(i)
    print(mq)
    mq.enqueue(11)
    mq.enqueue(12)
    print(mq)
    mq.dequeue()
    mq.dequeue()
    print(mq)
    mq.enqueue(17)
    mq.dequeue()
    print(mq)








