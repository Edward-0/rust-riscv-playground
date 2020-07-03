#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

//extern crate panic_halt;

extern crate alloc;

#[allow(unused_imports)]
use alloc::vec::Vec;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::hal::serial::{Serial, Rx, Tx, UartX};
use hifive1::hal::delay::Sleep;
use hifive1::{sprintln, pins, pin};

mod runtime;

struct MidiMessage {
	op: u8,
	a1: Option<u8>,
	a2: Option<u8>,
}

fn read_message<T: UartX>(rx: &mut Rx<T>) -> Option<MidiMessage> {
	let in_op = rx.read().unwrap();
	if in_op >> 7 != 0 {
		let in_a1 = None;
		let in_a2 = None;
		Some(MidiMessage {
			op: in_op,
			a1: in_a1,
			a2: in_a2,
		})
	} else {
		None
	}
}

fn write_message<T: UartX>(tx: &mut Tx<T>, message: MidiMessage) {
	tx.write(message.op).unwrap()
}

trait MidiHandler {

	fn handle(&mut self, message: MidiMessage) {
		match message.op >> 4 {
			0b0000_1000 => {	
				self.handle_note_off(
					message.op & 0b0000_1111, 
					message.a1.unwrap(), 
					message.a2.unwrap()
				);
			},
			0b0000_1001 => {
				self.handle_note_on(
					message.op & 0b0000_1111,
					message.a1.unwrap(),
					message.a2.unwrap(),
				);
			},
			_ => {
				"test";
			}
		};
	}

	fn handle_note_off(&mut self, channel: u8, note: u8, velocity: u8);

	fn handle_note_on(&mut self, channel: u8, note: u8, velocity: u8);
}

struct BasicMidiHandler {
	last_note: u8,
	last_velocity: u8,
}



impl MidiHandler for BasicMidiHandler {


	fn handle_note_off(&mut self, _channel: u8, note: u8, velocity: u8) {
		if self.last_note == note && velocity >= self.last_velocity {
			//TODO handle note off
		} 
	}
	
	fn handle_note_on(&mut self, _channel: u8, note: u8, velocity: u8) {
		self.last_note = note;
		self.last_velocity = velocity;
	}
}

fn launch<T: UartX>(rx: &mut Rx<T>, handler: &mut dyn MidiHandler) {	
	let mut op: u8 = 0;
	while op < 0b1000_0000 {
		op = rx.read().unwrap();
	}
	let mut a1: u8;
	let mut a2: u8;
	loop {
		a1 = rx.read().unwrap();
		if a1 < 0b1000_0000 {
			a2 = rx.read().unwrap();
			if a2 < 0b1000_0000 {
				handler.handle(MidiMessage {
					op: op,
					a1: Some(a1),
					a2: Some(a2),
				});
				op = rx.read().unwrap();
			} else {
				handler.handle(MidiMessage {
					op: op,
					a1: Some(a1),
					a2: None,
				});
				op = a2;
			}
			
		} else {
			handler.handle(MidiMessage {
				op: op,
				a1: None,
				a2: None,
			});
			op = a1;
		}
	}
}



#[entry]
fn main() -> ! {
	let dr = DeviceResources::take().unwrap();
	let p = dr.peripherals;
	let pins = dr.pins;

//	let midi_serial = Serial::new(p.UART0, (midi_tx.into_iof0(), midi_rx.into_iof0()));

	// Configure clocks
	let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

//	let (midi_tx, midi_rx) = pins!(pins, (dig2, dig7));
	
//	let midi_serial = Serial::new(p.UART1, (midi_tx.into_iof0(), midi_rx.into_iof0()), 31_250.bps(), clocks);

//	let (mut midi_tx, mut midi_rx) = midi_serial.split();

//	write_message(&mut midi_tx, MidiMessage {op: 0b1000_0001, a1: Some(0b0000_1111), a2: Some(0b0111_0000)});

//	let _ = read_message(&mut midi_rx);

	// Configure UART for stdout
	hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 9600.bps(), clocks);


	let mut sleep = Sleep::new(dr.core_peripherals.clint.mtimecmp, clocks); 	

	sprintln!("Hello, World!");


	const PERIOD: u32 = 1000;
	
	loop {
		sleep.delay_ms(PERIOD);
		sprintln!("Hello, World");
		panic!("Panic!");
	}
}

