extern crate cpuio;

use cpuio::UnsafePort;

struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: UnsafePort::new(0x20),
                    data: UnsafePort::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: UnsafePort::new(0xA0),
                    data: UnsafePort::new(0xA1),
                },
            ]
        }
    }
    pub unsafe fn initialize(&mut self) {
        let mut wait_port: cpuio::Port<u8> = cpuio::Port::new(0x80);
        let mut wait = || { wait_port.write(0) };

        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Send initialization command and data bytes.
        // ...

        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
		
		// Send command: Begin 3-byte initialization sequence.
		self.pics[0].command.write(CMD_INIT);
		wait();
		self.pics[1].command.write(CMD_INIT);
		wait();

		// Send data 1: Set interrupt offset.
		self.pics[0].data.write(self.pics[0].offset);
		wait();
		self.pics[1].data.write(self.pics[1].offset);
		wait();
	}
}
