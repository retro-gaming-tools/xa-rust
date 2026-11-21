#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_060bda88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda88); }
 let mut pc: u32 = 0x60bda88;
 loop { match pc {
 0x60bda88 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda8c); }
 let mut pc: u32 = 0x60bda8c;
 loop { match pc {
 0x60bda8c => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda90); }
 let mut pc: u32 = 0x60bda90;
 loop { match pc {
 0x60bda90 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda94); }
 let mut pc: u32 = 0x60bda94;
 loop { match pc {
 0x60bda94 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda98); }
 let mut pc: u32 = 0x60bda98;
 loop { match pc {
 0x60bda98 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda9c); }
 let mut pc: u32 = 0x60bda9c;
 loop { match pc {
 0x60bda9c => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaa0); }
 let mut pc: u32 = 0x60bdaa0;
 loop { match pc {
 0x60bdaa0 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaa4); }
 let mut pc: u32 = 0x60bdaa4;
 loop { match pc {
 0x60bdaa4 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaa8); }
 let mut pc: u32 = 0x60bdaa8;
 loop { match pc {
 0x60bdaa8 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaac); }
 let mut pc: u32 = 0x60bdaac;
 loop { match pc {
 0x60bdaac => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdab0); }
 let mut pc: u32 = 0x60bdab0;
 loop { match pc {
 0x60bdab0 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdab4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdab4); }
 let mut pc: u32 = 0x60bdab4;
 loop { match pc {
 0x60bdab4 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdab8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdab8); }
 let mut pc: u32 = 0x60bdab8;
 loop { match pc {
 0x60bdab8 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdabc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdabc); }
 let mut pc: u32 = 0x60bdabc;
 loop { match pc {
 0x60bdabc => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdac0); }
 let mut pc: u32 = 0x60bdac0;
 loop { match pc {
 0x60bdac0 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdac4); }
 let mut pc: u32 = 0x60bdac4;
 loop { match pc {
 0x60bdac4 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdac8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdac8); }
 let mut pc: u32 = 0x60bdac8;
 loop { match pc {
 0x60bdac8 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdacc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdacc); }
 let mut pc: u32 = 0x60bdacc;
 loop { match pc {
 0x60bdacc => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdad0); }
 let mut pc: u32 = 0x60bdad0;
 loop { match pc {
 0x60bdad0 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdad4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdad4); }
 let mut pc: u32 = 0x60bdad4;
 loop { match pc {
 0x60bdad4 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdad8); }
 let mut pc: u32 = 0x60bdad8;
 loop { match pc {
 0x60bdad8 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdadc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdadc); }
 let mut pc: u32 = 0x60bdadc;
 loop { match pc {
 0x60bdadc => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdae0); }
 let mut pc: u32 = 0x60bdae0;
 loop { match pc {
 0x60bdae0 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdae4); }
 let mut pc: u32 = 0x60bdae4;
 loop { match pc {
 0x60bdae4 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdae8); }
 let mut pc: u32 = 0x60bdae8;
 loop { match pc {
 0x60bdae8 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaec); }
 let mut pc: u32 = 0x60bdaec;
 loop { match pc {
 0x60bdaec => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaf0); }
 let mut pc: u32 = 0x60bdaf0;
 loop { match pc {
 0x60bdaf0 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaf4); }
 let mut pc: u32 = 0x60bdaf4;
 loop { match pc {
 0x60bdaf4 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdaf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdaf8); }
 let mut pc: u32 = 0x60bdaf8;
 loop { match pc {
 0x60bdaf8 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdafc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdafc); }
 let mut pc: u32 = 0x60bdafc;
 loop { match pc {
 0x60bdafc => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb00); }
 let mut pc: u32 = 0x60bdb00;
 loop { match pc {
 0x60bdb00 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb04); }
 let mut pc: u32 = 0x60bdb04;
 loop { match pc {
 0x60bdb04 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb08); }
 let mut pc: u32 = 0x60bdb08;
 loop { match pc {
 0x60bdb08 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb0c); }
 let mut pc: u32 = 0x60bdb0c;
 loop { match pc {
 0x60bdb0c => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb10); }
 let mut pc: u32 = 0x60bdb10;
 loop { match pc {
 0x60bdb10 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb14); }
 let mut pc: u32 = 0x60bdb14;
 loop { match pc {
 0x60bdb14 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb18); }
 let mut pc: u32 = 0x60bdb18;
 loop { match pc {
 0x60bdb18 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb1c); }
 let mut pc: u32 = 0x60bdb1c;
 loop { match pc {
 0x60bdb1c => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb20); }
 let mut pc: u32 = 0x60bdb20;
 loop { match pc {
 0x60bdb20 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb24); }
 let mut pc: u32 = 0x60bdb24;
 loop { match pc {
 0x60bdb24 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb28); }
 let mut pc: u32 = 0x60bdb28;
 loop { match pc {
 0x60bdb28 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb2c); }
 let mut pc: u32 = 0x60bdb2c;
 loop { match pc {
 0x60bdb2c => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb30); }
 let mut pc: u32 = 0x60bdb30;
 loop { match pc {
 0x60bdb30 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb34); }
 let mut pc: u32 = 0x60bdb34;
 loop { match pc {
 0x60bdb34 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb38); }
 let mut pc: u32 = 0x60bdb38;
 loop { match pc {
 0x60bdb38 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb3c); }
 let mut pc: u32 = 0x60bdb3c;
 loop { match pc {
 0x60bdb3c => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb40); }
 let mut pc: u32 = 0x60bdb40;
 loop { match pc {
 0x60bdb40 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb44); }
 let mut pc: u32 = 0x60bdb44;
 loop { match pc {
 0x60bdb44 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb48); }
 let mut pc: u32 = 0x60bdb48;
 loop { match pc {
 0x60bdb48 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb4c); }
 let mut pc: u32 = 0x60bdb4c;
 loop { match pc {
 0x60bdb4c => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb50); }
 let mut pc: u32 = 0x60bdb50;
 loop { match pc {
 0x60bdb50 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb54); }
 let mut pc: u32 = 0x60bdb54;
 loop { match pc {
 0x60bdb54 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb58); }
 let mut pc: u32 = 0x60bdb58;
 loop { match pc {
 0x60bdb58 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb5c); }
 let mut pc: u32 = 0x60bdb5c;
 loop { match pc {
 0x60bdb5c => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb60); }
 let mut pc: u32 = 0x60bdb60;
 loop { match pc {
 0x60bdb60 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb64); }
 let mut pc: u32 = 0x60bdb64;
 loop { match pc {
 0x60bdb64 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb68); }
 let mut pc: u32 = 0x60bdb68;
 loop { match pc {
 0x60bdb68 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb6c); }
 let mut pc: u32 = 0x60bdb6c;
 loop { match pc {
 0x60bdb6c => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb70); }
 let mut pc: u32 = 0x60bdb70;
 loop { match pc {
 0x60bdb70 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb74); }
 let mut pc: u32 = 0x60bdb74;
 loop { match pc {
 0x60bdb74 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb78); }
 let mut pc: u32 = 0x60bdb78;
 loop { match pc {
 0x60bdb78 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb7c); }
 let mut pc: u32 = 0x60bdb7c;
 loop { match pc {
 0x60bdb7c => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb80); }
 let mut pc: u32 = 0x60bdb80;
 loop { match pc {
 0x60bdb80 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb84); }
 let mut pc: u32 = 0x60bdb84;
 loop { match pc {
 0x60bdb84 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb88); }
 let mut pc: u32 = 0x60bdb88;
 loop { match pc {
 0x60bdb88 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb8c); }
 let mut pc: u32 = 0x60bdb8c;
 loop { match pc {
 0x60bdb8c => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb90); }
 let mut pc: u32 = 0x60bdb90;
 loop { match pc {
 0x60bdb90 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb94); }
 let mut pc: u32 = 0x60bdb94;
 loop { match pc {
 0x60bdb94 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb98); }
 let mut pc: u32 = 0x60bdb98;
 loop { match pc {
 0x60bdb98 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdb9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdb9c); }
 let mut pc: u32 = 0x60bdb9c;
 loop { match pc {
 0x60bdb9c => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdba0); }
 let mut pc: u32 = 0x60bdba0;
 loop { match pc {
 0x60bdba0 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdba4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdba4); }
 let mut pc: u32 = 0x60bdba4;
 loop { match pc {
 0x60bdba4 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdba8); }
 let mut pc: u32 = 0x60bdba8;
 loop { match pc {
 0x60bdba8 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbac); }
 let mut pc: u32 = 0x60bdbac;
 loop { match pc {
 0x60bdbac => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbb0); }
 let mut pc: u32 = 0x60bdbb0;
 loop { match pc {
 0x60bdbb0 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbb4); }
 let mut pc: u32 = 0x60bdbb4;
 loop { match pc {
 0x60bdbb4 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbb8); }
 let mut pc: u32 = 0x60bdbb8;
 loop { match pc {
 0x60bdbb8 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbbc); }
 let mut pc: u32 = 0x60bdbbc;
 loop { match pc {
 0x60bdbbc => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbc0); }
 let mut pc: u32 = 0x60bdbc0;
 loop { match pc {
 0x60bdbc0 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbc4); }
 let mut pc: u32 = 0x60bdbc4;
 loop { match pc {
 0x60bdbc4 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbc8); }
 let mut pc: u32 = 0x60bdbc8;
 loop { match pc {
 0x60bdbc8 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbcc); }
 let mut pc: u32 = 0x60bdbcc;
 loop { match pc {
 0x60bdbcc => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbd0); }
 let mut pc: u32 = 0x60bdbd0;
 loop { match pc {
 0x60bdbd0 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbd4); }
 let mut pc: u32 = 0x60bdbd4;
 loop { match pc {
 0x60bdbd4 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbd8); }
 let mut pc: u32 = 0x60bdbd8;
 loop { match pc {
 0x60bdbd8 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbdc); }
 let mut pc: u32 = 0x60bdbdc;
 loop { match pc {
 0x60bdbdc => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbe0); }
 let mut pc: u32 = 0x60bdbe0;
 loop { match pc {
 0x60bdbe0 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbe4); }
 let mut pc: u32 = 0x60bdbe4;
 loop { match pc {
 0x60bdbe4 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbe8); }
 let mut pc: u32 = 0x60bdbe8;
 loop { match pc {
 0x60bdbe8 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbec); }
 let mut pc: u32 = 0x60bdbec;
 loop { match pc {
 0x60bdbec => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbf0); }
 let mut pc: u32 = 0x60bdbf0;
 loop { match pc {
 0x60bdbf0 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbf4); }
 let mut pc: u32 = 0x60bdbf4;
 loop { match pc {
 0x60bdbf4 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbf8); }
 let mut pc: u32 = 0x60bdbf8;
 loop { match pc {
 0x60bdbf8 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdbfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdbfc); }
 let mut pc: u32 = 0x60bdbfc;
 loop { match pc {
 0x60bdbfc => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc00); }
 let mut pc: u32 = 0x60bdc00;
 loop { match pc {
 0x60bdc00 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc04); }
 let mut pc: u32 = 0x60bdc04;
 loop { match pc {
 0x60bdc04 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc08); }
 let mut pc: u32 = 0x60bdc08;
 loop { match pc {
 0x60bdc08 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc0c); }
 let mut pc: u32 = 0x60bdc0c;
 loop { match pc {
 0x60bdc0c => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc10); }
 let mut pc: u32 = 0x60bdc10;
 loop { match pc {
 0x60bdc10 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc14); }
 let mut pc: u32 = 0x60bdc14;
 loop { match pc {
 0x60bdc14 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc18); }
 let mut pc: u32 = 0x60bdc18;
 loop { match pc {
 0x60bdc18 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc1c); }
 let mut pc: u32 = 0x60bdc1c;
 loop { match pc {
 0x60bdc1c => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc20); }
 let mut pc: u32 = 0x60bdc20;
 loop { match pc {
 0x60bdc20 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc24); }
 let mut pc: u32 = 0x60bdc24;
 loop { match pc {
 0x60bdc24 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc28); }
 let mut pc: u32 = 0x60bdc28;
 loop { match pc {
 0x60bdc28 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc2c); }
 let mut pc: u32 = 0x60bdc2c;
 loop { match pc {
 0x60bdc2c => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc30); }
 let mut pc: u32 = 0x60bdc30;
 loop { match pc {
 0x60bdc30 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc34); }
 let mut pc: u32 = 0x60bdc34;
 loop { match pc {
 0x60bdc34 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc38); }
 let mut pc: u32 = 0x60bdc38;
 loop { match pc {
 0x60bdc38 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc3c); }
 let mut pc: u32 = 0x60bdc3c;
 loop { match pc {
 0x60bdc3c => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc40); }
 let mut pc: u32 = 0x60bdc40;
 loop { match pc {
 0x60bdc40 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc44); }
 let mut pc: u32 = 0x60bdc44;
 loop { match pc {
 0x60bdc44 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc48); }
 let mut pc: u32 = 0x60bdc48;
 loop { match pc {
 0x60bdc48 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc4c); }
 let mut pc: u32 = 0x60bdc4c;
 loop { match pc {
 0x60bdc4c => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc50); }
 let mut pc: u32 = 0x60bdc50;
 loop { match pc {
 0x60bdc50 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc54); }
 let mut pc: u32 = 0x60bdc54;
 loop { match pc {
 0x60bdc54 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc58); }
 let mut pc: u32 = 0x60bdc58;
 loop { match pc {
 0x60bdc58 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc5c); }
 let mut pc: u32 = 0x60bdc5c;
 loop { match pc {
 0x60bdc5c => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc60); }
 let mut pc: u32 = 0x60bdc60;
 loop { match pc {
 0x60bdc60 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc64); }
 let mut pc: u32 = 0x60bdc64;
 loop { match pc {
 0x60bdc64 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc68); }
 let mut pc: u32 = 0x60bdc68;
 loop { match pc {
 0x60bdc68 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc6c); }
 let mut pc: u32 = 0x60bdc6c;
 loop { match pc {
 0x60bdc6c => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc70); }
 let mut pc: u32 = 0x60bdc70;
 loop { match pc {
 0x60bdc70 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc74); }
 let mut pc: u32 = 0x60bdc74;
 loop { match pc {
 0x60bdc74 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc78); }
 let mut pc: u32 = 0x60bdc78;
 loop { match pc {
 0x60bdc78 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc7c); }
 let mut pc: u32 = 0x60bdc7c;
 loop { match pc {
 0x60bdc7c => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc80); }
 let mut pc: u32 = 0x60bdc80;
 loop { match pc {
 0x60bdc80 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc84); }
 let mut pc: u32 = 0x60bdc84;
 loop { match pc {
 0x60bdc84 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc88); }
 let mut pc: u32 = 0x60bdc88;
 loop { match pc {
 0x60bdc88 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc8c); }
 let mut pc: u32 = 0x60bdc8c;
 loop { match pc {
 0x60bdc8c => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc90); }
 let mut pc: u32 = 0x60bdc90;
 loop { match pc {
 0x60bdc90 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc94); }
 let mut pc: u32 = 0x60bdc94;
 loop { match pc {
 0x60bdc94 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc98); }
 let mut pc: u32 = 0x60bdc98;
 loop { match pc {
 0x60bdc98 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdc9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdc9c); }
 let mut pc: u32 = 0x60bdc9c;
 loop { match pc {
 0x60bdc9c => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdca0); }
 let mut pc: u32 = 0x60bdca0;
 loop { match pc {
 0x60bdca0 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdca4); }
 let mut pc: u32 = 0x60bdca4;
 loop { match pc {
 0x60bdca4 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdca8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdca8); }
 let mut pc: u32 = 0x60bdca8;
 loop { match pc {
 0x60bdca8 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcac); }
 let mut pc: u32 = 0x60bdcac;
 loop { match pc {
 0x60bdcac => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcb0); }
 let mut pc: u32 = 0x60bdcb0;
 loop { match pc {
 0x60bdcb0 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcb4); }
 let mut pc: u32 = 0x60bdcb4;
 loop { match pc {
 0x60bdcb4 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcb8); }
 let mut pc: u32 = 0x60bdcb8;
 loop { match pc {
 0x60bdcb8 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcbc); }
 let mut pc: u32 = 0x60bdcbc;
 loop { match pc {
 0x60bdcbc => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcc0); }
 let mut pc: u32 = 0x60bdcc0;
 loop { match pc {
 0x60bdcc0 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcc4); }
 let mut pc: u32 = 0x60bdcc4;
 loop { match pc {
 0x60bdcc4 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcc8); }
 let mut pc: u32 = 0x60bdcc8;
 loop { match pc {
 0x60bdcc8 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdccc); }
 let mut pc: u32 = 0x60bdccc;
 loop { match pc {
 0x60bdccc => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcd0); }
 let mut pc: u32 = 0x60bdcd0;
 loop { match pc {
 0x60bdcd0 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcd4); }
 let mut pc: u32 = 0x60bdcd4;
 loop { match pc {
 0x60bdcd4 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcd8); }
 let mut pc: u32 = 0x60bdcd8;
 loop { match pc {
 0x60bdcd8 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcdc); }
 let mut pc: u32 = 0x60bdcdc;
 loop { match pc {
 0x60bdcdc => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdce0); }
 let mut pc: u32 = 0x60bdce0;
 loop { match pc {
 0x60bdce0 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdce4); }
 let mut pc: u32 = 0x60bdce4;
 loop { match pc {
 0x60bdce4 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdce8); }
 let mut pc: u32 = 0x60bdce8;
 loop { match pc {
 0x60bdce8 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcec); }
 let mut pc: u32 = 0x60bdcec;
 loop { match pc {
 0x60bdcec => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcf0); }
 let mut pc: u32 = 0x60bdcf0;
 loop { match pc {
 0x60bdcf0 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcf4); }
 let mut pc: u32 = 0x60bdcf4;
 loop { match pc {
 0x60bdcf4 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcf8); }
 let mut pc: u32 = 0x60bdcf8;
 loop { match pc {
 0x60bdcf8 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdcfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdcfc); }
 let mut pc: u32 = 0x60bdcfc;
 loop { match pc {
 0x60bdcfc => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd00); }
 let mut pc: u32 = 0x60bdd00;
 loop { match pc {
 0x60bdd00 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd04); }
 let mut pc: u32 = 0x60bdd04;
 loop { match pc {
 0x60bdd04 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd08); }
 let mut pc: u32 = 0x60bdd08;
 loop { match pc {
 0x60bdd08 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd0c); }
 let mut pc: u32 = 0x60bdd0c;
 loop { match pc {
 0x60bdd0c => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd10); }
 let mut pc: u32 = 0x60bdd10;
 loop { match pc {
 0x60bdd10 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd14); }
 let mut pc: u32 = 0x60bdd14;
 loop { match pc {
 0x60bdd14 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd18); }
 let mut pc: u32 = 0x60bdd18;
 loop { match pc {
 0x60bdd18 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd1c); }
 let mut pc: u32 = 0x60bdd1c;
 loop { match pc {
 0x60bdd1c => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd20); }
 let mut pc: u32 = 0x60bdd20;
 loop { match pc {
 0x60bdd20 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd24); }
 let mut pc: u32 = 0x60bdd24;
 loop { match pc {
 0x60bdd24 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd28); }
 let mut pc: u32 = 0x60bdd28;
 loop { match pc {
 0x60bdd28 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd2c); }
 let mut pc: u32 = 0x60bdd2c;
 loop { match pc {
 0x60bdd2c => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd30); }
 let mut pc: u32 = 0x60bdd30;
 loop { match pc {
 0x60bdd30 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd34); }
 let mut pc: u32 = 0x60bdd34;
 loop { match pc {
 0x60bdd34 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd38); }
 let mut pc: u32 = 0x60bdd38;
 loop { match pc {
 0x60bdd38 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd3c); }
 let mut pc: u32 = 0x60bdd3c;
 loop { match pc {
 0x60bdd3c => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd40); }
 let mut pc: u32 = 0x60bdd40;
 loop { match pc {
 0x60bdd40 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd44); }
 let mut pc: u32 = 0x60bdd44;
 loop { match pc {
 0x60bdd44 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd48); }
 let mut pc: u32 = 0x60bdd48;
 loop { match pc {
 0x60bdd48 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd4c); }
 let mut pc: u32 = 0x60bdd4c;
 loop { match pc {
 0x60bdd4c => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd50); }
 let mut pc: u32 = 0x60bdd50;
 loop { match pc {
 0x60bdd50 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd54); }
 let mut pc: u32 = 0x60bdd54;
 loop { match pc {
 0x60bdd54 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd58); }
 let mut pc: u32 = 0x60bdd58;
 loop { match pc {
 0x60bdd58 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd5c); }
 let mut pc: u32 = 0x60bdd5c;
 loop { match pc {
 0x60bdd5c => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd60); }
 let mut pc: u32 = 0x60bdd60;
 loop { match pc {
 0x60bdd60 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd64); }
 let mut pc: u32 = 0x60bdd64;
 loop { match pc {
 0x60bdd64 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd68); }
 let mut pc: u32 = 0x60bdd68;
 loop { match pc {
 0x60bdd68 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd6c); }
 let mut pc: u32 = 0x60bdd6c;
 loop { match pc {
 0x60bdd6c => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd70); }
 let mut pc: u32 = 0x60bdd70;
 loop { match pc {
 0x60bdd70 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd74); }
 let mut pc: u32 = 0x60bdd74;
 loop { match pc {
 0x60bdd74 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd78); }
 let mut pc: u32 = 0x60bdd78;
 loop { match pc {
 0x60bdd78 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd7c); }
 let mut pc: u32 = 0x60bdd7c;
 loop { match pc {
 0x60bdd7c => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd80); }
 let mut pc: u32 = 0x60bdd80;
 loop { match pc {
 0x60bdd80 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd84); }
 let mut pc: u32 = 0x60bdd84;
 loop { match pc {
 0x60bdd84 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd88); }
 let mut pc: u32 = 0x60bdd88;
 loop { match pc {
 0x60bdd88 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd8c); }
 let mut pc: u32 = 0x60bdd8c;
 loop { match pc {
 0x60bdd8c => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd90); }
 let mut pc: u32 = 0x60bdd90;
 loop { match pc {
 0x60bdd90 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd94); }
 let mut pc: u32 = 0x60bdd94;
 loop { match pc {
 0x60bdd94 => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd98); }
 let mut pc: u32 = 0x60bdd98;
 loop { match pc {
 0x60bdd98 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdd9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdd9c); }
 let mut pc: u32 = 0x60bdd9c;
 loop { match pc {
 0x60bdd9c => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdda0); }
 let mut pc: u32 = 0x60bdda0;
 loop { match pc {
 0x60bdda0 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdda4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdda4); }
 let mut pc: u32 = 0x60bdda4;
 loop { match pc {
 0x60bdda4 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdda8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdda8); }
 let mut pc: u32 = 0x60bdda8;
 loop { match pc {
 0x60bdda8 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddac); }
 let mut pc: u32 = 0x60bddac;
 loop { match pc {
 0x60bddac => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddb0); }
 let mut pc: u32 = 0x60bddb0;
 loop { match pc {
 0x60bddb0 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddb4); }
 let mut pc: u32 = 0x60bddb4;
 loop { match pc {
 0x60bddb4 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddb8); }
 let mut pc: u32 = 0x60bddb8;
 loop { match pc {
 0x60bddb8 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddbc); }
 let mut pc: u32 = 0x60bddbc;
 loop { match pc {
 0x60bddbc => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddc0); }
 let mut pc: u32 = 0x60bddc0;
 loop { match pc {
 0x60bddc0 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddc4); }
 let mut pc: u32 = 0x60bddc4;
 loop { match pc {
 0x60bddc4 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddc8); }
 let mut pc: u32 = 0x60bddc8;
 loop { match pc {
 0x60bddc8 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddcc); }
 let mut pc: u32 = 0x60bddcc;
 loop { match pc {
 0x60bddcc => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddd0); }
 let mut pc: u32 = 0x60bddd0;
 loop { match pc {
 0x60bddd0 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddd4); }
 let mut pc: u32 = 0x60bddd4;
 loop { match pc {
 0x60bddd4 => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddd8); }
 let mut pc: u32 = 0x60bddd8;
 loop { match pc {
 0x60bddd8 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdddc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdddc); }
 let mut pc: u32 = 0x60bdddc;
 loop { match pc {
 0x60bdddc => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdde0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdde0); }
 let mut pc: u32 = 0x60bdde0;
 loop { match pc {
 0x60bdde0 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdde4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdde4); }
 let mut pc: u32 = 0x60bdde4;
 loop { match pc {
 0x60bdde4 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdde8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdde8); }
 let mut pc: u32 = 0x60bdde8;
 loop { match pc {
 0x60bdde8 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddec); }
 let mut pc: u32 = 0x60bddec;
 loop { match pc {
 0x60bddec => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddf0); }
 let mut pc: u32 = 0x60bddf0;
 loop { match pc {
 0x60bddf0 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddf4); }
 let mut pc: u32 = 0x60bddf4;
 loop { match pc {
 0x60bddf4 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddf8); }
 let mut pc: u32 = 0x60bddf8;
 loop { match pc {
 0x60bddf8 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bddfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bddfc); }
 let mut pc: u32 = 0x60bddfc;
 loop { match pc {
 0x60bddfc => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde00); }
 let mut pc: u32 = 0x60bde00;
 loop { match pc {
 0x60bde00 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde04); }
 let mut pc: u32 = 0x60bde04;
 loop { match pc {
 0x60bde04 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde08); }
 let mut pc: u32 = 0x60bde08;
 loop { match pc {
 0x60bde08 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde0c); }
 let mut pc: u32 = 0x60bde0c;
 loop { match pc {
 0x60bde0c => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde10); }
 let mut pc: u32 = 0x60bde10;
 loop { match pc {
 0x60bde10 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde14); }
 let mut pc: u32 = 0x60bde14;
 loop { match pc {
 0x60bde14 => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde18); }
 let mut pc: u32 = 0x60bde18;
 loop { match pc {
 0x60bde18 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x60bda18u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bde2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bde2e); }
 let mut pc: u32 = 0x60bde2e;
 loop { match pc {
 0x60bde2e => {
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
  cpu.flat_ds(0x60bde6c); cpu.push(0x60bde71); f_060bde18(cpu); if cpu.returned_to != 0x60bde71 { return; }
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
  cpu.flat_ds(0x60bdebf); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bdf9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bdf9d); }
 let mut pc: u32 = 0x60bdf9d;
 loop { match pc {
 0x60bdf9d => {
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
  if cpu.cf { pc = 0x60bdfdc; continue; }
  pc = 0x60bdfbf; continue;
 }
 0x60bdfbf => {
  if (cpu.cf || cpu.zf) { pc = 0x60bdfe9; continue; }
  pc = 0x60bdfc1; continue;
 }
 0x60bdfc1 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x60bdfd2; continue; }
  pc = 0x60bdfc6; continue;
 }
 0x60bdfc6 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bdffe; continue; }
  pc = 0x60bdfc8; continue;
 }
 0x60bdfc8 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x60bdfe9; continue; }
  pc = 0x60bdfcd; continue;
 }
 0x60bdfcd => {
  pc = 0x60be07b; continue;
 }
 0x60bdfd2 => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x60bdff9; continue; }
  pc = 0x60bdfd7; continue;
 }
 0x60bdfd7 => {
  pc = 0x60be07b; continue;
 }
 0x60bdfdc => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x60be07b; continue; }
  pc = 0x60bdfe5; continue;
 }
 0x60bdfe5 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bdff9; continue; }
  pc = 0x60bdfe7; continue;
 }
 0x60bdfe7 => {
  pc = 0x60bdffe; continue;
 }
 0x60bdfe9 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x60bdff6; continue; }
  pc = 0x60bdfef; continue;
 }
 0x60bdfef => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60bdff6; continue;
 }
 0x60bdff6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60bdff9; continue;
 }
 0x60bdff9 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x60bdffe; continue;
 }
 0x60bdffe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60be01f; continue; }
  pc = 0x60be018; continue;
 }
 0x60be018 => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60be01f; continue;
 }
 0x60be01f => {
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
  cpu.push(0x60be047); f_060c486d(cpu); if cpu.returned_to != 0x60be047 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x60be054; continue; }
  pc = 0x60be050; continue;
 }
 0x60be050 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x60be054; continue;
 }
 0x60be054 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x60be05b; continue;
 }
 0x60be05b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x60be067; continue; }
  pc = 0x60be064; continue;
 }
 0x60be064 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be05b; continue;
 }
 0x60be067 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60be070); f_060be087(cpu); if cpu.returned_to != 0x60be070 { return; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60be07b; continue;
 }
 0x60be07b => {
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
pub fn f_060be087(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be087); }
 let mut pc: u32 = 0x60be087;
 loop { match pc {
 0x60be087 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x60be0e4; continue; }
  pc = 0x60be098; continue;
 }
 0x60be098 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x60be0ae; continue; }
  pc = 0x60be09d; continue;
 }
 0x60be09d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x60be0ae; continue; }
  pc = 0x60be0a6; continue;
 }
 0x60be0a6 => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x60be0ae; continue; }
  pc = 0x60be0ab; continue;
 }
 0x60be0ab => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60be098; continue;
 }
 0x60be0ae => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x60be0c4; continue; }
  pc = 0x60be0bf; continue;
 }
 0x60be0bf => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x60be0d0; continue; }
  pc = 0x60be0c4; continue;
 }
 0x60be0c4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x60be0d0; continue; }
  pc = 0x60be0cd; continue;
 }
 0x60be0cd => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60be0c4; continue;
 }
 0x60be0d0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x60be0da; continue; }
  pc = 0x60be0d9; continue;
 }
 0x60be0d9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60be0da; continue;
 }
 0x60be0da => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60be0da; continue; }
  pc = 0x60be0e4; continue;
 }
 0x60be0e4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be0e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be0e6); }
 let mut pc: u32 = 0x60be0e6;
 loop { match pc {
 0x60be0e6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x60be0f0); f_0609c1ef(cpu); if cpu.returned_to != 0x60be0f0 { return; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be0f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be0f4); } r_060be0f4(cpu, 0x60be0f4); }
pub fn f_060be0f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be0f8); } r_060be0f4(cpu, 0x60be0f8); }
pub fn f_060be0fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be0fc); } r_060be0f4(cpu, 0x60be0fc); }
pub fn f_060be0fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be0fe); } r_060be0f4(cpu, 0x60be0fe); }
pub fn f_060be100(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be100); } r_060be0f4(cpu, 0x60be100); }
pub fn f_060be102(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be102); } r_060be0f4(cpu, 0x60be102); }
pub fn f_060be14e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be14e); } r_060be0f4(cpu, 0x60be14e); }
pub fn f_060be150(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be150); } r_060be0f4(cpu, 0x60be150); }
pub fn f_060be154(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be154); } r_060be0f4(cpu, 0x60be154); }
pub fn f_060be156(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be156); } r_060be0f4(cpu, 0x60be156); }
fn r_060be0f4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60be0f4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60be0f8; continue;
 }
 0x60be0f8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60be0fc; continue;
 }
 0x60be0fc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60be0fe; continue;
 }
 0x60be0fe => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60be100; continue;
 }
 0x60be100 => {
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
  cpu.flat_ds(0x60be162); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x60be102 => {
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
  cpu.flat_ds(0x60be162); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x60be14e => {
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
  cpu.flat_ds(0x60be162); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x60be150 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60be154; continue;
 }
 0x60be154 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60be156; continue;
 }
 0x60be156 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x60be162); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be19e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be19e); } r_060be19e(cpu, 0x60be19e); }
pub fn f_060be1a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be1a2); } r_060be19e(cpu, 0x60be1a2); }
pub fn f_060be1ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be1ac); } r_060be19e(cpu, 0x60be1ac); }
pub fn f_060be20c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be20c); } r_060be19e(cpu, 0x60be20c); }
pub fn f_060be26c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be26c); } r_060be19e(cpu, 0x60be26c); }
fn r_060be19e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60be19e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60be1a1; continue;
 }
 0x60be1a1 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60be1a2; continue;
 }
 0x60be1a2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x60be1a9; continue;
 }
 0x60be1a9 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let a = cpu.ebx; let b = cpu.ebx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x60be1ac; continue;
 }
 0x60be1ac => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0xffffffe7u32; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x60be1a1; continue; }
  pc = 0x60be1ba; continue;
 }
 0x60be1ba => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60be1a9; continue; }
  pc = 0x60be1c2; continue;
 }
 0x60be1c2 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  {  }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0xa8060be7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.eax as u8) & ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xc4060be7u32; cpu.edx = v; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60be1e8, "Int3");
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60be1ec, "Aam");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.fcom(a, b);  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out(0xe7u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.eax as u8) & ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  pc = 0x60be205; continue;
 }
 0x60be201 => {
  cpu.push(0x60be206); cpu.call(0xee19e811); if cpu.returned_to != 0x60be206 { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  pc = 0x60be209; continue;
 }
 0x60be205 => {
  cpu.push(0x60be20a); cpu.call(0xee22e815); if cpu.returned_to != 0x60be20a { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x60be20c; continue;
 }
 0x60be209 => {
  cpu.push(0x60be20e); cpu.call(0xed69e819); if cpu.returned_to != 0x60be20e { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0xffffffe7u32; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x60be201; continue; }
  pc = 0x60be21a; continue;
 }
 0x60be20c => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0xffffffe7u32; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x60be201; continue; }
  pc = 0x60be21a; continue;
 }
 0x60be21a => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x60be205; continue; }
  pc = 0x60be21e; continue;
 }
 0x60be21e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60be209; continue; }
  pc = 0x60be222; continue;
 }
 0x60be222 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  {  }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0xa8060be7u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = 0xe7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xc4060be7u32; cpu.edx = v; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60be248, "Int3");
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60be24c, "Aam");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.fcom(a, b);  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out(0xe7u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.cf = false; }
  { cpu.port_out(0xbu16, 4, cpu.eax as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  cpu.push(0x60be26a); cpu.call(0xee22e875); if cpu.returned_to != 0x60be26a { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x60be26c; continue;
 }
 0x60be26c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.push(0x60be272); cpu.call(0xef5ae87d); if cpu.returned_to != 0x60be272 { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  pc = 0x60be276; continue;
 }
 0x60be276 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xb7060bebu32); cpu.eax = v; }
  pc = 0x60be28a; continue;
 }
 0x60be28a => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60be276; continue; }
  pc = 0x60be28e; continue;
 }
 0x60be28e => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x60be296); cpu.call(0xf055e8a1); if cpu.returned_to != 0x60be296 { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x60be29e); cpu.call(0xf1e5e8a9); if cpu.returned_to != 0x60be29e { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x60be2a6); cpu.call(0xf442e8b1); if cpu.returned_to != 0x60be2a6 { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x60be2ae); cpu.call(0xeea6e8b9); if cpu.returned_to != 0x60be2ae { return; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xeb57060bu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.df = true; }
  pc = 0x60be2ca; continue;
 }
 0x60be2ca => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0xe8u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x60be2d7); cpu.call(0x0); if cpu.returned_to != 0x60be2d7 { return; }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x60be2e6; continue;
 }
 0x60be2e6 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = cpu.flags(); cpu.push(v); }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.call(0xf0ace901); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be46c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be46c); }
 let mut pc: u32 = 0x60be46c;
 loop { match pc {
 0x60be46c => {
  {  }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x60be484); cpu.push(0x60be489); f_060be500(cpu); if cpu.returned_to != 0x60be489 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x60be48c); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be4dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be4dc); }
 let mut pc: u32 = 0x60be4dc;
 loop { match pc {
 0x60be4dc => {
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
  cpu.flat_ds(0x60be4f3); cpu.push(0x60be4f8); f_060be500(cpu); if cpu.returned_to != 0x60be4f8 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.flat_ds(0x60be4fe); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be500(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60be500); } r_060be500(cpu, 0x60be500); }
pub fn f_060bf180(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bf180); } r_060be500(cpu, 0x60bf180); }
pub fn f_060bf41c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bf41c); } r_060be500(cpu, 0x60bf41c); }
pub fn f_060bf63e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bf63e); } r_060be500(cpu, 0x60bf63e); }
pub fn f_060bf6c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bf6c7); } r_060be500(cpu, 0x60bf6c7); }
pub fn f_060bfa85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bfa85); } r_060be500(cpu, 0x60bfa85); }
fn r_060be500(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60be500 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x60be58c; continue; }
  pc = 0x60be51a; continue;
 }
 0x60be51a => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xf8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if !cpu.zf { pc = 0x60be633; continue; }
  pc = 0x60be528; continue;
 }
 0x60be528 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xc0u8); }
  if !cpu.cf { pc = 0x60be5e9; continue; }
  pc = 0x60be539; continue;
 }
 0x60be539 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x1807u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x60be1acu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.flat_ds(0x60be54a); cpu.push(0x60be552); cpu.call(t); if cpu.returned_to != 0x60be552 { return; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x38u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x60be561); cpu.push(0x60be566); f_060adc60(cpu); if cpu.returned_to != 0x60be566 { return; }
  { let v: u32 = 0x617c788u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = cpu.r32(0x60be26cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60be586 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60be51a; continue; }
  pc = 0x60be58c; continue;
 }
 0x60be58c => {
  if cpu.zf { pc = 0x60be5b2; continue; }
  pc = 0x60be58e; continue;
 }
 0x60be58e => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x60be5d0; continue; }
  pc = 0x60be592; continue;
 }
 0x60be592 => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x60be5b5; continue; }
  pc = 0x60be596; continue;
 }
 0x60be596 => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x60be5c7; continue; }
  pc = 0x60be59a; continue;
 }
 0x60be59a => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x60be5be; continue; }
  pc = 0x60be59e; continue;
 }
 0x60be59e => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x60be5d9; continue; }
  pc = 0x60be5a2; continue;
 }
 0x60be5a2 => {
  { cpu.sub8((cpu.eax as u8), 0x65u8); }
  if cpu.zf { pc = 0x60be5e1; continue; }
  pc = 0x60be5a6; continue;
 }
 0x60be5a6 => {
  { cpu.sub8((cpu.eax as u8), 0x66u8); }
  if cpu.zf { pc = 0x60be5b2; continue; }
  pc = 0x60be5aa; continue;
 }
 0x60be5aa => {
  { cpu.sub8((cpu.eax as u8), 0x67u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60be51a; continue; }
  pc = 0x60be5b2; continue;
 }
 0x60be5b2 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5b5 => {
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5be => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5c7 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5d0 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5d9 => {
  { let v: u16 = (cpu.segments[4] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5e1 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60be586; continue;
 }
 0x60be5e9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x60be5ec); cpu.push(0x60be5f1); f_060adc60(cpu); if cpu.returned_to != 0x60be5f1 { return; }
  { let v: u32 = 0x617c788u32; cpu.ebp = v; }
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
  if !cpu.zf { pc = 0x60be61c; continue; }
  pc = 0x60be618; continue;
 }
 0x60be618 => {
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60be61c; continue;
 }
 0x60be61c => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = cpu.r32(0x60be36cu32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x60be633 => {
  cpu.flat_ds(0x60be633); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60be6a2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x60be19eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60be6b9; continue;
 }
 0x60be6b9 => {
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be6f4 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x60be19eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be858 => {
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be85d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be863); cpu.push(0x60be868); f_060c044e(cpu); if cpu.returned_to != 0x60be868 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be873); cpu.push(0x60be878); f_060bfc24(cpu); if cpu.returned_to != 0x60be878 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be87c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be882); cpu.push(0x60be887); f_060c044e(cpu); if cpu.returned_to != 0x60be887 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be892); cpu.push(0x60be897); f_060c0058(cpu); if cpu.returned_to != 0x60be897 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be89b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8a1); cpu.push(0x60be8a6); f_060c044e(cpu); if cpu.returned_to != 0x60be8a6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8ad); cpu.push(0x60be8b2); f_060bfdea(cpu); if cpu.returned_to != 0x60be8b2 { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60be8b7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8bd); cpu.push(0x60be8c2); f_060c044e(cpu); if cpu.returned_to != 0x60be8c2 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8c9); cpu.push(0x60be8ce); f_060bfdea(cpu); if cpu.returned_to != 0x60be8ce { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60be8d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8d9); cpu.push(0x60be8de); f_060c044e(cpu); if cpu.returned_to != 0x60be8de { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be8e9); cpu.push(0x60be8ee); f_060bfc16(cpu); if cpu.returned_to != 0x60be8ee { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be8f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be8f8); cpu.push(0x60be8fd); f_060c044e(cpu); if cpu.returned_to != 0x60be8fd { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be908); cpu.push(0x60be90d); f_060bfc16(cpu); if cpu.returned_to != 0x60be90d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be911 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be917); cpu.push(0x60be91c); f_060c044e(cpu); if cpu.returned_to != 0x60be91c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be927); cpu.push(0x60be92c); f_060bfe83(cpu); if cpu.returned_to != 0x60be92c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be930 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60be936); cpu.push(0x60be93b); f_060c044e(cpu); if cpu.returned_to != 0x60be93b { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60be946); cpu.push(0x60be94b); f_060bfe83(cpu); if cpu.returned_to != 0x60be94b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60be94f => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60be97f); cpu.push(0x60be984); f_060c044e(cpu); if cpu.returned_to != 0x60be984 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be989 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60be98d); cpu.push(0x60be992); f_060c0308(cpu); if cpu.returned_to != 0x60be992 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be99a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60be99e); cpu.push(0x60be9a3); f_060c0308(cpu); if cpu.returned_to != 0x60be9a3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60be9ab => {
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
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60be9c9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x300u16); }
  if !cpu.zf { pc = 0x60be9e8; continue; }
  pc = 0x60be9dc; continue;
 }
 0x60be9dc => {
  { let v: u32 = 0x60be500u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60bea04; continue;
 }
 0x60be9e8 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if !cpu.zf { pc = 0x60be9fa; continue; }
  pc = 0x60be9ee; continue;
 }
 0x60be9ee => {
  { let v: u32 = 0x60be670u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60bea04; continue;
 }
 0x60be9fa => {
  { let v: u32 = 0x60be634u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60bea04; continue;
 }
 0x60bea04 => {
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bea09 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bea1d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bea2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea31); cpu.push(0x60bea36); f_060c0242(cpu); if cpu.returned_to != 0x60bea36 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bea41); cpu.push(0x60bea46); f_060bfc24(cpu); if cpu.returned_to != 0x60bea46 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bea4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea50); cpu.push(0x60bea55); f_060c0242(cpu); if cpu.returned_to != 0x60bea55 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bea60); cpu.push(0x60bea65); f_060c0058(cpu); if cpu.returned_to != 0x60bea65 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bea69 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea6f); cpu.push(0x60bea74); f_060c0242(cpu); if cpu.returned_to != 0x60bea74 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea7b); cpu.push(0x60bea80); f_060bfdea(cpu); if cpu.returned_to != 0x60bea80 { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bea85 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea8b); cpu.push(0x60bea90); f_060c0242(cpu); if cpu.returned_to != 0x60bea90 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bea97); cpu.push(0x60bea9c); f_060bfdea(cpu); if cpu.returned_to != 0x60bea9c { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60beaa1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beaa7); cpu.push(0x60beaac); f_060c0242(cpu); if cpu.returned_to != 0x60beaac { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beab7); cpu.push(0x60beabc); f_060bfc16(cpu); if cpu.returned_to != 0x60beabc { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beac0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beac6); cpu.push(0x60beacb); f_060c0242(cpu); if cpu.returned_to != 0x60beacb { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bead6); cpu.push(0x60beadb); f_060bfc16(cpu); if cpu.returned_to != 0x60beadb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beadf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beae5); cpu.push(0x60beaea); f_060c0242(cpu); if cpu.returned_to != 0x60beaea { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beaf5); cpu.push(0x60beafa); f_060bfe83(cpu); if cpu.returned_to != 0x60beafa { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beafe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beb04); cpu.push(0x60beb09); f_060c0242(cpu); if cpu.returned_to != 0x60beb09 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beb14); cpu.push(0x60beb19); f_060bfe83(cpu); if cpu.returned_to != 0x60beb19 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beb1d => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60beb4d); cpu.push(0x60beb52); f_060c0242(cpu); if cpu.returned_to != 0x60beb52 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60beb57 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60beb71); cpu.push(0x60beb76); f_060c0513(cpu); if cpu.returned_to != 0x60beb76 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60beb7a); cpu.push(0x60beb7f); f_060c01d3(cpu); if cpu.returned_to != 0x60beb7f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60beb93 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60beba1); cpu.push(0x60beba6); f_060c0513(cpu); if cpu.returned_to != 0x60beba6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bebaa); cpu.push(0x60bebaf); f_060c01d3(cpu); if cpu.returned_to != 0x60bebaf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bebb7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bebc1); cpu.push(0x60bebc6); f_060c0288(cpu); if cpu.returned_to != 0x60bebc6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bebd1); cpu.push(0x60bebd6); f_060bfc24(cpu); if cpu.returned_to != 0x60bebd6 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bebda => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bebe4); cpu.push(0x60bebe9); f_060c0288(cpu); if cpu.returned_to != 0x60bebe9 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bebf4); cpu.push(0x60bebf9); f_060c0058(cpu); if cpu.returned_to != 0x60bebf9 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bebfd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bec07); cpu.push(0x60bec0c); f_060c0288(cpu); if cpu.returned_to != 0x60bec0c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bec13); cpu.push(0x60bec18); f_060bfdea(cpu); if cpu.returned_to != 0x60bec18 { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bec1d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bec27); cpu.push(0x60bec2c); f_060c0288(cpu); if cpu.returned_to != 0x60bec2c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bec33); cpu.push(0x60bec38); f_060bfdea(cpu); if cpu.returned_to != 0x60bec38 { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bec3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bec47); cpu.push(0x60bec4c); f_060c0288(cpu); if cpu.returned_to != 0x60bec4c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bec57); cpu.push(0x60bec5c); f_060bfc16(cpu); if cpu.returned_to != 0x60bec5c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bec60 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bec6a); cpu.push(0x60bec6f); f_060c0288(cpu); if cpu.returned_to != 0x60bec6f { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bec7a); cpu.push(0x60bec7f); f_060bfc16(cpu); if cpu.returned_to != 0x60bec7f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bec83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60bec8d); cpu.push(0x60bec92); f_060c0288(cpu); if cpu.returned_to != 0x60bec92 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bec9d); cpu.push(0x60beca2); f_060bfe83(cpu); if cpu.returned_to != 0x60beca2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beca6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x60becb0); cpu.push(0x60becb5); f_060c0288(cpu); if cpu.returned_to != 0x60becb5 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60becc0); cpu.push(0x60becc5); f_060bfe83(cpu); if cpu.returned_to != 0x60becc5 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60becc9 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bed10 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bed44); cpu.push(0x60bed49); f_060c0288(cpu); if cpu.returned_to != 0x60bed49 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bed4e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bed52); cpu.push(0x60bed57); f_060c0387(cpu); if cpu.returned_to != 0x60bed57 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bed63 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bed81 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bed85); cpu.push(0x60bed8a); f_060c0387(cpu); if cpu.returned_to != 0x60bed8a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bed96 => {
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
  pc = 0x60bedc4; continue;
 }
 0x60bedc4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x60bedc4; continue; }
  pc = 0x60bedd6; continue;
 }
 0x60bedd6 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60beddd => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x60bedef; continue;
 }
 0x60bedef => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x60bedef; continue; }
  pc = 0x60bee01; continue;
 }
 0x60bee01 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60bf6f3; continue;
 }
 0x60bee08 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bee16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee1e); cpu.push(0x60bee23); f_060c0242(cpu); if cpu.returned_to != 0x60bee23 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bee2e); cpu.push(0x60bee33); f_060bfc24(cpu); if cpu.returned_to != 0x60bee33 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bee37 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee3f); cpu.push(0x60bee44); f_060c0242(cpu); if cpu.returned_to != 0x60bee44 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bee4f); cpu.push(0x60bee54); f_060c0058(cpu); if cpu.returned_to != 0x60bee54 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bee58 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee60); cpu.push(0x60bee65); f_060c0242(cpu); if cpu.returned_to != 0x60bee65 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee6c); cpu.push(0x60bee71); f_060bfdea(cpu); if cpu.returned_to != 0x60bee71 { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bee76 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee7e); cpu.push(0x60bee83); f_060c0242(cpu); if cpu.returned_to != 0x60bee83 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee8a); cpu.push(0x60bee8f); f_060bfdea(cpu); if cpu.returned_to != 0x60bee8f { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bee94 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bee9c); cpu.push(0x60beea1); f_060c0242(cpu); if cpu.returned_to != 0x60beea1 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beeac); cpu.push(0x60beeb1); f_060bfc16(cpu); if cpu.returned_to != 0x60beeb1 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beeb5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beebd); cpu.push(0x60beec2); f_060c0242(cpu); if cpu.returned_to != 0x60beec2 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beecd); cpu.push(0x60beed2); f_060bfc16(cpu); if cpu.returned_to != 0x60beed2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beed6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beede); cpu.push(0x60beee3); f_060c0242(cpu); if cpu.returned_to != 0x60beee3 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60beeee); cpu.push(0x60beef3); f_060bfe83(cpu); if cpu.returned_to != 0x60beef3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60beef7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60beeff); cpu.push(0x60bef04); f_060c0242(cpu); if cpu.returned_to != 0x60bef04 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bef0f); cpu.push(0x60bef14); f_060bfe83(cpu); if cpu.returned_to != 0x60bef14 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bef18 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bef4a); cpu.push(0x60bef4f); f_060c0242(cpu); if cpu.returned_to != 0x60bef4f { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bef54 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60bef6e); cpu.push(0x60bef73); f_060c0513(cpu); if cpu.returned_to != 0x60bef73 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bef77); cpu.push(0x60bef7c); f_060c01d3(cpu); if cpu.returned_to != 0x60bef7c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x60bef87; continue; }
  pc = 0x60bef83; continue;
 }
 0x60bef83 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60bef87; continue;
 }
 0x60bef87 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bef9c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60befaa); cpu.push(0x60befaf); f_060c0513(cpu); if cpu.returned_to != 0x60befaf { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60befb3); cpu.push(0x60befb8); f_060c01d3(cpu); if cpu.returned_to != 0x60befb8 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x60befc3; continue; }
  pc = 0x60befbf; continue;
 }
 0x60befbf => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60befc3; continue;
 }
 0x60befc3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60befcc => {
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60befd1 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf001); cpu.push(0x60bf006); f_060c0253(cpu); if cpu.returned_to != 0x60bf006 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf00b => {
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bf010 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60bf01e); cpu.push(0x60bf023); f_060c0513(cpu); if cpu.returned_to != 0x60bf023 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf027); cpu.push(0x60bf02c); f_060c01d8(cpu); if cpu.returned_to != 0x60bf02c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bf034 => {
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf039 => {
  cpu.poll(); pc = cpu.r32(0x60bf041u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf061 => {
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf066 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf072); cpu.push(0x60bf077); f_060bfc24(cpu); if cpu.returned_to != 0x60bf077 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf07b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf087); cpu.push(0x60bf08c); f_060c0058(cpu); if cpu.returned_to != 0x60bf08c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf090 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf098); cpu.push(0x60bf09d); f_060bfdea(cpu); if cpu.returned_to != 0x60bf09d { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bf0a2 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf0aa); cpu.push(0x60bf0af); f_060bfdea(cpu); if cpu.returned_to != 0x60bf0af { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bf0b4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf0c0); cpu.push(0x60bf0c5); f_060bfc16(cpu); if cpu.returned_to != 0x60bf0c5 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf0c9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf0d5); cpu.push(0x60bf0da); f_060bfc16(cpu); if cpu.returned_to != 0x60bf0da { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf0de => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf0ea); cpu.push(0x60bf0ef); f_060bfe83(cpu); if cpu.returned_to != 0x60bf0ef { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf0f3 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf0ff); cpu.push(0x60bf104); f_060bfe83(cpu); if cpu.returned_to != 0x60bf104 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf108 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf150 => {
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
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf178 => {
  cpu.poll(); pc = cpu.r32(0x60bf180u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf180 => {
  { let v: u8 = cpu.r8(0xad060bf1u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x60bf185, "Int1");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = 0xeb060bf1u32; cpu.edx = v; }
  cpu.unimplemented(0x60bf195, "Int1");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x60bf1a0; continue;
 }
 0x60bf1a0 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf1ad => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf1ba => {
  { let v: u32 = cpu.r32(0x60be0f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x60be0f8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be0fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x60bf1e1); cpu.push(0x60bf1e6); f_060bfdea(cpu); if cpu.returned_to != 0x60bf1e6 { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bf1eb => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf1ef); cpu.push(0x60bf1f4); f_060c099a(cpu); if cpu.returned_to != 0x60bf1f4 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf1f9 => {
  cpu.poll(); pc = cpu.r32(0x60bf201u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf221 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf264 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xcd1b8afeu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xd49a784bu32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf2a9 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x5c17f0bcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb8aa3b29u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf2ee => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xc900daa2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf333 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xfbcff799u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x9a209a84u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffdu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf378 => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xd1cf79acu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb17217f7u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffeu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf3bd => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be150u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf414 => {
  cpu.poll(); pc = cpu.r32(0x60bf41cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf41c => {
  { cpu.sub8((cpu.eax as u8), 0xf4u8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x60bf421, "Hlt");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  {  }
  cpu.unimplemented(0x60bf425, "Hlt");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.movs(4, false); }
  {  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  {  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  {  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.st(0) / cpu.st(4); cpu.set_st(4, v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.df = true; }
  cpu.unimplemented(0x60bf439, "Hlt");
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  pc = 0x60bf43c; continue;
 }
 0x60bf43c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf440); cpu.push(0x60bf445); f_060c0fc0(cpu); if cpu.returned_to != 0x60bf445 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf449 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf487); cpu.push(0x60bf48c); f_060c0aa1(cpu); if cpu.returned_to != 0x60bf48c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf490 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf494); cpu.push(0x60bf499); f_060c0f8a(cpu); if cpu.returned_to != 0x60bf499 { return; }
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf4dc => {
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf4fd => {
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf51e => {
  cpu.poll(); pc = cpu.r32(0x60bf526u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf546 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf54e); cpu.push(0x60bf553); f_060c08b9(cpu); if cpu.returned_to != 0x60bf553 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x60be1a2u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf570 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf5ae); cpu.push(0x60bf5b3); f_060c0a88(cpu); if cpu.returned_to != 0x60bf5b3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf5b7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf5bb); cpu.push(0x60bf5c0); f_060c1194(cpu); if cpu.returned_to != 0x60bf5c0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf5c4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf5d4); cpu.push(0x60bf5d9); f_060c0f73(cpu); if cpu.returned_to != 0x60bf5d9 { return; }
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf612); cpu.push(0x60bf617); f_060c0f7d(cpu); if cpu.returned_to != 0x60bf617 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf61c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf620); cpu.push(0x60bf625); f_060c0f73(cpu); if cpu.returned_to != 0x60bf625 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf629 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x60bf62d); cpu.push(0x60bf632); f_060c0f7d(cpu); if cpu.returned_to != 0x60bf632 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf636 => {
  cpu.poll(); pc = cpu.r32(0x60bf63eu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf63e => {
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x6u8); }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x60bf65e; continue;
 }
 0x60bf65e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf6b5); cpu.push(0x60bf6ba); f_060bfdea(cpu); if cpu.returned_to != 0x60bf6ba { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bf6bf => {
  cpu.poll(); pc = cpu.r32(0x60bf6c7u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf6c7 => {
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.port_out(0xf6u16, 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x6u8); }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x60bf6e7; continue;
 }
 0x60bf6e7 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf6f3 => {
  { let v: u16 = 0x33fu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x60be500u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf719 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf725); cpu.push(0x60bf72a); f_060bfc24(cpu); if cpu.returned_to != 0x60bf72a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf72e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf73a); cpu.push(0x60bf73f); f_060c0058(cpu); if cpu.returned_to != 0x60bf73f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf743 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf74f); cpu.push(0x60bf754); f_060bfc16(cpu); if cpu.returned_to != 0x60bf754 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf758 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf764); cpu.push(0x60bf769); f_060bfc16(cpu); if cpu.returned_to != 0x60bf769 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf76d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf779); cpu.push(0x60bf77e); f_060bfe83(cpu); if cpu.returned_to != 0x60bf77e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf782 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf78e); cpu.push(0x60bf793); f_060bfe83(cpu); if cpu.returned_to != 0x60bf793 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf797 => {
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf7b6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bf7e2 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf7ea); cpu.push(0x60bf7ef); f_060bfdea(cpu); if cpu.returned_to != 0x60bf7ef { return; }
  cpu.poll(); pc = 0x60be6f4; continue;
 }
 0x60bf7f4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf7fc); cpu.push(0x60bf801); f_060bfdea(cpu); if cpu.returned_to != 0x60bf801 { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bf806 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be6b9; continue;
 }
 0x60bf832 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf83e); cpu.push(0x60bf843); f_060bfc24(cpu); if cpu.returned_to != 0x60bf843 { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf87d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf889); cpu.push(0x60bf88e); f_060c0058(cpu); if cpu.returned_to != 0x60bf88e { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf8c8 => {
  cpu.poll(); pc = cpu.r32(0x60bf8d0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bf8f0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bf947); cpu.push(0x60bf94c); f_060bfdea(cpu); if cpu.returned_to != 0x60bf94c { return; }
  cpu.poll(); pc = 0x60be6a2; continue;
 }
 0x60bf951 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf95d); cpu.push(0x60bf962); f_060bfc16(cpu); if cpu.returned_to != 0x60bf962 { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf99c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf9a8); cpu.push(0x60bf9ad); f_060bfc16(cpu); if cpu.returned_to != 0x60bf9ad { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bf9e7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bf9f3); cpu.push(0x60bf9f8); f_060bfe83(cpu); if cpu.returned_to != 0x60bf9f8 { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bfa32 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x60bfa3e); cpu.push(0x60bfa43); f_060bfe83(cpu); if cpu.returned_to != 0x60bfa43 { return; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bfa7d => {
  cpu.poll(); pc = cpu.r32(0x60bfa85u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60bfa85 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.df = false; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x60bfaa5; continue;
 }
 0x60bfaa5 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x60bfae5); cpu.push(0x60bfaea); f_060c06a0(cpu); if cpu.returned_to != 0x60bfaea { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bfaee => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x60be100u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x60be102u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x60be14eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
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
  cpu.flat_ds(0x60bfb49); cpu.push(0x60bfb4e); f_060c01d3(cpu); if cpu.returned_to != 0x60bfb4e { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bfb53 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x60bfb5b); cpu.push(0x60bfb60); f_060c08b9(cpu); if cpu.returned_to != 0x60bfb60 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x60be1a2u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x60bfb7d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60bfb8b); cpu.push(0x60bfb90); f_060c0513(cpu); if cpu.returned_to != 0x60bfb90 { return; }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bfb95 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x60be0feu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x60bfba7); cpu.push(0x60bfbac); f_060c01d3(cpu); if cpu.returned_to != 0x60bfbac { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x60bfbd4; continue; }
  pc = 0x60bfbb9; continue;
 }
 0x60bfbb9 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x60bfbc5; continue; }
  pc = 0x60bfbbd; continue;
 }
 0x60bfbbd => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60bfbd2; continue;
 }
 0x60bfbc5 => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60bfbd2; continue;
 }
 0x60bfbd2 => {
  pc = 0x60bfbfa; continue;
 }
 0x60bfbd4 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x60bfbfa; continue; }
  pc = 0x60bfbea; continue;
 }
 0x60bfbea => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x60bfbf6; continue; }
  pc = 0x60bfbee; continue;
 }
 0x60bfbee => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60bfbfa; continue;
 }
 0x60bfbf6 => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x60bfbfa; continue;
 }
 0x60bfbfa => {
  { let v: u16 = (cpu.edx as u16) | (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 0x60bfc06 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60be500; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be634); }
 let mut pc: u32 = 0x60be634;
 loop { match pc {
 0x60be634 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 25u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x60be662; continue; }
  pc = 0x60be645; continue;
 }
 0x60be645 => {
  if !cpu.zf { pc = 0x60be64e; continue; }
  pc = 0x60be647; continue;
 }
 0x60be647 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x60be64e; continue; }
  pc = 0x60be64c; continue;
 }
 0x60be64c => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x60be64e; continue;
 }
 0x60be64e => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x100u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x60be662; continue; }
  pc = 0x60be657; continue;
 }
 0x60be657 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60be662; continue;
 }
 0x60be662 => {
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_060be500(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be670); }
 let mut pc: u32 = 0x60be670;
 loop { match pc {
 0x60be670 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xfffff800u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 22u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x60be69b; continue; }
  pc = 0x60be680; continue;
 }
 0x60be680 => {
  if !cpu.zf { pc = 0x60be684; continue; }
  pc = 0x60be682; continue;
 }
 0x60be682 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x60be684; continue;
 }
 0x60be684 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x800u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x60be69b; continue; }
  pc = 0x60be690; continue;
 }
 0x60be690 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60be69b; continue;
 }
 0x60be69b => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_060be500(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be710); }
 let mut pc: u32 = 0x60be710;
 loop { match pc {
 0x60be710 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60be75d; continue; }
  pc = 0x60be71a; continue;
 }
 0x60be71a => {
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
  let t = cpu.r32(0x617b869u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60be737); cpu.push(0x60be73d); cpu.call(t); if cpu.returned_to != 0x60be73d { return; }
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
  pc = 0x60be75d; continue;
 }
 0x60be75d => {
  cpu.flat_ds(0x60be75d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be75e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be75e); }
 let mut pc: u32 = 0x60be75e;
 loop { match pc {
 0x60be75e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be764(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be764); }
 let mut pc: u32 = 0x60be764;
 loop { match pc {
 0x60be764 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be76a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be76a); }
 let mut pc: u32 = 0x60be76a;
 loop { match pc {
 0x60be76a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be770); }
 let mut pc: u32 = 0x60be770;
 loop { match pc {
 0x60be770 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be776(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be776); }
 let mut pc: u32 = 0x60be776;
 loop { match pc {
 0x60be776 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be77e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be77e); }
 let mut pc: u32 = 0x60be77e;
 loop { match pc {
 0x60be77e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be784(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be784); }
 let mut pc: u32 = 0x60be784;
 loop { match pc {
 0x60be784 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be78a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be78a); }
 let mut pc: u32 = 0x60be78a;
 loop { match pc {
 0x60be78a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be790); }
 let mut pc: u32 = 0x60be790;
 loop { match pc {
 0x60be790 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be798); }
 let mut pc: u32 = 0x60be798;
 loop { match pc {
 0x60be798 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7a0); }
 let mut pc: u32 = 0x60be7a0;
 loop { match pc {
 0x60be7a0 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7a8); }
 let mut pc: u32 = 0x60be7a8;
 loop { match pc {
 0x60be7a8 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7b0); }
 let mut pc: u32 = 0x60be7b0;
 loop { match pc {
 0x60be7b0 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7ba); }
 let mut pc: u32 = 0x60be7ba;
 loop { match pc {
 0x60be7ba => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7c4); }
 let mut pc: u32 = 0x60be7c4;
 loop { match pc {
 0x60be7c4 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7cc); }
 let mut pc: u32 = 0x60be7cc;
 loop { match pc {
 0x60be7cc => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7d4); }
 let mut pc: u32 = 0x60be7d4;
 loop { match pc {
 0x60be7d4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7dd); }
 let mut pc: u32 = 0x60be7dd;
 loop { match pc {
 0x60be7dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7e6); }
 let mut pc: u32 = 0x60be7e6;
 loop { match pc {
 0x60be7e6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7ef); }
 let mut pc: u32 = 0x60be7ef;
 loop { match pc {
 0x60be7ef => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be7f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be7f8); }
 let mut pc: u32 = 0x60be7f8;
 loop { match pc {
 0x60be7f8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be803(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be803); }
 let mut pc: u32 = 0x60be803;
 loop { match pc {
 0x60be803 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be80e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be80e); }
 let mut pc: u32 = 0x60be80e;
 loop { match pc {
 0x60be80e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be817(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be817); }
 let mut pc: u32 = 0x60be817;
 loop { match pc {
 0x60be817 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060be820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60be820); }
 let mut pc: u32 = 0x60be820;
 loop { match pc {
 0x60be820 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x18u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x60be20cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x60be83b); cpu.call(t); if cpu.returned_to != 0x60be83b { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x38u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x60be856; continue; }
  pc = 0x60be847; continue;
 }
 0x60be847 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  pc = 0x60be856; continue;
 }
 0x60be856 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bf201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bf201); }
 let mut pc: u32 = 0x60bf201;
 loop { match pc {
 0x60bf201 => {
  { let v: u32 = cpu.edx & cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[4])); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xee060bf2u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x60bf20a; continue; }
  pc = 0x60bf217; continue;
 }
 0x60bf20a => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x60bf20a; continue; }
  pc = 0x60bf217; continue;
 }
 0x60bf217 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = 0x34060bf3u32; cpu.ebp = v; }
  cpu.call(0x60bf21e); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bfc16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bfc16); } r_060bfc16(cpu, 0x60bfc16); }
pub fn f_060bfc24(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bfc24); } r_060bfc16(cpu, 0x60bfc24); }
fn r_060bfc16(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60bfc16 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bfc2b; continue;
 }
 0x60bfc24 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bfc2b; continue;
 }
 0x60bfc2b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x60bfc41); f_060bfc7e(cpu); if cpu.returned_to != 0x60bfc41 { return; }
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
pub fn f_060bfc4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bfc4e); }
 let mut pc: u32 = 0x60bfc4e;
 loop { match pc {
 0x60bfc4e => {
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
  cpu.push(0x60bfc6f); f_060bfc7e(cpu); if cpu.returned_to != 0x60bfc6f { return; }
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
pub fn f_060bfc7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bfc7e); }
 let mut pc: u32 = 0x60bfc7e;
 loop { match pc {
 0x60bfc7e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60bfca1; continue; }
  pc = 0x60bfc82; continue;
 }
 0x60bfc82 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60bfca1; continue; }
  pc = 0x60bfc86; continue;
 }
 0x60bfc86 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bfc9e; continue; }
  pc = 0x60bfc8b; continue;
 }
 0x60bfc8b => {
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16) | (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60bfc9d; continue; }
  pc = 0x60bfc9b; continue;
 }
 0x60bfc9b => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x60bfc9d; continue;
 }
 0x60bfc9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfc9e => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bfca1; continue;
 }
 0x60bfca1 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60bfcb2; continue; }
  pc = 0x60bfca5; continue;
 }
 0x60bfca5 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x60bfcb2; continue; }
  pc = 0x60bfca9; continue;
 }
 0x60bfca9 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x60bfcb2; continue; }
  pc = 0x60bfcb1; continue;
 }
 0x60bfcb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfcb2 => {
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
  if cpu.zf { pc = 0x60bfd01; continue; }
  pc = 0x60bfce3; continue;
 }
 0x60bfce3 => {
  if !cpu.cf { pc = 0x60bfced; continue; }
  pc = 0x60bfce5; continue;
 }
 0x60bfce5 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.esi; let b = cpu.edx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x60bfced; continue;
 }
 0x60bfced => {
  { cpu.sub16((cpu.ecx as u16), 0x40u16); }
  if (cpu.cf || cpu.zf) { pc = 0x60bfd01; continue; }
  pc = 0x60bfcf3; continue;
 }
 0x60bfcf3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfd01 => {
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x60bfd16; continue; }
  pc = 0x60bfd07; continue;
 }
 0x60bfd07 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x60bfd16; continue;
 }
 0x60bfd16 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x60bfd49; continue; }
  pc = 0x60bfd1d; continue;
 }
 0x60bfd1d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x60bfd3b; continue; }
  pc = 0x60bfd25; continue;
 }
 0x60bfd25 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x60bfd37; continue; }
  pc = 0x60bfd33; continue;
 }
 0x60bfd33 => {
  { let v: u32 = cpu.edi | cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x60bfd37; continue;
 }
 0x60bfd37 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x60bfd3b; continue;
 }
 0x60bfd3b => {
  { let v: u32 = cpu.shrd32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.edi | cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x60bfd49; continue;
 }
 0x60bfd49 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x60bfd7a; continue; }
  pc = 0x60bfd52; continue;
 }
 0x60bfd52 => {
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x60bfd6b; continue; }
  pc = 0x60bfd57; continue;
 }
 0x60bfd57 => {
  { cpu.and32(cpu.edi, 0x7fffffffu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60bfd6b; continue;
 }
 0x60bfd6b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x60bfd7a; continue;
 }
 0x60bfd7a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60bfdd1; continue; }
  pc = 0x60bfd82; continue;
 }
 0x60bfd82 => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60bfdd3; continue; }
  pc = 0x60bfd87; continue;
 }
 0x60bfd87 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x60bfd9a; continue; }
  pc = 0x60bfd8c; continue;
 }
 0x60bfd8c => {
  { let v: u32 = cpu.rol32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.ror32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x60bfd90; continue;
 }
 0x60bfd90 => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60bfdd3; continue; }
  pc = 0x60bfd94; continue;
 }
 0x60bfd94 => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x60bfd90; continue; }
  pc = 0x60bfd9a; continue;
 }
 0x60bfd9a => {
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x60bfddd; continue; }
  pc = 0x60bfda7; continue;
 }
 0x60bfda7 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x60bfdcf; continue; }
  pc = 0x60bfdae; continue;
 }
 0x60bfdae => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x60bfdb6; continue; }
  pc = 0x60bfdb2; continue;
 }
 0x60bfdb2 => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60bfdb6; continue;
 }
 0x60bfdb6 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x60bfdcf; continue; }
  pc = 0x60bfdbe; continue;
 }
 0x60bfdbe => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x60bfddd; continue; }
  pc = 0x60bfdcf; continue;
 }
 0x60bfdcf => {
  pc = 0x60bfdd3; continue;
 }
 0x60bfdd1 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x60bfdd3; continue;
 }
 0x60bfdd3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfddd => {
  { let v: u16 = 0x7fffu16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60bfdd3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bfdea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bfdea); }
 let mut pc: u32 = 0x60bfdea;
 loop { match pc {
 0x60bfdea => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bfe02; continue; }
  pc = 0x60bfdf9; continue;
 }
 0x60bfdf9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  if !cpu.zf { pc = 0x60bfe55; continue; }
  pc = 0x60bfe02; continue;
 }
 0x60bfe02 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bfe18; continue; }
  pc = 0x60bfe0f; continue;
 }
 0x60bfe0f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.esi = v; }
  if !cpu.zf { pc = 0x60bfe55; continue; }
  pc = 0x60bfe18; continue;
 }
 0x60bfe18 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.sf { pc = 0x60bfe5d; continue; }
  pc = 0x60bfe27; continue;
 }
 0x60bfe27 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x8u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x60bfe3d; continue; }
  pc = 0x60bfe31; continue;
 }
 0x60bfe31 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x60bfe3d; continue; }
  pc = 0x60bfe39; continue;
 }
 0x60bfe39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  pc = 0x60bfe3d; continue;
 }
 0x60bfe3d => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.zf { pc = 0x60bfe50; continue; }
  pc = 0x60bfe44; continue;
 }
 0x60bfe44 => {
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.eax; cpu.esi = v; }
  pc = 0x60bfe48; continue;
 }
 0x60bfe48 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, 0x0u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x60bfe50; continue;
 }
 0x60bfe50 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfe55 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfe5d => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60bfe48; continue; }
  pc = 0x60bfe80; continue;
 }
 0x60bfe80 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bfe83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bfe83); }
 let mut pc: u32 = 0x60bfe83;
 loop { match pc {
 0x60bfe83 => {
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
  cpu.push(0x60bfea0); f_060bfead(cpu); if cpu.returned_to != 0x60bfea0 { return; }
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
pub fn f_060bfead(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bfead); }
 let mut pc: u32 = 0x60bfead;
 loop { match pc {
 0x60bfead => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60bfef8; continue; }
  pc = 0x60bfeb1; continue;
 }
 0x60bfeb1 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x60bfef8; continue; }
  pc = 0x60bfeb5; continue;
 }
 0x60bfeb5 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x60bfef8; continue; }
  pc = 0x60bfebd; continue;
 }
 0x60bfebd => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60bfee2; continue; }
  pc = 0x60bfec1; continue;
 }
 0x60bfec1 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60bfee2; continue; }
  pc = 0x60bfec5; continue;
 }
 0x60bfec5 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bfee2; continue; }
  pc = 0x60bfecc; continue;
 }
 0x60bfecc => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x60bfed5); f_060be710(cpu); if cpu.returned_to != 0x60bfed5 { return; }
  { let v: u32 = 0xc0000000u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bfef7; continue;
 }
 0x60bfee2 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x83u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x60bfeeb); f_060be710(cpu); if cpu.returned_to != 0x60bfeeb { return; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bfef7; continue;
 }
 0x60bfef7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bfef8 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60bff09; continue; }
  pc = 0x60bfefc; continue;
 }
 0x60bfefc => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60bff09; continue; }
  pc = 0x60bff00; continue;
 }
 0x60bff00 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bff06; continue; }
  pc = 0x60bff05; continue;
 }
 0x60bff05 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bff06 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60bff09; continue;
 }
 0x60bff09 => {
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
  if !cpu.zf { pc = 0x60bff44; continue; }
  pc = 0x60bff3a; continue;
 }
 0x60bff3a => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x60bff3a; continue; }
  pc = 0x60bff44; continue;
 }
 0x60bff44 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60bff53; continue; }
  pc = 0x60bff49; continue;
 }
 0x60bff49 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.sf { cpu.poll(); pc = 0x60bff49; continue; }
  pc = 0x60bff53; continue;
 }
 0x60bff53 => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x3fffu16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x60bff76; continue; }
  pc = 0x60bff5d; continue;
 }
 0x60bff5d => {
  { cpu.sub16((cpu.edi as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x60bff76; continue; }
  pc = 0x60bff64; continue;
 }
 0x60bff64 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u16 = 0x7fffu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x60c004e; continue;
 }
 0x60bff76 => {
  { cpu.sub16((cpu.edi as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x60bff87; continue; }
  pc = 0x60bff7c; continue;
 }
 0x60bff7c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x60c004e; continue;
 }
 0x60bff87 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bff99; continue; }
  pc = 0x60bff96; continue;
 }
 0x60bff96 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60bff99; continue;
 }
 0x60bff99 => {
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
  if cpu.zf { pc = 0x60bffba; continue; }
  pc = 0x60bffb5; continue;
 }
 0x60bffb5 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  pc = 0x60bffba; continue;
 }
 0x60bffba => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edx); cpu.edi = v; }
  if cpu.zf { pc = 0x60bffd4; continue; }
  pc = 0x60bffc2; continue;
 }
 0x60bffc2 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60bffc2; continue; }
  pc = 0x60bffd4; continue;
 }
 0x60bffd4 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bffe9; continue; }
  pc = 0x60bffdf; continue;
 }
 0x60bffdf => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bffe9; continue;
 }
 0x60bffe9 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60c0020; continue; }
  pc = 0x60bfff4; continue;
 }
 0x60bfff4 => {
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
  if cpu.zf { pc = 0x60c0020; continue; }
  pc = 0x60c0009; continue;
 }
 0x60c0009 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c0009; continue; }
  pc = 0x60c0020; continue;
 }
 0x60c0020 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x60c0032; continue; }
  pc = 0x60c002d; continue;
 }
 0x60c002d => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x60c0032; continue;
 }
 0x60c0032 => {
  { let v: u16 = cpu.or16((cpu.edi as u16), (cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c004e; continue; }
  pc = 0x60c0037; continue;
 }
 0x60c0037 => {
  if !cpu.zf { pc = 0x60c003d; continue; }
  pc = 0x60c0039; continue;
 }
 0x60c0039 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c0043; continue;
 }
 0x60c003d => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0043; continue;
 }
 0x60c0043 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x60c004e; continue;
 }
 0x60c004e => {
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
pub fn f_060c0058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0058); }
 let mut pc: u32 = 0x60c0058;
 loop { match pc {
 0x60c0058 => {
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
  cpu.push(0x60c0075); f_060c0082(cpu); if cpu.returned_to != 0x60c0075 { return; }
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
pub fn f_060c0082(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0082); }
 let mut pc: u32 = 0x60c0082;
 loop { match pc {
 0x60c0082 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60c0093; continue; }
  pc = 0x60c0086; continue;
 }
 0x60c0086 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c0093; continue; }
  pc = 0x60c008a; continue;
 }
 0x60c008a => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c0090; continue; }
  pc = 0x60c008f; continue;
 }
 0x60c008f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c0090 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0093; continue;
 }
 0x60c0093 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60c00aa; continue; }
  pc = 0x60c0097; continue;
 }
 0x60c0097 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x60c00aa; continue; }
  pc = 0x60c009b; continue;
 }
 0x60c009b => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x60c00aa; continue; }
  pc = 0x60c00a3; continue;
 }
 0x60c00a3 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c00aa => {
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
  if cpu.cf { pc = 0x60c00da; continue; }
  pc = 0x60c00ce; continue;
 }
 0x60c00ce => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x60c00da; continue; }
  pc = 0x60c00d5; continue;
 }
 0x60c00d5 => {
  pc = 0x60c0171; continue;
 }
 0x60c00da => {
  { cpu.sub16((cpu.ecx as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x60c00eb; continue; }
  pc = 0x60c00e0; continue;
 }
 0x60c00e0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x60c0167; continue;
 }
 0x60c00eb => {
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
  if cpu.sf { pc = 0x60c0122; continue; }
  pc = 0x60c011a; continue;
 }
 0x60c011a => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0122; continue;
 }
 0x60c0122 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.cf { pc = 0x60c014e; continue; }
  pc = 0x60c0126; continue;
 }
 0x60c0126 => {
  if !cpu.zf { pc = 0x60c0135; continue; }
  pc = 0x60c0128; continue;
 }
 0x60c0128 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x60c0135; continue; }
  pc = 0x60c0131; continue;
 }
 0x60c0131 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x60c0135; continue;
 }
 0x60c0135 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x60c014e; continue; }
  pc = 0x60c013d; continue;
 }
 0x60c013d => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x60c0171; continue; }
  pc = 0x60c014e; continue;
 }
 0x60c014e => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c0167; continue; }
  pc = 0x60c0153; continue;
 }
 0x60c0153 => {
  if !cpu.zf { pc = 0x60c0159; continue; }
  pc = 0x60c0155; continue;
 }
 0x60c0155 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c015c; continue;
 }
 0x60c0159 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c015c; continue;
 }
 0x60c015c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0167; continue;
 }
 0x60c0167 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c0171 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60c0167; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c017e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c017e); } r_060c017e(cpu, 0x60c017e); }
pub fn f_060c01d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c01d3); } r_060c017e(cpu, 0x60c01d3); }
pub fn f_060c01d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c01d8); } r_060c017e(cpu, 0x60c01d8); }
fn r_060c017e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c017e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0xa0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x60c01db; continue;
 }
 0x60c01d3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x60c01db; continue;
 }
 0x60c01d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x60c01db; continue;
 }
 0x60c01db => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c023d; continue; }
  pc = 0x60c01f3; continue;
 }
 0x60c01f3 => {
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c0235; continue; }
  pc = 0x60c01f9; continue;
 }
 0x60c01f9 => {
  { let v: u8 = (cpu.ebx as u8) & 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), (cpu.ebx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c0235; continue; }
  pc = 0x60c0200; continue;
 }
 0x60c0200 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.rcr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x60c0214; continue; }
  pc = 0x60c0210; continue;
 }
 0x60c0210 => {
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c0221; continue;
 }
 0x60c0214 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shld32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60c0221; continue;
 }
 0x60c0221 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x60c0232; continue; }
  pc = 0x60c0230; continue;
 }
 0x60c0230 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60c0232; continue;
 }
 0x60c0232 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c0235 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c023d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0242(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0242); } r_060c0242(cpu, 0x60c0242); }
pub fn f_060c0253(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0253); } r_060c0242(cpu, 0x60c0253); }
fn r_060c0242(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c0242 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x60c0252; continue; }
  pc = 0x60c0249; continue;
 }
 0x60c0249 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xbfffu32; cpu.edx = v; }
  pc = 0x60c025b; continue;
 }
 0x60c0252 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x60c0253; continue;
 }
 0x60c0253 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3fffu32; cpu.edx = v; }
  pc = 0x60c025b; continue;
 }
 0x60c025b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60c0276; continue; }
  pc = 0x60c0260; continue;
 }
 0x60c0260 => {
  cpu.unimplemented(0x60c0260, "Bsr");
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60c027a; continue;
 }
 0x60c0276 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x60c027a; continue;
 }
 0x60c027a => {
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
pub fn f_060c0288(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0288); }
 let mut pc: u32 = 0x60c0288;
 loop { match pc {
 0x60c0288 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 11u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 20u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7ffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60c02d3; continue; }
  pc = 0x60c029c; continue;
 }
 0x60c029c => {
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if cpu.zf { pc = 0x60c02aa; continue; }
  pc = 0x60c02a3; continue;
 }
 0x60c02a3 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3c00u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c02cb; continue;
 }
 0x60c02aa => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c02b8; continue; }
  pc = 0x60c02b6; continue;
 }
 0x60c02b6 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c02b8; continue;
 }
 0x60c02b8 => {
  if cpu.zf { pc = 0x60c02cb; continue; }
  pc = 0x60c02ba; continue;
 }
 0x60c02ba => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x60c02c4); f_060be710(cpu); if cpu.returned_to != 0x60c02c4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx | 0x40000000u32; cpu.edx = v; }
  pc = 0x60c02cb; continue;
 }
 0x60c02cb => {
  { let v: u32 = cpu.edx | 0x80000000u32; cpu.edx = v; }
  pc = 0x60c02f8; continue;
 }
 0x60c02d3 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c02df; continue; }
  pc = 0x60c02d7; continue;
 }
 0x60c02d7 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60c02df; continue; }
  pc = 0x60c02db; continue;
 }
 0x60c02db => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x60c02f8; continue;
 }
 0x60c02df => {
  { let v: u16 = 0x3c01u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c02ec; continue; }
  pc = 0x60c02e7; continue;
 }
 0x60c02e7 => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c02ec; continue;
 }
 0x60c02ec => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x60c02f8; continue; }
  pc = 0x60c02f0; continue;
 }
 0x60c02f0 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60c02ec; continue;
 }
 0x60c02f8 => {
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
