use jcfreader::java_class_file;

fn main() {
    // let file_name = "tests/files/test1.class".to_string();
    let file_name = "a.class".to_string();
    let class_file = java_class_file::ClassFile::new(&file_name);

    println!("{}", class_file.get_version());
    println!("Fields");
    for field in class_file.get_fields() {
        println!("{}: {}", field.get_name(), field.get_descriptor());
    }
    println!("Methods");
    for method in class_file.get_methods() {
        println!("{}: {}", method.get_name(), method.get_descriptor());
    }
    let attr = class_file.get_class_attributes();
    println!("SourceFile {:?}", attr.source_file);
    println!("RuntimeVisibleAnnotations {:?}", attr.runtime_visible_annotations);
}
