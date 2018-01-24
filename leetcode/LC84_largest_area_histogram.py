# Given n non-negative integers representing the histogram's bar height
# where the width of each bar is 1, find the area of largest rectangle in
# the histogram.

# For example,
# Given heights = [2,1,5,6,2,3],
# return 10.


class Solution1(object):
    # for every bar 'x' if we calculate the area with 'x' as smallest bar and
    # then find the max of all those areas then we'll find largest area in the
    # histogram.
    def largestRectangleArea(self, heights):
        """
        :type heights: List[int]
        :rtype: int
        """
        # to calc the area with 'x' as smallest bar, we need index of first
        # smaller bar to its right and left. Difference between left_index and
        # right_index is num of bars with atleast 'x' height.
        stack = []
        max_area = 0
        area_at_top = 0
        i = 0
        while i < len(heights):
            # for each bar h[i] do the following
            if not stack or heights[i] > heights[stack[-1]]:  # step 1
                # if stack is empty or h[i] is greater than height of bar at
                # stack top, then push the index i to stack. Only increment i
                # when i is pushed to stack
                stack.append(i)
                i += 1
            else:  # step 2
                # if h[i] is smaller then pop index from stack. for each popped
                # out index, calc the area with popped out bar smallest one.
                # Keep popping until stack top is smaller than smaller than h[i]
                top = stack.pop()
                num_bars = i if not stack else i - stack[-1] - 1
                area_at_top = num_bars * heights[top]
                max_area = max(max_area, area_at_top)
        while stack:
            # if stack is not empty, pop out index and repeat step 2 for each
            # popped out bar
            top = stack.pop()
            num_bars = i if not stack else i - stack[-1] - 1
            area_at_top = num_bars * heights[top]
            max_area = max(max_area, area_at_top)
        return max_area
