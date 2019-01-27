use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
pub struct Group {
    delim: u32,
    group_vec: Vec<GroupInstance>
}



impl Group {
    fn new(delim: u32, capacity: usize) -> Self {
        Self {
            delim,
            group_vec: Vec::with_capacity(capacity)
        }
    }
    
    fn push(&mut self, val: GroupInstance) {
        self.group_vec.push(val)
    }
    
}

impl Index<usize> for Group {
    type Output = GroupInstance;
    
    fn index(&self, idx: usize) -> &Self::Output {
        self.group_vec.index(idx)
    }
}

impl IndexMut<usize> for Group {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.group_vec.index_mut(idx)
    }
}

impl IntoIterator for Group {
    type Item = GroupInstance;
    type IntoIter = ::std::vec::IntoIter<GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.into_iter()
    }
}

impl<'a> IntoIterator for &'a Group {
    type Item = &'a GroupInstance;
    type IntoIter = std::slice::Iter<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.iter()
    }
}

impl<'a> IntoIterator for &'a mut Group {
    type Item = &'a mut GroupInstance;
    type IntoIter = std::slice::IterMut<'a, GroupInstance>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.group_vec.iter_mut()
    }
}



#[derive(Debug)]
pub struct GroupInstance {
    grp_fields: FieldMap,
    sub_grp: HashMap<u32, Group>
}

impl GroupInstance {
    fn new() -> Self {
        Self {
            grp_fields: FieldMap::new(),
            sub_grp: HashMap::new()
        }
    }
    
    fn add_field(&mut self, tag: u32, val: String) {
        self.grp_fields.add(tag, val);
    }
    
    fn get_field(&self, tag: u32) -> Option<&Field> {
        self.grp_fields.get(tag)
    }
    
    fn set_group(&mut self, tag: u32, val: u32) -> &mut Group {
        self.add_field(tag, val.to_string());
        let mut group = Group::new(tag, val as usize);
        for _i in 0..val {
            let sub_group = GroupInstance::new();
            group.push(sub_group);
        }
        self.sub_grp.entry(tag).or_insert(group)
    }
    
    fn get_group(&self, tag: u32) -> std::result::Result<&Group, &'static str> {
        match self.sub_grp.get(&tag) {
            None => Err("No repeating groups"),
            Some(grp) => Ok(grp),
        }
    }
}


pub struct Message {
    fields: FieldMap,
    group: HashMap<u32, Group>
}

// impl Message {
//     fn new() -> Self {
//         Self {
//             fields: FieldMap::new(),
//             group: FieldMapCollection::new()
//         }
//     }
    
//     fn add_field(&mut self, tag: u32, val: String) {
//         self.fields.add(tag, val);
//     }
    
//     fn get_field(&self, tag: u32) -> Option<&Field> {
//         self.fields.get(tag)
//     }
    
//     fn add_group(&mut self, tag: u32, val: FieldMap) {
//         self.group.add(tag, val);
//     }
    
//     fn get_group(&self, tag: u32) -> Option<&Group> {
//         self.group.get(tag)
//     }
// }

#[allow(unused_variables)]
fn main() {
    let mut grp_inst1 = GroupInstance::new();
    grp_inst1.add_field(35, "A".to_string());
    grp_inst1.add_field(40, "D".to_string());
    
    {
        let group = grp_inst1.set_group(263, 2);
    
        group[0].add_field(264, "abx".to_string());
        group[0].add_field(267, "aby".to_string());
    
        {
            let sub_group = group[0].set_group(72, 2);
            sub_group[0].add_field(78, "leg1 id1".to_string());
            sub_group[0].add_field(77, "leg1 ref1".to_string());
            sub_group[1].add_field(78, "leg1 id2".to_string());
            sub_group[1].add_field(77, "leg1 ref2".to_string());
        }
    
        group[1].add_field(264, "gaurav".to_string());
        group[1].add_field(267, "tatke".to_string());
    
        {
            let sub_group2 = group[1].set_group(72, 2);
            sub_group2[0].add_field(78, "leg2 id1".to_string());
            sub_group2[0].add_field(77, "leg2 ref1".to_string());
            sub_group2[1].add_field(78, "leg2 id2".to_string());
            sub_group2[1].add_field(77, "leg2 ref2".to_string());
        }
    }
    println!("Field 1 = {:?}", grp_inst1.get_field(35).unwrap());
    println!("Field 2 = {:?}", grp_inst1.get_field(40).unwrap());
    let rep_grp = grp_inst1.get_field(263).unwrap();
    println!("Repeating group field: {:?}", rep_grp);
    let rep_num = rep_grp.value.parse::<u32>();
    for grp in grp_inst1.get_group(263).unwrap() {
        println!("Group: ");
        println!("Group field 1: {:?}", grp.get_field(264).unwrap());
        println!("Group Field 2: {:?}", grp.get_field(267).unwrap());
        println!("Repeating group field 3: {:?}", grp.get_field(72).unwrap());
        for sub_grp in grp.get_group(72).unwrap() {
            println!("Sub Group: ");
            println!("Sub group field 1: {:?}", sub_grp.get_field(77).unwrap());
            println!("Sub group field 2: {:?}", sub_grp.get_field(78).unwrap());
        }
    }
    
}