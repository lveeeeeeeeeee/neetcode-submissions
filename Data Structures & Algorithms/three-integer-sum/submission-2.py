class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        # nums[k] = -(nums[j] + nums[i])
        # just do the two sum for all numbers between nums[0] and nums[len(nums)-1]
        # (the sorted array method)
        # also since we sort we can store the triples as tuples
        # and add them to the set while we construct an answer
        nums.sort()
        result = []
        i = 0
        while nums[i] <= 0 and i < len(nums) - 1:
            target = nums[i]
            if i > 0 and target == nums[i-1]: 
                i += 1
                continue
            left = i+1
            right = len(nums)-1
            while left < right:
                cmp = nums[left] + nums[right] + target
                if cmp == 0:
                    result.append([target, nums[left], nums[right]])
                    right -= 1
                    left += 1
                    while nums[left] == nums[left-1] and left < right:
                        left += 1
                elif cmp > 0:
                    right -= 1
                elif cmp < 0:
                    left += 1
            i += 1

        return result   
