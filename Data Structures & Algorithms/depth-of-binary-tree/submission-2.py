# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

from collections import deque

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        q = deque([(root, 1)])
        depth = 0
        while len(q) > 0:
            node, curr = q.pop()
            if node == None: break
            if node.left != None: 
                q.appendleft((node.left, curr+1))
            if node.right != None: 
                q.appendleft((node.right, curr+1))
            depth = max(depth, curr)

        return depth