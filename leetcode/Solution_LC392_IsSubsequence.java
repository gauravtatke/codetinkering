//Given a string s and a string t, check if s is subsequence of t.

//A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).

//Follow up:
//If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?

//Credits:
//Special thanks to @pbrother for adding this problem and creating all test cases.

 

//Example 1:

//Input: s = "abc", t = "ahbgdc"
//Output: true
//Example 2:

//Input: s = "axc", t = "ahbgdc"
//Output: false

public class Solution_LC392_IsSubsequence {
    public static void main(String[] args) {
        Solution_LC392_IsSubsequence sol = new Solution_LC392_IsSubsequence();
        System.out.println(sol.isSubsequence("abc", "ahbgdc"));        
    }

    // returns whether s is subsequence of t or not
    public boolean isSubsequence(String s, String t) {
        //return isSubsequenceBrute(s, t);       
        return isSubsequenceRecursive(s, s.length()-1, t, t.length()-1);
    }

    public boolean isSubsequenceBrute(String s, String t) {
        // in brute force approach, we just traverse through s and find each
        // char in t. if we find a match then we pick second char in s and start
        // from previous match location in t
        int i = 0;
        int j = 0;
        while (i < s.length() && j < t.length()) {
            if (s.charAt(i) == t.charAt(j)) {
                // its match then increment both pointer
                i++;
                j++;
            } else {
                // just increment j
                j++;
            }
        }

        if (i == s.length() && j <= t.length()) {
            return true;
        }
        return false;
    }

    public boolean isSubsequenceRecursive(String s, int sindex, String t, int tindex) {
        if (tindex < 0 && sindex >= 0) {
            //System.out.println(sindex + ", "+tindex);
            return false;
        }
        if (sindex < 0) {
            //System.out.pintln(sindex + ", "+tindex);
            return true;
        }
        if (s.charAt(sindex) == t.charAt(tindex)) {
            return isSubsequenceRecursive(s, sindex-1, t, tindex-1);
        } else {
            return isSubsequenceRecursive(s, sindex, t, tindex-1);
        }
    }
}








