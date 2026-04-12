class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        cnt = [0] * 26
        for c1 , c2 in zip (s, t):
            cnt[ord(c1) - ord('a')] += 1
            cnt[ord(c2) - ord('a')] -= 1
        return all( c == 0 for c in cnt)
        