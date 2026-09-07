class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l = -1
        r = len(nums)
        m = l + ((r - l) // 2)
        while l < r-1:
            if nums[m] < target:
                l = m
            elif nums[m] > target:
                r = m
            else:
                return m
            m = l + ((r - l) // 2)
        if nums[m] != target:
            return -1
        return m