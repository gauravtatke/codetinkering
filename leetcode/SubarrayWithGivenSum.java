import java.util.*;

public class SubarrayWithGivenSum {
    /* this only works for non-negative integer values */
    public static boolean subarrayWithGivenSum(int[] arr, int sum) {
        // here we use a sliding window concept.
        // we start from 0 and keep adding element until the sum is less than given sum
        // if sum becomes greater than givenSum the remove element from start of the
        // window and adjust starting position of the window
        int currSum = 0;
        int start = 0;
        for (int i = 0; i < arr.length; i++) {
            currSum += arr[i];
            while (currSum > sum && start <= i) {
                // remove element from start
                currSum = currSum - arr[start];
                // adjust start
                start++; 
            }
            if (currSum == sum) {
                System.out.println("subarray: " + start + ", " + i);
                return true; 
            }
        }
        return false;
    }
    
    /* this handles negative integers as well */ 
    public static boolean subarrayWithGivenSumEfficient(int[] arr, int sum) {
        // this will use same approach like subarray with zero sum with hashmap
        int currSum = 0;
        HashMap<Integer, Integer> sumMap = new HashMap<Integer, Integer>();
        for (int i = 0; i < arr.length; i++) {
            currSum += arr[i];
            if (currSum == sum) {
                System.out.println("required subarray between 0" + ", " + i);
                return true; 
            }

            if (sumMap.containsKey(currSum - sum)) {
                // if map[currSum - sum] exists that means that
                // subarray between map[currSum - sum] & map[currSum] has required sum
                int start = sumMap.get(currSum - sum) + 1; // subarray starts from next location
                System.out.println("required subarray between " + start + ", " + i);
                return true;
            }
            // else add currSum and index to hashmap
            sumMap.put(currSum, i); 
        }

        return false; 
    }

    public static void main(String[] args) {
        int[] arr = {15, 2, 4, 8, 9, 5, 10, 23};
        int[] arr2 = {10, 2, -2, -20, 10};
        System.out.println(subarrayWithGivenSumEfficient(arr2, -10));
        System.out.println(subarrayWithGivenSum(arr, 23));
    }
}
