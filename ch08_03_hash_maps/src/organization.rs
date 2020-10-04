use std::collections::HashMap;


#[derive(Debug)]
pub struct Departments {
    index: HashMap<String, Vec<String>>,
}

impl Departments {
    pub fn new() -> Departments {
        Departments {
            index: HashMap::new()
        }
    }

    pub fn add_member(&mut self, department_name: String, member: String) {
        let department_members = Vec::new();
        let department = self.index.entry(
            department_name
        ).or_insert(
            department_members
        );

        if department.contains(&member) { return }

        department.push(member);
    }

    pub fn remove_member(&mut self, department_name: String, member: String) {
        let department = self.index.get_mut(&department_name).unwrap();

        let index = department.iter().position(|x| *x == member).unwrap();

        department.remove(index);
    }
}