# Design and implement a data structure for Least Recently Used (LRU) cache. It should support the following operations: get and put.

# get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
# put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.

# Follow up:
# Could you do both operations in O(1) time complexity?

# Example:

# LRUCache cache = new LRUCache( 2 /* capacity */ );

# cache.put(1, 1);
# cache.put(2, 2);
# cache.get(1);       // returns 1
# cache.put(3, 3);    // evicts key 2
# cache.get(2);       // returns -1 (not found)
# cache.put(4, 4);    // evicts key 1
# cache.get(1);       // returns -1 (not found)
# cache.get(3);       // returns 3
# cache.get(4);       // returns 4


class DllNode:
    def __init__(self, key=None, value=None):
        self.key = key
        self.value = value
        self.prev = None
        self.nxt = None

    def __str__(self):
        return 'key = {}, value = {}'.format(self.key, self.value)


class LRUCache:
    def __init__(self, capacity):
        """
        :type capacity: int
        """
        self.capacity = capacity
        self._head = DllNode()  # making dummy nodes for head & tail
        self._tail = DllNode()
        self._head.nxt = self._tail
        self._tail.prev = self._head
        self._cache = {}

    def _add(self, node):
        # add node after head
        node.nxt = self._head.nxt
        self._head.nxt.prev = node
        self._head.nxt = node
        node.prev = self._head

    def _remove(self, node):
        nxtnode = node.nxt
        prevnode = node.prev
        nxtnode.prev = prevnode
        prevnode.nxt = nxtnode
        return node

    def _evict(self):
        remnode = self._tail.prev
        self._remove(remnode)
        return remnode

    def get(self, key):
        """
        :type key: int
        :rtype: int
        """
        node = self._cache.get(key, None)
        if node is None:
            return -1
        retval = node.value
        # now move this node to head
        # any node after head is most recently used and least recently used is before tail
        self._add(self._remove(node))
        return retval

    def put(self, key, value):
        """
        :type key: int
        :type value: int
        :rtype: void
        """
        node = self._cache.get(key, None)
        # print('putting key = {}, val = {}'.format(key, value))
        if node is None:
            node = DllNode(key, value)
            self._add(node)
            # print('head => ', self._head)
            # print('head next => ', self._head.nxt)
            self._cache[key] = node
            # print('after putting = ', self._cache)
            if len(self._cache) > self.capacity:
                evictnode = self._evict()
                del self._cache[evictnode.key]
        else:
            node.value = value
            # move this node to head
            self._add(self._remove(node))


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)


def main():
    obj = LRUCache(2)
    obj.put(1, 1)
    obj.put(2, 2)
    print(obj.get(1))
    obj.put(3, 3)
    print(obj._cache)
    print(obj.get(2))
    obj.put(4, 4)
    print(obj._cache)
    print(obj.get(1))
    print(obj._cache)
    print(obj.get(3))
    print(obj._cache)
    print(obj.get(4))


if __name__ == '__main__':
    main()