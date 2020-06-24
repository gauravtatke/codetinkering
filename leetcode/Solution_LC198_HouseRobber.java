//You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

//Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.

 

//Example 1:

//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
             //Total amount you can rob = 1 + 3 = 4.
//Example 2:

//Input: nums = [2,7,9,3,1]
//Output: 12
//Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
             //Total amount you can rob = 2 + 9 + 1 = 12.


public class Solution_LC198_HouseRobber {
    public int rob(int[] nums) {
        if (nums.length == 0) {
            return 0;
        }
        if (nums.length == 1) {
            return nums[0];
        }
        return robSol2(nums);
    }

    public int robSol1(int[] house) {
        int[] profit = new int[house.length+1];
        profit[0] = 0; // starting condition
        // copy all of house[i] into profit[i+1]
        // profit[i+1] is the profit for robbing house[i]
        // profit[i+1] is profit of house[i] + max of profit until house[i-2] or house[i-3]
        // because we cannot rob house[i-1] and rob house[i] as well.
        for (int i = 0; i < house.length; i++) {
            profit[i+1] = house[i];
        }

        // profit[i] is profit uptill now
        for(int i = 3; i < profit.length; i++) {
            profit[i] = profit[i] + Integer.max(profit[i-2], profit[i-3]); 
        }

        // last 2 cell of profit array will have largest profit so
        // return whichever is max
        return Integer.max(profit[profit.length - 1], profit[profit.length-2]); 
    }

    public static void main(String[] args) {
       Solution_LC198_HouseRobber sol = new Solution_LC198_HouseRobber();
       int[] house = {2, 7, 9, 3, 1};
       System.out.println(sol.rob(house));
    }

    public int robSol2(int[] house) {
        // same concept as sol1 but instead of calculating
        // profit[i] including house[i] and doing i-2, i-3 logic
        // we can say profit[i] is either profit[i-2] + house[i]
        // OR profit[i-1]. Which means we either include current house
        // and count i-2 house profit or just count i-1 house and exclude
        // house[i]. In effect profit[i] may or may not contain house[i].
        int[] profit = new int[house.length+1];
        profit[0] = house[0];
        profit[1] = house[1];
        for (int i = 2; i < profit.length; i++) {
            profit[i] = Integer.max(profit[i-1], house[i-1] + profit[i-2]);
        }
        return profit[profit.length - 1];
    }
}
