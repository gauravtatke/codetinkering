# A robot is located at the top-left corner of a m x n grid (marked 'Start' in
# the diagram below).
# The robot can only move either down or right at any point in time. The robot
# is trying to reach the bottom-right corner of the grid (marked 'Finish' in
# the diagram below). How many possible unique paths are there?


class Solution1(object):
    # brute force approach
    def uniquePaths(self, m, n):
        """
        :type m: int
        :type n: int
        :rtype: int
        """
        return self.uniquePathBrute(0, 0, m - 1, n - 1)

    def uniquePathBrute(self, i, j, m, n):
        # start from bottom right cell and recursively check how many unique
        # ways robot can reach grid[m][n]
        if i + 1 == m or j + 1 == n:
            # from i,j if next cell is bottom right then only 1 unique way
            return 1

        distance = 0
        if i + 1 > m:
            distance = self.uniquePathBrute(i, j + 1, m, n)
        elif j + 1 > n:
            distance = self.uniquePathBrute(i + 1, j, m, n)
        else:
            distance = self.uniquePathBrute(
                i + 1, j, m, n) + self.uniquePathBrute(i, j + 1, m, n)
        return distance


class Solution2(object):
    # memoized recursive approach
    def uniquePaths(self, m, n):
        """
        :type m: int
        :type n: int
        :rtype: int
        """
        path = [[0 for col in range(n)] for row in range(m)]
        for row in range(m):
            path[row][n - 1] = 1  # last column. Only 1 way to reach bottom right
        for col in range(n):
            path[m - 1][col] = 1  # last row, only way to reach bottom

        return self.uniquePathMemoized(0, 0, m - 1, n - 1, path)

    def uniquePathMemoized(self, i, j, m, n, path):
        if path[i][j]:
            return path[i][j]

        path[i][j] = self.uniquePathMemoized(
            i + 1, j, m, n, path) + self.uniquePathMemoized(i, j + 1, m, n, path)

        return path[i][j]


class Solution3(object):
    # memoized and iterative approach
    def uniquePaths(self, m, n):
        """
        :type m: int
        :type n: int
        :rtype: int
        """
        path = [[0 for col in range(n)] for row in range(m)]
        for row in range(m):
            path[row][0] = 1  # first column
        for col in range(n):
            path[0][col] = 1  # firt row, only way to reach bottom

        for row in range(1, m):
            for col in range(1, n):
                path[row][col] = path[row - 1][col] + path[row][col - 1]

        return path[m - 1][n - 1]


class Solution4(object):
    # memoized and iterative approach
    def uniquePaths(self, m, n):
        """
        :type m: int
        :type n: int
        :rtype: int
        """
        # In above approach, we can see that we only need one row and one
        # previous column to calculate current cell value. So one row can be
        # stored in path and prev column can be store in a variable.
        # this will cut down memory footprint
        path = [1 for col in range(n)]
        for row in range(1, m):
            prev_col = 1
            for col in range(1, n):
                val = prev_col + path[col]
                path[col] = val
                prev_col = val

        # we can see that prev is just path[col-1] so we can just write
        # for row in range(1, m):
        #     for col in range(1, n):
        #         path[col] = path[col - 1] + path[col]

        return path[n - 1]
