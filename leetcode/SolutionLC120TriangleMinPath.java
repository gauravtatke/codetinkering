// Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.

// For example, given the following triangle

// [
//      [2],
//     [3,4],
//    [6,5,7],
//   [4,1,8,3]
// ]



import java.util.*;

class SolutionLC120TriangleMinPath {
    
    public int minimumTotal(List<List<Integer>> triangle) {
        // return minFrom(triangle, 0, 0);
        HashMap<String, Integer> mem = new HashMap<String, Integer>();
        return minFromMemoized(triangle, 0, 0, mem);
    }
    
    public int minFrom(List<List<Integer>> triangle, int row, int col) {
        // time limit exceeding because for some rows, col we are calculating the path again
        // for e.g. row=1, col1 comes in path twice
        if (row >= triangle.size()) {
            return 0;
        }
        
        return triangle.get(row).get(col) + Integer.min(minFrom(triangle, row+1, col), minFrom(triangle, row+1, col+1));
    }
    
    public int minFromMemoized(List<List<Integer>> triangle, int row, int col, HashMap<String, Integer> mem) {
        if (row >= triangle.size()) {
            return 0;
        }
        
        String rowCol = "" + row + "," + col;
        Integer val = mem.get(rowCol);
        if (val != null) {
            return val;
        }
        
        val = triangle.get(row).get(col) + Integer.min(minFromMemoized(triangle, row+1, col, mem), minFromMemoized(triangle, row+1, col+1, mem));
        mem.put(rowCol, val);
        return val;
    }

    public static void main(String[] args) {
        SolutionLC120TriangleMinPath sol = new SolutionLC120TriangleMinPath();
        List<List<Integer>> input = new ArrayList<List<Integer>>();
        List<Integer> l1 = new ArrayList<Integer>();
        l1.add(2);
        input.add(l1);

        List<Integer> l2 = new ArrayList<Integer>();
        l2.add(3);
        l2.add(4);
        input.add(l2);
        List<Integer> l3 = new ArrayList<Integer>();
        l3.add(6);
        l3.add(5);
        l3.add(7);
        input.add(l3);
        List<Integer> l4 = new ArrayList<Integer>();
        l4.add(4);
        l4.add(1);
        l4.add(8);
        l4.add(3);
        input.add(l4);

        System.out.println(sol.minimumTotal(input));
        


    }
}