#!/usr/bin/env python3

# Given an unsorted array of integers, find a subarray which adds to a given number. If there are more than one subarrays with sum as the given number, print any of them.

def findsubarray_brute(arr, reqsum):
    for i, num in enumerate(arr):
        currsum = num
        for j, num2 in enumerate(arr[i+1:], i+1):
            currsum = currsum + num2
            if currsum == reqsum:
                return (i, j)
    return None


def findsubarray_usingmap(arr, reqsum):
    # we can use hashmap to speedup the process
    # maintain sum of elements encountered so far in map as key & val as index upto which sum is taken
    # if curr_sum - reqsum is present as a key in hashmap then we have a subarray with req sum
    # otherwise we put curr_sum in hash map. after iteration if we don't have any subarray then return None
    sumdict = {}
    curr_sum = 0
    for i, num in enumerate(arr):
        curr_sum = curr_sum + num
        if curr_sum == reqsum:
            # 0 to i sums up to reqsum
            return (0, i)
        elif sumdict.get(curr_sum - reqsum, None):
            # means (0, i) sums upto curr_sum
            # (0, dict[curr_sum-reqsum]) sums upto (curr_sum-reqsum)
            # i.e reqsum is from (dict[curr_sum-reqsum]+1, i)
            return (sumdict[curr_sum - reqsum]+1, i)
        else:
            # insert curr_sum
            sumdict[curr_sum] = i
    return None



def main():
    arr1 = [1, 4, 20, 3, 10, 5]
    sum1 = 33
    arr2 = [10, 2, -2, -20, 10]
    sum2 = -10
    arr3 = [-10, 0, 2, -2, -20, 10]
    sum3 = 20
    print(findsubarray_usingmap(arr1, sum1))
    print(findsubarray_usingmap(arr2, sum2))
    print(findsubarray_usingmap(arr3, sum3))


if __name__ == '__main__':
    main()
