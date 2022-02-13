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
    pub fn createTree(&mut self,_key:TypeKey,_K:u128){
        let mut tree: SortitionSumTree = SortitionSumTree::new(_K);
        tree.nodes.push(0);
        self.sortitionSumTrees.insert(_key, tree);
    }
    pub fn updateParents(&mut self,_key:TypeKey,_treeIndex:u128,_plusOrMinus:bool,_value:u128){
        if let Some(tree)=self.sortitionSumTrees.get_mut(&_key){
            let mut parentIndex=_treeIndex;
            while(parentIndex!=0){
                parentIndex = (parentIndex - 1) / tree.K;
                tree.nodes[parentIndex as usize] = if _plusOrMinus {tree.nodes[parentIndex as usize] + _value} else {tree.nodes[parentIndex as usize] - _value};
            }
        }
    }
    pub fn set(&mut self,_key:TypeKey,_value:u128,_ID:TypeAddress){
        if let Some(tree)=self.sortitionSumTrees.get_mut(&_key){
            if let Some(_treeIndex)=tree.IDsToNodeIndexes.get_mut(&_ID){
                let treeIndex=_treeIndex.clone();
                if (_value == 0) {
                    let value = tree.nodes[treeIndex as usize];
                    tree.nodes[treeIndex.clone() as usize] = 0;
                    tree.stack.push(treeIndex);
                    tree.nodeIndexesToIDs.remove(&treeIndex);
                    let y=treeIndex;
                    tree.IDsToNodeIndexes.remove(&_ID);
    
                    self.updateParents( _key, treeIndex, false, value);
                } else if (_value != tree.nodes[treeIndex as usize]) { // New, non zero value.
                    // Set.
                    let plusOrMinus = tree.nodes[treeIndex as usize] <= _value;
                    let plusOrMinusValue:u128 = if plusOrMinus {_value - tree.nodes[treeIndex.clone() as usize]} else {tree.nodes[treeIndex.clone() as usize] - _value};
                    tree.nodes[treeIndex as usize] = _value;
                    //updateParents(self, _key, treeIndex, plusOrMinus, plusOrMinusValue);
                    self.updateParents(_key, treeIndex, plusOrMinus, plusOrMinusValue);
                }   
            }else {
                if (_value != 0) {
                    let mut treeIndex:u128=0;
                    if (tree.stack.len() == 0) {
                        treeIndex = tree.nodes.len() as u128;
                        tree.nodes.push(_value);
                        if (treeIndex != 1 && (treeIndex - 1) % tree.K == 0) {
                            let parentIndex = treeIndex / tree.K;
                            let parentID : TypeAddress= tree.nodeIndexesToIDs[&parentIndex];
                            let newIndex:u128= treeIndex + 1;
                            tree.nodes.push(tree.nodes[parentIndex as usize]);
                            tree.nodeIndexesToIDs.remove(&parentIndex);
                            tree.IDsToNodeIndexes.insert(parentID,newIndex);
                            tree.nodeIndexesToIDs.insert(newIndex,parentID);
                        }
                    } else {
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

