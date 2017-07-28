#!/usr/bin/env python3

# Given an input stream of n characters consisting only of small case
# alphabets the task is to find the first non repeating character each
# time a character is inserted to the stream.


class DllNode:
    # doubly linked list node
    def __init__(self, key=None, nextp=None, prevp=None):
        self.key = key
        self.nextp = nextp
        self.prevp = prevp


class DLL:
    def __init__(self, head=None, tail=None):
        self.head = head
        self.tail = tail

    def insert(self, node, nextp=None, prevp=None):
        # insert at tail
        if self.tail:
            self.tail.nextp = node
            node.prevp = self.tail
            self.tail = node
        else:
            self.head = node
            self.tail = self.head

    def remove(self, node):
        if self.head == node:
            self.head = self.head.nextp
            if self.head:
                self.head.prevp = None
        elif self.tail == node:
            self.tail = self.tail.prevp
            self.tail.nextp = None
        else:
            node.prevp.nextp = node.nextp
            node.nextp.prevp = node.prevp

    def peek(self):
        # peek at the head
        return self.head.key if self.head else None


def printlist(head):
    while head:
        print(head.key, end='->')
        head = head.nextp
    print(None)


def findFirstNRC(arr):
    # since char are at max 256, we can declare 2 list with 256 size acc to ascii
    # inDLL stores the pointer to location in DLL
    # repeated stores bool if repeated 2 or more times
    inDLL = [None for i in range(256)]
    repeated = [0 for i in range(256)]
    dll = DLL()
    for ch in arr:
        if not repeated[ord(ch)]:
            if not inDLL[ord(ch)]:
                # ch coming for first time
                temp = DllNode(ch)
                dll.insert(temp)
                inDLL[ord(ch)] = temp
            else:
                # coming for second time
                # remove from DLL
                dll.remove(inDLL[ord(ch)])
                inDLL[ord(ch)] = None
                repeated[ord(ch)] = True
        print(dll.peek())


def main(argv):
    findFirstNRC(argv[0])
    return 0


if __name__ == '__main__':
    import sys
    sys.exit(main(sys.argv[1:]))
