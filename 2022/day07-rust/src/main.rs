#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn pretty_print(&self) -> String {
        return format!("- {} (file, size={})", self.name, self.size);
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    parent: Option<&'a Directory<'a>>,
    directories: Vec<&'a Directory<'a>>,
    files: Vec<&'a File>,
}

impl<'a> Directory<'a> {
    fn new(name: String) -> Directory<'a> {
        return Directory {
            name,
            parent: None,
            directories: Vec::new(),
            files: Vec::new(),
        };
    }

    fn new_with_parent(name: String, parent: &'a Directory<'a>) -> Directory<'a> {
        return Directory {
            name,
            parent: Some(parent),
            directories: Vec::new(),
            files: Vec::new(),
        };
    }

    fn pretty_print(&self) -> String {
        let left_padding = self.deapth_padding();
        let mut tmp = format!(
            "{}- {} (dir, size={})",
            left_padding,
            self.name,
            self.size()
        );

        for directory in self.directories.iter() {
            tmp = format!("{}\n{}  {}", tmp, left_padding, directory.pretty_print());
        }

        for file in self.files.iter() {
            tmp = format!("{}\n{}  {}", tmp, left_padding, file.pretty_print());
        }

        return format!("{}", tmp);
    }

    // fn add_directory(&mut self, directory: &'a mut Directory<'a>) {
    //     self.directories.push(&directory);
    //     directory.parent = Some(self);
    // }

    fn size(&self) -> usize {
        let mut tmp: usize = 0;

        for directory in self.directories.iter() {
            tmp += directory.size();
        }

        for file in self.files.iter() {
            tmp += file.size;
        }

        return tmp;
    }

    fn deapth_padding(&self) -> String {
        match self.parent {
            None => return String::from(""),
            Some(parent) => {
                return format!("{}  ", parent.deapth_padding());
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let f1 = File {
        name: "file1.txt".to_string(),
        size: 234,
    };

    let f2 = File {
        name: "file2.txt".to_string(),
        size: 456,
    };

    let f3 = File {
        name: "file3.txt".to_string(),
        size: 456,
    };

    let mut d1 = Directory::new(String::from("/"));
    d1.files.push(&f1);
    d1.files.push(&f2);

    // let mut d2 = Directory::new_with_parent(String::from("var"), &d1);
    let mut d2 = Directory::new(String::from("/var"));
    d2.files.push(&f3);

    d1.directories.push(&d2);
    d2.parent = Some(&d1);

    println!("{}", d1.pretty_print());
}
