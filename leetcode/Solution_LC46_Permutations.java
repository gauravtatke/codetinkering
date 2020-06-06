import java.util.*;
/* Given a collection of distinct integers, return all possible permutations.

Example:

Input: [1,2,3]
Output:
[
  [1,2,3],
  [1,3,2],
  [2,1,3],
  [2,3,1],
  [3,1,2],
  [3,2,1]
] */

class Solution_LC46_Permutations {
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> plist = new ArrayList<List<Integer>>();
        permuteSol1(nums, 0, nums.length-1,  plist);
        return plist;
    }
    
    public void permuteSol1(int[] nums, int start, int end, List<List<Integer>> plist) {
        if (start == end) {
            List<Integer> alist = new ArrayList<Integer>(nums.length);
            for(int n = 0; n < nums.length; n++) {
                alist.add(nums[n]);
            }
            plist.add(alist);
        } else {
            for (int i=start; i<=end; i++) {
                swap(nums, start, i);
                permuteSol1(nums, start+1, end, plist);
                swap(nums, start, i);
            }
        } 
    }
    
    public void swap(int[] nums, int idx1, int idx2) {
        int temp = nums[idx1];
        nums[idx1] = nums[idx2];
        nums[idx2] = temp;
    }

    public static void main(String[] args) {
        int[] nums = {1,2,3};
        Solution_LC46_Permutations sol = new Solution_LC46_Permutations();
        System.out.println(sol.permute(nums));
    }
}