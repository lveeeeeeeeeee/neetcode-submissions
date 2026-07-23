public class Solution {
    public bool hasDuplicate(int[] nums) {
        Dictionary<int, int> counts = new Dictionary<int, int>();

        for (int i = 0; i < nums.Length; i++) {
            if (counts.ContainsKey(nums[i])) {
                if (counts[nums[i]] >= 1) {
                    return true;
                }
            } 
            else {
                counts.Add(nums[i], 1);
            }
        }
        return false;
    }
}