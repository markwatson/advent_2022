
use std::{fmt, fs::read_to_string};

#[derive(Debug, Clone)]
struct Item {
    path: String,
    is_file: bool,
    size: i32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Item(path: {}, is_file: {}, size: {})",
            self.path, self.is_file, self.size
        )
    }
}

// Tree
// #[derive(Debug)]
// struct TreeNode {
//     value: Option<Item>,
//     children: Vec<Rc<RefCell<TreeNode>>>,
//     parent: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     pub fn new() -> TreeNode {
//         return TreeNode {
//             value: None,
//             children: vec![],
//             parent: None,
//         };
//     }

//     pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
//         self.children.push(new_node);
//     }

//     pub fn print(&self) -> String {
//         if let Some(value) = self.value {
//             return value.to_string();
//         } else {
//             for child in &self.children {
//                 let thing
//             }
//             return String::from("[")
//                 + &self
//                     .children
//                     .iter()
//                     .map(|tn| {
//                         let thing = tn.borrow().borrow() as &TreeNode;
//                         thing.print()
//                     })
//                     .collect::<Vec<String>>()
//                     .join(",")
//                 + "]";
//         }
//     }
// }

// fn parse_dirs(input: &str) -> &Item {
//     let re_cd = Regex::new(r"^\$ cd (.+)$").unwrap();

//     let root = &Item {
//         path: "/".to_string(),
//         is_file: false,
//         size: 0,
//     };
//     return root;
//     // let mut current_item = root.clone();
//     // for line in input.lines() {
//     //     let cd_match = re_cd.captures(line);
//     //     if cd_match.is_some() {
//     //         match cd_match.unwrap().get(1).unwrap().as_str().trim() {
//     //             ".." => {
//     //                 // Move up one level
//     //                 match &current_item.parent {
//     //                     Some(item) => {
//     //                         current_item = item.clone();
//     //                     }
//     //                     None => {
//     //                         continue;
//     //                     }
//     //                 }
//     //             }
//     //             "/" => {
//     //                 // Move to root
//     //                 current_item = root.clone();
//     //             }
//     //             matched_change => {
//     //                 // Move to subdirectory
//     //                 let new_item = Rc::new(Item {
//     //                     path: matched_change.to_string(),
//     //                     is_file: false,
//     //                     size: 0,
//     //                     parent: Some(current_item),
//     //                     children: Rc::new(RefCell::new(vec![])),
//     //                 });
//     //                 current_item.children.push(new_item.clone());
//     //                 current_item = new_item.clone();
//     //             }
//     //         }
//     //         continue;
//     //     }
//     // }

//     // return root;
// }

fn main() {
    let input = read_to_string("./data/day_7_test").expect("Error reading file");

    // let dirs = parse_dirs(&input);
    // println!("{:?}", dirs);

    println!("{}", input);
}
