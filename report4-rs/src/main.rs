use std::vec::Vec;
#[macro_use()]
use serde::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node<T> where T:Default{
    identifier:String,
    children: Vec<Node<T>>,
    content:T
}

impl<T> Node<T> where T:Default{
    pub fn new(identifier:&str)->Node<T> {
        Node::<T> {
            identifier: identifier.to_owned(),
            children: Vec::new(),
            content: Default::default()
        }
    }
    fn new_value<V:Into<T>>(identifier:&str, value:V) -> Node<T>{
        Node::<T> {
            identifier: identifier.to_owned(),
            children: Vec::new(),
            content: value.into()
        }        
    }
    fn append<V:Into<T>>(&mut self, identifier:&str, value:V) -> &mut Node<T>{
        self.children.push(Self::new_value(identifier, value));
        self.children.last_mut().unwrap()
    }
    fn prepend<V:Into<T>>(&mut self, identifier:&str, value:V) -> &mut Node<T>{
        self.children.insert(0,Self::new_value(identifier, value));
        self.children.get_mut(0).unwrap()
    }
    fn local(&mut self, identifier:&str) -> Option<&mut Node<T>>{
        self.children.iter_mut().filter(|x| x.identifier == identifier).next()
    }
    fn get(&mut self, identifier:&str) -> Option<&mut Node<T>>{
        
        if self.identifier == identifier{
            return Some(self);
        }
        for item in self.children.iter_mut(){
            let x = item.get(identifier);
            if x.is_some(){
                return x;
            }
        }
        return None;
    }
    fn find(&mut self, identifier:&str) -> Option<(usize, &mut Node<T>, bool)>{
        if self.identifier == identifier{
            return Some((0, self, false));
        }
        else{
            for (i,item) in self.children.iter_mut().enumerate(){
                if item.identifier == identifier{
                    return Some((i, self, true));
                }
            }
            for item in self.children.iter_mut(){
                let x = item.find(identifier);
                if x.is_some(){
                    return x;
                }
            }
        }
        return None;
    }

    fn after<V:Into<T>>(&mut self, place_identifier:&str, identifier:&str, value:V) -> Option<&mut Node<T>>{
        self.find(place_identifier).map(
            |(i,x,b)|
            if b{
                x.children.insert(i+1,Self::new_value(identifier, value));
                x.children.get_mut(i+1).unwrap()
            }
            else{
                x.append(identifier, value)
            }
        )
    }

    fn before<V:Into<T>>(&mut self, place_identifier:&str, identifier:&str, value:V) -> Option<&mut Node<T>>{
        self.find(place_identifier).map(
            |(i,x,b)|
            if b{
                x.children.insert(i,Self::new_value(identifier, value));
                x.children.get_mut(i).unwrap()
            }
            else{
                x.prepend(identifier, value)
            }
        )
    }
    fn replace<V:Into<T>>(&mut self, place_identifier:&str, identifier:&str, value:V) -> Option<&mut Node<T>>{
        self.find(place_identifier).filter(|(_,_,b)| *b).map(
            |(i,x,b)|{
                x.children[i] = Self::new_value(identifier, value);
                x.children.get_mut(i).unwrap()
            }
        )
    }

}
/*
pub trait NodeValue<T,V> where T:Default{
    fn new_value(identifier:&str, value:V) -> Node<T>;
}

impl<T,V> NodeValue<T,V> for Node<T> where T:Default + From<V>{
    fn new_value(identifier:&str, value:V) -> Node<T>{
        Node::<T> {
            identifier: identifier.to_owned(),
            children: Vec::new(),
            content: value.into()
        }        
    }
}
*/
fn main(){
    let node = Node::<String>{
        identifier:"hello".to_owned(),
        children: vec![],
        content:"Hello!".to_owned()
    
    };
    let node = Node::<String>::new("hello");
    let mut node = Node::<String>::new_value("hello", "Hello!");
    node.append("hello1", "Hello").content+=" world!";
    node.prepend("aaa","aaa").content+=" AAA";
    node.local("hello1").unwrap().content+=" ***";
    node.local("hello1").unwrap().append("deep","Deep");
    assert!(node.local("deep").is_none());
    node.get("deep").unwrap().content+="!!!";
    node.after("aaa", "cc", "CC");
    node.replace("cc", "ccc", "CCC");
    node.before("ccc", "bbb", "BBB");

    let serialized = serde_json::to_string(&node).unwrap();
    let serialized = serde_yaml::to_string(&node).unwrap();
    println!("Node:{:?}",node);
    println!("Serialized:\n{}",serialized);
 }
