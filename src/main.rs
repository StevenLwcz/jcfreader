use jcfreader::java_class_file;

fn main() {
    let file_name = "tests/files/test1.class".to_string();
    let class_file = java_class_file::ClassFile::new(&file_name);

    println!("{}", class_file.get_version());
    for field in class_file.get_fields() {
        println!("{}", field.get_name());
        println!("{}", field.get_descriptor());
    }
    for method in class_file.get_methods() {
        println!("{}", method.get_name());
        println!("{}", method.get_descriptor());
    }
    for attribute in class_file.get_attributes() {
        println!("{}", attribute.get_name());
    }
}
