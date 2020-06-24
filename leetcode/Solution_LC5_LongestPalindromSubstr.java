//Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

//Example 1:

//Input: "babad"
//Output: "bab"
//Note: "aba" is also a valid answer.
//Example 2:

//Input: "cbbd"
//Output: "bb"

public class Solution_LC5_LongestPalindromSubstr {
    public String longestPalindrom(String s){
       return longestPalindromDp(s); 
    
    }

    public String longestPalindromBrute(String s) {
        int start = 0;
        int end = s.length()-1;
        while (start < end) {
            if (isPalindrom(s, start, end)) {
                return s.substring(start, end+1);
            } else if (isPalindrom(s, start+1, end)) {
                return s.substring(start+1, end+1);                
            } else if (isPalindrom(s, start, end-1)) {
                return s.substring(start, end);
            } else {
                start++;
                end--;
            }
        }

        return null;
    }

    public boolean isPalindrom(String s, int start, int end) {
        while(start < end) {
            if (s.charAt(start) != s.charAt(end)) {
                return false;
            } else {
                start++;
                end--;
            }
        }
        return true;
    }

    public String longestPalindromDp(String s) {
        int len = s.length();
        // create a table[n][n]. table[i][j] is true if s[i] == s[j] and table[i+1][j-1] is true
        // table[i+1][j-1] is substring between i and j
        boolean[][] table = new boolean[len][len];
        
        // every single character is palindrom
        for (int i = 0; i < len; i++) {
            table[i][i] = true;
        }
        
        int startIndex = 0;
        int maxLength = 1; // single character palindrom
        // find 2 char size palindrom. if s[i]== s[i+1] then its palindrom
        for (int i = 0; i < len-1; i++) {
            if (s.charAt(i) == s.charAt(i+1)) {
                table[i][i+1] = true;
                startIndex = i;
                maxLength = 2;    
            }
        }

        // find size 3, 4, 5, ... len size of palindrom
        for (int k = 3; k <= len; k++) {
            for (int start = 0; start < len-k+1; start++) {
                int endIndex = start+k-1; // for each start, get the end index of substring
                if (s.charAt(start) == s.charAt(endIndex) && table[start+1][endIndex-1]) {
                   table[start][endIndex] = true;
                   if (maxLength < k) {
                       maxLength = k;
                      startIndex = start; 
                   }
                }
            }
        }

        return s.substring(startIndex, startIndex+maxLength);
    }

    public static void main(String[] args) {
        Solution_LC5_LongestPalindromSubstr sol = new Solution_LC5_LongestPalindromSubstr();
        System.out.println(sol.longestPalindrom("asdgasd"));
    }
}
