public class Solution {
    public int[] TwoSum(int[] nums, int target) {
        Dictionary<int, int> pairs = new Dictionary<int, int>();
        int[] result = {0, 0};

        for (int i = 0; i < nums.Length; i++)
        {
            if (pairs.ContainsKey(-nums[i]))
            {
                result = new int[] {pairs[-nums[i]], i};
                break;
            }
            else
            {
                pairs.Add(nums[i] - target, i);
            }
        }
        return result;
    }
}
