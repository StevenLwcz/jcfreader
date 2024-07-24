// mod class_file_reader;
// use crate::java_class_file::class_file_reader::{AttributeInfo};
use std::io::Read;

struct Exception {
    start_pc: u16,
    end_pc: u16,
    hander_pc: u16,
    catch_type: u16,
}

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    pub code: Vec<String>,
    // _exception_table: Vec<Exception>,
    // _attributes: Vec<AttributeInfo>,
}

impl Code {
   pub fn new(info: &Vec<u8>) -> Code {
       let mut reader = CodeReader::new(info);
       Self {
           max_stack : reader.read_u16(),
           max_locals : reader.read_u16(),
           code: {
               let code_length = reader.read_u32();
               let mut v = Vec::<String>::with_capacity(code_length as usize);
               for _ in 0..code_length {
                   v.push(reader.read_u8().to_string())
               }
               v
         }
      }
    }
}

struct CodeReader<'a> {
    bytes: &'a [u8],
}

impl <'a>CodeReader<'a> {
    fn new(info: &'a Vec<u8>) -> Self {
       Self {
           bytes: info
       }
    }

    fn read_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.bytes.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
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

