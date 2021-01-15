
pub struct SessionIterator<I:Clone,It: Iterator<Item =I>>{
    inner_iter: It,
    session_stack: Vec<Vec<I>>,
    prev_collection: Vec<Vec<I>>
}

impl<I:Clone,It:Iterator<Item =I>> SessionIterator<I,It> {

    pub fn new(inner_iter:It)->Self{
        SessionIterator {inner_iter,session_stack:Vec::new(),prev_collection:Vec::new()}
    }

    pub fn new_session(&mut self)->usize{
        self.session_stack.push(Vec::new());
        self.session_stack.len()
    }
    pub fn commit_session(&mut self)->usize{
        self.session_stack.pop();
        self.session_stack.len()
    }
    pub fn revert_session(&mut self){
        if let Some(v) = self.session_stack.pop(){
            self.prev_collection.push(v.into_iter().rev().collect());
        }
    }
}

impl<I: Clone,It:Iterator<Item =I>> Iterator for SessionIterator<I,It>{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last_collected = None;

        while let Some(v) = self.prev_collection.last_mut(){
            if v.len() > 0 {
                last_collected = v.pop();
                break;
            }else{
                self.prev_collection.pop();
            }
        }

        if let None = last_collected{
            last_collected = self.inner_iter.next();
        }

        if let Some(ref last) = last_collected{
            if let Some(last_session) = self.session_stack.last_mut(){
                last_session.push(last.clone());
            }
        }
        last_collected
    }
}

pub trait ToSessionIterator<I: Clone> : Iterator<Item =I> + Sized{
    fn session(self)->SessionIterator<I,Self>;
}

impl<I:Clone,T:Sized + Iterator<Item =I>> ToSessionIterator<I> for T {
    fn session(self)->SessionIterator<I,Self> {
        SessionIterator::new(self)
    }
}