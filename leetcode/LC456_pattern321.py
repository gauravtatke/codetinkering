# Given a sequence of n integers a1, a2, ..., an, a 132 pattern is a subsequence ai, aj, ak such that i < j < k and ai < ak < aj.
# Design an algorithm that takes a list of n numbers as input and checks whether there is a 132 pattern in the list.
# Note: n will be less than 15,000.

# Example 1:
# Input: [1, 2, 3, 4]
# Output: False
# Explanation: There is no 132 pattern in the sequence.

# Example 2:
# Input: [3, 1, 4, 2]
# Output: True
# Explanation: There is a 132 pattern in the sequence: [1, 4, 2].

# Example 3:
# Input: [-1, 3, 2, 0]
# Output: True
# Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].

class Solution:
    def find132pattern(self, nums: List[int]) -> bool:
        return self.find132pattern_smarter_bruteforce(nums)
    
    def find132pattern_bruteforce(self, nums: List[int]) -> bool:
        # consider all triplets and find
        if len(nums) < 3:
            return False
        for i in range(len(nums)-2):
            for j in range(i+1, len(nums)-1):
                for k in range(j+1, len(nums)):
                    if nums[k] > nums[i] and nums[k] < nums[j]:
                        return True
        return False
    
    def find132pattern_smarter_bruteforce(self, nums: List[int]) -> bool:
        # chances of finding num[k] such that its between num[i] & num[j]
        # increase if the range of i and j is large. So instead of finding i,j, k
        # we can just find num[j] and num[k] but keep a minimum num[i] for
        # each num[j]. In other words for each j, store num[i] which is min in range
        # 0 to j
        if len(nums) < 3:
            return False
        min_i = nums[0]
        for j in range(1, len(nums)-1):
            min_i = min(min_i, nums[j])
            for k in range(j+1, len(nums)):
                if nums[k] < nums[j] and nums[k] > min_i:
                    return True
        return False