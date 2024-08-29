use crate::code::CodeReader;
use std::fmt;

pub enum Opcode {
        Aaload,	// 50	Load reference from array
        Aastore,	// 83	Store into reference array
        AconstNull,	// 1	Push null
        Aload(u8),	// 25	Load reference from local variable
        Aload0,	// 42	Load reference from local variable 0
        Aload1,	// 43	Load reference from local variable 1
        Aload2,	// 44	Load reference from local variable 2
        Aload3,	// 45	Load reference from local variable 3
        Anewarray(u16),	// 189	Create new array of reference
        Areturn,	// 176	Return reference from method
        Arraylength,	// 190	Get length of array
        Astore(u8),	// 58	Store reference into local variable
        Astore0,	// 75	Store reference into local variable 0
        Astore1,	// 76	Store reference into local variable 1
        Astore2,	// 77	Store reference into local variable 2
        Astore3,	// 78	Store reference into local variable 3
        Athrow,	// 191	Throw Throwable reference
        Baload,	// 51	Load byte or boolean from array
        Bastore,	// 84	Store into byte or boolean array
        Bipush(u8),	// 16	Push byte
        Breakpoint,	// 202	Reserved for internal usage in debuggers
        Caload,	// 52	Load char from array
        Castore,	// 85	Store into char array
        Checkcast(u16),	// 192	Check whether reference is of given type
        D2f,	// 144	Convert double to float
        D2i,	// 142	Convert double to int
        D2l,	// 143	Convert double to long
        Dadd,	// 99	Add double
        Daload,	// 49	Load double from array
        Dastore,	// 82	Store into double array
        Dcmpg,	// 152	Compare double
        Dcmpl,	// 151	Compare double
        Dconst0,	// 14	Push double value 0.0
        Dconst1,	// 15	Push double value 1.0
        Ddiv,	// 111	Divide double
        Dload(u8),	// 24	Load double from local variable
        Dload0,	// 38	Load double from local variable 0
        Dload1,	// 39	Load double from local variable 1
        Dload2,	// 40	Load double from local variable 2
        Dload3,	// 41	Load double from local variable 3
        Dmul,	// 107	Multiply double
        Dneg,	// 119	Negate double
        Drem,	// 115	Remainder double
        Dreturn,	// 175	Return double from method
        Dstore(u8),	// 57	Store double into local variable
        Dstore0,	// 71	Store double into local variable 0
        Dstore1,	// 72	Store double into local variable 1
        Dstore2,	// 73	Store double into local variable 2
        Dstore3,	// 74	Store double into local variable 3
        Dsub,	// 103	Subtract double
        Dup,	// 89	Duplicate the top operand stack value
        DupX1,	// 90	Duplicate the top operand stack value and insert two values down
        DupX2,	// 91	Duplicate the top operand stack value and insert two or three values down
        Dup2,	// 92	Duplicate the top one or two operand stack values
        Dup2X1,	// 93	Duplicate the top one or two operand stack values and insert two or three values down
        Dup2X2,	// 94	Duplicate the top one or two operand stack values and insert two, three, or four values down
        F2d,	// 141	Convert float to double
        F2i,	// 139	Convert float to int
        F2l,	// 140	Convert float to long
        Fadd,	// 98	Add float
        Faload,	// 48	Load float from array
        Fastore,	// 81	Store into float array
        Fcmpg,	// 150	Compare float
        Fcmpl,	// 149	Compare float
        Fconst0,	// 11	Push float value 0.0
        Fconst1,	// 12	Push float value 1.0
        Fconst2,	// 13	Push float value 2.0
        Fdiv,	// 110	Divide float
        Fload(u8),	// 23	Load float from local variable
        Fload0,	// 34	Load float from local variable 0
        Fload1,	// 35	Load float from local variable 1
        Fload2,	// 36	Load float from local variable 2
        Fload3,	// 37	Load float from local variable 3
        Fmul,	// 106	Multiply float
        Fneg,	// 118	Negate float
        Frem,	// 114	Remainder float
        Freturn,	// 174	Return float from method
        Fstore(u8),	// 56	Store float into local variable
        Fstore0,	// 67	Store float into local variable 0
        Fstore1,	// 68	Store float into local variable 1
        Fstore2,	// 69	Store float into local variable 2
        Fstore3,	// 70	Store float into local variable 3
        Fsub,	// 102	Subtract float
        Getfield(u16),	// 180	Fetch field from object
        Getstatic(u16),	// 178	Get static field from class
        Goto(u16),	// 167	Unconditional jump
        GotoW(u32),	// 200	Unconditional jump (wide index)
        I2b,	// 145	Convert int to byte
        I2c,	// 146	Convert int to char
        I2d,	// 135	Convert int to double
        I2f,	// 134	Convert int to float
        I2l,	// 133	Convert int to long
        I2s,	// 147	Convert int to short
        Iadd,	// 96	Add int
        Iaload,	// 46	Load int from array
        Iand,	// 126	Boolean AND int
        Iastore,	// 79	Store into int array
        Iconst0,	// 3	Push int constant 0
        Iconst1,	// 4	Push int constant 1
        Iconst2,	// 5	Push int constant 2
        Iconst3,	// 6	Push int constant 3
        Iconst4,	// 7	Push int constant 4
        Iconst5,	// 8	Push int constant 5
        IconstM1,	// 2	Push int constant -1
        Idiv,	// 108	Divide int
        IfAcmpeq(u16),	// 165	Jump if reference comparison succeeds
        IfAcmpne(u16),	// 166	Jump if reference comparison succeeds
        IfIcmpeq(u16),	// 159	Jump if int comparison succeeds
        IfIcmpge(u16),	// 162	Jump if int comparison succeeds
        IfIcmpgt(u16),	// 163	Jump if int comparison succeeds
        IfIcmple(u16),	// 164	Jump if int comparison succeeds
        IfIcmplt(u16),	// 161	Jump if int comparison succeeds
        IfIcmpne(u16),	// 160	Jump if int comparison succeeds
        Ifeq(u16),	// 153	Jump if int comparison with zero succeeds
        Ifge(u16),	// 156	Jump if int comparison with zero succeeds
        Ifgt(u16),	// 157	Jump if int comparison with zero succeeds
        Ifle(u16),	// 158	Jump if int comparison with zero succeeds
        Iflt(u16),	// 155	Jump if int comparison with zero succeeds
        Ifne(u16),	// 154	Jump if int comparison with zero succeeds
        Ifnonnull(u16),	// 199	Jump if reference not null
        Ifnull(u16),	// 198	Jump if reference is null
        Iinc(u16),	// 132	Increment local variable by constant
        Iload(u8),	// 21	Load int from local variable
        Iload0,	// 26	Load int from local variable
        Iload1,	// 27	Load int from local variable
        Iload2,	// 28	Load int from local variable
        Iload3,	// 29	Load int from local variable
        Impdep1,	// 254	Reserved for internal usage in JVM
        Impdep2,	// 255	Reserved for internal usage in JVM
        Imul,	// 104	Multiply int
        Ineg,	// 116	Negate int
        Instanceof,	// 193	Determine if reference is of given type
        Invokedynamic,	// 186	Invoke a dynamically-computed call site
        Invokeinterface,	// 185	Invoke interface method
        Invokespecial(u16),	// 183	Directly invoke instance (initialization) method of the current class or its supertypes
        Invokestatic(u16),	// 184	Invoke static method
        Invokevirtual(u16),	// 182	Invoke instance method, dispatch based on class
        Ior,	// 128	Boolean OR int
        Irem,	// 112	Remainder int
        Ireturn,	// 172	Return int from method
        Ishl,	// 120	Shift left int
        Ishr,	// 122	Arithmetic shift right int
        Istore,	// 54	Store int into local variable
        Istore0,	// 59	Store int into local variable 0
        Istore1,	// 60	Store int into local variable 1
        Istore2,	// 61	Store int into local variable 2
        Istore3,	// 62	Store int into local variable 3
        Isub,	// 100	Subtract int
        Iushr,	// 124	Logical shift right int
        Ixor,	// 130	Boolean XOR int
        Jsr(u16),	// 168	Jump subroutine
        JsrW(u32),	// 201	Jump subroutine (wide index)
        L2d,	// 138	Convert long to double
        L2f,	// 137	Convert long to float
        L2i,	// 136	Convert long to int
        Ladd,	// 97	Add long
        Laload,	// 47	Load long from array
        Land,	// 127	Boolean AND long
        Lastore,	// 80	Store into long array
        Lcmp,	// 148	Compare long
        Lconst0,	// 9	Push long constant
        Lconst1,	// 10	Push long constant
        Ldc(u8),	// 18	Push item from constant pool
        LdcW(u16),	// 19	Push item from constant pool (wide index)
        Ldc2W(u16),	// 20	Push long or double from constant pool (wide index)
        Ldiv,	// 109	Divide long
        Lload,	// 22	Load long from local variable
        Lload0,	// 30	Load long from local variable 0
        Lload1,	// 31	Load long from local variable 1
        Lload2,	// 32	Load long from local variable 2
        Lload3,	// 33	Load long from local variable 3
        Lmul,	// 105	Multiply long
        Lneg,	// 117	Negate long
        Lookupswitch,	// 171	Access jump table by key match and jump
        Lor,	// 129	Boolean OR long
        Lrem,	// 113	Remainder long
        Lreturn,	// 173	Return long from method
        Lshl,	// 121	Shift left long
        Lshr,	// 123	Arithmetic shift right long
        Lstore,	// 55	Store long into local variable
        Lstore0,	// 63	Store long into local variable 0
        Lstore1,	// 64	Store long into local variable 1
        Lstore2,	// 65	Store long into local variable 2
        Lstore3,	// 66	Store long into local variable 3
        Lsub,	// 101	Subtract long
        Lushr,	// 125	Logical shift right long
        Lxor,	// 131	Boolean XOR long
        Monitorenter,	// 194	Enter monitor for object
        Monitorexit,	// 195	Exit monitor for object
        Multianewarray(u16,u8),	// 197	Create new multidimensional array
        New(u16),	// 187	Create new object
        Newarray(u8),	// 188	Create new array
        Nop,	// 0	Do nothing
        Pop,	// 87	Pop the top operand stack value
        Pop2,	// 88	Pop the top one or two operand stack values
        Putfield(u16),	// 181	Set field in object
        Putstatic(u16),	// 179	Set static field in class
        Ret(u8),	// 169	Return from subroutine
        Return,	// 177	Return void from method
        Saload,	// 53	Load short from array
        Sastore,	// 86	Store into short array
        Sipush(u16),	// 17	Push short
        Swap,	// 95	Swap the top two operand stack values
        Tableswitch(u32, u32),	// 170	Access jump table by index and jump
        Wide,/* TO DO */	// 196	Extend local variable index by additional bytes
}

pub fn get_opcode(code: u8, reader: &mut CodeReader) -> Opcode {
        match code {
            0 => Opcode::Nop,
            1 => Opcode::AconstNull,
            2 => Opcode::IconstM1,
            3 => Opcode::Iconst0,
            4 => Opcode::Iconst1,
            5 => Opcode::Iconst2,
            6 => Opcode::Iconst3,
            7 => Opcode::Iconst4,
            8 => Opcode::Iconst5,
            9 => Opcode::Lconst0,
            10 => Opcode::Lconst1,
            11 => Opcode::Fconst0,
            12 => Opcode::Fconst1,
            13 => Opcode::Fconst2,
            14 => Opcode::Dconst0,
            15 => Opcode::Dconst1,
            16 => Opcode::Bipush(reader.read_u8()),
            17 => Opcode::Sipush(reader.read_u16()),
            18 => Opcode::Ldc(reader.read_u8()),
            19 => Opcode::LdcW(reader.read_u16()),
            20 => Opcode::Ldc2W(reader.read_u16()),
            21 => Opcode::Iload(reader.read_u8()),
            22 => Opcode::Lload,
            23 => Opcode::Fload(reader.read_u8()),
            24 => Opcode::Dload(reader.read_u8()),
            25 => Opcode::Aload(reader.read_u8()),
            26 => Opcode::Iload0,
            27 => Opcode::Iload1,
            28 => Opcode::Iload2,
            29 => Opcode::Iload3,
            30 => Opcode::Lload0,
            31 => Opcode::Lload1,
            32 => Opcode::Lload2,
            33 => Opcode::Lload3,
            34 => Opcode::Fload0,
            35 => Opcode::Fload1,
            36 => Opcode::Fload2,
            37 => Opcode::Fload3,
            38 => Opcode::Dload0,
            39 => Opcode::Dload1,
            40 => Opcode::Dload2,
            41 => Opcode::Dload3,
            42 => Opcode::Aload0,
            43 => Opcode::Aload1,
            44 => Opcode::Aload2,
            45 => Opcode::Aload3,
            46 => Opcode::Iaload,
            47 => Opcode::Laload,
            48 => Opcode::Faload,
            49 => Opcode::Daload,
            50 => Opcode::Aaload,
            51 => Opcode::Baload,
            52 => Opcode::Caload,
            53 => Opcode::Saload,
            54 => Opcode::Istore,
            55 => Opcode::Lstore,
            56 => Opcode::Fstore(reader.read_u8()),
            57 => Opcode::Dstore(reader.read_u8()),
            58 => Opcode::Astore(reader.read_u8()),
            59 => Opcode::Istore0,
            60 => Opcode::Istore1,
            61 => Opcode::Istore2,
            62 => Opcode::Istore3,
            63 => Opcode::Lstore0,
            64 => Opcode::Lstore1,
            65 => Opcode::Lstore2,
            66 => Opcode::Lstore3,
            67 => Opcode::Fstore0,
            68 => Opcode::Fstore1,
            69 => Opcode::Fstore2,
            70 => Opcode::Fstore3,
            71 => Opcode::Dstore0,
            72 => Opcode::Dstore1,
            73 => Opcode::Dstore2,
            74 => Opcode::Dstore3,
            75 => Opcode::Astore0,
            76 => Opcode::Astore1,
            77 => Opcode::Astore2,
            78 => Opcode::Astore3,
            79 => Opcode::Iastore,
            80 => Opcode::Lastore,
            81 => Opcode::Fastore,
            82 => Opcode::Dastore,
            83 => Opcode::Aastore,
            84 => Opcode::Bastore,
            85 => Opcode::Castore,
            86 => Opcode::Sastore,
            87 => Opcode::Pop,
            88 => Opcode::Pop2,
            89 => Opcode::Dup,
            90 => Opcode::DupX1,
            91 => Opcode::DupX2,
            92 => Opcode::Dup2,
            93 => Opcode::Dup2X1,
            94 => Opcode::Dup2X2,
            95 => Opcode::Swap,
            96 => Opcode::Iadd,
            97 => Opcode::Ladd,
            98 => Opcode::Fadd,
            99 => Opcode::Dadd,
            100 => Opcode::Isub,
            101 => Opcode::Lsub,
            102 => Opcode::Fsub,
            103 => Opcode::Dsub,
            104 => Opcode::Imul,
            105 => Opcode::Lmul,
            106 => Opcode::Fmul,
            107 => Opcode::Dmul,
            108 => Opcode::Idiv,
            109 => Opcode::Ldiv,
            110 => Opcode::Fdiv,
            111 => Opcode::Ddiv,
            112 => Opcode::Irem,
            113 => Opcode::Lrem,
            114 => Opcode::Frem,
            115 => Opcode::Drem,
            116 => Opcode::Ineg,
            117 => Opcode::Lneg,
            118 => Opcode::Fneg,
            119 => Opcode::Dneg,
            120 => Opcode::Ishl,
            121 => Opcode::Lshl,
            122 => Opcode::Ishr,
            123 => Opcode::Lshr,
            124 => Opcode::Iushr,
            125 => Opcode::Lushr,
            126 => Opcode::Iand,
            127 => Opcode::Land,
            128 => Opcode::Ior,
            129 => Opcode::Lor,
            130 => Opcode::Ixor,
            131 => Opcode::Lxor,
            132 => Opcode::Iinc(reader.read_u16()),
            133 => Opcode::I2l,
            134 => Opcode::I2f,
            135 => Opcode::I2d,
            136 => Opcode::L2i,
            137 => Opcode::L2f,
            138 => Opcode::L2d,
            139 => Opcode::F2i,
            140 => Opcode::F2l,
            141 => Opcode::F2d,
            142 => Opcode::D2i,
            143 => Opcode::D2l,
            144 => Opcode::D2f,
            145 => Opcode::I2b,
            146 => Opcode::I2c,
            147 => Opcode::I2s,
            148 => Opcode::Lcmp,
            149 => Opcode::Fcmpl,
            150 => Opcode::Fcmpg,
            151 => Opcode::Dcmpl,
            152 => Opcode::Dcmpg,
            153 => Opcode::Ifeq(reader.read_u16()),
            154 => Opcode::Ifne(reader.read_u16()),
            155 => Opcode::Iflt(reader.read_u16()),
            156 => Opcode::Ifge(reader.read_u16()),
            157 => Opcode::Ifgt(reader.read_u16()),
            158 => Opcode::Ifle(reader.read_u16()),
            159 => Opcode::IfIcmpeq(reader.read_u16()),
            160 => Opcode::IfIcmpne(reader.read_u16()),
            161 => Opcode::IfIcmplt(reader.read_u16()),
            162 => Opcode::IfIcmpge(reader.read_u16()),
            163 => Opcode::IfIcmpgt(reader.read_u16()),
            164 => Opcode::IfIcmple(reader.read_u16()),
            165 => Opcode::IfAcmpeq(reader.read_u16()),
            166 => Opcode::IfAcmpne(reader.read_u16()),
            167 => Opcode::Goto(reader.read_u16()),
            168 => Opcode::Jsr(reader.read_u16()),
            169 => Opcode::Ret(reader.read_u8()),
            170 => Opcode::Tableswitch(reader.read_u32(), reader.read_u32()), // to fix
            171 => Opcode::Lookupswitch,
            172 => Opcode::Ireturn,
            173 => Opcode::Lreturn,
            174 => Opcode::Freturn,
            175 => Opcode::Dreturn,
            176 => Opcode::Areturn,
            177 => Opcode::Return,
            178 => Opcode::Getstatic(reader.read_u16()),
            179 => Opcode::Putstatic(reader.read_u16()),
            180 => Opcode::Getfield(reader.read_u16()),
            181 => Opcode::Putfield(reader.read_u16()),
            182 => Opcode::Invokevirtual(reader.read_u16()),
            183 => Opcode::Invokespecial(reader.read_u16()),
            184 => Opcode::Invokestatic(reader.read_u16()),
            185 => Opcode::Invokeinterface,
            186 => Opcode::Invokedynamic,
            187 => Opcode::New(reader.read_u16()),
            188 => Opcode::Newarray(reader.read_u8()),
            189 => Opcode::Anewarray(reader.read_u16()),
            190 => Opcode::Arraylength,
            191 => Opcode::Athrow,
            192 => Opcode::Checkcast(reader.read_u16()),
            193 => Opcode::Instanceof,
            194 => Opcode::Monitorenter,
            195 => Opcode::Monitorexit,
            196 => Opcode::Wide,
            197 => Opcode::Multianewarray(reader.read_u16(),reader.read_u8()),
            198 => Opcode::Ifnull(reader.read_u16()),
            199 => Opcode::Ifnonnull(reader.read_u16()),
            200 => Opcode::GotoW(reader.read_u32()),
            201 => Opcode::JsrW(reader.read_u32()),
            202 => Opcode::Breakpoint,
            254 => Opcode::Impdep1,
            255 => Opcode::Impdep2,
            203_u8..=253_u8 => todo!(),
         }
    }


impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Aaload => write!(f, "aaload"),
            Opcode::Aastore => write!(f, "aastore"),
            Opcode::AconstNull => write!(f, "aconst_null"),
            Opcode::Aload(i) => write!(f, "aload {}", i),
            Opcode::Aload0 => write!(f, "aload_0"),
            Opcode::Aload1 => write!(f, "aload_1"),
            Opcode::Aload2 => write!(f, "aload_2"),
            Opcode::Aload3 => write!(f, "aload_3"),
            Opcode::Anewarray(i) => write!(f, "anewarray {}", i),
            Opcode::Areturn => write!(f, "areturn"),
            Opcode::Arraylength => write!(f, "arraylength"),
            Opcode::Astore(i) => write!(f, "astore {}", i),
            Opcode::Astore0 => write!(f, "astore_0"),
            Opcode::Astore1 => write!(f, "astore_1"),
            Opcode::Astore2 => write!(f, "astore_2"),
            Opcode::Astore3 => write!(f, "astore_3"),
            Opcode::Athrow => write!(f, "athrow"),
            Opcode::Baload => write!(f, "baload"),
            Opcode::Bastore => write!(f, "bastore"),
            Opcode::Bipush(i) => write!(f, "bipush {}", i),
            Opcode::Breakpoint => write!(f, "breakpoint"),
            Opcode::Caload => write!(f, "caload"),
            Opcode::Castore => write!(f, "castore"),
            Opcode::Checkcast(i) => write!(f, "checkcast {}", i),
            Opcode::D2f => write!(f, "d2f"),
            Opcode::D2i => write!(f, "d2i"),
            Opcode::D2l => write!(f, "d2l"),
            Opcode::Dadd => write!(f, "dadd"),
            Opcode::Daload => write!(f, "daload"),
            Opcode::Dastore => write!(f, "dastore"),
            Opcode::Dcmpg => write!(f, "dcmpg"),
            Opcode::Dcmpl => write!(f, "dcmpl"),
            Opcode::Dconst0 => write!(f, "dconst_0"),
            Opcode::Dconst1 => write!(f, "dconst_1"),
            Opcode::Ddiv => write!(f, "ddiv"),
            Opcode::Dload(i) => write!(f, "dload {}", i),
            Opcode::Dload0 => write!(f, "dload_0"),
            Opcode::Dload1 => write!(f, "dload_1"),
            Opcode::Dload2 => write!(f, "dload_2"),
            Opcode::Dload3 => write!(f, "dload_3"),
            Opcode::Dmul => write!(f, "dmul"),
            Opcode::Dneg => write!(f, "dneg"),
            Opcode::Drem => write!(f, "drem"),
            Opcode::Dreturn => write!(f, "dreturn"),
            Opcode::Dstore(i) => write!(f, "dstore {}", i),
            Opcode::Dstore0 => write!(f, "dstore_0"),
            Opcode::Dstore1 => write!(f, "dstore_1"),
            Opcode::Dstore2 => write!(f, "dstore_2"),
            Opcode::Dstore3 => write!(f, "dstore_3"),
            Opcode::Dsub => write!(f, "dsub"),
            Opcode::Dup => write!(f, "dup"),
            Opcode::DupX1 => write!(f, "dup_x1"),
            Opcode::DupX2 => write!(f, "dup_x2"),
            Opcode::Dup2 => write!(f, "dup2"),
            Opcode::Dup2X1 => write!(f, "dup2_x1"),
            Opcode::Dup2X2 => write!(f, "dup2_x2"),
            Opcode::F2d => write!(f, "f2d"),
            Opcode::F2i => write!(f, "f2i"),
            Opcode::F2l => write!(f, "f2l"),
            Opcode::Fadd => write!(f, "fadd"),
            Opcode::Faload => write!(f, "faload"),
            Opcode::Fastore => write!(f, "fastore"),
            Opcode::Fcmpg => write!(f, "fcmpg"),
            Opcode::Fcmpl => write!(f, "fcmpl"),
            Opcode::Fconst0 => write!(f, "fconst_0"),
            Opcode::Fconst1 => write!(f, "fconst_1"),
            Opcode::Fconst2 => write!(f, "fconst_2"),
            Opcode::Fdiv => write!(f, "fdiv"),
            Opcode::Fload(i) => write!(f, "fload {}", i),
            Opcode::Fload0 => write!(f, "fload_0"),
            Opcode::Fload1 => write!(f, "fload_1"),
            Opcode::Fload2 => write!(f, "fload_2"),
            Opcode::Fload3 => write!(f, "fload_3"),
            Opcode::Fmul => write!(f, "fmul"),
            Opcode::Fneg => write!(f, "fneg"),
            Opcode::Frem => write!(f, "frem"),
            Opcode::Freturn => write!(f, "freturn"),
            Opcode::Fstore(i) => write!(f, "fstore {}", i),
            Opcode::Fstore0 => write!(f, "fstore_0"),
            Opcode::Fstore1 => write!(f, "fstore_1"),
            Opcode::Fstore2 => write!(f, "fstore_2"),
            Opcode::Fstore3 => write!(f, "fstore_3"),
            Opcode::Fsub => write!(f, "fsub"),
            Opcode::Getfield(i) => write!(f, "getfield {}", i),
            Opcode::Getstatic(i) => write!(f, "getstatic {}", i),
            Opcode::Goto(i) => write!(f, "goto {}", i),
            Opcode::GotoW(i) => write!(f, "goto_w {}", i),
            Opcode::I2b => write!(f, "i2b"),
            Opcode::I2c => write!(f, "i2c"),
            Opcode::I2d => write!(f, "i2d"),
            Opcode::I2f => write!(f, "i2f"),
            Opcode::I2l => write!(f, "i2l"),
            Opcode::I2s => write!(f, "i2s"),
            Opcode::Iadd => write!(f, "iadd"),
            Opcode::Iaload => write!(f, "iaload"),
            Opcode::Iand => write!(f, "iand"),
            Opcode::Iastore => write!(f, "iastore"),
            Opcode::Iconst0 => write!(f, "iconst_0"),
            Opcode::Iconst1 => write!(f, "iconst_1"),
            Opcode::Iconst2 => write!(f, "iconst_2"),
            Opcode::Iconst3 => write!(f, "iconst_3"),
            Opcode::Iconst4 => write!(f, "iconst_4"),
            Opcode::Iconst5 => write!(f, "iconst_5"),
            Opcode::IconstM1 => write!(f, "iconst_m1"),
            Opcode::Idiv => write!(f, "idiv"),
            Opcode::IfAcmpeq(i) => write!(f, "if_acmpeq {}", i),
            Opcode::IfAcmpne(i) => write!(f, "if_acmpne {}", i),
            Opcode::IfIcmpeq(i) => write!(f, "if_icmpeq {}", i),
            Opcode::IfIcmpge(i) => write!(f, "if_icmpge {}", i),
            Opcode::IfIcmpgt(i) => write!(f, "if_icmpgt {}", i),
            Opcode::IfIcmple(i) => write!(f, "if_icmple {}", i),
            Opcode::IfIcmplt(i) => write!(f, "if_icmplt {}", i),
            Opcode::IfIcmpne(i) => write!(f, "if_icmpne {}", i),
            Opcode::Ifeq(i) => write!(f, "ifeq {}", i),
            Opcode::Ifge(i) => write!(f, "ifge {}", i),
            Opcode::Ifgt(i) => write!(f, "ifgt {}", i),
            Opcode::Ifle(i) => write!(f, "ifle {}", i),
            Opcode::Iflt(i) => write!(f, "iflt {}", i),
            Opcode::Ifne(i) => write!(f, "ifne {}", i),
            Opcode::Ifnonnull(i) => write!(f, "ifnonnull {}", i),
            Opcode::Ifnull(i) => write!(f, "ifnull {}", i),
            Opcode::Iinc(i) => write!(f, "iinc {}", i),
            Opcode::Iload(i) => write!(f, "iload {}", i),
            Opcode::Iload0 => write!(f, "iload_0"),
            Opcode::Iload1 => write!(f, "iload_1"),
            Opcode::Iload2 => write!(f, "iload_2"),
            Opcode::Iload3 => write!(f, "iload_3"),
            Opcode::Impdep1 => write!(f, "impdep1"),
            Opcode::Impdep2 => write!(f, "impdep2"),
            Opcode::Imul => write!(f, "imul"),
            Opcode::Ineg => write!(f, "ineg"),
            Opcode::Instanceof => write!(f, "instanceof"),
            Opcode::Invokedynamic => write!(f, "invokedynamic"),
            Opcode::Invokeinterface => write!(f, "invokeinterface"),
            Opcode::Invokespecial(i) => write!(f, "invokespecial {}", i),
            Opcode::Invokestatic(i) => write!(f, "invokestatic {}", i),
            Opcode::Invokevirtual(i) => write!(f, "invokevirtual {}", i),
            Opcode::Ior => write!(f, "ior"),
            Opcode::Irem => write!(f, "irem"),
            Opcode::Ireturn => write!(f, "ireturn"),
            Opcode::Ishl => write!(f, "ishl"),
            Opcode::Ishr => write!(f, "ishr"),
            Opcode::Istore => write!(f, "istore"),
            Opcode::Istore0 => write!(f, "istore_0"),
            Opcode::Istore1 => write!(f, "istore_1"),
            Opcode::Istore2 => write!(f, "istore_2"),
            Opcode::Istore3 => write!(f, "istore_3"),
            Opcode::Isub => write!(f, "isub"),
            Opcode::Iushr => write!(f, "iushr"),
            Opcode::Ixor => write!(f, "ixor"),
            Opcode::Jsr(i) => write!(f, "jsr {}", i),
            Opcode::JsrW(i) => write!(f, "jsr_w {}", i),
            Opcode::L2d => write!(f, "l2d"),
            Opcode::L2f => write!(f, "l2f"),
            Opcode::L2i => write!(f, "l2i"),
            Opcode::Ladd => write!(f, "ladd"),
            Opcode::Laload => write!(f, "laload"),
            Opcode::Land => write!(f, "land"),
            Opcode::Lastore => write!(f, "lastore"),
            Opcode::Lcmp => write!(f, "lcmp"),
            Opcode::Lconst0 => write!(f, "lconst_0"),
            Opcode::Lconst1 => write!(f, "lconst_1"),
            Opcode::Ldc(i) => write!(f, "ldc {}", i),
            Opcode::LdcW(i) => write!(f, "ldc_w {}", i),
            Opcode::Ldc2W(i) => write!(f, "ldc2_w {}", i),
            Opcode::Ldiv => write!(f, "ldiv"),
            Opcode::Lload => write!(f, "lload"),
            Opcode::Lload0 => write!(f, "lload_0"),
            Opcode::Lload1 => write!(f, "lload_1"),
            Opcode::Lload2 => write!(f, "lload_2"),
            Opcode::Lload3 => write!(f, "lload_3"),
            Opcode::Lmul => write!(f, "lmul"),
            Opcode::Lneg => write!(f, "lneg"),
            Opcode::Lookupswitch => write!(f, "lookupswitch"),
            Opcode::Lor => write!(f, "lor"),
            Opcode::Lrem => write!(f, "lrem"),
            Opcode::Lreturn => write!(f, "lreturn"),
            Opcode::Lshl => write!(f, "lshl"),
            Opcode::Lshr => write!(f, "lshr"),
            Opcode::Lstore => write!(f, "lstore"),
            Opcode::Lstore0 => write!(f, "lstore_0"),
            Opcode::Lstore1 => write!(f, "lstore_1"),
            Opcode::Lstore2 => write!(f, "lstore_2"),
            Opcode::Lstore3 => write!(f, "lstore_3"),
            Opcode::Lsub => write!(f, "lsub"),
            Opcode::Lushr => write!(f, "lushr"),
            Opcode::Lxor => write!(f, "lxor"),
            Opcode::Monitorenter => write!(f, "monitorenter"),
            Opcode::Monitorexit => write!(f, "monitorexit"),
            Opcode::Multianewarray(i,j) => write!(f, "multianewarray {} {}", i, j),
            Opcode::New(i) => write!(f, "new {}", i),
            Opcode::Newarray(i) => write!(f, "newarray {}", i),
            Opcode::Nop => write!(f, "nop"),
            Opcode::Pop => write!(f, "pop"),
            Opcode::Pop2 => write!(f, "pop2"),
            Opcode::Putfield(i) => write!(f, "putfield {}", i),
            Opcode::Putstatic(i) => write!(f, "putstatic {}", i),
            Opcode::Ret(i) => write!(f, "ret {}", i),
            Opcode::Return => write!(f, "return"),
            Opcode::Saload => write!(f, "saload"),
            Opcode::Sastore => write!(f, "sastore"),
            Opcode::Sipush(i) => write!(f, "sipush {}", i),
            Opcode::Swap => write!(f, "swap"),
            Opcode::Tableswitch(i, j) => write!(f, "tableswitch {}{}", i, j), /* TODO */
            Opcode::Wide => write!(f, "wide"),
        }
    }
}
