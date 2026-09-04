class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        right = len(numbers) - 1
        left = 0
        while (left != right-1):
            cmp = numbers[left] + numbers[right] - target
            if cmp == 0:
                return [left + 1, right + 1]
            if cmp > 0:
                right -= 1
            if cmp < 0:
                left += 1

        return [left + 1, right + 1]