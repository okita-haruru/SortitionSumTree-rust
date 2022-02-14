use std::collections::HashMap;
type TypeAddress = u128;
type TypeKey = u128;
struct SortitionSumTree {
    K: u128,
    stack: Vec<u128>,
    nodes: Vec<u128>,
    IDsToNodeIndexes: HashMap<TypeAddress,u128>,
    nodeIndexesToIDs: HashMap<u128,TypeAddress>,
}
impl SortitionSumTree{
    pub fn new(_k:u128)->SortitionSumTree{
        SortitionSumTree{
            K:_k,
            stack:Vec::new(),
            nodes:Vec::new(),
            IDsToNodeIndexes:HashMap::new(),
            nodeIndexesToIDs:HashMap::new(),
        }
    }
}
struct SortitionSumTrees {
   sortitionSumTrees: HashMap<TypeKey,SortitionSumTree>,
}
impl SortitionSumTrees{
    /**
     *  @dev Create a sortition sum tree with a key.
     *  @param _key The key of the new tree.
     *  @param _K The max number of children for each node in the new tree.
     */
    pub fn createTree(&mut self,_key:TypeKey,_K:u128){
        let mut tree: SortitionSumTree = SortitionSumTree::new(_K);
        tree.nodes.push(0);
        self.sortitionSumTrees.insert(_key, tree);
    }

      /**
     *  @dev Update the parents of a node until root.
     *  @param _key The key of the tree to update.
     *  @param _treeIndex The index of the node to start from.
     *  @param _plusOrMinus Wether to add (true) or substract (false).
     *  @param _value The value to add or substract.
     */
    pub fn updateParents(&mut self,_key:TypeKey,_treeIndex:u128,_plusOrMinus:bool,_value:u128){
        if let Some(tree)=self.sortitionSumTrees.get_mut(&_key){
            let mut parentIndex=_treeIndex;
            while(parentIndex!=0){
                parentIndex = (parentIndex - 1) / tree.K;
                tree.nodes[parentIndex as usize] = if _plusOrMinus {tree.nodes[parentIndex as usize] + _value} else {tree.nodes[parentIndex as usize] - _value};
            }
        }
    }

     /**
     *  @dev Set a value of an address in a tree.
     *  @param _key The key of the tree.
     *  @param _value The new value.
     *  @param _ID The ID of the value.
     *  `O(log_k(n))` where
     *  `k` is the maximum number of childs per node in the tree,
     *   and `n` is the maximum number of nodes ever appended.
     */
    pub fn set(&mut self,_key:TypeKey,_value:u128,_ID:TypeAddress){
        if let Some(tree)=self.sortitionSumTrees.get_mut(&_key){
            if let Some(_treeIndex)=tree.IDsToNodeIndexes.get_mut(&_ID){//node exist
                let treeIndex=_treeIndex.clone();
                if (_value == 0) {//new value==0
                    //remove
                    let value = tree.nodes[treeIndex as usize];
                    tree.nodes[treeIndex.clone() as usize] = 0;
                    tree.stack.push(treeIndex);
                    tree.nodeIndexesToIDs.remove(&treeIndex);
                    let y=treeIndex;
                    tree.IDsToNodeIndexes.remove(&_ID);
                    self.updateParents( _key, treeIndex, false, value);
                } else if (_value != tree.nodes[treeIndex as usize]) { // New value,and!=0
                    // Set.
                    let plusOrMinus = tree.nodes[treeIndex as usize] <= _value;
                    let plusOrMinusValue:u128 = if plusOrMinus {_value - tree.nodes[treeIndex.clone() as usize]} else {tree.nodes[treeIndex.clone() as usize] - _value};
                    tree.nodes[treeIndex as usize] = _value;
                    self.updateParents(_key, treeIndex, plusOrMinus, plusOrMinusValue);
                }   
            }else {
                if (_value != 0) {//node not exist
                    let mut treeIndex:u128=0;
                    if (tree.stack.len() == 0) {//no vacant node
                        treeIndex = tree.nodes.len() as u128;
                        tree.nodes.push(_value);
                        if (treeIndex != 1 && (treeIndex - 1) % tree.K == 0) {//is the first node of a layer
                            //move the parent  down
                            let parentIndex = treeIndex / tree.K;
                            let parentID : TypeAddress= tree.nodeIndexesToIDs[&parentIndex];
                            let newIndex:u128= treeIndex + 1;
                            tree.nodes.push(tree.nodes[parentIndex as usize]);
                            tree.nodeIndexesToIDs.remove(&parentIndex);
                            tree.IDsToNodeIndexes.insert(parentID,newIndex);
                            tree.nodeIndexesToIDs.insert(newIndex,parentID);
                        }
                    } else {//vacant node
                        treeIndex = tree.stack[tree.stack.len() - 1];
                        tree.stack.pop();
                        tree.nodes[treeIndex as usize] = _value;
                    }
                    tree.IDsToNodeIndexes.insert(_ID,treeIndex);
                    tree.nodeIndexesToIDs.insert(treeIndex,_ID);    
                    //updateParents( _key, treeIndex, true, _value);
                    self.updateParents(_key, treeIndex, true, _value);
                }
            }
        }
    }

    /** @dev Gets a specified ID's associated value.
     *  @param _key The key of the tree.
     *  @param _ID The ID of the value.
     *  @return value The associated value.
     */
    pub fn stakeOf(&self,_key:TypeKey,_ID:TypeAddress)->u128{
        if let Some(tree)=self.sortitionSumTrees.get(&_key){
            if let Some(treeIndex)=tree.IDsToNodeIndexes.get(&_ID){
                return tree.nodes[*treeIndex as usize];
            }else {
                return 0;
            }
        }else {
            return 0;
        }
    }

    /**
     *  @dev Draw an ID from a tree using a number. Note that this function reverts if the sum of all values in the tree is 0.
     *  @param _key The key of the tree.
     *  @param _drawnNumber The drawn number.
     *  @return ID The drawn ID.
     *  `O(k * log_k(n))` where
     *  `k` is the maximum number of childs per node in the tree,
     *   and `n` is the maximum number of nodes ever appended.
     */
    pub fn draw(&self,_key:TypeKey,_drawnNumber:u128)->TypeAddress{
        if let Some(tree)=self.sortitionSumTrees.get(&_key){
            let mut treeIndex:u128=0;
            let mut currentDrawnNumber=_drawnNumber%tree.nodes[0];
            while((tree.K*treeIndex)+1<tree.nodes.len() as u128){
                for i in (1..=tree.K){
                    let nodeIndex:u128=(tree.K*treeIndex)+i;
                    let nodeValue:u128=tree.nodes[nodeIndex as usize];
                    if currentDrawnNumber>=nodeValue {
                        currentDrawnNumber=currentDrawnNumber-nodeValue;
                    }else{
                        treeIndex=nodeIndex;
                        break;
                    }
                }
            }
            return tree.nodeIndexesToIDs[&treeIndex];
        }else {
            return 0;
        }
    }

    /**
     *  @dev Query the leaves of a tree. Note that if `startIndex == 0`, the tree is empty and the root node will be returned.
     *  @param _key The key of the tree to get the leaves from.
     *  @param _cursor The pagination cursor.
     *  @param _count The number of items to return.
     *  @return startIndex The index at which leaves start.
     *  @return values The values of the returned leaves.
     *  @return hasMore Whether there are more for pagination.
     *  `O(n)` where
     *  `n` is the maximum number of nodes ever appended.
     */
    pub fn queryLeaves(&self,_key:TypeKey,_cursor:u128,_count:u128)->(u128,Vec<u128>,bool){
        let mut startIndex:u128=0;
        let mut values:Vec<u128>=Vec::new();
        let mut hasMore:bool=false;
        if let Some(tree)=self.sortitionSumTrees.get(&_key){
            for i in 1..=tree.nodes.len() as u128{
                if (tree.K*i)+1>=tree.nodes.len() as u128{
                    startIndex=i;
                    break;
                }
            }
            let loopStartIndex=startIndex+_cursor;
            for j in loopStartIndex..tree.nodes.len() as u128{
                if values.len() < _count as usize{
                    values.push(tree.nodes[j as usize]);
                }else{
                    hasMore=true;
                    break;
                }
            }
        }
        return(startIndex,values,hasMore);
    }
}

