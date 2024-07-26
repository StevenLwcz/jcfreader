use crate::code::CodeReader;

#[derive(Debug)]
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
        Invokestatic,	// 184	Invoke static method
        Invokevirtual,	// 182	Invoke instance method, dispatch based on class
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
        Ldcw(u16),	// 19	Push item from constant pool (wide index)
        Ldc2w(u16),	// 20	Push long or double from constant pool (wide index)
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
        New,	// 187	Create new object
        Newarray(u8),	// 188	Create new array
        Nop,	// 0	Do nothing
        Pop,	// 87	Pop the top operand stack value
        Pop2,	// 88	Pop the top one or two operand stack values
        Putfield,	// 181	Set field in object
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
        let opcode = match code {
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
            19 => Opcode::Ldcw(reader.read_u16()),
            20 => Opcode::Ldc2w(reader.read_u16()),
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
            181 => Opcode::Putfield,
            182 => Opcode::Invokevirtual,
            183 => Opcode::Invokespecial(reader.read_u16()),
            184 => Opcode::Invokestatic,
            185 => Opcode::Invokeinterface,
            186 => Opcode::Invokedynamic,
            187 => Opcode::New,
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
         };
         opcode
    }
