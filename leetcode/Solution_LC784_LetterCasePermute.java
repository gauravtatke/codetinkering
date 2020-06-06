/* Given a string S, we can transform every letter individually to be lowercase or uppercase to create another string.  Return a list of all possible strings we could create.

Examples:
Input: S = "a1b2"
Output: ["a1b2", "a1B2", "A1b2", "A1B2"]

Input: S = "3z4"
Output: ["3z4", "3Z4"]

Input: S = "12345"
Output: ["12345"]
Note:

S will be a string with length between 1 and 12.
S will consist only of letters or digits. */

import java.lang.Character;
import java.util.*;

class Solution_LC784_LetterCasePermute {
    public static void main(String[] args) {
        Solution_LC784_LetterCasePermute sol = new Solution_LC784_LetterCasePermute();
        System.out.println(sol.letterCasePermutation("1234"));
    }

    public List<String> letterCasePermutation(String S) {
        return letterCasePermuteSol1(S);
    }
    
    public List<String> letterCasePermuteSol1(String s) {
        char[] arr = s.toCharArray();
        List<String> solution = new ArrayList<String>();
        upperLowerPermute(arr, 0, solution);
        return solution;
    }
    
    public void upperLowerPermute(char[] arr, int start, List<String> sol) {
        // System.out.println(dbg + ", " + String.valueOf(arr) + ", start=" + start + ", end=" + end);
        sol.add(new String(arr));
        
        for (int i = start; i < arr.length; i++) {
                if (Character.isLetter(arr[i])) {
                    if (Character.isUpperCase(arr[i])) {
                        arr[i] = Character.toLowerCase(arr[i]);
                        upperLowerPermute(arr, i+1, sol);
                        arr[i] = Character.toUpperCase(arr[i]);
                    } else {
                        // System.out.println(dbg + ", i = " + i);
                        arr[i] = Character.toUpperCase(arr[i]);
                        upperLowerPermute(arr, i+1, sol);
                        arr[i] = Character.toLowerCase(arr[i]);
                    }
                    
                }
            }
        
        }
}