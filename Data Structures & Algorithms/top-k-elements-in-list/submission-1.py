from collections import Counter 

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        ans=[]
        lms=Counter(nums)
        common = lms.most_common(k)
        for i in common:
            ans.append(i[0])
        return ans
                
        
        