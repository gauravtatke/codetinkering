// Given an integer array nums, find the contiguous subarray within an array (containing at least one number) which has the largest product.

// Example 1:

// Input: [2,3,-2,4]
// Output: 6
// Explanation: [2,3] has the largest product 6.
// Example 2:

// Input: [-2,0,-1]
// Output: 0
// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

// examples to consider -
//[0, 2] 

//[2,-5,-2,-4,3]


class SolutionLC152MaxProductSubArray {
    public int maxProduct(int[] nums) {
        return maxProductSol1(nums);
    }
    
    public int maxProductSol1(int[] nums) {
        // this is similar to kadane's also for max subarray sum
        // but we need to take one more variable currMin to keep track of min value until
        // index i. Because minvalue can result in highest product if current i elem is negative
        // and minValue is also negative.
        int maxProduct = nums[0];
        int currMin = nums[0];
        int currMax = nums[0];
        int prevMin = nums[0];
        int prevMax = nums[0];
        for (int i = 1; i < nums.length; i++) {
            currMax = Integer.max(nums[i], Integer.max(prevMax*nums[i], prevMin*nums[i]));
            currMin = Integer.min(nums[i], Integer.min(prevMax*nums[i], prevMin*nums[i]));
            maxProduct = Integer.max(currMax, maxProduct);
            
            prevMax = currMax;
            prevMin = currMin;
        }
        
        return maxProduct;
    }
}