# You are given an integer array nums and you have to return a new counts array.
# The counts array has the property where counts[i] is the number of smaller elements to the right of nums[i].
#
# Example:
#
# Given nums = [5, 2, 6, 1]
#
# To the right of 5 there are 2 smaller elements (2 and 1).
# To the right of 2 there is only 1 smaller element (1).
# To the right of 6 there is 1 smaller element (1).
# To the right of 1 there is 0 smaller element.
#
# Return the array [2, 1, 1, 0].


def countSmaller(nums):
    # index_map = {num: ind for ind, num in enumerate(nums)}
    cntArr = [0 for i in nums]
    matrix = list(enumerate(nums))
    mergeSort(matrix, cntArr, 0, len(nums) - 1)
    return cntArr


def mergeSort(arr, cnt, start, end):
    if start >= end:
        return
    mid = (start + end) // 2
    mergeSort(arr, cnt, start, mid)
    mergeSort(arr, cnt, mid + 1, end)
    merge(arr, cnt, start, mid, end)


def merge(arr, cnt, start, mid, end):
    # we keep rightcount as count of elements moving from right to sorted
    # array. When element from right moves, increment rightcount by 1. When
    # elem moves from left then increment cnt[left] by rightcount
    left = [arr[i] for i in range(start, mid + 1)]
    right = [arr[j] for j in range(mid + 1, end + 1)]
    li = ri = 0
    indx = start
    rightcount = 0

    while li < len(left) and ri < len(right):
        if right[ri][1] < left[li][1]:
            rightcount += 1
            arr[indx] = right[ri]
            ri += 1
        else:
            # update count for left item as rightcount
            cnt[left[li][0]] += rightcount
            arr[indx] = left[li]
            li += 1
        indx += 1
    while li < len(left):
        arr[indx] = left[li]
        cnt[left[li][0]] += rightcount
        li = li + 1
        indx = indx + 1
    while ri < len(right):
        arr[indx] = right[ri]
        # no need to increment rightcount. there is nothing left in left arr
        indx = indx + 1
        ri = ri + 1


class TreeNode:
    def __init__(self, val, s):
        self.val = val
        self.left = None
        self.right = None
        # every node will contain sum of all number of nodes to its left and
        # number of its duplicates
        self.dup = 1
        self.sum = s


def countSmaller_UsingBST(nums):
    nlen = len(nums) - 1
    cntArr = [0 for i in nums]
    root = None
    while nlen >= 0:
        root = insertNode(cntArr, root, nums[nlen], nlen, 0)
        nlen = nlen - 1
    return cntArr


def insertNode(arr, node, num, indx, preSum):
    if node is None:
        node = TreeNode(num, 0)
        arr[indx] = preSum
    elif node.val == num:
        # duplicate node
        node.dup += 1
        arr[indx] = node.sum + preSum
    elif node.val > num:
        # num is left node
        node.left = insertNode(arr, node.left, num, indx, preSum)
        node.sum += 1
    else:
        # num is right. Whereever node takes right turn, its sum becomes sum+dup of
        # all the nodes where we turned right in inserting
        node.right = insertNode(arr, node.right, num,
                                indx, preSum + node.dup + node.sum)
    return node


def main():
    n = [5, 2, 6, 1]
    print(countSmaller_UsingBST(n))


if __name__ == '__main__':
    main()
