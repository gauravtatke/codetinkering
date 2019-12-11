#Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.
#
#Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
#
#Example 1:
#
#Given nums = [1,1,2],
#
#Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
#
#It doesn't matter what you leave beyond the returned length.
#
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        return self.removeDuplicates_sol2(nums)

    def removeDuplicates_sol1(self, nums: List[int]) -> int:
        if nums == None or len(nums) == 0:
            return 0
        unik = nums[0]
        i = 1
        while i < len(nums):
            if nums[i] == unik:
                # remove ith number and don't increment i
                nums.pop(i)
            else:
                unik = nums[i]
                i += 1
        return len(nums)

    def removeDuplicates_sol2(self, nums: List[int]) -> int:
        if nums == None or len(nums) == 0:
            return 0
        next_uniq_loc = 1
        for curr_loc in range(1, len(nums)):
            if nums[curr_loc] != nums[curr_loc-1]:
                # means nums[curr_loc] is next uniq element
                # should be copied to next_uniq_loc
                nums[next_uniq_loc] = nums[curr_loc]
                next_uniq_loc += 1
        # next_uniq_loc is the length of the uniq array
        # we don't need to remove other elements
        return next_uniq_loc
