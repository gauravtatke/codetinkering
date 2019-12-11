#Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
#
#Note:
#
#    The number of elements initialized in nums1 and nums2 are m and n respectively.
#    You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
#
#Example:
#
#Input:
#nums1 = [1,2,3,0,0,0], m = 3
#nums2 = [2,5,6],       n = 3
#
#Output: [1,2,2,3,5,6]

class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        self.merge_sol2(nums1, m, nums2, n)

    def merge_sol1(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        index1 = index2 = 0
        while index2 < n:
            while index1 < m and nums1[index1] < nums2[index2]:
                index1 += 1
            if index1 == m:
                # index1 reached end, copy nums2 to nums1 end as it is
                while index1 < len(nums1):
                    nums1[index1] = nums2[index2]
                    index1 += 1
                    index2 += 1
            else:
                # insert position found
                # first shift the elements to right
                i = m
                while i >= index1:
                    nums1[i] = nums1[i-1]
                    i = i-1
                m = m+1 # because last position in nums1 is increased now
                # once shifted to right, copy from nums2 to nums1
                nums1[index1] = nums2[index2]
                index2 += 1
                index1 += 1

    def merge_sol2(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        index1 = index2 = 0
        while index1 < m and index2 < n:
            if nums1[index1] < nums2[index2]:
                index1 += 1
                continue
            else:
                # insert location found
                # instead of moving one by one, we can insert at location
                # and pop one element from last
                nums1.insert(index1, nums2[index2])
                nums1.pop()
                m += 1
                index1 += 1
                index2 += 1
        # out of loop. If index1 >= m then
        # copy all remaining nums2 after nums1
        if index1 >= m:
            while index1 < len(nums1):
                nums1[index1] = nums2[index2]
                index1 += 1
                index2 += 1
