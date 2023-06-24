const JAVAP_FILE_NOT_FOUND: i32 = 1;
const JAVA_MAGIC: u32 = 0xcafebabe;

// todo provide iterators so can be private again ???
pub struct ClassFile {
    version             : JavaVersion,
    pub constant_pool       : ConstantPool,
    _access_flags       : u16, // to do  AccessFlags struct??
    _this_class         : Index,
    _super_class        : Index,
    pub interfaces          : Vec<Index>,
    pub fields              : Vec<FieldInfo>,
    pub methods             : Vec<Method>,
    pub attributes          : Vec<Attribute>,
}

// probably needs to return a Result of ClassFile and any errors found....

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

   pub fn get_version(&self) -> String {
       self.version.to_string()
   }
}

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;

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
    _Integer = 3,
    _Float,
    _Long,
    _Double,
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
            Tag::_Integer =>  write!(f, "Integer           "),
            Tag::_Float =>    write!(f, "Float             "),
            Tag::_Long =>     write!(f, "Long              "),
            Tag::_Double =>   write!(f, "Double            "),
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

pub enum LiteralInfo {
    String(String),
    Integer(i32), 
    Float(f32),
    Long(i64),
    Double(f64),
}

impl fmt::Display for LiteralInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralInfo::String(s) => write!(f, "{} ", s),
            LiteralInfo::Integer(i) => write!(f, "{} ", i),
            LiteralInfo::Long(l) => write!(f, "{} ", l),
            LiteralInfo::Double(d) => write!(f, "{} ", d),
            &LiteralInfo::Float(_) => todo!(),
        }
    }
}

pub struct FieldInfo {
    access_flags : u16, // todo Enum
    name_index : Index,
    descriptor_index: Index,
    attributes: Vec<Attribute>,
}

impl FieldInfo {
   fn new(reader: &mut ClassFileReader) -> Self {
       FieldInfo {
           access_flags: reader.read_u16(),
           name_index:  reader.read_constant_index(),
           descriptor_index: reader.read_constant_index(),
           attributes: reader.read_attributes(),
       }
   }
}

#[derive(Debug)] // todo format numbers to say 17.0
pub struct Attribute {
     attribute_name_index: Index,
     info: Vec<u8>,
}

#[derive(Debug)] // todo format numbers to say 17.0
pub struct Method {
    access_flags: u16, // todo enum
    pub name_index: Index,
    pub descriptor_index: Index,
    pub attributes : Vec<Attribute>,
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

struct JavaVersion(u16, u16);

impl fmt::Display for JavaVersion {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.1 - 44, self.0)
    }
}

struct ClassFileReader {
    file: File,
    file_name: String,
}

impl ClassFileReader {
    fn new(file_name: &String) -> Self {
        Self {
            file : match File::open(file_name) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("javap: Can't open file {} - {}", &file_name, err);
                    std::process::exit(JAVAP_FILE_NOT_FOUND);
                }
            },
            file_name: file_name.to_string(),
        }
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u32::from_be_bytes(buf),
            Err(err) => {
                eprintln!("javap: error reading u32 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u16::from_be_bytes(buf),
            Err(err) => {
                eprintln!("javap: error reading u16 {} - {}", self.file_name, err); 
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        match self.file.read_exact(&mut buf) {
            Ok(_) => u8::from_be_bytes(buf),
            Err(err) => {
                eprintln!("javap: error reading u8 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        match self.file.read_exact(&mut buf) {
            Ok(_) => buf,
            Err(err) => {
                eprintln!("javap: error reading n bytes {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_constant_index(&mut self) -> Index {
        Index::Single(self.read_u16())
    }

    fn read_constant_index_pair(&mut self) -> Index {
        Index::Pair(self.read_u16(), self.read_u16())
    }

    fn read_constant_index_ref(&mut self) -> Index {
        Index::Ref(self.read_u16(), self.read_u16())
    }

   fn read_interfaces(&mut self) -> Vec<Index> {
       let mut count =  self.read_u16();
       let mut interfaces = Vec::<Index>::with_capacity(count as usize);
       while count > 0 {
           interfaces.push(self.read_constant_index());
           count -= 1;
       }
       interfaces
   }

   fn read_fields(&mut self) -> Vec<FieldInfo> {
       let mut count =  self.read_u16();
       let mut fields = Vec::<FieldInfo>::with_capacity(count as usize);
       while count > 0 {
           fields.push(FieldInfo::new(self));
           count -= 1;
       }
       fields
   }

   fn read_vec_len_u32(&mut self) -> Vec<u8> {
        let count = self.read_u32() as usize;
        self.read_bytes(count)
   }

   fn read_attributes(&mut self) -> Vec<Attribute> {
       let mut count =  self.read_u16();
       let mut attributes = Vec::<Attribute>::with_capacity(count as usize);
       while count > 0 {
           attributes.push(
               Attribute {
                   attribute_name_index : self.read_constant_index(),
                   info: self.read_vec_len_u32(),
               });
           count -= 1;
       }
       attributes
   }

   fn read_methods(&mut self) -> Vec<Method> {
       let mut count =  self.read_u16();
       let mut methods = Vec::<Method>::with_capacity(count as usize);
       while count > 0 {
           methods.push(
               Method {
                   access_flags : self.read_u16(),
                   name_index  :  self.read_constant_index(),
                   descriptor_index : self.read_constant_index(),
                   attributes : self.read_attributes(),
           });
           count -= 1;
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
    fn new(reader: &mut ClassFileReader) -> Self {
        let count = reader.read_u16();
        let mut cp = Self {
            constant_info : Vec::<ConstantInfo>::with_capacity(count as usize),
            literal_pool : HashMap::new(),
        };
        cp.read_constant_pool(count, reader);
        cp
    }

    fn read_constant_pool(&mut self, count: u16, reader: &mut ClassFileReader) {
        for index in 1..count {
            let tag = reader.read_u8();

            let info = match tag {
                TAG_UTF8 => self.read_utf8(index, reader),
                TAG_INTEGER => self.read_integer(reader),
                TAG_FLOAT => self.read_float(reader),
                TAG_LONG => self.read_long(reader),
                TAG_DOUBLE => self.read_double(reader),
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
        let string = String::from_utf8(bytes).unwrap(); // todo add string to hashset
        self.literal_pool.insert(index, LiteralInfo::String(string));
        ConstantInfo(Tag::Utf8, Index::Single(index))  
    }

    fn read_integer(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_float(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_long(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_double(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
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
      todo!()
    }

    fn read_method_type(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_dynamic(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_invoke_dynamic(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_module(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }

    fn read_package(&mut self, _reader: &mut ClassFileReader) -> ConstantInfo {
      todo!()
    }


    fn get_literal(&self, index: u16) -> &LiteralInfo {
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

