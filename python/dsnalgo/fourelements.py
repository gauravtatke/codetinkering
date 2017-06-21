#!/usr/bin/env python3

# Given an array of integers, find a combination of four elements in the array whose sum is equal to a given value X.

def fourelements_brute(arr, sum):
    # brute force approach. generate all combinations
    # O(n4) complexity
    n = len(arr)
    for i, num1 in enumerate(arr[0:n-3]):
        for j, num2 in enumerate(arr[i+1:n-2], i+1):
            for k, num3 in enumerate(arr[j+1:n-1], j+1):
                for l, num4 in enumerate(arr[k+1:n], k+1):
                    if num1+num2+num3+num4 == sum:
                        return (num1, num2, num3, num4)
    return None


def fourelements_using1lessloop(arr, reqsum):
    # we can reduce one loop from above bruteforce approach by sorting the array
    # fix 2 elements and then do a linear search to find other 2 elems
    # O(nlogn + n3) -> O(n3) complexity
    arr.sort()
    n = len(arr)
    for i, num1 in enumerate(arr[0:n-3]):
        for j, num2 in enumerate(arr[i+1:n-2], i+1):
            curr_reqsum = reqsum - (num1+num2)
            start = j+1
            end = n-1
            while start < end:
                if arr[start] + arr[end] == curr_reqsum:
                    return (num1, num2, arr[start], arr[end])
                elif arr[start] + arr[end] < curr_reqsum:
                    start += 1
                else:
                    end -= 1
    return None
    

def fourelements_usinghashmap(arr, reqsum):
    # we can reduce one loop from brute force
    # no need to sort but we need aux memory for hash
    # fix 2 elem and then use hash map to find remaining 2 elements
    n = len(arr)
    sumdict = {}
    for i, num1 in enumerate(arr[0:n-3]):
        for j, num2 in enumerate(arr[i+1:n-2], i+1):
            curr_reqsum = reqsum - (num1+num2)
            for num3 in arr[j+1:]:
                if sumdict.get(curr_reqsum-num3, None):
                    return (num1, num2, num3, curr_reqsum-num3)
                else:
                    sumdict[num3] = True
    return None



def main():
    arr1 = [1, 5, 1, 0, 6, 0]
    reqsum = 12
    #print(fourelements_brute(arr1, reqsum))
    #print(fourelements_using1lessloop(arr1, reqsum))
    print(fourelements_usinghashmap(arr1, reqsum))


if __name__ == '__main__':
    main()
