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
   
  // to do transofrm the FieldInfo  struct to an expanded version and 
  // pass back a vector of better things 
   pub fn get_fields(&self) -> Iter<FieldInfo> {
       self.fields.iter()
   }

   // todo Beef up Method struct
   pub fn get_methods(&self) -> Vec<Method> {
       self.methods.iter().map(|m| {
           Method {
              name: self.constant_pool.get_item(&m.name_index),
           }
        })
        .collect()
   }

  // pass back a vector of better things 
   // pub fn get_attributes() -> Vec<AttributeInfo>
}

pub struct Method {
    name: String,
}

impl Method {
    pub fn get_name(&self) -> &String {
        &self.name 
    }
}

} // mod java_class_file
