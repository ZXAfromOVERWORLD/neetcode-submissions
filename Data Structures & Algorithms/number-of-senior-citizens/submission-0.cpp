class Solution {
public:
    int countSeniors(vector<string>& details) {
        char s1,s2;
        int ans = 0;
        for (auto i : details){
            s1 = i[11];
            s2 = i[12];
            int a,b;
            a = s1-'0';
            b = s2 - '0';
            int num = a*10 + b;

            if (num > 60){
                ans++;
            }
        }
        return ans;
    }
};