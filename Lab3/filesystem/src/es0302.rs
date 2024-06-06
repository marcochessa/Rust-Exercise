use std::collections::VecDeque;
use std::time::SystemTime;
use walkdir::WalkDir;

struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

struct Dir {
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

// Define this enum in order to be able to store different types in the same vector
enum Node {
    File(File),
    Dir(Dir),
}
impl Node {
    fn name(&self) -> &str {
        match self {
            Node::File(f) => &f.name,
            Node::Dir(d) => &d.name,
        }
    }
}

#[derive(Debug)]
enum FSError {
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}

// define lifetimes
struct MatchResult<'a> {
    q: &'a str, // matched query string
    path: String, // matched path
    node: &'a Node, // matched node
}

struct Filesystem {
    root: Node,
}

impl Filesystem {
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    pub fn new() -> Self {
        Filesystem{
            root: Node::Dir(Dir{
                name: String::from(""),
                modified: SystemTime::now(),
                children: vec![]
            })
        }
    }

    // create a new filesystem reading from disk all the structure under the given path
    // in the file content just write the firt 1k bytes of the file
    // return the root node of the filesystem
    // (implement this function at the end, after all the other methods, the only purpose is to take a look std::fs functions, use std::fs:read_dir)
    pub fn from(path: &str) -> Self {
        let mut fs = Filesystem::new();

        for entry in WalkDir::new(path) {
            let entry = entry.unwrap();

            let path = entry.path();
            let parent = path.parent();

            // if we want ot unwrap many options together we can use a match with tuple
            // we also use and_then extract the valure from parent, before trying to convert it to a string
            if let (Some(_path), Some(_parent)) = (path.to_str(), parent.and_then(|p| p.to_str())) {
                if path.is_file() {
                    fs.create_file(_parent, _path).unwrap();
                } else if path.is_dir() {
                    // let d = Dir::
                    fs.mkdir(_parent, _path).unwrap();
                }
            }
        }

        fs
    }

    // create a new directory in the filesystem under the given path
    // return a reference the created dir
    // possible errors: NotFound, path NotADir, Duplicate
    pub fn mkdir(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> {
        match self.get_mut(path) {
            Ok(n) => {
                match n {
                    Node::Dir(d) => {
                        if d.children.iter().any(|n| n.name() == name) {
                            return Err(FSError::Duplicate);
                        }
                        let dir = Node::Dir(Dir {
                            name: name.to_string(),
                            modified: SystemTime::now(),
                            children: vec![],
                        });
                        d.children.push(dir);

                        // dir has been moved inside the vector, now we must get a mutable reference from
                        // the vector itself
                        if let Node::Dir(d) = d.children.last_mut().unwrap() {
                            return Ok(d);
                        } else {
                            // this should never happen, but necessary to compile
                            return Err(FSError::GenericError);
                        }
                    }
                    _ => Err(FSError::NotADir),
                }
            }
            Err(e) => Err(e),
        }
    }

    // possible errors: NotFound, path is NotADir, Duplicate
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> {
        match self.get_mut(path) {
            Ok(n) => match n {
                Node::Dir(d) => {
                    if d.children.iter().any(|n| n.name() == name) {
                        return Err(FSError::Duplicate);
                    }
                    let file = Node::File(File {
                        name: name.to_string(),
                        modified: SystemTime::now(),
                        content: vec![],
                    });
                    d.children.push(file);
                    if let Node::File(f) = d.children.last_mut().unwrap() {
                        return Ok(f);
                    } else {
                        return Err(FSError::GenericError);
                    }
                }
                _ => Err(FSError::NotADir),
            },
            Err(e) => Err(e),
        }
    }

    // updated modification time of the file or the dir
    // possible errors: NotFound
    pub fn touch(&mut self, path: &str) -> Result<(), FSError> {
        match self.get_mut(path) {
            Ok(n) => match n {
                Node::Dir(d) => {
                    d.modified = SystemTime::now();
                    Ok(())
                }
                Node::File(f) => {
                    f.modified = SystemTime::now();
                    Ok(())
                }
            },
            Err(e) => Err(e),
        }
    }

    // remove a node from the filesystem and return it
    // if it's a dir, it must be empty
    // possible errors: NotFound, DirNotEmpty
    pub fn delete(&mut self, path: &str) -> Result<Node, FSError> {
        // get the parent dir
        let parts = path.split("/").collect::<Vec<&str>>();
        let parent_path = parts[..parts.len() - 1].join("/");
        let name = parts.last().unwrap();
        match self.get_mut(parent_path.as_str()) {
            Ok(n) => match n {
                Node::Dir(d) => {
                    if let Some(i) = d.children.iter().position(|n| n.name() == *name) {
                        let node = d.children.get(i).unwrap();
                        if let Node::Dir(_d) = node {
                            if _d.children.len() > 0 {
                                return Err(FSError::DirNotEmpty);
                            }
                        }
                        return Ok(d.children.remove(i));
                    } else {
                        return Err(FSError::NotFound);
                    }
                }
                _ => Err(FSError::NotADir),
            },
            Err(e) => Err(e),
        }
    }

    // get a reference to a node in the filesystem, given the path
    pub fn get(&mut self, path: &str) -> Result<&Node, FSError> {

        if path == "/" {
            return  Ok(&self.root);
        }

        let mut parts = path.split("/").collect::<VecDeque<&str>>();
        let mut cnode = &self.root;

        while let Some(part) = parts.pop_front() {
            if parts.len() == 0 {
                if cnode.name() == part {
                    return Ok(cnode);
                } else {
                    return Err(FSError::NotFound);
                }
            }

            match cnode {
                Node::Dir(d) => {
                    let pos = d.children.iter().position(|n| n.name() == parts[0]);
                    if let Some(i) = pos {
                        cnode = d.children.get(i).unwrap();
                    } else {
                        return Err(FSError::NotFound);
                    }
                }
                Node::File(_) => {
                    return Err(FSError::NotADir);
                }
            }
        }

        Err(FSError::NotFound)
    }

    // get a mutable reference to a node in the filesystem, given the path
    pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> {

        if path == "/" {
            return  Ok(&mut self.root);
        }

        let mut parts = path.split("/").collect::<VecDeque<&str>>();
        let mut cnode = &mut self.root;

        while let Some(part) = parts.pop_front() {
            if parts.len() == 0 {
                if cnode.name() == part {
                    return Ok(cnode);
                } else {
                    return Err(FSError::NotFound);
                }
            }

            match cnode {
                Node::Dir(d) => {
                    // We keep this example loop since many may have found this problem
                    // Please note: this does not work since we try move the mut ref to n while we are in the loop
                    // and we keep a mut ref to the container d too
                    //for n in d.children.iter_mut() {
                    //    if n.name() == part {
                    //        cnode = n;
                    //        break;
                    //    }
                    //}

                    // this instead will work since we have only a immutable reference in the loop: before we find the postion,
                    // the we take the mutable reference using the found position
                    // we don't need to have a mut ref while search for the node
                    let pos = d.children.iter().position(|n| n.name() == parts[0]);
                    // now we can get it as mutable
                    if let Some(i) = pos {
                        cnode = d.children.get_mut(i).unwrap();
                    } else {
                        return Err(FSError::NotFound);
                    }
                }
                Node::File(_) => {
                    return Err(FSError::NotADir);
                }
            }
        }

        //Ok(borrow_mut())
        Err(FSError::GenericError)
    }

    // search for a list of paths in the filesystem
    // qs is a list query strings with constraints
    // the constraints must be matched in or (it's returned any node matching at least one constraint)
    // constraint format: "type:pattern"
    // constraints:
    // - "type:dir" -> match only directories
    // - "type:file" -> match only files
    // - "name:value" -> match only nodes with the given name
    // - "partname:value" -> match only nodes with the given string in the name

    pub fn find<'a>(&'a self, qs: &[&'a str]) -> Vec<MatchResult> {
        let mut res = Vec::new();
        let mut visit = VecDeque::from([&self.root]);
        while let Some(n) = visit.pop_front() {
            for q in qs {
                if Filesystem::do_match(q, &n) {
                    res.push(MatchResult {
                        q: q,
                        path: "".to_string(),
                        node: &n,
                    });
                }
            }
            match n {
                Node::Dir(d) => {
                    for c in d.children.iter() {
                        visit.push_back(c);
                    }
                }
                _ => {}
            }
        }

        res
    }


    fn do_match(qs: &str, n: &Node) -> bool {
        let parts = qs.split(":").collect::<Vec<&str>>();
        match parts[0] {
            "type" => match n {
                Node::Dir(_) => parts[1] == "dir",
                Node::File(_) => parts[1] == "file",
            },
            "name" => n.name() == parts[1],
            "partname" => n.name().contains(parts[1]),
            _ => false,
        }
    }


    // walk the filesystem, starting from the root, and call the closure for each node with its path
    // the first parameter of the closure is the path of the node, second is the node itself
    pub fn walk(&self, f: impl Fn(&str, &Node)) {
        let mut visit = VecDeque::from([(String::from(""), &self.root)]);
        while let Some((path, n)) = visit.pop_front() {
            f(&path, n);

            match n {
                Node::Dir(d) => {
                    for c in d.children.iter() {
                        visit.push_back((format!("{}/{}", path, c.name()), c));
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn demo() {

    let mut fs = Filesystem::new();

    // create a directory structure, 10 dirs with a child dir and file each one
    for i in 0..10 {
        fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
        fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
        fs.create_file(format!("/dir{}", i).as_str(), "file1").unwrap();
    }

    println!("find /child2");
    if let Ok(res) = fs.get_mut("/dir2/child1") {
        match res {
            Node::Dir(d) => {
                d.name = "dir2 found".to_string();
            }
            // try to match all possible errros
            _ => {}
        }
    } else {
        println!("not found");
    }

    // let's try with matches
    let matches = fs.find(&["name:child1", "type:file"]);
    for m in matches {
        match m.node {
            Node::File(f) => {
                // inspect content
            },
            Node::Dir(d) => {
                // inspect children
            },
            _ => {}
        }
    }

    // see note "riferimenti mutabili" in exercise text 
    // now let's try to modify the filesystem using the found matches
    // is it possible to do it? which error do you get from the compiler?

    // *** ANSWER:
    // matches is a vector of immutable references, so we cannot get a mutable reference to the filesystem while we are iterating over the matches
    let matches = fs.find(&["/dir2/child1", "/dir3/child1"]);
    //for m in matches {
    //    let node = fs.get_mut(m.path).unwrap();
    //    match node {
    //        Node::File(f) => {
    //            // inspect content
    //        }
    //        _ => {}
    //    }
    //}


    // how can you fix the previous code?
    // suggestion: this code using paths which are not referenced by MatchResults should compile. Why?
    // Therefore how can you use the paths returned in the MatchResults to modify the filesystem?
    //let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
    //for p in paths {
    //    // let n = fs.get_mut(p.as_str());
    //}
    // **** ANSWER:
    let paths = matches.iter().map(|m| m.path.clone()).collect::<Vec<String>>();
    // after this line we don't use anymore matches, therefore we can obtain a mutable reference to the filesystem
    for p in paths {
        let _ = fs.get_mut(p.as_str());
    }

    // now let's try to walk the filesystem
    fs.walk(|path, node| {
        match node {
            Node::File(f) => {
                println!("file: {}", path);
            }
            Node::Dir(d) => {
                println!("dir: {}", path);
            }
        }
    });

}

