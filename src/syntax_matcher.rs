
struct TokenTree<T>{
    children:Option<Box<Self>>,
    val:T
}