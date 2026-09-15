// Definition for singly-linked list.
// public class ListNode {
//      public int val;
//      public ListNode next;
//      public ListNode(int val=0, ListNode next=null) {
//          this.val = val;
//          this.next = next;
//      }
//  }
//  

public class Solution {
    public ListNode ReverseList(ListNode head) {
        ListNode reversed = null;
        while (head != null) {
            ListNode curr = head;
            head = head.next;
            curr.next = reversed;
            reversed = curr;
        }
        return reversed;
    }
}
