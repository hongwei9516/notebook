pub fn sort(tree: &mut [i32]) {
    build_heap(tree);
    let mut i = tree.len();
    while i > 0 {
        i -= 1;
        tree.swap(i, 0);
        heapify(tree, i, 0);
    }
}

fn build_heap(tree: &mut [i32]) {
    let last_node = tree.len() - 1;
    let parent = (last_node - 1) / 2;
    let mut i = parent + 1;
    while i > 0 {
        i -= 1;
        heapify(tree, tree.len(), i);
    }
}

// 数组只能使用usize索引，不然得自己实现index trait
// 所以使用index循环时，判断条件边界不能是 >=0 ，可能使用加1再减的方法
// while i > 0 {
//     i -= 1;
//     }
pub fn heapify(tree: &mut [i32], n: usize, i: usize) {
    let mut max = i;
    let c1 = 2 * i + 1;
    let c2 = 2 * i + 2;
    if c1 < n && tree[c1] > tree[max] {
        max = c1;
    }
    if c2 < n && tree[c2] > tree[max] {
        max = c2;
    }
    if max != i {
        tree.swap(i, max);
        heapify(tree, n, max);
    }
}
