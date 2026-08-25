class Solution {
public:

    static bool comp(pair<int,int> p1, pair<int,int> p2){
        if (p1.second > p2.second) return true;
        else return false;
    }




    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int,int> mp;

        vector<int> ans;

        for (auto i : nums){
            mp[i]++;
        }
        
        vector<pair<int,int>> v(mp.begin(),mp.end());
        

        sort(v.begin(),v.end(),comp);

        
        for (auto i = 0; i < k ;i++){
            ans.push_back(v[i].first);
        }

        return ans;

    }
};
