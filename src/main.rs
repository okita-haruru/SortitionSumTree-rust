pub mod sortitionSumTree;

include!("sortitionSumTree.rs");

fn displayMenu()->u128{
    println!("Enter a number to operate the trees:");
    println!("[1]Create a tree with a key");
    println!("[2]Set the value of an address");
    println!("[3]Display a tree's node");
    println!("[4]Draw with a number");
    println!("[5]Get the balance of an address");
    let mut n:u128 =0;
    
    return n;

}
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
        assert_eq!( trees.draw(1, 35),1 as TypeAddress);
        assert_eq!(trees.stakeOf(1, 1 as TypeAddress),25);
        trees.set(1, 0, 1);
        assert_eq!(trees.stakeOf(1, 1 as TypeAddress),0);
     }
fn main() {
    let mut trees: SortitionSumTrees = SortitionSumTrees {sortitionSumTrees:HashMap::new()};
    trees.createTree(1, 2);
    trees.createTree(2, 2);
    trees.set(1, 25, 1);
    trees.set(1, 25, 2);
    trees.set(1, 50, 1);

    let mut tree=trees.sortitionSumTrees.get(&1);
    println!("{}",tree.unwrap().nodes[0]);
}
