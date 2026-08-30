class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        int start = 0;
        int ans = 0;
        int count = 0;
        unordered_set<int> st;

        if (nums.size() == 0){
            return 0;
        }

        if (nums.size() == 1){
            return 1;
        }

        for(auto i = 0 ; i < nums.size(); i++){
            st.emplace(nums[i]);
        }
        for(auto i = 0; i<nums.size(); i++){
            if(st.count(nums[i]-1)){
                continue;
            }
            start = nums[i];
            count = 0;
            while (st.count(start)){
                count += 1;
                start += 1;
            }
            ans = max(ans,count);
        }
        return ans;
    }
};
