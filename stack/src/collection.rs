
pub struct Store<T>{
    goods:VecVar<T>    
}

impl<T> Store<T>  {
  
  pub fn new() -> Self {
    Store{ 
        goods: VecVar::new()
    }
    
  }
  pub fn insert(&mut self,element:T){
    self.goods.push(element);
  }   

  pub fn get(&mut self)->Option<T>{
    self.goods.pop()
  }
  
  pub fn is_empty(&self) -> bool {
      self.goods.is_empty()
  }
}
struct VecVar<T> {
    item: Vec<T>,
  }
  impl<T> VecVar<T> {
    pub fn new() -> Self {
        VecVar { item: Vec::new() }
    }
  
    pub fn push(&mut self, element: T) {
      self.item.push(element);
    }
  
    pub fn pop(&mut self) -> Option<T> {
      self.item.pop()
    }
  
    pub fn is_empty(&self) -> bool {
      self.item.is_empty()
    }
  }