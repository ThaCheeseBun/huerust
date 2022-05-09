use std::fs;

fn main() {

    // constants
    let input_folder = "./input/";
    let output_folder = "./output/";

    // read entries, 10/10 code very efficent
    let num_entries: f32 = fs::read_dir(input_folder).unwrap().count() as f32;
    let entries = fs::read_dir(input_folder).unwrap();

    // loop entries
    let mut i: f32 = 0 as f32;
    for file in entries {
        println!("{}{}", i as i32, num_entries as i32);
        let img = image::open(file.unwrap().path()).unwrap();
        // big meth, perfect hue loop
        let wang = img.huerotate(((i / num_entries) * 360 as f32) as i32);
        // save with spacing
        wang.save(format!("{}{:04}{}", output_folder, i, ".png")).unwrap();
        i += 1 as f32;
    }
    
}
