use crate::quickfix_errors::*;
use crate::types::*

type Tag = u32;

#[derive(Debug)]
struct FieldMap (HashMap<Tag, FixTypeField>)

impl FieldMap {
    fn new() -> FieldMap {
        FieldMap (HashMap::new())
    }

    // fn set_int<T: Into<Int>>(&mut self, tag: u32, value: T) {
    //         self.0.insert(tag, value.into::<Int>().into::<FixTypeField>());
    // }

    // fn set_float<T: Into<Float>>(&mut self, tag: u32, value: T) {
    //     self.0.insert(tag, value.into::<Float>().into::<FixTypeField>());
    // }

    // fn set_string<T: ToString>(&mut self, tag: u32, value: T) {
    //     self.0.insert(tag, value.to_string().into::<FixTypeField>())
    // }

    // fn set_char<T: Into<Char>>(&mut self, tag: u32, value: T) {
    //     self.0.insert(tag, value.into::<Char>().into::<FixTypeField>());
    // }

    // fn set_bool<T: Into<Bool>>(&mut self, tag: u32, value: T) {
    //     self.0.insert(tag, value.into::<Bool>().into::<FixTypeField>());
    // }

    fn insert(&mut self, tag: Tag, value: FixTypeField) {
        self.0.insert(tag, FixTypeField)
    }

    fn get(&self, tag: Tag) -> Option<&FixTypeField> {
        self.0.get(&tag)
    }

    fn get_mut(&mut self, tag: Tag) -> Option<&mut FixTypeField> {
        self.0.get_mut(&tag)
    }
}

pub trait MessageBuilder {
    fn add_field(&mut self, tag: Tag, field: FixTypeField);
    fn get_field(&self, tag: Tag) -> Option<&FixTypeField>;
    fn get_mut_field(&mut self, tag: Tag) -> Option<&FixTypeField>;
    fn add_group(&mut self, tag: Tag, group: Group);
    fn get_group(&self, tag: Tag) -> Option<&Group>;
    fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group>;
}

#[derive(Debug)]
struct GroupInstance {
    group_fields: FieldMap,
    sub_groups: HashMap<Tag, Group>
}

impl GroupInstance {
    fn new() -> GroupInstance {
        GroupInstance {
            group_fields: FieldMap::new(),
            sub_groups: HashMap::new()
        }
    }

    fn set_int<T: Into<Int>>(&mut self, tag: u32, value: T) {
            self.0.insert(tag, value.into::<Int>().into::<FixTypeField>());
    }

    fn set_float<T: Into<Float>>(&mut self, tag: u32, value: T) {
        self.0.insert(tag, value.into::<Float>().into::<FixTypeField>());
    }

    fn set_string<T: ToString>(&mut self, tag: u32, value: T) {
        self.0.insert(tag, value.to_string().into::<FixTypeField>())
    }

    fn set_char<T: Into<Char>>(&mut self, tag: u32, value: T) {
        self.0.insert(tag, value.into::<Char>().into::<FixTypeField>());
    }

    fn set_bool<T: Into<Bool>>(&mut self, tag: u32, value: T) {
        self.0.insert(tag, value.into::<Bool>().into::<FixTypeField>());
    }
}

impl MessageBuilder for GroupInstance {
    pub fn add_field(&mut self, tag: Tag, field: FixTypeField) {
        self.group_fields.insert(tag, field);
    }

    pub fn get_field(&self, tag: Tag) -> Option<&FixTypeField> {
        self.group_fields.get(tag)
    }

    pub fn get_mut_field(&mut self, tag: Tag) -> Option<&mut FixTypeField> {
        self.group_fields.get_mut(tag)
    }

    pub fn add_group(&mut self, tag: Tag, group: Group) {
        self.sub_groups.insert(tag, group);
    }

    pub fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.sub_groups.get(tag)
    }

    pub fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.sub_groups.get_mut(tag)
    }
}

#[derive(Debug)]
pub struct Group {
    subgroup_delimiter: Tag,
    group_list: Vec<GroupInstance>
}

#[derive(Debug)]
pub struct Message {
    message_fields: FieldMap,
    message_groups: HashMap<Tag, Group>
}

impl MessageBuilder for Message {
    pub fn add_field(&mut self, tag: Tag, field: FixTypeField) {
        self.message_fields.insert(tag, field);
    }

    pub fn get_field(&self, tag: Tag) -> Option<&FixTypeField> {
        self.message_fields.get(tag)
    }

    pub fn get_mut_field(&mut self, tag: Tag) -> Option<&mut FixTypeField> {
        self.message_fields.get_mut(tag)
    }

    pub fn add_group(&mut self, tag: Tag, group: Group) {
        self.message_groups.insert(tag, group);
    }

    pub fn get_group(&self, tag: Tag) -> Option<&Group> {
        self.message_groups.get(tag)
    }

    pub fn get_mut_group(&mut self, tag: Tag) -> Option<&mut Group> {
        self.message_groups.get_mut(tag)
    }
}

impl Message {
    fn new() -> Self {
        Self {
            message_fields: FieldMap::new()
            message_groups: HashMap::new()
        }
    }

    fn set_int<T: Into<Int>>(&mut self, tag: Tag, value: T) {
            self.0.insert(tag, value.into::<Int>().into::<FixTypeField>());
    }

    fn get_int(&self, tag: Tag) -> Option<Int> {
        let field = self.get_field(tag: Tag);
        if let Some(i) = field {
            let fxfield = Some(i).cloned().unwrap();
            Int::try_from(fxfield).ok()
        } else {
            None
        }
    }

    fn set_float<T: Into<Float>>(&mut self, tag: Tag, value: T) {
        self.0.insert(tag, value.into::<Float>().into::<FixTypeField>());
    }

    fn set_string<T: ToString>(&mut self, tag: Tag, value: T) {
        self.0.insert(tag, value.to_string().into::<FixTypeField>())
    }

    fn set_char<T: Into<Char>>(&mut self, tag: Tag, value: T) {
        self.0.insert(tag, value.into::<Char>().into::<FixTypeField>());
    }

    fn set_bool<T: Into<Bool>>(&mut self, tag: Tag, value: T) {
        self.0.insert(tag, value.into::<Bool>().into::<FixTypeField>());
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

}