use std::env;
use jcfreader::java_class_file;
use jcfreader::java_class_file::Dump;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dump :Dump = Dump::None;
    let mut file_name :Option<String> = None;
    for arg in args {
        println!("{}", arg);
        if arg == "-hex".to_string() {
            dump = Dump::Hex;
        } else if arg == "-byte".to_string() {
            dump = Dump::Byte;
        } else {
            file_name = Some(arg.to_string());
        }
    }

    if file_name.is_none() {
        println!("jcfr: Filename not specified");
        std::process::exit(1);
    }
    println!("filename: {}", file_name.as_ref().unwrap());

    let class_file = java_class_file::ClassFile::new(&file_name.unwrap(), dump);

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
