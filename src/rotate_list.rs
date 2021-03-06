impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut cur = &head;

        let mut length = 0;
        while let Some(next) = cur {
            cur = &next.next;
            length += 1;
        }
        // C 语言里很容易想到的一个高效做法是在这里将链表首尾相连, 然后再在第 length - k 个节点处分开
        // 然而在 Rust 里就题目所给的结构而言(个人认为)似乎做不到:
        // 如果存在这样一个链表, 那链表每一个节点的所有权都归属上一个节点的 .next 成员
        // 那么根本无法访问这个链表, 因为栈上没有变量拥有任何节点的所有权, 就更不可能有引用了
        // 换言之, 这样的玩意儿是应该立刻被 drop 掉的

        k %= length;
        if k != 0 {
            // 找到新的头结点
            let tail = cur;
            cur = &head;
            for _ in 0..(length - k) {
                cur = &cur.as_ref().unwrap().next;
            }

            // 问: 能干掉 unsafe 吗
            // 答: 不能
            unsafe {
                let cur = cur as *const _ as *mut Option<Box<ListNode>>;
                let tail = tail as *const _ as *mut Option<Box<ListNode>>;
                let mid = std::ptr::replace(cur, None);
                std::ptr::replace(tail, head);
                mid
            }
        } else {
            head
        }
    }

    pub fn rotate_right_safe(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut cur = &mut head;

        let mut length = 0;
        while let Some(next) = cur {
            cur = &mut next.next;
            length += 1;
        }

        k %= length;
        if k != 0 {
            // 找到新的头结点
            cur = &mut head;
            for _ in 0..(length - k) {
                cur = &mut cur.as_mut().unwrap().next;
            }
            let mut mid = std::mem::replace(cur, None);
            // 再次找到尾节点
            let mut tail = &mut mid;
            while let Some(next) = tail {
                tail = &mut next.next;
            }
            // 首尾相连
            std::mem::replace(tail, head);
            mid
        } else {
            head
        }
    }
}

use crate::ListNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::rotate_right(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![4, 5, 1, 2, 3]
        );

        assert_eq!(
            Solution::rotate_right(linkedlist![0, 1, 2], 4),
            linkedlist![2, 0, 1]
        );

        assert_eq!(
            Solution::rotate_right(linkedlist![], 0),
            linkedlist![]
        );
    }

    #[test]
    fn test_safe() {
        assert_eq!(
            Solution::rotate_right_safe(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![4, 5, 1, 2, 3]
        );

        assert_eq!(
            Solution::rotate_right_safe(linkedlist![0, 1, 2], 4),
            linkedlist![2, 0, 1]
        );

        assert_eq!(
            Solution::rotate_right_safe(linkedlist![], 0),
            linkedlist![]
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use crate::linkedlist;
    use super::Solution;
    use self::test::Bencher;

    #[bench]
    fn _unsafe(b: &mut Bencher) {
        b.iter(|| Solution::rotate_right(linkedlist![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5));
    }

    #[bench]
    fn safe(b: &mut Bencher) {
        b.iter(||  Solution::rotate_right_safe(linkedlist![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5));
    }
}
