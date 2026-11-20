#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_050a6edc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6edc); }
 let mut pc: u32 = 0x50a6edc;
 loop { match pc {
 0x50a6edc => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ee0); }
 let mut pc: u32 = 0x50a6ee0;
 loop { match pc {
 0x50a6ee0 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ee4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ee4); }
 let mut pc: u32 = 0x50a6ee4;
 loop { match pc {
 0x50a6ee4 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ee8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ee8); }
 let mut pc: u32 = 0x50a6ee8;
 loop { match pc {
 0x50a6ee8 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6eec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6eec); }
 let mut pc: u32 = 0x50a6eec;
 loop { match pc {
 0x50a6eec => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ef0); }
 let mut pc: u32 = 0x50a6ef0;
 loop { match pc {
 0x50a6ef0 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ef4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ef4); }
 let mut pc: u32 = 0x50a6ef4;
 loop { match pc {
 0x50a6ef4 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ef8); }
 let mut pc: u32 = 0x50a6ef8;
 loop { match pc {
 0x50a6ef8 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6efc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6efc); }
 let mut pc: u32 = 0x50a6efc;
 loop { match pc {
 0x50a6efc => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f00); }
 let mut pc: u32 = 0x50a6f00;
 loop { match pc {
 0x50a6f00 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f04); }
 let mut pc: u32 = 0x50a6f04;
 loop { match pc {
 0x50a6f04 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f08); }
 let mut pc: u32 = 0x50a6f08;
 loop { match pc {
 0x50a6f08 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f0c); }
 let mut pc: u32 = 0x50a6f0c;
 loop { match pc {
 0x50a6f0c => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f10); }
 let mut pc: u32 = 0x50a6f10;
 loop { match pc {
 0x50a6f10 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f14); }
 let mut pc: u32 = 0x50a6f14;
 loop { match pc {
 0x50a6f14 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f18); }
 let mut pc: u32 = 0x50a6f18;
 loop { match pc {
 0x50a6f18 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f1c); }
 let mut pc: u32 = 0x50a6f1c;
 loop { match pc {
 0x50a6f1c => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f20); }
 let mut pc: u32 = 0x50a6f20;
 loop { match pc {
 0x50a6f20 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f24); }
 let mut pc: u32 = 0x50a6f24;
 loop { match pc {
 0x50a6f24 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f28); }
 let mut pc: u32 = 0x50a6f28;
 loop { match pc {
 0x50a6f28 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f2c); }
 let mut pc: u32 = 0x50a6f2c;
 loop { match pc {
 0x50a6f2c => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f30); }
 let mut pc: u32 = 0x50a6f30;
 loop { match pc {
 0x50a6f30 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f34); }
 let mut pc: u32 = 0x50a6f34;
 loop { match pc {
 0x50a6f34 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f38); }
 let mut pc: u32 = 0x50a6f38;
 loop { match pc {
 0x50a6f38 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f3c); }
 let mut pc: u32 = 0x50a6f3c;
 loop { match pc {
 0x50a6f3c => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f40); }
 let mut pc: u32 = 0x50a6f40;
 loop { match pc {
 0x50a6f40 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f44); }
 let mut pc: u32 = 0x50a6f44;
 loop { match pc {
 0x50a6f44 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f48); }
 let mut pc: u32 = 0x50a6f48;
 loop { match pc {
 0x50a6f48 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f4c); }
 let mut pc: u32 = 0x50a6f4c;
 loop { match pc {
 0x50a6f4c => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f50); }
 let mut pc: u32 = 0x50a6f50;
 loop { match pc {
 0x50a6f50 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f54); }
 let mut pc: u32 = 0x50a6f54;
 loop { match pc {
 0x50a6f54 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f58); }
 let mut pc: u32 = 0x50a6f58;
 loop { match pc {
 0x50a6f58 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f5c); }
 let mut pc: u32 = 0x50a6f5c;
 loop { match pc {
 0x50a6f5c => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f60); }
 let mut pc: u32 = 0x50a6f60;
 loop { match pc {
 0x50a6f60 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f64); }
 let mut pc: u32 = 0x50a6f64;
 loop { match pc {
 0x50a6f64 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f68); }
 let mut pc: u32 = 0x50a6f68;
 loop { match pc {
 0x50a6f68 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f6c); }
 let mut pc: u32 = 0x50a6f6c;
 loop { match pc {
 0x50a6f6c => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f70); }
 let mut pc: u32 = 0x50a6f70;
 loop { match pc {
 0x50a6f70 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f74); }
 let mut pc: u32 = 0x50a6f74;
 loop { match pc {
 0x50a6f74 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f78); }
 let mut pc: u32 = 0x50a6f78;
 loop { match pc {
 0x50a6f78 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f7c); }
 let mut pc: u32 = 0x50a6f7c;
 loop { match pc {
 0x50a6f7c => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f80); }
 let mut pc: u32 = 0x50a6f80;
 loop { match pc {
 0x50a6f80 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f84); }
 let mut pc: u32 = 0x50a6f84;
 loop { match pc {
 0x50a6f84 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f88); }
 let mut pc: u32 = 0x50a6f88;
 loop { match pc {
 0x50a6f88 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f8c); }
 let mut pc: u32 = 0x50a6f8c;
 loop { match pc {
 0x50a6f8c => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f90); }
 let mut pc: u32 = 0x50a6f90;
 loop { match pc {
 0x50a6f90 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f94); }
 let mut pc: u32 = 0x50a6f94;
 loop { match pc {
 0x50a6f94 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f98); }
 let mut pc: u32 = 0x50a6f98;
 loop { match pc {
 0x50a6f98 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6f9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6f9c); }
 let mut pc: u32 = 0x50a6f9c;
 loop { match pc {
 0x50a6f9c => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fa0); }
 let mut pc: u32 = 0x50a6fa0;
 loop { match pc {
 0x50a6fa0 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fa4); }
 let mut pc: u32 = 0x50a6fa4;
 loop { match pc {
 0x50a6fa4 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fa8); }
 let mut pc: u32 = 0x50a6fa8;
 loop { match pc {
 0x50a6fa8 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fac); }
 let mut pc: u32 = 0x50a6fac;
 loop { match pc {
 0x50a6fac => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fb0); }
 let mut pc: u32 = 0x50a6fb0;
 loop { match pc {
 0x50a6fb0 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fb4); }
 let mut pc: u32 = 0x50a6fb4;
 loop { match pc {
 0x50a6fb4 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fb8); }
 let mut pc: u32 = 0x50a6fb8;
 loop { match pc {
 0x50a6fb8 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fbc); }
 let mut pc: u32 = 0x50a6fbc;
 loop { match pc {
 0x50a6fbc => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fc0); }
 let mut pc: u32 = 0x50a6fc0;
 loop { match pc {
 0x50a6fc0 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fc4); }
 let mut pc: u32 = 0x50a6fc4;
 loop { match pc {
 0x50a6fc4 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fc8); }
 let mut pc: u32 = 0x50a6fc8;
 loop { match pc {
 0x50a6fc8 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fcc); }
 let mut pc: u32 = 0x50a6fcc;
 loop { match pc {
 0x50a6fcc => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fd0); }
 let mut pc: u32 = 0x50a6fd0;
 loop { match pc {
 0x50a6fd0 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fd4); }
 let mut pc: u32 = 0x50a6fd4;
 loop { match pc {
 0x50a6fd4 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fd8); }
 let mut pc: u32 = 0x50a6fd8;
 loop { match pc {
 0x50a6fd8 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fdc); }
 let mut pc: u32 = 0x50a6fdc;
 loop { match pc {
 0x50a6fdc => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fe0); }
 let mut pc: u32 = 0x50a6fe0;
 loop { match pc {
 0x50a6fe0 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fe4); }
 let mut pc: u32 = 0x50a6fe4;
 loop { match pc {
 0x50a6fe4 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fe8); }
 let mut pc: u32 = 0x50a6fe8;
 loop { match pc {
 0x50a6fe8 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6fec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6fec); }
 let mut pc: u32 = 0x50a6fec;
 loop { match pc {
 0x50a6fec => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ff0); }
 let mut pc: u32 = 0x50a6ff0;
 loop { match pc {
 0x50a6ff0 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ff4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ff4); }
 let mut pc: u32 = 0x50a6ff4;
 loop { match pc {
 0x50a6ff4 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ff8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ff8); }
 let mut pc: u32 = 0x50a6ff8;
 loop { match pc {
 0x50a6ff8 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ffc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ffc); }
 let mut pc: u32 = 0x50a6ffc;
 loop { match pc {
 0x50a6ffc => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7000); }
 let mut pc: u32 = 0x50a7000;
 loop { match pc {
 0x50a7000 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7004); }
 let mut pc: u32 = 0x50a7004;
 loop { match pc {
 0x50a7004 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7008); }
 let mut pc: u32 = 0x50a7008;
 loop { match pc {
 0x50a7008 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a700c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a700c); }
 let mut pc: u32 = 0x50a700c;
 loop { match pc {
 0x50a700c => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7010); }
 let mut pc: u32 = 0x50a7010;
 loop { match pc {
 0x50a7010 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7014); }
 let mut pc: u32 = 0x50a7014;
 loop { match pc {
 0x50a7014 => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7018); }
 let mut pc: u32 = 0x50a7018;
 loop { match pc {
 0x50a7018 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a701c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a701c); }
 let mut pc: u32 = 0x50a701c;
 loop { match pc {
 0x50a701c => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7020); }
 let mut pc: u32 = 0x50a7020;
 loop { match pc {
 0x50a7020 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7024); }
 let mut pc: u32 = 0x50a7024;
 loop { match pc {
 0x50a7024 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7028); }
 let mut pc: u32 = 0x50a7028;
 loop { match pc {
 0x50a7028 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a702c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a702c); }
 let mut pc: u32 = 0x50a702c;
 loop { match pc {
 0x50a702c => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7030); }
 let mut pc: u32 = 0x50a7030;
 loop { match pc {
 0x50a7030 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7034(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7034); }
 let mut pc: u32 = 0x50a7034;
 loop { match pc {
 0x50a7034 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7038); }
 let mut pc: u32 = 0x50a7038;
 loop { match pc {
 0x50a7038 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a703c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a703c); }
 let mut pc: u32 = 0x50a703c;
 loop { match pc {
 0x50a703c => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7040); }
 let mut pc: u32 = 0x50a7040;
 loop { match pc {
 0x50a7040 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7044(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7044); }
 let mut pc: u32 = 0x50a7044;
 loop { match pc {
 0x50a7044 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7048); }
 let mut pc: u32 = 0x50a7048;
 loop { match pc {
 0x50a7048 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a704c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a704c); }
 let mut pc: u32 = 0x50a704c;
 loop { match pc {
 0x50a704c => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7050); }
 let mut pc: u32 = 0x50a7050;
 loop { match pc {
 0x50a7050 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7054); }
 let mut pc: u32 = 0x50a7054;
 loop { match pc {
 0x50a7054 => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7058); }
 let mut pc: u32 = 0x50a7058;
 loop { match pc {
 0x50a7058 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a705c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a705c); }
 let mut pc: u32 = 0x50a705c;
 loop { match pc {
 0x50a705c => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7060); }
 let mut pc: u32 = 0x50a7060;
 loop { match pc {
 0x50a7060 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7064); }
 let mut pc: u32 = 0x50a7064;
 loop { match pc {
 0x50a7064 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7068(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7068); }
 let mut pc: u32 = 0x50a7068;
 loop { match pc {
 0x50a7068 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a706c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a706c); }
 let mut pc: u32 = 0x50a706c;
 loop { match pc {
 0x50a706c => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7070); }
 let mut pc: u32 = 0x50a7070;
 loop { match pc {
 0x50a7070 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7074); }
 let mut pc: u32 = 0x50a7074;
 loop { match pc {
 0x50a7074 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7078); }
 let mut pc: u32 = 0x50a7078;
 loop { match pc {
 0x50a7078 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a707c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a707c); }
 let mut pc: u32 = 0x50a707c;
 loop { match pc {
 0x50a707c => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7080); }
 let mut pc: u32 = 0x50a7080;
 loop { match pc {
 0x50a7080 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x50a6c80u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7096(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7096); }
 let mut pc: u32 = 0x50a7096;
 loop { match pc {
 0x50a7096 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(4, v as u16); }
  { let v = cpu.r16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.cf = false; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50a70d4); cpu.push(0x50a70d9); f_050a7080(cpu); if cpu.returned_to != 0x50a70d9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.pop(); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.and32(cpu.ebx, 0x1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x50a7127); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7128(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7128); }
 let mut pc: u32 = 0x50a7128;
 loop { match pc {
 0x50a7128 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x50a7139); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a71f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a71f6); }
 let mut pc: u32 = 0x50a71f6;
 loop { match pc {
 0x50a71f6 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x514bc20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7205(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7205); }
 let mut pc: u32 = 0x50a7205;
 loop { match pc {
 0x50a7205 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  if cpu.cf { pc = 0x50a7244; continue; }
  pc = 0x50a7227; continue;
 }
 0x50a7227 => {
  if (cpu.cf || cpu.zf) { pc = 0x50a7251; continue; }
  pc = 0x50a7229; continue;
 }
 0x50a7229 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x50a723a; continue; }
  pc = 0x50a722e; continue;
 }
 0x50a722e => {
  if (cpu.cf || cpu.zf) { pc = 0x50a7266; continue; }
  pc = 0x50a7230; continue;
 }
 0x50a7230 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x50a7251; continue; }
  pc = 0x50a7235; continue;
 }
 0x50a7235 => {
  pc = 0x50a72e3; continue;
 }
 0x50a723a => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x50a7261; continue; }
  pc = 0x50a723f; continue;
 }
 0x50a723f => {
  pc = 0x50a72e3; continue;
 }
 0x50a7244 => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x50a72e3; continue; }
  pc = 0x50a724d; continue;
 }
 0x50a724d => {
  if (cpu.cf || cpu.zf) { pc = 0x50a7261; continue; }
  pc = 0x50a724f; continue;
 }
 0x50a724f => {
  pc = 0x50a7266; continue;
 }
 0x50a7251 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x50a725e; continue; }
  pc = 0x50a7257; continue;
 }
 0x50a7257 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a725e; continue;
 }
 0x50a725e => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50a7261; continue;
 }
 0x50a7261 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x50a7266; continue;
 }
 0x50a7266 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x50a7287; continue; }
  pc = 0x50a7280; continue;
 }
 0x50a7280 => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a7287; continue;
 }
 0x50a7287 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x5fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = 0xffu32; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x50a72af); f_050ad9e5(cpu); if cpu.returned_to != 0x50a72af { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x50a72bc; continue; }
  pc = 0x50a72b8; continue;
 }
 0x50a72b8 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a72bc; continue;
 }
 0x50a72bc => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x50a72c3; continue;
 }
 0x50a72c3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x50a72cf; continue; }
  pc = 0x50a72cc; continue;
 }
 0x50a72cc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a72c3; continue;
 }
 0x50a72cf => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x50a72d8); f_050a72ef(cpu); if cpu.returned_to != 0x50a72d8 { return; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a72e3; continue;
 }
 0x50a72e3 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a72ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a72ef); }
 let mut pc: u32 = 0x50a72ef;
 loop { match pc {
 0x50a72ef => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a734c; continue; }
  pc = 0x50a7300; continue;
 }
 0x50a7300 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x50a7316; continue; }
  pc = 0x50a7305; continue;
 }
 0x50a7305 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x50a7316; continue; }
  pc = 0x50a730e; continue;
 }
 0x50a730e => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x50a7316; continue; }
  pc = 0x50a7313; continue;
 }
 0x50a7313 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50a7300; continue;
 }
 0x50a7316 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x50a732c; continue; }
  pc = 0x50a7327; continue;
 }
 0x50a7327 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x50a7338; continue; }
  pc = 0x50a732c; continue;
 }
 0x50a732c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x50a7338; continue; }
  pc = 0x50a7335; continue;
 }
 0x50a7335 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50a732c; continue;
 }
 0x50a7338 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x50a7342; continue; }
  pc = 0x50a7341; continue;
 }
 0x50a7341 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50a7342; continue;
 }
 0x50a7342 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x50a7342; continue; }
  pc = 0x50a734c; continue;
 }
 0x50a734c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a734e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a734e); }
 let mut pc: u32 = 0x50a734e;
 loop { match pc {
 0x50a734e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x50a7358); f_05085f7f(cpu); if cpu.returned_to != 0x50a7358 { return; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a735c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a735c); } r_050a735c(cpu, 0x50a735c); }
pub fn f_050a7360(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7360); } r_050a735c(cpu, 0x50a7360); }
pub fn f_050a7364(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7364); } r_050a735c(cpu, 0x50a7364); }
pub fn f_050a7366(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7366); } r_050a735c(cpu, 0x50a7366); }
pub fn f_050a7368(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7368); } r_050a735c(cpu, 0x50a7368); }
pub fn f_050a736a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a736a); } r_050a735c(cpu, 0x50a736a); }
pub fn f_050a73b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a73b6); } r_050a735c(cpu, 0x50a73b6); }
pub fn f_050a73b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a73b8); } r_050a735c(cpu, 0x50a73b8); }
pub fn f_050a73bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a73bc); } r_050a735c(cpu, 0x50a73bc); }
pub fn f_050a73be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a73be); } r_050a735c(cpu, 0x50a73be); }
fn r_050a735c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a735c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7360; continue;
 }
 0x50a7360 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7364; continue;
 }
 0x50a7364 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7366; continue;
 }
 0x50a7366 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a7368; continue;
 }
 0x50a7368 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_sub((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x46000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x50a73ca); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x50a736a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_sub((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x46000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x50a73ca); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x50a73b6 => {
  { cpu.df = false; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x50a73ca); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x50a73b8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a73bc; continue;
 }
 0x50a73bc => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a73be; continue;
 }
 0x50a73be => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x50a73ca); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7406(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7406); } r_050a7406(cpu, 0x50a7406); }
pub fn f_050a740a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a740a); } r_050a7406(cpu, 0x50a740a); }
fn r_050a7406(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a7406 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x50a740a; continue;
 }
 0x50a740a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let a = cpu.ebx; let b = cpu.ebx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.ebx = v; } }
  cpu.call(0x50a7414); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a74d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a74d4); } r_050a74d4(cpu, 0x50a74d4); }
pub fn f_050a75d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a75d4); } r_050a74d4(cpu, 0x50a75d4); }
fn r_050a74d4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a746e => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a7a7fu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x50a7474); return;
 }
 0x50a74d4 => {
  { let at = 0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edi = v; cpu.set_segment(3, selector); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7bb7u32); cpu.eax = v; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  if (cpu.sf != cpu.of) { pc = 0x50a74e9; continue; }
  pc = 0x50a74df; continue;
 }
 0x50a74df => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7d85u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a74f1; continue; }
  pc = 0x50a74e7; continue;
 }
 0x50a74e7 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7f78u32); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50a746e; continue; }
  pc = 0x50a74ee; continue;
 }
 0x50a74e9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a74f5; continue; }
  pc = 0x50a74eb; continue;
 }
 0x50a74eb => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a807eu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x7ae4050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])).wrapping_add(0xau8); cpu.w8(0x7ae4050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a74f7; continue;
 }
 0x50a74ee => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a8180u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.port_in(0x7au16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7ac0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x7cu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7ac0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a7511; continue; }
  pc = 0x50a7507; continue;
 }
 0x50a74f1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa7ae405u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a74f7; continue;
 }
 0x50a74f5 => {
  if cpu.pf() { pc = 0x50a7501; continue; }
  pc = 0x50a74f7; continue;
 }
 0x50a74f7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7ac0u32); cpu.eax = v; }
  { let v: u8 = 0x7cu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7ac0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a7511; continue; }
  pc = 0x50a7507; continue;
 }
 0x50a7501 => {
  if cpu.pf() { pc = 0x50a750d; continue; }
  pc = 0x50a7503; continue;
 }
 0x50a7503 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7e42u32); cpu.eax = v; }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  pc = 0x50a750d; continue;
 }
 0x50a7507 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7ac0u32); cpu.eax = v; }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  pc = 0x50a750d; continue;
 }
 0x50a750d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7bf1u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])), 1u32); cpu.w32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x65050a7du32; cpu.edi = v; }
  pc = 0x50a7525; continue;
 }
 0x50a7511 => {
  if cpu.pf() { pc = 0x50a751d; continue; }
  pc = 0x50a7513; continue;
 }
 0x50a7513 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7b03u32); cpu.eax = v; }
  cpu.unimplemented(0x50a7518, "Int1");
  if !cpu.pf() { pc = 0x50a7525; continue; }
  pc = 0x50a751b; continue;
 }
 0x50a751b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7cd1u32); cpu.eax = v; }
  { let v: u32 = 0x65050a7du32; cpu.edi = v; }
  pc = 0x50a7525; continue;
 }
 0x50a751d => {
  if (cpu.sf != cpu.of) { pc = 0x50a7529; continue; }
  pc = 0x50a751f; continue;
 }
 0x50a751f => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7dbfu32); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a7531; continue; }
  pc = 0x50a7527; continue;
 }
 0x50a7525 => {
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a7531; continue; }
  pc = 0x50a7527; continue;
 }
 0x50a7527 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7fb6u32); cpu.eax = v; }
  { let v: u8 = cpu.rol8(cpu.r8(0x81bc050au32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 10u32); cpu.w8(0x81bc050au32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7b1fu32); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), cpu.r8(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x50a7549; continue; }
  pc = 0x50a753f; continue;
 }
 0x50a7529 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a7535; continue; }
  pc = 0x50a752b; continue;
 }
 0x50a752b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a80c0u32); cpu.eax = v; }
  { let v: u32 = 0x1f050a81u32; cpu.esp = v; }
  pc = 0x50a7535; continue;
 }
 0x50a7531 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa7b1f05u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7537; continue;
 }
 0x50a7535 => {
  if !cpu.pf() { pc = 0x50a7541; continue; }
  pc = 0x50a7537; continue;
 }
 0x50a7537 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7c02u32); cpu.eax = v; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x50a7549; continue; }
  pc = 0x50a753f; continue;
 }
 0x50a753f => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7dfbu32); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xau32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), cpu.edi); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7fe9u32); cpu.eax = v; }
  { let v = cpu.st(0) + (cpu.r16(0x8204050au32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) as i16 as f64); cpu.set_st(0, v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a7b3bu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ebp)) | cpu.edi; cpu.w32(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x50a755f; continue;
 }
 0x50a7541 => {
  if (cpu.sf == cpu.of) { pc = 0x50a754d; continue; }
  pc = 0x50a7543; continue;
 }
 0x50a7543 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7e85u32); cpu.eax = v; }
  cpu.call(0xe30f7fcc); return;
 }
 0x50a7549 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a7555; continue; }
  pc = 0x50a754b; continue;
 }
 0x50a754b => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a80deu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x82u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a7b3bu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ebp)) | cpu.edi; cpu.w32(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x50a755f; continue;
 }
 0x50a754d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x82u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a7b3bu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ebp)) | cpu.edi; cpu.w32(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x50a755f; continue;
 }
 0x50a7555 => {
  if !cpu.pf() { pc = 0x50a7561; continue; }
  pc = 0x50a7557; continue;
 }
 0x50a7557 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7c13u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.ebp)) | cpu.edi; cpu.w32(0xau32.wrapping_add(cpu.ebp), v); }
  pc = 0x50a755f; continue;
 }
 0x50a755d => {
  if (cpu.sf == cpu.of) { pc = 0x50a7569; continue; }
  pc = 0x50a755f; continue;
 }
 0x50a755f => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7ac0u32); cpu.eax = v; }
  { cpu.movs(4, false); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a7571; continue; }
  pc = 0x50a7567; continue;
 }
 0x50a7561 => {
  if cpu.pf() { pc = 0x50a756d; continue; }
  pc = 0x50a7563; continue;
 }
 0x50a7563 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7ea5u32); cpu.eax = v; }
  cpu.call(0x50a7568); return;
 }
 0x50a7567 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7ffeu32); cpu.eax = v; }
  { cpu.df = false; }
  pc = 0x50a756d; continue;
 }
 0x50a7569 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a7575; continue; }
  pc = 0x50a756b; continue;
 }
 0x50a756b => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a80fcu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x82u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7b5au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a7578; continue;
 }
 0x50a756d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0x82u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7b5au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a7578; continue;
 }
 0x50a7571 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x50a7575; continue;
 }
 0x50a7575 => {
  if !cpu.pf() { pc = 0x50a7581; continue; }
  pc = 0x50a7577; continue;
 }
 0x50a7577 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7c31u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7f31u32); cpu.eax = v; }
  cpu.unimplemented(0x50a7584, "Enter");
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x39050a81u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  if !cpu.sf { pc = 0x50a7611; continue; }
  pc = 0x50a7596; continue;
 }
 0x50a7578 => {
  { let v: u32 = cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])) ^ cpu.edi; cpu.w32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.ebp)).wrapping_sub(((cpu.ebx >> 8) as u8)); cpu.w8(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7f31u32); cpu.eax = v; }
  cpu.unimplemented(0x50a7584, "Enter");
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x39050a81u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  if !cpu.sf { pc = 0x50a7611; continue; }
  pc = 0x50a7596; continue;
 }
 0x50a7581 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a758d; continue; }
  pc = 0x50a7583; continue;
 }
 0x50a7583 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7ec8u32); cpu.eax = v; }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x39050a81u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  if !cpu.sf { pc = 0x50a7611; continue; }
  pc = 0x50a7596; continue;
 }
 0x50a758d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa823905u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7b79u32); cpu.eax = v; }
  if !cpu.of { pc = 0x50a7616; continue; }
  pc = 0x50a759a; continue;
 }
 0x50a7596 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7c71u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  if (cpu.sf == cpu.of) { pc = 0x50a75a9; continue; }
  pc = 0x50a759f; continue;
 }
 0x50a759a => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a7d47u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8(cpu.r8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 5u32); cpu.w8(0xau32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7624; continue;
 }
 0x50a759f => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a7ac0u32); cpu.eax = v; }
  pc = 0x50a7624; continue;
 }
 0x50a75a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa827305u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a7b98u32); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x5u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])), cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.call(0x75c9); return; }
  pc = 0x50a75bf; continue;
 }
 0x50a75bf => {
  { let v: u32 = cpu.add32(cpu.eax, 0x50a7fcbu32); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a75d1; continue; }
  pc = 0x50a75c7; continue;
 }
 0x50a75c7 => {
  { let v: u32 = cpu.eax.wrapping_add(0x50a8070u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0xa827805u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a82ceu32); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x50a755d; continue; }
  pc = 0x50a75da; continue;
 }
 0x50a75d1 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a75d4; continue;
 }
 0x50a75d4 => {
  cpu.unimplemented(0x50a75d4, "Into");
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  if cpu.of { cpu.poll(); pc = 0x50a755d; continue; }
  pc = 0x50a75da; continue;
 }
 0x50a75da => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a829cu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x89ff050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | 0x8a9a050au32; cpu.w32(0x89ff050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a89ffu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.ecx == 0 { cpu.poll(); pc = 0x50a7578; continue; }
  pc = 0x50a75f6; continue;
 }
 0x50a75dd => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x89ff050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])) | 0x8a9a050au32; cpu.w32(0x89ff050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a89ffu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.ecx == 0 { cpu.poll(); pc = 0x50a7578; continue; }
  pc = 0x50a75f6; continue;
 }
 0x50a75f6 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a83b8u32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a83b8u32); cpu.eax = v; }
  { let v: u32 = cpu.port_in(0x8au16, 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a83b8u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x9c050a82u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7620; continue;
 }
 0x50a7611 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { cpu.cf = false; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x9c050a82u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7620; continue;
 }
 0x50a7616 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a82a1u32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7620; continue;
 }
 0x50a7620 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7624; continue;
 }
 0x50a7624 => {
  { cpu.cf = false; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50a829cu32); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50a830au32); cpu.eax = v; }
  cpu.unimplemented(0x50a7638, "Outsb");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a829cu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x8a6e050au32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a8b30u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x50a7650, "Outsb");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a831cu32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x50a75dd; continue; }
  pc = 0x50a765a; continue;
 }
 0x50a765a => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a829cu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x50a7660, "Daa");
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a89abu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x50a8bb9u32); cpu.eax = v; }
  { let v: u32 = cpu.port_in(0x8cu16, 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a8331u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), (cpu.ecx as u8)); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a889eu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.ror8(cpu.r8(0x8a5c050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 10u32); cpu.w8(0x8a5c050au32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a8c04u32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u32); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50a7620; continue; }
  pc = 0x50a769a; continue;
 }
 0x50a769a => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a829cu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x50a76a4, "Aad");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a829cu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a829cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let a = cpu.r8(0x829c050au32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); let b = (cpu.eax as u8); { let v: u8 = b; cpu.w8(0x829c050au32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a829cu32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a76d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a76d4); }
 let mut pc: u32 = 0x50a76d4;
 loop { match pc {
 0x50a76d4 => {
  {  }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x50a76ec); cpu.push(0x50a76f1); f_050a7768(cpu); if cpu.returned_to != 0x50a76f1 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x50a76f4); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7744); }
 let mut pc: u32 = 0x50a7744;
 loop { match pc {
 0x50a7744 => {
  {  }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1cu32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  cpu.flat_ds(0x50a775b); cpu.push(0x50a7760); f_050a7768(cpu); if cpu.returned_to != 0x50a7760 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.flat_ds(0x50a7766); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7768(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a7768); } r_050a7768(cpu, 0x50a7768); }
pub fn f_050a82a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a82a9); } r_050a7768(cpu, 0x50a82a9); }
pub fn f_050a83e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a83e8); } r_050a7768(cpu, 0x50a83e8); }
pub fn f_050a8469(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a8469); } r_050a7768(cpu, 0x50a8469); }
pub fn f_050a8684(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a8684); } r_050a7768(cpu, 0x50a8684); }
pub fn f_050a878e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a878e); } r_050a7768(cpu, 0x50a878e); }
pub fn f_050a892f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a892f); } r_050a7768(cpu, 0x50a892f); }
pub fn f_050a8b38(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a8b38); } r_050a7768(cpu, 0x50a8b38); }
fn r_050a7768(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a7768 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x50a77f4; continue; }
  pc = 0x50a7782; continue;
 }
 0x50a7782 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xf8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if !cpu.zf { pc = 0x50a789b; continue; }
  pc = 0x50a7790; continue;
 }
 0x50a7790 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xc0u8); }
  if !cpu.cf { pc = 0x50a7851; continue; }
  pc = 0x50a77a1; continue;
 }
 0x50a77a1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x1807u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x50a7414u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.flat_ds(0x50a77b2); cpu.push(0x50a77ba); cpu.call(t); if cpu.returned_to != 0x50a77ba { return; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x38u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x50a77c9); cpu.push(0x50a77ce); f_05096e08(cpu); if cpu.returned_to != 0x50a77ce { return; }
  { let v: u32 = 0x514bc24u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = cpu.r32(0x50a74d4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a77ee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50a7782; continue; }
  pc = 0x50a77f4; continue;
 }
 0x50a77f4 => {
  if cpu.zf { pc = 0x50a781a; continue; }
  pc = 0x50a77f6; continue;
 }
 0x50a77f6 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x50a7838; continue; }
  pc = 0x50a77fa; continue;
 }
 0x50a77fa => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x50a781d; continue; }
  pc = 0x50a77fe; continue;
 }
 0x50a77fe => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x50a782f; continue; }
  pc = 0x50a7802; continue;
 }
 0x50a7802 => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x50a7826; continue; }
  pc = 0x50a7806; continue;
 }
 0x50a7806 => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x50a7841; continue; }
  pc = 0x50a780a; continue;
 }
 0x50a780a => {
  { cpu.sub8((cpu.eax as u8), 0x65u8); }
  if cpu.zf { pc = 0x50a7849; continue; }
  pc = 0x50a780e; continue;
 }
 0x50a780e => {
  { cpu.sub8((cpu.eax as u8), 0x66u8); }
  if cpu.zf { pc = 0x50a781a; continue; }
  pc = 0x50a7812; continue;
 }
 0x50a7812 => {
  { cpu.sub8((cpu.eax as u8), 0x67u8); }
  if !cpu.zf { cpu.poll(); pc = 0x50a7782; continue; }
  pc = 0x50a781a; continue;
 }
 0x50a781a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a781d => {
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a7826 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a782f => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a7838 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a7841 => {
  { let v: u16 = (cpu.segments[4] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a7849 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a77ee; continue;
 }
 0x50a7851 => {
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x50a7854); cpu.push(0x50a7859); f_05096e08(cpu); if cpu.returned_to != 0x50a7859 { return; }
  { let v: u32 = 0x514bc24u32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x38u32; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) | (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub16((cpu.eax as u16), 0xf801u16); }
  if !cpu.zf { pc = 0x50a7884; continue; }
  pc = 0x50a7880; continue;
 }
 0x50a7880 => {
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a7884; continue;
 }
 0x50a7884 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = cpu.r32(0x50a75d4u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x50a789b => {
  cpu.flat_ds(0x50a789b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a790a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x50a7406u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7921; continue;
 }
 0x50a7921 => {
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a795c => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x50a7406u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7ac0 => {
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7ac5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7acb); cpu.push(0x50a7ad0); f_050a96b6(cpu); if cpu.returned_to != 0x50a7ad0 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7adb); cpu.push(0x50a7ae0); f_050a8e8c(cpu); if cpu.returned_to != 0x50a7ae0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7ae4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7aea); cpu.push(0x50a7aef); f_050a96b6(cpu); if cpu.returned_to != 0x50a7aef { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7afa); cpu.push(0x50a7aff); f_050a92c0(cpu); if cpu.returned_to != 0x50a7aff { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7b03 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b09); cpu.push(0x50a7b0e); f_050a96b6(cpu); if cpu.returned_to != 0x50a7b0e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b15); cpu.push(0x50a7b1a); f_050a9052(cpu); if cpu.returned_to != 0x50a7b1a { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a7b1f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b25); cpu.push(0x50a7b2a); f_050a96b6(cpu); if cpu.returned_to != 0x50a7b2a { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b31); cpu.push(0x50a7b36); f_050a9052(cpu); if cpu.returned_to != 0x50a7b36 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a7b3b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b41); cpu.push(0x50a7b46); f_050a96b6(cpu); if cpu.returned_to != 0x50a7b46 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7b51); cpu.push(0x50a7b56); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7b56 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7b5a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b60); cpu.push(0x50a7b65); f_050a96b6(cpu); if cpu.returned_to != 0x50a7b65 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7b70); cpu.push(0x50a7b75); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7b75 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7b79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b7f); cpu.push(0x50a7b84); f_050a96b6(cpu); if cpu.returned_to != 0x50a7b84 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7b8f); cpu.push(0x50a7b94); f_050a90eb(cpu); if cpu.returned_to != 0x50a7b94 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7b98 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7b9e); cpu.push(0x50a7ba3); f_050a96b6(cpu); if cpu.returned_to != 0x50a7ba3 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7bae); cpu.push(0x50a7bb3); f_050a90eb(cpu); if cpu.returned_to != 0x50a7bb3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7bb7 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a7be7); cpu.push(0x50a7bec); f_050a96b6(cpu); if cpu.returned_to != 0x50a7bec { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7bf1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7bf5); cpu.push(0x50a7bfa); f_050a9570(cpu); if cpu.returned_to != 0x50a7bfa { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7c02 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7c06); cpu.push(0x50a7c0b); f_050a9570(cpu); if cpu.returned_to != 0x50a7c0b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a7c13 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.edx as u16); cpu.set_segment(0, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7c31 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x300u16); }
  if !cpu.zf { pc = 0x50a7c50; continue; }
  pc = 0x50a7c44; continue;
 }
 0x50a7c44 => {
  { let v: u32 = 0x50a7768u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7c6c; continue;
 }
 0x50a7c50 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if !cpu.zf { pc = 0x50a7c62; continue; }
  pc = 0x50a7c56; continue;
 }
 0x50a7c56 => {
  { let v: u32 = 0x50a78d8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7c6c; continue;
 }
 0x50a7c62 => {
  { let v: u32 = 0x50a789cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a7c6c; continue;
 }
 0x50a7c6c => {
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7c71 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7c85 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7c93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7c99); cpu.push(0x50a7c9e); f_050a94aa(cpu); if cpu.returned_to != 0x50a7c9e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7ca9); cpu.push(0x50a7cae); f_050a8e8c(cpu); if cpu.returned_to != 0x50a7cae { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7cb2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7cb8); cpu.push(0x50a7cbd); f_050a94aa(cpu); if cpu.returned_to != 0x50a7cbd { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7cc8); cpu.push(0x50a7ccd); f_050a92c0(cpu); if cpu.returned_to != 0x50a7ccd { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7cd1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7cd7); cpu.push(0x50a7cdc); f_050a94aa(cpu); if cpu.returned_to != 0x50a7cdc { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7ce3); cpu.push(0x50a7ce8); f_050a9052(cpu); if cpu.returned_to != 0x50a7ce8 { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a7ced => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7cf3); cpu.push(0x50a7cf8); f_050a94aa(cpu); if cpu.returned_to != 0x50a7cf8 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7cff); cpu.push(0x50a7d04); f_050a9052(cpu); if cpu.returned_to != 0x50a7d04 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a7d09 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7d0f); cpu.push(0x50a7d14); f_050a94aa(cpu); if cpu.returned_to != 0x50a7d14 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7d1f); cpu.push(0x50a7d24); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7d24 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7d28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7d2e); cpu.push(0x50a7d33); f_050a94aa(cpu); if cpu.returned_to != 0x50a7d33 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7d3e); cpu.push(0x50a7d43); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7d43 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7d47 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7d4d); cpu.push(0x50a7d52); f_050a94aa(cpu); if cpu.returned_to != 0x50a7d52 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7d5d); cpu.push(0x50a7d62); f_050a90eb(cpu); if cpu.returned_to != 0x50a7d62 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7d66 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7d6c); cpu.push(0x50a7d71); f_050a94aa(cpu); if cpu.returned_to != 0x50a7d71 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7d7c); cpu.push(0x50a7d81); f_050a90eb(cpu); if cpu.returned_to != 0x50a7d81 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7d85 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a7db5); cpu.push(0x50a7dba); f_050a94aa(cpu); if cpu.returned_to != 0x50a7dba { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7dbf => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a7dd9); cpu.push(0x50a7dde); f_050a977b(cpu); if cpu.returned_to != 0x50a7dde { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7de2); cpu.push(0x50a7de7); f_050a943b(cpu); if cpu.returned_to != 0x50a7de7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7dfb => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a7e09); cpu.push(0x50a7e0e); f_050a977b(cpu); if cpu.returned_to != 0x50a7e0e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7e12); cpu.push(0x50a7e17); f_050a943b(cpu); if cpu.returned_to != 0x50a7e17 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a7e1f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e29); cpu.push(0x50a7e2e); f_050a94f0(cpu); if cpu.returned_to != 0x50a7e2e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e39); cpu.push(0x50a7e3e); f_050a8e8c(cpu); if cpu.returned_to != 0x50a7e3e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7e42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e4c); cpu.push(0x50a7e51); f_050a94f0(cpu); if cpu.returned_to != 0x50a7e51 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e5c); cpu.push(0x50a7e61); f_050a92c0(cpu); if cpu.returned_to != 0x50a7e61 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7e65 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e6f); cpu.push(0x50a7e74); f_050a94f0(cpu); if cpu.returned_to != 0x50a7e74 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7e7b); cpu.push(0x50a7e80); f_050a9052(cpu); if cpu.returned_to != 0x50a7e80 { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a7e85 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7e8f); cpu.push(0x50a7e94); f_050a94f0(cpu); if cpu.returned_to != 0x50a7e94 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a7e9b); cpu.push(0x50a7ea0); f_050a9052(cpu); if cpu.returned_to != 0x50a7ea0 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a7ea5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7eaf); cpu.push(0x50a7eb4); f_050a94f0(cpu); if cpu.returned_to != 0x50a7eb4 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7ebf); cpu.push(0x50a7ec4); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7ec4 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7ec8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7ed2); cpu.push(0x50a7ed7); f_050a94f0(cpu); if cpu.returned_to != 0x50a7ed7 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7ee2); cpu.push(0x50a7ee7); f_050a8e7e(cpu); if cpu.returned_to != 0x50a7ee7 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7eeb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7ef5); cpu.push(0x50a7efa); f_050a94f0(cpu); if cpu.returned_to != 0x50a7efa { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7f05); cpu.push(0x50a7f0a); f_050a90eb(cpu); if cpu.returned_to != 0x50a7f0a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7f0e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x50a7f18); cpu.push(0x50a7f1d); f_050a94f0(cpu); if cpu.returned_to != 0x50a7f1d { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7f28); cpu.push(0x50a7f2d); f_050a90eb(cpu); if cpu.returned_to != 0x50a7f2d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a7f31 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7f78 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a7fac); cpu.push(0x50a7fb1); f_050a94f0(cpu); if cpu.returned_to != 0x50a7fb1 { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7fb6 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7fba); cpu.push(0x50a7fbf); f_050a95ef(cpu); if cpu.returned_to != 0x50a7fbf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a7fcb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a7fe9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a7fed); cpu.push(0x50a7ff2); f_050a95ef(cpu); if cpu.returned_to != 0x50a7ff2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a7ffe => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.edx as u16); cpu.set_segment(0, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x3800u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 11u32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x50a802c; continue;
 }
 0x50a802c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x50a802c; continue; }
  pc = 0x50a803e; continue;
 }
 0x50a803e => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8045 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x50a8057; continue;
 }
 0x50a8057 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x50a8057; continue; }
  pc = 0x50a8069; continue;
 }
 0x50a8069 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x50a895b; continue;
 }
 0x50a8070 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a807e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8086); cpu.push(0x50a808b); f_050a94aa(cpu); if cpu.returned_to != 0x50a808b { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8096); cpu.push(0x50a809b); f_050a8e8c(cpu); if cpu.returned_to != 0x50a809b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a809f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a80a7); cpu.push(0x50a80ac); f_050a94aa(cpu); if cpu.returned_to != 0x50a80ac { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a80b7); cpu.push(0x50a80bc); f_050a92c0(cpu); if cpu.returned_to != 0x50a80bc { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a80c0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a80c8); cpu.push(0x50a80cd); f_050a94aa(cpu); if cpu.returned_to != 0x50a80cd { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a80d4); cpu.push(0x50a80d9); f_050a9052(cpu); if cpu.returned_to != 0x50a80d9 { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a80de => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a80e6); cpu.push(0x50a80eb); f_050a94aa(cpu); if cpu.returned_to != 0x50a80eb { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a80f2); cpu.push(0x50a80f7); f_050a9052(cpu); if cpu.returned_to != 0x50a80f7 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a80fc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8104); cpu.push(0x50a8109); f_050a94aa(cpu); if cpu.returned_to != 0x50a8109 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8114); cpu.push(0x50a8119); f_050a8e7e(cpu); if cpu.returned_to != 0x50a8119 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a811d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8125); cpu.push(0x50a812a); f_050a94aa(cpu); if cpu.returned_to != 0x50a812a { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8135); cpu.push(0x50a813a); f_050a8e7e(cpu); if cpu.returned_to != 0x50a813a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a813e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8146); cpu.push(0x50a814b); f_050a94aa(cpu); if cpu.returned_to != 0x50a814b { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8156); cpu.push(0x50a815b); f_050a90eb(cpu); if cpu.returned_to != 0x50a815b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a815f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8167); cpu.push(0x50a816c); f_050a94aa(cpu); if cpu.returned_to != 0x50a816c { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8177); cpu.push(0x50a817c); f_050a90eb(cpu); if cpu.returned_to != 0x50a817c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8180 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a81b2); cpu.push(0x50a81b7); f_050a94aa(cpu); if cpu.returned_to != 0x50a81b7 { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a81bc => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a81d6); cpu.push(0x50a81db); f_050a977b(cpu); if cpu.returned_to != 0x50a81db { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a81df); cpu.push(0x50a81e4); f_050a943b(cpu); if cpu.returned_to != 0x50a81e4 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x50a81ef; continue; }
  pc = 0x50a81eb; continue;
 }
 0x50a81eb => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50a81ef; continue;
 }
 0x50a81ef => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8204 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a8212); cpu.push(0x50a8217); f_050a977b(cpu); if cpu.returned_to != 0x50a8217 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a821b); cpu.push(0x50a8220); f_050a943b(cpu); if cpu.returned_to != 0x50a8220 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x50a822b; continue; }
  pc = 0x50a8227; continue;
 }
 0x50a8227 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50a822b; continue;
 }
 0x50a822b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a8234 => {
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8239 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8269); cpu.push(0x50a826e); f_050a94bb(cpu); if cpu.returned_to != 0x50a826e { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8273 => {
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a8278 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a8286); cpu.push(0x50a828b); f_050a977b(cpu); if cpu.returned_to != 0x50a828b { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a828f); cpu.push(0x50a8294); f_050a9440(cpu); if cpu.returned_to != 0x50a8294 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a829c => {
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a82a1 => {
  cpu.poll(); pc = cpu.r32(0x50a82a9u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a82a9 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a82c9; continue;
 }
 0x50a82c9 => {
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a82ce => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a82da); cpu.push(0x50a82df); f_050a8e8c(cpu); if cpu.returned_to != 0x50a82df { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a82e3 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a82ef); cpu.push(0x50a82f4); f_050a92c0(cpu); if cpu.returned_to != 0x50a82f4 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a82f8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8300); cpu.push(0x50a8305); f_050a9052(cpu); if cpu.returned_to != 0x50a8305 { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a830a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8312); cpu.push(0x50a8317); f_050a9052(cpu); if cpu.returned_to != 0x50a8317 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a831c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8328); cpu.push(0x50a832d); f_050a8e7e(cpu); if cpu.returned_to != 0x50a832d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8331 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a833d); cpu.push(0x50a8342); f_050a8e7e(cpu); if cpu.returned_to != 0x50a8342 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8346 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8352); cpu.push(0x50a8357); f_050a90eb(cpu); if cpu.returned_to != 0x50a8357 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a835b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8367); cpu.push(0x50a836c); f_050a90eb(cpu); if cpu.returned_to != 0x50a836c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8370 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a83b8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a83e0 => {
  cpu.poll(); pc = cpu.r32(0x50a83e8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a83e8 => {
  { let v: u8 = cpu.r8(0xa841505u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0xa841505u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a829cu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0xa845305u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a829cu32); cpu.eax = v; }
  pc = 0x50a8404; continue;
 }
 0x50a8404 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a8408; continue;
 }
 0x50a8408 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8415 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8422 => {
  { let v: u32 = cpu.r32(0x50a735cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x50a7360u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a7364u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x50a8449); cpu.push(0x50a844e); f_050a9052(cpu); if cpu.returned_to != 0x50a844e { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a8453 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a8457); cpu.push(0x50a845c); f_050a9c02(cpu); if cpu.returned_to != 0x50a845c { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8461 => {
  cpu.poll(); pc = cpu.r32(0x50a8469u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a8469 => {
  { let v: u32 = cpu.eax; cpu.w32(0xa84cc05u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a8511u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), cpu.ecx); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a859bu32); cpu.eax = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x50a8404; continue; }
  pc = 0x50a847f; continue;
 }
 0x50a847f => {
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50a8625u32.wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a8489; continue;
 }
 0x50a8489 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a84cc => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xcd1b8afeu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xd49a784bu32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8511 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x5c17f0bcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb8aa3b29u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8556 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xc900daa2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a859b => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xfbcff799u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x9a209a84u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffdu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a85e0 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xd1cf79acu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb17217f7u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffeu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8625 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a867c => {
  cpu.poll(); pc = cpu.r32(0x50a8684u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a8684 => {
  { cpu.movs(1, false); }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = (cpu.ecx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u8 = a; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.eax.wrapping_add(0x50a86b1u32); cpu.eax = v; }
  { cpu.cf = false; }
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = (cpu.ecx as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u8 = a; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; } }
  { let v: u32 = cpu.eax.wrapping_add(0x50a8d0du32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x50a8dbbu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a8765u32); cpu.eax = v; }
  pc = 0x50a86a4; continue;
 }
 0x50a86a4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a86a8); cpu.push(0x50a86ad); f_050aa228(cpu); if cpu.returned_to != 0x50a86ad { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a86b1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a86ef); cpu.push(0x50a86f4); f_050a9d09(cpu); if cpu.returned_to != 0x50a86f4 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a86f8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a86fc); cpu.push(0x50a8701); f_050aa1f2(cpu); if cpu.returned_to != 0x50a8701 { return; }
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8744 => {
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8765 => {
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8786 => {
  cpu.poll(); pc = cpu.r32(0x50a878eu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a878e => {
  { cpu.scas(1, 0); }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.eax.wrapping_add(0x50a87d8u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x50a882cu32); cpu.eax = v; }
  { let v: u32 = cpu.port_in(0x8du16, 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50a8dfdu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x8891050au32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x1c2f448du32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a87b6); cpu.push(0x50a87bb); f_050a9b21(cpu); if cpu.returned_to != 0x50a87bb { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x50a740au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a87ae => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a87b6); cpu.push(0x50a87bb); f_050a9b21(cpu); if cpu.returned_to != 0x50a87bb { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x50a740au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a87d8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8816); cpu.push(0x50a881b); f_050a9cf0(cpu); if cpu.returned_to != 0x50a881b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a881f => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a8823); cpu.push(0x50a8828); f_050aa3fc(cpu); if cpu.returned_to != 0x50a8828 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a882c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a883c); cpu.push(0x50a8841); f_050aa1db(cpu); if cpu.returned_to != 0x50a8841 { return; }
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a887a); cpu.push(0x50a887f); f_050aa1e5(cpu); if cpu.returned_to != 0x50a887f { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8884 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a8888); cpu.push(0x50a888d); f_050aa1db(cpu); if cpu.returned_to != 0x50a888d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8891 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x50a8895); cpu.push(0x50a889a); f_050aa1e5(cpu); if cpu.returned_to != 0x50a889a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a889e => {
  cpu.poll(); pc = cpu.r32(0x50a88a6u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a88c6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a891d); cpu.push(0x50a8922); f_050a9052(cpu); if cpu.returned_to != 0x50a8922 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a8927 => {
  cpu.poll(); pc = cpu.r32(0x50a892fu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a892f => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a895bu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a894f; continue;
 }
 0x50a894f => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a895b => {
  { let v: u16 = 0x33fu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50a7768u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8981 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a898d); cpu.push(0x50a8992); f_050a8e8c(cpu); if cpu.returned_to != 0x50a8992 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8996 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a89a2); cpu.push(0x50a89a7); f_050a92c0(cpu); if cpu.returned_to != 0x50a89a7 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a89ab => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a89b7); cpu.push(0x50a89bc); f_050a8e7e(cpu); if cpu.returned_to != 0x50a89bc { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a89c0 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a89cc); cpu.push(0x50a89d1); f_050a8e7e(cpu); if cpu.returned_to != 0x50a89d1 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a89d5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a89e1); cpu.push(0x50a89e6); f_050a90eb(cpu); if cpu.returned_to != 0x50a89e6 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a89ea => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a89f6); cpu.push(0x50a89fb); f_050a90eb(cpu); if cpu.returned_to != 0x50a89fb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a89ff => {
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8a1e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8a4a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8a52); cpu.push(0x50a8a57); f_050a9052(cpu); if cpu.returned_to != 0x50a8a57 { return; }
  cpu.poll(); pc = 0x50a795c; continue;
 }
 0x50a8a5c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8a64); cpu.push(0x50a8a69); f_050a9052(cpu); if cpu.returned_to != 0x50a8a69 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a8a6e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7921; continue;
 }
 0x50a8a9a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8aa6); cpu.push(0x50a8aab); f_050a8e8c(cpu); if cpu.returned_to != 0x50a8aab { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8ae5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8af1); cpu.push(0x50a8af6); f_050a92c0(cpu); if cpu.returned_to != 0x50a8af6 { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8b30 => {
  cpu.poll(); pc = cpu.r32(0x50a8b38u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a8b38 => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x50a829cu32); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x5u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])) | 0x5u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50a8b58; continue;
 }
 0x50a8b58 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8baf); cpu.push(0x50a8bb4); f_050a9052(cpu); if cpu.returned_to != 0x50a8bb4 { return; }
  cpu.poll(); pc = 0x50a790a; continue;
 }
 0x50a8bb9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8bc5); cpu.push(0x50a8bca); f_050a8e7e(cpu); if cpu.returned_to != 0x50a8bca { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8c04 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8c10); cpu.push(0x50a8c15); f_050a8e7e(cpu); if cpu.returned_to != 0x50a8c15 { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8c4f => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8c5b); cpu.push(0x50a8c60); f_050a90eb(cpu); if cpu.returned_to != 0x50a8c60 { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8c9a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x50a8ca6); cpu.push(0x50a8cab); f_050a90eb(cpu); if cpu.returned_to != 0x50a8cab { return; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8ce5 => {
  cpu.poll(); pc = cpu.r32(0x50a8cedu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50a8d0d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x50a8d4d); cpu.push(0x50a8d52); f_050a9908(cpu); if cpu.returned_to != 0x50a8d52 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8d56 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x50a7368u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x50a736au32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x50a73b6u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x8000u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) | 0x3fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x7fffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.flat_ds(0x50a8db1); cpu.push(0x50a8db6); f_050a943b(cpu); if cpu.returned_to != 0x50a8db6 { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8dbb => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x50a8dc3); cpu.push(0x50a8dc8); f_050a9b21(cpu); if cpu.returned_to != 0x50a8dc8 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x50a740au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x50a8de5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a8df3); cpu.push(0x50a8df8); f_050a977b(cpu); if cpu.returned_to != 0x50a8df8 { return; }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8dfd => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x50a7366u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x50a8e0f); cpu.push(0x50a8e14); f_050a943b(cpu); if cpu.returned_to != 0x50a8e14 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x50a8e3c; continue; }
  pc = 0x50a8e21; continue;
 }
 0x50a8e21 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x50a8e2d; continue; }
  pc = 0x50a8e25; continue;
 }
 0x50a8e25 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a8e3a; continue;
 }
 0x50a8e2d => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a8e3a; continue;
 }
 0x50a8e3a => {
  pc = 0x50a8e62; continue;
 }
 0x50a8e3c => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x50a8e62; continue; }
  pc = 0x50a8e52; continue;
 }
 0x50a8e52 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x50a8e5e; continue; }
  pc = 0x50a8e56; continue;
 }
 0x50a8e56 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a8e62; continue;
 }
 0x50a8e5e => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x50a8e62; continue;
 }
 0x50a8e62 => {
  { let v: u16 = (cpu.edx as u16) | (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 0x50a8e6e => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x50a7768; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a789c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a789c); }
 let mut pc: u32 = 0x50a789c;
 loop { match pc {
 0x50a789c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 25u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x50a78ca; continue; }
  pc = 0x50a78ad; continue;
 }
 0x50a78ad => {
  if !cpu.zf { pc = 0x50a78b6; continue; }
  pc = 0x50a78af; continue;
 }
 0x50a78af => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x50a78b6; continue; }
  pc = 0x50a78b4; continue;
 }
 0x50a78b4 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x50a78b6; continue;
 }
 0x50a78b6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x100u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x50a78ca; continue; }
  pc = 0x50a78bf; continue;
 }
 0x50a78bf => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a78ca; continue;
 }
 0x50a78ca => {
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_050a7768(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a78d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a78d8); }
 let mut pc: u32 = 0x50a78d8;
 loop { match pc {
 0x50a78d8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xfffff800u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 22u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x50a7903; continue; }
  pc = 0x50a78e8; continue;
 }
 0x50a78e8 => {
  if !cpu.zf { pc = 0x50a78ec; continue; }
  pc = 0x50a78ea; continue;
 }
 0x50a78ea => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x50a78ec; continue;
 }
 0x50a78ec => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x800u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x50a7903; continue; }
  pc = 0x50a78f8; continue;
 }
 0x50a78f8 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a7903; continue;
 }
 0x50a7903 => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_050a7768(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7978(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7978); }
 let mut pc: u32 = 0x50a7978;
 loop { match pc {
 0x50a7978 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x50a79c5; continue; }
  pc = 0x50a7982; continue;
 }
 0x50a7982 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { cpu.df = false; }
  { cpu.movs(4, true); }
  { let v: u32 = ((cpu.eax >> 8) as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x514ad21u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x50a799f); cpu.push(0x50a79a5); cpu.call(t); if cpu.returned_to != 0x50a79a5 { return; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1bu32; cpu.ecx = v; }
  { cpu.df = false; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x50a79c5; continue;
 }
 0x50a79c5 => {
  cpu.flat_ds(0x50a79c5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79c6); }
 let mut pc: u32 = 0x50a79c6;
 loop { match pc {
 0x50a79c6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79cc); }
 let mut pc: u32 = 0x50a79cc;
 loop { match pc {
 0x50a79cc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79d2); }
 let mut pc: u32 = 0x50a79d2;
 loop { match pc {
 0x50a79d2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79d8); }
 let mut pc: u32 = 0x50a79d8;
 loop { match pc {
 0x50a79d8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79de); }
 let mut pc: u32 = 0x50a79de;
 loop { match pc {
 0x50a79de => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79e6); }
 let mut pc: u32 = 0x50a79e6;
 loop { match pc {
 0x50a79e6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79ec); }
 let mut pc: u32 = 0x50a79ec;
 loop { match pc {
 0x50a79ec => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79f2); }
 let mut pc: u32 = 0x50a79f2;
 loop { match pc {
 0x50a79f2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a79f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a79f8); }
 let mut pc: u32 = 0x50a79f8;
 loop { match pc {
 0x50a79f8 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a00); }
 let mut pc: u32 = 0x50a7a00;
 loop { match pc {
 0x50a7a00 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a08); }
 let mut pc: u32 = 0x50a7a08;
 loop { match pc {
 0x50a7a08 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a10); }
 let mut pc: u32 = 0x50a7a10;
 loop { match pc {
 0x50a7a10 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a18); }
 let mut pc: u32 = 0x50a7a18;
 loop { match pc {
 0x50a7a18 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a22); }
 let mut pc: u32 = 0x50a7a22;
 loop { match pc {
 0x50a7a22 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a2c); }
 let mut pc: u32 = 0x50a7a2c;
 loop { match pc {
 0x50a7a2c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a34); }
 let mut pc: u32 = 0x50a7a34;
 loop { match pc {
 0x50a7a34 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a3c); }
 let mut pc: u32 = 0x50a7a3c;
 loop { match pc {
 0x50a7a3c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a45); }
 let mut pc: u32 = 0x50a7a45;
 loop { match pc {
 0x50a7a45 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a4e); }
 let mut pc: u32 = 0x50a7a4e;
 loop { match pc {
 0x50a7a4e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a57); }
 let mut pc: u32 = 0x50a7a57;
 loop { match pc {
 0x50a7a57 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a60); }
 let mut pc: u32 = 0x50a7a60;
 loop { match pc {
 0x50a7a60 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a6b); }
 let mut pc: u32 = 0x50a7a6b;
 loop { match pc {
 0x50a7a6b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a76); }
 let mut pc: u32 = 0x50a7a76;
 loop { match pc {
 0x50a7a76 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a7f); }
 let mut pc: u32 = 0x50a7a7f;
 loop { match pc {
 0x50a7a7f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a7a88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a7a88); }
 let mut pc: u32 = 0x50a7a88;
 loop { match pc {
 0x50a7a88 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x18u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x50a7474u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x50a7aa3); cpu.call(t); if cpu.returned_to != 0x50a7aa3 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x38u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x50a7abe; continue; }
  pc = 0x50a7aaf; continue;
 }
 0x50a7aaf => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  pc = 0x50a7abe; continue;
 }
 0x50a7abe => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a8e7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a8e7e); } r_050a8e7e(cpu, 0x50a8e7e); }
pub fn f_050a8e8c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a8e8c); } r_050a8e7e(cpu, 0x50a8e8c); }
fn r_050a8e7e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a8e7e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a8e93; continue;
 }
 0x50a8e8c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a8e93; continue;
 }
 0x50a8e93 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50a8ea9); f_050a8ee6(cpu); if cpu.returned_to != 0x50a8ea9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a8eb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a8eb6); }
 let mut pc: u32 = 0x50a8eb6;
 loop { match pc {
 0x50a8eb6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50a8ed7); f_050a8ee6(cpu); if cpu.returned_to != 0x50a8ed7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a8ee6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a8ee6); }
 let mut pc: u32 = 0x50a8ee6;
 loop { match pc {
 0x50a8ee6 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a8f09; continue; }
  pc = 0x50a8eea; continue;
 }
 0x50a8eea => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a8f09; continue; }
  pc = 0x50a8eee; continue;
 }
 0x50a8eee => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a8f06; continue; }
  pc = 0x50a8ef3; continue;
 }
 0x50a8ef3 => {
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16) | (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x50a8f05; continue; }
  pc = 0x50a8f03; continue;
 }
 0x50a8f03 => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x50a8f05; continue;
 }
 0x50a8f05 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a8f06 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a8f09; continue;
 }
 0x50a8f09 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50a8f1a; continue; }
  pc = 0x50a8f0d; continue;
 }
 0x50a8f0d => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x50a8f1a; continue; }
  pc = 0x50a8f11; continue;
 }
 0x50a8f11 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x50a8f1a; continue; }
  pc = 0x50a8f19; continue;
 }
 0x50a8f19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a8f1a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let a = cpu.esi; let b = cpu.ecx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi & 0x80007fffu32; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0x80007fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add((cpu.edi as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), (cpu.edi as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a8f69; continue; }
  pc = 0x50a8f4b; continue;
 }
 0x50a8f4b => {
  if !cpu.cf { pc = 0x50a8f55; continue; }
  pc = 0x50a8f4d; continue;
 }
 0x50a8f4d => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.esi; let b = cpu.edx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x50a8f55; continue;
 }
 0x50a8f55 => {
  { cpu.sub16((cpu.ecx as u16), 0x40u16); }
  if (cpu.cf || cpu.zf) { pc = 0x50a8f69; continue; }
  pc = 0x50a8f5b; continue;
 }
 0x50a8f5b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a8f69 => {
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x50a8f7e; continue; }
  pc = 0x50a8f6f; continue;
 }
 0x50a8f6f => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x50a8f7e; continue;
 }
 0x50a8f7e => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x50a8fb1; continue; }
  pc = 0x50a8f85; continue;
 }
 0x50a8f85 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x50a8fa3; continue; }
  pc = 0x50a8f8d; continue;
 }
 0x50a8f8d => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x50a8f9f; continue; }
  pc = 0x50a8f9b; continue;
 }
 0x50a8f9b => {
  { let v: u32 = cpu.edi | cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x50a8f9f; continue;
 }
 0x50a8f9f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x50a8fa3; continue;
 }
 0x50a8fa3 => {
  { let v: u32 = cpu.shrd32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.edi | cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x50a8fb1; continue;
 }
 0x50a8fb1 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x50a8fe2; continue; }
  pc = 0x50a8fba; continue;
 }
 0x50a8fba => {
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x50a8fd3; continue; }
  pc = 0x50a8fbf; continue;
 }
 0x50a8fbf => {
  { cpu.and32(cpu.edi, 0x7fffffffu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50a8fd3; continue;
 }
 0x50a8fd3 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x50a8fe2; continue;
 }
 0x50a8fe2 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x50a9039; continue; }
  pc = 0x50a8fea; continue;
 }
 0x50a8fea => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a903b; continue; }
  pc = 0x50a8fef; continue;
 }
 0x50a8fef => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x50a9002; continue; }
  pc = 0x50a8ff4; continue;
 }
 0x50a8ff4 => {
  { let v: u32 = cpu.rol32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.ror32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x50a8ff8; continue;
 }
 0x50a8ff8 => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a903b; continue; }
  pc = 0x50a8ffc; continue;
 }
 0x50a8ffc => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x50a8ff8; continue; }
  pc = 0x50a9002; continue;
 }
 0x50a9002 => {
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x50a9045; continue; }
  pc = 0x50a900f; continue;
 }
 0x50a900f => {
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x50a9037; continue; }
  pc = 0x50a9016; continue;
 }
 0x50a9016 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x50a901e; continue; }
  pc = 0x50a901a; continue;
 }
 0x50a901a => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a901e; continue;
 }
 0x50a901e => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x50a9037; continue; }
  pc = 0x50a9026; continue;
 }
 0x50a9026 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x50a9045; continue; }
  pc = 0x50a9037; continue;
 }
 0x50a9037 => {
  pc = 0x50a903b; continue;
 }
 0x50a9039 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x50a903b; continue;
 }
 0x50a903b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9045 => {
  { let v: u16 = 0x7fffu16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x50a903b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9052(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9052); }
 let mut pc: u32 = 0x50a9052;
 loop { match pc {
 0x50a9052 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a906a; continue; }
  pc = 0x50a9061; continue;
 }
 0x50a9061 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  if !cpu.zf { pc = 0x50a90bd; continue; }
  pc = 0x50a906a; continue;
 }
 0x50a906a => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a9080; continue; }
  pc = 0x50a9077; continue;
 }
 0x50a9077 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.esi = v; }
  if !cpu.zf { pc = 0x50a90bd; continue; }
  pc = 0x50a9080; continue;
 }
 0x50a9080 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.sf { pc = 0x50a90c5; continue; }
  pc = 0x50a908f; continue;
 }
 0x50a908f => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x8u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x50a90a5; continue; }
  pc = 0x50a9099; continue;
 }
 0x50a9099 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x50a90a5; continue; }
  pc = 0x50a90a1; continue;
 }
 0x50a90a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  pc = 0x50a90a5; continue;
 }
 0x50a90a5 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.zf { pc = 0x50a90b8; continue; }
  pc = 0x50a90ac; continue;
 }
 0x50a90ac => {
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.eax; cpu.esi = v; }
  pc = 0x50a90b0; continue;
 }
 0x50a90b0 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, 0x0u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x50a90b8; continue;
 }
 0x50a90b8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a90bd => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a90c5 => {
  { let v: u32 = cpu.edi | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi | cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi | cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0x7fff7fffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a90b0; continue; }
  pc = 0x50a90e8; continue;
 }
 0x50a90e8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a90eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a90eb); }
 let mut pc: u32 = 0x50a90eb;
 loop { match pc {
 0x50a90eb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50a9108); f_050a9115(cpu); if cpu.returned_to != 0x50a9108 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9115); }
 let mut pc: u32 = 0x50a9115;
 loop { match pc {
 0x50a9115 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50a9160; continue; }
  pc = 0x50a9119; continue;
 }
 0x50a9119 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x50a9160; continue; }
  pc = 0x50a911d; continue;
 }
 0x50a911d => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x50a9160; continue; }
  pc = 0x50a9125; continue;
 }
 0x50a9125 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a914a; continue; }
  pc = 0x50a9129; continue;
 }
 0x50a9129 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a914a; continue; }
  pc = 0x50a912d; continue;
 }
 0x50a912d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a914a; continue; }
  pc = 0x50a9134; continue;
 }
 0x50a9134 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x50a913d); f_050a7978(cpu); if cpu.returned_to != 0x50a913d { return; }
  { let v: u32 = 0xc0000000u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a915f; continue;
 }
 0x50a914a => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x83u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x50a9153); f_050a7978(cpu); if cpu.returned_to != 0x50a9153 { return; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a915f; continue;
 }
 0x50a915f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9160 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a9171; continue; }
  pc = 0x50a9164; continue;
 }
 0x50a9164 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a9171; continue; }
  pc = 0x50a9168; continue;
 }
 0x50a9168 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a916e; continue; }
  pc = 0x50a916d; continue;
 }
 0x50a916d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a916e => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a9171; continue;
 }
 0x50a9171 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let a = cpu.esi; let b = cpu.ecx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi & 0x80007fffu32; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0x80007fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edi as u16).wrapping_add((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.edi as u16), (cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a91ac; continue; }
  pc = 0x50a91a2; continue;
 }
 0x50a91a2 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x50a91a2; continue; }
  pc = 0x50a91ac; continue;
 }
 0x50a91ac => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a91bb; continue; }
  pc = 0x50a91b1; continue;
 }
 0x50a91b1 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.sf { cpu.poll(); pc = 0x50a91b1; continue; }
  pc = 0x50a91bb; continue;
 }
 0x50a91bb => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x3fffu16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x50a91de; continue; }
  pc = 0x50a91c5; continue;
 }
 0x50a91c5 => {
  { cpu.sub16((cpu.edi as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x50a91de; continue; }
  pc = 0x50a91cc; continue;
 }
 0x50a91cc => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u16 = 0x7fffu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x50a92b6; continue;
 }
 0x50a91de => {
  { cpu.sub16((cpu.edi as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x50a91ef; continue; }
  pc = 0x50a91e4; continue;
 }
 0x50a91e4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x50a92b6; continue;
 }
 0x50a91ef => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50a9201; continue; }
  pc = 0x50a91fe; continue;
 }
 0x50a91fe => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50a9201; continue;
 }
 0x50a9201 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x1u8); }
  if cpu.zf { pc = 0x50a9222; continue; }
  pc = 0x50a921d; continue;
 }
 0x50a921d => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  pc = 0x50a9222; continue;
 }
 0x50a9222 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edx); cpu.edi = v; }
  if cpu.zf { pc = 0x50a923c; continue; }
  pc = 0x50a922a; continue;
 }
 0x50a922a => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a922a; continue; }
  pc = 0x50a923c; continue;
 }
 0x50a923c => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50a9251; continue; }
  pc = 0x50a9247; continue;
 }
 0x50a9247 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50a9251; continue;
 }
 0x50a9251 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50a9288; continue; }
  pc = 0x50a925c; continue;
 }
 0x50a925c => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edx); cpu.edi = v; }
  if cpu.zf { pc = 0x50a9288; continue; }
  pc = 0x50a9271; continue;
 }
 0x50a9271 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a9271; continue; }
  pc = 0x50a9288; continue;
 }
 0x50a9288 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x50a929a; continue; }
  pc = 0x50a9295; continue;
 }
 0x50a9295 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x50a929a; continue;
 }
 0x50a929a => {
  { let v: u16 = cpu.or16((cpu.edi as u16), (cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a92b6; continue; }
  pc = 0x50a929f; continue;
 }
 0x50a929f => {
  if !cpu.zf { pc = 0x50a92a5; continue; }
  pc = 0x50a92a1; continue;
 }
 0x50a92a1 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50a92ab; continue;
 }
 0x50a92a5 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a92ab; continue;
 }
 0x50a92ab => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x50a92b6; continue;
 }
 0x50a92b6 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u16 = cpu.rcr16((cpu.edi as u16), 1u32); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a92c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a92c0); }
 let mut pc: u32 = 0x50a92c0;
 loop { match pc {
 0x50a92c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50a92dd); f_050a92ea(cpu); if cpu.returned_to != 0x50a92dd { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a92ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a92ea); }
 let mut pc: u32 = 0x50a92ea;
 loop { match pc {
 0x50a92ea => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a92fb; continue; }
  pc = 0x50a92ee; continue;
 }
 0x50a92ee => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a92fb; continue; }
  pc = 0x50a92f2; continue;
 }
 0x50a92f2 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a92f8; continue; }
  pc = 0x50a92f7; continue;
 }
 0x50a92f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a92f8 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50a92fb; continue;
 }
 0x50a92fb => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50a9312; continue; }
  pc = 0x50a92ff; continue;
 }
 0x50a92ff => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x50a9312; continue; }
  pc = 0x50a9303; continue;
 }
 0x50a9303 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x50a9312; continue; }
  pc = 0x50a930b; continue;
 }
 0x50a930b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9312 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let a = cpu.esi; let b = cpu.ecx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.ecx = v; } }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.rol32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edi & 0x80007fffu32; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0x80007fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x50a9342; continue; }
  pc = 0x50a9336; continue;
 }
 0x50a9336 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x50a9342; continue; }
  pc = 0x50a933d; continue;
 }
 0x50a933d => {
  pc = 0x50a93d9; continue;
 }
 0x50a9342 => {
  { cpu.sub16((cpu.ecx as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x50a9353; continue; }
  pc = 0x50a9348; continue;
 }
 0x50a9348 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50a93cf; continue;
 }
 0x50a9353 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.edx; cpu.mul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.sf { pc = 0x50a938a; continue; }
  pc = 0x50a9382; continue;
 }
 0x50a9382 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a938a; continue;
 }
 0x50a938a => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.cf { pc = 0x50a93b6; continue; }
  pc = 0x50a938e; continue;
 }
 0x50a938e => {
  if !cpu.zf { pc = 0x50a939d; continue; }
  pc = 0x50a9390; continue;
 }
 0x50a9390 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x50a939d; continue; }
  pc = 0x50a9399; continue;
 }
 0x50a9399 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x50a939d; continue;
 }
 0x50a939d => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x50a93b6; continue; }
  pc = 0x50a93a5; continue;
 }
 0x50a93a5 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x50a93d9; continue; }
  pc = 0x50a93b6; continue;
 }
 0x50a93b6 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a93cf; continue; }
  pc = 0x50a93bb; continue;
 }
 0x50a93bb => {
  if !cpu.zf { pc = 0x50a93c1; continue; }
  pc = 0x50a93bd; continue;
 }
 0x50a93bd => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50a93c4; continue;
 }
 0x50a93c1 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a93c4; continue;
 }
 0x50a93c4 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a93cf; continue;
 }
 0x50a93cf => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a93d9 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50a93cf; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a93e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a93e6); } r_050a93e6(cpu, 0x50a93e6); }
pub fn f_050a943b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a943b); } r_050a93e6(cpu, 0x50a943b); }
pub fn f_050a9440(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a9440); } r_050a93e6(cpu, 0x50a9440); }
fn r_050a93e6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a93e6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0xa0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x50a9443; continue;
 }
 0x50a943b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x50a9443; continue;
 }
 0x50a9440 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x50a9443; continue;
 }
 0x50a9443 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50a94a5; continue; }
  pc = 0x50a945b; continue;
 }
 0x50a945b => {
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a949d; continue; }
  pc = 0x50a9461; continue;
 }
 0x50a9461 => {
  { let v: u8 = (cpu.ebx as u8) & 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), (cpu.ebx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50a949d; continue; }
  pc = 0x50a9468; continue;
 }
 0x50a9468 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.rcr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x50a947c; continue; }
  pc = 0x50a9478; continue;
 }
 0x50a9478 => {
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50a9489; continue;
 }
 0x50a947c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shld32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x50a9489; continue;
 }
 0x50a9489 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x50a949a; continue; }
  pc = 0x50a9498; continue;
 }
 0x50a9498 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50a949a; continue;
 }
 0x50a949a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a949d => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a94a5 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a94aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a94aa); } r_050a94aa(cpu, 0x50a94aa); }
pub fn f_050a94bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a94bb); } r_050a94aa(cpu, 0x50a94bb); }
fn r_050a94aa(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a94aa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x50a94ba; continue; }
  pc = 0x50a94b1; continue;
 }
 0x50a94b1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xbfffu32; cpu.edx = v; }
  pc = 0x50a94c3; continue;
 }
 0x50a94ba => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x50a94bb; continue;
 }
 0x50a94bb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3fffu32; cpu.edx = v; }
  pc = 0x50a94c3; continue;
 }
 0x50a94c3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50a94de; continue; }
  pc = 0x50a94c8; continue;
 }
 0x50a94c8 => {
  cpu.unimplemented(0x50a94c8, "Bsr");
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x50a94e2; continue;
 }
 0x50a94de => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50a94e2; continue;
 }
 0x50a94e2 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a94f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a94f0); }
 let mut pc: u32 = 0x50a94f0;
 loop { match pc {
 0x50a94f0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 11u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 20u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7ffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a953b; continue; }
  pc = 0x50a9504; continue;
 }
 0x50a9504 => {
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if cpu.zf { pc = 0x50a9512; continue; }
  pc = 0x50a950b; continue;
 }
 0x50a950b => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3c00u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a9533; continue;
 }
 0x50a9512 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50a9520; continue; }
  pc = 0x50a951e; continue;
 }
 0x50a951e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50a9520; continue;
 }
 0x50a9520 => {
  if cpu.zf { pc = 0x50a9533; continue; }
  pc = 0x50a9522; continue;
 }
 0x50a9522 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x50a952c); f_050a7978(cpu); if cpu.returned_to != 0x50a952c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx | 0x40000000u32; cpu.edx = v; }
  pc = 0x50a9533; continue;
 }
 0x50a9533 => {
  { let v: u32 = cpu.edx | 0x80000000u32; cpu.edx = v; }
  pc = 0x50a9560; continue;
 }
 0x50a953b => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a9547; continue; }
  pc = 0x50a953f; continue;
 }
 0x50a953f => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a9547; continue; }
  pc = 0x50a9543; continue;
 }
 0x50a9543 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50a9560; continue;
 }
 0x50a9547 => {
  { let v: u16 = 0x3c01u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a9554; continue; }
  pc = 0x50a954f; continue;
 }
 0x50a954f => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a9554; continue;
 }
 0x50a9554 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x50a9560; continue; }
  pc = 0x50a9558; continue;
 }
 0x50a9558 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x50a9554; continue;
 }
 0x50a9560 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9570); }
 let mut pc: u32 = 0x50a9570;
 loop { match pc {
 0x50a9570 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 25u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x50a959f; continue; }
  pc = 0x50a9588; continue;
 }
 0x50a9588 => {
  if !cpu.zf { pc = 0x50a9591; continue; }
  pc = 0x50a958a; continue;
 }
 0x50a958a => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9591; continue; }
  pc = 0x50a958f; continue;
 }
 0x50a958f => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x50a9591; continue;
 }
 0x50a9591 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x50a959f; continue; }
  pc = 0x50a9598; continue;
 }
 0x50a9598 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a959f; continue;
 }
 0x50a959f => {
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a95eb; continue; }
  pc = 0x50a95aa; continue;
 }
 0x50a95aa => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x50a95c2; continue; }
  pc = 0x50a95b1; continue;
 }
 0x50a95b1 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a95eb; continue;
 }
 0x50a95c2 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0xc080u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x50a95cd; continue; }
  pc = 0x50a95c9; continue;
 }
 0x50a95c9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50a95eb; continue;
 }
 0x50a95cd => {
  { cpu.sub16((cpu.ecx as u16), 0xffu16); }
  if (cpu.sf != cpu.of) { pc = 0x50a95e0; continue; }
  pc = 0x50a95d4; continue;
 }
 0x50a95d4 => {
  { let v: u32 = 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a95eb; continue;
 }
 0x50a95e0 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.ecx, 8u32); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a95eb; continue;
 }
 0x50a95eb => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a95ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a95ef); }
 let mut pc: u32 = 0x50a95ef;
 loop { match pc {
 0x50a95ef => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffff800u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 22u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x50a961c; continue; }
  pc = 0x50a9607; continue;
 }
 0x50a9607 => {
  if !cpu.zf { pc = 0x50a960b; continue; }
  pc = 0x50a9609; continue;
 }
 0x50a9609 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  pc = 0x50a960b; continue;
 }
 0x50a960b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x50a961c; continue; }
  pc = 0x50a9615; continue;
 }
 0x50a9615 => {
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a961c; continue;
 }
 0x50a961c => {
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xc400u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if !cpu.cf { pc = 0x50a9652; continue; }
  pc = 0x50a9631; continue;
 }
 0x50a9631 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a9641; continue; }
  pc = 0x50a9636; continue;
 }
 0x50a9636 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 12u32); cpu.edx = v; }
  pc = 0x50a964b; continue;
 }
 0x50a9641 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ecx, 11u32); cpu.edx = v; }
  pc = 0x50a964b; continue;
 }
 0x50a964b => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x50a96b2; continue;
 }
 0x50a9652 => {
  { cpu.sub16((cpu.ecx as u16), 0xc400u16); }
  if cpu.cf { pc = 0x50a968c; continue; }
  pc = 0x50a9659; continue;
 }
 0x50a9659 => {
  { cpu.sub16((cpu.ecx as u16), 0xffccu16); }
  if (cpu.sf != cpu.of) { pc = 0x50a9686; continue; }
  pc = 0x50a965f; continue;
 }
 0x50a965f => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x50a9674; continue; }
  pc = 0x50a966b; continue;
 }
 0x50a966b => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x50a9674; continue;
 }
 0x50a9674 => {
  { let v: u32 = cpu.shrd32(cpu.esi, cpu.eax, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x50a968a; continue;
 }
 0x50a9686 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x50a968a; continue;
 }
 0x50a968a => {
  pc = 0x50a96b2; continue;
 }
 0x50a968c => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 11u32); cpu.edx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x7ff00000u32; cpu.edx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x43ffu16); }
  if cpu.zf { pc = 0x50a96b2; continue; }
  pc = 0x50a96a7; continue;
 }
 0x50a96a7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x84u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x50a96b1); f_050a7978(cpu); if cpu.returned_to != 0x50a96b1 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x50a96b2; continue;
 }
 0x50a96b2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a96b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a96b6); }
 let mut pc: u32 = 0x50a96b6;
 loop { match pc {
 0x50a96b6 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 23u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0xffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50a96ef; continue; }
  pc = 0x50a96c8; continue;
 }
 0x50a96c8 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x50a96d7; continue; }
  pc = 0x50a96d0; continue;
 }
 0x50a96d0 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3f80u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50a96e5; continue;
 }
 0x50a96d7 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x50a96e5; continue; }
  pc = 0x50a96e0; continue;
 }
 0x50a96e0 => {
  { let v: u32 = cpu.eax | 0x40000000u32; cpu.eax = v; }
  pc = 0x50a96e5; continue;
 }
 0x50a96e5 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, 0x80000000u32); cpu.eax = v; }
  pc = 0x50a96ef; continue;
 }
 0x50a96ef => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a977b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a977b); }
 let mut pc: u32 = 0x50a977b;
 loop { match pc {
 0x50a977b => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x50a9790; continue; }
  pc = 0x50a978b; continue;
 }
 0x50a978b => {
  { cpu.sub16((cpu.edx as u16), 0x800u16); }
  pc = 0x50a9790; continue;
 }
 0x50a9790 => {
  if !cpu.zf { pc = 0x50a979b; continue; }
  pc = 0x50a9792; continue;
 }
 0x50a9792 => {
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x50a979b; continue; }
  pc = 0x50a9796; continue;
 }
 0x50a9796 => {
  { let v: u16 = (cpu.edx as u16) ^ 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x50a979b; continue;
 }
 0x50a979b => {
  { cpu.sub16((cpu.edx as u16), 0xc00u16); }
  if !cpu.zf { pc = 0x50a97a6; continue; }
  pc = 0x50a97a2; continue;
 }
 0x50a97a2 => {
  { let v: u16 = 0x400u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x50a97a6; continue;
 }
 0x50a97a6 => {
  { let v: u16 = (cpu.ebx as u16) & 0x7fffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x403fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50a97b4; continue; }
  pc = 0x50a97b2; continue;
 }
 0x50a97b2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a97b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50a980a; continue; }
  pc = 0x50a97bc; continue;
 }
 0x50a97bc => {
  { let v: u32 = cpu.r32(0x50a96fbu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x50a9808; continue; }
  pc = 0x50a97d0; continue;
 }
 0x50a97d0 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x50a97f5; continue; }
  pc = 0x50a97d6; continue;
 }
 0x50a97d6 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50a9808; continue; }
  pc = 0x50a97e0; continue;
 }
 0x50a97e0 => {
  if !cpu.zf { pc = 0x50a97f5; continue; }
  pc = 0x50a97e2; continue;
 }
 0x50a97e2 => {
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x50a97eb; continue; }
  pc = 0x50a97e6; continue;
 }
 0x50a97e6 => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x50a97ed; continue;
 }
 0x50a97eb => {
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x50a97ed; continue;
 }
 0x50a97ed => {
  if cpu.zf { pc = 0x50a9808; continue; }
  pc = 0x50a97f3; continue;
 }
 0x50a97f3 => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a97f5; continue;
 }
 0x50a97f5 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50a9808; continue; }
  pc = 0x50a97f9; continue;
 }
 0x50a97f9 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x50a9808; continue; }
  pc = 0x50a9801; continue;
 }
 0x50a9801 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a9808; continue;
 }
 0x50a9808 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a980a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50a986d; continue; }
  pc = 0x50a981a; continue;
 }
 0x50a981a => {
  { let v: u32 = cpu.r32(0x50a96fbu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi & cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) ^ cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x50a986a; continue; }
  pc = 0x50a982f; continue;
 }
 0x50a982f => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x50a985a; continue; }
  pc = 0x50a9835; continue;
 }
 0x50a9835 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50a986a; continue; }
  pc = 0x50a9841; continue;
 }
 0x50a9841 => {
  if !cpu.zf { pc = 0x50a985a; continue; }
  pc = 0x50a9843; continue;
 }
 0x50a9843 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50a985e; continue; }
  pc = 0x50a984b; continue;
 }
 0x50a984b => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x50a9852; continue; }
  pc = 0x50a984f; continue;
 }
 0x50a984f => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x50a9852; continue;
 }
 0x50a9852 => {
  if cpu.zf { pc = 0x50a986a; continue; }
  pc = 0x50a9858; continue;
 }
 0x50a9858 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x50a985a; continue;
 }
 0x50a985a => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  if cpu.zf { pc = 0x50a986a; continue; }
  pc = 0x50a985e; continue;
 }
 0x50a985e => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x50a986a; continue; }
  pc = 0x50a9863; continue;
 }
 0x50a9863 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a986a; continue;
 }
 0x50a986a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a986d => {
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x50a98a2; continue; }
  pc = 0x50a9874; continue;
 }
 0x50a9874 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x50a9892; continue; }
  pc = 0x50a987a; continue;
 }
 0x50a987a => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a98a2; continue; }
  pc = 0x50a9882; continue;
 }
 0x50a9882 => {
  { cpu.sub32(cpu.esi, 0x80000000u32); }
  if !cpu.zf { pc = 0x50a9892; continue; }
  pc = 0x50a988a; continue;
 }
 0x50a988a => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50a98a2; continue; }
  pc = 0x50a9892; continue;
 }
 0x50a9892 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a98a2 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a98ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a98ae); }
 let mut pc: u32 = 0x50a98ae;
 loop { match pc {
 0x50a98ae => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.sbb8(((cpu.edx >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.eax >> 8) as u8), 199u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.r32(0xb293ffau32.wrapping_add(cpu.ebp)), 1u32); cpu.w32(0xb293ffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8(cpu.r8(0xbffb84c6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.ebx as u8)); cpu.w8(0xbffb84c6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.sbb8(((cpu.eax >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let at = 0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebp = v; cpu.set_segment(3, selector); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  {  }
  cpu.unimplemented(0x50a98cb, "Aas");
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0xb99d0a6bu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x171bffbu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xe38e2727u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  {  }
  cpu.unimplemented(0x50a98df, "Aas");
  { cpu.df = false; }
  { let v = 0x2472dd9cu32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.df = false; }
  { let v: u32 = 0x94ebadc1u32; cpu.edi = v; }
  cpu.unimplemented(0x50a98ee, "Int3");
  cpu.unimplemented(0x50a98ef, "Int3");
  cpu.unimplemented(0x50a98f0, "Int3");
  cpu.unimplemented(0x50a98f1, "Int3");
  { cpu.df = false; }
  cpu.unimplemented(0x50a98f3, "Aas");
  { let v: u32 = cpu.adc32(cpu.r32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0xbffdaau32); cpu.w32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x3fu8); }
  cpu.flat_ds(0x50a9905); f_050a9908(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9908(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9908); }
 let mut pc: u32 = 0x50a9908;
 loop { match pc {
 0x50a9908 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a9946; continue; }
  pc = 0x50a9916; continue;
 }
 0x50a9916 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x50a9931; continue; }
  pc = 0x50a991c; continue;
 }
 0x50a991c => {
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a9944; continue;
 }
 0x50a9931 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a9944; continue;
 }
 0x50a9944 => {
  pc = 0x50a99ba; continue;
 }
 0x50a9946 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a996e; continue; }
  pc = 0x50a994f; continue;
 }
 0x50a994f => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a99ba; continue;
 }
 0x50a996e => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x50a9976); f_050a90eb(cpu); if cpu.returned_to != 0x50a9976 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x50a997d); f_050a99be(cpu); if cpu.returned_to != 0x50a997d { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x50a99a0; continue; }
  pc = 0x50a9984; continue;
 }
 0x50a9984 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x50a999e; continue; }
  pc = 0x50a9988; continue;
 }
 0x50a9988 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x4000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x50a999e); f_050a8eb6(cpu); if cpu.returned_to != 0x50a999e { return; }
  pc = 0x50a999e; continue;
 }
 0x50a999e => {
  pc = 0x50a99ba; continue;
 }
 0x50a99a0 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50a99ba; continue; }
  pc = 0x50a99a4; continue;
 }
 0x50a99a4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xc000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x50a99ba); f_050a8eb6(cpu); if cpu.returned_to != 0x50a99ba { return; }
  pc = 0x50a99ba; continue;
 }
 0x50a99ba => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a99be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a99be); }
 let mut pc: u32 = 0x50a99be;
 loop { match pc {
 0x50a99be => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if !cpu.zf { pc = 0x50a9a00; continue; }
  pc = 0x50a99d4; continue;
 }
 0x50a99d4 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x80000000u32); }
  if !cpu.zf { pc = 0x50a9a00; continue; }
  pc = 0x50a99dd; continue;
 }
 0x50a99dd => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9a00; continue; }
  pc = 0x50a99e2; continue;
 }
 0x50a99e2 => {
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = 0x3ffeu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x80u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), (cpu.ecx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9a00 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x50a9a36; continue; }
  pc = 0x50a9a0a; continue;
 }
 0x50a9a0a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x50a9a2a); f_050a9115(cpu); if cpu.returned_to != 0x50a9a2a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50a9a36; continue;
 }
 0x50a9a36 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3ffdu16); }
  if !cpu.zf { pc = 0x50a9a51; continue; }
  pc = 0x50a9a40; continue;
 }
 0x50a9a40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8930a2f4u32); }
  if !cpu.zf { pc = 0x50a9a51; continue; }
  pc = 0x50a9a4a; continue;
 }
 0x50a9a4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xf66ab09bu32); }
  pc = 0x50a9a51; continue;
 }
 0x50a9a51 => {
  if (cpu.cf || cpu.zf) { pc = 0x50a9ac4; continue; }
  pc = 0x50a9a53; continue;
 }
 0x50a9a53 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x50a9a71); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9a71 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x50a9a91); f_050a92ea(cpu); if cpu.returned_to != 0x50a9a91 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0xbfffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x50a9aa7); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9aa7 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50a9ab7); f_050a9115(cpu); if cpu.returned_to != 0x50a9ab7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50a9ac4; continue;
 }
 0x50a9ac4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x50a98aeu32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.push(0x50a9ad6); f_050aa53a(cpu); if cpu.returned_to != 0x50a9ad6 { return; }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x50a9af2; continue; }
  pc = 0x50a9ada; continue;
 }
 0x50a9ada => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0x860a91c1u32; cpu.push(v); }
  { let v = 0x6b9b2c23u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50a9af2); f_050a8eb6(cpu); if cpu.returned_to != 0x50a9af2 { return; }
  pc = 0x50a9af2; continue;
 }
 0x50a9af2 => {
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x50a9b12; continue; }
  pc = 0x50a9af6; continue;
 }
 0x50a9af6 => {
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50a9b0e); f_050a8eb6(cpu); if cpu.returned_to != 0x50a9b0e { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50a9b12; continue;
 }
 0x50a9b12 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x50a9b1b; continue; }
  pc = 0x50a9b17; continue;
 }
 0x50a9b17 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x50a9b1b; continue;
 }
 0x50a9b1b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9b21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9b21); }
 let mut pc: u32 = 0x50a9b21;
 loop { match pc {
 0x50a9b21 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x50a9b36; continue; }
  pc = 0x50a9b28; continue;
 }
 0x50a9b28 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9b36; continue; }
  pc = 0x50a9b2e; continue;
 }
 0x50a9b2e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9b36; continue; }
  pc = 0x50a9b33; continue;
 }
 0x50a9b33 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9b36 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x50a9b56; continue; }
  pc = 0x50a9b3d; continue;
 }
 0x50a9b3d => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9b56; continue; }
  pc = 0x50a9b43; continue;
 }
 0x50a9b43 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9b56; continue; }
  pc = 0x50a9b48; continue;
 }
 0x50a9b48 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a9b56 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50a9b74); f_050a9b87(cpu); if cpu.returned_to != 0x50a9b74 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9b87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9b87); }
 let mut pc: u32 = 0x50a9b87;
 loop { match pc {
 0x50a9b87 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi & 0x7fffu32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0x7fffu32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x50a9be6; continue; }
  pc = 0x50a9b99; continue;
 }
 0x50a9b99 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x50a9b9b; continue;
 }
 0x50a9b9b => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.zf { pc = 0x50a9ba7; continue; }
  pc = 0x50a9b9f; continue;
 }
 0x50a9b9f => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.zf { pc = 0x50a9ba9; continue; }
  pc = 0x50a9ba7; continue;
 }
 0x50a9ba7 => {
  if !cpu.cf { pc = 0x50a9bae; continue; }
  pc = 0x50a9ba9; continue;
 }
 0x50a9ba9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { cpu.cf = true; }
  pc = 0x50a9bae; continue;
 }
 0x50a9bae => {
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x50a9bc3; continue; }
  pc = 0x50a9bb7; continue;
 }
 0x50a9bb7 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.cf { cpu.poll(); pc = 0x50a9ba9; continue; }
  pc = 0x50a9bbd; continue;
 }
 0x50a9bbd => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { cpu.poll(); pc = 0x50a9b9b; continue; }
  pc = 0x50a9bc1; continue;
 }
 0x50a9bc1 => {
  cpu.poll(); pc = 0x50a9bae; continue;
 }
 0x50a9bc3 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a9bcf; continue; }
  pc = 0x50a9bc9; continue;
 }
 0x50a9bc9 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a9bcf; continue; }
  pc = 0x50a9bcd; continue;
 }
 0x50a9bcd => {
  pc = 0x50a9be4; continue;
 }
 0x50a9bcf => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x50a9bda; continue; }
  pc = 0x50a9bd3; continue;
 }
 0x50a9bd3 => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50a9bcf; continue;
 }
 0x50a9bda => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x7fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x50a9be4; continue;
 }
 0x50a9be4 => {
  pc = 0x50a9bea; continue;
 }
 0x50a9be6 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x50a9bea; continue;
 }
 0x50a9bea => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi | cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50a9bfa; continue; }
  pc = 0x50a9bf8; continue;
 }
 0x50a9bf8 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x50a9bfa; continue;
 }
 0x50a9bfa => {
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x50a9c01; continue; }
  pc = 0x50a9bff; continue;
 }
 0x50a9bff => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x50a9c01; continue;
 }
 0x50a9c01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9c02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9c02); }
 let mut pc: u32 = 0x50a9c02;
 loop { match pc {
 0x50a9c02 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x50a73beu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x50a9c20; continue; }
  pc = 0x50a9c1a; continue;
 }
 0x50a9c1a => {
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50a9c93; continue;
 }
 0x50a9c20 => {
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) & (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.rol8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x50a9c54; continue; }
  pc = 0x50a9c3a; continue;
 }
 0x50a9c3a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x50a9c52; continue; }
  pc = 0x50a9c43; continue;
 }
 0x50a9c43 => {
  { let v: u16 = cpu.r16(0x50a73b8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a9c52; continue;
 }
 0x50a9c52 => {
  pc = 0x50a9c93; continue;
 }
 0x50a9c54 => {
  { cpu.sub16((cpu.eax as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x50a9c7c; continue; }
  pc = 0x50a9c5a; continue;
 }
 0x50a9c5a => {
  { let v: u16 = cpu.r16(0x50a73bcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x50a9c74; continue; }
  pc = 0x50a9c6c; continue;
 }
 0x50a9c6c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  pc = 0x50a9c74; continue;
 }
 0x50a9c74 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x50a9c7a; continue; }
  pc = 0x50a9c78; continue;
 }
 0x50a9c78 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a9c7a; continue;
 }
 0x50a9c7a => {
  pc = 0x50a9c93; continue;
 }
 0x50a9c7c => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.ebx)), 0x80u8); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x50a9c93; continue; }
  pc = 0x50a9c84; continue;
 }
 0x50a9c84 => {
  { let v: u16 = cpu.r16(0x50a73bcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50a9c93; continue;
 }
 0x50a9c93 => {
  { let v: u8 = (cpu.eax as u8) | ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), (cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9caa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9caa); }
 let mut pc: u32 = 0x50a9caa;
 loop { match pc {
 0x50a9caa => {
  cpu.call(0x9fc085f5); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9cc8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a9cc8); } r_050a9cc8(cpu, 0x50a9cc8); }
pub fn f_050a9d09(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a9d09); } r_050a9cc8(cpu, 0x50a9d09); }
fn r_050a9cc8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a9cc8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x76bb3fffu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x76bb3fffu32.wrapping_add(cpu.eax), v); }
  if cpu.of { pc = 0x50a9d14; continue; }
  pc = 0x50a9cd6; continue;
 }
 0x50a9cd6 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xef60c004u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.r32(0x9c041fd0u32.wrapping_add(cpu.ecx))); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0xf4e06c83u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.ror8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 192u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_050a9cf0(cpu); return;
 }
 0x50a9d09 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x50a9d14); f_050a9d4c(cpu); if cpu.returned_to != 0x50a9d14 { return; }
  pc = 0x50a9d14; continue;
 }
 0x50a9d14 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x50a9d1d); f_050a92c0(cpu); if cpu.returned_to != 0x50a9d1d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8aa3b29u32; cpu.edx = v; }
  { let v: u32 = 0x5c17f0bdu32; cpu.eax = v; }
  cpu.push(0x50a9d3d); f_050a92ea(cpu); if cpu.returned_to != 0x50a9d3d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9cf0); }
 let mut pc: u32 = 0x50a9cf0;
 loop { match pc {
 0x50a9cf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3fffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x50a9d06); f_050a8eb6(cpu); if cpu.returned_to != 0x50a9d06 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_050a9d09(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9d4c); }
 let mut pc: u32 = 0x50a9d4c;
 loop { match pc {
 0x50a9d4c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x50a9d5e; continue; }
  pc = 0x50a9d5c; continue;
 }
 0x50a9d5c => {
  pc = 0x50a9d5e; continue;
 }
 0x50a9d5e => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x3ffeu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x3ffeu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = 0xbffe3ffeu32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50a9d86); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9d86 { return; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3ffeu16); }
  if !cpu.zf { pc = 0x50a9d9d; continue; }
  pc = 0x50a9d8e; continue;
 }
 0x50a9d8e => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xb504f333u32); }
  if !cpu.zf { pc = 0x50a9d9d; continue; }
  pc = 0x50a9d97; continue;
 }
 0x50a9d97 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0xf9de6484u32); }
  pc = 0x50a9d9d; continue;
 }
 0x50a9d9d => {
  if (cpu.cf || cpu.zf) { pc = 0x50a9db9; continue; }
  pc = 0x50a9d9f; continue;
 }
 0x50a9d9f => {
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbffe0000u32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50a9db7); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9db7 { return; }
  pc = 0x50a9dc5; continue;
 }
 0x50a9db9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x50a9dc5; continue;
 }
 0x50a9dc5 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.dec16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffe0000u32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50a9deb); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9deb { return; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x50a9dfd); f_050a9115(cpu); if cpu.returned_to != 0x50a9dfd { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x50a9e11); f_050a92ea(cpu); if cpu.returned_to != 0x50a9e11 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x50a9cc8u32; cpu.ecx = v; }
  cpu.push(0x50a9e2a); f_050aa4e8(cpu); if cpu.returned_to != 0x50a9e2a { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x50a9caau32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  cpu.push(0x50a9e46); f_050aa4e8(cpu); if cpu.returned_to != 0x50a9e46 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50a9e57); f_050a9115(cpu); if cpu.returned_to != 0x50a9e57 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x50a9e6b); f_050a92ea(cpu); if cpu.returned_to != 0x50a9e6b { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x50a9e82); f_050a92ea(cpu); if cpu.returned_to != 0x50a9e82 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50a9e93); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9e93 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50a9f0e; continue; }
  pc = 0x50a9e98; continue;
 }
 0x50a9e98 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x50a9ea4); f_050a94aa(cpu); if cpu.returned_to != 0x50a9ea4 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbff20000u32; cpu.esi = v; }
  { let v: u32 = 0xde8082e3u32; cpu.ecx = v; }
  { let v: u32 = 0x865435cu32; cpu.ebx = v; }
  cpu.push(0x50a9ec8); f_050a92ea(cpu); if cpu.returned_to != 0x50a9ec8 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50a9ed9); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9ed9 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffe0000u32; cpu.esi = v; }
  { let v: u32 = 0xb1800000u32; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50a9efd); f_050a92ea(cpu); if cpu.returned_to != 0x50a9efd { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50a9f0e); f_050a8ee6(cpu); if cpu.returned_to != 0x50a9f0e { return; }
  pc = 0x50a9f0e; continue;
 }
 0x50a9f0e => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9f1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a9f1e); }
 let mut pc: u32 = 0x50a9f1e;
 loop { match pc {
 0x50a9f1e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffff85u32; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x3fceca96u32); }
  cpu.call(0x50a9f28); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a9fd2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a9fd2); } r_050a9fd2(cpu, 0x50a9fd2); }
pub fn f_050a9fe4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a9fe4); } r_050a9fd2(cpu, 0x50a9fe4); }
fn r_050a9fd2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a9fd2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x6000500u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x57u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.edx as u8)); cpu.w8(0x57u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (cpu.sf != cpu.of) { pc = 0x50aa006; continue; }
  pc = 0x50aa006; continue;
 }
 0x50a9fe4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (cpu.sf != cpu.of) { pc = 0x50aa006; continue; }
  pc = 0x50aa006; continue;
 }
 0x50aa006 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3ffeu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc90fdaa2u32; cpu.ecx = v; }
  { let v: u32 = 0x2168c235u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50aa025); f_050a9b87(cpu); if cpu.returned_to != 0x50aa025 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); }
  if cpu.zf { pc = 0x50aa055; continue; }
  pc = 0x50aa03a; continue;
 }
 0x50aa03a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x50aa053); f_050a8eb6(cpu); if cpu.returned_to != 0x50aa053 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x50aa055; continue;
 }
 0x50aa055 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x50aa081; continue; }
  pc = 0x50aa065; continue;
 }
 0x50aa065 => {
  { let v = 0xbffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50aa07d); f_050a8eb6(cpu); if cpu.returned_to != 0x50aa07d { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50aa081; continue;
 }
 0x50aa081 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xffe0u16); }
  if (cpu.sf == cpu.of) { pc = 0x50aa0ab; continue; }
  pc = 0x50aa092; continue;
 }
 0x50aa092 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aa142; continue;
 }
 0x50aa0ab => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x50aa0b4; continue; }
  pc = 0x50aa0b0; continue;
 }
 0x50aa0b0 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50aa0b4; continue;
 }
 0x50aa0b4 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aa0c1; continue; }
  pc = 0x50aa0bd; continue;
 }
 0x50aa0bd => {
  { let v: u16 = 0x8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x50aa0c1; continue;
 }
 0x50aa0c1 => {
  { let v: u16 = cpu.r16(0x50a9fd2u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x50aa0db; continue; }
  pc = 0x50aa0d4; continue;
 }
 0x50aa0d4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x50aa0db; continue; }
  pc = 0x50aa0d9; continue;
 }
 0x50aa0d9 => {
  pc = 0x50aa119; continue;
 }
 0x50aa0db => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x50aa0f5); f_050a92ea(cpu); if cpu.returned_to != 0x50aa0f5 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x50a9f78u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  cpu.push(0x50aa10e); f_050aa4e8(cpu); if cpu.returned_to != 0x50aa10e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x50aa119; continue;
 }
 0x50aa119 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x50aa124; continue; }
  pc = 0x50aa11d; continue;
 }
 0x50aa11d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x50aa124; continue; }
  pc = 0x50aa122; continue;
 }
 0x50aa122 => {
  pc = 0x50aa142; continue;
 }
 0x50aa124 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x50a9f1eu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  cpu.push(0x50aa142); f_050aa53a(cpu); if cpu.returned_to != 0x50aa142 { return; }
  pc = 0x50aa142; continue;
 }
 0x50aa142 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x50aa157; continue; }
  pc = 0x50aa14b; continue;
 }
 0x50aa14b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa160; continue;
 }
 0x50aa157 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa160; continue;
 }
 0x50aa160 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x50aa16d; continue; }
  pc = 0x50aa168; continue;
 }
 0x50aa168 => {
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa16d; continue;
 }
 0x50aa16d => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x50aa1ca; continue; }
  pc = 0x50aa172; continue;
 }
 0x50aa172 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x50aa190; continue; }
  pc = 0x50aa181; continue;
 }
 0x50aa181 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa19c; continue;
 }
 0x50aa190 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa19c; continue;
 }
 0x50aa19c => {
  if cpu.zf { pc = 0x50aa1a4; continue; }
  pc = 0x50aa19e; continue;
 }
 0x50aa19e => {
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  pc = 0x50aa1a4; continue;
 }
 0x50aa1a4 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x50aa1c5; continue; }
  pc = 0x50aa1ab; continue;
 }
 0x50aa1ab => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50aa1c5; continue; }
  pc = 0x50aa1af; continue;
 }
 0x50aa1af => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x50aa1c5; continue; }
  pc = 0x50aa1b7; continue;
 }
 0x50aa1b7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa1ca; continue;
 }
 0x50aa1c5 => {
  cpu.push(0x50aa1ca); f_050a9115(cpu); if cpu.returned_to != 0x50aa1ca { return; }
  pc = 0x50aa1ca; continue;
 }
 0x50aa1ca => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa1db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa1db); }
 let mut pc: u32 = 0x50aa1db;
 loop { match pc {
 0x50aa1db => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x50aa1e3); f_050a9fe4(cpu); if cpu.returned_to != 0x50aa1e3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa1e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa1e5); }
 let mut pc: u32 = 0x50aa1e5;
 loop { match pc {
 0x50aa1e5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  cpu.push(0x50aa1f0); f_050a9fe4(cpu); if cpu.returned_to != 0x50aa1f0 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa1f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa1f2); }
 let mut pc: u32 = 0x50aa1f2;
 loop { match pc {
 0x50aa1f2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x50aa1fd); f_050a9fe4(cpu); if cpu.returned_to != 0x50aa1fd { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa200); }
 let mut pc: u32 = 0x50aa200;
 loop { match pc {
 0x50aa200 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x67cc487bu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa228); }
 let mut pc: u32 = 0x50aa228;
 loop { match pc {
 0x50aa228 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x0u16); }
  if !cpu.zf { pc = 0x50aa24a; continue; }
  pc = 0x50aa241; continue;
 }
 0x50aa241 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x50aa24a; continue; }
  pc = 0x50aa247; continue;
 }
 0x50aa247 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  pc = 0x50aa24a; continue;
 }
 0x50aa24a => {
  if cpu.zf { pc = 0x50aa2b2; continue; }
  pc = 0x50aa24c; continue;
 }
 0x50aa24c => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x50aa26b; continue; }
  pc = 0x50aa251; continue;
 }
 0x50aa251 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50aa267); f_050a8eb6(cpu); if cpu.returned_to != 0x50aa267 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50aa26b; continue;
 }
 0x50aa26b => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x50aa27a; continue;
 }
 0x50aa27a => {
  { cpu.sub16((cpu.ebx as u16), 0x3fffu16); }
  if cpu.cf { pc = 0x50aa28b; continue; }
  pc = 0x50aa281; continue;
 }
 0x50aa281 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x50aa27a; continue;
 }
 0x50aa28b => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50aa291; continue; }
  pc = 0x50aa28f; continue;
 }
 0x50aa28f => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50aa291; continue;
 }
 0x50aa291 => {
  if !cpu.zf { pc = 0x50aa297; continue; }
  pc = 0x50aa293; continue;
 }
 0x50aa293 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x50aa2a7; continue;
 }
 0x50aa297 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x50aa2a3; continue; }
  pc = 0x50aa29b; continue;
 }
 0x50aa29b => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x50aa297; continue;
 }
 0x50aa2a3 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x50aa2a7; continue;
 }
 0x50aa2a7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50aa2b4; continue;
 }
 0x50aa2b2 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50aa2b4; continue;
 }
 0x50aa2b4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x50aa2cf); f_050a92ea(cpu); if cpu.returned_to != 0x50aa2cf { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xa6829a79u32; cpu.ecx = v; }
  { let v: u32 = 0x1bf21f8cu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40030000u32; cpu.esi = v; }
  cpu.push(0x50aa2ed); f_050a8ee6(cpu); if cpu.returned_to != 0x50aa2ed { return; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = 0xec96f0d6u32; cpu.ecx = v; }
  { let v: u32 = 0x9d7bfdbu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffa0000u32; cpu.esi = v; }
  cpu.push(0x50aa313); f_050a92ea(cpu); if cpu.returned_to != 0x50aa313 { return; }
  { let v: u32 = 0xe6d5051au32; cpu.ecx = v; }
  { let v: u32 = 0xe536e187u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40010000u32; cpu.esi = v; }
  cpu.push(0x50aa32e); f_050a8ee6(cpu); if cpu.returned_to != 0x50aa32e { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x50aa342); f_050a92ea(cpu); if cpu.returned_to != 0x50aa342 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x50aa360); f_050a8ee6(cpu); if cpu.returned_to != 0x50aa360 { return; }
  { let a = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x50aa37e); f_050a8ee6(cpu); if cpu.returned_to != 0x50aa37e { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50aa390); f_050a9115(cpu); if cpu.returned_to != 0x50aa390 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x50aa200u32; cpu.esi = v; }
  pc = 0x50aa3a3; continue;
 }
 0x50aa3a3 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50aa3db; continue; }
  pc = 0x50aa3a7; continue;
 }
 0x50aa3a7 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x50aa3d6; continue; }
  pc = 0x50aa3ab; continue;
 }
 0x50aa3ab => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x50aa3cb); f_050a92ea(cpu); if cpu.returned_to != 0x50aa3cb { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x50aa3d6; continue;
 }
 0x50aa3d6 => {
  { let v: u32 = 0xau32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50aa3a3; continue;
 }
 0x50aa3db => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.add16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.esi as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50aa3f5); f_050a8eb6(cpu); if cpu.returned_to != 0x50aa3f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa3fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa3fc); }
 let mut pc: u32 = 0x50aa3fc;
 loop { match pc {
 0x50aa3fc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x50aa442; continue; }
  pc = 0x50aa40e; continue;
 }
 0x50aa40e => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50aa437; continue; }
  pc = 0x50aa412; continue;
 }
 0x50aa412 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50aa4c2; continue; }
  pc = 0x50aa41b; continue;
 }
 0x50aa41b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50aa42e; continue;
 }
 0x50aa42e => {
  { let v: u8 = cpu.or8(cpu.r8(0x7u32.wrapping_add(cpu.eax)), 0xc0u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  pc = 0x50aa4c2; continue;
 }
 0x50aa437 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x50aa4c2; continue; }
  pc = 0x50aa442; continue;
 }
 0x50aa442 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x7fffu16); }
  if cpu.zf { cpu.poll(); pc = 0x50aa42e; continue; }
  pc = 0x50aa44e; continue;
 }
 0x50aa44e => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { cpu.poll(); pc = 0x50aa42e; continue; }
  pc = 0x50aa453; continue;
 }
 0x50aa453 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50aa45f; continue; }
  pc = 0x50aa458; continue;
 }
 0x50aa458 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x50aa41b; continue; }
  pc = 0x50aa45f; continue;
 }
 0x50aa45f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x50aa475; continue; }
  pc = 0x50aa46f; continue;
 }
 0x50aa46f => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x50aa475; continue;
 }
 0x50aa475 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x50aa4a8; continue; }
  pc = 0x50aa48a; continue;
 }
 0x50aa48a => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x50aa48b; continue;
 }
 0x50aa48b => {
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x50aa49d; continue; }
  pc = 0x50aa492; continue;
 }
 0x50aa492 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x50aa48b; continue;
 }
 0x50aa49d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x50aa4b1; continue;
 }
 0x50aa4a8 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x50aa4ba; continue; }
  pc = 0x50aa4ac; continue;
 }
 0x50aa4ac => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  pc = 0x50aa4b1; continue;
 }
 0x50aa4b1 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.edx); cpu.eax = v; }
  pc = 0x50aa4ba; continue;
 }
 0x50aa4ba => {
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x50aa4c2; continue;
 }
 0x50aa4c2 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa4c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa4c8); }
 let mut pc: u32 = 0x50aa4c8;
 loop { match pc {
 0x50aa4c8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50aa4db); f_050aa4e8(cpu); if cpu.returned_to != 0x50aa4db { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa4e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa4e8); }
 let mut pc: u32 = 0x50aa4e8;
 loop { match pc {
 0x50aa4e8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x50aa4ff; continue;
 }
 0x50aa4ff => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x50aa514); f_050a92ea(cpu); if cpu.returned_to != 0x50aa514 { return; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x50aa52e); f_050a8ee6(cpu); if cpu.returned_to != 0x50aa52e { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x50aa4ff; continue; }
  pc = 0x50aa533; continue;
 }
 0x50aa533 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa53a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa53a); }
 let mut pc: u32 = 0x50aa53a;
 loop { match pc {
 0x50aa53a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x50aa55a); f_050a92ea(cpu); if cpu.returned_to != 0x50aa55a { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x50aa561); f_050aa4e8(cpu); if cpu.returned_to != 0x50aa561 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x50aa575); f_050a92ea(cpu); if cpu.returned_to != 0x50aa575 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa582(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa582); }
 let mut pc: u32 = 0x50aa582;
 loop { match pc {
 0x50aa582 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u16 = 0x1600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x50aa5f2; continue; }
  pc = 0x50aa595; continue;
 }
 0x50aa595 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x50aa5f2; continue; }
  pc = 0x50aa5a1; continue;
 }
 0x50aa5a1 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, 0x666u32); }
  if !cpu.zf { pc = 0x50aa5f2; continue; }
  pc = 0x50aa5b5; continue;
 }
 0x50aa5b5 => {
  { let v: u8 = 0x22u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x50aa5f2; continue; }
  pc = 0x50aa5c6; continue;
 }
 0x50aa5c6 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x514bca1u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u32 = 0x52b5054u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50aa64c; continue;
 }
 0x50aa5f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x50aa5fe; continue; }
  pc = 0x50aa5f9; continue;
 }
 0x50aa5f9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x50aa64a; continue; }
  pc = 0x50aa5fe; continue;
 }
 0x50aa5fe => {
  { let v: u32 = 0xa00u32; cpu.eax = v; }
  { let v: u32 = 0x513b0b0u32; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50aa64a; continue; }
  pc = 0x50aa612; continue;
 }
 0x50aa612 => {
  { let v: u32 = 0x50a7744u32; cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  cpu.push(0x50aa623); f_05098b85(cpu); if cpu.returned_to != 0x50aa623 { return; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.w8(0x3eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x40u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x514bca0u32, v); }
  pc = 0x50aa64a; continue;
 }
 0x50aa64a => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50aa64c; continue;
 }
 0x50aa64c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa656); }
 let mut pc: u32 = 0x50aa656;
 loop { match pc {
 0x50aa656 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x514bca1u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x50aa685; continue; }
  pc = 0x50aa665; continue;
 }
 0x50aa665 => {
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x21u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1fu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aa685 => {
  { cpu.sub8(cpu.r8(0x514bca0u32), 0x0u8); }
  if cpu.zf { pc = 0x50aa6a9; continue; }
  pc = 0x50aa68e; continue;
 }
 0x50aa68e => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3eu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = (cpu.ebx as u8) & 0xfbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x50aa6a9; continue;
 }
 0x50aa6a9 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa6af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa6af); }
 let mut pc: u32 = 0x50aa6af;
 loop { match pc {
 0x50aa6af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa6b0); }
 let mut pc: u32 = 0x50aa6b0;
 loop { match pc {
 0x50aa6b0 => {
  { let v = (cpu.eax as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpush(1.0); }
  { cpu.fpush(0.0); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x50aa6ca; continue; }
  pc = 0x50aa6c8; continue;
 }
 0x50aa6c8 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50aa6ca; continue;
 }
 0x50aa6ca => {
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  { let a = cpu.r16(0x0u32.wrapping_add(cpu.esp)); let b = (cpu.eax as u16); { let v: u16 = b; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa6e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa6e2); }
 let mut pc: u32 = 0x50aa6e2;
 loop { match pc {
 0x50aa6e2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x50aa6f1); f_05097dd4(cpu); if cpu.returned_to != 0x50aa6f1 { return; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if !cpu.cf { pc = 0x50aa73c; continue; }
  pc = 0x50aa6f9; continue;
 }
 0x50aa6f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad1fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aa722; continue; }
  pc = 0x50aa705; continue;
 }
 0x50aa705 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if !cpu.zf { pc = 0x50aa712; continue; }
  pc = 0x50aa70e; continue;
 }
 0x50aa70e => {
  { let v: u8 = 0xeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50aa722; continue;
 }
 0x50aa712 => {
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aa71b; continue; }
  pc = 0x50aa717; continue;
 }
 0x50aa717 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50aa722; continue;
 }
 0x50aa71b => {
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aa722; continue; }
  pc = 0x50aa720; continue;
 }
 0x50aa720 => {
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50aa722; continue;
 }
 0x50aa722 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x13u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aa72d; continue; }
  pc = 0x50aa72b; continue;
 }
 0x50aa72b => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50aa72d; continue;
 }
 0x50aa72d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x514bcadu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x50aa746; continue;
 }
 0x50aa73c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x50aa746; continue;
 }
 0x50aa746 => {
  cpu.push(0x50aa74b); f_05097db1(cpu); if cpu.returned_to != 0x50aa74b { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa753(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa753); }
 let mut pc: u32 = 0x50aa753;
 loop { match pc {
 0x50aa753 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa77a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa77a); }
 let mut pc: u32 = 0x50aa77a;
 loop { match pc {
 0x50aa77a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x514bcc8u32)); }
  if cpu.cf { pc = 0x50aa78d; continue; }
  pc = 0x50aa787; continue;
 }
 0x50aa787 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aa78d => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50aa7c3; continue; }
  pc = 0x50aa792; continue;
 }
 0x50aa792 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514bd1cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x50aa7c3; continue; }
  pc = 0x50aa7a6; continue;
 }
 0x50aa7a6 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50aa7b5); f_050aa753(cpu); if cpu.returned_to != 0x50aa7b5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50aa7c3; continue; }
  pc = 0x50aa7b9; continue;
 }
 0x50aa7b9 => {
  { let v: u32 = cpu.r32(0x514bd1cu32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x50aa7c3; continue;
 }
 0x50aa7c3 => {
  { let v: u32 = cpu.r32(0x514bd1cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa7cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa7cf); }
 let mut pc: u32 = 0x50aa7cf;
 loop { match pc {
 0x50aa7cf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x514bd1cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa7e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa7e4); }
 let mut pc: u32 = 0x50aa7e4;
 loop { match pc {
 0x50aa7e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50aa806; continue; }
  pc = 0x50aa7f7; continue;
 }
 0x50aa7f7 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x50aa800); f_050aa7cf(cpu); if cpu.returned_to != 0x50aa800 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aa806 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x50aa810); f_05097db1(cpu); if cpu.returned_to != 0x50aa810 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa819(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa819); }
 let mut pc: u32 = 0x50aa819;
 loop { match pc {
 0x50aa819 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x514acf8u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa843(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa843); }
 let mut pc: u32 = 0x50aa843;
 loop { match pc {
 0x50aa843 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x50aa868; continue; }
  pc = 0x50aa850; continue;
 }
 0x50aa850 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x50aa85a); f_05097db1(cpu); if cpu.returned_to != 0x50aa85a { return; }
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa85e; continue;
 }
 0x50aa85e => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x50aa8e1; continue;
 }
 0x50aa868 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x50aa875; continue; }
  pc = 0x50aa86e; continue;
 }
 0x50aa86e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50aa875); f_050aa8e7(cpu); if cpu.returned_to != 0x50aa875 { return; }
  pc = 0x50aa875; continue;
 }
 0x50aa875 => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x50aa8b2; continue; }
  pc = 0x50aa87f; continue;
 }
 0x50aa87f => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x50aa8b2; continue; }
  pc = 0x50aa88b; continue;
 }
 0x50aa88b => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if !cpu.zf { pc = 0x50aa8b2; continue; }
  pc = 0x50aa8a7; continue;
 }
 0x50aa8a7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50aa8ae); f_050a5f0a(cpu); if cpu.returned_to != 0x50aa8ae { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50aa85e; continue; }
  pc = 0x50aa8b2; continue;
 }
 0x50aa8b2 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)) | 0x10u8; cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.edi); }
  if !cpu.zf { pc = 0x50aa8d2; continue; }
  pc = 0x50aa8cd; continue;
 }
 0x50aa8cd => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x50aa8dd; continue; }
  pc = 0x50aa8d2; continue;
 }
 0x50aa8d2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50aa8d9); f_050a5f0a(cpu); if cpu.returned_to != 0x50aa8d9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50aa85e; continue; }
  pc = 0x50aa8dd; continue;
 }
 0x50aa8dd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50aa8e1; continue;
 }
 0x50aa8e1 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa8e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa8e7); }
 let mut pc: u32 = 0x50aa8e7;
 loop { match pc {
 0x50aa8e7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x50aa8f0); f_050adc1f(cpu); if cpu.returned_to != 0x50aa8f0 { return; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x50aa91c; continue; }
  pc = 0x50aa8f6; continue;
 }
 0x50aa8f6 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x50aa907; continue; }
  pc = 0x50aa8fe; continue;
 }
 0x50aa8fe => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa91c; continue;
 }
 0x50aa907 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x50aa915; continue; }
  pc = 0x50aa90c; continue;
 }
 0x50aa90c => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa91c; continue;
 }
 0x50aa915 => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa91c; continue;
 }
 0x50aa91c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x50aa924); f_050a629c(cpu); if cpu.returned_to != 0x50aa924 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50aa94b; continue; }
  pc = 0x50aa92b; continue;
 }
 0x50aa92b => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa94f; continue;
 }
 0x50aa94b => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x50aa94f; continue;
 }
 0x50aa94f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa95e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa95e); }
 let mut pc: u32 = 0x50aa95e;
 loop { match pc {
 0x50aa95e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bd20u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50aa97c; continue; }
  pc = 0x50aa969; continue;
 }
 0x50aa969 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x514bd25u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x50aa97c; continue;
 }
 0x50aa97c => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50aa993; continue; }
  pc = 0x50aa980; continue;
 }
 0x50aa980 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aa993 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa99c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa99c); }
 let mut pc: u32 = 0x50aa99c;
 loop { match pc {
 0x50aa99c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x50aa9b2; continue; }
  pc = 0x50aa9a6; continue;
 }
 0x50aa9a6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aa9b2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa9b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa9b6); }
 let mut pc: u32 = 0x50aa9b6;
 loop { match pc {
 0x50aa9b6 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x50aa9d6; continue; }
  pc = 0x50aa9bc; continue;
 }
 0x50aa9bc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x52b50d4u32, v); }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52b50d5u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x52b50d6u32, v); }
  pc = 0x50aa9e3; continue;
 }
 0x50aa9d6 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x52b50d4u32, v); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x52b50d5u32, v); }
  pc = 0x50aa9e3; continue;
 }
 0x50aa9e3 => {
  { let v: u32 = 0x52b50d4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aa9ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aa9ea); }
 let mut pc: u32 = 0x50aa9ea;
 loop { match pc {
 0x50aa9ea => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.push(0x50aa9f1); f_050aaa18(cpu); if cpu.returned_to != 0x50aa9f1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50aa9fa; continue; }
  pc = 0x50aa9f5; continue;
 }
 0x50aa9f5 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x50aaa15; continue; }
  pc = 0x50aa9fa; continue;
 }
 0x50aa9fa => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if !cpu.zf { pc = 0x50aaa0b; continue; }
  pc = 0x50aaa03; continue;
 }
 0x50aaa03 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if cpu.zf { pc = 0x50aaa15; continue; }
  pc = 0x50aaa0b; continue;
 }
 0x50aaa0b => {
  { let v: u32 = 0x1u32; cpu.w32(0x514bd20u32, v); }
  pc = 0x50aaa15; continue;
 }
 0x50aaa15 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aaa18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aaa18); }
 let mut pc: u32 = 0x50aaa18;
 loop { match pc {
 0x50aaa18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aaabe; continue; }
  pc = 0x50aaa2e; continue;
 }
 0x50aaa2e => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50aaabe; continue; }
  pc = 0x50aaa37; continue;
 }
 0x50aaa37 => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x50aaa47); f_05097b90(cpu); if cpu.returned_to != 0x50aaa47 { return; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x2511u32; cpu.ecx = v; }
  cpu.push(0x50aaa5c); f_05097b90(cpu); if cpu.returned_to != 0x50aaa5c { return; }
  { let v: u32 = 0x6300u32; cpu.ebx = v; }
  { let v: u32 = 0x21u32; cpu.edx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x66u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x50u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x50aaa90); f_050adc50(cpu); if cpu.returned_to != 0x50aaa90 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x50aab1a; continue; }
  pc = 0x50aaaa2; continue;
 }
 0x50aaaa2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x50aaab3; continue;
 }
 0x50aaab3 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x52b50e8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x50aab1e; continue;
 }
 0x50aaabe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50aab1a; continue; }
  pc = 0x50aaaca; continue;
 }
 0x50aaaca => {
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x50aaada); f_05097b90(cpu); if cpu.returned_to != 0x50aaada { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x6300u32; cpu.edx = v; }
  { let v: u8 = 0x21u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u16 = 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x50aab1a; continue; }
  pc = 0x50aab06; continue;
 }
 0x50aab06 => {
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x50aaab3; continue;
 }
 0x50aab1a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50aab1e; continue;
 }
 0x50aab1e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aab25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aab25); }
 let mut pc: u32 = 0x50aab25;
 loop { match pc {
 0x50aab25 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bd20u32); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50aab83; continue; }
  pc = 0x50aab35; continue;
 }
 0x50aab35 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50aab8e; continue; }
  pc = 0x50aab3f; continue;
 }
 0x50aab3f => {
  { let v: u32 = cpu.r32(0x514bd20u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x50aab60; continue; }
  pc = 0x50aab4b; continue;
 }
 0x50aab4b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x514bd25u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  pc = 0x50aab62; continue;
 }
 0x50aab60 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x50aab62; continue;
 }
 0x50aab62 => {
  if cpu.zf { pc = 0x50aab8e; continue; }
  pc = 0x50aab64; continue;
 }
 0x50aab64 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x50aab7c; continue; }
  pc = 0x50aab6a; continue;
 }
 0x50aab6a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aab7c => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aab83 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x50aab8e; continue;
 }
 0x50aab8e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aac2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aac2a); }
 let mut pc: u32 = 0x50aac2a;
 loop { match pc {
 0x50aac2a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bd20u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.cf { pc = 0x50aac42; continue; }
  pc = 0x50aac3b; continue;
 }
 0x50aac3b => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x50aaca4; continue;
 }
 0x50aac42 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x50aaca4; continue; }
  pc = 0x50aac47; continue;
 }
 0x50aac47 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  if cpu.zf { pc = 0x50aac64; continue; }
  pc = 0x50aac4b; continue;
 }
 0x50aac4b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x514bd25u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x50aac66; continue;
 }
 0x50aac64 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x50aac66; continue;
 }
 0x50aac66 => {
  if cpu.zf { pc = 0x50aac6d; continue; }
  pc = 0x50aac68; continue;
 }
 0x50aac68 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x50aaca4; continue;
 }
 0x50aac6d => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50aac8f; continue; }
  pc = 0x50aac72; continue;
 }
 0x50aac72 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x50aac8b; continue; }
  pc = 0x50aac76; continue;
 }
 0x50aac76 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x514bd25u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x50aac8d; continue;
 }
 0x50aac8b => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x50aac8d; continue;
 }
 0x50aac8d => {
  if !cpu.zf { cpu.poll(); pc = 0x50aac6d; continue; }
  pc = 0x50aac8f; continue;
 }
 0x50aac8f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x50aaca4; continue;
 }
 0x50aaca4 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x514bd20u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aacb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aacb0); }
 let mut pc: u32 = 0x50aacb0;
 loop { match pc {
 0x50aacb0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x51439b9u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aad01; continue; }
  pc = 0x50aacde; continue;
 }
 0x50aacde => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x513b0c0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x50aad01; continue; }
  pc = 0x50aacfa; continue;
 }
 0x50aacfa => {
  { let v: u8 = cpu.r8(0x514adccu32) | 0x1u8; cpu.w8(0x514adccu32, v); }
  pc = 0x50aad01; continue;
 }
 0x50aad01 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aad05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aad05); }
 let mut pc: u32 = 0x50aad05;
 loop { match pc {
 0x50aad05 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x50aad0d); f_050aad0f(cpu); if cpu.returned_to != 0x50aad0d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aad0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aad0f); }
 let mut pc: u32 = 0x50aad0f;
 loop { match pc {
 0x50aad0f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x50aad3b; continue; }
  pc = 0x50aad34; continue;
 }
 0x50aad34 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad3b => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x50aad47; continue; }
  pc = 0x50aad40; continue;
 }
 0x50aad40 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad47 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x50aad53; continue; }
  pc = 0x50aad4c; continue;
 }
 0x50aad4c => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad53 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x50aad5f; continue; }
  pc = 0x50aad58; continue;
 }
 0x50aad58 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad5f => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x50aad6b; continue; }
  pc = 0x50aad64; continue;
 }
 0x50aad64 => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad6b => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x50aad75; continue; }
  pc = 0x50aad70; continue;
 }
 0x50aad70 => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x50aad75; continue;
 }
 0x50aad75 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514be24u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x50aad99; continue; }
  pc = 0x50aad8c; continue;
 }
 0x50aad8c => {
  { let v = cpu.rf64(0x513b36cu32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x50aaddb; continue;
 }
 0x50aad99 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x50aada8; continue; }
  pc = 0x50aad9e; continue;
 }
 0x50aad9e => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aaddb; continue;
 }
 0x50aada8 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x50aadbc; continue; }
  pc = 0x50aadad; continue;
 }
 0x50aadad => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aaddb; continue;
 }
 0x50aadbc => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x50aadd0; continue; }
  pc = 0x50aadc1; continue;
 }
 0x50aadc1 => {
  { let v: u32 = cpu.r32(0x513b36cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x513b370u32); cpu.eax = v; }
  pc = 0x50aadd8; continue;
 }
 0x50aadd0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x50aadd8; continue;
 }
 0x50aadd8 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aaddb; continue;
 }
 0x50aaddb => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x50aade3); f_050addbf(cpu); if cpu.returned_to != 0x50aade3 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aadea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aadea); }
 let mut pc: u32 = 0x50aadea;
 loop { match pc {
 0x50aadea => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.cf { pc = 0x50aae2c; continue; }
  pc = 0x50aadfb; continue;
 }
 0x50aadfb => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x50aae07; continue; }
  pc = 0x50aae00; continue;
 }
 0x50aae00 => {
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x50aae18; continue; }
  pc = 0x50aae05; continue;
 }
 0x50aae05 => {
  pc = 0x50aae2c; continue;
 }
 0x50aae07 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50aae50; continue;
 }
 0x50aae18 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x50aae2c; continue; }
  pc = 0x50aae22; continue;
 }
 0x50aae22 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aae58; continue;
 }
 0x50aae2c => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x81u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.edx, 0x6u32); }
  if !cpu.zf { pc = 0x50aae4b; continue; }
  pc = 0x50aae3c; continue;
 }
 0x50aae3c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x50aae4b; continue; }
  pc = 0x50aae46; continue;
 }
 0x50aae46 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x11u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x50aae4b; continue;
 }
 0x50aae4b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x50aae50; continue;
 }
 0x50aae50 => {
  cpu.push(0x50aae55); f_050aad05(cpu); if cpu.returned_to != 0x50aae55 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x50aae58; continue;
 }
 0x50aae58 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aae68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aae68); }
 let mut pc: u32 = 0x50aae68;
 loop { match pc {
 0x50aae68 => {
  { let v = 0x79050ab2u32; cpu.push(v); }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50aae7c; continue;
 }
 0x50aae7c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.r32(0xb2a4050au32.wrapping_add(cpu.edx)), cpu.esi); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50ab2b8u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let at = cpu.ebx.wrapping_add(cpu.eax & 0xff); cpu.eax = (cpu.eax & 0xffffff00) | cpu.r8(at) as u32; }
  { let v: u8 = 0xau8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x50aae7c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acc94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acc94); }
 let mut pc: u32 = 0x50acc94;
 loop { match pc {
 0x50acc94 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x50accb3; continue; }
  pc = 0x50accad; continue;
 }
 0x50accad => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x50accb6; continue; }
  pc = 0x50accb3; continue;
 }
 0x50accb3 => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x50accb6; continue;
 }
 0x50accb6 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x50accc2; continue; }
  pc = 0x50accbc; continue;
 }
 0x50accbc => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x50accd1; continue; }
  pc = 0x50accc2; continue;
 }
 0x50accc2 => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x50accd1; continue; }
  pc = 0x50accce; continue;
 }
 0x50accce => {
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  pc = 0x50accd1; continue;
 }
 0x50accd1 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x50accdd; continue; }
  pc = 0x50accd7; continue;
 }
 0x50accd7 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x50acce0; continue; }
  pc = 0x50accdd; continue;
 }
 0x50accdd => {
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  pc = 0x50acce0; continue;
 }
 0x50acce0 => {
  { cpu.and32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x50accee; continue; }
  pc = 0x50acce7; continue;
 }
 0x50acce7 => {
  { cpu.and32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x50acd00; continue; }
  pc = 0x50accee; continue;
 }
 0x50accee => {
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x50acd00; continue; }
  pc = 0x50accf4; continue;
 }
 0x50accf4 => {
  { cpu.sub32(cpu.r32(0x6cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x50acd00; continue; }
  pc = 0x50accfa; continue;
 }
 0x50accfa => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x40u32); }
  if cpu.zf { pc = 0x50acd0a; continue; }
  pc = 0x50acd00; continue;
 }
 0x50acd00 => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x50acd16; continue; }
  pc = 0x50acd0a; continue;
 }
 0x50acd0a => {
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x50acd16; continue; }
  pc = 0x50acd13; continue;
 }
 0x50acd13 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x50acd16; continue;
 }
 0x50acd16 => {
  { let v: u32 = cpu.eax; cpu.w32(0x514be84u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50acd2d; continue; }
  pc = 0x50acd28; continue;
 }
 0x50acd28 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x50acd2d; continue;
 }
 0x50acd2d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10000u32); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50acd41; continue; }
  pc = 0x50acd3f; continue;
 }
 0x50acd3f => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x50acd41; continue;
 }
 0x50acd41 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x52b4d58u32.wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x50acd8b; continue; }
  pc = 0x50acd49; continue;
 }
 0x50acd49 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514be98u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x50acd69; continue; }
  pc = 0x50acd65; continue;
 }
 0x50acd65 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x50acd69; continue;
 }
 0x50acd69 => {
  { cpu.and32(cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x50acd79; continue; }
  pc = 0x50acd75; continue;
 }
 0x50acd75 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x50acd79; continue;
 }
 0x50acd79 => {
  { let v: u32 = cpu.eax; cpu.w32(0x514be9cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bea0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])) | 0x20u32; cpu.w32(0x514be84u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50acd8b; continue;
 }
 0x50acd8b => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514be90u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514be94u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514be8cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  let t = cpu.r32(0x50ab068u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.flat_ds(0x50acdd1); cpu.push(0x50acdd9); cpu.call(t); if cpu.returned_to != 0x50acdd9 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x514be90u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x50acdf6); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acdf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acdf7); }
 let mut pc: u32 = 0x50acdf7;
 loop { match pc {
 0x50acdf7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x50ace14; continue; }
  pc = 0x50ace0e; continue;
 }
 0x50ace0e => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x50ace17; continue; }
  pc = 0x50ace14; continue;
 }
 0x50ace14 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x50ace17; continue;
 }
 0x50ace17 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x50ace23; continue; }
  pc = 0x50ace1d; continue;
 }
 0x50ace1d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x50ace26; continue; }
  pc = 0x50ace23; continue;
 }
 0x50ace23 => {
  { let v: u32 = cpu.eax | 0x20u32; cpu.eax = v; }
  pc = 0x50ace26; continue;
 }
 0x50ace26 => {
  { cpu.sub32(cpu.r32(0x68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x50ace2f; continue; }
  pc = 0x50ace2c; continue;
 }
 0x50ace2c => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  pc = 0x50ace2f; continue;
 }
 0x50ace2f => {
  { let v: u32 = cpu.eax; cpu.w32(0x514be84u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x20u32); }
  if !cpu.zf { pc = 0x50ace5d; continue; }
  pc = 0x50ace43; continue;
 }
 0x50ace43 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x50ace76; continue; }
  pc = 0x50ace53; continue;
 }
 0x50ace53 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x50ace76; continue;
 }
 0x50ace5d => {
  { let v: u32 = cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])) | 0x8u32; cpu.w32(0x514be84u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  pc = 0x50ace76; continue;
 }
 0x50ace76 => {
  { let v: u32 = cpu.r32(0x514be84u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  let t = cpu.r32(0x50aae68u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.flat_ds(0x50ace7b); cpu.push(0x50ace83); cpu.call(t); if cpu.returned_to != 0x50ace83 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x50ace89); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ace8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ace8a); }
 let mut pc: u32 = 0x50ace8a;
 loop { match pc {
 0x50ace8a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.df = false; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x50aceb5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aceb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aceb6); }
 let mut pc: u32 = 0x50aceb6;
 loop { match pc {
 0x50aceb6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0x514bea4u32; cpu.push(v); }
  { let v = 0x514be84u32; cpu.push(v); }
  cpu.flat_ds(0x50acec5); cpu.push(0x50aceca); f_0509c2f0(cpu); if cpu.returned_to != 0x50aceca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50aceb6u32; cpu.push(v); }
  { let v = 0x50aae68u32; cpu.push(v); }
  cpu.flat_ds(0x50aced7); cpu.push(0x50acedc); f_0509c2f0(cpu); if cpu.returned_to != 0x50acedc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50acee4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acee5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acee5); }
 let mut pc: u32 = 0x50acee5;
 loop { match pc {
 0x50acee5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x514a1c5u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x514a1c5u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x514a184u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x514a1c5u32, v); }
  pc = 0x50acf4b; continue;
 }
 0x50acf09 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x50acf13); f_050a629c(cpu); if cpu.returned_to != 0x50acf13 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50acf38; continue; }
  pc = 0x50acf19; continue;
 }
 0x50acf19 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x50acf23); f_050a629c(cpu); if cpu.returned_to != 0x50acf23 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50acf38; continue; }
  pc = 0x50acf29; continue;
 }
 0x50acf29 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x513b134u32; cpu.eax = v; }
  cpu.push(0x50acf38); f_050a65b2(cpu); if cpu.returned_to != 0x50acf38 { return; }
  pc = 0x50acf38; continue;
 }
 0x50acf38 => {
  { let v: u32 = cpu.r32(0x52b5030u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52b5030u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50acf4b; continue;
 }
 0x50acf4b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x50acf09; continue; }
  pc = 0x50acf52; continue;
 }
 0x50acf52 => {
  { let v: u32 = cpu.edx; cpu.w32(0x52b44fcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acf5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acf5c); }
 let mut pc: u32 = 0x50acf5c;
 loop { match pc {
 0x50acf5c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x50acf63); f_050acf6d(cpu); if cpu.returned_to != 0x50acf63 { return; }
  pc = 0x50adecd; continue;
 }
 0x50adecd => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x50adece; continue;
 }
 0x50adece => {
  { let v: u32 = cpu.r32(0x52b44fcu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50adee9; continue; }
  pc = 0x50aded8; continue;
 }
 0x50aded8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x50adee1); f_050a637f(cpu); if cpu.returned_to != 0x50adee1 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x52b44fcu32, v); }
  cpu.poll(); pc = 0x50adece; continue;
 }
 0x50adee9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acf6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acf6d); }
 let mut pc: u32 = 0x50acf6d;
 loop { match pc {
 0x50acf6d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x514a184u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52b5030u32); cpu.eax = v; }
  pc = 0x50acfba; continue;
 }
 0x50acf8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x50acfb2; continue; }
  pc = 0x50acf9f; continue;
 }
 0x50acf9f => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x50acfb2; continue; }
  pc = 0x50acfa5; continue;
 }
 0x50acfa5 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x50acfb8; continue; }
  pc = 0x50acfa9; continue;
 }
 0x50acfa9 => {
  { cpu.sub32(cpu.eax, 0x514a206u32); }
  if !cpu.cf { pc = 0x50acfb2; continue; }
  pc = 0x50acfb0; continue;
 }
 0x50acfb0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x50acfb2; continue;
 }
 0x50acfb2 => {
  cpu.push(0x50acfb7); f_050adf18(cpu); if cpu.returned_to != 0x50acfb7 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x50acfb8; continue;
 }
 0x50acfb8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x50acfba; continue;
 }
 0x50acfba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50acf8f; continue; }
  pc = 0x50acfbe; continue;
 }
 0x50acfbe => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050acfc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50acfc5); }
 let mut pc: u32 = 0x50acfc5;
 loop { match pc {
 0x50acfc5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x50acfd7); f_050aa77a(cpu); if cpu.returned_to != 0x50acfd7 { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x50ad00a; continue; }
  pc = 0x50acfdb; continue;
 }
 0x50acfdb => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.rcl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50ad00a; continue; }
  pc = 0x50acffd; continue;
 }
 0x50acffd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x50ad008); f_050aa6e2(cpu); if cpu.returned_to != 0x50ad008 { return; }
  pc = 0x50ad031; continue;
 }
 0x50ad00a => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50acffd; continue; }
  pc = 0x50ad021; continue;
 }
 0x50ad021 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x50ad02f; continue; }
  pc = 0x50ad025; continue;
 }
 0x50ad025 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x50ad02f); f_05097db1(cpu); if cpu.returned_to != 0x50ad02f { return; }
  pc = 0x50ad02f; continue;
 }
 0x50ad02f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x50ad031; continue;
 }
 0x50ad031 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad039(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad039); }
 let mut pc: u32 = 0x50ad039;
 loop { match pc {
 0x50ad039 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x514bec4u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ad05a; continue; }
  pc = 0x50ad055; continue;
 }
 0x50ad055 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ad061; continue; }
  pc = 0x50ad05a; continue;
 }
 0x50ad05a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x50ad063; continue;
 }
 0x50ad061 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50ad063; continue;
 }
 0x50ad063 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad09c; continue; }
  pc = 0x50ad067; continue;
 }
 0x50ad067 => {
  cpu.push(0x50ad06c); f_050ae13b(cpu); if cpu.returned_to != 0x50ad06c { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ad09c; continue; }
  pc = 0x50ad071; continue;
 }
 0x50ad071 => {
  { let v: u32 = 0x5096e13u32; cpu.edx = v; }
  { let v: u32 = 0x5096e08u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x5096e08u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50ad09c; continue;
 }
 0x50ad09c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad0a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad0a2); }
 let mut pc: u32 = 0x50ad0a2;
 loop { match pc {
 0x50ad0a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad0a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad0a3); }
 let mut pc: u32 = 0x50ad0a3;
 loop { match pc {
 0x50ad0a3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x514bea4u32); cpu.push(0x50ad0ac); cpu.call(t); if cpu.returned_to != 0x50ad0ac { return; }
  let t = cpu.r32(0x514bea8u32); cpu.push(0x50ad0b2); cpu.call(t); if cpu.returned_to != 0x50ad0b2 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50ad0b9); f_050ad0bb(cpu); if cpu.returned_to != 0x50ad0b9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad0d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad0d2); }
 let mut pc: u32 = 0x50ad0d2;
 loop { match pc {
 0x50ad0d2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50ad121; continue; }
  pc = 0x50ad0dc; continue;
 }
 0x50ad0dc => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x50ad121; continue; }
  pc = 0x50ad0e1; continue;
 }
 0x50ad0e1 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad121; continue; }
  pc = 0x50ad0f6; continue;
 }
 0x50ad0f6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad105; continue; }
  pc = 0x50ad100; continue;
 }
 0x50ad100 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x50ad105; continue;
 }
 0x50ad105 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x50ad125; continue; }
  pc = 0x50ad10b; continue;
 }
 0x50ad10b => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad105; continue; }
  pc = 0x50ad11e; continue;
 }
 0x50ad11e => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad121; continue;
 }
 0x50ad121 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50ad170; continue;
 }
 0x50ad125 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x50ad153; continue; }
  pc = 0x50ad135; continue;
 }
 0x50ad135 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad168; continue;
 }
 0x50ad153 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x50ad168; continue;
 }
 0x50ad168 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x50ad170; continue;
 }
 0x50ad170 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x50ad174); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad17a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad17a); }
 let mut pc: u32 = 0x50ad17a;
 loop { match pc {
 0x50ad175 => {
  pc = 0x50ad280; continue;
 }
 0x50ad17a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x50ad175; continue; }
  pc = 0x50ad184; continue;
 }
 0x50ad184 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x50ad175; continue; }
  pc = 0x50ad18f; continue;
 }
 0x50ad18f => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x50ad1bc; continue; }
  pc = 0x50ad19d; continue;
 }
 0x50ad19d => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x50ad1a5; continue; }
  pc = 0x50ad1a2; continue;
 }
 0x50ad1a2 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad1a5; continue;
 }
 0x50ad1a5 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad238; continue;
 }
 0x50ad1bc => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x50ad1d3; continue; }
  pc = 0x50ad1c5; continue;
 }
 0x50ad1c5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad238; continue; }
  pc = 0x50ad1ca; continue;
 }
 0x50ad1ca => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x50ad238; continue; }
  pc = 0x50ad1d1; continue;
 }
 0x50ad1d1 => {
  pc = 0x50ad1e2; continue;
 }
 0x50ad1d3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x50ad238; continue; }
  pc = 0x50ad1da; continue;
 }
 0x50ad1da => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad238; continue; }
  pc = 0x50ad1e2; continue;
 }
 0x50ad1e2 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x50ad217; continue; }
  pc = 0x50ad1f2; continue;
 }
 0x50ad1f2 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad1ff; continue; }
  pc = 0x50ad1fd; continue;
 }
 0x50ad1fd => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x50ad1ff; continue;
 }
 0x50ad1ff => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x50ad203; continue;
 }
 0x50ad203 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x50ad238; continue; }
  pc = 0x50ad20a; continue;
 }
 0x50ad20a => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x50ad217; continue; }
  pc = 0x50ad20f; continue;
 }
 0x50ad20f => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50ad203; continue; }
  pc = 0x50ad217; continue;
 }
 0x50ad217 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x50ad221; continue; }
  pc = 0x50ad21e; continue;
 }
 0x50ad21e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x50ad221; continue;
 }
 0x50ad221 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x50ad238; continue; }
  pc = 0x50ad225; continue;
 }
 0x50ad225 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x50ad238; continue; }
  pc = 0x50ad22c; continue;
 }
 0x50ad22c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x50ad238; continue; }
  pc = 0x50ad233; continue;
 }
 0x50ad233 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x50ad221; continue;
 }
 0x50ad238 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x50ad257; continue; }
  pc = 0x50ad247; continue;
 }
 0x50ad247 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x50ad253; continue; }
  pc = 0x50ad250; continue;
 }
 0x50ad250 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad253; continue;
 }
 0x50ad253 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x50ad268; continue;
 }
 0x50ad257 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad268; continue;
 }
 0x50ad268 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x50ad278; continue; }
  pc = 0x50ad270; continue;
 }
 0x50ad270 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x50ad278; continue; }
  pc = 0x50ad275; continue;
 }
 0x50ad275 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad278; continue;
 }
 0x50ad278 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x50ad280; continue; }
  pc = 0x50ad27d; continue;
 }
 0x50ad27d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ad280; continue;
 }
 0x50ad280 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x50ad284); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad285); }
 let mut pc: u32 = 0x50ad285;
 loop { match pc {
 0x50ad285 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bc10u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x50ad29b; continue; }
  pc = 0x50ad292; continue;
 }
 0x50ad292 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514bc10u32, v); }
  pc = 0x50ad29b; continue;
 }
 0x50ad29b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x52b5034u32)); }
  if !cpu.zf { pc = 0x50ad2ab; continue; }
  pc = 0x50ad2a3; continue;
 }
 0x50ad2a3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52b5034u32, v); }
  pc = 0x50ad2ab; continue;
 }
 0x50ad2ab => {
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x50ad2be; continue; }
  pc = 0x50ad2b5; continue;
 }
 0x50ad2b5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514bc0cu32, v); }
  pc = 0x50ad2be; continue;
 }
 0x50ad2be => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50ad2cb; continue; }
  pc = 0x50ad2c8; continue;
 }
 0x50ad2c8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x50ad2cb; continue;
 }
 0x50ad2cb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad2d2; continue; }
  pc = 0x50ad2cf; continue;
 }
 0x50ad2cf => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad2d2; continue;
 }
 0x50ad2d2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad2d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ad2d6); } r_050ad2d6(cpu, 0x50ad2d6); }
pub fn f_050ad4f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ad4f5); } r_050ad2d6(cpu, 0x50ad4f5); }
pub fn f_050ad5c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ad5c6); } r_050ad2d6(cpu, 0x50ad5c6); }
fn r_050ad2d6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50ad2d6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.edx = v; }
  pc = 0x50ad2e1; continue;
 }
 0x50ad2e1 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50ad5c0; continue; }
  pc = 0x50ad2e9; continue;
 }
 0x50ad2e9 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x50ad32d; continue; }
  pc = 0x50ad2f7; continue;
 }
 0x50ad2f7 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x50ad304); f_050ad285(cpu); if cpu.returned_to != 0x50ad304 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x50ad31d; continue; }
  pc = 0x50ad30d; continue;
 }
 0x50ad30d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x50ad2e1; continue;
 }
 0x50ad31d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x50ad2e1; continue;
 }
 0x50ad32d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x50ad2e1; continue;
 }
 0x50ad4f5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x50ad509); f_050ad2d6(cpu); if cpu.returned_to != 0x50ad509 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50ad524; continue; }
  pc = 0x50ad514; continue;
 }
 0x50ad514 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad524; continue;
 }
 0x50ad524 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad541; continue; }
  pc = 0x50ad528; continue;
 }
 0x50ad528 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x50ad5bd; continue;
 }
 0x50ad541 => {
  { cpu.and16(cpu.r16(0x52b54eeu32), 0xfff0u16); }
  if cpu.zf { pc = 0x50ad556; continue; }
  pc = 0x50ad54c; continue;
 }
 0x50ad54c => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x52b54ecu32, v); }
  pc = 0x50ad556; continue;
 }
 0x50ad556 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad5bb; continue; }
  pc = 0x50ad55f; continue;
 }
 0x50ad55f => {
  { let v: u32 = cpu.r32(0x52b54ecu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50ad5bb; continue; }
  pc = 0x50ad579; continue;
 }
 0x50ad579 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x50ad5bb; continue; }
  pc = 0x50ad59d; continue;
 }
 0x50ad59d => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad5bd; continue;
 }
 0x50ad5bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50ad5bd; continue;
 }
 0x50ad5bd => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x50ad5c0; continue;
 }
 0x50ad5c0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad5c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514bee4u32), 0x0u32); }
  if !cpu.zf { pc = 0x50ad5d9; continue; }
  pc = 0x50ad5d5; continue;
 }
 0x50ad5d5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x50ad5bd; continue;
 }
 0x50ad5d9 => {
  { cpu.sub32(cpu.r32(0x514ace8u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x50ad5d5; continue; }
  pc = 0x50ad5e2; continue;
 }
 0x50ad5e2 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x50ad5e9); f_050ad7c2(cpu); if cpu.returned_to != 0x50ad5e9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x50ad5bd; continue; }
  pc = 0x50ad5ed; continue;
 }
 0x50ad5ed => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ad60e; continue; }
  pc = 0x50ad5f9; continue;
 }
 0x50ad5f9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x50ad601); f_050ad4f5(cpu); if cpu.returned_to != 0x50ad601 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad629; continue; }
  pc = 0x50ad607; continue;
 }
 0x50ad607 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x50ad629; continue;
 }
 0x50ad60e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x80004800u32; cpu.eax = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x50ad629; continue;
 }
 0x50ad629 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x50ad5d5; continue; }
  pc = 0x50ad62d; continue;
 }
 0x50ad62d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50ad5d5; continue; }
  pc = 0x50ad63a; continue;
 }
 0x50ad63a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x50ad5d5; continue; }
  pc = 0x50ad642; continue;
 }
 0x50ad642 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50ad64b); f_050ad431(cpu); if cpu.returned_to != 0x50ad64b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x50ad66c); f_050a637f(cpu); if cpu.returned_to != 0x50ad66c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x50ad5bd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad332(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad332); }
 let mut pc: u32 = 0x50ad332;
 loop { match pc {
 0x50ad332 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x50ad340); f_050ad2d6(cpu); if cpu.returned_to != 0x50ad340 { return; }
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.edx = v; }
  pc = 0x50ad421; continue;
 }
 0x50ad34b => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad429; continue; }
  pc = 0x50ad353; continue;
 }
 0x50ad353 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x50ad41e; continue; }
  pc = 0x50ad35e; continue;
 }
 0x50ad35e => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x50ad41e; continue; }
  pc = 0x50ad368; continue;
 }
 0x50ad368 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50ad36f); f_050ad285(cpu); if cpu.returned_to != 0x50ad36f { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x50ad429; continue; }
  pc = 0x50ad37c; continue;
 }
 0x50ad37c => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50ad3c0; continue; }
  pc = 0x50ad3b0; continue;
 }
 0x50ad3b0 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad3c0; continue;
 }
 0x50ad3c0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad42b; continue; }
  pc = 0x50ad3c4; continue;
 }
 0x50ad3c4 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x50ad3db); f_050ad431(cpu); if cpu.returned_to != 0x50ad3db { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x50ad417; continue; }
  pc = 0x50ad3f0; continue;
 }
 0x50ad3f0 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u16 = (cpu.esi as u16) | 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x50ad415); f_050a637f(cpu); if cpu.returned_to != 0x50ad415 { return; }
  pc = 0x50ad41a; continue;
 }
 0x50ad417 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad41a; continue;
 }
 0x50ad41a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x50ad42b; continue;
 }
 0x50ad41e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x50ad421; continue;
 }
 0x50ad421 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad34b; continue; }
  pc = 0x50ad429; continue;
 }
 0x50ad429 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50ad42b; continue;
 }
 0x50ad42b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad431(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad431); }
 let mut pc: u32 = 0x50ad431;
 loop { match pc {
 0x50ad431 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x50ad447; continue;
 }
 0x50ad43e => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x50ad44b; continue; }
  pc = 0x50ad442; continue;
 }
 0x50ad442 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x50ad447; continue;
 }
 0x50ad447 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad43e; continue; }
  pc = 0x50ad44b; continue;
 }
 0x50ad44b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50ad45a; continue; }
  pc = 0x50ad455; continue;
 }
 0x50ad455 => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50ad460; continue;
 }
 0x50ad45a => {
  { let v: u32 = cpu.edx; cpu.w32(0x514bc0cu32, v); }
  pc = 0x50ad460; continue;
 }
 0x50ad460 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad467; continue; }
  pc = 0x50ad464; continue;
 }
 0x50ad464 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ad467; continue;
 }
 0x50ad467 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xffffffecu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x2cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad4a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad4a5); }
 let mut pc: u32 = 0x50ad4a5;
 loop { match pc {
 0x50ad4a5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50ad4f0; continue; }
  pc = 0x50ad4b1; continue;
 }
 0x50ad4b1 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ad4e3; continue; }
  pc = 0x50ad4c7; continue;
 }
 0x50ad4c7 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ad4e3; continue; }
  pc = 0x50ad4cc; continue;
 }
 0x50ad4cc => {
  { cpu.sub8(cpu.r8(0x514ad18u32), 0x0u8); }
  if !cpu.zf { pc = 0x50ad4e3; continue; }
  pc = 0x50ad4d5; continue;
 }
 0x50ad4d5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x50ad4da, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514ace8u32, v); }
  pc = 0x50ad4e3; continue;
 }
 0x50ad4e3 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x514ace8u32)); }
  if !cpu.zf { pc = 0x50ad4f0; continue; }
  pc = 0x50ad4eb; continue;
 }
 0x50ad4eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad4f0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad676(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad676); }
 let mut pc: u32 = 0x50ad676;
 loop { match pc {
 0x50ad676 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ad691; continue; }
  pc = 0x50ad688; continue;
 }
 0x50ad688 => {
  { cpu.sub8(cpu.r8(0x514ad17u32), 0x0u8); }
  if cpu.zf { pc = 0x50ad69d; continue; }
  pc = 0x50ad691; continue;
 }
 0x50ad691 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x50ad6aa; continue; }
  pc = 0x50ad69d; continue;
 }
 0x50ad69d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x50ad6a5); f_050ad5c6(cpu); if cpu.returned_to != 0x50ad6a5 { return; }
  pc = 0x50ad7b9; continue;
 }
 0x50ad6aa => {
  { cpu.sub32(cpu.r32(0x514bee4u32), 0x0u32); }
  if !cpu.zf { pc = 0x50ad6ba; continue; }
  pc = 0x50ad6b3; continue;
 }
 0x50ad6b3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50ad7b9; continue;
 }
 0x50ad6ba => {
  { cpu.sub32(cpu.r32(0x514ace8u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad6c3; continue;
 }
 0x50ad6c3 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x50ad6ca); f_050ad7c2(cpu); if cpu.returned_to != 0x50ad6ca { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ad7b9; continue; }
  pc = 0x50ad6d2; continue;
 }
 0x50ad6d2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ad6fa; continue; }
  pc = 0x50ad6de; continue;
 }
 0x50ad6de => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ad6fa; continue; }
  pc = 0x50ad6e3; continue;
 }
 0x50ad6e3 => {
  { cpu.sub8(cpu.r8(0x514ad18u32), 0x0u8); }
  if !cpu.zf { pc = 0x50ad6fa; continue; }
  pc = 0x50ad6ec; continue;
 }
 0x50ad6ec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x50ad6f1, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514ace8u32, v); }
  pc = 0x50ad6fa; continue;
 }
 0x50ad6fa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x514ace8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x50ad70e; continue; }
  pc = 0x50ad709; continue;
 }
 0x50ad709 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x50ad70e; continue;
 }
 0x50ad70e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50ad715); f_050ae6cb(cpu); if cpu.returned_to != 0x50ad715 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad71a; continue;
 }
 0x50ad71a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad721; continue;
 }
 0x50ad721 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad725; continue;
 }
 0x50ad725 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad731; continue;
 }
 0x50ad731 => {
  { let v: u32 = cpu.r32(0x514bc0cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x50ad74e; continue;
 }
 0x50ad73c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x50ad752; continue; }
  pc = 0x50ad743; continue;
 }
 0x50ad743 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ad74b; continue; }
  pc = 0x50ad747; continue;
 }
 0x50ad747 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x50ad752; continue; }
  pc = 0x50ad74b; continue;
 }
 0x50ad74b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x50ad74e; continue;
 }
 0x50ad74e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad73c; continue; }
  pc = 0x50ad752; continue;
 }
 0x50ad752 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50ad77c; continue; }
  pc = 0x50ad756; continue;
 }
 0x50ad756 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x50ad77c; continue; }
  pc = 0x50ad761; continue;
 }
 0x50ad761 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x50ad79b; continue;
 }
 0x50ad77c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x50ad6b3; continue; }
  pc = 0x50ad786; continue;
 }
 0x50ad786 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x50ad794); f_050ad431(cpu); if cpu.returned_to != 0x50ad794 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x50ad79b; continue;
 }
 0x50ad79b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x50ad7b4); f_050a637f(cpu); if cpu.returned_to != 0x50ad7b4 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x50ad7b9; continue;
 }
 0x50ad7b9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad7c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad7c2); }
 let mut pc: u32 = 0x50ad7c2;
 loop { match pc {
 0x50ad7c2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x50ad7d9; continue; }
  pc = 0x50ad7d3; continue;
 }
 0x50ad7d3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad7d9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ad7ee; continue; }
  pc = 0x50ad7e5; continue;
 }
 0x50ad7e5 => {
  { cpu.sub8(cpu.r8(0x514ad17u32), 0x0u8); }
  if cpu.zf { pc = 0x50ad7fa; continue; }
  pc = 0x50ad7ee; continue;
 }
 0x50ad7ee => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x50ad7ff; continue; }
  pc = 0x50ad7fa; continue;
 }
 0x50ad7fa => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x50ad806; continue;
 }
 0x50ad7ff => {
  cpu.push(0x50ad804); f_050ad4a5(cpu); if cpu.returned_to != 0x50ad804 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x50ad806; continue;
 }
 0x50ad806 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x50ad7d3; continue; }
  pc = 0x50ad80f; continue;
 }
 0x50ad80f => {
  { let v: u32 = cpu.r32(0x514bec4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x50ad81e; continue; }
  pc = 0x50ad819; continue;
 }
 0x50ad819 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50ad81e; continue;
 }
 0x50ad81e => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x50ad7d3; continue; }
  pc = 0x50ad82a; continue;
 }
 0x50ad82a => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad83f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad83f); }
 let mut pc: u32 = 0x50ad83f;
 loop { match pc {
 0x50ad83f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad842(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad842); }
 let mut pc: u32 = 0x50ad842;
 loop { match pc {
 0x50ad842 => {
  { cpu.sub8(cpu.r8(0x514beb8u32), 0x0u8); }
  if cpu.zf { pc = 0x50ad865; continue; }
  pc = 0x50ad84b; continue;
 }
 0x50ad84b => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x50ad853, "Int3");
  pc = 0x50ad85c; continue;
 }
 0x50ad85c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad865 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad86e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ad86e); } r_050ad86e(cpu, 0x50ad86e); }
pub fn f_050ad911(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ad911); } r_050ad86e(cpu, 0x50ad911); }
fn r_050ad86e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50ad86e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50ad887; continue; }
  pc = 0x50ad881; continue;
 }
 0x50ad881 => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x50ad889; continue;
 }
 0x50ad887 => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50ad889; continue;
 }
 0x50ad889 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x50ad8ac; continue; }
  pc = 0x50ad897; continue;
 }
 0x50ad897 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x50ad8ac; continue; }
  pc = 0x50ad8a5; continue;
 }
 0x50ad8a5 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x50ad8ac; continue;
 }
 0x50ad8ac => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50ad8bc; continue; }
  pc = 0x50ad8b0; continue;
 }
 0x50ad8b0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x50ad8bc; continue; }
  pc = 0x50ad8ba; continue;
 }
 0x50ad8ba => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x50ad8bc; continue;
 }
 0x50ad8bc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ad908; continue; }
  pc = 0x50ad8c4; continue;
 }
 0x50ad8c4 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ad905; continue; }
  pc = 0x50ad8c8; continue;
 }
 0x50ad8c8 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50ad8ca; continue;
 }
 0x50ad8ca => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ad8d5; continue; }
  pc = 0x50ad8cf; continue;
 }
 0x50ad8cf => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x50ad8ca; continue;
 }
 0x50ad8d5 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad8d5; continue; }
  pc = 0x50ad903; continue;
 }
 0x50ad903 => {
  pc = 0x50ad908; continue;
 }
 0x50ad905 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x50ad908; continue;
 }
 0x50ad908 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x50ad90f; continue;
 }
 0x50ad90f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad911 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x50ad926; continue; }
  pc = 0x50ad918; continue;
 }
 0x50ad918 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ad926; continue; }
  pc = 0x50ad91c; continue;
 }
 0x50ad91c => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ad926 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ad937; continue; }
  pc = 0x50ad92a; continue;
 }
 0x50ad92a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x50ad937; continue; }
  pc = 0x50ad930; continue;
 }
 0x50ad930 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50ad926; continue;
 }
 0x50ad937 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ad944; continue; }
  pc = 0x50ad93b; continue;
 }
 0x50ad93b => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50ad93b; continue; }
  pc = 0x50ad944; continue;
 }
 0x50ad944 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ad958; continue; }
  pc = 0x50ad94c; continue;
 }
 0x50ad94c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x50ad958; continue; }
  pc = 0x50ad950; continue;
 }
 0x50ad950 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50ad94c; continue; }
  pc = 0x50ad958; continue;
 }
 0x50ad958 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50ad90f; continue; }
  pc = 0x50ad95c; continue;
 }
 0x50ad95c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x50ad969; continue; }
  pc = 0x50ad962; continue;
 }
 0x50ad962 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50ad95c; continue; }
  pc = 0x50ad969; continue;
 }
 0x50ad969 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x50ad90f; continue; }
  pc = 0x50ad96d; continue;
 }
 0x50ad96d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50ad969; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad974); }
 let mut pc: u32 = 0x50ad974;
 loop { match pc {
 0x50ad974 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x50ad9dd; continue; }
  pc = 0x50ad985; continue;
 }
 0x50ad985 => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x50ad994; continue;
 }
 0x50ad994 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x50ad994; continue; }
  pc = 0x50ad9a3; continue;
 }
 0x50ad9a3 => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x50ad9b3; continue; }
  pc = 0x50ad9ac; continue;
 }
 0x50ad9ac => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x50ad9d1; continue;
 }
 0x50ad9b3 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x50ad9bd; continue; }
  pc = 0x50ad9b8; continue;
 }
 0x50ad9b8 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x50ad9d1; continue; }
  pc = 0x50ad9bd; continue;
 }
 0x50ad9bd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x50ad9d1; continue; }
  pc = 0x50ad9c7; continue;
 }
 0x50ad9c7 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x50ad9d1; continue;
 }
 0x50ad9d1 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x50ad9dd); f_05097b90(cpu); if cpu.returned_to != 0x50ad9dd { return; }
  pc = 0x50ad9dd; continue;
 }
 0x50ad9dd => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ad9e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ad9e5); }
 let mut pc: u32 = 0x50ad9e5;
 loop { match pc {
 0x50ad9e5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x50ada05); f_050ae86a(cpu); if cpu.returned_to != 0x50ada05 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ada25; continue; }
  pc = 0x50ada09; continue;
 }
 0x50ada09 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x50ada18; continue;
 }
 0x50ada10 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x50ada16; continue; }
  pc = 0x50ada14; continue;
 }
 0x50ada14 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50ada16; continue;
 }
 0x50ada16 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x50ada18; continue;
 }
 0x50ada18 => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x50ada10; continue; }
  pc = 0x50ada20; continue;
 }
 0x50ada20 => {
  pc = 0x50adc02; continue;
 }
 0x50ada25 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x50ada9a; continue; }
  pc = 0x50ada33; continue;
 }
 0x50ada33 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50ada4c; continue; }
  pc = 0x50ada48; continue;
 }
 0x50ada48 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ada93; continue; }
  pc = 0x50ada4c; continue;
 }
 0x50ada4c => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x50ada54); f_050ae943(cpu); if cpu.returned_to != 0x50ada54 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x50ada64); f_050ae96b(cpu); if cpu.returned_to != 0x50ada64 { return; }
  cpu.push(0x50ada69); f_05086172(cpu); if cpu.returned_to != 0x50ada69 { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x50ada78; continue; }
  pc = 0x50ada74; continue;
 }
 0x50ada74 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ada7c; continue; }
  pc = 0x50ada78; continue;
 }
 0x50ada78 => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50ada9a; continue;
 }
 0x50ada7c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ada89; continue; }
  pc = 0x50ada86; continue;
 }
 0x50ada86 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50ada89; continue;
 }
 0x50ada89 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50ada93; continue; }
  pc = 0x50ada92; continue;
 }
 0x50ada92 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x50ada93; continue;
 }
 0x50ada93 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50ada9a; continue;
 }
 0x50ada9a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x50adac9; continue; }
  pc = 0x50adaa3; continue;
 }
 0x50adaa3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50adab5; continue; }
  pc = 0x50adaae; continue;
 }
 0x50adaae => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50adac9; continue; }
  pc = 0x50adab5; continue;
 }
 0x50adab5 => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x50adac4); f_05097b90(cpu); if cpu.returned_to != 0x50adac4 { return; }
  pc = 0x50adc16; continue;
 }
 0x50adac9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x50adae6; continue; }
  pc = 0x50adad5; continue;
 }
 0x50adad5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50adae1; continue; }
  pc = 0x50adadc; continue;
 }
 0x50adadc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x50adae6; continue;
 }
 0x50adae1 => {
  if (cpu.sf == cpu.of) { pc = 0x50adae6; continue; }
  pc = 0x50adae3; continue;
 }
 0x50adae3 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x50adae6; continue;
 }
 0x50adae6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50adaff; continue; }
  pc = 0x50adafb; continue;
 }
 0x50adafb => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50adb20; continue; }
  pc = 0x50adaff; continue;
 }
 0x50adaff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x50adb20; continue; }
  pc = 0x50adb08; continue;
 }
 0x50adb08 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50adb20; continue; }
  pc = 0x50adb0f; continue;
 }
 0x50adb0f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x50adb1d); f_05097e21(cpu); if cpu.returned_to != 0x50adb1d { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x50adb20; continue;
 }
 0x50adb20 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x50adb3f); f_050ae9a7(cpu); if cpu.returned_to != 0x50adb3f { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x50adb58; continue; }
  pc = 0x50adb4d; continue;
 }
 0x50adb4d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x50adb65; continue;
 }
 0x50adb58 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x50adb65; continue; }
  pc = 0x50adb5c; continue;
 }
 0x50adb5c => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x50adb65; continue;
 }
 0x50adb65 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x50adbd7; continue; }
  pc = 0x50adb71; continue;
 }
 0x50adb71 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x50adc00; continue; }
  pc = 0x50adb80; continue;
 }
 0x50adb80 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x50adb8d); f_050ad911(cpu); if cpu.returned_to != 0x50adb8d { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x50adba9; continue; }
  pc = 0x50adb9e; continue;
 }
 0x50adb9e => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x50adba9; continue; }
  pc = 0x50adba2; continue;
 }
 0x50adba2 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50adba9; continue;
 }
 0x50adba9 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50adbbb; continue; }
  pc = 0x50adbb7; continue;
 }
 0x50adbb7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50adbc1; continue; }
  pc = 0x50adbbb; continue;
 }
 0x50adbbb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50adbc1; continue;
 }
 0x50adbc1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x50adbd3); f_050ad86e(cpu); if cpu.returned_to != 0x50adbd3 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x50adc02; continue;
 }
 0x50adbd7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x50adc00; continue; }
  pc = 0x50adbeb; continue;
 }
 0x50adbeb => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x50adbfe); f_050ad911(cpu); if cpu.returned_to != 0x50adbfe { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x50adc00; continue;
 }
 0x50adc00 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x50adc02; continue;
 }
 0x50adc02 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50adab5; continue; }
  pc = 0x50adc0a; continue;
 }
 0x50adc0a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x50adc16); f_050ad974(cpu); if cpu.returned_to != 0x50adc16 { return; }
  pc = 0x50adc16; continue;
 }
 0x50adc16 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adc1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adc1f); }
 let mut pc: u32 = 0x50adc1f;
 loop { match pc {
 0x50adc1f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x50adc4c; continue; }
  pc = 0x50adc2a; continue;
 }
 0x50adc2a => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x50adc32); f_050aa753(cpu); if cpu.returned_to != 0x50adc32 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50adc4c; continue; }
  pc = 0x50adc36; continue;
 }
 0x50adc36 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x50adc4c; continue; }
  pc = 0x50adc44; continue;
 }
 0x50adc44 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x50adc4c; continue;
 }
 0x50adc4c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adc50); }
 let mut pc: u32 = 0x50adc50;
 loop { match pc {
 0x50a662d => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x50a6635); cpu.push(0x50a663a); f_050a667a(cpu); if cpu.returned_to != 0x50a663a { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x22u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x50a6679); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50adc50 => {
  cpu.poll(); pc = 0x50a662d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adc55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adc55); }
 let mut pc: u32 = 0x50adc55;
 loop { match pc {
 0x50adc55 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x50adc7d; continue; }
  pc = 0x50adc64; continue;
 }
 0x50adc64 => {
  { cpu.sub8(cpu.r8(0x514ad18u32), 0x0u8); }
  if cpu.zf { pc = 0x50adc7d; continue; }
  pc = 0x50adc6d; continue;
 }
 0x50adc6d => {
  { let v: u16 = cpu.r16(0x514a162u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50adc73; continue;
 }
 0x50adc73 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x52b50e8u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50adc7d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50adc8e; continue; }
  pc = 0x50adc89; continue;
 }
 0x50adc89 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50adc98; continue; }
  pc = 0x50adc8e; continue;
 }
 0x50adc8e => {
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x50adca3; continue; }
  pc = 0x50adc98; continue;
 }
 0x50adc98 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x34u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x50adc73; continue;
 }
 0x50adca3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50adcb5; continue; }
  pc = 0x50adcac; continue;
 }
 0x50adcac => {
  { cpu.sub8(cpu.r8(0x514ad17u32), 0x0u8); }
  if cpu.zf { pc = 0x50adcc1; continue; }
  pc = 0x50adcb5; continue;
 }
 0x50adcb5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x50adcc6; continue; }
  pc = 0x50adcc1; continue;
 }
 0x50adcc1 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x50adc73; continue;
 }
 0x50adcc6 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50add3c; continue; }
  pc = 0x50adccb; continue;
 }
 0x50adccb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad17u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50add3c; continue; }
  pc = 0x50adcd7; continue;
 }
 0x50adcd7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x50add03; continue; }
  pc = 0x50adcf4; continue;
 }
 0x50adcf4 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x513b164u32; cpu.eax = v; }
  cpu.push(0x50add03); f_050a65b2(cpu); if cpu.returned_to != 0x50add03 { return; }
  pc = 0x50add03; continue;
 }
 0x50add03 => {
  { let v: u32 = 0xfffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x52b50e8u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50add45; continue; }
  pc = 0x50add29; continue;
 }
 0x50add29 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x513b18au32; cpu.eax = v; }
  cpu.push(0x50add38); f_050a65b2(cpu); if cpu.returned_to != 0x50add38 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50add3c => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x52b50e8u32, v); }
  pc = 0x50add45; continue;
 }
 0x50add45 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050add49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50add49); }
 let mut pc: u32 = 0x50add49;
 loop { match pc {
 0x50add49 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50add6f; continue; }
  pc = 0x50add56; continue;
 }
 0x50add56 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad17u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50add6f; continue; }
  pc = 0x50add62; continue;
 }
 0x50add62 => {
  { let v: u16 = cpu.r16(0x52b50e8u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x50add6f; continue;
 }
 0x50add6f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050add77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50add77); }
 let mut pc: u32 = 0x50add77;
 loop { match pc {
 0x50add77 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x50add88); f_050aeb89(cpu); if cpu.returned_to != 0x50add88 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x513b464u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x50add9a); f_050aeba8(cpu); if cpu.returned_to != 0x50add9a { return; }
  { let v: u32 = 0x513b239u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x50adda6); f_050aeba8(cpu); if cpu.returned_to != 0x50adda6 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x50addaf); f_050aeba8(cpu); if cpu.returned_to != 0x50addaf { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x50addbb); f_050aa843(cpu); if cpu.returned_to != 0x50addbb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050addbf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50addbf); }
 let mut pc: u32 = 0x50addbf;
 loop { match pc {
 0x50addbf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x514bec0u32); cpu.push(0x50addc9); cpu.call(t); if cpu.returned_to != 0x50addc9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50adde8; continue; }
  pc = 0x50addcd; continue;
 }
 0x50addcd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x50addd7); f_050add77(cpu); if cpu.returned_to != 0x50addd7 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x50adde3; continue; }
  pc = 0x50adddc; continue;
 }
 0x50adddc => {
  cpu.push(0x50adde1); f_05097dac(cpu); if cpu.returned_to != 0x50adde1 { return; }
  pc = 0x50adde8; continue;
 }
 0x50adde3 => {
  cpu.push(0x50adde8); f_05097dbd(cpu); if cpu.returned_to != 0x50adde8 { return; }
  pc = 0x50adde8; continue;
 }
 0x50adde8 => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050addee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50addee); }
 let mut pc: u32 = 0x50addee;
 loop { match pc {
 0x50addee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52b44fcu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x50ade16; continue; }
  pc = 0x50addfd; continue;
 }
 0x50addfd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52b44fcu32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x50ade5d; continue;
 }
 0x50ade16 => {
  { let v: u32 = 0x514a184u32; cpu.ecx = v; }
  pc = 0x50ade3d; continue;
 }
 0x50ade1d => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x50ade3a; continue; }
  pc = 0x50ade23; continue;
 }
 0x50ade23 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x50ade2d); f_050a629c(cpu); if cpu.returned_to != 0x50ade2d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ade82; continue; }
  pc = 0x50ade33; continue;
 }
 0x50ade33 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x50ade5d; continue;
 }
 0x50ade3a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x50ade3d; continue;
 }
 0x50ade3d => {
  { cpu.sub32(cpu.ecx, 0x514a38cu32); }
  if cpu.cf { cpu.poll(); pc = 0x50ade1d; continue; }
  pc = 0x50ade45; continue;
 }
 0x50ade45 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x50ade54); f_050a629c(cpu); if cpu.returned_to != 0x50ade54 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ade82; continue; }
  pc = 0x50ade5a; continue;
 }
 0x50ade5a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x50ade5d; continue;
 }
 0x50ade5d => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x50ade6b); f_05097b90(cpu); if cpu.returned_to != 0x50ade6b { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x52b5030u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x52b5030u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x50ade8e; continue;
 }
 0x50ade82 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x50ade8c); f_05097db1(cpu); if cpu.returned_to != 0x50ade8c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x50ade8e; continue;
 }
 0x50ade8e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ade94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ade94); }
 let mut pc: u32 = 0x50ade94;
 loop { match pc {
 0x50ade94 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x52b5030u32; cpu.edx = v; }
  pc = 0x50ade9e; continue;
 }
 0x50ade9e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50adec9; continue; }
  pc = 0x50adea4; continue;
 }
 0x50adea4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x50adeaf; continue; }
  pc = 0x50adeab; continue;
 }
 0x50adeab => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x50ade9e; continue;
 }
 0x50adeaf => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x52b44fcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52b44fcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x50adec9; continue;
 }
 0x50adec9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adeeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adeeb); }
 let mut pc: u32 = 0x50adeeb;
 loop { match pc {
 0x50adeeb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x52b5030u32); cpu.eax = v; }
  pc = 0x50adef4; continue;
 }
 0x50adef4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x50adf00; continue; }
  pc = 0x50adef8; continue;
 }
 0x50adef8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50adf00 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x50adf09; continue; }
  pc = 0x50adf05; continue;
 }
 0x50adf05 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x50adef4; continue;
 }
 0x50adf09 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x50adf15); f_050adf18(cpu); if cpu.returned_to != 0x50adf15 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adf18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adf18); }
 let mut pc: u32 = 0x50adf18;
 loop { match pc {
 0x50adf18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x50adf20); f_050adf9e(cpu); if cpu.returned_to != 0x50adf20 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x50adf29); f_050ade94(cpu); if cpu.returned_to != 0x50adf29 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adf2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adf2d); }
 let mut pc: u32 = 0x50adf2d;
 loop { match pc {
 0x50adf2d => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50adf38; continue; }
  pc = 0x50adf35; continue;
 }
 0x50adf35 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x50adf38; continue;
 }
 0x50adf38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adf39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adf39); }
 let mut pc: u32 = 0x50adf39;
 loop { match pc {
 0x50adf39 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x50adf46); f_050aec23(cpu); if cpu.returned_to != 0x50adf46 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x50adf50; continue;
 }
 0x50adf50 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x50adf5a); f_050adf2d(cpu); if cpu.returned_to != 0x50adf5a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50adf50; continue; }
  pc = 0x50adf64; continue;
 }
 0x50adf64 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x50adf75); f_050adf2d(cpu); if cpu.returned_to != 0x50adf75 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x50adf82); f_050adf2d(cpu); if cpu.returned_to != 0x50adf82 { return; }
  { let v: u8 = 0x2eu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x74u8; cpu.w8(0x9u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x6du8; cpu.w8(0xau32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x70u8; cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050adf9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50adf9e); }
 let mut pc: u32 = 0x50adf9e;
 loop { match pc {
 0x50adf9e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x50adfba; continue; }
  pc = 0x50adfb0; continue;
 }
 0x50adfb0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x50ae025; continue;
 }
 0x50adfba => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x50adfcd; continue; }
  pc = 0x50adfc4; continue;
 }
 0x50adfc4 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x50adfcb); f_050a5f0a(cpu); if cpu.returned_to != 0x50adfcb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x50adfcd; continue;
 }
 0x50adfcd => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x50adfd4); f_050aec29(cpu); if cpu.returned_to != 0x50adfd4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x50adfe7; continue; }
  pc = 0x50adfd9; continue;
 }
 0x50adfd9 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x50adfe7); f_0509a80d(cpu); if cpu.returned_to != 0x50adfe7 { return; }
  pc = 0x50adfe7; continue;
 }
 0x50adfe7 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x50adff5; continue; }
  pc = 0x50adfeb; continue;
 }
 0x50adfeb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x50adff3); f_050aa7e4(cpu); if cpu.returned_to != 0x50adff3 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x50adff5; continue;
 }
 0x50adff5 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x50ae00a; continue; }
  pc = 0x50adffb; continue;
 }
 0x50adffb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x50ae003); f_050a637f(cpu); if cpu.returned_to != 0x50ae003 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x50ae00a; continue;
 }
 0x50ae00a => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x50ae023; continue; }
  pc = 0x50ae010; continue;
 }
 0x50ae010 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x50ae01c); f_050adf39(cpu); if cpu.returned_to != 0x50ae01c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x50ae023); f_0509a850(cpu); if cpu.returned_to != 0x50ae023 { return; }
  pc = 0x50ae023; continue;
 }
 0x50ae023 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x50ae025; continue;
 }
 0x50ae025 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae02e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ae02e); } r_050ae02e(cpu, 0x50ae02e); }
pub fn f_050ae0c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ae0c3); } r_050ae02e(cpu, 0x50ae0c3); }
fn r_050ae02e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50ae02e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x50ae038); cpu.push(0x50ae03d); f_05096e08(cpu); if cpu.returned_to != 0x50ae03d { return; }
  { let v: u32 = 0x52b60dcu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514acf8u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x514ad10u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x52b50ecu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514acf8u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x50ae092; continue; }
  pc = 0x50ae071; continue;
 }
 0x50ae071 => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x50ad0a2u32); }
  if cpu.zf { pc = 0x50ae092; continue; }
  pc = 0x50ae07d; continue;
 }
 0x50ae07d => {
  { cpu.sub32(cpu.eax, 0x50ae385u32); }
  if cpu.zf { pc = 0x50ae092; continue; }
  pc = 0x50ae084; continue;
 }
 0x50ae084 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x50ae08b); cpu.push(0x50ae090); f_050aed5f(cpu); if cpu.returned_to != 0x50ae090 { return; }
  pc = 0x50ae0a3; continue;
 }
 0x50ae092 => {
  { let v: u16 = cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x50ae09e; continue;
 }
 0x50ae09e => {
  cpu.flat_ds(0x50ae09e); cpu.push(0x50ae0a3); f_05098b3a(cpu); if cpu.returned_to != 0x50ae0a3 { return; }
  pc = 0x50ae0a3; continue;
 }
 0x50ae0a3 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x514acf8u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x50ae0c2); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x50ae0c3 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x50ae0cd); cpu.push(0x50ae0d2); f_05096e08(cpu); if cpu.returned_to != 0x50ae0d2 { return; }
  { let v: u32 = 0x52b60dcu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514acf8u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x514ad10u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x52b50ecu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514acf8u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x50ae12a; continue; }
  pc = 0x50ae106; continue;
 }
 0x50ae106 => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x50ad0a2u32); }
  if cpu.zf { pc = 0x50ae12a; continue; }
  pc = 0x50ae112; continue;
 }
 0x50ae112 => {
  { cpu.sub32(cpu.eax, 0x50ae1cfu32); }
  if cpu.zf { pc = 0x50ae12a; continue; }
  pc = 0x50ae119; continue;
 }
 0x50ae119 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x50ae120); cpu.push(0x50ae125); f_050aed5f(cpu); if cpu.returned_to != 0x50ae125 { return; }
  cpu.poll(); pc = 0x50ae0a3; continue;
 }
 0x50ae12a => {
  { let v: u16 = cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x50ae09e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae13b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae13b); }
 let mut pc: u32 = 0x50ae13b;
 loop { match pc {
 0x50ae13b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x514bee0u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x50ae1c2; continue; }
  pc = 0x50ae14f; continue;
 }
 0x50ae14f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x50ae166; continue; }
  pc = 0x50ae15b; continue;
 }
 0x50ae15b => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x50ae1b8; continue; }
  pc = 0x50ae166; continue;
 }
 0x50ae166 => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x400u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esp)) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50ae1b8; continue; }
  pc = 0x50ae1ac; continue;
 }
 0x50ae1ac => {
  { let v: u32 = 0x1u32; cpu.w32(0x514bee0u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae1c2; continue;
 }
 0x50ae1b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x514bee0u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae1c2; continue;
 }
 0x50ae1c2 => {
  { let v: u32 = cpu.r32(0x514bee0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae1ce); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae1cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae1cf); }
 let mut pc: u32 = 0x50ae1cf;
 loop { match pc {
 0x50ae1cf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x50ae1ea; continue; }
  pc = 0x50ae1dc; continue;
 }
 0x50ae1dc => {
  { cpu.sub16(cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x50ae2bd; continue; }
  pc = 0x50ae1ea; continue;
 }
 0x50ae1ea => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x50ae380u32); }
  if !cpu.zf { pc = 0x50ae204; continue; }
  pc = 0x50ae1f8; continue;
 }
 0x50ae1f8 => {
  { let v: u32 = 0x50ae385u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae216; continue;
 }
 0x50ae204 => {
  { cpu.sub32(cpu.ebx, 0x50ae1cfu32); }
  if !cpu.zf { pc = 0x50ae216; continue; }
  pc = 0x50ae20c; continue;
 }
 0x50ae20c => {
  { let v: u32 = 0x50ad0a2u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae216; continue;
 }
 0x50ae216 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ae25f; continue; }
  pc = 0x50ae222; continue;
 }
 0x50ae222 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ae25f; continue; }
  pc = 0x50ae227; continue;
 }
 0x50ae227 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bed8u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bed4u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x50ae2ac; continue;
 }
 0x50ae25f => {
  cpu.flat_ds(0x50ae25f); cpu.push(0x50ae264); f_050ae13b(cpu); if cpu.returned_to != 0x50ae264 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae295; continue; }
  pc = 0x50ae269; continue;
 }
 0x50ae269 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bed8u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bed4u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x50ae2ac; continue;
 }
 0x50ae295 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x50ae2a7); cpu.push(0x50ae2ac); f_05098b85(cpu); if cpu.returned_to != 0x50ae2ac { return; }
  pc = 0x50ae2ac; continue;
 }
 0x50ae2ac => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x514beccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x514bec8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae2bd; continue;
 }
 0x50ae2bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae2c1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae380); }
 let mut pc: u32 = 0x50ae380;
 loop { match pc {
 0x50ae380 => {
  cpu.push(0x50ae385); f_050ae1cf(cpu); if cpu.returned_to != 0x50ae385 { return; }
  f_050ae385(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae385(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae385); }
 let mut pc: u32 = 0x50ae385;
 loop { match pc {
 0x50ae2c2 => {
  { cpu.sub32(cpu.ebx, 0x50ae385u32); }
  if !cpu.zf { pc = 0x50ae2d4; continue; }
  pc = 0x50ae2ca; continue;
 }
 0x50ae2ca => {
  { let v: u32 = 0x50ad0a2u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae2d4; continue;
 }
 0x50ae2d4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ae31d; continue; }
  pc = 0x50ae2e0; continue;
 }
 0x50ae2e0 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ae31d; continue; }
  pc = 0x50ae2e5; continue;
 }
 0x50ae2e5 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bedeu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bedau32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x50ae36a; continue;
 }
 0x50ae31d => {
  cpu.flat_ds(0x50ae31d); cpu.push(0x50ae322); f_050ae13b(cpu); if cpu.returned_to != 0x50ae322 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae353; continue; }
  pc = 0x50ae327; continue;
 }
 0x50ae327 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x514bedeu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514bedau32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x50ae36a; continue;
 }
 0x50ae353 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x50ae365); cpu.push(0x50ae36a); f_05098b85(cpu); if cpu.returned_to != 0x50ae36a { return; }
  pc = 0x50ae36a; continue;
 }
 0x50ae36a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x514bed2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x514beceu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae37b; continue;
 }
 0x50ae37b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae37f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ae385 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x50ae39c; continue; }
  pc = 0x50ae392; continue;
 }
 0x50ae392 => {
  { cpu.sub16(cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x50ae37b; continue; }
  pc = 0x50ae39c; continue;
 }
 0x50ae39c => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x50ae380u32); }
  if !cpu.zf { cpu.poll(); pc = 0x50ae2c2; continue; }
  pc = 0x50ae3ae; continue;
 }
 0x50ae3ae => {
  { let v: u32 = 0x50ae1cfu32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x50ae2d4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae3bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae3bd); }
 let mut pc: u32 = 0x50ae3bd;
 loop { match pc {
 0x50ae3bd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514bec8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x50ae4d4; continue; }
  pc = 0x50ae3cd; continue;
 }
 0x50ae3cd => {
  { cpu.sub16(cpu.r16(0x514beccu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x50ae4d4; continue; }
  pc = 0x50ae3db; continue;
 }
 0x50ae3db => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ae43a; continue; }
  pc = 0x50ae3e7; continue;
 }
 0x50ae3e7 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ae43a; continue; }
  pc = 0x50ae3ec; continue;
 }
 0x50ae3ec => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514beccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514bec8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x50ae02eu32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bed8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bed4u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x50ae4a6; continue;
 }
 0x50ae43a => {
  cpu.flat_ds(0x50ae43a); cpu.push(0x50ae43f); f_050ae13b(cpu); if cpu.returned_to != 0x50ae43f { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae47f; continue; }
  pc = 0x50ae444; continue;
 }
 0x50ae444 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514beccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514bec8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bed8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514bed4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50ae02eu32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x50ae4a6; continue;
 }
 0x50ae47f => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x50ae02eu32; cpu.ebx = v; }
  cpu.flat_ds(0x50ae489); cpu.push(0x50ae48e); f_05098b53(cpu); if cpu.returned_to != 0x50ae48e { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514bec8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x514beccu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x50ae4a1); cpu.push(0x50ae4a6); f_05098b85(cpu); if cpu.returned_to != 0x50ae4a6 { return; }
  pc = 0x50ae4a6; continue;
 }
 0x50ae4a6 => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x50ad0a2u32); }
  if !cpu.zf { pc = 0x50ae4c2; continue; }
  pc = 0x50ae4b4; continue;
 }
 0x50ae4b4 => {
  { let v: u32 = 0x50ae1cfu32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae4c1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ae4c2 => {
  { cpu.sub32(cpu.ebx, 0x50ae385u32); }
  if !cpu.zf { pc = 0x50ae4d4; continue; }
  pc = 0x50ae4ca; continue;
 }
 0x50ae4ca => {
  { let v: u32 = 0x50ae380u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae4d4; continue;
 }
 0x50ae4d4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae4d7); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae4d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae4d8); }
 let mut pc: u32 = 0x50ae4d8;
 loop { match pc {
 0x50ae4d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514beceu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x50ae5ef; continue; }
  pc = 0x50ae4e8; continue;
 }
 0x50ae4e8 => {
  { cpu.sub16(cpu.r16(0x514bed2u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x50ae5ef; continue; }
  pc = 0x50ae4f6; continue;
 }
 0x50ae4f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x50ae555; continue; }
  pc = 0x50ae502; continue;
 }
 0x50ae502 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50ae555; continue; }
  pc = 0x50ae507; continue;
 }
 0x50ae507 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bed2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514beceu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x50ae0c3u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bedeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bedau32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x50ae5c1; continue;
 }
 0x50ae555 => {
  cpu.flat_ds(0x50ae555); cpu.push(0x50ae55a); f_050ae13b(cpu); if cpu.returned_to != 0x50ae55a { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae59a; continue; }
  pc = 0x50ae55f; continue;
 }
 0x50ae55f => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bed2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514beceu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bedeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514bedau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50ae0c3u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x50ae5c1; continue;
 }
 0x50ae59a => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x50ae0c3u32; cpu.ebx = v; }
  cpu.flat_ds(0x50ae5a4); cpu.push(0x50ae5a9); f_05098b53(cpu); if cpu.returned_to != 0x50ae5a9 { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514beceu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x514bed2u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x50ae5bc); cpu.push(0x50ae5c1); f_05098b85(cpu); if cpu.returned_to != 0x50ae5c1 { return; }
  pc = 0x50ae5c1; continue;
 }
 0x50ae5c1 => {
  { let v: u32 = cpu.r32(0x514bea8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x50ad0a2u32); }
  if !cpu.zf { pc = 0x50ae5dd; continue; }
  pc = 0x50ae5cf; continue;
 }
 0x50ae5cf => {
  { let v: u32 = 0x50ae385u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae5dc); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ae5dd => {
  { cpu.sub32(cpu.ebx, 0x50ae1cfu32); }
  if !cpu.zf { pc = 0x50ae5ef; continue; }
  pc = 0x50ae5e5; continue;
 }
 0x50ae5e5 => {
  { let v: u32 = 0x50ae380u32; cpu.w32(0x514bea8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50ae5ef; continue;
 }
 0x50ae5ef => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x50ae5f2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae6cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae6cb); }
 let mut pc: u32 = 0x50ae6cb;
 loop { match pc {
 0x50ae6cb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x514acfcu32)); }
  if cpu.cf { pc = 0x50ae736; continue; }
  pc = 0x50ae6db; continue;
 }
 0x50ae6db => {
  { cpu.sub8(cpu.r8(0x514ad16u32), 0x0u8); }
  if !cpu.zf { pc = 0x50ae748; continue; }
  pc = 0x50ae6e4; continue;
 }
 0x50ae6e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfu32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x50ae6f6; continue; }
  pc = 0x50ae6f1; continue;
 }
 0x50ae6f1 => {
  { let v: u32 = 0xfffffffu32; cpu.ebp = v; }
  pc = 0x50ae6f6; continue;
 }
 0x50ae6f6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xedu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.shl32(cpu.eax, 31u32); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi | cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x50ae723; continue; }
  pc = 0x50ae713; continue;
 }
 0x50ae713 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.edi; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50ae736; continue; }
  pc = 0x50ae723; continue;
 }
 0x50ae723 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x50ae728; continue;
 }
 0x50ae728 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x50ae77d; continue; }
  pc = 0x50ae736; continue;
 }
 0x50ae736 => {
  cpu.push(0x50ae73b); f_0509a511(cpu); if cpu.returned_to != 0x50ae73b { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x50ae78b; continue;
 }
 0x50ae748 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x50ae75d; continue; }
  pc = 0x50ae758; continue;
 }
 0x50ae758 => {
  { let v: u32 = 0xfffffu32; cpu.ebx = v; }
  pc = 0x50ae75d; continue;
 }
 0x50ae75d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad16u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae778; continue; }
  pc = 0x50ae769; continue;
 }
 0x50ae769 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x514ad17u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x50ae778; continue; }
  pc = 0x50ae775; continue;
 }
 0x50ae775 => {
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x50ae778; continue;
 }
 0x50ae778 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x50ae728; continue;
 }
 0x50ae77d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514ace8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514ace8u32, v); }
  pc = 0x50ae78b; continue;
 }
 0x50ae78b => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae796(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae796); }
 let mut pc: u32 = 0x50ae796;
 loop { match pc {
 0x50ae796 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x514beb0u32), 0x0u32); }
  if !cpu.zf { pc = 0x50ae861; continue; }
  pc = 0x50ae7ab; continue;
 }
 0x50ae7ab => {
  { let v = cpu.r16(0x514ad1du32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514ad19u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x50ae7c1; continue;
 }
 0x50ae7c1 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x50ae7d9; continue; }
  pc = 0x50ae7c6; continue;
 }
 0x50ae7c6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x50ae7d4; continue; }
  pc = 0x50ae7d0; continue;
 }
 0x50ae7d0 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x50ae7c6; continue;
 }
 0x50ae7d4 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x50ae7c1; continue;
 }
 0x50ae7d9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x50ae7e2; continue; }
  pc = 0x50ae7dd; continue;
 }
 0x50ae7dd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x50ae7e2; continue;
 }
 0x50ae7e2 => {
  cpu.push(0x50ae7e7); f_050a629c(cpu); if cpu.returned_to != 0x50ae7e7 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ae861; continue; }
  pc = 0x50ae7f3; continue;
 }
 0x50ae7f3 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x50ae802); f_050a629c(cpu); if cpu.returned_to != 0x50ae802 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ae85a; continue; }
  pc = 0x50ae806; continue;
 }
 0x50ae806 => {
  { let v: u32 = cpu.eax; cpu.w32(0x514beb0u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x50ae815; continue;
 }
 0x50ae815 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x50ae835; continue; }
  pc = 0x50ae81b; continue;
 }
 0x50ae81b => {
  { let v: u32 = cpu.r32(0x514beb0u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x50ae824; continue;
 }
 0x50ae824 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x50ae824; continue; }
  pc = 0x50ae82f; continue;
 }
 0x50ae82f => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x50ae815; continue;
 }
 0x50ae835 => {
  { let v: u32 = cpu.r32(0x514beb0u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514beb4u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x50ae858); f_05097b90(cpu); if cpu.returned_to != 0x50ae858 { return; }
  pc = 0x50ae861; continue;
 }
 0x50ae85a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50ae861); f_050a637f(cpu); if cpu.returned_to != 0x50ae861 { return; }
  pc = 0x50ae861; continue;
 }
 0x50ae861 => {
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae86a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae86a); }
 let mut pc: u32 = 0x50ae86a;
 loop { match pc {
 0x50ae86a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff0u32); }
  if !cpu.zf { pc = 0x50ae8f2; continue; }
  pc = 0x50ae897; continue;
 }
 0x50ae897 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x50ae8b0; continue; }
  pc = 0x50ae8aa; continue;
 }
 0x50ae8aa => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50ae8b0; continue;
 }
 0x50ae8b0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx | cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.zf { pc = 0x50ae8e3; continue; }
  pc = 0x50ae8db; continue;
 }
 0x50ae8db => {
  { let v: u32 = cpu.r32(0x513b240u32); cpu.edx = v; }
  pc = 0x50ae8e9; continue;
 }
 0x50ae8e3 => {
  { let v: u32 = cpu.r32(0x513b244u32); cpu.edx = v; }
  pc = 0x50ae8e9; continue;
 }
 0x50ae8e9 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x50ae8f4; continue;
 }
 0x50ae8f2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50ae8f4; continue;
 }
 0x50ae8f4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae8fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ae8fc); } r_050ae8fc(cpu, 0x50ae8fc); }
pub fn f_050ae943(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50ae943); } r_050ae8fc(cpu, 0x50ae943); }
fn r_050ae8fc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50ae8fc => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50ae8fe; continue;
 }
 0x50ae8fe => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x50ae923; continue; }
  pc = 0x50ae911; continue;
 }
 0x50ae911 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x50ae91f); f_050aee21(cpu); if cpu.returned_to != 0x50ae91f { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50ae93b; continue;
 }
 0x50ae923 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x50ae92b; continue; }
  pc = 0x50ae927; continue;
 }
 0x50ae927 => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x50ae935; continue;
 }
 0x50ae92b => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x50ae933; continue; }
  pc = 0x50ae92f; continue;
 }
 0x50ae92f => {
  { cpu.fpush(1.0); }
  pc = 0x50ae935; continue;
 }
 0x50ae933 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x50ae935; continue;
 }
 0x50ae935 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x50ae93b; continue;
 }
 0x50ae93b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50ae943 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x50ae8fe; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae947); }
 let mut pc: u32 = 0x50ae947;
 loop { match pc {
 0x50ae947 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x50ae950); f_050ae8fc(cpu); if cpu.returned_to != 0x50ae950 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae96b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae96b); }
 let mut pc: u32 = 0x50ae96b;
 loop { match pc {
 0x50ae96b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x50ae985); f_050aee5e(cpu); if cpu.returned_to != 0x50ae985 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x50ae99a; continue; }
  pc = 0x50ae98e; continue;
 }
 0x50ae98e => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x513b248u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x50ae99a; continue;
 }
 0x50ae99a => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050ae9a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50ae9a7); }
 let mut pc: u32 = 0x50ae9a7;
 loop { match pc {
 0x50ae9a7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x50ae9d8); f_050ae86a(cpu); if cpu.returned_to != 0x50ae9d8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50ae9e3; continue; }
  pc = 0x50ae9dc; continue;
 }
 0x50ae9dc => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x50aeb78; continue;
 }
 0x50ae9e3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50aea0d; continue; }
  pc = 0x50aea05; continue;
 }
 0x50aea05 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x50aeb35; continue; }
  pc = 0x50aea0d; continue;
 }
 0x50aea0d => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x50aea25; continue; }
  pc = 0x50aea17; continue;
 }
 0x50aea17 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x50aea25; continue;
 }
 0x50aea25 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x50aea33); f_050aee77(cpu); if cpu.returned_to != 0x50aea33 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50aea3d; continue; }
  pc = 0x50aea3b; continue;
 }
 0x50aea3b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x50aea3d; continue;
 }
 0x50aea3d => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x50aea5d; continue; }
  pc = 0x50aea5b; continue;
 }
 0x50aea5b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x50aea5d; continue;
 }
 0x50aea5d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x50aea69; continue; }
  pc = 0x50aea67; continue;
 }
 0x50aea67 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x50aea69; continue;
 }
 0x50aea69 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50aeb03; continue; }
  pc = 0x50aea71; continue;
 }
 0x50aea71 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aea7b; continue; }
  pc = 0x50aea76; continue;
 }
 0x50aea76 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x50aea7b; continue;
 }
 0x50aea7b => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x50aea91); f_050aee5e(cpu); if cpu.returned_to != 0x50aea91 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x50aeaa5; continue; }
  pc = 0x50aea9a; continue;
 }
 0x50aea9a => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aeaa5; continue; }
  pc = 0x50aea9e; continue;
 }
 0x50aea9e => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50aeaa5; continue;
 }
 0x50aeaa5 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x50aeab9); f_05097de0(cpu); if cpu.returned_to != 0x50aeab9 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50aeb03; continue; }
  pc = 0x50aeabf; continue;
 }
 0x50aeabf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x50aeacd; continue; }
  pc = 0x50aeac7; continue;
 }
 0x50aeac7 => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x50aeacf; continue; }
  pc = 0x50aeacd; continue;
 }
 0x50aeacd => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x50aeacf; continue;
 }
 0x50aeacf => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x50aeadb; continue; }
  pc = 0x50aead4; continue;
 }
 0x50aead4 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x50aeae5; continue;
 }
 0x50aeadb => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aeae5; continue; }
  pc = 0x50aeae0; continue;
 }
 0x50aeae0 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x50aeae5; continue;
 }
 0x50aeae5 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x50aeaf9); f_05097de0(cpu); if cpu.returned_to != 0x50aeaf9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aeb03; continue; }
  pc = 0x50aeafd; continue;
 }
 0x50aeafd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x50aeb03; continue;
 }
 0x50aeb03 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd6u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x50aeb35; continue; }
  pc = 0x50aeb29; continue;
 }
 0x50aeb29 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x50aeb35; continue; }
  pc = 0x50aeb33; continue;
 }
 0x50aeb33 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x50aeb35; continue;
 }
 0x50aeb35 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x50aeb41; continue; }
  pc = 0x50aeb3a; continue;
 }
 0x50aeb3a => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x50aeb4b; continue;
 }
 0x50aeb41 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aeb4b; continue; }
  pc = 0x50aeb46; continue;
 }
 0x50aeb46 => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x50aeb4b; continue;
 }
 0x50aeb4b => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x50aeb5c); f_050a6a53(cpu); if cpu.returned_to != 0x50aeb5c { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x50aeb5d; continue;
 }
 0x50aeb5d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x50aeb75; continue; }
  pc = 0x50aeb65; continue;
 }
 0x50aeb65 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x50aeb5d; continue; }
  pc = 0x50aeb75; continue;
 }
 0x50aeb75 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x50aeb78; continue;
 }
 0x50aeb78 => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(32);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aeb84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aeb84); }
 let mut pc: u32 = 0x50aeb84;
 loop { match pc {
 0x50aeb84 => {
  pc = 0x50aeee6; continue;
 }
 0x50aeee6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aeb89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aeb89); }
 let mut pc: u32 = 0x50aeb89;
 loop { match pc {
 0x50aeb89 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.cf || cpu.zf) { pc = 0x50aeb93; continue; }
  pc = 0x50aeb8f; continue;
 }
 0x50aeb8f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aeb93 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x514a184u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aeba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aeba8); }
 let mut pc: u32 = 0x50aeba8;
 loop { match pc {
 0x50aeba8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x50aebbe; continue; }
  pc = 0x50aebb7; continue;
 }
 0x50aebb7 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x50aebbe); f_050aa8e7(cpu); if cpu.returned_to != 0x50aebbe { return; }
  pc = 0x50aebbe; continue;
 }
 0x50aebbe => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x50aebda; continue; }
  pc = 0x50aebc8; continue;
 }
 0x50aebc8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x50aebda; continue;
 }
 0x50aebda => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x50aebde; continue;
 }
 0x50aebde => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50aebf5; continue; }
  pc = 0x50aebe6; continue;
 }
 0x50aebe6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x50aebee); f_050aa843(cpu); if cpu.returned_to != 0x50aebee { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x50aebde; continue; }
  pc = 0x50aebf3; continue;
 }
 0x50aebf3 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x50aebf5; continue;
 }
 0x50aebf5 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x50aec13; continue; }
  pc = 0x50aebf9; continue;
 }
 0x50aebf9 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x50aec13; continue; }
  pc = 0x50aec0a; continue;
 }
 0x50aec0a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x50aec11); f_050a5f0a(cpu); if cpu.returned_to != 0x50aec11 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x50aec13; continue;
 }
 0x50aec13 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x50aec1b; continue; }
  pc = 0x50aec17; continue;
 }
 0x50aec17 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x50aec1b; continue;
 }
 0x50aec1b => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aec23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aec23); }
 let mut pc: u32 = 0x50aec23;
 loop { match pc {
 0x50aec23 => {
  { let v: u32 = cpu.r32(0x514acf4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aec29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aec29); }
 let mut pc: u32 = 0x50aec29;
 loop { match pc {
 0x50aec29 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x50aec35); f_0509a92b(cpu); if cpu.returned_to != 0x50aec35 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x50aec66; continue; }
  pc = 0x50aec3c; continue;
 }
 0x50aec3c => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x50aec4f; continue; }
  pc = 0x50aec42; continue;
 }
 0x50aec42 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x50aec4f; continue; }
  pc = 0x50aec48; continue;
 }
 0x50aec48 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50aec4f); f_050aeee9(cpu); if cpu.returned_to != 0x50aec4f { return; }
  pc = 0x50aec4f; continue;
 }
 0x50aec4f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x50aec66; continue; }
  pc = 0x50aec55; continue;
 }
 0x50aec55 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x50aec63; continue; }
  pc = 0x50aec5b; continue;
 }
 0x50aec5b => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aec63 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x50aec66; continue;
 }
 0x50aec66 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aec6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aec6b); }
 let mut pc: u32 = 0x50aec6b;
 loop { match pc {
 0x50aec6b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_050aed5f(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aec75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aec75); }
 let mut pc: u32 = 0x50aec75;
 loop { match pc {
 0x50aec75 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x514bef0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x50aeca4; continue; }
  pc = 0x50aec88; continue;
 }
 0x50aec88 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x50aeca4; continue; }
  pc = 0x50aec8d; continue;
 }
 0x50aec8d => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x50aeca4; continue; }
  pc = 0x50aec92; continue;
 }
 0x50aec92 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x514bef0u32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x50aeca4); cpu.call(t); if cpu.returned_to != 0x50aeca4 { return; }
  pc = 0x50aeca4; continue;
 }
 0x50aeca4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aecaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aecaa); }
 let mut pc: u32 = 0x50aecaa;
 loop { match pc {
 0x50aecaa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x50aecb7; continue; }
  pc = 0x50aecb2; continue;
 }
 0x50aecb2 => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50aecc8; continue; }
  pc = 0x50aecb7; continue;
 }
 0x50aecb7 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x50aecc1); f_05097db1(cpu); if cpu.returned_to != 0x50aecc1 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aecc8 => {
  { let v: u32 = 0x50aec6bu32; cpu.w32(0x514bf1cu32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x50aecef; continue; }
  pc = 0x50aecd7; continue;
 }
 0x50aecd7 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x50aece3; continue; }
  pc = 0x50aecdc; continue;
 }
 0x50aecdc => {
  cpu.push(0x50aece1); f_050ae1cf(cpu); if cpu.returned_to != 0x50aece1 { return; }
  pc = 0x50aed1f; continue;
 }
 0x50aece3 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x50aed1f; continue; }
  pc = 0x50aece8; continue;
 }
 0x50aece8 => {
  cpu.push(0x50aeced); f_050ae3bd(cpu); if cpu.returned_to != 0x50aeced { return; }
  pc = 0x50aed1f; continue;
 }
 0x50aecef => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x50aed0c; continue; }
  pc = 0x50aecf4; continue;
 }
 0x50aecf4 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x50aed00; continue; }
  pc = 0x50aecf9; continue;
 }
 0x50aecf9 => {
  cpu.push(0x50aecfe); f_050ae385(cpu); if cpu.returned_to != 0x50aecfe { return; }
  pc = 0x50aed1f; continue;
 }
 0x50aed00 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x50aed1f; continue; }
  pc = 0x50aed05; continue;
 }
 0x50aed05 => {
  cpu.push(0x50aed0a); f_050ae4d8(cpu); if cpu.returned_to != 0x50aed0a { return; }
  pc = 0x50aed1f; continue;
 }
 0x50aed0c => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x50aed1f; continue; }
  pc = 0x50aed11; continue;
 }
 0x50aed11 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x50aed1f; continue; }
  pc = 0x50aed15; continue;
 }
 0x50aed15 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x50aed1f; continue; }
  pc = 0x50aed1a; continue;
 }
 0x50aed1a => {
  cpu.push(0x50aed1f); f_050aef11(cpu); if cpu.returned_to != 0x50aed1f { return; }
  pc = 0x50aed1f; continue;
 }
 0x50aed1f => {
  { let v: u32 = cpu.r32(0x514bee8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514bee8u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aed2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aed2f); }
 let mut pc: u32 = 0x50aed2f;
 loop { match pc {
 0x50aed2f => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50aed89u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50aeda9u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x50aedf5u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50aedf5u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50aedf5u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x50aedf5u32)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  f_050aed5f(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aed5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aed5f); }
 let mut pc: u32 = 0x50aed5f;
 loop { match pc {
 0x50aed5f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x514bee8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50aee17; continue; }
  pc = 0x50aed81; continue;
 }
 0x50aed81 => {
  cpu.poll(); pc = cpu.r32(0x50aed2fu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x50aed89 => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x50aed93); f_050aec75(cpu); if cpu.returned_to != 0x50aed93 { return; }
  pc = 0x50aed93; continue;
 }
 0x50aed93 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x50aed95; continue;
 }
 0x50aed95 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aed9e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x50aeda9; continue; }
  pc = 0x50aeda4; continue;
 }
 0x50aeda4 => {
  cpu.push(0x50aeda9); f_050aeeff(cpu); if cpu.returned_to != 0x50aeda9 { return; }
  pc = 0x50aeda9; continue;
 }
 0x50aeda9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aedb1; continue;
 }
 0x50aedb1 => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aedb6; continue;
 }
 0x50aedb6 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aedbb; continue;
 }
 0x50aedbb => {
  { let v: u32 = 0x2u32; cpu.w32(0x514bee8u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x50aedcb); f_050ae1cf(cpu); if cpu.returned_to != 0x50aedcb { return; }
  pc = 0x50aedcb; continue;
 }
 0x50aedcb => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x50aedcd; continue;
 }
 0x50aedcd => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x50aedd0); cpu.call(t); if cpu.returned_to != 0x50aedd0 { return; }
  cpu.poll(); pc = 0x50aed93; continue;
 }
 0x50aedd2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aedda; continue;
 }
 0x50aedda => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aeddf; continue;
 }
 0x50aeddf => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aede4; continue;
 }
 0x50aede4 => {
  { let v: u32 = 0x2u32; cpu.w32(0x514bee8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x50aedf3); f_050ae385(cpu); if cpu.returned_to != 0x50aedf3 { return; }
  cpu.poll(); pc = 0x50aedcb; continue;
 }
 0x50aedf5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aedfd; continue;
 }
 0x50aedfd => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aee02; continue;
 }
 0x50aee02 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x50aed93; continue; }
  pc = 0x50aee07; continue;
 }
 0x50aee07 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x514bee8u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x50aedcd; continue;
 }
 0x50aee17 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x50aed95; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aee21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aee21); }
 let mut pc: u32 = 0x50aee21;
 loop { match pc {
 0x50aee21 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x50aee4a; continue; }
  pc = 0x50aee32; continue;
 }
 0x50aee32 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50aee4a; continue; }
  pc = 0x50aee40; continue;
 }
 0x50aee40 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x50aee4a; continue; }
  pc = 0x50aee44; continue;
 }
 0x50aee44 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50aee4e; continue;
 }
 0x50aee4a => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50aee4e; continue;
 }
 0x50aee4e => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x50aee56); f_050aad05(cpu); if cpu.returned_to != 0x50aee56 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aee5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aee5e); }
 let mut pc: u32 = 0x50aee5e;
 loop { match pc {
 0x50aee5e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x50aee6a); f_05086172(cpu); if cpu.returned_to != 0x50aee6a { return; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aee77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aee77); }
 let mut pc: u32 = 0x50aee77;
 loop { match pc {
 0x50aee77 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x50aee9e; continue; }
  pc = 0x50aee9a; continue;
 }
 0x50aee9a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50aeed3; continue; }
  pc = 0x50aee9e; continue;
 }
 0x50aee9e => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7ff0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0x800fu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx | 0x3fe0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3feu32); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x50aeed3; continue;
 }
 0x50aeed3 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
