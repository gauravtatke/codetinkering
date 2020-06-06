/* Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.

Note: You can only move either down or right at any point in time.

Example:

Input:
[
  [1,3,1],
  [1,5,1],
  [4,2,1]
]
Output: 7
Explanation: Because the path 1→3→1→1→1 minimizes the sum.
 */
class Solution_LC64_MinPathSum {
    public int minPathSum(int[][] grid) {
        int row = grid.length;
        int col = grid[0].length;
        return minPathSumRec(grid, row-1, col-1);
    }
    
    public int minPathSumRec(int[][] grid, int row,  int col) {
        System.out.println("row=" + row + " col=" + col);
        if (row < 0 || col < 0) {
            return Integer.MAX_VALUE;
        } else if (col == 0 && row == 0) {
            return grid[row][col];
        }
        int mpath = grid[row][col] + Math.min(minPathSumRec(grid, row, col-1), minPathSumRec(grid, row-1, col));
        System.out.println("Mini path for row " + row + " & col " + col + " is " + mpath);
        return mpath;
    }

    public static void main(String[] args) {
        int[][] grid = {
            {1, 3, 1},
            {1, 5, 1},
            {4, 2, 1},
        };

        Solution_LC64_MinPathSum sol = new Solution_LC64_MinPathSum();
        System.out.println(sol.minPathSum(grid));
    }
}