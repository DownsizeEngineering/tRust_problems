#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn run() -> Option<Box<ListNode>> {

    let mut list = ListNode::new(1);
    list.next = Some(Box::new(ListNode::new(1)));
    list = ListNode{val: 2, next:Some(Box::new(list))};

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut new_head = head;
        new_head = loop {
            if let Some(node) = new_head {
                if node.val != val { break Some(node); }
                new_head = node.next;
            }
            else { return None; }
        };

        let mut node_ptr = &mut new_head;

        while node_ptr.is_some() { 
            if let Some(node) = &mut node_ptr {
                loop {
                    if let Some(next) = &node.next {
                        if next.val == val {
                            node.next = next.next.clone();
                            continue;
                        }
                    }
                    break;
                }
            }
                node_ptr = &mut node_ptr.as_mut().unwrap().next;
        }


        new_head
    }

    remove_elements(Some(Box::new(list)), 1)
}