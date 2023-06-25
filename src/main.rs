use jcfreader::java_class_file;

fn main() {
    let file_name = "tests/files/test1.class".to_string();
    let class_file = java_class_file::ClassFile::new(&file_name);

    println!("{}", class_file.get_version());
    // needs enumerate 
    for info in &class_file.constant_pool.constant_info {
        println!("{} // {}",  &info, class_file.constant_pool.get_item(&info.1));
    }
    println!("No intefaces {}", class_file.get_interfaces().size_hint().0);
    println!("No fields {}", class_file.get_fields().size_hint().0);
    println!("No methods {}", class_file.get_methods().size_hint().0);
    println!("No attributes {}", class_file.attributes.len());
    for method in class_file.get_methods() {
        println!("{}", class_file.constant_pool.get_item(&method.name_index));
        println!("{}", class_file.constant_pool.get_item(&method.descriptor_index));
        println!("{}", method.attributes.len());
        println!("{:?}", method.attributes);
    }
}
