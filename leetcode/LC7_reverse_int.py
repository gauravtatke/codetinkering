from __future__ import annotations

class Solution:
    def reverse(self, x: int) -> int:
        return reverse1(x)
    
MAX_INT = 2147483647
MIN_INT = -2147483648

def reverse1(x: int) -> int:
    rev_num = 0
    signed = False
    if x < 0:
        signed = True
        x = abs(x)
    while x != 0:
        remainder = x % 10
        x = x // 10
        # rev_num = reverse_num * 10 + remainder
        # rev_num overflows if rev_num >= MAX_INT // 10
        # rev_num > MAX_INT // 10 -> it will definitely overflow after adding remainder
        # rev_num == MAX_INT // 10 -> overflow only when remainder > 7 (last digit)
        # similar logic for negative int
        # NOTE: Python's modulo of negative numbers does not work like C or Java
        if rev_num > MAX_INT//10 or (rev_num == MAX_INT//10 and remainder > 7):
            return 0
        rev_num = rev_num * 10 + remainder
    return rev_num if not signed else rev_num * -1
        
        
        