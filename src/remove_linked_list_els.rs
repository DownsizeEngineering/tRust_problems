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

pub fn run() -> String {

    let mut list = ListNode::new(1);
    list.next = Some(Box::new(ListNode::new(2)));

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut new_head = head;
        new_head = loop {
            if let Some(node) = &new_head {
                if node.val != val { break new_head; }
                new_head = node.next.clone();
            }
            else { break None; }
        };

        let mut node_ptr = &mut new_head;
        while node_ptr != &None {
            if let Some(node) = &mut node_ptr {
                if let Some(next) = &node.next {
                    if next.val == val {
                        node.next = next.next.clone();
                    }
                }
                *node_ptr = node.next.clone();
            } 
        }


        new_head
    }

    pub fn print_list (list: Option<Box<ListNode>>) -> String {
        let mut list = list;
        let mut output = String::new();

        while list != None {
            if let Some(node_ptr) = list {
                let node = *node_ptr;
                output += &format!("{:?}", node);
                output += ", ";
                list = node.next;
            }
        }
        output
    }

    print_list(remove_elements(Some(Box::new(list)), 2))
}