# Write a function to check whether an input string is a valid IPv4 address or IPv6 address or neither.
# IPv4 addresses are canonically represented in dot-decimal notation, which consists of four decimal numbers, each ranging from 0 to 255, separated by dots ("."), e.g.,172.16.254.1;
# Besides, leading zeros in the IPv4 is invalid. For example, the address 172.16.254.01 is invalid.
# IPv6 addresses are represented as eight groups of four hexadecimal digits, each group representing 16 bits. The groups are separated by colons (":"). For example, the address 2001:0db8:85a3:0000:0000:8a2e:0370:7334 is a valid one. Also, we could omit some leading zeros among four hexadecimal digits and some low-case characters in the address to upper-case ones, so 2001:db8:85a3:0:0:8A2E:0370:7334 is also a valid IPv6 address(Omit leading zeros and using upper cases).
# However, we don't replace a consecutive group of zero value with a single empty group using two consecutive colons (::) to pursue simplicity. For example, 2001:0db8:85a3::8A2E:0370:7334 is an invalid IPv6 address.
# Besides, extra leading zeros in the IPv6 is also invalid. For example, the address 02001:0db8:85a3:0000:0000:8a2e:0370:7334 is invalid.

# Note: You may assume there is no extra space or special characters in the input string.
# Example 1:
# Input: "172.16.254.1"
# Output: "IPv4"
# Explanation: This is a valid IPv4 address, return "IPv4".

# Example 2:
# Input: "2001:0db8:85a3:0:0:8A2E:0370:7334"
# Output: "IPv6"
# Explanation: This is a valid IPv6 address, return "IPv6".

# Example 3:
# Input: "256.256.256.256"
# Output: "Neither"
# Explanation: This is neither a IPv4 address nor a IPv6 address.


class Solution1(object):
    def validIPAddress(self, IP):
        """
        :type IP: str
        :rtype: str
        """
        if self.isValidIPV4(IP):
            return "IPv4"
        elif self.isValidIPV6(IP):
            return "IPv6"
        else:
            return "Neither"

    def isValidIPV4(self, IP):
        if len(IP) < 7 or len(IP) > 15:
            return False
        tokens = IP.split('.')
        if len(tokens) != 4:
            return False
        for token in tokens:
            if not self.isValidIPV4Token(token):
                return False
        return True

    def isValidIPV4Token(self, token):
        if not len(token) or token[0] in ('-', '+') or len(token) > 3:
            return False
        if token.startswith('0') and len(token) > 1:
            return False
        try:
            parseint = int(token)
        except Exception:
            print('Not an IPV4 token')
            return False
        else:
            if parseint > 255:
                return False
        return True

    def isValidIPV6(self, IP):
        if len(IP) < 15 or len(IP) > 39:
            return False
        tokens = IP.split(':')
        if len(tokens) != 8:
            return False
        for token in tokens:
            if not self.isValidIPV6Token(token):
                return False
        return True

    def isValidIPV6Token(self, token):
        if len(token) > 4 or not len(token):
            return False
        characters = list(token)
        digit = lowercase = uppercase = False
        for ch in characters:
            if not ((ord('0') <= ord(ch) <= ord('9')) or
                    (ord('a') <= ord(ch) <= ord('f')) or
                    (ord('A') <= ord(ch) <= ord('F'))):
                return False
        return True


class Solution1(object):
    def validIPAddress(self, IP):
        """
        :type IP: str
        :rtype: str
        """

        def is_hex(token):
            if not len(token):
                return False
            hex_digits = set('0123456789abcdefABCDEF')
            if len(token) > 4:
                return False
            for ch in token:
                if ch not in hex_digits:
                    return False
            return True

        tokens = IP.split('.')
        if len(tokens) == 4:
            # check for IPv4
            for token in tokens:
                try:
                    val = int(token)
                    if val < 0 or val > 255 or (token[0] in ('0', '-') and
                                                len(token) > 1):
                        return "Neither"
                except Exception:
                    return "Neither"
            return "IPv4"

        tokens = IP.split(':')
        if len(tokens) == 8:
            # check for IPv6
            for token in tokens:
                if len(token) > 4 or not is_hex(token):
                    return "Neither"
            return "IPv6"

        return "Neither"
