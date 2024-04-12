use std::collections::HashMap;
#[derive(Debug)]
struct Contents{
    content:String,

}
fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents{content:"Books".to_owned()});
    lockers.insert(2, Contents{content:"Laptop".to_owned()});
    lockers.insert(3, Contents{content:"Bag".to_owned()});
    for(locker_number,content) in lockers.iter(){
        println!("Locker number: {:?} contains {:?}",locker_number,content.content);
    }
}
