mod cfr;
use cfr::{ClassFile};

fn main() {
    let file_name = "tests/files/test1.class".to_string();
    let class_file = ClassFile::new(&file_name);

    println!("{}", class_file.get_version());
    // needs enumerate 
    for info in &class_file.constant_pool.constant_info {
        println!("{} // {}",  &info, class_file.constant_pool.get_item(&info.1));
    }
    println!("No intefaces {}", class_file.interfaces.len());
    println!("No fields {}", class_file.fields.len());
    println!("No methods {}", class_file.methods.len());
    println!("No attributes {}", class_file.attributes.len());
    for method in &class_file.methods {
        println!("{}", class_file.constant_pool.get_item(&method.name_index));
        println!("{}", class_file.constant_pool.get_item(&method.descriptor_index));
        println!("{}", method.attributes.len());
        println!("{:?}", method.attributes);
    }
}
