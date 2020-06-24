// You are given an n x n 2D matrix representing an image. Rotate the image by 90 degrees (clockwise).
// Note: You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
// Example 1:
// Given input matrix = 
// [
//   [1,2,3],
//   [4,5,6],
//   [7,8,9]
// ],

// rotate the input matrix in-place such that it becomes:
// [
//   [7,4,1],
//   [8,5,2],
//   [9,6,3]
// ]

// Example 2:
// Given input matrix =
// [
//   [ 5, 1, 9,11],
//   [ 2, 4, 8,10],
//   [13, 3, 6, 7],
//   [15,14,12,16]
// ], 

// rotate the input matrix in-place such that it becomes:
// [
//   [15,13, 2, 5],
//   [14, 3, 4, 1],
//   [12, 6, 8, 9],
//   [16, 7,10,11]
// ]

public class SolutionLC48RotateImageMatrix {
    public void rotate(int[][] matrix) {
        int nRow = matrix.length;
        int nCol = matrix[0].length;
        int row = 0;
        int col = 0;
        int endRow = nRow-1;
        int endCol = nCol-1;
        // printMatrix(matrix);
        while (row < nRow/2 && col < nCol/2) {
            rotateNinety(matrix, row, col, endRow, endCol);
            // System.out.println("after rotation");
            // printMatrix(matrix);
            // now shrink the matrix to cover inner layer
            row++;
            col++;
            endRow--;
            endCol--;
        }
        // printMatrix(matrix);
    }

    public void rotateNinety(int[][] matrix, int startRow, int startCol, int endRow, int endCol) {
        // this will just rotate the matrix row, col to one place. this is only outer layer rotation
        int row = startRow;
        int col = startCol;
        int elementToMove = matrix[row][col];
        int rotate90Count = endCol-startCol; // count to rotate everything by 90 degree
        int count = (endRow-startRow) * 4; // this is number of cells in matrix perimeter.
        // count is also the number of times we need to copy val from one cell to another
        int temp;
        while (rotate90Count > 0) {
            // this loop will rotate matrix by 90 degree
            // to rorate 90 degree, we need to rotate n-1 times
            int i = 0;
            elementToMove = matrix[row][col];
            while (i < count) {
                // this will just rotate perimeter by one cell such that whole perimeter
                // is shifted one place
                if (row == startRow && col < endCol) {
                    // move to right
                    temp = matrix[row][col+1]; // copy the next element
                    matrix[row][col+1] = elementToMove; // copy to right
                    elementToMove = temp; // store temp value for next interation
                    col++; 
                } else if (row == endRow && col > startCol) {
                    // elements move to left
                    temp = matrix[row][col-1];
                    matrix[row][col-1] = elementToMove;
                    elementToMove = temp;
                    col--;
                } else if (col == startCol && row > startRow) {
                    // elements move up
                    temp = matrix[row-1][col];
                    matrix[row-1][col] = elementToMove;
                    elementToMove = temp;
                    row--;
                } else if (col == endCol && row < endRow) {
                    // elements move down
                    temp = matrix[row+1][col];
                    matrix[row+1][col] = elementToMove;
                    elementToMove = temp;
                    row++;
                }
                // System.out.println("count: " + i + "After one rotation: ");
                i++; // one movement done
                // printMatrix(matrix);
            }
            // System.out.println(row + ", " + col);
            rotate90Count--;
        }
    }

    public void printMatrix(int[][] matrix) {
        for (int[] rows: matrix) {
            for (int num: rows) {
                System.out.print(num + ", ");
            }
            System.out.println();
        }
        System.out.println();
    }

    public static void main(String[] args) {
        SolutionLC48RotateImageMatrix sol = new SolutionLC48RotateImageMatrix();
        int[][] matrix = {
            {7, 6, 8},
            {9, 4, 3},
            {1, 5, 2},
        };
        sol.rotate(matrix);
    }
}