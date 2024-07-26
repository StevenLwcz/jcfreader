// module code

mod opcode;
use std::io::Read;
use jcfreader::class_file_reader::{AttributeInfo};
use opcode::Opcode;

struct Exception {
    start_pc: u16,
    end_pc: u16,
    hander_pc: u16,
    catch_type: u16,
}

pub struct Code {
    max_stack: u16,
    max_locals: u16,
    // pub code: Vec<String>,
    pub code: Vec<Opcode>,
    exception_table: Vec<Exception>,
    attributes: Vec<AttributeInfo>,
}

impl Code {
    pub fn new(info: &Vec<u8>) -> Code {
        let mut reader = CodeReader::new(info);
        Self {
            max_stack : reader.read_u16(),
            max_locals : reader.read_u16(),
            code: {
                let code_length = reader.read_u32();
                println!("Self code code length {}", code_length);
                let mut v = Vec::<Opcode>::with_capacity(code_length as usize);
                reader.set_length(code_length);
                while reader.has_bytes() {
                    let o = opcode::get_opcode(reader.read_u8(), &mut reader);
                    v.push(o);
                }
                v
            },
            exception_table: {
                let exception_table_length = reader.read_u16();
                println!("Exception table length: {}", exception_table_length);
                let mut v = Vec::<Exception>::with_capacity(exception_table_length as usize);
                v
            },
            attributes: {
                let attributes_count = reader.read_u16();
                println!("attribute count: {}", attributes_count);
                let mut v = Vec::<AttributeInfo>::with_capacity(attributes_count  as usize);
                v
            },
        }
    }
}

pub struct CodeReader<'a> {
    count: u32,
    length: u32,
    bytes: &'a [u8],
}

impl <'a>CodeReader<'a> {
    fn new(info: &'a Vec<u8>) -> Self {
       Self {
           count: 0,
           length: 0,
           bytes: info,
       }
    }

    fn has_bytes(&mut self) -> bool {
        self.count < self.length
    }

    fn set_length(&mut self, len: u32) {
        self.count = 0;
        self.length = len;
    }

    fn read_u32(&mut self) -> u32 {
        self.count += 2;
        let mut buf = [0; 4];
        self.bytes.read_exact(&mut buf).unwrap();
        u32::from_be_bytes(buf)
    }

    fn read_u16(&mut self) -> u16 {
        self.count += 2;
        let mut buf = [0; 2];
        self.bytes.read_exact(&mut buf).unwrap();
        u16::from_be_bytes(buf)
    }

    fn read_u8(&mut self) -> u8 {
        self.count += 1;
        let mut buf = [0; 1];
        self.bytes.read_exact(&mut buf).unwrap();
        u8::from_be_bytes(buf)
    }
}

