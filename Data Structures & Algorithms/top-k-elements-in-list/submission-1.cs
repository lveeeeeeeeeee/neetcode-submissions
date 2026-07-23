public class Solution {
    public int[] TopKFrequent(int[] nums, int k) {
        int[] result = new int[k];
        Dictionary<int, int> freqDict = new Dictionary<int, int>();

        int max = -1001;

        for (int i = 0; i < nums.Length; i++)
        {
            Console.WriteLine("hi 1");
            if (max < nums[i]) max = nums[i];
            if (!freqDict.ContainsKey(nums[i]))
            {
                freqDict.Add(nums[i], 1);
            }
            else
            {
                freqDict[nums[i]] += 1;
            }
        }

        Queue<int>[] sortFreq = new Queue<int>[2001];
        foreach (var pair in freqDict)
        {
            Console.WriteLine("hi 2");
            if (sortFreq[pair.Value] == null)
            {
                sortFreq[pair.Value] = new Queue<int>();
            }
            sortFreq[pair.Value].Enqueue(pair.Key);
        }

        for (int i = sortFreq.Length - 1; i >= 0; i--)
        {
            Console.WriteLine("hi 3");
            if (sortFreq[i] == null) continue;
            while (sortFreq[i].Count > 0)
            {
                if (k == 0) break;
                Console.WriteLine(result.Length - k);
                result[result.Length - k--] = sortFreq[i].Dequeue();
            }
            if (k == 0) break;
        }

        return result;
    }
}
