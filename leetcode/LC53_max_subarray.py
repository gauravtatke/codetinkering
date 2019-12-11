#Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
#
#Example:
#
#Input: [-2,1,-3,4,-1,2,1,-5,4],
#Output: 6
#Explanation: [4,-1,2,1] has the largest sum = 6.

class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        return self.maxSubArray_sol1(nums)

    def maxSubArray_sol1(self, nums: List[int]) -> int:
        curr_max = max_sum = nums[0]
        for num in nums[1:]:
            curr_max = max(num, curr_max+num)
            max_sum = max(curr_max, max_sum)
        return max_sum
