# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if root == None:
            return True
        stack = [root]
        v = {None: (0, 0)}
        max_iter = 0
        while len(stack) > 0 and max_iter < 100:
            max_iter += 1
            peek = stack[-1]
            if peek.left != None and not peek.left in v:
                stack.append(peek.left)
                # print(peek.left.val)
            elif peek.right != None and not peek.right in v:
                stack.append(peek.right)
                # print(peek.right.val)
            else:
                # print("popping yayy")
                peek = stack.pop()
                ll_h, lr_h = v[peek.left]
                rl_h, rr_h = v[peek.right]
                if not (-1 <= (max(ll_h, lr_h) - max(rl_h, rr_h)) <= 1):
                    return False
                v[peek] = (max(ll_h, lr_h) + 1, max(rl_h, rr_h) + 1)
        
        print(v[root][0], v[root][1])
        return 0 <= abs(v[root][0] - v[root][1]) <= 1
