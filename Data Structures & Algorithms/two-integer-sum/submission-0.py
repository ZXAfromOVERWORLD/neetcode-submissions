class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        d={}
        for i , j in enumerate (nums):
            comp = target - j
            if comp in d:
                return [d[comp],i]
            d[j]=i
        return []


        