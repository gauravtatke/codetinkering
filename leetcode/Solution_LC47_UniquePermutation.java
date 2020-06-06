/* 47. Permutations II
Given a collection of numbers that might contain duplicates, return all possible unique permutations.
Example:
Input: [1,1,2]
Output:
[
  [1,1,2],
  [1,2,1],
  [2,1,1]
] */

import java.util.*;

class Solution_LC47_UniquePermutation {
    public List<List<Integer>> permuteUnique(int[] nums) {
        List<List<Integer>> result = new ArrayList<List<Integer>>();
        // HashSet<Integer> visited = new HashSet<Integer>();
        uniquePermute(nums, 0, result);
        return result;
    }
    
    public void uniquePermute(int[] nums, int start, List<List<Integer>> result) {
        if (start == nums.length-1) {
            List<Integer> temp = new ArrayList<Integer>(nums.length);
            for(Integer num: nums) {
                temp.add(num);
            }
            result.add(temp);
        } else {
            // for each subarray we are permuting, maintain a set of integers visited for that subset
            // if number is not seen prev then permute for it otherwise don't
            // because swaping already seen number would generate duplicate
            // permutations
            HashSet<Integer> visited = new HashSet<Integer>();
            for(int i = start; i < nums.length; i++) {
                if (!visited.contains(nums[i])) {
                    swap(nums, i, start);
                    uniquePermute(nums, start+1, result);
                    swap(nums, i, start);
                    visited.add(nums[i]);
                }
                
            }
        }
    }
    
    public void swap(int[] nums, int indx1, int indx2) {
        int temp = nums[indx1];
        nums[indx1] = nums[indx2];
        nums[indx2] = temp;
    }
}