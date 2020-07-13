// Say you have an array prices for which the ith element is the price of a given stock on day i.

// Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).

// Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).

// Example 1:

// Input: [7,1,5,3,6,4]
// Output: 7
// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
//              Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
// Example 2:

// Input: [1,2,3,4,5]
// Output: 4
// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//              engaging multiple transactions at the same time. You must sell before buying again.
// Example 3:

// Input: [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transaction is done, i.e. max profit = 0.

class SolutionLC122BestTimeToBuyStock2 {
    public int maxProfit(int[] prices) {
        return maxProfitSol1(prices);
    }
    
    public int maxProfitSol1(int[] prices) {
        // basically we need to scan and find the dip in market
        int profit = 0;
        int low = 0;
        int high = 0;
        for (int i = 1; i < prices.length; i++) {
            while (i < prices.length && prices[i] < prices[low]) {
                low = i;
                high = i;
                i++;
            }

            while (i < prices.length && prices[i] > prices[high]) {
                high = i;
                i++;
            }
            
            System.out.println();
            if (high > low) {
                profit += (prices[high] - prices[low]);
            }
            if (i < prices.length) {
                low = i;
                high = i;
            }
        }
        
        return profit;
    }

    public static void main(String[] args) {
        SolutionLC122BestTimeToBuyStock2 sol = new SolutionLC122BestTimeToBuyStock2();
        int[] prices1 = {7, 1, 5, 3, 6, 4};
        int[] prices2 = {7, 5, 4, 3, 2, 1};
        int[] prices3 = {1, 2, 4, 6, 7};
        System.out.println(sol.maxProfitSol1(prices1));
        System.out.println(sol.maxProfitSol1(prices2));
        System.out.println(sol.maxProfitSol1(prices3));
    }
}