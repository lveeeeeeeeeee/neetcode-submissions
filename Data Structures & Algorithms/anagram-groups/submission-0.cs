public class Solution {
    public List<List<string>> GroupAnagrams(string[] strs) {
        Dictionary<string, List<int>> dict = new Dictionary<string, List<int>>();
        for (int j = 0; j < strs.Length; j++) {
            char[] ch = strs[j].ToCharArray();
            char[] letters = new char[26];
            for (int i = 0; i < ch.Length; i++) {
                letters[ch[i] - 'a'] += (char)1;
            }
            string repr = new string(letters);
            if (!dict.ContainsKey(repr))
            {
                dict.Add(repr, new List<int>(strs.Length));
            }
            dict[repr].Add(j);
        }

        List<List<string>> result = new List<List<string>>(dict.Keys.Count);
        List<string> keys = new List<string>(dict.Keys);
        for (int i = 0; i < keys.Count; i++)
        {
            List<string> toPut = new List<string>();
            List<int> indeces = dict[keys[i]];
            foreach (int ind in indeces)
            {
                toPut.Add(strs[ind]);
                Console.WriteLine(strs[ind] + $" {i}");
            }
            result.Add(toPut);
        }
        return result;
    }
}
