#!/usr/bin/env python3

# Given a Linked List where every node represents a linked list and contains two pointers of its type:
# (i) a next pointer to the next node
# (ii) a bottom pointer to a linked list where this node is head.
#
# You have to flatten the linked list to a single linked list which is sorted.
#
# For Ex: Shown below is a given linked list
#
#  5 -> 10 -> 19 -> 28
#  |    |     |     |
#  V    V     V     V
#  7    20    22    35
#  |          |     |
#  V          V     V
#  8          50    40
#  |                |
#  V                V
#  30               45
#
# and after flattening it, the sorted linked list looks like:
#
#  5->7->8->10->19->20->22->28->30->35->40->45->50

import heapq
from functools import total_ordering


@total_ordering
class SllNode:
    def __init__(self, key=None, nextp=None, bottomp=None):
        self.key = key
        self.np = nextp  # next pointer
        self.bp = bottomp  # bottom pointer

    def __str__(self):
        retstr = 'key=' + str(self.key)
        retstr += ', next=' + (str(self.np.key) if self.np else str(None))
        retstr += ', bottom=' + (str(self.bp.key) if self.bp else str(None))
        return retstr

    def __cmp__(self, other):
        return cmp(self.key, other.key)

    def __lt__(self, other):
        if isinstance(other, self.__class__):
            return self.key < other.key
        return NotImplemented

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.key == other.key
        return NotImplemented


def searchlist(node, key):
    nlist = [node] if node else []
    for item in nlist:
        if item.key == key:
            return item
        if item.np:
            nlist.append(item.np)
        if item.bp:
            nlist.append(item.bp)
    return None


def createlist(klist):
    head = None
    for key, pos, isnp in klist:
        if head:
            node = searchlist(head, pos)
            if node and isnp:
                # means this is supposed to be nextp to pos
                node.np = SllNode(key)
            elif node and not isnp:
                # means bottomp
                node.bp = SllNode(key)
            else:
                # node is none. continue and ignore
                continue
        else:
            head = SllNode(key)
    return head


def printlist(head):
    nlist = [head] if head else []
    for item in nlist:
        print(item, end=' / ')
        if item.np:
            nlist.append(item.np)
        if item.bp:
            nlist.append(item.bp)
    print()


def mergelist(node1, node2):
    head = None
    tail = None
    if not node1 and node2:
        # node1 is None and node2 is valid pointer
        return node2
    if node1 and not node2:
        return node1

    while node1 and node2:
        if node1.key < node2.key:
            if tail:
                tail.np = node1
                tail = tail.np
            else:
                head = tail = node1
            node1 = node1.np
        else:
            if tail:
                tail.np = node2
                tail = tail.np
            else:
                head = tail = node2
            node2 = node2.np

    if node1:
        tail.np = node1
    else:
        tail.np = node2

    # print('## After Merge ##')
    # printlist(head)
    return head


def flattenlist_recursive(head):
    if not head:
        return None
    if not head.np and not head.bp:
        # node does not have next or bottom pointer
        # make bottom ptr as None and return it
        head.bp = None
        return head
    nextp = flattenlist_recursive(head.np)
    bottomp = flattenlist_recursive(head.bp)
    head.np = mergelist(nextp, bottomp)
    head.bp = None
    return head


def flattenlist_usingpriorityq(head):
    # since this is more like a tree structure, traverse by BFS and at each
    # layer push all elements to priority q and take out the minimum
    heap = []
    heapq.heappush(heap, head)
    head = tail = None
    while len(heap):
        count = len(heap)
        while count:
            node = heapq.heappop(heap)
            if node.np:
                heapq.heappush(heap, node.np)
            if node.bp:
                heapq.heappush(heap, node.bp)
            # insert the node in linked list and make node.bp = None
            node.bp = None
            if tail:
                tail.np = node
                tail = tail.np
            else:
                head = tail = node
            count -= 1
    return head


def main():
    klist = [(5, None, None), (15, 5, True), (8, 5, False), (25, 15, True),
             (17, 15, False), (9, 8, True), (20, 8, False), (21, 9, True),
             (18, 17, True), (30, 17, False), (32, 25, False), (34, 32, False)]

    head = createlist(klist)
    # printlist(head)
    head = flattenlist_usingpriorityq(head)
    printlist(head)


if __name__ == '__main__':
    main()
