// module jcfreader
use std::slice::Iter;
use std::io::Read;

pub mod class_file_reader;
use crate::class_file_reader::{Index, ConstantPool, FieldInfo, MethodInfo, AttributeInfo, JavaVersion, LiteralInfo, ClassFileReader};

const JAVAP_FILE_NOT_FOUND: i32 = 1;
const JAVA_MAGIC: u32 = 0xcafebabe;

pub enum Dump {
    Hex,
    Byte,
    None,
}

// todo provide iterators so can be private again ???
pub struct ClassFile {
    version             : JavaVersion,
    constant_pool       : ConstantPool,
    _access_flags       : u16, // to do  AccessFlags struct??
    _this_class         : Index,
    _super_class        : Index,
    interfaces          : Vec<Index>,
    fields              : Vec<FieldInfo>,
    methods             : Vec<MethodInfo>,
    pub attributes      : Vec<AttributeInfo>,
}

// needs to return a Result<ClassFile><Error>  for any errors found..

impl ClassFile {
    pub fn new(file_name: &String, dump: Dump) -> Self {
        let mut reader = ClassFileReader::new(file_name, dump);
        if reader.context("magic").read_u32() != JAVA_MAGIC {
            eprintln!("jcfr: Not a java class file {}", reader.file_name);
                        std::process::exit(JAVAP_FILE_NOT_FOUND);
        };

        Self {
            version         : JavaVersion(reader.context("minor").read_u16(), reader.context("major").read_u16()),
            constant_pool   : ConstantPool::new(&mut reader),
            _access_flags   : reader.context("access flags").read_u16(),
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
              attributes: self.get_attributes_vec(&m.attributes),
           }
        })
        .collect()
   }

   pub fn get_class_attributes(&self) -> ClassAttributes {
       ClassAttributes::new(self)
   }

   pub fn get_attributes_vec(&self, a: &Vec<AttributeInfo>) -> Vec<Attribute> {
      a.iter().map(|i| {
          Attribute {
               name: self.constant_pool.get_item(&i.attribute_name_index),
               info: i.info.to_owned(),
         }
     })
     .collect()
   }

   // get_method_attributes(&self) -> Vec<Attribute> {
       // self.attributes.iter().map(|m| {
           // Attribute {
              // name: self.constant_pool.get_item(&m.attribute_name_index),
              // info: m.info.to_owned(),
           // }
        // })
        // .collect()
   // }

} // ClassFile

pub struct Attribute {
    pub name: String,
    pub info: Vec<u8>,
}

pub struct Method {
    flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>, 
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
    pub fn get_attributes(&self) -> &Vec<Attribute>
    {
        &self.attributes
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

// runtime annotation
#[derive(Debug)]
pub struct Annotation {
    r#type: String,
    value_pair: Vec<ValuePair>,
}

impl Annotation {
    fn new(reader: &mut AnnotationReader, class_file: &ClassFile) -> Self {
        let index = reader.read_u16();
        let r#type = class_file.constant_pool.get_item(&Index::Single(index));
        let mut pairs = Vec::<ValuePair>::new();
        
        let num = reader.read_u16();

        for _ in 0..num {
            pairs.push(ValuePair::new(reader, class_file));
        }

        Self {
            r#type,
            value_pair: pairs,
        }
    }
}

#[derive(Debug)]
struct ValuePair {
    name: String,
    // value_index: u16, // enum of possible values
    value: LiteralInfo,
}

impl ValuePair {
    fn new(reader: &mut AnnotationReader, class_file: &ClassFile) -> Self {
        let index = reader.read_u16();
        let name = class_file.constant_pool.get_item(&Index::Single(index));
        let tag = char::from(reader.read_u8());

        Self {
            name,
            value : match tag {
                'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' =>  {
                    class_file.constant_pool.get_literal(reader.read_u16()).clone()
                }
                _ => todo!(),
            },
        }
    }
}

#[derive(Default)]
pub struct ClassAttributes {
    pub source_file: Option<String>,
    pub runtime_visible_annotations: Option<Vec<Annotation>>,
    // pub bootstrap_methods: Option<Vec<BootStrapMethods>>,
    // etc
}

impl ClassAttributes {
    fn new(class_file: &ClassFile) -> Self {
        let mut source_file : Option<String> = None;
        let mut runtime_visible_annotations : Option<Vec<Annotation>> = None;

        for a in &class_file.attributes {
            let name = &class_file.constant_pool.get_item(&a.attribute_name_index);
                match name.as_str() {
                    "SourceFile" => {
                        let index = u16::from_be_bytes(a.info[0..2].try_into().unwrap());
                        source_file = Some(class_file.constant_pool.get_item(&Index::Single(index)));
                    },
                    "RuntimeVisibleAnnotations" => { 
                        runtime_visible_annotations = Some(ClassAttributes::get_annotations(class_file, &a.info));
                    }
                    "SourceDebugExtension" => (),
                    "InnerClasses" => (),
                    "BootStrapMethods" => {
                        // bootstrap_methods = Some(ClassAttributes::get_bootstrap_methods(class_file, &a.info));
                    }
                    &_ => todo!("{}", name),
            }
        }
        Self {
           source_file, runtime_visible_annotations,
        }
    }

    fn get_annotations(class_file: &ClassFile, info: &Vec<u8>) -> Vec<Annotation> {
        let mut reader = AnnotationReader::new(info);
        let num = reader.read_u16();
        let mut annotations = Vec::<Annotation>::with_capacity(num as usize);
        for _ in 0..num {
            let annotation = Annotation::new(&mut reader, class_file);
            annotations.push(annotation);
        }
        annotations
    }
}

struct AnnotationReader<'a> {
    bytes: &'a [u8],
}

impl <'a>AnnotationReader<'a> {
    fn new(info: &'a Vec<u8>) -> Self {
       Self {
           bytes: info 
       }
    }

    fn read_u16(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.bytes.read_exact(&mut buf).unwrap();
        u16::from_be_bytes(buf)
    }

    fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.bytes.read_exact(&mut buf).unwrap();
        u8::from_be_bytes(buf)
    }
}

