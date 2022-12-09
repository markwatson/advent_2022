use slab_tree::*;
use std::{fmt, fs::read_to_string};

use regex::Regex;

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

fn parse_dirs(input: &str) -> (Tree<Item>, Vec<NodeId>) {
    let re_cd = Regex::new(r"^\$ cd (.+)$").unwrap();
    let re_file = Regex::new(r"^(\d+) (.+)$").unwrap();

    let root_item = Item {
        path: "FS".to_string(),
        is_file: false,
        size: 0,
    };

    let mut tree = TreeBuilder::new().with_root(root_item).build();
    let root_id = tree.root_id().expect("Root node not found");
    let mut current_item_id = root_id;
    let mut leaves = vec![];

    for line in input.lines() {
        let cd_match = re_cd.captures(line);
        if cd_match.is_some() {
            match cd_match.unwrap().get(1).unwrap().as_str().trim() {
                ".." => {
                    let current_item = tree
                        .get(current_item_id)
                        .expect("current item ID not found");
                    current_item_id = current_item
                        .parent()
                        .expect("No parent node found")
                        .node_id();
                }
                "/" => {
                    // Move to root
                    current_item_id = root_id;
                }
                matched_change => {
                    // Move to subdirectory
                    let new_item = Item {
                        path: matched_change.to_string(),
                        is_file: false,
                        size: 0,
                    };
                    let mut current_item = tree.get_mut(current_item_id).unwrap();
                    let added = current_item.append(new_item);
                    current_item_id = added.node_id();
                }
            }
            continue;
        }

        let file_match = re_file.captures(line);
        if file_match.is_some() {
            let matched = file_match.unwrap();
            let size = matched.get(1).unwrap().as_str().trim();
            let name = matched.get(2).unwrap().as_str().trim();

            // Move to subdirectory
            let new_file = Item {
                path: name.to_string(),
                is_file: true,
                size: size.parse().unwrap(),
            };
            let mut current_item = tree.get_mut(current_item_id).unwrap();
            let new_item = current_item.append(new_file);
            leaves.push(new_item.node_id());
        }
    }

    return (tree, leaves);
}

fn calculate_dir_sizes(mut tree: Tree<Item>, leaves: Vec<NodeId>) -> Tree<Item> {
    for file in leaves {
        let mut item = tree.get_mut(file).expect("file not found");
        let size = item.data().size;
        let mut parent = item.parent().unwrap().node_id();
        while tree.get(parent).is_some() {
            let mut parent_node = tree.get_mut(parent).unwrap();
            parent_node.data().size += size;
            let grandparent = parent_node.parent();
            if grandparent.is_none() {
                break;
            }
            parent = parent_node.parent().unwrap().node_id();
        }
    }
    return tree;
}

fn print_tree(tree: &Tree<Item>) {
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    println!("{}", s);
}

fn main() {
    let input = read_to_string("./data/day_7_test").expect("Error reading file");

    let (dirs, files) = parse_dirs(&input);
    print_tree(&dirs);

    let dirs_with_sizes = calculate_dir_sizes(dirs, files);

    print_tree(&dirs_with_sizes);

    // TODO: sum up the sizes of dirs, etc.
    //println!("{}", input);
}
