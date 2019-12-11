#Given an array nums and a value val, remove all instances of that value in-place and return the new length.
#
#Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
#
#The order of elements can be changed. It doesn't matter what you leave beyond the new length.
#
#Example 1:
#
#Given nums = [3,2,2,3], val = 3,
#
#Your function should return length = 2, with the first two elements of nums being 2.
#
#It doesn't matter what you leave beyond the returned length.

class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        return self.removeElement_sol3(nums, val)
    
    def removeElement_sol1(self, nums: List[int], val: int) -> int:
        if len(nums) == 0:
            return 0
        i = 0
        while i < len(nums):
            if nums[i] == val:
                nums.pop(i)
            else:
                i += 1
        return len(nums)
    
    def removeElement_sol2(self, nums: List[int], val: int) -> int:
        if len(nums) == 0:
            return 0
        i = 0
        while i < len(nums):
            if val == nums[i]:
                nums[i] = nums[-1] # copy the last element
                # then discard the last element
                nums.pop()
            else:
                i += 1
        return len(nums)
    
    def removeElement_sol3(self, nums: List[int], val: int) -> int:
        if len(nums) == 0:
            return 0
        next_uniq_loc = 0
        i = 0
        while i < len(nums):
            if nums[i] != val:
                nums[next_uniq_loc] = nums[i]
                next_uniq_loc += 1
            i += 1
        return next_uniq_loc
