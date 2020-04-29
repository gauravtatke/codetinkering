from typing import List

class Stack:
    def __init__(self):
        self._stack = [None]
        self.top = 0
        
    def push(self, val):
        self._stack.append(val)
        self.top = self.top+1
    
    def pop(self):
        if self.top:
            ret_val = self._stack.pop()
            self.top = self.top-1
            return ret_val    
        else:
            return None
    
    def is_empty(self):
        if self.top:
            return False
        return True
    
    def peek(self):
        if self.top:
            return self._stack[self.top]
    
    def __str__(self):
        return self._stack.__str__()
    
        
class Solution:
    def nextGreaterElements(self, nums: List[int]) -> List[int]:
        return self.nextGreaterElements_sol2(nums)
    
    def nextGreaterElements_sol1(self, nums: List[int]) -> List[int]:
        # using bruteforce
        length = len(nums)
        result = []
        for i, num in enumerate(nums):
            modulo_index = (i+1) % length
            while modulo_index != i:
                if nums[modulo_index] > num:
                    result.append(nums[modulo_index])
                    break
                else:
                    modulo_index = (modulo_index+1) % length
            if modulo_index == i:
                result.append(-1)
        return result
    
    def nextGreaterElements_sol2(self, nums: List[int]) -> List[int]:
        # using stack. Consider that stack is not circular
        # to get next greater element, start from right to left
        # insert index of element if element is less than top of stack
        # top of stack element is the next greater element for current element
        # insert top of stack in result list for next greater element of ith elem
        # if stack is empty result for ith is -1. insert ith element index on top
        # if current ith element is greater than top then pop stack until top
        # of stack becomes higher than ith element. then insert i at top
        # for e.g. after 1st pass result of list = [3, 8, 4, 1, 2] -> [8, -1, -1, 2, -1]
        # as you can see, since array is circular, result for 4 should be 8 and 2 should 8
        # so we need to do another pass of above to correct the result because 1st pass only
        # considers items to right of i but i could be at left as well
        my_stack = Stack()
        result = [-1 for i in range(len(nums))]
        # run 2 pass
        for i in range(2):
            for indx in range(len(nums)-1, -1, -1):
                while my_stack.is_empty() is not True and nums[my_stack.peek()] <= nums[indx]:
                    # stack is not empty and top of stack is less than current element
                    # pop it
                    my_stack.pop()
                if my_stack.is_empty():
                    # result for this index is -1
                    result[indx] = -1
                else:
                    result[indx] = nums[my_stack.peek()]
                my_stack.push(indx)
        return result
                            
if __name__ == '__main__':
    ret_val = Solution().nextGreaterElements([1, 2, 1])
    print(ret_val)