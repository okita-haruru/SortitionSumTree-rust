pub mod sortitionSumTree;

include!("sortitionSumTree.rs");
#[test]
    fn test1(){
        let mut trees: SortitionSumTrees = SortitionSumTrees {sortitionSumTrees:HashMap::new()};
        trees.createTree(1, 2);
        trees.set(1, 25, 1);
        trees.set(1, 25, 2);
        trees.set(1, 25, 3);
        trees.set(1, 25, 4);
        let tree=trees.sortitionSumTrees.get(&1);
        assert_eq!(tree.unwrap().nodes[0],100);
        assert_eq!(tree.unwrap().nodes[1],50);
        assert_eq!(tree.unwrap().nodes[2],50);
        assert_eq!(tree.unwrap().nodes[3],25);
        assert_eq!(tree.unwrap().nodes[4],25);
        assert_eq!(tree.unwrap().nodes[5],25);
        assert_eq!(tree.unwrap().nodes[6],25);
        assert_eq!( trees.draw(1, 35),1);
        assert_eq!(trees.stakeOf(1, 1),25);
        trees.set(1, 0, 1);
        assert_eq!(trees.stakeOf(1, 1),0);
     }
fn main() {}
