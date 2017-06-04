#!/usr/bin/env python3

import pdb
from listnstack import Stack

STACK_CAP = 5

class SetOfStack():
    def __init__(self):
        self.sos = []

    def push(self, val):
        if self.sos == []:
            self.sos.append(Stack(STACK_CAP))
        try:
            self.sos[-1].push(val)
        except IndexError:
            #means threshold is crossed
            #new stack is needed
            self.sos.append(Stack(STACK_CAP))
            self.sos[-1].push(val)

    def pop(self):
        retval = None
        if self.sos == []:
            return None
        last = self.sos[-1]
        retval = last.pop()
        if len(last) == 0:
            #last stack is empty so we need to remove the stack
            __ = self.sos.pop() #we don't care about the popped item
        return retval

    def popAt(self, stnum):
        return self.leftshift(stnum, True)

    def leftshift(self, stacknum, remtop=True):
        stack = self.sos[stacknum]
        if remtop:
            retval = stack.pop()
        else:
            retval = stack.pop(False)

        if len(stack) == 0: #Stack is empty, remove it
            __ = self.sos.pop(stacknum)
        else:
            if len(self)-1 > stacknum: #still have another stack
                stack.push(self.leftshift(stacknum+1, False))

        return retval


    def __str__(self):
        sosstr = ''
        for stackno, stackitem in enumerate(self.sos):
            sosstr = sosstr +'Stack {0}: {1}\n'.format(stackno, str(stackitem))
        return sosstr

    def __len__(self):
        return len(self.sos)


if __name__ == '__main__':
    sos = SetOfStack()
    for i in range(20):
        sos.push(i)
    print(sos)
    sos.popAt(2)
    print('#### after popAt ####\n', sos)
    sos.popAt(3)
    print('#### after popAt ####\n', sos)
