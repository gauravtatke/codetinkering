# Follow up for "Unique Paths":
# Now consider if some obstacles are added to the grids. How many unique paths would there be?
# An obstacle and empty space is marked as 1 and 0 respectively in the grid.
# For example,
# There is one obstacle in the middle of a 3x3 grid as illustrated below.
# [
#   [0,0,0],
#   [0,1,0],
#   [0,0,0]
# ]
# The total number of unique paths is 2.


class Solution1(object):
    def uniquePathsWithObstacles(self, obstacleGrid):
        """
        :type obstacleGrid: List[List[int]]
        :rtype: int
        """
        path = [[0 for col in row] for row in obstacleGrid]
        nrow = len(obstacleGrid)
        ncol = len(obstacleGrid[0])
        for row in range(nrow):
            if not obstacleGrid[row][0]:
                path[row][0] = 1
            else:
                # if there is an obstacle in the first column itself, we cannot
                # go beyond that point so path won't be updated beyond that
                break
        for col in range(ncol):
            if not obstacleGrid[0][col]:
                path[0][col] = 1
            else:
                # if there is an obstacle in first row itself then we cannot go
                # past that so path will not be updated for those cells
                break

        for row in range(1, nrow):
            for col in range(1, ncol):
                if not obstacleGrid[row][col]:
                    path[row][col] = path[row - 1][col] + path[row][col - 1]

        return path[nrow - 1][ncol - 1]


class Solution2(object):
    def uniquePathsWithObstacles(self, obstacleGrid):
        """
        :type obstacleGrid: List[List[int]]
        :rtype: int
        """
        # we can put similar optimization as in LC62 so save space
        if not obstacleGrid:
            return 0
        path = [0 for i in obstacleGrid[0]]
        nrow, ncol = len(obstacleGrid), len(obstacleGrid[0])

        path[0] = 1
        for row in range(nrow):
            for col in range(ncol):
                if obstacleGrid[row][col]:
                    path[col] = 0
                elif col > 0:
                    path[col] = path[col - 1] + path[col]
        return path[ncol - 1]
