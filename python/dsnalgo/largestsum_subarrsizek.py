#!/usr/bin/env python3

# Given an array and a number k, find the largest sum of the subarray
# containing at least k numbers. It may be assumed that the size of array
# is at-least k.


def largestsum_subarrsizek_brute(arr, size):
    # this will generate all subarrays of size = size, size+1, size+2...
    # and find max sum
    maxsum = float('-inf')
    while size <= len(arr):
        for i, num in enumerate(arr[:len(arr) - size + 1]):
            currsum = num
            for num2 in arr[i + 1:i + size]:
                currsum += num2
            maxsum = max(currsum, maxsum)
        size += 1
    return maxsum


def largestsum_sizek_fast(arr, size):
    # 1) We first compute maximum sum till every index and store it in an array maxSum[].
    # 2) After filling the array, we use the sliding window concept of size k.
    # After getting the sum of current window, we add the maxSum of the
    # previous window, if it is greater than current max, then update it else
    # not.

    # we will use kadane's algo to compute maxsum[i] for every index i
    maxsum = []
    currsum = 0
    smax = float('-inf')
    for i, num in enumerate(arr):
        currsum = max(num, currsum + num)
        smax = max(currsum, smax)
        maxsum.append(smax)

    # now compute the sum of first k elem
    sum_k = arr[0]
    for num in arr[1:size]:
        sum_k += num

    # now use the concept of sliding window and compute the sum of window
    # endind at index i
    result = sum_k
    for i, num in enumerate(arr[size:], size):
        sum_k = sum_k + num - arr[i - size]
        result = max(result, sum_k)

        # include the sum uptill i-size also if it increases overall max
        result = max(result, sum_k + maxsum[i - size])

    return result


def main():
    arr1 = [-4, -2, 1, -3]
    k1 = 2
    arr2 = [1, 1, 1, 1, 1, 1]
    k2 = 2
    print(largestsum_subarrsizek_brute(arr1, k1))
    print(largestsum_sizek_fast(arr1, k1))
    print(largestsum_subarrsizek_brute(arr2, k2))
    print(largestsum_sizek_fast(arr2, k2))


if __name__ == '__main__':
    main()
