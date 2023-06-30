use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;

const JAVAP_FILE_NOT_FOUND: i32 = 1;

    const TAG_UTF8: u8 = 1;
    const TAG_INTEGER: u8 = 3;
    const TAG_FLOAT: u8 = 4;
    const TAG_LONG: u8 = 5;
    const TAG_DOUBLE: u8 = 6;
    const TAG_CLASS: u8 = 7;
    const TAG_STRING: u8 = 8;
    const TAG_FIELDREF: u8 = 9;
    const TAG_METHODREF: u8 = 10;
    const TAG_INTERFACEMETHODREF: u8 = 11;
    const TAG_NAMEANDTYPE: u8 = 12;
    const TAG_METHODHANDLE: u8 = 15;
    const TAG_METHODTYPE: u8 = 16;
    const TAG_DYNAMIC: u8 = 17;
    const TAG_INVOKEDYNAMIC: u8 = 18;
    const TAG_MODULE: u8 = 19;
    const TAG_PACKAGE: u8 = 20;

enum Tag {
    Utf8,
    Integer = 3,
    Float,
    Long,
    Double,
    Class,
    String,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    NameAndType,
    _MethodHandle = 15,
    _MethodType,
    _InvokeDynamic,
    _Module,
    _Package,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::Utf8 =>      write!(f, "Utf8              "),
            Tag::Integer =>  write!(f, "Integer           "),
            Tag::Float =>    write!(f, "Float             "),
            Tag::Long =>     write!(f, "Long              "),
            Tag::Double =>   write!(f, "Double            "),
            Tag::Class =>     write!(f, "Class             "),
            Tag::String =>    write!(f, "String            "),
            Tag::FieldRef =>  write!(f, "FieldRef          "),
            Tag::MethodRef => write!(f, "MethodRef         "),
            Tag::InterfaceMethodRef => write!(f, "InterfaceMethodRef"),
            Tag::NameAndType =>        write!(f, "NameAndType       "),
            Tag::_MethodHandle =>      write!(f, "MethodHandle      "),
            Tag::_MethodType =>        write!(f, "MethodType        "),
            Tag::_InvokeDynamic =>     write!(f, "InvokeDynamic     "),
            Tag::_Module =>            write!(f, "Module            "),
            Tag::_Package =>           write!(f, "Package           "),
        }
    }
}

#[derive(Debug)] // todo format numbers to say 17.0
pub enum Index {
    Single(u16),
    Pair(u16, u16),
    Ref(u16, u16),
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Index::Single(i) => write!(f, "#{}", i),
            Index::Pair(i1, i2) | Index::Ref(i1, i2) => write!(f, "#{},#{}", i1,i2),
        }
    }
}

pub struct ConstantInfo(Tag, pub Index);

impl fmt::Display for ConstantInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:14}",  self.0, self.1)
    }
}

#[derive(Debug,Clone)]
pub enum LiteralInfo {
    String(String),
    Integer(u32), 
    Float(f32),
    Long(u64),
    Double(f64),
}

impl fmt::Display for LiteralInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralInfo::String(s) => write!(f, "{}", s),
            LiteralInfo::Integer(i) => write!(f, "{}", i),
            LiteralInfo::Long(l) => write!(f, "{}", l),
            LiteralInfo::Double(d) => write!(f, "{}", d),
            &LiteralInfo::Float(_) => todo!(),
        }
    }
}

pub struct FieldInfo {
    pub access_flags : u16, // todo Enum
    pub name_index : Index,
    pub descriptor_index: Index,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
   pub fn new(reader: &mut ClassFileReader) -> Self {
       FieldInfo {
           access_flags: reader.read_u16(),
           name_index:  reader.read_constant_index(),
           descriptor_index: reader.read_constant_index(),
           attributes: reader.read_attributes(),
       }
   }
}

pub struct AttributeInfo {
     pub attribute_name_index: Index,
     pub info: Vec<u8>,
}

pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: Index,
    pub descriptor_index: Index,
    pub attributes : Vec<AttributeInfo>,
}

struct _Code {
    stack: u16,
    locals: u16,
    args_size: u16,
    byte_code: Vec<u8>,
//    line_number_table: Vec<LineNumber>,
}

impl _Code {
    fn _new(reader: &mut ClassFileReader) -> Self {
        Self {
            stack: reader.read_u16(),
            locals: reader.read_u16(),
            args_size: reader.read_u16(),
            byte_code: reader.read_vec_len_u32(),
        }
    }
}

pub struct JavaVersion(pub u16, pub u16);

impl fmt::Display for JavaVersion {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.1 - 44, self.0)
    }
}

pub struct ClassFileReader {
    file: File,
    pub file_name: String,
}

impl ClassFileReader {
    pub fn new(file_name: &String) -> Self {
        Self {
            file : match File::open(file_name) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("jcfreader: Can't open file {} - {}", &file_name, err);
                    std::process::exit(JAVAP_FILE_NOT_FOUND);
                }
            },
            file_name: file_name.to_string(),
        }
    }

    pub fn read_f64(&mut self) -> f64 {
        let mut buf = [0; 8];
        match self.file.read_exact(&mut buf) {
            Ok(_) => f64::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading f64 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_f32(&mut self) -> f32 {
        let mut buf = [0; 4];
        match self.file.read_exact(&mut buf) {
            Ok(_) => f32::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading f32 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u64::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading u64 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u32::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading u32 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u16::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading u16 {} - {}", self.file_name, err); 
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u8::from_be_bytes(buf),
            Err(err) => {
                eprintln!("jcfreader: error reading u8 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        match self.file.read_exact(&mut buf) {
            Ok(_) => buf,
            Err(err) => {
                eprintln!("jcfreader: error reading n bytes {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_constant_index(&mut self) -> Index {
        Index::Single(self.read_u16())
    }

    fn read_constant_index_pair(&mut self) -> Index {
        Index::Pair(self.read_u16(), self.read_u16())
    }

    fn read_constant_index_ref(&mut self) -> Index {
        Index::Ref(self.read_u16(), self.read_u16())
    }

   pub fn read_interfaces(&mut self) -> Vec<Index> {
       let count =  self.read_u16();
       let mut interfaces = Vec::<Index>::with_capacity(count as usize);
       for _ in 0..count {
           interfaces.push(self.read_constant_index());
       }
       interfaces
   }

   pub fn read_fields(&mut self) -> Vec<FieldInfo> {
       let count =  self.read_u16();
       let mut fields = Vec::<FieldInfo>::with_capacity(count as usize);
       for _ in 0..count {
           fields.push(FieldInfo::new(self));
       }
       fields
   }

   fn read_vec_len_u32(&mut self) -> Vec<u8> {
        let count = self.read_u32() as usize;
        self.read_bytes(count)
   }

   pub fn read_attributes(&mut self) -> Vec<AttributeInfo> {
       let count =  self.read_u16();
       let mut attributes = Vec::<AttributeInfo>::with_capacity(count as usize);
       for _ in 0..count {
           attributes.push(
               AttributeInfo {
                   attribute_name_index : self.read_constant_index(),
                   info: self.read_vec_len_u32(),
               });
       }
       attributes
   }

   pub fn read_methods(&mut self) -> Vec<MethodInfo> {
       let count =  self.read_u16();
       let mut methods = Vec::<MethodInfo>::with_capacity(count as usize);
       for _ in 0..count {
           methods.push(
               MethodInfo {
                   access_flags : self.read_u16(),
                   name_index  :  self.read_constant_index(),
                   descriptor_index : self.read_constant_index(),
                   attributes : self.read_attributes(),
           });
       }
       methods
   }

} //

// todo create iterator for this which combines the both :___)
pub struct ConstantPool {
    pub constant_info: Vec<ConstantInfo>,
    literal_pool: HashMap<u16, LiteralInfo>, 
}

impl ConstantPool {
    pub fn new(reader: &mut ClassFileReader) -> Self {
        let count = reader.read_u16();
        let mut cp = Self {
            constant_info : Vec::<ConstantInfo>::with_capacity(count as usize),
            literal_pool : HashMap::new(),
        };
        cp.read_constant_pool(count, reader);
        cp
    }

    fn read_constant_pool(&mut self, count: u16, reader: &mut ClassFileReader) {
        let mut index = 1;
        while index < count {
            let tag = reader.read_u8();
            let info = match tag {
                TAG_UTF8 => self.read_utf8(index, reader),
                TAG_INTEGER => self.read_integer(index, reader),
                TAG_FLOAT => self.read_float(index, reader),
                TAG_LONG => { 
                    index += 1;
                    self.read_long(index - 1, reader)
                },
                TAG_DOUBLE => { 
                    index += 1;
                    self.read_double(index - 1, reader)
                },
                TAG_CLASS => self.read_class(reader),
                TAG_STRING => self.read_string(reader), 
                TAG_FIELDREF => self.read_field_ref(reader),
                TAG_METHODREF => self.read_method_ref(reader),
                TAG_INTERFACEMETHODREF => self.read_interface_method_ref(reader),
                TAG_NAMEANDTYPE => self.read_name_and_type(reader),
                TAG_METHODHANDLE => self.read_method_handle(reader),
                TAG_METHODTYPE => self.read_method_type(reader),
                TAG_DYNAMIC => self.read_dynamic(reader),
                TAG_INVOKEDYNAMIC => self.read_invoke_dynamic(reader),
                TAG_MODULE => self.read_module(reader),
                TAG_PACKAGE => self.read_package(reader),
                _ => { 
                    eprintln!("Invalid tag {} in constant pool", tag);
                    std::process::exit(JAVAP_FILE_NOT_FOUND);
                }
            };
            self.constant_info.push(info);
            index += 1; 
        };
    }

    fn read_method_ref(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::MethodRef, reader.read_constant_index_ref())
    }

    fn read_field_ref(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::FieldRef, reader.read_constant_index_ref())
    }

    fn read_interface_method_ref(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::InterfaceMethodRef, reader.read_constant_index_ref())
    }

    fn read_utf8(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
        let len = reader.read_u16();  
        let bytes = reader.read_bytes(len as usize);
        let string = String::from_utf8(bytes).unwrap(); 
        self.literal_pool.insert(index, LiteralInfo::String(string));
        ConstantInfo(Tag::Utf8, Index::Single(index))  
    }

    fn read_integer(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let i = reader.read_u32();
      self.literal_pool.insert(index, LiteralInfo::Integer(i));
      ConstantInfo(Tag::Integer, Index::Single(index))
    }

    fn read_float(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let f = reader.read_f32();
      self.literal_pool.insert(index, LiteralInfo::Float(f));
      ConstantInfo(Tag::Float, Index::Single(index))
    }

    fn read_long(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let l = reader.read_u64();
      self.literal_pool.insert(index, LiteralInfo::Long(l));
      ConstantInfo(Tag::Long, Index::Single(index))
    }

    fn read_double(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let d = reader.read_f64();
      self.literal_pool.insert(index, LiteralInfo::Double(d));
      ConstantInfo(Tag::Double, Index::Single(index))
    }

    fn read_class(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::Class, reader.read_constant_index())
    }

    fn read_string(&mut self, reader: &mut ClassFileReader)  -> ConstantInfo {
        ConstantInfo(Tag::String, reader.read_constant_index())
    }

    fn read_name_and_type(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::NameAndType, reader.read_constant_index_pair())
    }

    fn read_method_handle(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Handle")
    }

    fn read_method_type(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Method Type")
    }

    fn read_dynamic(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Dynamic")
    }

    fn read_invoke_dynamic(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Invoke Dynamic")
    }

    fn read_module(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Module")
    }

    fn read_package(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!("Package")
    }


    pub fn get_literal(&self, index: u16) -> &LiteralInfo {
        self.literal_pool.get(&index).unwrap()
    }

    pub fn get_item(&self, info: &Index) -> String {
        match info {
            Index::Single(index) => 
                format!("{}", self.get_literal(*index)),
            Index::Pair(i1, i2) => 
                format!("{}:{}", self.get_literal(*i1), self.get_literal(*i2)),
            Index::Ref(i1, i2) => {
                let info = &self.constant_info[*i1 as usize];
                let data1 = self.get_item(&info.1);
                let info = &self.constant_info[*i2 as usize];
                let data2 = self.get_item(&info.1);
                format!("{}.{}", data2, data1)
            },
        }
    }

    // fn iter(&self) -> PoolIter {
        // let mut iter = self.constant_pool.iter();
        // iter
    // }
}

// struct PoolIter {
// }

// impl Iterator for ConstantPool {
    // fn next(&mut self) -> Option<String> {
     //  
    // }

// }
