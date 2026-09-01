class Solution {
public:
    bool isPalindrome(string s) {
        vector<char> v;

        for (auto i : s){
            if (!isalnum(i)){
                continue;
            }
            i = tolower(i);
            v.push_back(i);
        }

        int i = 0;
        int j = v.size() - 1;

        while (i < j){
            if (v[i] == v[j]){
                i++;
                j--;
            } else{
                break;
            }
        }

        if (i==j || j < i){
            return true;
        }else{
            return false;
        }

    }
};
