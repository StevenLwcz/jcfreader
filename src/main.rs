use jcfreader::java_class_file;

fn main() {
    let file_name = "tests/files/test1.class".to_string();
    let class_file = java_class_file::ClassFile::new(&file_name);

    println!("{}", class_file.get_version());
    for method in class_file.get_methods() {
        println!("{}", method.get_name());
        println!("{}", method.get_descriptor());
    }
}
