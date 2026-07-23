public class Solution {

    public string Encode(IList<string> strs) {
        string result = "";
        foreach (string str in strs)
        {
            result += $"@{str.Length}@";
            result += str;
        }
        return result;
    }

    public int EncodedLength(ref string s, ref int pos) {
        int res = 0;
        if (s[pos] == '@')
        {
            pos += 1;
            while (s[pos] != '@')
            {
                res = (res * 10) + (s[pos] - '0');
                pos += 1;
            }
        }
        pos += 1;
        return res;
    }

    public List<string> Decode(string s) {
        var res = new List<string>();
        int i = 0;
        while (i < s.Length)
        {
            int toRead = 0;
            if (s[i] == '@')
            {
                toRead = EncodedLength(ref s, ref i);
            }
            char[] word = new char[toRead];
            int cnt = i;
            while (cnt < i + toRead && cnt < s.Length)
            {
                word[cnt - i] = s[cnt];
                cnt++;
            }
            res.Add(new string(word));
            i = cnt;
        }
        return res;
   }
}
