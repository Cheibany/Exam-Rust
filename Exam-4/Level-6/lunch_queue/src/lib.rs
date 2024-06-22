#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}
pub type Link = Option<Box<Person>>;
#[derive(Debug,PartialEq)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link
}
impl Queue {
    pub fn new() -> Queue {
        Queue {node : None}
    }
    
    pub fn add(&mut self, name: String, discount: i32) {
        let add_element=Box::new(Person {discount,name,next_person:self.node.take()});
        self.node=Some(add_element)
    }
    pub fn invert_queue(&mut self) {
        let mut prev =None;
        let mut current=self.node.take();
        while let Some(mut person) = current{
            let next = person.next_person.take();
            person.next_person=prev.take();
            prev=Some(person);
            current=next;
        }
        self.node=prev
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        self.invert_queue();
        if let Some(person)=self.node.take(){
            self.node=person.next_person;
            self.invert_queue();
            return Some((person.name.clone(),person.discount.clone()))
        }
        None
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut current = &self.node;
        while let Some(person)=current{
            if person.name==name{
                return Some((person.name.clone(),person.discount.clone()))
            }
            current=&person.next_person;
        }
        None
    }
}