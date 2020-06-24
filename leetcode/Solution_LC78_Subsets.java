
import java.util.*;
class Solution_LC78_Subsets {
    public List<List<Integer>> subsets(int[] nums) {
        // return subsetsSol1(nums);
        return subsetsSol2(nums);
    }
    
    public List<List<Integer>> subsetsSol2(int[] nums) {
        // idea is to start with empty set and loop over items of array
        // create new set by combining pre-existing set with new item
        // add new set to pre-existing set
        List<List<Integer>> result = new ArrayList<List<Integer>>();
        result.add(new ArrayList<Integer>());
        for(Integer num: nums) {
            // combine with previous result
            List<List<Integer>> newSubSet = new ArrayList<List<Integer>>();
            for (List<Integer> currResult: result) {
                List<Integer> temp = new ArrayList<Integer>(currResult); // create new list for existing set
                temp.add(num); // add num to new list to get new set
                newSubSet.add(temp);
            }
            
            // now add newSubSet values back to result
            result.addAll(newSubSet);
        }
        return result;
    }
    
    public List<List<Integer>> subsetsSol1(int[] nums) {
        // power set of n elements is set of all combinations of 1 elem, 2 elem, 3 elem .. n elem
        List<List<Integer>> powerSet = new ArrayList<List<Integer>>();
        powerSet.add(new ArrayList<Integer>()); // adding empty list
        for(int i = 1; i <= nums.length; i++) {
            getCombination(nums, i, powerSet);
        }
        
        return powerSet;
    }
    
    public void getCombination(int[] nums, int nComb, List<List<Integer>> combinations) {
        //ipLength -> inputLength, nComb -> num of elements for combination nCr
        // System.out.println("nComb = " + nComb);
        int[] result = new int[nComb];
        combinationHelper(nums, 0, 0, nComb, result, combinations);
    }
    
    public void combinationHelper(int[] nums, int start, int index, int nComb, int[] result, List<List<Integer>> combinations) {
        // index -> start index in result, start, end -> start & end in nums array
        // System.out.println("start=" + start + ", index=" + index + ", ncomb=" + nComb);
        if (index == nComb) {
            // combination is complete add it in list
            combinations.add(createList(result));
        } else {
            for (int i = start; i < nums.length && (nums.length - i >= nComb - index); i++) {
                result[index] = nums[i];
                combinationHelper(nums, i+1, index+1, nComb, result, combinations);
            }
        }
    }
    
    public List<Integer> createList(int[] nums) {
        List<Integer> retList = new ArrayList<Integer>(nums.length);
        for(Integer num: nums) {
            retList.add(num);
        }
        return retList;
    }
}