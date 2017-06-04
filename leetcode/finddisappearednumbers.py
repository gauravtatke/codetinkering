# Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some
# elements appear twice and others appear once.
#
# Find all the elements of [1, n] inclusive that do not appear in this array.
#
# Could you do it without extra space and in O(n) runtime? You may assume the
# returned list does not count as extra space.
#
# Example:
# Input:
# [4,3,2,7,8,2,3,1]
#
# Output:
# [5,6]

def findDisappearedNumbers(nums):
    return [i for i in range(1, len(nums)+1) if i not in nums]

def findDisappearedNumbers2(nums):
    numset = set(nums)
    totalset = set([i for i in range(1, len(nums)+1)])
    totalset -= numset
    return list(totalset)

def findDisappearedNumbers3(nums):
    # just mark the position by incrementing the values by len(nums)
    # so wherever the value is less than len(nums), then it the number
    n = len(nums)
    for i in range(n):
        nums[(nums[i]-1) % n] += n
    return [i+1 for i in range(n) if nums[i] <= n]

def main():
    print(findDisappearedNumbers3([4,1,1,1]))

if __name__ == '__main__':
    main()
