from __future__ import annotations
from typing import Tuple

class Solution:
    def addBinary(self, a: str, b: str) -> str:
        ai = len(a) - 1
        bj = len(b) - 1
        result = ''
        carry = '0'
        while ai >= 0 and bj >= 0:
            digit_sum, carry = binary_add(a[ai], b[bj], carry)
            result = digit_sum + result
            ai = ai - 1
            bj = bj - 1
        if ai >= 0 and bj < 0:
            while ai >= 0:
                digit_sum, carry = binary_add(a[ai], '0', carry)
                result = digit_sum + result
                ai = ai - 1
        if ai < 0 and bj >= 0:
            while bj >= 0:
                digit_sum, carry = binary_add('0', b[bj], carry)
                result = digit_sum + result
                bj = bj - 1
        if carry == '1':
            result = carry + result
        return result
        
            

def binary_add(num1: str, num2: str, carry: str) -> Tuple[str, str]:
    digit_sum = 0
    digit_carry = 0
    if num1 == '1' and num2 == '1' and carry == '1':
        digit_sum = '1'
        digit_carry = '1'
    elif (num1 == '0' and num2 == '1' and carry == '1') or (num1 == '1' and num2 == '0' and carry == '1'
            ) or (num1 == '1' and num2 == '1' and carry == '0'):
        # any 2 digits are 1,  sum = 0, carry = 1
        digit_sum = '0'
        digit_carry = '1'
    elif (num1 == '0' and num2 == '0' and carry == '1') or (num1 == '0' and num2 == '1' and carry == '0'
            ) or (num1 == '1' and num2 == '0' and carry == '0'):
        # any one digit is 1
        digit_sum = '1'
        digit_carry = '0' 
    elif num1 == '0' and num2 == '0' and carry == '0':
        digit_sum = '0'
        digit_carry = '0'
    return (digit_sum, digit_carry)

if __name__ == '__main__':
    a = '1111'
    b = '1111'
    result = Solution().addBinary(a, b)
    print(f'a = {a}, b = {b}, sum = {result}')
