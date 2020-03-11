# Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
# (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
# You are given a target value to search. If found in the array return its index, otherwise return -1.
# You may assume no duplicate exists in the array.
# Your algorithm's runtime complexity must be in the order of O(log n).

# Example 1:

# Input: nums = [4,5,6,7,0,1,2], target = 0
# Output: 4

# Example 2:

# Input: nums = [4,5,6,7,0,1,2], target = 3
# Output: -1


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        if not len(nums):
            return -1
        return self.search_binary(nums, 0, len(nums)-1, target)
    
    def search_binary(self, nums: List[int], left: int, right: int, target: int) -> int:
        if left == right:
            # single element list
            if target == nums[left]:
                return left
            else:
                return -1
        else:
            if nums[left] <= nums[right]:
                # list is sorted
                if target < nums[left] or target > nums[right]:
                    return -1
                mid = (left + right)//2
                if target <= nums[mid]:
                    return self.search_binary(nums, left, mid, target)
                else: 
                    return self.search_binary(nums, mid+1, right, target)
            else:
                mid = (left + right)//2
                left_result = self.search_binary(nums, left, mid, target)
                right_result = self.search_binary(nums, mid+1, right, target)
                if left_result == -1:
                    return right_result
                else:
                    return left_result