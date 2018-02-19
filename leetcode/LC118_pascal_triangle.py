# Given numRows, generate the first numRows of Pascal's triangle.
# For example, given numRows = 5,
# Return

# [
#      [1],
#     [1,1],
#    [1,2,1],
#   [1,3,3,1],
#  [1,4,6,4,1]
# ]


class Solution(object):
    def generate(self, numRows):
        """
        :type numRows: int
        :rtype: List[List[int]]
        """
        triangle = []
        for row in range(numRows):
            singlerow = []
            for col in range(row + 1):
                if col == 0 or col == row:
                    singlerow.append(1)
                else:
                    val = triangle[row - 1][col - 1] + triangle[row - 1][col]
                    singlerow.append(val)
            triangle.append(singlerow)
        return triangle