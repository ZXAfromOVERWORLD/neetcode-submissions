class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        int len = nums.size();
        vector<int> ans;

        for (int i = 0; i < len ;i++){
            int find = target - nums[i];
            for(int j = i+1; j < len;j++){
                if (nums[j]==find){
                    ans.push_back(i);
                    ans.push_back(j);
                    return ans;
                }
            }
        }
        return ans;
    }
};
