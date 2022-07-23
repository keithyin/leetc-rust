use std::collections::{HashMap, HashSet};

struct UnionFind {
    tree: HashMap<i32, i32>,
    counter: HashMap<i32, usize>
}

impl UnionFind {
    pub fn new() -> Self {
        UnionFind{tree: HashMap::new(), counter: HashMap::new()}
    }

    pub fn root(&mut self, mut item: i32) -> i32 {
        return loop {
            let father = self.tree[&item];

            if father == item {
                break item
            }
            let father_of_father = self.tree[&father];
            self.tree.insert(item, father_of_father);
            item = father;
        }
    }

    pub fn union(&mut self, record: &Vec<i32>) {
        let item1 = record[0];
        let item2 = record[1];
        let connected = record[2];

        if !self.tree.contains_key(&item1) {
            self.tree.insert(item1, item1);
            self.counter.insert(item1, 1);
        }
        if !self.counter.contains_key(&item2) {
            self.tree.insert(item2, item2);
            self.counter.insert(item2, 1);
        }

        if connected == 1 {
            let item1_root = self.root(item1);
            let item2_root = self.root(item2);

            let count1 = self.counter[&item1_root];
            let count2 = self.counter[&item2_root];

            let tot_num = count2 + count1;

            if count1 < count2 {
                self.tree.insert(item1_root, item2_root);
                self.counter.insert(item2_root, tot_num);
            } else {
                self.tree.insert(item2_root, item1_root);
                self.counter.insert(item1_root, tot_num);
            }

        }
    }

    pub fn num_root(&self) -> i32 {
        let mut root_set = HashSet::new();
        let fathers:Vec<i32> = self.tree.values().map(|x| *x).collect();
        for v in fathers.iter() {
            root_set.insert(*v);
        }
        return root_set.len() as i32;
    }
}


pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut union_find = UnionFind::new();
    let n = is_connected.len();
    for row in 0..n {
        for col in 0..n {
            union_find.union(&vec![row as i32, col as i32, is_connected[row][col]]);
        }
    }

    union_find.num_root()

}