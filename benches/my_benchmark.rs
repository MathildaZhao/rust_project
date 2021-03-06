use criterion::{black_box, criterion_group, criterion_main, Criterion};
use the_tree::avl_tree::{Node, Avl};
use the_tree::BST;


fn test_avl(tree_size: i32) {
    let mut root:Node<_> = Avl::generate_new_tree();
    for i in 0..tree_size {
        root.insert_node(i);
    }
    let search_threshold = 10 / tree_size;
    for i in 0..search_threshold {
        root.is_exist(i);
    }
}

fn test_bst(tree_size: i32){
    let mut root=BST::BST::new();
    for i in 0..tree_size{
        root.insert(i);
    }
    let search_threshold = 10/tree_size;
    for i in 0..search_threshold {
        root.find(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    //test bench
    // c.bench_function("AVL TREE", |b| b.iter(|| test_avl(black_box(10000))));
    for tree_size in [10000, 40000, 70000, 100000, 130000].iter() {
        c.bench_function("AVL TREE", |b| {
                b.iter(|| test_avl(black_box(*tree_size)))
        });
        //the bst will overflowed its stack in size 40000.
        // c.bench_function("BST TREE", |b| {
        //     b.iter(|| test_bst(black_box(*tree_size)))
        // });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);