// module class_file_reader
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::fmt;
use std::io::Seek;
use crate::Dump;

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
    MethodHandle = 15,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
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
            Tag::MethodHandle =>      write!(f, "MethodHandle      "),
            Tag::MethodType =>        write!(f, "MethodType        "),
            Tag::Dynamic =>     write!(f, "Dynamic     "),
            Tag::InvokeDynamic =>     write!(f, "InvokeDynamic     "),
            Tag::Module =>            write!(f, "Module            "),
            Tag::Package =>           write!(f, "Package           "),
        }
    }
}

#[derive(Debug)] // todo format numbers to say 17.0
pub enum Index {
    Single(u16),
    Pair(u16, u16),
    Ref(u16, u16),
    Dynamic(u16, u16),
    MethodHandle(u8, u16),
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Index::Single(i) => write!(f, "#{}", i),
            Index::Pair(i1, i2) | Index::Ref(i1, i2) => write!(f, "#{},#{}", i1,i2),
            Index::Dynamic(i1, i2) => write!(f, "{}:#{}", i1, i2),
            Index::MethodHandle(i1, i2) => write!(f, "{}:#{}", i1, i2),
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
           access_flags: reader.context("access flags").read_u16(),
           name_index:  reader.context("name index").read_constant_index(),
           descriptor_index: reader.context("descriptor index").read_constant_index(),
           attributes: reader.context("attributes").read_attributes(),
       }
   }
}

#[derive(Debug)]
pub struct AttributeInfo {
     pub attribute_name_index: Index,
     pub info: Vec<u8>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: Index,
    pub descriptor_index: Index,
    pub attributes : Vec<AttributeInfo>,
}

pub struct JavaVersion(pub u16, pub u16);

impl fmt::Display for JavaVersion {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.1 - 44, self.0)
    }
}

pub struct ClassFileReader {
    file: File,
    mode: Dump,
    pub file_name: String,
    context: Option<String>,
}

impl ClassFileReader {
    pub fn new(file_name: &String, dump: Dump) -> Self {
        Self {
            file : match File::open(file_name) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("jcfreader: Can't open file {} - {}", &file_name, err);
                    std::process::exit(JAVAP_FILE_NOT_FOUND);
                }
            },
            file_name: file_name.to_string(),
            mode: dump,
            context: None,
        }
    }

    pub fn dump_string(&mut self, pos: u64, s: String) -> String {
        match self.mode {
            Dump::Hex | Dump::Byte => println!("{:06x}: {:?}", pos, s),
            Dump::None => ()
        }
        s
    }

    pub fn dump_bytes<'a>(&'a mut self, pos: u64, buf: &'a [u8]) -> &[u8] {
        let mut pos = pos;
        match self.mode {
            Dump::Hex => {
                for slice in buf.chunks(16) {
                    println!("{:06x}: {:02x?}", pos, slice);
                    pos += 16;
                };
           },
           Dump::Byte => {
                for slice in buf.chunks(16) {
                    println!("{:06x}: {:3?}", pos, slice);
                    pos += 16;
                };
           },
           Dump::None => (),
        }
        buf
    }

    pub fn dump<N: std::fmt::Display>(&mut self, pos: u64, buf: &[u8], num: N) -> N {
        if self.context.is_some() {
            match self.mode {
                Dump::Hex =>  println!("{:06x}: {:16} {:x?} {}", pos, self.context.as_ref().unwrap(), buf, num),
                Dump::Byte => println!("{:06x}: {:16} {:?} {}", pos, self.context.as_ref().unwrap(), buf, num),
                Dump::None => ()
            }
            self.context = None;
        } else {
            match self.mode {
                Dump::Hex => println!("{:06x}: {:x?} {}", pos, buf, num),
                Dump::Byte => println!("{:06x}: {:?} {}", pos, buf, num),
                Dump::None => ()
            }
        }
        num
    }

    pub fn read_f64(&mut self) -> f64 {
        let mut buf = [0; 8];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, f64::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading f64 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_f32(&mut self) -> f32 {
        let mut buf = [0; 4];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, f32::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading f32 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, u64::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading u64 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, u32::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading u32 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    pub fn context(&mut self, context: &str) -> &mut Self {
        self.context = Some(context.to_string());
        self
    }
     

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, u16::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading u16 {} - {}", self.file_name, err); 
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump(pos, &buf, u8::from_be_bytes(buf)),
            Err(err) => {
                eprintln!("jcfreader: error reading u8 {} - {}", self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }

    fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump_bytes(pos, &buf).to_vec(),
            Err(err) => {
                eprintln!("jcfreader: error reading {} bytes {} - {}", len, self.file_name, err);
                std::process::exit(JAVAP_FILE_NOT_FOUND);
            }
        }
    }
 
    fn read_string(&mut self, len: usize) -> String {
        let mut buf = vec![0u8; len];
        let pos = self.file.stream_position().unwrap();
        match self.file.read_exact(&mut buf) {
            Ok(_) => self.dump_string(pos, String::from_utf8(buf).unwrap()),
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
        Index::Pair(self.context("name").read_u16(), self.context("type").read_u16())
    }

    fn read_constant_index_ref(&mut self) -> Index {
        Index::Ref(self.context("class").read_u16(), self.context("name & type").read_u16())
    }

    fn read_constant_index_dynamic(&mut self) -> Index {
        Index::Dynamic(self.context("bootstrap index").read_u16(), self.context("name & type").read_u16())
    }

    fn read_constant_index_handle(&mut self) -> Index {
        Index::MethodHandle(self.context("ref kind").read_u8(), self.context("ref index").read_u16())
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
       let count =  self.context("field count").read_u16();
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
       let count =  self.context("attr count").read_u16();
       let mut attributes = Vec::<AttributeInfo>::with_capacity(count as usize);
       for _ in 0..count {
           attributes.push(
               AttributeInfo {
                   attribute_name_index : self.context("name index").read_constant_index(),
                   info: self.context("info").read_vec_len_u32(),
               });
       }
       attributes
   }

   pub fn read_methods(&mut self) -> Vec<MethodInfo> {
       let count =  self.context("method count").read_u16();
       let mut methods = Vec::<MethodInfo>::with_capacity(count as usize);
       for _ in 0..count {
           methods.push(
               MethodInfo {
                   access_flags : self.context("access flags").read_u16(),
                   name_index  :  self.context("name index").read_constant_index(),
                   descriptor_index : self.context("descriptor index").read_constant_index(),
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
        let count = reader.context("constant count").read_u16();
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
            let tag = reader.context("tag").read_u8();
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
        let len = reader.context("utf8 len").read_u16();  
        let string = reader.read_string(len as usize);
        self.literal_pool.insert(index, LiteralInfo::String(string));
        ConstantInfo(Tag::Utf8, Index::Single(index))  
    }

    fn read_integer(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let i = reader.context("integer").read_u32();
      self.literal_pool.insert(index, LiteralInfo::Integer(i));
      ConstantInfo(Tag::Integer, Index::Single(index))
    }

    fn read_float(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let f = reader.context("float").read_f32();
      self.literal_pool.insert(index, LiteralInfo::Float(f));
      ConstantInfo(Tag::Float, Index::Single(index))
    }

    fn read_long(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let l = reader.context("long").read_u64();
      self.literal_pool.insert(index, LiteralInfo::Long(l));
      ConstantInfo(Tag::Long, Index::Single(index))
    }

    fn read_double(&mut self, index: u16, reader: &mut ClassFileReader) -> ConstantInfo {
      let d = reader.context("double").read_f64();
      self.literal_pool.insert(index, LiteralInfo::Double(d));
      ConstantInfo(Tag::Double, Index::Single(index))
    }

    fn read_class(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::Class, reader.context("class index").read_constant_index())
    }

    fn read_string(&mut self, reader: &mut ClassFileReader)  -> ConstantInfo {
        ConstantInfo(Tag::String, reader.context("string index").read_constant_index())
    }

    fn read_name_and_type(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::NameAndType, reader.read_constant_index_pair())
    }

    fn read_method_handle(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::MethodHandle, reader.read_constant_index_handle())
    }

    fn read_method_type(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::MethodType, reader.context("method type").read_constant_index())
    }

    fn read_dynamic(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::Dynamic, reader.read_constant_index_dynamic())
    }

    fn read_invoke_dynamic(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::InvokeDynamic, reader.read_constant_index_dynamic())
    }

    fn read_module(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::Module, reader.context("module").read_constant_index())
    }

    fn read_package(&mut self, reader: &mut ClassFileReader) -> ConstantInfo {
        ConstantInfo(Tag::Package, reader.context("package").read_constant_index())
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
            Index::Dynamic(i1, i2) => {
                let info = &self.constant_info[*i1 as usize];
                let bootstrap = &info.1;
                let info = &self.constant_info[*i2 as usize];
                let name_and_type = self.get_item(&info.1);
                format!("{}.{}", bootstrap, name_and_type)
            }
            Index::MethodHandle(i1, i2) => {
                let info = &self.constant_info[*i1 as usize];
                let kind = &info.1;
                let info = &self.constant_info[*i2 as usize];
                let index = self.get_item(&info.1);
                format!("{}.{}", kind, index)
            }
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
