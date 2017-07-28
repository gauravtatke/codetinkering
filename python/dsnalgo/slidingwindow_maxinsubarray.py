#!/usr/bin/env python3

# given an array and an integer k, find the maximum for each and every
# contiguous subarray of size k.

from collections import deque


def findmaxinsubarray_brute(arr, size):
    # complexity is O(n*k) where k is size of the subarray
    for i, num in enumerate(arr[:len(arr) - size + 1]):
        maxitem = num
        j = 1
        while j < size:
            maxitem = max(maxitem, arr[i + j])
            j += 1
        print(maxitem, end=' ')
    print('\n')


def findmaxinsubarr_slidingwindow(arr, size):
    # we can use a sliding window deque
    # We create a Dequeue, Qi of capacity k, that stores only useful elements
    # of current window of k elements. An element is useful if it is in
    # current window and is greater than all other elements on left side of it
    # in current window. We process all array elements one by one and maintain
    # Qi to contain useful elements of current window and these useful
    # elements are maintained in sorted order. The element at front of the Qi
    # is the largest and element at rear of Qi is the smallest of current
    # window.
    dq = deque(maxlen=size)a
    for i, num in enumerate(arr[:size]):
        # this will make window of 0 to size-1 elements
        while len(dq) and num >= arr[dq[-1]]:
            # means new elements is greater than last element in dq
            # remove last element from dq
            dq.pop()
        dq.append(i)

    for i, num in enumerate(arr[size:], size):
        # elem at the front of the dq will the largest for previous window
        # print it
        print(arr[dq[0]], end=' ')

        # remove the elem which are out of this window
        # remember that window is ending that i means that
        # window is [i, i-1, i-2 .... i-size+1]
        while len(dq) and dq[0] <= i - size:
            dq.popleft()

        # remove all the elements' index from the back which are smaller than
        # num
        while len(dq) and arr[dq[-1]] < num:
            dq.pop()

        # now push the element index in the dq
        dq.append(i)

    # print the max for the last window
    print(arr[dq[0]])


def main():
    arr1 = [12, 1, 78, 90, 57, 89, 56]
    findmaxinsubarray_brute(arr1, 3)
    findmaxinsubarr_slidingwindow(arr1, 3)


if __name__ == '__main__':
    main()
