from __future__ import annotations

roman_dict = {
        'I': 1,
        'V': 5,
        'X': 10,
        'L': 50,
        'C': 100,
        'D': 500,
        'M': 1000
    }

class Solution:
    def romanToInt(self, s: str) -> int:
        result_int = 0
        prev = None
        for curr in s[-1::-1]:
            if (prev in ('V', 'X') and curr == 'I') or (prev in ('L', 'C') and curr == 'X') or (prev in ('D', 'M') and curr == 'C'):
                result_int = result_int - roman_dict[curr]
            else:
                result_int = result_int + roman_dict[curr]
            prev = curr
        return result_int
            
            
if __name__ == '__main__':
    roman = 'MCMXCIV'
    solution = Solution()
    print(f'Roman {roman} to int {solution.romanToInt(roman)}')