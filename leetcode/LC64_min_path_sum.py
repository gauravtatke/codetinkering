# Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
# Note: You can only move either down or right at any point in time.
# Example 1:
#
# [[1,3,1],
#  [1,5,1],
#  [4,2,1]]
#
# Given the above grid map, return 7. Because the path 1→3→1→1→1 minimizes
# the sum.


class Solution1(object):
    def minPathSum(self, grid):
        """
        :type grid: List[List[int]]
        :rtype: int
        """
        nrow = len(grid)
        ncol = len(grid[0])
        path = [[0 for col in row] for row in grid]
        path[0][0] = grid[0][0]

        for row in range(1, nrow):
            path[row][0] = grid[row][0] + path[row - 1][0]
        for col in range(1, ncol):
            path[0][col] = grid[0][col] + path[0][col - 1]

        for row in range(1, nrow):
            for col in range(1, ncol):
                path[row][col] = grid[row][col] + \
                    min(path[row - 1][col], path[row][col - 1])

        return path[nrow - 1][ncol - 1]


class Solution2(object):
    # we can put optimization similar to LC62 and have only single row list
    # for path
    def minPathSum(self, grid):
        """
        :type grid: List[List[int]]
        :rtype: int
        """
        nrow = len(grid)
        ncol = len(grid[0])
        path = [grid[0][col] for col in range(ncol)]
        for col in range(1, ncol):
            path[col] += path[col - 1]

        for row in range(1, nrow):
            path[0] = grid[row][0] + path[0]
            for col in range(1, ncol):
                path[col] = grid[row][col] + min(path[col - 1], path[col])

        return path[ncol - 1]
