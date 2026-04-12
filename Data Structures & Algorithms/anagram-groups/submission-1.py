from collections import defaultdict

class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        ans = defaultdict(list)
        for i in strs:
            cnt = [0] * 26
            for chrs in i:
                cnt[ord(chrs) - ord('a')] += 1
            key = tuple(cnt)
            ans[key].append(i)
        return list(ans.values())