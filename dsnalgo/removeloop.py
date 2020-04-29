#!/usr/bin/env python3

# Given a linked list, remove the loop in it if present. The task is to
# complete the function removeTheLoop which takes only one argument the
# head of the linked list . The function removes the loop in the linked
# list if present.


class SllNode:
    def __init__(self, val=None, np=None):
        self.key = val
        self.np = np

    def __str__(self):
        return str(self.key)


def createlistWithLoop(alist, loopat):
    head = None
    tail = None
    for item in alist:
        if tail:
            tail.np = SllNode(item)
            tail = tail.np
        else:
            head = tail = SllNode(item)

    # to make a loop we point tail to some element in the list. element val is
    # given in loopat variable
    tail.np = searchlist(head, loopat)
    return head


def searchlist(head, val):
    curr = head
    while curr and curr.key != val:
        curr = curr.np

    if curr:
        return curr
    else:
        return None


def printlist(head):
    curr = head
    while curr:
        print(curr, end='->')
        curr = curr.np
    print(None)


def removeloop(head):
    curr = head
    result = False
    count = 0
    while curr:
        run = head
        while run.np != curr.np:
            run = run.np
        # when it comes out loop, means either there is lopp at curr.np because
        # run.np and curr.np points to same node. or run and curr are same node
        if run != curr:
            # loop at curr.np
            curr.np = None
            result = True
        elif run == curr and curr == head:
            # loop can happen at head too
            count += 1
            if count > 1:
                # means it is circular list, break it at this location
                head = curr.np
                curr.np = None
                result = True
        curr = curr.np
    return head, result


def detectloop_usinghash(head):
    # in method removeloop function, loop is detected by traversing the list
    # multiple times -> O(n^2). We can use hash to check whether loop is present
    curr = head
    visited = {}
    while curr.np and not visited.get(curr.np, False):
        visited[curr] = True
        curr = curr.np
    if not curr.np:
        # no loop
        return False, None
    else:
        # there is loop. curr.np is pointing to already visited node.
        # return true and curr which is the last node where loop starts.
        return True, curr


def detectloop_usingfloydcycle(head):
    # we floyd's cycle algorithm to detect loop
    # have a slow and fast pointers. Fast moves 2 step and slow moves 1
    # if fast meets slow then there is a loop. if they dont meet and fast
    # reaches None then there is no loop. This is fastest method to detect a
    # loop
    slow = head
    fast = head
    result = False
    while fast and fast.np:
        fast = fast.np.np
        slow = slow.np
        if fast == slow:
            # loop exists
            result = True
            break

    # if the node where loop starts has to be returned we have to do
    # additional steps
    if result:
        # start with head and try to reach head from fast pointer. If it is
        # reachable then we get the node where loop starts. If is doesn't then
        # move to head.np and so forth and repeat the process.
        curr = head
        while True:
            run = fast
            while run.np != curr and run.np != fast:
                run = run.np
            if run.np == curr:
                # loop starting node is run
                return result, run
            else:
                # move curr to next and repeat the process
                curr = curr.np
    else:
        return result, None


def removeloop_usinghash(head):
    result, node = detectloop_usinghash(head)
    if result:
        node.np = None
    return head, result


def removeloop_usingfloydcycle(head):
    result, node = detectloop_usingfloydcycle(head)
    if result:
        node.np = None
    return head, result


def main():
    arr = [1, 2, 3, 4, 5, 6]
    head = createlistWithLoop(arr, 6)
    head, result = removeloop_usinghash(head)
    print(result)
    printlist(head)


if __name__ == '__main__':
    main()
