
import java.util.*;

class Solution_LC357_CountNumWithUniqDigits {
    public int countNumbersWithUniqueDigits(int n) {
        return countNumbersWithUniqDigitsSol2(n);
    }
    
    public int countNumbersWithUniqDigitsSol1(int n) {
        // simple combinatorics solution
        // for n digits, 1st place has 10 options, second digit has 9 option,
        // 3rd digit has 8 option and so on. As per the restriction 0 should be included so 1
        // can be added
        if (n == 0) {
            // there is only way with zero digits
            // this is test case but i dont know how a number can be formed with zero digits
            return 1;
        }
        
        if (n == 1) {
            // only 10 possibilities
            return 10;
        }
        
        // for r digits total count of unique numbers = nP1 + nP2 + nP2 ... nPr
        // i.e. all permutations of 1 digit numbers + all permutations of 2 digit nums ... all permutations of r digit nums
        // But if we allow 0 to be first digit then 1 digit permutation will be 0-9
        // 2 digit permutations will also include 01, 02, 03 .. 98 which is counting 01-09 twice
        // similarly for 3 digits 012, 013 .. combinations will be counting twice as they are also included
        // as 12, 13 in 2 digit solution. So for every nPr we need to only take count of 90% of numbers
        // as others will be starting with 0 (for 1 digit permutation out of 10, we take only 9, from 2 digit permutation we discard 9 , 1-9 numbers)
        double count = 0;
        for(int i = 1; i <= n; i++) {
            count = count + (factorial(10)/ factorial(10-i)) * 0.9;
        }
        
        // since 0 needs to be included in solution, we discarded it in 10P1
        // we need to add that count
        return (int)count+1;
    }
    
    public int factorial(int num) {
        int fact = 1;
        for (int i = num; i > 0; i--) {
            fact = fact*i;
        }
        return fact;
    }
    
    public int countNumbersWithUniqDigitsSol2(int n) {
        // same concept as above but we can create dp approach
        if (n > 10) {
            // for n > 10 there cannot be any number without having duplicate digits
            return 0;
        }
        int[] uniqDigitNums = new int[10];
        uniqDigitNums[0] = 1;
        uniqDigitNums[1] = 10;
        // as stated in above method 1st digit have 10 options but if we discard 0 as first digit
        // then 1st -> 9 options
        // 2nd digit -> 9 options (including 0)
        // 3rd digit -> 8 options (not repeating any previous digits)
        // 4th digit -> 7 options ... so on
        // uniqDigitNums[n] contains count of uniqDigit numbers for n digits (including leading 0s)
        // uniqDigitsNum[n+1] = uniqdigits nums for n+1 digits without leading 0 + uniqDigitNums[n]
        // for e.g. uniqDigitsNum[2] = 9*9 (1st and 2nd digit) + uniqDigitsNum[1] = 81+10=91
        // uniqDigitsNum[3] = 9*9*8 + uniqDigitsNum[2] = 648 + 91 = 739
        for (int i=2; i <= n; i++) {
            // start from 2 to n, for each i calculate uniqDigitNum
            int count = 9; // start count for 1st digit
            // below does 9*9... calc
            for(int j = 1; j < i; j++) {
                count = count*(10-j);
            }
            uniqDigitNums[i] = count + uniqDigitNums[i-1];
        }
        return uniqDigitNums[n];
    }

    public int countNumbersWithUniqDigitsSol2Simplified(int n) {
        if (n > 10) {
            // for n > 10 there cannot be any number without having duplicate digits
            return 0;
        }
        int[] uniqDigitNums = new int[10];
        uniqDigitNums[0] = 1;
        uniqDigitNums[1] = 10;
        int count = 9;
        for (int i = 2; i <= n; i++) {
            count = count * (11-i);
            uniqDigitNums[i] = uniqDigitNums[i-1] + count;
        }
        return uniqDigitNums[n];
    }

    public static void main(String[] args) {
        Solution_LC357_CountNumWithUniqDigits sol = new Solution_LC357_CountNumWithUniqDigits();
        System.out.println(sol.countNumbersWithUniqDigitsSol2Simplified(3));
    }
}