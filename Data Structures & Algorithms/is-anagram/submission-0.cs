public class Solution {
    public bool IsAnagram(string s, string t) {
        
        if (s.Length != t.Length) return false;

        Span<int> l = stackalloc int[26];
        char[] sc = s.ToCharArray();
        char[] tc = t.ToCharArray();

        for (int i = 0; i < s.Length; i++) {
            l[sc[i] - 'a'] += 1;
            l[tc[i] - 'a'] -= 1;
        }

        for (int i = 0; i < l.Length; i++)
        {
            if (l[i] != 0)
            {
                return false;
            } 
        }
        return true;
    }
}
