public class Solution {
    public ListNode ReverseKGroup(ListNode head, int k) {
        var initrun = head;
        int length = 0;
        while (initrun != null) {
            initrun = initrun.next;
            ++length;
        }
        if (length < k) {
            return head;
        }
        Stack<ListNode> stack = new Stack<ListNode>();
        int iterations = 0;
        ListNode prev = null;
        ListNode curr = null;
        ListNode trail = null;
        while (head != null) {
            if (iterations % k == 0) {
                int runLen = 0;
                var run = head;
                while (run != null && runLen < k) {
                    run = run.next;
                    runLen += 1;
                }
                if (runLen < k) {
                    iterations += runLen;
                    trail = head;
                    break;
                }
            }
            curr = head;
            head = head.next;
            curr.next = prev;
            prev = curr;
            if (++iterations % k == 0) {
                stack.Push(prev);
                prev = null;
            }
        }
        ListNode nextStitch = (iterations % k == 0) ? null : trail;
        ListNode prevStitch = null;
        while (stack.Count != 0) {
            var node = stack.Pop();
            prevStitch = node;
            while (node != null) {
                if (node.next != null) {
                    node = node.next;
                } else {
                    break;
                }
            }
            node.next = nextStitch;
            nextStitch = prevStitch;
        }
        return prevStitch;
    }
}
