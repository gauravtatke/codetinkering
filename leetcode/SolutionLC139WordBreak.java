// Given a non-empty string s and a dictionary wordDict containing a list of non-empty words, determine if s can be segmented into a space-separated sequence of one or more dictionary words.

// Note:

// The same word in the dictionary may be reused multiple times in the segmentation.
// You may assume the dictionary does not contain duplicate words.
// Example 1:

// Input: s = "leetcode", wordDict = ["leet", "code"]
// Output: true
// Explanation: Return true because "leetcode" can be segmented as "leet code".
// Example 2:

// Input: s = "applepenapple", wordDict = ["apple", "pen"]
// Output: true
// Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//              Note that you are allowed to reuse a dictionary word.
// Example 3:

// Input: s = "catsandog", wordDict = ["cats", "dog", "sand", "and", "cat"]
// Output: false

import java.util.*;

class SolutionLC139WordBreak {
    public boolean wordBreak(String s, List<String> wordDict) {
        return canWordBreak(s, wordDict, 0);
    }
    
    public boolean canWordBreak(String s, List<String> wordDict, int searchIndex) {
        // traverse wordDict and find if s starts with word in word dict from starting from searchIndex=0
        // if match is found and recursively call but adjust the searchIndex such that
        // we try to find word in s starting from searchIndex+word.length()
        
        if (searchIndex >= s.length()) {
            // means we have found the last substring match so
            // word can be broken
            return true;
        }
        
        // boolean found = false;
        for(String word: wordDict) {
            if (s.startsWith(word, searchIndex)) {
                // look for another word in substring starting searchIndex+word.length()
                System.out.println(word + ", " + searchIndex);
                if (canWordBreak(s, wordDict, searchIndex+word.length())) {
                    return true;
                }
            }
        }
        
        return false;
    }

    public static void main(String[] args) {
        SolutionLC139WordBreak sol = new SolutionLC139WordBreak();
        String s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
        // String s = "aaaaaaaaab";
        String[] dict = {"a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"};
        System.out.println(sol.wordBreak(s, Arrays.asList(dict)));
    }
}