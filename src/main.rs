use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = &args[1];

    println!("Scanning {}", path);

    let php_file_paths: Vec<String> = get_php_files_in(&path);

    for file_path in php_file_paths {
        println!("Path {}", file_path);
        let file_content = fs::read_to_string(file_path).unwrap();
        println!("Path {}", file_content);
    }

}

fn get_php_files_in(path: &String)-> Vec<String>{

    let paths = fs::read_dir(path).unwrap();

    let mut files: Vec<String> = Vec::new();

    for path in paths {
        let dir_entry: fs::DirEntry = path.unwrap();
        let file_type: fs::FileType = dir_entry.file_type().unwrap();
        let path: String = dir_entry.path().to_str().unwrap().to_string();
    
        if file_type.is_dir() {
            let mut sub_paths: Vec<String> = get_php_files_in(&path); 

            files.append(&mut sub_paths);
            continue
        }

        if get_extension_from_filename(&path).unwrap_or("") == "php" {
            files.push(path);
        }
    }

    files
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

mod tests {
    #[test]
    fn allow_to_get_extension_from_filename(){
        assert_eq!(super::get_extension_from_filename("abc.gz"), Some("gz"));
    }
}



