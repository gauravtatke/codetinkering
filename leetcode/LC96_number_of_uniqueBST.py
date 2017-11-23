# Given n, how many structurally unique BST's (binary search trees) that store values 1...n?
# For example,
# Given n = 3, there are a total of 5 unique BST's.


def numTrees(n):
    if not n:
        return 1
    # ntree[i] stores number of unique BST for n = i
    ntree = [0 for i in range(n + 1)]
    ntree[0] = 1  # None is only node
    ntree[1] = 1  # Root is 1, no left no right
    for num in range(2, n + 1):
        # for n nodes, at any given root i, there will be i-1 nodes in left
        # subtree and n-i nodes in right subtree. num of left subtree ->
        # ntree[i-1] and num of right subtree is ntree[n-i]. ntree[n] will be
        # cartesian product of ntree[i-1] and ntree[n-i]
        for root in range(1, num + 1):
            ntree[num] += (ntree[root - 1] * ntree[num - root])
    return ntree[n]


def main():
    print(numTrees(4))


if __name__ == '__main__':
    main()
