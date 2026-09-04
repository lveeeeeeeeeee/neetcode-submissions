class Solution:
    def isAlphanumeric(self, sym: chr):
        return ( (ord('A') <= ord(sym) <= ord('z')) or (ord('0') <= ord(sym) <= ord('9')) ) and not (ord('Z') < ord(sym) < ord('a'))

    def convertForChecks(self, s: str):
        res = []
        capital = s.lower()
        for sym in capital:
            if self.isAlphanumeric(sym):
                res.append(sym)
        return res

    def isPalindrome(self, s: str) -> bool:
        toCheck = self.convertForChecks(s)

        left = len(toCheck) // 2 - (len(toCheck) + 1) % 2
        right = len(toCheck) // 2
        result = False
        while (left >= 0 and right < len(toCheck)):
            while(not self.isAlphanumeric(toCheck[left])):
                left -= 1
            while(not self.isAlphanumeric(toCheck[right])):
                right += 1
            if toCheck[left] != toCheck[right]:
                print(right, left, toCheck[right], toCheck[left])
                return result
            left -= 1
            right += 1
        return True
        