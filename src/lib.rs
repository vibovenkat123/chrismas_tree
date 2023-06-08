use std::{
    fs::{self, ReadDir},
    io,
};
fn get_all_files() -> Vec<std::path::PathBuf> {
    let files = fs::read_dir("./").unwrap();
    loop_files(files, 0)
}

enum ImportStack {
    List(std::path::PathBuf, Box<ImportStack>),
    Nil,
}
use crate::ImportStack::{List, Nil};

fn loop_files(files: ReadDir, rec_num: u32) -> Vec<std::path::PathBuf> {
    let mut path_vec = vec![];
    if rec_num > 4 {
        return path_vec;
    }
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        if path.is_dir() {
            let f = fs::read_dir(&path).unwrap();
            let res = loop_files(f, rec_num + 1);
            path_vec.extend(res);
            continue;
        }
        match &path.extension() {
            Some(val) => {
                if val.to_str() == Some("ts") {
                    path_vec.push(path.clone());
                }
            }
            None => continue,
        }
    }
    if path_vec.is_empty() && rec_num == 0 {
        println!("No typescript files found");
    }
    path_vec
}

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut contents = vec![];
    let files = get_all_files();
    for file in files {
        let file_ext = file.extension();
        match file_ext {
            Some(ext) => {
                if ext == "ts" {
                    let content = read_file(&file)?;
                    contents.push(content);
                }
            }
            None => continue,
        }
    }
    let mut import_stack = Nil;
    for content in contents {
        let lines = content.split("\n");
        for line in lines {
            // if the content has import statement, add it to the import stack and also add the
            // imported files path to the import stack
            if line.contains("import") {
                let mut import_path = line.split("from").collect::<Vec<&str>>();
                import_path = import_path[1].split(";").collect::<Vec<&str>>();
                let mut import_path = import_path[0].trim().to_string();
                import_path = import_path.replace("\"", "");
                import_path = import_path.replace("'", "");
                let mut import_path = std::path::PathBuf::from(import_path);
                import_path.set_extension("ts");
                import_stack = List(import_path, Box::new(import_stack));
            }
        }
    }
    loop {
        match import_stack {
            List(path, next) => {
                println!("importing {:?}", path);
                import_stack = *next;
            }
            Nil => break,
        }
    }
    Ok(())
}

fn read_file(file: &std::path::PathBuf) -> Result<String, io::Error> {
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}
