use std::collections::HashMap;

fn main() {
    let x:i64 = 1211111123131;
    let a:i64 = 4561212112122;
    another_function();
    let resutl:i64 = add(x,a);
    println!("{}",x);
    println!("{}",resutl);
    
}

struct Todo {
    map: HashMap<String,bool>
}

impl Todo {
    fn insert(&mut self, key:String){
        // 在我们的 map 中新增一个新的元素。
        // 我们默认将其状态值设置为 true
        self.map.insert(key,true);
    }
}

fn another_function() {
    println!("set")
}



fn add(a:i64,b:i64) -> i64 {
    return a + b
}