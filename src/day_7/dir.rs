use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct Dir {
    pub size_of_files: u32,
    pub children: HashSet<String>,
}

impl Dir {
    pub fn new() -> Self {
        Self {
            size_of_files: 0,
            children: HashSet::new(),
        }
    }

    pub fn add_to_file_size(&mut self, l: &Vec<String>) {
        let file_size = &l[0];
        self.size_of_files += file_size.parse::<u32>().unwrap();
    }

    pub fn add_children(&mut self, dir_name: String) {
        self.children.insert(dir_name.clone());
    }
}
