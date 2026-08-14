class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        int n = strs.length;
        List<List<String>> ans = new ArrayList<>();

        HashMap<String,ArrayList<Integer>> map = new HashMap<>();
        for(int i=0;i<n;i++){
            char[] chArray = strs[i].toCharArray();
            Arrays.sort(chArray);
            String s = new String(chArray);
            if(!map.containsKey(s)){
                map.put(s,new ArrayList<>());
            }
            map.get(s).add(i);
        }
        for(List<Integer> value : map.values()){
            List<String> list = new ArrayList<>();
            for(int i : value){
                list.add(strs[i]);
            }
            ans.add(list);
        }
        return ans;
    }
}
