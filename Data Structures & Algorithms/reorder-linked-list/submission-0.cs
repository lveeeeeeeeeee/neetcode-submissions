/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

public class Solution {
    public void ReorderList(ListNode head) {
        var to_rev = head;
        ListNode rev = null;
        var fast = head;
        var slow = head;
        int counter = 1;
        while (fast != null) {
            if (counter == 0) {
                counter = 1;
                slow = slow.next;
            }
            else {
                counter -= 1;
            }
            fast = fast.next;
        }
        var swap = slow.next;
        slow.next = null;
        slow = swap;
        // Console.WriteLine(slow.val + " " + prev_slow.val);
        // reverse second 
        while (slow != null) {
            var curr = slow;
            slow = slow.next;
            curr.next = rev;
            rev = curr;
        }
        int step = 0;
        var iter = head;
        while (iter != null) {
            if ((step & 1) == 0) {
                if (rev == null) {
                    break;
                }
                var append = iter.next;
                iter.next = rev;
                rev = rev.next;
                iter.next.next = append;
            }
            iter = iter.next;
            step += 1;
        }
    }
}
