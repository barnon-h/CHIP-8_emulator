// CPU Structre for the Chip-8
pub struct CPU
{
    pub memory : [ u8; 4096 ],      // 4 kb memory
    pub display : [ bool; 64 * 32 ],  // 64 x 32 monochrome display
    pub pc : u16,                   // Program Counter
    pub i : u16,                    // Index Register
    pub stack : [ u16; 16 ],        // Call stack
    pub sp : u8,                    // Stack Pointer
    pub delay_timer : u8,           // Delay Timer
    pub sound_timer : u8,           // Sound Timer
    pub v : [ u8; 16 ],             // General Purpose register V0 to VF
    pub keyboard : [ bool; 16 ],    // Hexadecimal keypad state
    pub draw_flag : bool,           // Flag to draw display
}

// CPU implementation
impl CPU
{
    // Constructor
    pub fn new() -> Self
    {
        let mut cpu = Self
        {
            // Initialize all fields to 0 or false, program counter starts at 0x200
            memory: [ 0; 4096 ],
            display: [ false; 64 * 32 ],
            pc: 0x200,
            i: 0,
            stack: [ 0; 16 ],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [ 0; 16 ],
            keyboard: [ false; 16 ],
            draw_flag: false,
        };

        // Load fonts
        cpu.load_fonts();
        cpu
    }

    // Load Fonts into memory
    fn load_fonts( &mut self )
    {
        // 16 x 5 pixel font
        let fonts : [ u8; 80 ] = [0xF0, 0x90, 0x90, 0x90, 0xF0,  // 0
                                  0x20, 0x60, 0x20, 0x20, 0x70,  // 1
                                  0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
                                  0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
                                  0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
                                  0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
                                  0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
                                  0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
                                  0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
                                  0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
                                  0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
                                  0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
                                  0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
                                  0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
                                  0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
                                  0xF0, 0x80, 0xF0, 0x80, 0x80]; // F

        // Load fonts into memory
        self.memory[ 0x50..0x50 + 80 ].copy_from_slice( &fonts );
    }

    // Load ROM
    pub fn load_rom( &mut self, path: &str )
    {
        // Read ROM file into memory starting at 0x200
        let rom = std::fs::read( path ).unwrap();
        self.memory[ 0x200..0x200 + rom.len() ].copy_from_slice( &rom );
    }

    // Fetch, Decode and Execute cycle

    pub fn cycle( &mut self )
    {
        let op_code = self.fetch();
        self.execute( op_code );
    }

    fn fetch( &mut self ) -> u16
    {
        // Read 2 bytes from memory at pc
        let hi = self.memory[ self.pc as usize ] as u16;
        let lo = self.memory[ self.pc as usize + 1 ] as u16;

        // Increment program counter by 2
        self.pc += 2;

        // Combine hi and lo to form the opcode
        ( hi << 8 ) | lo
    }

    fn execute( &mut self, opcode : u16 )
    {
        let nibbles = (
            ( opcode & 0xF000 ) >> 12,
            ( opcode & 0x0F00 ) >> 8,
            ( opcode & 0x00F0 ) >> 4,
            ( opcode & 0x000F )
        );

        // Decode common values
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as u8;

        // low byte and low 12 bits
        let nn = (opcode & 0x00ff ) as u8;
        let nnn = (opcode & 0x0fff ) as u16;

        // Match opcode to instruction
        match nibbles
        {
            //(1) Clear Screen
            ( 0x0, 0x0, 0xE, 0x0 ) => self.op_00e0(),

            //(2) Return from subroutine
            ( 0x0, 0x0, 0xE, 0xE ) => self.op_00ee(),

            //(3) Jump to address NNN
            ( 0x1, _, _, _ ) => self.op_1nnn( nnn ),

            //(4) Call subroutine at NNN
            ( 0x2, _, _, _ ) => self.op_2nnn( nnn ),

            //(5) Skip instruction if Vx == NN
            ( 0x3, _, _, _ ) => self.op_3xnn( x, nn ),

            //(6) Skip instruction if Vx != NN
            ( 0x4, _, _, _ ) => self.op_4xnn( x, nn ),

            //(7) Skip instruction if Vx == Vy
            ( 0x5, _, _, 0x0 ) => self.op_5xy0( x, y ),

            //(8) Set Vx = NN
            ( 0x6, _, _, _ ) => self.op_6xnn( x, nn ),

            //(9) Set Vx = Vx + NN
            ( 0x7, _, _, _ ) => self.op_7xnn( x, nn ),

            //(10) Set Vx = Vy
            ( 0x8, _, _, 0x0 ) => self.op_8xy0( x, y ),

            //(11) Set Vx = Vx OR Vy
            ( 0x8, _, _, 0x1 ) => self.op_8xy1( x, y ),

            //(12) Set Vx = Vx AND Vy
            ( 0x8, _, _, 0x2 ) => self.op_8xy2( x, y ),

            //(13) Set Vx = Vx XOR Vy
            ( 0x8, _, _, 0x3 ) => self.op_8xy3( x, y ),

            //(14) Set Vx = Vx + Vy, set VF = carry
            ( 0x8, _, _, 0x4 ) => self.op_8xy4( x, y ),

            //(15) Set Vx = Vx - Vy, set VF = NOT borrow
            ( 0x8, _, _, 0x5 ) => self.op_8xy5( x, y ),

            //(16) Shift VX right by 1, store LSB in VF before shift
            ( 0x8, _, _, 0x6 ) => self.op_8xy6( x, y ),

            //(17) Set Vx = Vy - Vx, set VF = NOT borrow
            ( 0x8, _, _, 0x7 ) => self.op_8xy7( x, y ),

            //(18) 8xye
            ( 0x8, _, _, 0xE ) => self.op_8xye( x, y ),

            //(19) Set Vx = Vy
            ( 0x9, _, _, 0x0 ) => self.op_9xy0( x, y ),

            //(20) Set I = NNN
            ( 0xA, _, _, _ ) => self.op_annn( nnn ),

            //(21) Jump to address NNN + V0
            ( 0xB, _, _, _ ) => self.op_bnnn( nnn ),

            //(22) draw
            ( 0xD, _, _, _) => self.op_dxyn( x, y, n ),

            //(23) Skip if key in VX is pressed
            ( 0xE, _, 0x9, 0xE ) => self.op_ex9e( x ),

            //(24) Skip if key in VX is NOT pressed
            ( 0xE, _, 0xA, 0x1 ) => self.op_exa1( x ),

            //(25) Set VX = delay timer
            ( 0xF, _, 0x0, 0x7 ) => self.op_fx07( x ),

            //(26) Wait for keypress, store in VX (blocking)
            ( 0xF, _, 0x0, 0xA ) => self.op_fx0a( x ),

            //(27) Set delay timer = VX
            ( 0xF, _, 0x1, 0x5 ) => self.op_fx15( x ),

            //(28) Set sound timer = VX
            ( 0xF, _, 0x1, 0x8 ) => self.op_fx18( x ),

            //(29) Set I = I + VX
            ( 0xF, _, 0x1, 0xE ) => self.op_fx1e( x ),

            //(30) Set I = font address for digit in VX
            ( 0xF, _, 0x2, 0x9 ) => self.op_fx29( x ),

            //(31) Store BCD of VX at I, I+1, I+2
            ( 0xF, _, 0x3, 0x3 ) => self.op_fx33( x ),

            //(32) Dump V0-VX to memory at I
            ( 0xF, _, 0x5, 0x5 ) => self.op_fx55( x ),

            //(33) Load memory at I into V0-VX
            ( 0xF, _, 0x6, 0x5 ) => self.op_fx65( x ),

            //(34) Set Vx = random byte AND NN
            ( 0xC, _, _, _ ) => self.op_cxnn(x, nn),

            // Unknown opcode
            _ => println!( "[!] Unknown opcode: {:06X}", opcode )
        }
    }

    // opcode implementations

    // (1) Clear Display
    fn op_00e0( &mut self )
    {
        // set all pixels to false and set draw flag
        self.display = [ false; 64 * 32 ];
        self.draw_flag = true;
    }

    // (2) Return function
    fn op_00ee( &mut self )
    {
        // Decrement stack pointer and set program counter to value on top of stack
        self.sp -= 1;
        self.pc = self.stack[ self.sp as usize ];
    }

    // (3) Jump to address NNN
    fn op_1nnn( &mut self, nnn : u16 )
    {
        // Set program counter to NNN
        self.pc = nnn;
    }

    // (4) Call subroutine at NNN
    fn op_2nnn( &mut self, nnn : u16 )
    {
        // Push current program counter to stack and increment stack pointer
        self.stack[ self.sp as usize ] = self.pc;
        self.sp += 1;

        // Set program counter to NNN
        self.pc = nnn;
    }

    // (5) Skip instruction
    fn op_3xnn( &mut self, x : usize, nn : u8 )
    {
        // Skip next instruction if Vx == NN
        if self.v[ x ] == nn
        {
            self.pc += 2;
        }
    }

    // (6) TODO
    fn op_4xnn( &mut self, x : usize, nn : u8 )
    {
        if self.v[ x ] != nn
        {
            self.pc += 2
        }
    }

    // (7) TODO
    fn op_5xy0( &mut self, x : usize, y : usize )
    {
        if self. v[ x ] == self.v[ y ]
        {
            self.pc += 2
        }
    }

    // (8)
    fn op_6xnn( &mut self, x : usize, nn : u8 )
    {
        self.v[ x ] = nn;
    }

    // (9)
    fn op_7xnn( &mut self, x : usize, nn : u8 )
    {
        self.v[ x ] = self.v[ x ].wrapping_add( nn )
    }

    // (10)
    fn op_9xy0( &mut self, x : usize, y : usize )
    {
        if self.v[ x ] != self.v[ y ]
        {
            self.pc += 2;
        }
    }

    // (11)
    fn op_annn( &mut self, nnn : u16 )
    {
        self.i = nnn;
    }

    // (12)
    fn op_bnnn( &mut self, nnn : u16 )
    {
        self.pc = nnn + self.v[ 0 ]  as u16;
    }

    // (13)
    fn op_8xy0( &mut self, x : usize, y : usize )
    {
        self.v[ x ] = self.v[ y ];
    }

    // (14)
    fn op_8xy1( &mut self, x : usize, y : usize )
    {
        self.v[ x ] |= self.v[ y ];
    }

    // (15)
    fn op_8xy2( &mut self, x : usize, y : usize )
    {
        self.v[ x ] &= self.v[ y ];
    }

    // (16)
    fn op_8xy3( &mut self, x : usize, y : usize )
    {
        self.v[ x ] ^= self.v[ y ];
    }

    // (17)
    fn op_8xy4( &mut self, x : usize, y : usize )
    {
        let ( result, overflow) = self.v[ x ].overflowing_add( self.v[ y ]);
        self.v[ x ] = result;
        self.v[ 0xF ] = overflow as u8;
    }

    // (18)
    fn op_8xy5( &mut self, x : usize, y : usize )
    {
        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = !borrow as u8;
    }

    // (19)
    fn op_8xy6( &mut self, x : usize, _y : usize )
    {
        self.v[ 0xF ] = self.v[ x ] & 0x1;
        self.v[x] >>= 1;
    }

    // (20)
    fn op_8xy7( &mut self, x : usize, y : usize )
    {
        let ( result, borrow) = self.v[ y ].overflowing_sub( self.v[ x ]);
        self.v[ x ] = result;
        self.v[ 0xF ] = !borrow as u8;
    }

    // (21)
    fn op_8xye( &mut self, x : usize, _y : usize )
    {
        self.v[ 0xF ] = (self.v[ x ] >> 7) & 0x1;
        self.v[ x ] <<= 1;
    }

    // (22) Draw
    fn op_dxyn( &mut self, x : usize, y : usize, n : u8 )
    {
        let x_pos = self.v[ x ] as usize % 64;
        let y_pos = self.v[ y ] as usize % 32;
        self.v[0xF] = 0;

        for row in 0..n as usize
        {
            let sprite_byte = self.memory[ self.i as usize + row ];

            for col in 0..8
            {
                let pixel = ( sprite_byte >> ( 7 - col )) & 0x1;

                let x_wrapped = ( x_pos + col ) % 64;
                let y_wrapped = ( y_pos + row ) % 32;

                // Turn 2d coordinates to 1d framebuffer index
                let idx = y_wrapped * 64 + x_wrapped;

                if pixel == 1
                {
                    if self.display[ idx ] 
                    {
                        self.v[ 0xF ] = 1; // collision
                    }
                    self.display[ idx ] ^= true;
                }
            }
        }
        self.draw_flag = true;
    }

    // (23) skip if pressed
    fn op_ex9e( &mut self, x : usize )
    {
        if self.keyboard[ self.v[ x ] as usize ]
        {
            self.pc += 2;
        }
    }

    // (24) skip if not pressed
    fn op_exa1( &mut self, x : usize )
    {
        if !self.keyboard[ self.v[ x ] as usize ]
        {
            self.pc += 2;
        }
    }

    // (25) sets VX to current value in delay timer
    fn op_fx07( &mut self, x : usize )
    {
        self.v[ x ] = self.delay_timer;
    }

    // (26) Wait function
    fn op_fx0a( &mut self, x : usize )
    {
        // halt execution till button press
        let mut pressed = false;
        for i in 0..16
        {
            if self.keyboard[ i ]
            {
                self.v[ x ] = i as u8;
                pressed = true;
                break;
            }
        }

        // repeat this function of not pressed
        if !pressed
        {
            self.pc -= 2
        }
    }

    // (27) Set delay timer to VX
    fn op_fx15( &mut self, x : usize )
    {
        self.delay_timer = self.v[ x ];
    }

    // (28) set sound timer to value in VX
    fn op_fx18( &mut self, x : usize )
    {
        self.sound_timer = self.v[ x ];
    }

    // (29) Add to Index
    fn op_fx1e( &mut self, x : usize )
    {
        self.i += self.v[ x ] as u16;
    }

    // (30) Font Character
    fn op_fx29( &mut self, x : usize )
    {
        // Point I to font sprite in VX
        self.i = self.v[ x ] as u16 * 5;
    }

    // (31) Binarary-coded Decimal conversion
    fn op_fx33( &mut self, x : usize )
    {
        // e.g 123 = 1, 2, 3
        let val = self.v[ x ];
        self.memory[ self.i as usize ] = val / 100; // 1
        self.memory[ self.i as usize + 1 ] = ( val / 10 ) % 10; // 2
        self.memory[ self.i as usize + 2 ] = val % 10; // 3
    }

    // (32) Store registors V0 to VX to memory starting as I
    fn op_fx55( &mut self, x : usize )
    {
        for idx in 0 ..= x
        {
            self.memory[ self.i as usize + idx ] = self.v[ idx ]
        }
    }

    // (33) Load memory starting at I into regesters V0 through VX
    fn op_fx65( &mut self, x : usize )
    {
        for idx in 0 ..= x
        {
            self.v[ idx ] = self.memory[ self.i as usize + idx ];
        }
    }

    // (34) set Vx = random byte AND NN
    fn op_cxnn( &mut self, x : usize, nn : u8 )
    {
        let random : u8 = rand::random();
        self.v[ x ] = random & nn;
    }

}