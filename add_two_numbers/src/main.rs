fn main() {
    println!("Add two numbers");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

trait AddTwoNumbers {
    fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

struct Solution;

impl AddTwoNumbers for Solution {
    fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut result: Option<Box<ListNode>> = None;
        let mut head = &mut result;

        let (mut p1,  mut p2) = (l1, l2);
        while p1.is_some() || p2.is_some() {
            let mut sum = carry;

            if let Some(node) = p1 {
                sum += node.val;
                p1 = node.next;
            }

            if let Some(node) = p2 {
                sum += node.val;
                p2 = node.next;
            }

            if sum > 9 {
                carry = 1;
                sum = sum - 10;
            } else {
                carry = 0;
            }

            *head = Some(Box::new(ListNode::new(sum)));
            head = &mut head.as_mut().unwrap().next;
        }

        if carry > 0 {
            *head = Some(Box::new(ListNode::new(carry)));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution_1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        assert_eq!(
            Solution::solution(l1, l2),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );
    }

    #[test]
    fn solution_2() {
        let l1 = Some(Box::new(ListNode { val: 0, next: None }));
        let l2 = Some(Box::new(ListNode { val: 0, next: None }));
        assert_eq!(
            Solution::solution(l1, l2),
            Some(Box::new(ListNode { val: 0, next: None }))
        );
    }

    #[test]
    fn solution_3() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        assert_eq!(
            Solution::solution(l1, l2),
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 0,
                                        next: Some(Box::new(ListNode { val: 1, next: None }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        );
    }
}
