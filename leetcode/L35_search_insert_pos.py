#Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
#
#You may assume no duplicates in the array.

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        return self.searchInsert_sol1(nums, target)

    def searchInsert_sol1(self, nums: List[int], target: int) -> int:
        retIndex = len(nums)
        for i, num in enumerate(nums):
            if num >= target:
                retIndex = i
                break
        return retIndex
