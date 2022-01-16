use std::fs;
use std::path::Path;

pub fn get_all_galleries() -> Vec<String>{
    let paths = fs::read_dir("./images").unwrap();
    let mut vec = Vec::new();

    for path in paths {
        // vec.push(path.unwrap()
        //                 .path()
        //                 .display()
        //                 .to_string());

        vec.push(Path::new(&path.unwrap()
                                .path()
                                .display()
                                .to_string()).file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string());
    }

    vec
}

pub fn get_all_gallery_image_names(gallery_name: String) -> Vec<String>{
    let paths = fs::read_dir(format!("./images/{gallery_name}")).unwrap();
    let mut vec = Vec::new();

    for path in paths {
        vec.push(Path::new(&path.unwrap()
                                .path()
                                .display()
                                .to_string()).file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string());
    }

    vec
}
