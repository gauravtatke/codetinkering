import java.util.*;

public class SubArrayWithSumZero {
    public static boolean subarrayWithSumZero(int[] arr) {
        // return true false depending on whether we find a subarray
        // brute force approach. Calc all subarrays
        int currSum = 0;
        for (int i = 0; i < arr.length; i++) {
            currSum = arr[i];
            for (int j = i+1; j < arr.length; j++) {
                currSum += arr[j];
                if (currSum == 0) {
                    return true; 
                } 
            } 
        }
        return false;
    }

    public static boolean subarrayZeroSumEfficient(int[] arr) {
        // this is linear approach. We traverse the array
        // calc sum from 0 to i in a hashmap/hashset. if curr_sum is zero
        // OR curr_sum already exists in hashmap/hashset then there is 
        // subarray with sum 0
        int currSum = 0;
        HashSet<Integer> sum = new HashSet<Integer>();
        for (int i = 0; i < arr.length; i++) {
            currSum += arr[i];
            // sum contains same currSum then it means that there exists
            // a subarray between previous index where currSum happen and curr index
            // where same currSum happend. For e.g. there is index i there sum0..i is x
            // and then if we get same currSum for 0..j then subarray[i..j] is addin upto zero
            if (currSum == 0 || sum.contains(currSum)) {
                return true; 
            }
            sum.add(currSum); 
        }
        return false;
    }

    public static void main(String[] args) {
        int[] arr = {1, 4, -2, -2, 5, -4, -3};
        System.out.println(subarrayWithSumZero(arr));
        System.out.println(subarrayZeroSumEfficient(arr));
    }
}
