use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    tag: u32,
    value: String
}


#[derive(Debug)]
pub struct FieldMap (HashMap<u32, Field>);

impl FieldMap {
    fn new() -> Self {
        FieldMap (HashMap::new())
    }
    
    fn add(&mut self, tag: u32, val: String) {
        self.0.insert(tag, Field {tag, value: val});
    }
    
    fn get(&self, tag: u32) -> Option<&Field> {
        self.0.get(&tag)
    }
}

#[derive(Debug)]
pub struct Group (Vec<FieldMap>);



impl Group {
    fn new() -> Self {
        Group (Vec::with_capacity(2))
    }
    
    fn push(&mut self, val: FieldMap) {
        self.0.push(val)
    }
    
}

impl IntoIterator for Group {
    type Item = FieldMap;
    type IntoIter = ::std::vec::IntoIter<FieldMap>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Group {
    type Item = &'a FieldMap;
    type IntoIter = std::slice::Iter<'a, FieldMap>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Group {
    type Item = &'a mut FieldMap;
    type IntoIter = std::slice::IterMut<'a, FieldMap>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}


#[derive(Debug)]
pub struct FieldMapCollection (HashMap<u32, Group>);

impl FieldMapCollection {
    fn new() -> Self {
        FieldMapCollection(HashMap::new())
    }
    
    fn add(&mut self, tag: u32, val: FieldMap) {
        let field_vec = self.0.entry(tag).or_insert(Group::new());
        field_vec.push(val);
    }
    
    fn get(&self, tag: u32) -> Option<&Group> {
        self.0.get(&tag)
    }
}


#[derive(Debug)]
pub struct GroupInstance {
    delim: u32,
    fields: FieldMap,
    sub_grp: FieldMapCollection
}

impl GroupInstance {
    fn new(delim: u32) -> Self {
        Self {
            delim: delim,
            fields: FieldMap::new(),
            sub_grp: FieldMapCollection::new()
        }
    }
    
    fn add_field(&mut self, tag: u32, val: String) {
        self.fields.add(tag, val);
    }
    
    fn get_field(&self, tag: u32) -> Option<&Field> {
        self.fields.get(tag)
    }
    
    fn add_group(&mut self, tag: u32, val: FieldMap) {
        self.sub_grp.add(tag, val);
    }
    
    fn get_group(&self, tag: u32) -> Option<&Group> {
        self.sub_grp.get(tag)
    }
}


pub struct Message {
    fields: FieldMap,
    group: FieldMapCollection
}

impl Message {
    fn new() -> Self {
        Self {
            fields: FieldMap::new(),
            group: FieldMapCollection::new()
        }
    }
    
    fn add_field(&mut self, tag: u32, val: String) {
        self.fields.add(tag, val);
    }
    
    fn get_field(&self, tag: u32) -> Option<&Field> {
        self.fields.get(tag)
    }
    
    fn add_group(&mut self, tag: u32, val: FieldMap) {
        self.group.add(tag, val);
    }
    
    fn get_group(&self, tag: u32) -> Option<&Group> {
        self.group.get(tag)
    }
}

#[allow(unused_variables)]
fn main() {
    let mut grp_instance1 = FieldMap::new();
    grp_instance1.add(264, "gaurav".to_string());
    grp_instance1.add(267, "1.8".to_string());
    
    let mut grp = GroupInstance::new(264);
    grp.add_field(1, "account".to_string());
    grp.add_field(2, "subaccount".to_string());
    grp.add_group(2, grp_instance1);
    
    let mut grp_instance2 = FieldMap::new();
    grp_instance2.add(264, "tatke".to_string());
    grp_instance2.add(267, "1.9".to_string());
    grp.add_group(2, grp_instance2);
    
    println!("GroupInstance field 1 = {:?}, GroupInstance field 2 = {:?}", grp.get_field(1), grp.get_field(2));
    match grp.get_group(2) {
        Some(v) => {
            for group in v.into_iter() {
                println!("GroupInstance = {:?}", group);
            }
        }
        None => println!("No GroupInstance found")
    }
    
    
    let mut new_order_single = Message::new();
    new_order_single.add_field(35, "D".to_string());
    new_order_single.add_field(44, "1.188".to_string());
    new_order_single.add_field(263, "2".to_string());
    
    
}