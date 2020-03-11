class Solution:
    def maxArea(self, height: List[int]) -> int:
        return self.maxArea_TwoPointer(height)
    
    
    def maxArea_brute(self, height: List[int]) -> int:
        # brute force approach, calc all area
        # exceeds time limit
        max_area = 0
        for x, height1 in enumerate(height):
            distance_in_lines = 0
            for height2 in height[x+1:]:
                distance_in_lines += 1
                min_height = min(height1, height2)
                area = min_height * distance_in_lines
                max_area = max(area, max_area)
        return max_area
                
        
    def maxArea_TwoPointer(self, height: List[int]) -> int:
        # area is limited by shorter line. Also, farther the lines are larger will be
        # the area. Start with farthest lines and move smaller line inward.
        near_end = 0
        far_end = len(height)-1
        max_area = 0
        while near_end < far_end:        
            area = (far_end - near_end) * min(height[near_end], height[far_end])
            max_area = max(max_area, area)
            if height[near_end] <= height[far_end]:
                near_end += 1
            else:
                far_end -= 1
        return max_area