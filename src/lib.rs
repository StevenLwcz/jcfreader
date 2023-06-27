pub mod java_class_file {

mod class_file_reader;

use std::slice::Iter;

use crate::java_class_file::class_file_reader::class_file_reader::{Index, ConstantPool, FieldInfo, MethodInfo, AttributeInfo, ClassFileReader, JavaVersion};

const JAVAP_FILE_NOT_FOUND: i32 = 1;
const JAVA_MAGIC: u32 = 0xcafebabe;

// todo provide iterators so can be private again ???
pub struct ClassFile {
    version             : JavaVersion,
    pub constant_pool       : ConstantPool,
    _access_flags       : u16, // to do  AccessFlags struct??
    _this_class         : Index,
    _super_class        : Index,
    interfaces          : Vec<Index>,
    fields              : Vec<FieldInfo>,
    methods             : Vec<MethodInfo>,
    pub attributes          : Vec<AttributeInfo>,
}

// needs to return a Result<ClassFile><Error>  for any errors found..

impl ClassFile {
    pub fn new(file_name: &String) -> Self {
        let mut reader = ClassFileReader::new(&file_name);
        if reader.read_u32() != JAVA_MAGIC {
            eprintln!("javap: Not a java class file {}", reader.file_name);
                        std::process::exit(JAVAP_FILE_NOT_FOUND);
        };

        Self {
            version         : JavaVersion(reader.read_u16(), reader.read_u16()),
            constant_pool   : ConstantPool::new(&mut reader),
            _access_flags   : reader.read_u16(),
            _this_class     : reader.read_constant_index(),
            _super_class    : reader.read_constant_index(),
            interfaces      : reader.read_interfaces(),
            fields          : reader.read_fields(),
            methods         : reader.read_methods(),
            attributes      : reader.read_attributes(),
      }
   }

   // pub fn get_access_flags()
   // pub fn get_class_name()
   // pub fn get_super_class_name()

   pub fn get_version(&self) -> String {
       self.version.to_string()
   }

  // to do transofrm the Index  struct to an expanded version and 
  // pass back a vector of better things 
   pub fn get_interfaces(&self) -> Iter<Index> {
       self.interfaces.iter()
   }
   
   pub fn get_fields(&self) -> Vec<Field> {
       self.fields.iter().map(|m| {
           Field {
              flags: m.access_flags, 
              name: self.constant_pool.get_item(&m.name_index),
              descriptor: self.constant_pool.get_item(&m.descriptor_index),
           }
        })
        .collect()
   }

   pub fn get_methods(&self) -> Vec<Method> {
       self.methods.iter().map(|m| {
           Method {
              flags: m.access_flags, 
              name: self.constant_pool.get_item(&m.name_index),
              descriptor: self.constant_pool.get_item(&m.descriptor_index),
           }
        })
        .collect()
   }

   pub fn get_class_attributes(&self) -> ClassAttributes {
       ClassAttributes::new(self)
   }

   // pub fn get_attributes(&self) -> Vec<Attribute> {
       // self.attributes.iter().map(|m| {
           // Attribute {
              // name: self.constant_pool.get_item(&m.attribute_name_index),
              // info: m.info.to_owned(),
           // }
        // })
        // .collect()
   // }

} // ClassFile

pub struct Method {
    flags: u16,
    name: String,
    descriptor: String,
}

impl Method {
    pub fn get_flags(&self) -> &u16 {
        &self.flags
    }
    pub fn get_name(&self) -> &String {
        &self.name 
    }
    pub fn get_descriptor(&self) -> &String {
        &self.descriptor
    }
}

pub struct Field {
    flags: u16,
    name: String,
    descriptor: String,
}

impl Field {
    pub fn get_flags(&self) -> &u16 {
        &self.flags
    }
    pub fn get_name(&self) -> &String {
        &self.name 
    }
    pub fn get_descriptor(&self) -> &String {
        &self.descriptor
    }
}

// pub struct Attribute {
    // name: String,
    // info: Vec<u8>
// }

// impl Attribute {
    // pub fn get_name(&self) -> &String {
        // &self.name 
    // }

    // pub fn get_info(&self) -> &Vec<u8> {
        // &self.info 
    // }
// }

#[derive(Default)]
pub struct ClassAttributes {
    pub source_file: Option<String>,
    // etc
}

impl ClassAttributes {
    fn new(class_file: &ClassFile) -> Self {
        let mut source_file : Option<String> = None;

        for a in &class_file.attributes {
            let name = &class_file.constant_pool.get_item(&a.attribute_name_index);
                match name.as_str() {
                    "SourceFile" => {
                        let index = u16::from_be_bytes(a.info[0..2].try_into().unwrap());
                        source_file = Some(class_file.constant_pool.get_item(&Index::Single(index)));
                    },
                    &_ => todo!("{}", name),
            }
        }
        Self {
           source_file,
        }
    }
}


} // mod java_class_file
