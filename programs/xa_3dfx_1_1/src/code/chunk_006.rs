#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_020be360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be360); }
 let mut pc: u32 = 0x20be360;
 loop { match pc {
 0x20be360 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be364(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be364); }
 let mut pc: u32 = 0x20be364;
 loop { match pc {
 0x20be364 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be368); }
 let mut pc: u32 = 0x20be368;
 loop { match pc {
 0x20be368 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be36c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be36c); }
 let mut pc: u32 = 0x20be36c;
 loop { match pc {
 0x20be36c => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be370); }
 let mut pc: u32 = 0x20be370;
 loop { match pc {
 0x20be370 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be374(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be374); }
 let mut pc: u32 = 0x20be374;
 loop { match pc {
 0x20be374 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be378(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be378); }
 let mut pc: u32 = 0x20be378;
 loop { match pc {
 0x20be378 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be37c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be37c); }
 let mut pc: u32 = 0x20be37c;
 loop { match pc {
 0x20be37c => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be380); }
 let mut pc: u32 = 0x20be380;
 loop { match pc {
 0x20be380 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be384(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be384); }
 let mut pc: u32 = 0x20be384;
 loop { match pc {
 0x20be384 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be388(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be388); }
 let mut pc: u32 = 0x20be388;
 loop { match pc {
 0x20be388 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be38c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be38c); }
 let mut pc: u32 = 0x20be38c;
 loop { match pc {
 0x20be38c => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be390); }
 let mut pc: u32 = 0x20be390;
 loop { match pc {
 0x20be390 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be394(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be394); }
 let mut pc: u32 = 0x20be394;
 loop { match pc {
 0x20be394 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be398); }
 let mut pc: u32 = 0x20be398;
 loop { match pc {
 0x20be398 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be39c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be39c); }
 let mut pc: u32 = 0x20be39c;
 loop { match pc {
 0x20be39c => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3a0); }
 let mut pc: u32 = 0x20be3a0;
 loop { match pc {
 0x20be3a0 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3a4); }
 let mut pc: u32 = 0x20be3a4;
 loop { match pc {
 0x20be3a4 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3a8); }
 let mut pc: u32 = 0x20be3a8;
 loop { match pc {
 0x20be3a8 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3ac); }
 let mut pc: u32 = 0x20be3ac;
 loop { match pc {
 0x20be3ac => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3b0); }
 let mut pc: u32 = 0x20be3b0;
 loop { match pc {
 0x20be3b0 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3b4); }
 let mut pc: u32 = 0x20be3b4;
 loop { match pc {
 0x20be3b4 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3b8); }
 let mut pc: u32 = 0x20be3b8;
 loop { match pc {
 0x20be3b8 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3bc); }
 let mut pc: u32 = 0x20be3bc;
 loop { match pc {
 0x20be3bc => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3c0); }
 let mut pc: u32 = 0x20be3c0;
 loop { match pc {
 0x20be3c0 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3c4); }
 let mut pc: u32 = 0x20be3c4;
 loop { match pc {
 0x20be3c4 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3c8); }
 let mut pc: u32 = 0x20be3c8;
 loop { match pc {
 0x20be3c8 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3cc); }
 let mut pc: u32 = 0x20be3cc;
 loop { match pc {
 0x20be3cc => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3d0); }
 let mut pc: u32 = 0x20be3d0;
 loop { match pc {
 0x20be3d0 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3d4); }
 let mut pc: u32 = 0x20be3d4;
 loop { match pc {
 0x20be3d4 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3d8); }
 let mut pc: u32 = 0x20be3d8;
 loop { match pc {
 0x20be3d8 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3dc); }
 let mut pc: u32 = 0x20be3dc;
 loop { match pc {
 0x20be3dc => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3e0); }
 let mut pc: u32 = 0x20be3e0;
 loop { match pc {
 0x20be3e0 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3e4); }
 let mut pc: u32 = 0x20be3e4;
 loop { match pc {
 0x20be3e4 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3e8); }
 let mut pc: u32 = 0x20be3e8;
 loop { match pc {
 0x20be3e8 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3ec); }
 let mut pc: u32 = 0x20be3ec;
 loop { match pc {
 0x20be3ec => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3f0); }
 let mut pc: u32 = 0x20be3f0;
 loop { match pc {
 0x20be3f0 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3f4); }
 let mut pc: u32 = 0x20be3f4;
 loop { match pc {
 0x20be3f4 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3f8); }
 let mut pc: u32 = 0x20be3f8;
 loop { match pc {
 0x20be3f8 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be3fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be3fc); }
 let mut pc: u32 = 0x20be3fc;
 loop { match pc {
 0x20be3fc => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be400); }
 let mut pc: u32 = 0x20be400;
 loop { match pc {
 0x20be400 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be404(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be404); }
 let mut pc: u32 = 0x20be404;
 loop { match pc {
 0x20be404 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be408(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be408); }
 let mut pc: u32 = 0x20be408;
 loop { match pc {
 0x20be408 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be40c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be40c); }
 let mut pc: u32 = 0x20be40c;
 loop { match pc {
 0x20be40c => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be410); }
 let mut pc: u32 = 0x20be410;
 loop { match pc {
 0x20be410 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be414); }
 let mut pc: u32 = 0x20be414;
 loop { match pc {
 0x20be414 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be418(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be418); }
 let mut pc: u32 = 0x20be418;
 loop { match pc {
 0x20be418 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be41c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be41c); }
 let mut pc: u32 = 0x20be41c;
 loop { match pc {
 0x20be41c => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be420); }
 let mut pc: u32 = 0x20be420;
 loop { match pc {
 0x20be420 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be424); }
 let mut pc: u32 = 0x20be424;
 loop { match pc {
 0x20be424 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be428(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be428); }
 let mut pc: u32 = 0x20be428;
 loop { match pc {
 0x20be428 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be42c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be42c); }
 let mut pc: u32 = 0x20be42c;
 loop { match pc {
 0x20be42c => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be430); }
 let mut pc: u32 = 0x20be430;
 loop { match pc {
 0x20be430 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be434(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be434); }
 let mut pc: u32 = 0x20be434;
 loop { match pc {
 0x20be434 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be438(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be438); }
 let mut pc: u32 = 0x20be438;
 loop { match pc {
 0x20be438 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be43c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be43c); }
 let mut pc: u32 = 0x20be43c;
 loop { match pc {
 0x20be43c => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be440); }
 let mut pc: u32 = 0x20be440;
 loop { match pc {
 0x20be440 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be444); }
 let mut pc: u32 = 0x20be444;
 loop { match pc {
 0x20be444 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be448(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be448); }
 let mut pc: u32 = 0x20be448;
 loop { match pc {
 0x20be448 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be44c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be44c); }
 let mut pc: u32 = 0x20be44c;
 loop { match pc {
 0x20be44c => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be450); }
 let mut pc: u32 = 0x20be450;
 loop { match pc {
 0x20be450 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be454); }
 let mut pc: u32 = 0x20be454;
 loop { match pc {
 0x20be454 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be458); }
 let mut pc: u32 = 0x20be458;
 loop { match pc {
 0x20be458 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be45c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be45c); }
 let mut pc: u32 = 0x20be45c;
 loop { match pc {
 0x20be45c => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be460); }
 let mut pc: u32 = 0x20be460;
 loop { match pc {
 0x20be460 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be464); }
 let mut pc: u32 = 0x20be464;
 loop { match pc {
 0x20be464 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be468(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be468); }
 let mut pc: u32 = 0x20be468;
 loop { match pc {
 0x20be468 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be46c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be46c); }
 let mut pc: u32 = 0x20be46c;
 loop { match pc {
 0x20be46c => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be470); }
 let mut pc: u32 = 0x20be470;
 loop { match pc {
 0x20be470 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be474); }
 let mut pc: u32 = 0x20be474;
 loop { match pc {
 0x20be474 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be478(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be478); }
 let mut pc: u32 = 0x20be478;
 loop { match pc {
 0x20be478 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be47c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be47c); }
 let mut pc: u32 = 0x20be47c;
 loop { match pc {
 0x20be47c => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be480); }
 let mut pc: u32 = 0x20be480;
 loop { match pc {
 0x20be480 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be484(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be484); }
 let mut pc: u32 = 0x20be484;
 loop { match pc {
 0x20be484 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be488); }
 let mut pc: u32 = 0x20be488;
 loop { match pc {
 0x20be488 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be48c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be48c); }
 let mut pc: u32 = 0x20be48c;
 loop { match pc {
 0x20be48c => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be490); }
 let mut pc: u32 = 0x20be490;
 loop { match pc {
 0x20be490 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be494(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be494); }
 let mut pc: u32 = 0x20be494;
 loop { match pc {
 0x20be494 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be498(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be498); }
 let mut pc: u32 = 0x20be498;
 loop { match pc {
 0x20be498 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be49c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be49c); }
 let mut pc: u32 = 0x20be49c;
 loop { match pc {
 0x20be49c => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4a0); }
 let mut pc: u32 = 0x20be4a0;
 loop { match pc {
 0x20be4a0 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4a4); }
 let mut pc: u32 = 0x20be4a4;
 loop { match pc {
 0x20be4a4 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4a8); }
 let mut pc: u32 = 0x20be4a8;
 loop { match pc {
 0x20be4a8 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4ac); }
 let mut pc: u32 = 0x20be4ac;
 loop { match pc {
 0x20be4ac => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4b0); }
 let mut pc: u32 = 0x20be4b0;
 loop { match pc {
 0x20be4b0 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4b4); }
 let mut pc: u32 = 0x20be4b4;
 loop { match pc {
 0x20be4b4 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4b8); }
 let mut pc: u32 = 0x20be4b8;
 loop { match pc {
 0x20be4b8 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4bc); }
 let mut pc: u32 = 0x20be4bc;
 loop { match pc {
 0x20be4bc => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4c0); }
 let mut pc: u32 = 0x20be4c0;
 loop { match pc {
 0x20be4c0 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4c4); }
 let mut pc: u32 = 0x20be4c4;
 loop { match pc {
 0x20be4c4 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4c8); }
 let mut pc: u32 = 0x20be4c8;
 loop { match pc {
 0x20be4c8 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4cc); }
 let mut pc: u32 = 0x20be4cc;
 loop { match pc {
 0x20be4cc => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4d0); }
 let mut pc: u32 = 0x20be4d0;
 loop { match pc {
 0x20be4d0 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4d4); }
 let mut pc: u32 = 0x20be4d4;
 loop { match pc {
 0x20be4d4 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4d8); }
 let mut pc: u32 = 0x20be4d8;
 loop { match pc {
 0x20be4d8 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4dc); }
 let mut pc: u32 = 0x20be4dc;
 loop { match pc {
 0x20be4dc => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4e0); }
 let mut pc: u32 = 0x20be4e0;
 loop { match pc {
 0x20be4e0 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4e4); }
 let mut pc: u32 = 0x20be4e4;
 loop { match pc {
 0x20be4e4 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4e8); }
 let mut pc: u32 = 0x20be4e8;
 loop { match pc {
 0x20be4e8 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4ec); }
 let mut pc: u32 = 0x20be4ec;
 loop { match pc {
 0x20be4ec => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4f0); }
 let mut pc: u32 = 0x20be4f0;
 loop { match pc {
 0x20be4f0 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4f4); }
 let mut pc: u32 = 0x20be4f4;
 loop { match pc {
 0x20be4f4 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4f8); }
 let mut pc: u32 = 0x20be4f8;
 loop { match pc {
 0x20be4f8 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be4fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be4fc); }
 let mut pc: u32 = 0x20be4fc;
 loop { match pc {
 0x20be4fc => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be500); }
 let mut pc: u32 = 0x20be500;
 loop { match pc {
 0x20be500 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be504(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be504); }
 let mut pc: u32 = 0x20be504;
 loop { match pc {
 0x20be504 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be508(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be508); }
 let mut pc: u32 = 0x20be508;
 loop { match pc {
 0x20be508 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be50c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be50c); }
 let mut pc: u32 = 0x20be50c;
 loop { match pc {
 0x20be50c => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be510); }
 let mut pc: u32 = 0x20be510;
 loop { match pc {
 0x20be510 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be514(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be514); }
 let mut pc: u32 = 0x20be514;
 loop { match pc {
 0x20be514 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be518(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be518); }
 let mut pc: u32 = 0x20be518;
 loop { match pc {
 0x20be518 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be51c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be51c); }
 let mut pc: u32 = 0x20be51c;
 loop { match pc {
 0x20be51c => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be520); }
 let mut pc: u32 = 0x20be520;
 loop { match pc {
 0x20be520 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be524(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be524); }
 let mut pc: u32 = 0x20be524;
 loop { match pc {
 0x20be524 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be528(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be528); }
 let mut pc: u32 = 0x20be528;
 loop { match pc {
 0x20be528 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be52c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be52c); }
 let mut pc: u32 = 0x20be52c;
 loop { match pc {
 0x20be52c => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be530); }
 let mut pc: u32 = 0x20be530;
 loop { match pc {
 0x20be530 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be534(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be534); }
 let mut pc: u32 = 0x20be534;
 loop { match pc {
 0x20be534 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be538(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be538); }
 let mut pc: u32 = 0x20be538;
 loop { match pc {
 0x20be538 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be53c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be53c); }
 let mut pc: u32 = 0x20be53c;
 loop { match pc {
 0x20be53c => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be540); }
 let mut pc: u32 = 0x20be540;
 loop { match pc {
 0x20be540 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be544); }
 let mut pc: u32 = 0x20be544;
 loop { match pc {
 0x20be544 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be548); }
 let mut pc: u32 = 0x20be548;
 loop { match pc {
 0x20be548 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be54c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be54c); }
 let mut pc: u32 = 0x20be54c;
 loop { match pc {
 0x20be54c => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be550); }
 let mut pc: u32 = 0x20be550;
 loop { match pc {
 0x20be550 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be554(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be554); }
 let mut pc: u32 = 0x20be554;
 loop { match pc {
 0x20be554 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be558); }
 let mut pc: u32 = 0x20be558;
 loop { match pc {
 0x20be558 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be55c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be55c); }
 let mut pc: u32 = 0x20be55c;
 loop { match pc {
 0x20be55c => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be560); }
 let mut pc: u32 = 0x20be560;
 loop { match pc {
 0x20be560 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be564(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be564); }
 let mut pc: u32 = 0x20be564;
 loop { match pc {
 0x20be564 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be568(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be568); }
 let mut pc: u32 = 0x20be568;
 loop { match pc {
 0x20be568 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be56c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be56c); }
 let mut pc: u32 = 0x20be56c;
 loop { match pc {
 0x20be56c => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be570); }
 let mut pc: u32 = 0x20be570;
 loop { match pc {
 0x20be570 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be574); }
 let mut pc: u32 = 0x20be574;
 loop { match pc {
 0x20be574 => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be578(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be578); }
 let mut pc: u32 = 0x20be578;
 loop { match pc {
 0x20be578 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be57c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be57c); }
 let mut pc: u32 = 0x20be57c;
 loop { match pc {
 0x20be57c => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be580); }
 let mut pc: u32 = 0x20be580;
 loop { match pc {
 0x20be580 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be584(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be584); }
 let mut pc: u32 = 0x20be584;
 loop { match pc {
 0x20be584 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be588(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be588); }
 let mut pc: u32 = 0x20be588;
 loop { match pc {
 0x20be588 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be58c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be58c); }
 let mut pc: u32 = 0x20be58c;
 loop { match pc {
 0x20be58c => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be590); }
 let mut pc: u32 = 0x20be590;
 loop { match pc {
 0x20be590 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be594(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be594); }
 let mut pc: u32 = 0x20be594;
 loop { match pc {
 0x20be594 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be598(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be598); }
 let mut pc: u32 = 0x20be598;
 loop { match pc {
 0x20be598 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be59c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be59c); }
 let mut pc: u32 = 0x20be59c;
 loop { match pc {
 0x20be59c => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5a0); }
 let mut pc: u32 = 0x20be5a0;
 loop { match pc {
 0x20be5a0 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5a4); }
 let mut pc: u32 = 0x20be5a4;
 loop { match pc {
 0x20be5a4 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5a8); }
 let mut pc: u32 = 0x20be5a8;
 loop { match pc {
 0x20be5a8 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5ac); }
 let mut pc: u32 = 0x20be5ac;
 loop { match pc {
 0x20be5ac => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5b0); }
 let mut pc: u32 = 0x20be5b0;
 loop { match pc {
 0x20be5b0 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5b4); }
 let mut pc: u32 = 0x20be5b4;
 loop { match pc {
 0x20be5b4 => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5b8); }
 let mut pc: u32 = 0x20be5b8;
 loop { match pc {
 0x20be5b8 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5bc); }
 let mut pc: u32 = 0x20be5bc;
 loop { match pc {
 0x20be5bc => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5c0); }
 let mut pc: u32 = 0x20be5c0;
 loop { match pc {
 0x20be5c0 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5c4); }
 let mut pc: u32 = 0x20be5c4;
 loop { match pc {
 0x20be5c4 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5c8); }
 let mut pc: u32 = 0x20be5c8;
 loop { match pc {
 0x20be5c8 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5cc); }
 let mut pc: u32 = 0x20be5cc;
 loop { match pc {
 0x20be5cc => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5d0); }
 let mut pc: u32 = 0x20be5d0;
 loop { match pc {
 0x20be5d0 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5d4); }
 let mut pc: u32 = 0x20be5d4;
 loop { match pc {
 0x20be5d4 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5d8); }
 let mut pc: u32 = 0x20be5d8;
 loop { match pc {
 0x20be5d8 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5dc); }
 let mut pc: u32 = 0x20be5dc;
 loop { match pc {
 0x20be5dc => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5e0); }
 let mut pc: u32 = 0x20be5e0;
 loop { match pc {
 0x20be5e0 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5e4); }
 let mut pc: u32 = 0x20be5e4;
 loop { match pc {
 0x20be5e4 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5e8); }
 let mut pc: u32 = 0x20be5e8;
 loop { match pc {
 0x20be5e8 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5ec); }
 let mut pc: u32 = 0x20be5ec;
 loop { match pc {
 0x20be5ec => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5f0); }
 let mut pc: u32 = 0x20be5f0;
 loop { match pc {
 0x20be5f0 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5f4); }
 let mut pc: u32 = 0x20be5f4;
 loop { match pc {
 0x20be5f4 => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be5f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be5f8); }
 let mut pc: u32 = 0x20be5f8;
 loop { match pc {
 0x20be5f8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x20be1f8u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be60e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be60e); }
 let mut pc: u32 = 0x20be60e;
 loop { match pc {
 0x20be60e => {
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
  cpu.flat_ds(0x20be64c); cpu.push(0x20be651); f_020be5f8(cpu); if cpu.returned_to != 0x20be651 { return; }
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
  cpu.flat_ds(0x20be69f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be77d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be77d); }
 let mut pc: u32 = 0x20be77d;
 loop { match pc {
 0x20be77d => {
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
  if cpu.cf { pc = 0x20be7bc; continue; }
  pc = 0x20be79f; continue;
 }
 0x20be79f => {
  if (cpu.cf || cpu.zf) { pc = 0x20be7c9; continue; }
  pc = 0x20be7a1; continue;
 }
 0x20be7a1 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x20be7b2; continue; }
  pc = 0x20be7a6; continue;
 }
 0x20be7a6 => {
  if (cpu.cf || cpu.zf) { pc = 0x20be7de; continue; }
  pc = 0x20be7a8; continue;
 }
 0x20be7a8 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x20be7c9; continue; }
  pc = 0x20be7ad; continue;
 }
 0x20be7ad => {
  pc = 0x20be85b; continue;
 }
 0x20be7b2 => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x20be7d9; continue; }
  pc = 0x20be7b7; continue;
 }
 0x20be7b7 => {
  pc = 0x20be85b; continue;
 }
 0x20be7bc => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x20be85b; continue; }
  pc = 0x20be7c5; continue;
 }
 0x20be7c5 => {
  if (cpu.cf || cpu.zf) { pc = 0x20be7d9; continue; }
  pc = 0x20be7c7; continue;
 }
 0x20be7c7 => {
  pc = 0x20be7de; continue;
 }
 0x20be7c9 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x20be7d6; continue; }
  pc = 0x20be7cf; continue;
 }
 0x20be7cf => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20be7d6; continue;
 }
 0x20be7d6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20be7d9; continue;
 }
 0x20be7d9 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x20be7de; continue;
 }
 0x20be7de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20be7ff; continue; }
  pc = 0x20be7f8; continue;
 }
 0x20be7f8 => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20be7ff; continue;
 }
 0x20be7ff => {
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
  cpu.push(0x20be827); f_020c540e(cpu); if cpu.returned_to != 0x20be827 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x20be834; continue; }
  pc = 0x20be830; continue;
 }
 0x20be830 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x20be834; continue;
 }
 0x20be834 => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x20be83b; continue;
 }
 0x20be83b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x20be847; continue; }
  pc = 0x20be844; continue;
 }
 0x20be844 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20be83b; continue;
 }
 0x20be847 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20be850); f_020be867(cpu); if cpu.returned_to != 0x20be850 { return; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20be85b; continue;
 }
 0x20be85b => {
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
pub fn f_020be867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be867); }
 let mut pc: u32 = 0x20be867;
 loop { match pc {
 0x20be867 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x20be8c4; continue; }
  pc = 0x20be878; continue;
 }
 0x20be878 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20be88e; continue; }
  pc = 0x20be87d; continue;
 }
 0x20be87d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x20be88e; continue; }
  pc = 0x20be886; continue;
 }
 0x20be886 => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x20be88e; continue; }
  pc = 0x20be88b; continue;
 }
 0x20be88b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20be878; continue;
 }
 0x20be88e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x20be8a4; continue; }
  pc = 0x20be89f; continue;
 }
 0x20be89f => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x20be8b0; continue; }
  pc = 0x20be8a4; continue;
 }
 0x20be8a4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x20be8b0; continue; }
  pc = 0x20be8ad; continue;
 }
 0x20be8ad => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20be8a4; continue;
 }
 0x20be8b0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x20be8ba; continue; }
  pc = 0x20be8b9; continue;
 }
 0x20be8b9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20be8ba; continue;
 }
 0x20be8ba => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20be8ba; continue; }
  pc = 0x20be8c4; continue;
 }
 0x20be8c4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be8c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be8c6); }
 let mut pc: u32 = 0x20be8c6;
 loop { match pc {
 0x20be8c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x20be8d0); f_0209c4df(cpu); if cpu.returned_to != 0x20be8d0 { return; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be8d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8d4); } r_020be8d4(cpu, 0x20be8d4); }
pub fn f_020be8d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8d8); } r_020be8d4(cpu, 0x20be8d8); }
pub fn f_020be8dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8dc); } r_020be8d4(cpu, 0x20be8dc); }
pub fn f_020be8de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8de); } r_020be8d4(cpu, 0x20be8de); }
pub fn f_020be8e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8e0); } r_020be8d4(cpu, 0x20be8e0); }
pub fn f_020be8e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be8e2); } r_020be8d4(cpu, 0x20be8e2); }
pub fn f_020be92e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be92e); } r_020be8d4(cpu, 0x20be92e); }
pub fn f_020be930(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be930); } r_020be8d4(cpu, 0x20be930); }
pub fn f_020be934(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be934); } r_020be8d4(cpu, 0x20be934); }
pub fn f_020be936(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be936); } r_020be8d4(cpu, 0x20be936); }
fn r_020be8d4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20be8d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20be8d8; continue;
 }
 0x20be8d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20be8dc; continue;
 }
 0x20be8dc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20be8de; continue;
 }
 0x20be8de => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20be8e0; continue;
 }
 0x20be8e0 => {
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
  cpu.flat_ds(0x20be942); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x20be8e2 => {
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
  cpu.flat_ds(0x20be942); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x20be92e => {
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
  cpu.flat_ds(0x20be942); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x20be930 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20be934; continue;
 }
 0x20be934 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20be936; continue;
 }
 0x20be936 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x20be942); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be97e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be97e); } r_020be97e(cpu, 0x20be97e); }
pub fn f_020be982(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be982); } r_020be97e(cpu, 0x20be982); }
pub fn f_020be98c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be98c); } r_020be97e(cpu, 0x20be98c); }
fn r_020be97e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20be97e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20be982; continue;
 }
 0x20be982 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let a = cpu.ebx; let b = cpu.ebx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x20be98c; continue;
 }
 0x20be98c => {
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x20be9a1; continue;
 }
 0x20be9a1 => {
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = 0xffffffefu32; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.call(0x20be99d); return; }
  pc = 0x20be9ae; continue;
 }
 0x20be9ae => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x20be9a1; continue; }
  pc = 0x20be9b2; continue;
 }
 0x20be9b2 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(0xbu8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8((cpu.ecx as u8), cpu.r8(0x20befu32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.call(0x20be9bd); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be9ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be9ec); }
 let mut pc: u32 = 0x20be9ec;
 loop { match pc {
 0x20be9ec => {
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x20be9fd; continue;
 }
 0x20be9fd => {
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x20bea01; continue;
 }
 0x20bea01 => {
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = 0xffffffefu32; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20be9fd; continue; }
  pc = 0x20bea0e; continue;
 }
 0x20bea0e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.sf { cpu.poll(); pc = 0x20bea01; continue; }
  pc = 0x20bea12; continue;
 }
 0x20bea12 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_sub(0xbu8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x90020befu32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x20bea27); cpu.call(0x0); if cpu.returned_to != 0x20bea27 { return; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xefb4020bu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(8)))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xc6020befu32; cpu.ebp = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bea4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bea4c); }
 let mut pc: u32 = 0x20bea4c;
 loop { match pc {
 0x20bea4c => {
  { cpu.sub32(cpu.eax, 0x2f020bf0u32); }
  cpu.unimplemented(0x20bea51, "Int1");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.esi | cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  cpu.call(0x20bea60); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020beb4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20beb4c); }
 let mut pc: u32 = 0x20beb4c;
 loop { match pc {
 0x20beb4c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x20beb55); cpu.call(0x160df74d); if cpu.returned_to != 0x20beb55 { return; }
  { cpu.cf = false; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.cf = true; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0xffffffffu32.wrapping_add(cpu.edi))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (!cpu.cf && !cpu.zf) { pc = 0x20beb69; continue; }
  pc = 0x20beb6a; continue;
 }
 0x20beb69 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0xfffffff8u32.wrapping_add(cpu.ebx))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20beb86; continue; }
  pc = 0x20beb8e; continue;
 }
 0x20beb6a => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20beb86; continue; }
  pc = 0x20beb8e; continue;
 }
 0x20beb86 => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20beb86; continue; }
  pc = 0x20beb8e; continue;
 }
 0x20beb8e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20beb96; continue;
 }
 0x20beb96 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20beb96; continue; }
  pc = 0x20beb9e; continue;
 }
 0x20beb9e => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0xe6020bf8u32.wrapping_add(cpu.edx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xe6020c00u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0xf958020bu32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, 0xd9020bf8u32); }
  { cpu.cf = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.sub8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.unimplemented(0x20bec00, "Aam");
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x20bec07; continue; }
  pc = 0x20bec06; continue;
 }
 0x20bec06 => {
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xf4020bf8u32; cpu.esi = v; }
  {  }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xf814020cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  cpu.call(0x20bec30); return;
 }
 0x20bec07 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xf4020bf8u32; cpu.esi = v; }
  {  }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xf814020cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  cpu.call(0x20bec30); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bec4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bec4c); }
 let mut pc: u32 = 0x20bec4c;
 loop { match pc {
 0x20bec4c => {
  {  }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bec64); cpu.push(0x20bec69); f_020bece0(cpu); if cpu.returned_to != 0x20bec69 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x20bec6c); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020becbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20becbc); }
 let mut pc: u32 = 0x20becbc;
 loop { match pc {
 0x20becbc => {
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
  cpu.flat_ds(0x20becd3); cpu.push(0x20becd8); f_020bece0(cpu); if cpu.returned_to != 0x20becd8 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.flat_ds(0x20becde); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bece0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bece0); } r_020bece0(cpu, 0x20bece0); }
pub fn f_020bf821(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bf821); } r_020bece0(cpu, 0x20bf821); }
pub fn f_020bf960(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bf960); } r_020bece0(cpu, 0x20bf960); }
pub fn f_020bf9e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bf9e1); } r_020bece0(cpu, 0x20bf9e1); }
pub fn f_020bfbfc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bfbfc); } r_020bece0(cpu, 0x20bfbfc); }
pub fn f_020bfd06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bfd06); } r_020bece0(cpu, 0x20bfd06); }
pub fn f_020bfe1e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bfe1e); } r_020bece0(cpu, 0x20bfe1e); }
pub fn f_020c00b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c00b0); } r_020bece0(cpu, 0x20c00b0); }
pub fn f_020c0265(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c0265); } r_020bece0(cpu, 0x20c0265); }
fn r_020bece0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20bece0 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x20bed6c; continue; }
  pc = 0x20becfa; continue;
 }
 0x20becfa => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xf8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if !cpu.zf { pc = 0x20bee13; continue; }
  pc = 0x20bed08; continue;
 }
 0x20bed08 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xc0u8); }
  if !cpu.cf { pc = 0x20bedc9; continue; }
  pc = 0x20bed19; continue;
 }
 0x20bed19 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x1807u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x20be98cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.flat_ds(0x20bed2a); cpu.push(0x20bed32); cpu.call(t); if cpu.returned_to != 0x20bed32 { return; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x38u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x20bed41); cpu.push(0x20bed46); f_020ae040(cpu); if cpu.returned_to != 0x20bed46 { return; }
  { let v: u32 = 0x217c868u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = cpu.r32(0x20bea4cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bed66 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20becfa; continue; }
  pc = 0x20bed6c; continue;
 }
 0x20bed6c => {
  if cpu.zf { pc = 0x20bed92; continue; }
  pc = 0x20bed6e; continue;
 }
 0x20bed6e => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x20bedb0; continue; }
  pc = 0x20bed72; continue;
 }
 0x20bed72 => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x20bed95; continue; }
  pc = 0x20bed76; continue;
 }
 0x20bed76 => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x20beda7; continue; }
  pc = 0x20bed7a; continue;
 }
 0x20bed7a => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x20bed9e; continue; }
  pc = 0x20bed7e; continue;
 }
 0x20bed7e => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x20bedb9; continue; }
  pc = 0x20bed82; continue;
 }
 0x20bed82 => {
  { cpu.sub8((cpu.eax as u8), 0x65u8); }
  if cpu.zf { pc = 0x20bedc1; continue; }
  pc = 0x20bed86; continue;
 }
 0x20bed86 => {
  { cpu.sub8((cpu.eax as u8), 0x66u8); }
  if cpu.zf { pc = 0x20bed92; continue; }
  pc = 0x20bed8a; continue;
 }
 0x20bed8a => {
  { cpu.sub8((cpu.eax as u8), 0x67u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20becfa; continue; }
  pc = 0x20bed92; continue;
 }
 0x20bed92 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bed95 => {
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bed9e => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20beda7 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bedb0 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bedb9 => {
  { let v: u16 = (cpu.segments[4] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bedc1 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20bed66; continue;
 }
 0x20bedc9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bedcc); cpu.push(0x20bedd1); f_020ae040(cpu); if cpu.returned_to != 0x20bedd1 { return; }
  { let v: u32 = 0x217c868u32; cpu.ebp = v; }
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
  if !cpu.zf { pc = 0x20bedfc; continue; }
  pc = 0x20bedf8; continue;
 }
 0x20bedf8 => {
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20bedfc; continue;
 }
 0x20bedfc => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = cpu.r32(0x20beb4cu32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x20bee13 => {
  cpu.flat_ds(0x20bee13); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bee82 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x20be97eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20bee99; continue;
 }
 0x20bee99 => {
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20beed4 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x20be97eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf038 => {
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf03d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf043); cpu.push(0x20bf048); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf048 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf053); cpu.push(0x20bf058); f_020c0404(cpu); if cpu.returned_to != 0x20bf058 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf05c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf062); cpu.push(0x20bf067); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf067 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf072); cpu.push(0x20bf077); f_020c0838(cpu); if cpu.returned_to != 0x20bf077 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf07b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf081); cpu.push(0x20bf086); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf086 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf08d); cpu.push(0x20bf092); f_020c05ca(cpu); if cpu.returned_to != 0x20bf092 { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf097 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf09d); cpu.push(0x20bf0a2); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf0a2 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf0a9); cpu.push(0x20bf0ae); f_020c05ca(cpu); if cpu.returned_to != 0x20bf0ae { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bf0b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf0b9); cpu.push(0x20bf0be); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf0be { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf0c9); cpu.push(0x20bf0ce); f_020c03f6(cpu); if cpu.returned_to != 0x20bf0ce { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf0d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf0d8); cpu.push(0x20bf0dd); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf0dd { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf0e8); cpu.push(0x20bf0ed); f_020c03f6(cpu); if cpu.returned_to != 0x20bf0ed { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf0f1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf0f7); cpu.push(0x20bf0fc); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf0fc { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf107); cpu.push(0x20bf10c); f_020c0663(cpu); if cpu.returned_to != 0x20bf10c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf110 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf116); cpu.push(0x20bf11b); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf11b { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf126); cpu.push(0x20bf12b); f_020c0663(cpu); if cpu.returned_to != 0x20bf12b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf12f => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf15f); cpu.push(0x20bf164); f_020c0c2e(cpu); if cpu.returned_to != 0x20bf164 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf169 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf16d); cpu.push(0x20bf172); f_020c0ae8(cpu); if cpu.returned_to != 0x20bf172 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf17a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf17e); cpu.push(0x20bf183); f_020c0ae8(cpu); if cpu.returned_to != 0x20bf183 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf18b => {
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
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf1a9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x300u16); }
  if !cpu.zf { pc = 0x20bf1c8; continue; }
  pc = 0x20bf1bc; continue;
 }
 0x20bf1bc => {
  { let v: u32 = 0x20bece0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20bf1e4; continue;
 }
 0x20bf1c8 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if !cpu.zf { pc = 0x20bf1da; continue; }
  pc = 0x20bf1ce; continue;
 }
 0x20bf1ce => {
  { let v: u32 = 0x20bee50u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20bf1e4; continue;
 }
 0x20bf1da => {
  { let v: u32 = 0x20bee14u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20bf1e4; continue;
 }
 0x20bf1e4 => {
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf1e9 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf1fd => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf20b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf211); cpu.push(0x20bf216); f_020c0a22(cpu); if cpu.returned_to != 0x20bf216 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf221); cpu.push(0x20bf226); f_020c0404(cpu); if cpu.returned_to != 0x20bf226 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf22a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf230); cpu.push(0x20bf235); f_020c0a22(cpu); if cpu.returned_to != 0x20bf235 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf240); cpu.push(0x20bf245); f_020c0838(cpu); if cpu.returned_to != 0x20bf245 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf249 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf24f); cpu.push(0x20bf254); f_020c0a22(cpu); if cpu.returned_to != 0x20bf254 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf25b); cpu.push(0x20bf260); f_020c05ca(cpu); if cpu.returned_to != 0x20bf260 { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf265 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf26b); cpu.push(0x20bf270); f_020c0a22(cpu); if cpu.returned_to != 0x20bf270 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf277); cpu.push(0x20bf27c); f_020c05ca(cpu); if cpu.returned_to != 0x20bf27c { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bf281 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf287); cpu.push(0x20bf28c); f_020c0a22(cpu); if cpu.returned_to != 0x20bf28c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf297); cpu.push(0x20bf29c); f_020c03f6(cpu); if cpu.returned_to != 0x20bf29c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf2a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf2a6); cpu.push(0x20bf2ab); f_020c0a22(cpu); if cpu.returned_to != 0x20bf2ab { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf2b6); cpu.push(0x20bf2bb); f_020c03f6(cpu); if cpu.returned_to != 0x20bf2bb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf2bf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf2c5); cpu.push(0x20bf2ca); f_020c0a22(cpu); if cpu.returned_to != 0x20bf2ca { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf2d5); cpu.push(0x20bf2da); f_020c0663(cpu); if cpu.returned_to != 0x20bf2da { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf2de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf2e4); cpu.push(0x20bf2e9); f_020c0a22(cpu); if cpu.returned_to != 0x20bf2e9 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf2f4); cpu.push(0x20bf2f9); f_020c0663(cpu); if cpu.returned_to != 0x20bf2f9 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf2fd => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf32d); cpu.push(0x20bf332); f_020c0a22(cpu); if cpu.returned_to != 0x20bf332 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf337 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20bf351); cpu.push(0x20bf356); f_020c0cf3(cpu); if cpu.returned_to != 0x20bf356 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf35a); cpu.push(0x20bf35f); f_020c09b3(cpu); if cpu.returned_to != 0x20bf35f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf373 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20bf381); cpu.push(0x20bf386); f_020c0cf3(cpu); if cpu.returned_to != 0x20bf386 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf38a); cpu.push(0x20bf38f); f_020c09b3(cpu); if cpu.returned_to != 0x20bf38f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf397 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf3a1); cpu.push(0x20bf3a6); f_020c0a68(cpu); if cpu.returned_to != 0x20bf3a6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf3b1); cpu.push(0x20bf3b6); f_020c0404(cpu); if cpu.returned_to != 0x20bf3b6 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf3ba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf3c4); cpu.push(0x20bf3c9); f_020c0a68(cpu); if cpu.returned_to != 0x20bf3c9 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf3d4); cpu.push(0x20bf3d9); f_020c0838(cpu); if cpu.returned_to != 0x20bf3d9 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf3dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf3e7); cpu.push(0x20bf3ec); f_020c0a68(cpu); if cpu.returned_to != 0x20bf3ec { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf3f3); cpu.push(0x20bf3f8); f_020c05ca(cpu); if cpu.returned_to != 0x20bf3f8 { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf3fd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf407); cpu.push(0x20bf40c); f_020c0a68(cpu); if cpu.returned_to != 0x20bf40c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf413); cpu.push(0x20bf418); f_020c05ca(cpu); if cpu.returned_to != 0x20bf418 { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bf41d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf427); cpu.push(0x20bf42c); f_020c0a68(cpu); if cpu.returned_to != 0x20bf42c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf437); cpu.push(0x20bf43c); f_020c03f6(cpu); if cpu.returned_to != 0x20bf43c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf440 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf44a); cpu.push(0x20bf44f); f_020c0a68(cpu); if cpu.returned_to != 0x20bf44f { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf45a); cpu.push(0x20bf45f); f_020c03f6(cpu); if cpu.returned_to != 0x20bf45f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf463 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf46d); cpu.push(0x20bf472); f_020c0a68(cpu); if cpu.returned_to != 0x20bf472 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf47d); cpu.push(0x20bf482); f_020c0663(cpu); if cpu.returned_to != 0x20bf482 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf486 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x20bf490); cpu.push(0x20bf495); f_020c0a68(cpu); if cpu.returned_to != 0x20bf495 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf4a0); cpu.push(0x20bf4a5); f_020c0663(cpu); if cpu.returned_to != 0x20bf4a5 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf4a9 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf4f0 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf524); cpu.push(0x20bf529); f_020c0a68(cpu); if cpu.returned_to != 0x20bf529 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf52e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf532); cpu.push(0x20bf537); f_020c0b67(cpu); if cpu.returned_to != 0x20bf537 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf543 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf561 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf565); cpu.push(0x20bf56a); f_020c0b67(cpu); if cpu.returned_to != 0x20bf56a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf576 => {
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
  pc = 0x20bf5a4; continue;
 }
 0x20bf5a4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x20bf5a4; continue; }
  pc = 0x20bf5b6; continue;
 }
 0x20bf5b6 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf5bd => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x20bf5cf; continue;
 }
 0x20bf5cf => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x20bf5cf; continue; }
  pc = 0x20bf5e1; continue;
 }
 0x20bf5e1 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20bfed3; continue;
 }
 0x20bf5e8 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf5f6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf5fe); cpu.push(0x20bf603); f_020c0a22(cpu); if cpu.returned_to != 0x20bf603 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf60e); cpu.push(0x20bf613); f_020c0404(cpu); if cpu.returned_to != 0x20bf613 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf617 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf61f); cpu.push(0x20bf624); f_020c0a22(cpu); if cpu.returned_to != 0x20bf624 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf62f); cpu.push(0x20bf634); f_020c0838(cpu); if cpu.returned_to != 0x20bf634 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf638 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf640); cpu.push(0x20bf645); f_020c0a22(cpu); if cpu.returned_to != 0x20bf645 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf64c); cpu.push(0x20bf651); f_020c05ca(cpu); if cpu.returned_to != 0x20bf651 { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf656 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf65e); cpu.push(0x20bf663); f_020c0a22(cpu); if cpu.returned_to != 0x20bf663 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf66a); cpu.push(0x20bf66f); f_020c05ca(cpu); if cpu.returned_to != 0x20bf66f { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bf674 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf67c); cpu.push(0x20bf681); f_020c0a22(cpu); if cpu.returned_to != 0x20bf681 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf68c); cpu.push(0x20bf691); f_020c03f6(cpu); if cpu.returned_to != 0x20bf691 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf695 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf69d); cpu.push(0x20bf6a2); f_020c0a22(cpu); if cpu.returned_to != 0x20bf6a2 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf6ad); cpu.push(0x20bf6b2); f_020c03f6(cpu); if cpu.returned_to != 0x20bf6b2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf6b6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf6be); cpu.push(0x20bf6c3); f_020c0a22(cpu); if cpu.returned_to != 0x20bf6c3 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf6ce); cpu.push(0x20bf6d3); f_020c0663(cpu); if cpu.returned_to != 0x20bf6d3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf6d7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf6df); cpu.push(0x20bf6e4); f_020c0a22(cpu); if cpu.returned_to != 0x20bf6e4 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf6ef); cpu.push(0x20bf6f4); f_020c0663(cpu); if cpu.returned_to != 0x20bf6f4 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf6f8 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf72a); cpu.push(0x20bf72f); f_020c0a22(cpu); if cpu.returned_to != 0x20bf72f { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf734 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20bf74e); cpu.push(0x20bf753); f_020c0cf3(cpu); if cpu.returned_to != 0x20bf753 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf757); cpu.push(0x20bf75c); f_020c09b3(cpu); if cpu.returned_to != 0x20bf75c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x20bf767; continue; }
  pc = 0x20bf763; continue;
 }
 0x20bf763 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20bf767; continue;
 }
 0x20bf767 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf77c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20bf78a); cpu.push(0x20bf78f); f_020c0cf3(cpu); if cpu.returned_to != 0x20bf78f { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf793); cpu.push(0x20bf798); f_020c09b3(cpu); if cpu.returned_to != 0x20bf798 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x20bf7a3; continue; }
  pc = 0x20bf79f; continue;
 }
 0x20bf79f => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20bf7a3; continue;
 }
 0x20bf7a3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf7ac => {
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf7b1 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf7e1); cpu.push(0x20bf7e6); f_020c0a33(cpu); if cpu.returned_to != 0x20bf7e6 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf7eb => {
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf7f0 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20bf7fe); cpu.push(0x20bf803); f_020c0cf3(cpu); if cpu.returned_to != 0x20bf803 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf807); cpu.push(0x20bf80c); f_020c09b8(cpu); if cpu.returned_to != 0x20bf80c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20bf814 => {
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf819 => {
  cpu.poll(); pc = cpu.r32(0x20bf821u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bf821 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.cf = false; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20bf841; continue;
 }
 0x20bf841 => {
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf846 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf852); cpu.push(0x20bf857); f_020c0404(cpu); if cpu.returned_to != 0x20bf857 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf85b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf867); cpu.push(0x20bf86c); f_020c0838(cpu); if cpu.returned_to != 0x20bf86c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf870 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf878); cpu.push(0x20bf87d); f_020c05ca(cpu); if cpu.returned_to != 0x20bf87d { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf882 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bf88a); cpu.push(0x20bf88f); f_020c05ca(cpu); if cpu.returned_to != 0x20bf88f { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bf894 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf8a0); cpu.push(0x20bf8a5); f_020c03f6(cpu); if cpu.returned_to != 0x20bf8a5 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf8a9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf8b5); cpu.push(0x20bf8ba); f_020c03f6(cpu); if cpu.returned_to != 0x20bf8ba { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf8be => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf8ca); cpu.push(0x20bf8cf); f_020c0663(cpu); if cpu.returned_to != 0x20bf8cf { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf8d3 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bf8df); cpu.push(0x20bf8e4); f_020c0663(cpu); if cpu.returned_to != 0x20bf8e4 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bf8e8 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf930 => {
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
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf958 => {
  cpu.poll(); pc = cpu.r32(0x20bf960u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bf960 => {
  { cpu.sub8((cpu.ecx as u8), 0xbu8); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x14020bf9u32.wrapping_add(cpu.ebp))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.flat_ds(0x20bf970); cpu.push(0x20bf977); cpu.call(0x0); if cpu.returned_to != 0x20bf977 { return; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)).wrapping_add(cpu.bases[3]))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20bf980; continue;
 }
 0x20bf980 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf98d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf99a => {
  { let v: u32 = cpu.r32(0x20be8d4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20be8d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be8dcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x20bf9c1); cpu.push(0x20bf9c6); f_020c05ca(cpu); if cpu.returned_to != 0x20bf9c6 { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bf9cb => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bf9cf); cpu.push(0x20bf9d4); f_020c117a(cpu); if cpu.returned_to != 0x20bf9d4 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bf9d9 => {
  cpu.poll(); pc = cpu.r32(0x20bf9e1u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bf9e1 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  {  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  cpu.unimplemented(0x20bf9ed, "Into");
  {  }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  {  }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20bfa01; continue;
 }
 0x20bfa01 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfa44 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xcd1b8afeu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xd49a784bu32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfa89 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x5c17f0bcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb8aa3b29u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bface => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xc900daa2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfb13 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xfbcff799u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x9a209a84u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffdu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfb58 => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xd1cf79acu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb17217f7u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffeu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfb9d => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be930u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfbf4 => {
  cpu.poll(); pc = cpu.r32(0x20bfbfcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bfbfc => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0xfcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.edi); cpu.esp = v; }
  pc = 0x20bfc02; continue;
 }
 0x20bfc02 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x20bfc02; continue; }
  pc = 0x20bfc06; continue;
 }
 0x20bfc06 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), cpu.eax); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x20bfc0c, "Into");
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xdd020bfcu32; cpu.esp = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  pc = 0x20bfc1c; continue;
 }
 0x20bfc1c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfc20); cpu.push(0x20bfc25); f_020c17a0(cpu); if cpu.returned_to != 0x20bfc25 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfc29 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bfc67); cpu.push(0x20bfc6c); f_020c1281(cpu); if cpu.returned_to != 0x20bfc6c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfc70 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfc74); cpu.push(0x20bfc79); f_020c176a(cpu); if cpu.returned_to != 0x20bfc79 { return; }
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfcbc => {
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfcdd => {
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfcfe => {
  cpu.poll(); pc = cpu.r32(0x20bfd06u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bfd06 => {
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let a = cpu.edi; let b = cpu.eax; { let v: u32 = b; cpu.edi = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.movs(1, false); }
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20bfd1f; continue; }
  pc = 0x20bfd1c; continue;
 }
 0x20bfd1c => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  pc = 0x20bfd1f; continue;
 }
 0x20bfd1f => {
  { cpu.df = true; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.esi | cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20bfd26; continue;
 }
 0x20bfd26 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bfd2e); cpu.push(0x20bfd33); f_020c1099(cpu); if cpu.returned_to != 0x20bfd33 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x20be982u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfd50 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bfd8e); cpu.push(0x20bfd93); f_020c1268(cpu); if cpu.returned_to != 0x20bfd93 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfd97 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfd9b); cpu.push(0x20bfda0); f_020c1974(cpu); if cpu.returned_to != 0x20bfda0 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfda4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfdb4); cpu.push(0x20bfdb9); f_020c1753(cpu); if cpu.returned_to != 0x20bfdb9 { return; }
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfdf2); cpu.push(0x20bfdf7); f_020c175d(cpu); if cpu.returned_to != 0x20bfdf7 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfdfc => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfe00); cpu.push(0x20bfe05); f_020c1753(cpu); if cpu.returned_to != 0x20bfe05 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfe09 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x20bfe0d); cpu.push(0x20bfe12); f_020c175d(cpu); if cpu.returned_to != 0x20bfe12 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bfe16 => {
  cpu.poll(); pc = cpu.r32(0x20bfe1eu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bfe1e => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(8)).wrapping_add(cpu.bases[3]))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20bfe3e; continue;
 }
 0x20bfe3e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bfe95); cpu.push(0x20bfe9a); f_020c05ca(cpu); if cpu.returned_to != 0x20bfe9a { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bfe9f => {
  cpu.poll(); pc = cpu.r32(0x20bfea7u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20bfec7 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfed3 => {
  { let v: u16 = 0x33fu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x20bece0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bfef9 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff05); cpu.push(0x20bff0a); f_020c0404(cpu); if cpu.returned_to != 0x20bff0a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff0e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff1a); cpu.push(0x20bff1f); f_020c0838(cpu); if cpu.returned_to != 0x20bff1f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff23 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff2f); cpu.push(0x20bff34); f_020c03f6(cpu); if cpu.returned_to != 0x20bff34 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff38 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff44); cpu.push(0x20bff49); f_020c03f6(cpu); if cpu.returned_to != 0x20bff49 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff4d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff59); cpu.push(0x20bff5e); f_020c0663(cpu); if cpu.returned_to != 0x20bff5e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff62 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20bff6e); cpu.push(0x20bff73); f_020c0663(cpu); if cpu.returned_to != 0x20bff73 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20bff77 => {
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bff96 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20bffc2 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bffca); cpu.push(0x20bffcf); f_020c05ca(cpu); if cpu.returned_to != 0x20bffcf { return; }
  cpu.poll(); pc = 0x20beed4; continue;
 }
 0x20bffd4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20bffdc); cpu.push(0x20bffe1); f_020c05ca(cpu); if cpu.returned_to != 0x20bffe1 { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20bffe6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bee99; continue;
 }
 0x20c0012 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c001e); cpu.push(0x20c0023); f_020c0404(cpu); if cpu.returned_to != 0x20c0023 { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c005d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c0069); cpu.push(0x20c006e); f_020c0838(cpu); if cpu.returned_to != 0x20c006e { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c00a8 => {
  cpu.poll(); pc = cpu.r32(0x20c00b0u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20c00b0 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u8 = cpu.rol8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 1u32); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20c00d0; continue;
 }
 0x20c00d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20c0127); cpu.push(0x20c012c); f_020c05ca(cpu); if cpu.returned_to != 0x20c012c { return; }
  cpu.poll(); pc = 0x20bee82; continue;
 }
 0x20c0131 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c013d); cpu.push(0x20c0142); f_020c03f6(cpu); if cpu.returned_to != 0x20c0142 { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c017c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c0188); cpu.push(0x20c018d); f_020c03f6(cpu); if cpu.returned_to != 0x20c018d { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c01c7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c01d3); cpu.push(0x20c01d8); f_020c0663(cpu); if cpu.returned_to != 0x20c01d8 { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c0212 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x20c021e); cpu.push(0x20c0223); f_020c0663(cpu); if cpu.returned_to != 0x20c0223 { return; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c025d => {
  cpu.poll(); pc = cpu.r32(0x20c0265u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20c0265 => {
  { cpu.port_out(0x3u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20c0285; continue;
 }
 0x20c0285 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x20c02c5); cpu.push(0x20c02ca); f_020c0e80(cpu); if cpu.returned_to != 0x20c02ca { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c02ce => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x20be8e0u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x20be8e2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x20be92eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
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
  cpu.flat_ds(0x20c0329); cpu.push(0x20c032e); f_020c09b3(cpu); if cpu.returned_to != 0x20c032e { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20c0333 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x20c033b); cpu.push(0x20c0340); f_020c1099(cpu); if cpu.returned_to != 0x20c0340 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x20be982u32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x20c035d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20c036b); cpu.push(0x20c0370); f_020c0cf3(cpu); if cpu.returned_to != 0x20c0370 { return; }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20c0375 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x20be8deu32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x20c0387); cpu.push(0x20c038c); f_020c09b3(cpu); if cpu.returned_to != 0x20c038c { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x20c03b4; continue; }
  pc = 0x20c0399; continue;
 }
 0x20c0399 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x20c03a5; continue; }
  pc = 0x20c039d; continue;
 }
 0x20c039d => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c03b2; continue;
 }
 0x20c03a5 => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c03b2; continue;
 }
 0x20c03b2 => {
  pc = 0x20c03da; continue;
 }
 0x20c03b4 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x20c03da; continue; }
  pc = 0x20c03ca; continue;
 }
 0x20c03ca => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x20c03d6; continue; }
  pc = 0x20c03ce; continue;
 }
 0x20c03ce => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c03da; continue;
 }
 0x20c03d6 => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x20c03da; continue;
 }
 0x20c03da => {
  { let v: u16 = (cpu.edx as u16) | (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 0x20c03e6 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x20bece0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bee14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bee14); }
 let mut pc: u32 = 0x20bee14;
 loop { match pc {
 0x20bee14 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 25u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x20bee42; continue; }
  pc = 0x20bee25; continue;
 }
 0x20bee25 => {
  if !cpu.zf { pc = 0x20bee2e; continue; }
  pc = 0x20bee27; continue;
 }
 0x20bee27 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x20bee2e; continue; }
  pc = 0x20bee2c; continue;
 }
 0x20bee2c => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x20bee2e; continue;
 }
 0x20bee2e => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x100u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x20bee42; continue; }
  pc = 0x20bee37; continue;
 }
 0x20bee37 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bee42; continue;
 }
 0x20bee42 => {
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_020bece0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bee50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bee50); }
 let mut pc: u32 = 0x20bee50;
 loop { match pc {
 0x20bee50 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xfffff800u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 22u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x20bee7b; continue; }
  pc = 0x20bee60; continue;
 }
 0x20bee60 => {
  if !cpu.zf { pc = 0x20bee64; continue; }
  pc = 0x20bee62; continue;
 }
 0x20bee62 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x20bee64; continue;
 }
 0x20bee64 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x800u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x20bee7b; continue; }
  pc = 0x20bee70; continue;
 }
 0x20bee70 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bee7b; continue;
 }
 0x20bee7b => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_020bece0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020beef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20beef0); }
 let mut pc: u32 = 0x20beef0;
 loop { match pc {
 0x20beef0 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bef3d; continue; }
  pc = 0x20beefa; continue;
 }
 0x20beefa => {
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
  let t = cpu.r32(0x217b949u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20bef17); cpu.push(0x20bef1d); cpu.call(t); if cpu.returned_to != 0x20bef1d { return; }
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
  pc = 0x20bef3d; continue;
 }
 0x20bef3d => {
  cpu.flat_ds(0x20bef3d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef3e); }
 let mut pc: u32 = 0x20bef3e;
 loop { match pc {
 0x20bef3e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef44); }
 let mut pc: u32 = 0x20bef44;
 loop { match pc {
 0x20bef44 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef4a); }
 let mut pc: u32 = 0x20bef4a;
 loop { match pc {
 0x20bef4a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef50); }
 let mut pc: u32 = 0x20bef50;
 loop { match pc {
 0x20bef50 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef56); }
 let mut pc: u32 = 0x20bef56;
 loop { match pc {
 0x20bef56 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef5e); }
 let mut pc: u32 = 0x20bef5e;
 loop { match pc {
 0x20bef5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef64); }
 let mut pc: u32 = 0x20bef64;
 loop { match pc {
 0x20bef64 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef6a); }
 let mut pc: u32 = 0x20bef6a;
 loop { match pc {
 0x20bef6a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef70); }
 let mut pc: u32 = 0x20bef70;
 loop { match pc {
 0x20bef70 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef78); }
 let mut pc: u32 = 0x20bef78;
 loop { match pc {
 0x20bef78 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef80); }
 let mut pc: u32 = 0x20bef80;
 loop { match pc {
 0x20bef80 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef88); }
 let mut pc: u32 = 0x20bef88;
 loop { match pc {
 0x20bef88 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef90); }
 let mut pc: u32 = 0x20bef90;
 loop { match pc {
 0x20bef90 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bef9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bef9a); }
 let mut pc: u32 = 0x20bef9a;
 loop { match pc {
 0x20bef9a => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befa4); }
 let mut pc: u32 = 0x20befa4;
 loop { match pc {
 0x20befa4 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befac); }
 let mut pc: u32 = 0x20befac;
 loop { match pc {
 0x20befac => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befb4); }
 let mut pc: u32 = 0x20befb4;
 loop { match pc {
 0x20befb4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befbd); }
 let mut pc: u32 = 0x20befbd;
 loop { match pc {
 0x20befbd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befc6); }
 let mut pc: u32 = 0x20befc6;
 loop { match pc {
 0x20befc6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befcf); }
 let mut pc: u32 = 0x20befcf;
 loop { match pc {
 0x20befcf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befd8); }
 let mut pc: u32 = 0x20befd8;
 loop { match pc {
 0x20befd8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befe3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befe3); }
 let mut pc: u32 = 0x20befe3;
 loop { match pc {
 0x20befe3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020befee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20befee); }
 let mut pc: u32 = 0x20befee;
 loop { match pc {
 0x20befee => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020beff7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20beff7); }
 let mut pc: u32 = 0x20beff7;
 loop { match pc {
 0x20beff7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bf000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bf000); }
 let mut pc: u32 = 0x20bf000;
 loop { match pc {
 0x20bf000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x18u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x20be9ecu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x20bf01b); cpu.call(t); if cpu.returned_to != 0x20bf01b { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x38u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x20bf036; continue; }
  pc = 0x20bf027; continue;
 }
 0x20bf027 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  pc = 0x20bf036; continue;
 }
 0x20bf036 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bfea7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bfea7); }
 let mut pc: u32 = 0x20bfea7;
 loop { match pc {
 0x20bfea7 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  cpu.call(0x20bfeaf); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c03f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c03f6); } r_020c03f6(cpu, 0x20c03f6); }
pub fn f_020c0404(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c0404); } r_020c03f6(cpu, 0x20c0404); }
fn r_020c03f6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c03f6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c040b; continue;
 }
 0x20c0404 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c040b; continue;
 }
 0x20c040b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20c0421); f_020c045e(cpu); if cpu.returned_to != 0x20c0421 { return; }
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
pub fn f_020c042e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c042e); }
 let mut pc: u32 = 0x20c042e;
 loop { match pc {
 0x20c042e => {
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
  cpu.push(0x20c044f); f_020c045e(cpu); if cpu.returned_to != 0x20c044f { return; }
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
pub fn f_020c045e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c045e); }
 let mut pc: u32 = 0x20c045e;
 loop { match pc {
 0x20c045e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c0481; continue; }
  pc = 0x20c0462; continue;
 }
 0x20c0462 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c0481; continue; }
  pc = 0x20c0466; continue;
 }
 0x20c0466 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c047e; continue; }
  pc = 0x20c046b; continue;
 }
 0x20c046b => {
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16) | (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20c047d; continue; }
  pc = 0x20c047b; continue;
 }
 0x20c047b => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x20c047d; continue;
 }
 0x20c047d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c047e => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c0481; continue;
 }
 0x20c0481 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20c0492; continue; }
  pc = 0x20c0485; continue;
 }
 0x20c0485 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x20c0492; continue; }
  pc = 0x20c0489; continue;
 }
 0x20c0489 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x20c0492; continue; }
  pc = 0x20c0491; continue;
 }
 0x20c0491 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0492 => {
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
  if cpu.zf { pc = 0x20c04e1; continue; }
  pc = 0x20c04c3; continue;
 }
 0x20c04c3 => {
  if !cpu.cf { pc = 0x20c04cd; continue; }
  pc = 0x20c04c5; continue;
 }
 0x20c04c5 => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.esi; let b = cpu.edx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x20c04cd; continue;
 }
 0x20c04cd => {
  { cpu.sub16((cpu.ecx as u16), 0x40u16); }
  if (cpu.cf || cpu.zf) { pc = 0x20c04e1; continue; }
  pc = 0x20c04d3; continue;
 }
 0x20c04d3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c04e1 => {
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x20c04f6; continue; }
  pc = 0x20c04e7; continue;
 }
 0x20c04e7 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x20c04f6; continue;
 }
 0x20c04f6 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c0529; continue; }
  pc = 0x20c04fd; continue;
 }
 0x20c04fd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x20c051b; continue; }
  pc = 0x20c0505; continue;
 }
 0x20c0505 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x20c0517; continue; }
  pc = 0x20c0513; continue;
 }
 0x20c0513 => {
  { let v: u32 = cpu.edi | cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x20c0517; continue;
 }
 0x20c0517 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x20c051b; continue;
 }
 0x20c051b => {
  { let v: u32 = cpu.shrd32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.edi | cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x20c0529; continue;
 }
 0x20c0529 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x20c055a; continue; }
  pc = 0x20c0532; continue;
 }
 0x20c0532 => {
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x20c054b; continue; }
  pc = 0x20c0537; continue;
 }
 0x20c0537 => {
  { cpu.and32(cpu.edi, 0x7fffffffu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c054b; continue;
 }
 0x20c054b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x20c055a; continue;
 }
 0x20c055a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20c05b1; continue; }
  pc = 0x20c0562; continue;
 }
 0x20c0562 => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c05b3; continue; }
  pc = 0x20c0567; continue;
 }
 0x20c0567 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x20c057a; continue; }
  pc = 0x20c056c; continue;
 }
 0x20c056c => {
  { let v: u32 = cpu.rol32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.ror32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x20c0570; continue;
 }
 0x20c0570 => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c05b3; continue; }
  pc = 0x20c0574; continue;
 }
 0x20c0574 => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x20c0570; continue; }
  pc = 0x20c057a; continue;
 }
 0x20c057a => {
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x20c05bd; continue; }
  pc = 0x20c0587; continue;
 }
 0x20c0587 => {
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x20c05af; continue; }
  pc = 0x20c058e; continue;
 }
 0x20c058e => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x20c0596; continue; }
  pc = 0x20c0592; continue;
 }
 0x20c0592 => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0596; continue;
 }
 0x20c0596 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x20c05af; continue; }
  pc = 0x20c059e; continue;
 }
 0x20c059e => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x20c05bd; continue; }
  pc = 0x20c05af; continue;
 }
 0x20c05af => {
  pc = 0x20c05b3; continue;
 }
 0x20c05b1 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x20c05b3; continue;
 }
 0x20c05b3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c05bd => {
  { let v: u16 = 0x7fffu16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c05b3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c05ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c05ca); }
 let mut pc: u32 = 0x20c05ca;
 loop { match pc {
 0x20c05ca => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c05e2; continue; }
  pc = 0x20c05d9; continue;
 }
 0x20c05d9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  if !cpu.zf { pc = 0x20c0635; continue; }
  pc = 0x20c05e2; continue;
 }
 0x20c05e2 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c05f8; continue; }
  pc = 0x20c05ef; continue;
 }
 0x20c05ef => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.esi = v; }
  if !cpu.zf { pc = 0x20c0635; continue; }
  pc = 0x20c05f8; continue;
 }
 0x20c05f8 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.sf { pc = 0x20c063d; continue; }
  pc = 0x20c0607; continue;
 }
 0x20c0607 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x8u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x20c061d; continue; }
  pc = 0x20c0611; continue;
 }
 0x20c0611 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x20c061d; continue; }
  pc = 0x20c0619; continue;
 }
 0x20c0619 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  pc = 0x20c061d; continue;
 }
 0x20c061d => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.zf { pc = 0x20c0630; continue; }
  pc = 0x20c0624; continue;
 }
 0x20c0624 => {
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.eax; cpu.esi = v; }
  pc = 0x20c0628; continue;
 }
 0x20c0628 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, 0x0u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x20c0630; continue;
 }
 0x20c0630 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0635 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c063d => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c0628; continue; }
  pc = 0x20c0660; continue;
 }
 0x20c0660 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0663(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0663); }
 let mut pc: u32 = 0x20c0663;
 loop { match pc {
 0x20c0663 => {
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
  cpu.push(0x20c0680); f_020c068d(cpu); if cpu.returned_to != 0x20c0680 { return; }
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
pub fn f_020c068d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c068d); }
 let mut pc: u32 = 0x20c068d;
 loop { match pc {
 0x20c068d => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20c06d8; continue; }
  pc = 0x20c0691; continue;
 }
 0x20c0691 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x20c06d8; continue; }
  pc = 0x20c0695; continue;
 }
 0x20c0695 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x20c06d8; continue; }
  pc = 0x20c069d; continue;
 }
 0x20c069d => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c06c2; continue; }
  pc = 0x20c06a1; continue;
 }
 0x20c06a1 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c06c2; continue; }
  pc = 0x20c06a5; continue;
 }
 0x20c06a5 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c06c2; continue; }
  pc = 0x20c06ac; continue;
 }
 0x20c06ac => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x20c06b5); f_020beef0(cpu); if cpu.returned_to != 0x20c06b5 { return; }
  { let v: u32 = 0xc0000000u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c06d7; continue;
 }
 0x20c06c2 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x83u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x20c06cb); f_020beef0(cpu); if cpu.returned_to != 0x20c06cb { return; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c06d7; continue;
 }
 0x20c06d7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c06d8 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c06e9; continue; }
  pc = 0x20c06dc; continue;
 }
 0x20c06dc => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c06e9; continue; }
  pc = 0x20c06e0; continue;
 }
 0x20c06e0 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c06e6; continue; }
  pc = 0x20c06e5; continue;
 }
 0x20c06e5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c06e6 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c06e9; continue;
 }
 0x20c06e9 => {
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
  if !cpu.zf { pc = 0x20c0724; continue; }
  pc = 0x20c071a; continue;
 }
 0x20c071a => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x20c071a; continue; }
  pc = 0x20c0724; continue;
 }
 0x20c0724 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0733; continue; }
  pc = 0x20c0729; continue;
 }
 0x20c0729 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.sf { cpu.poll(); pc = 0x20c0729; continue; }
  pc = 0x20c0733; continue;
 }
 0x20c0733 => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x3fffu16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x20c0756; continue; }
  pc = 0x20c073d; continue;
 }
 0x20c073d => {
  { cpu.sub16((cpu.edi as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x20c0756; continue; }
  pc = 0x20c0744; continue;
 }
 0x20c0744 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u16 = 0x7fffu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x20c082e; continue;
 }
 0x20c0756 => {
  { cpu.sub16((cpu.edi as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x20c0767; continue; }
  pc = 0x20c075c; continue;
 }
 0x20c075c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x20c082e; continue;
 }
 0x20c0767 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c0779; continue; }
  pc = 0x20c0776; continue;
 }
 0x20c0776 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20c0779; continue;
 }
 0x20c0779 => {
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
  if cpu.zf { pc = 0x20c079a; continue; }
  pc = 0x20c0795; continue;
 }
 0x20c0795 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  pc = 0x20c079a; continue;
 }
 0x20c079a => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edx); cpu.edi = v; }
  if cpu.zf { pc = 0x20c07b4; continue; }
  pc = 0x20c07a2; continue;
 }
 0x20c07a2 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c07a2; continue; }
  pc = 0x20c07b4; continue;
 }
 0x20c07b4 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c07c9; continue; }
  pc = 0x20c07bf; continue;
 }
 0x20c07bf => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c07c9; continue;
 }
 0x20c07c9 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20c0800; continue; }
  pc = 0x20c07d4; continue;
 }
 0x20c07d4 => {
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
  if cpu.zf { pc = 0x20c0800; continue; }
  pc = 0x20c07e9; continue;
 }
 0x20c07e9 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c07e9; continue; }
  pc = 0x20c0800; continue;
 }
 0x20c0800 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x20c0812; continue; }
  pc = 0x20c080d; continue;
 }
 0x20c080d => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x20c0812; continue;
 }
 0x20c0812 => {
  { let v: u16 = cpu.or16((cpu.edi as u16), (cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c082e; continue; }
  pc = 0x20c0817; continue;
 }
 0x20c0817 => {
  if !cpu.zf { pc = 0x20c081d; continue; }
  pc = 0x20c0819; continue;
 }
 0x20c0819 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c0823; continue;
 }
 0x20c081d => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0823; continue;
 }
 0x20c0823 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x20c082e; continue;
 }
 0x20c082e => {
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
pub fn f_020c0838(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0838); }
 let mut pc: u32 = 0x20c0838;
 loop { match pc {
 0x20c0838 => {
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
  cpu.push(0x20c0855); f_020c0862(cpu); if cpu.returned_to != 0x20c0855 { return; }
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
pub fn f_020c0862(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0862); }
 let mut pc: u32 = 0x20c0862;
 loop { match pc {
 0x20c0862 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c0873; continue; }
  pc = 0x20c0866; continue;
 }
 0x20c0866 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c0873; continue; }
  pc = 0x20c086a; continue;
 }
 0x20c086a => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0870; continue; }
  pc = 0x20c086f; continue;
 }
 0x20c086f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0870 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c0873; continue;
 }
 0x20c0873 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20c088a; continue; }
  pc = 0x20c0877; continue;
 }
 0x20c0877 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x20c088a; continue; }
  pc = 0x20c087b; continue;
 }
 0x20c087b => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x20c088a; continue; }
  pc = 0x20c0883; continue;
 }
 0x20c0883 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c088a => {
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
  if cpu.cf { pc = 0x20c08ba; continue; }
  pc = 0x20c08ae; continue;
 }
 0x20c08ae => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x20c08ba; continue; }
  pc = 0x20c08b5; continue;
 }
 0x20c08b5 => {
  pc = 0x20c0951; continue;
 }
 0x20c08ba => {
  { cpu.sub16((cpu.ecx as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x20c08cb; continue; }
  pc = 0x20c08c0; continue;
 }
 0x20c08c0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x20c0947; continue;
 }
 0x20c08cb => {
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
  if cpu.sf { pc = 0x20c0902; continue; }
  pc = 0x20c08fa; continue;
 }
 0x20c08fa => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0902; continue;
 }
 0x20c0902 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.cf { pc = 0x20c092e; continue; }
  pc = 0x20c0906; continue;
 }
 0x20c0906 => {
  if !cpu.zf { pc = 0x20c0915; continue; }
  pc = 0x20c0908; continue;
 }
 0x20c0908 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x20c0915; continue; }
  pc = 0x20c0911; continue;
 }
 0x20c0911 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x20c0915; continue;
 }
 0x20c0915 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x20c092e; continue; }
  pc = 0x20c091d; continue;
 }
 0x20c091d => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x20c0951; continue; }
  pc = 0x20c092e; continue;
 }
 0x20c092e => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c0947; continue; }
  pc = 0x20c0933; continue;
 }
 0x20c0933 => {
  if !cpu.zf { pc = 0x20c0939; continue; }
  pc = 0x20c0935; continue;
 }
 0x20c0935 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c093c; continue;
 }
 0x20c0939 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c093c; continue;
 }
 0x20c093c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0947; continue;
 }
 0x20c0947 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0951 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20c0947; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c095e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c095e); } r_020c095e(cpu, 0x20c095e); }
pub fn f_020c09b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c09b3); } r_020c095e(cpu, 0x20c09b3); }
pub fn f_020c09b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c09b8); } r_020c095e(cpu, 0x20c09b8); }
fn r_020c095e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c095e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0xa0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x20c09bb; continue;
 }
 0x20c09b3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x20c09bb; continue;
 }
 0x20c09b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x20c09bb; continue;
 }
 0x20c09bb => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c0a1d; continue; }
  pc = 0x20c09d3; continue;
 }
 0x20c09d3 => {
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c0a15; continue; }
  pc = 0x20c09d9; continue;
 }
 0x20c09d9 => {
  { let v: u8 = (cpu.ebx as u8) & 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), (cpu.ebx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c0a15; continue; }
  pc = 0x20c09e0; continue;
 }
 0x20c09e0 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.rcr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x20c09f4; continue; }
  pc = 0x20c09f0; continue;
 }
 0x20c09f0 => {
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c0a01; continue;
 }
 0x20c09f4 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shld32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20c0a01; continue;
 }
 0x20c0a01 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x20c0a12; continue; }
  pc = 0x20c0a10; continue;
 }
 0x20c0a10 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20c0a12; continue;
 }
 0x20c0a12 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0a15 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0a1d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0a22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c0a22); } r_020c0a22(cpu, 0x20c0a22); }
pub fn f_020c0a33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c0a33); } r_020c0a22(cpu, 0x20c0a33); }
fn r_020c0a22(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c0a22 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x20c0a32; continue; }
  pc = 0x20c0a29; continue;
 }
 0x20c0a29 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xbfffu32; cpu.edx = v; }
  pc = 0x20c0a3b; continue;
 }
 0x20c0a32 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x20c0a33; continue;
 }
 0x20c0a33 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3fffu32; cpu.edx = v; }
  pc = 0x20c0a3b; continue;
 }
 0x20c0a3b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20c0a56; continue; }
  pc = 0x20c0a40; continue;
 }
 0x20c0a40 => {
  cpu.unimplemented(0x20c0a40, "Bsr");
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20c0a5a; continue;
 }
 0x20c0a56 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x20c0a5a; continue;
 }
 0x20c0a5a => {
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
pub fn f_020c0a68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0a68); }
 let mut pc: u32 = 0x20c0a68;
 loop { match pc {
 0x20c0a68 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 11u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 20u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7ffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c0ab3; continue; }
  pc = 0x20c0a7c; continue;
 }
 0x20c0a7c => {
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if cpu.zf { pc = 0x20c0a8a; continue; }
  pc = 0x20c0a83; continue;
 }
 0x20c0a83 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3c00u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0aab; continue;
 }
 0x20c0a8a => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c0a98; continue; }
  pc = 0x20c0a96; continue;
 }
 0x20c0a96 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c0a98; continue;
 }
 0x20c0a98 => {
  if cpu.zf { pc = 0x20c0aab; continue; }
  pc = 0x20c0a9a; continue;
 }
 0x20c0a9a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x20c0aa4); f_020beef0(cpu); if cpu.returned_to != 0x20c0aa4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx | 0x40000000u32; cpu.edx = v; }
  pc = 0x20c0aab; continue;
 }
 0x20c0aab => {
  { let v: u32 = cpu.edx | 0x80000000u32; cpu.edx = v; }
  pc = 0x20c0ad8; continue;
 }
 0x20c0ab3 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c0abf; continue; }
  pc = 0x20c0ab7; continue;
 }
 0x20c0ab7 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c0abf; continue; }
  pc = 0x20c0abb; continue;
 }
 0x20c0abb => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x20c0ad8; continue;
 }
 0x20c0abf => {
  { let v: u16 = 0x3c01u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c0acc; continue; }
  pc = 0x20c0ac7; continue;
 }
 0x20c0ac7 => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0acc; continue;
 }
 0x20c0acc => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x20c0ad8; continue; }
  pc = 0x20c0ad0; continue;
 }
 0x20c0ad0 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20c0acc; continue;
 }
 0x20c0ad8 => {
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
pub fn f_020c0ae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0ae8); }
 let mut pc: u32 = 0x20c0ae8;
 loop { match pc {
 0x20c0ae8 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 25u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x20c0b17; continue; }
  pc = 0x20c0b00; continue;
 }
 0x20c0b00 => {
  if !cpu.zf { pc = 0x20c0b09; continue; }
  pc = 0x20c0b02; continue;
 }
 0x20c0b02 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c0b09; continue; }
  pc = 0x20c0b07; continue;
 }
 0x20c0b07 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x20c0b09; continue;
 }
 0x20c0b09 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x20c0b17; continue; }
  pc = 0x20c0b10; continue;
 }
 0x20c0b10 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0b17; continue;
 }
 0x20c0b17 => {
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c0b63; continue; }
  pc = 0x20c0b22; continue;
 }
 0x20c0b22 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x20c0b3a; continue; }
  pc = 0x20c0b29; continue;
 }
 0x20c0b29 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0b63; continue;
 }
 0x20c0b3a => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0xc080u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x20c0b45; continue; }
  pc = 0x20c0b41; continue;
 }
 0x20c0b41 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c0b63; continue;
 }
 0x20c0b45 => {
  { cpu.sub16((cpu.ecx as u16), 0xffu16); }
  if (cpu.sf != cpu.of) { pc = 0x20c0b58; continue; }
  pc = 0x20c0b4c; continue;
 }
 0x20c0b4c => {
  { let v: u32 = 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0b63; continue;
 }
 0x20c0b58 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.ecx, 8u32); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0b63; continue;
 }
 0x20c0b63 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0b67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0b67); }
 let mut pc: u32 = 0x20c0b67;
 loop { match pc {
 0x20c0b67 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffff800u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 22u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x20c0b94; continue; }
  pc = 0x20c0b7f; continue;
 }
 0x20c0b7f => {
  if !cpu.zf { pc = 0x20c0b83; continue; }
  pc = 0x20c0b81; continue;
 }
 0x20c0b81 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  pc = 0x20c0b83; continue;
 }
 0x20c0b83 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x20c0b94; continue; }
  pc = 0x20c0b8d; continue;
 }
 0x20c0b8d => {
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0b94; continue;
 }
 0x20c0b94 => {
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xc400u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if !cpu.cf { pc = 0x20c0bca; continue; }
  pc = 0x20c0ba9; continue;
 }
 0x20c0ba9 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0bb9; continue; }
  pc = 0x20c0bae; continue;
 }
 0x20c0bae => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 12u32); cpu.edx = v; }
  pc = 0x20c0bc3; continue;
 }
 0x20c0bb9 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ecx, 11u32); cpu.edx = v; }
  pc = 0x20c0bc3; continue;
 }
 0x20c0bc3 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x20c0c2a; continue;
 }
 0x20c0bca => {
  { cpu.sub16((cpu.ecx as u16), 0xc400u16); }
  if cpu.cf { pc = 0x20c0c04; continue; }
  pc = 0x20c0bd1; continue;
 }
 0x20c0bd1 => {
  { cpu.sub16((cpu.ecx as u16), 0xffccu16); }
  if (cpu.sf != cpu.of) { pc = 0x20c0bfe; continue; }
  pc = 0x20c0bd7; continue;
 }
 0x20c0bd7 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x20c0bec; continue; }
  pc = 0x20c0be3; continue;
 }
 0x20c0be3 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x20c0bec; continue;
 }
 0x20c0bec => {
  { let v: u32 = cpu.shrd32(cpu.esi, cpu.eax, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x20c0c02; continue;
 }
 0x20c0bfe => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x20c0c02; continue;
 }
 0x20c0c02 => {
  pc = 0x20c0c2a; continue;
 }
 0x20c0c04 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 11u32); cpu.edx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x7ff00000u32; cpu.edx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x43ffu16); }
  if cpu.zf { pc = 0x20c0c2a; continue; }
  pc = 0x20c0c1f; continue;
 }
 0x20c0c1f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x84u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x20c0c29); f_020beef0(cpu); if cpu.returned_to != 0x20c0c29 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x20c0c2a; continue;
 }
 0x20c0c2a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0c2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0c2e); }
 let mut pc: u32 = 0x20c0c2e;
 loop { match pc {
 0x20c0c2e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 23u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0xffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c0c67; continue; }
  pc = 0x20c0c40; continue;
 }
 0x20c0c40 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x20c0c4f; continue; }
  pc = 0x20c0c48; continue;
 }
 0x20c0c48 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3f80u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c0c5d; continue;
 }
 0x20c0c4f => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20c0c5d; continue; }
  pc = 0x20c0c58; continue;
 }
 0x20c0c58 => {
  { let v: u32 = cpu.eax | 0x40000000u32; cpu.eax = v; }
  pc = 0x20c0c5d; continue;
 }
 0x20c0c5d => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, 0x80000000u32); cpu.eax = v; }
  pc = 0x20c0c67; continue;
 }
 0x20c0c67 => {
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
pub fn f_020c0cf3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0cf3); }
 let mut pc: u32 = 0x20c0cf3;
 loop { match pc {
 0x20c0cf3 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x20c0d08; continue; }
  pc = 0x20c0d03; continue;
 }
 0x20c0d03 => {
  { cpu.sub16((cpu.edx as u16), 0x800u16); }
  pc = 0x20c0d08; continue;
 }
 0x20c0d08 => {
  if !cpu.zf { pc = 0x20c0d13; continue; }
  pc = 0x20c0d0a; continue;
 }
 0x20c0d0a => {
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x20c0d13; continue; }
  pc = 0x20c0d0e; continue;
 }
 0x20c0d0e => {
  { let v: u16 = (cpu.edx as u16) ^ 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x20c0d13; continue;
 }
 0x20c0d13 => {
  { cpu.sub16((cpu.edx as u16), 0xc00u16); }
  if !cpu.zf { pc = 0x20c0d1e; continue; }
  pc = 0x20c0d1a; continue;
 }
 0x20c0d1a => {
  { let v: u16 = 0x400u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x20c0d1e; continue;
 }
 0x20c0d1e => {
  { let v: u16 = (cpu.ebx as u16) & 0x7fffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x403fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c0d2c; continue; }
  pc = 0x20c0d2a; continue;
 }
 0x20c0d2a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0d2c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c0d82; continue; }
  pc = 0x20c0d34; continue;
 }
 0x20c0d34 => {
  { let v: u32 = cpu.r32(0x20c0c73u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x20c0d80; continue; }
  pc = 0x20c0d48; continue;
 }
 0x20c0d48 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x20c0d6d; continue; }
  pc = 0x20c0d4e; continue;
 }
 0x20c0d4e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c0d80; continue; }
  pc = 0x20c0d58; continue;
 }
 0x20c0d58 => {
  if !cpu.zf { pc = 0x20c0d6d; continue; }
  pc = 0x20c0d5a; continue;
 }
 0x20c0d5a => {
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x20c0d63; continue; }
  pc = 0x20c0d5e; continue;
 }
 0x20c0d5e => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x20c0d65; continue;
 }
 0x20c0d63 => {
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x20c0d65; continue;
 }
 0x20c0d65 => {
  if cpu.zf { pc = 0x20c0d80; continue; }
  pc = 0x20c0d6b; continue;
 }
 0x20c0d6b => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0d6d; continue;
 }
 0x20c0d6d => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20c0d80; continue; }
  pc = 0x20c0d71; continue;
 }
 0x20c0d71 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x20c0d80; continue; }
  pc = 0x20c0d79; continue;
 }
 0x20c0d79 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20c0d80; continue;
 }
 0x20c0d80 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0d82 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c0de5; continue; }
  pc = 0x20c0d92; continue;
 }
 0x20c0d92 => {
  { let v: u32 = cpu.r32(0x20c0c73u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi & cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) ^ cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x20c0de2; continue; }
  pc = 0x20c0da7; continue;
 }
 0x20c0da7 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x20c0dd2; continue; }
  pc = 0x20c0dad; continue;
 }
 0x20c0dad => {
  { let v: u32 = cpu.rol32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c0de2; continue; }
  pc = 0x20c0db9; continue;
 }
 0x20c0db9 => {
  if !cpu.zf { pc = 0x20c0dd2; continue; }
  pc = 0x20c0dbb; continue;
 }
 0x20c0dbb => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20c0dd6; continue; }
  pc = 0x20c0dc3; continue;
 }
 0x20c0dc3 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x20c0dca; continue; }
  pc = 0x20c0dc7; continue;
 }
 0x20c0dc7 => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x20c0dca; continue;
 }
 0x20c0dca => {
  if cpu.zf { pc = 0x20c0de2; continue; }
  pc = 0x20c0dd0; continue;
 }
 0x20c0dd0 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20c0dd2; continue;
 }
 0x20c0dd2 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  if cpu.zf { pc = 0x20c0de2; continue; }
  pc = 0x20c0dd6; continue;
 }
 0x20c0dd6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x20c0de2; continue; }
  pc = 0x20c0ddb; continue;
 }
 0x20c0ddb => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20c0de2; continue;
 }
 0x20c0de2 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0de5 => {
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x20c0e1a; continue; }
  pc = 0x20c0dec; continue;
 }
 0x20c0dec => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x20c0e0a; continue; }
  pc = 0x20c0df2; continue;
 }
 0x20c0df2 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0e1a; continue; }
  pc = 0x20c0dfa; continue;
 }
 0x20c0dfa => {
  { cpu.sub32(cpu.esi, 0x80000000u32); }
  if !cpu.zf { pc = 0x20c0e0a; continue; }
  pc = 0x20c0e02; continue;
 }
 0x20c0e02 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20c0e1a; continue; }
  pc = 0x20c0e0a; continue;
 }
 0x20c0e0a => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c0e1a => {
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
pub fn f_020c0e26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0e26); }
 let mut pc: u32 = 0x20c0e26;
 loop { match pc {
 0x20c0e26 => {
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
  cpu.unimplemented(0x20c0e43, "Aas");
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0xb99d0a6bu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x171bffbu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xe38e2727u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  {  }
  cpu.unimplemented(0x20c0e57, "Aas");
  { cpu.df = false; }
  { let v = 0x2472dd9cu32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.df = false; }
  { let v: u32 = 0x94ebadc1u32; cpu.edi = v; }
  cpu.unimplemented(0x20c0e66, "Int3");
  cpu.unimplemented(0x20c0e67, "Int3");
  cpu.unimplemented(0x20c0e68, "Int3");
  cpu.unimplemented(0x20c0e69, "Int3");
  { cpu.df = false; }
  cpu.unimplemented(0x20c0e6b, "Aas");
  { let v: u32 = cpu.adc32(cpu.r32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0xbffdaau32); cpu.w32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x3fu8); }
  cpu.flat_ds(0x20c0e7d); f_020c0e80(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0e80); }
 let mut pc: u32 = 0x20c0e80;
 loop { match pc {
 0x20c0e80 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0ebe; continue; }
  pc = 0x20c0e8e; continue;
 }
 0x20c0e8e => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x20c0ea9; continue; }
  pc = 0x20c0e94; continue;
 }
 0x20c0e94 => {
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c0ebc; continue;
 }
 0x20c0ea9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c0ebc; continue;
 }
 0x20c0ebc => {
  pc = 0x20c0f32; continue;
 }
 0x20c0ebe => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c0ee6; continue; }
  pc = 0x20c0ec7; continue;
 }
 0x20c0ec7 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c0f32; continue;
 }
 0x20c0ee6 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x20c0eee); f_020c0663(cpu); if cpu.returned_to != 0x20c0eee { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c0ef5); f_020c0f36(cpu); if cpu.returned_to != 0x20c0ef5 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x20c0f18; continue; }
  pc = 0x20c0efc; continue;
 }
 0x20c0efc => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x20c0f16; continue; }
  pc = 0x20c0f00; continue;
 }
 0x20c0f00 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x4000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x20c0f16); f_020c042e(cpu); if cpu.returned_to != 0x20c0f16 { return; }
  pc = 0x20c0f16; continue;
 }
 0x20c0f16 => {
  pc = 0x20c0f32; continue;
 }
 0x20c0f18 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c0f32; continue; }
  pc = 0x20c0f1c; continue;
 }
 0x20c0f1c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xc000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x20c0f32); f_020c042e(cpu); if cpu.returned_to != 0x20c0f32 { return; }
  pc = 0x20c0f32; continue;
 }
 0x20c0f32 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c0f36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c0f36); }
 let mut pc: u32 = 0x20c0f36;
 loop { match pc {
 0x20c0f36 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if !cpu.zf { pc = 0x20c0f78; continue; }
  pc = 0x20c0f4c; continue;
 }
 0x20c0f4c => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x80000000u32); }
  if !cpu.zf { pc = 0x20c0f78; continue; }
  pc = 0x20c0f55; continue;
 }
 0x20c0f55 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x20c0f78; continue; }
  pc = 0x20c0f5a; continue;
 }
 0x20c0f5a => {
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
 0x20c0f78 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x20c0fae; continue; }
  pc = 0x20c0f82; continue;
 }
 0x20c0f82 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x20c0fa2); f_020c068d(cpu); if cpu.returned_to != 0x20c0fa2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c0fae; continue;
 }
 0x20c0fae => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3ffdu16); }
  if !cpu.zf { pc = 0x20c0fc9; continue; }
  pc = 0x20c0fb8; continue;
 }
 0x20c0fb8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8930a2f4u32); }
  if !cpu.zf { pc = 0x20c0fc9; continue; }
  pc = 0x20c0fc2; continue;
 }
 0x20c0fc2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xf66ab09bu32); }
  pc = 0x20c0fc9; continue;
 }
 0x20c0fc9 => {
  if (cpu.cf || cpu.zf) { pc = 0x20c103c; continue; }
  pc = 0x20c0fcb; continue;
 }
 0x20c0fcb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x20c0fe9); f_020c045e(cpu); if cpu.returned_to != 0x20c0fe9 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x20c1009); f_020c0862(cpu); if cpu.returned_to != 0x20c1009 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0xbfffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x20c101f); f_020c045e(cpu); if cpu.returned_to != 0x20c101f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c102f); f_020c068d(cpu); if cpu.returned_to != 0x20c102f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c103c; continue;
 }
 0x20c103c => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x20c0e26u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.push(0x20c104e); f_020c1ab2(cpu); if cpu.returned_to != 0x20c104e { return; }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x20c106a; continue; }
  pc = 0x20c1052; continue;
 }
 0x20c1052 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0x860a91c1u32; cpu.push(v); }
  { let v = 0x6b9b2c23u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c106a); f_020c042e(cpu); if cpu.returned_to != 0x20c106a { return; }
  pc = 0x20c106a; continue;
 }
 0x20c106a => {
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x20c108a; continue; }
  pc = 0x20c106e; continue;
 }
 0x20c106e => {
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c1086); f_020c042e(cpu); if cpu.returned_to != 0x20c1086 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c108a; continue;
 }
 0x20c108a => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x20c1093; continue; }
  pc = 0x20c108f; continue;
 }
 0x20c108f => {
  { let v: u8 = cpu.xor8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x20c1093; continue;
 }
 0x20c1093 => {
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
pub fn f_020c100e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c100e); }
 let mut pc: u32 = 0x20c100e;
 loop { match pc {
 0x20c100e => {
  { let v: u8 = cpu.add8(cpu.r8(0x6610c6c1u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x6610c6c1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xc6c1bfffu32; cpu.esi = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x20c101b, "Aas");
  cpu.unimplemented(0x20c101c, "Hlt");
  cpu.call(0x20c101d); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1099(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1099); }
 let mut pc: u32 = 0x20c1099;
 loop { match pc {
 0x20c1099 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x20c10ae; continue; }
  pc = 0x20c10a0; continue;
 }
 0x20c10a0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20c10ae; continue; }
  pc = 0x20c10a6; continue;
 }
 0x20c10a6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20c10ae; continue; }
  pc = 0x20c10ab; continue;
 }
 0x20c10ab => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c10ae => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x20c10ce; continue; }
  pc = 0x20c10b5; continue;
 }
 0x20c10b5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c10ce; continue; }
  pc = 0x20c10bb; continue;
 }
 0x20c10bb => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c10ce; continue; }
  pc = 0x20c10c0; continue;
 }
 0x20c10c0 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c10ce => {
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
  cpu.push(0x20c10ec); f_020c10ff(cpu); if cpu.returned_to != 0x20c10ec { return; }
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
pub fn f_020c10ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c10ff); }
 let mut pc: u32 = 0x20c10ff;
 loop { match pc {
 0x20c10ff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi & 0x7fffu32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0x7fffu32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x20c115e; continue; }
  pc = 0x20c1111; continue;
 }
 0x20c1111 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x20c1113; continue;
 }
 0x20c1113 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.zf { pc = 0x20c111f; continue; }
  pc = 0x20c1117; continue;
 }
 0x20c1117 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.zf { pc = 0x20c1121; continue; }
  pc = 0x20c111f; continue;
 }
 0x20c111f => {
  if !cpu.cf { pc = 0x20c1126; continue; }
  pc = 0x20c1121; continue;
 }
 0x20c1121 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { cpu.cf = true; }
  pc = 0x20c1126; continue;
 }
 0x20c1126 => {
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x20c113b; continue; }
  pc = 0x20c112f; continue;
 }
 0x20c112f => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.cf { cpu.poll(); pc = 0x20c1121; continue; }
  pc = 0x20c1135; continue;
 }
 0x20c1135 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { cpu.poll(); pc = 0x20c1113; continue; }
  pc = 0x20c1139; continue;
 }
 0x20c1139 => {
  cpu.poll(); pc = 0x20c1126; continue;
 }
 0x20c113b => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c1147; continue; }
  pc = 0x20c1141; continue;
 }
 0x20c1141 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c1147; continue; }
  pc = 0x20c1145; continue;
 }
 0x20c1145 => {
  pc = 0x20c115c; continue;
 }
 0x20c1147 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x20c1152; continue; }
  pc = 0x20c114b; continue;
 }
 0x20c114b => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20c1147; continue;
 }
 0x20c1152 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x7fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x20c115c; continue;
 }
 0x20c115c => {
  pc = 0x20c1162; continue;
 }
 0x20c115e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x20c1162; continue;
 }
 0x20c1162 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi | cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c1172; continue; }
  pc = 0x20c1170; continue;
 }
 0x20c1170 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x20c1172; continue;
 }
 0x20c1172 => {
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x20c1179; continue; }
  pc = 0x20c1177; continue;
 }
 0x20c1177 => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x20c1179; continue;
 }
 0x20c1179 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c117a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c117a); }
 let mut pc: u32 = 0x20c117a;
 loop { match pc {
 0x20c117a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x20be936u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x20c1198; continue; }
  pc = 0x20c1192; continue;
 }
 0x20c1192 => {
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c120b; continue;
 }
 0x20c1198 => {
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) & (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.rol8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x20c11cc; continue; }
  pc = 0x20c11b2; continue;
 }
 0x20c11b2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x20c11ca; continue; }
  pc = 0x20c11bb; continue;
 }
 0x20c11bb => {
  { let v: u16 = cpu.r16(0x20be930u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c11ca; continue;
 }
 0x20c11ca => {
  pc = 0x20c120b; continue;
 }
 0x20c11cc => {
  { cpu.sub16((cpu.eax as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x20c11f4; continue; }
  pc = 0x20c11d2; continue;
 }
 0x20c11d2 => {
  { let v: u16 = cpu.r16(0x20be934u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c11ec; continue; }
  pc = 0x20c11e4; continue;
 }
 0x20c11e4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  pc = 0x20c11ec; continue;
 }
 0x20c11ec => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x20c11f2; continue; }
  pc = 0x20c11f0; continue;
 }
 0x20c11f0 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c11f2; continue;
 }
 0x20c11f2 => {
  pc = 0x20c120b; continue;
 }
 0x20c11f4 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.ebx)), 0x80u8); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x20c120b; continue; }
  pc = 0x20c11fc; continue;
 }
 0x20c11fc => {
  { let v: u16 = cpu.r16(0x20be934u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c120b; continue;
 }
 0x20c120b => {
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
pub fn f_020c1222(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1222); }
 let mut pc: u32 = 0x20c1222;
 loop { match pc {
 0x20c1222 => {
  cpu.call(0x9cc1fb6d); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1240(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c1240); } r_020c1240(cpu, 0x20c1240); }
pub fn f_020c1281(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c1281); } r_020c1240(cpu, 0x20c1281); }
fn r_020c1240(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c1240 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x76bb3fffu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x76bb3fffu32.wrapping_add(cpu.eax), v); }
  if cpu.of { pc = 0x20c128c; continue; }
  pc = 0x20c124e; continue;
 }
 0x20c124e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xef60c004u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.r32(0x9c041fd0u32.wrapping_add(cpu.ecx))); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0xf4e06c83u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.ror8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 192u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_020c1268(cpu); return;
 }
 0x20c1281 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20c128c); f_020c12c4(cpu); if cpu.returned_to != 0x20c128c { return; }
  pc = 0x20c128c; continue;
 }
 0x20c128c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20c1295); f_020c0838(cpu); if cpu.returned_to != 0x20c1295 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8aa3b29u32; cpu.edx = v; }
  { let v: u32 = 0x5c17f0bdu32; cpu.eax = v; }
  cpu.push(0x20c12b5); f_020c0862(cpu); if cpu.returned_to != 0x20c12b5 { return; }
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
pub fn f_020c1268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1268); }
 let mut pc: u32 = 0x20c1268;
 loop { match pc {
 0x20c1268 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3fffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20c127e); f_020c042e(cpu); if cpu.returned_to != 0x20c127e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_020c1281(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c12c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c12c4); }
 let mut pc: u32 = 0x20c12c4;
 loop { match pc {
 0x20c12c4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x20c12d6; continue; }
  pc = 0x20c12d4; continue;
 }
 0x20c12d4 => {
  pc = 0x20c12d6; continue;
 }
 0x20c12d6 => {
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
  cpu.push(0x20c12fe); f_020c045e(cpu); if cpu.returned_to != 0x20c12fe { return; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3ffeu16); }
  if !cpu.zf { pc = 0x20c1315; continue; }
  pc = 0x20c1306; continue;
 }
 0x20c1306 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xb504f333u32); }
  if !cpu.zf { pc = 0x20c1315; continue; }
  pc = 0x20c130f; continue;
 }
 0x20c130f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0xf9de6484u32); }
  pc = 0x20c1315; continue;
 }
 0x20c1315 => {
  if (cpu.cf || cpu.zf) { pc = 0x20c1331; continue; }
  pc = 0x20c1317; continue;
 }
 0x20c1317 => {
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbffe0000u32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x20c132f); f_020c045e(cpu); if cpu.returned_to != 0x20c132f { return; }
  pc = 0x20c133d; continue;
 }
 0x20c1331 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c133d; continue;
 }
 0x20c133d => {
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
  cpu.push(0x20c1363); f_020c045e(cpu); if cpu.returned_to != 0x20c1363 { return; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x20c1375); f_020c068d(cpu); if cpu.returned_to != 0x20c1375 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20c1389); f_020c0862(cpu); if cpu.returned_to != 0x20c1389 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x20c1240u32; cpu.ecx = v; }
  cpu.push(0x20c13a2); f_020c1a60(cpu); if cpu.returned_to != 0x20c13a2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x20c1222u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  cpu.push(0x20c13be); f_020c1a60(cpu); if cpu.returned_to != 0x20c13be { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c13cf); f_020c068d(cpu); if cpu.returned_to != 0x20c13cf { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x20c13e3); f_020c0862(cpu); if cpu.returned_to != 0x20c13e3 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x20c13fa); f_020c0862(cpu); if cpu.returned_to != 0x20c13fa { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c140b); f_020c045e(cpu); if cpu.returned_to != 0x20c140b { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20c1486; continue; }
  pc = 0x20c1410; continue;
 }
 0x20c1410 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c141c); f_020c0a22(cpu); if cpu.returned_to != 0x20c141c { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbff20000u32; cpu.esi = v; }
  { let v: u32 = 0xde8082e3u32; cpu.ecx = v; }
  { let v: u32 = 0x865435cu32; cpu.ebx = v; }
  cpu.push(0x20c1440); f_020c0862(cpu); if cpu.returned_to != 0x20c1440 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c1451); f_020c045e(cpu); if cpu.returned_to != 0x20c1451 { return; }
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
  cpu.push(0x20c1475); f_020c0862(cpu); if cpu.returned_to != 0x20c1475 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c1486); f_020c045e(cpu); if cpu.returned_to != 0x20c1486 { return; }
  pc = 0x20c1486; continue;
 }
 0x20c1486 => {
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
pub fn f_020c1496(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1496); }
 let mut pc: u32 = 0x20c1496;
 loop { match pc {
 0x20c1496 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffff85u32; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x3fceca96u32); }
  cpu.call(0x20c14a0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c154a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c154a); } r_020c154a(cpu, 0x20c154a); }
pub fn f_020c155c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c155c); } r_020c154a(cpu, 0x20c155c); }
fn r_020c154a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c154a => {
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
  if (cpu.sf != cpu.of) { pc = 0x20c157e; continue; }
  pc = 0x20c157e; continue;
 }
 0x20c155c => {
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
  if (cpu.sf != cpu.of) { pc = 0x20c157e; continue; }
  pc = 0x20c157e; continue;
 }
 0x20c157e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3ffeu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc90fdaa2u32; cpu.ecx = v; }
  { let v: u32 = 0x2168c235u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20c159d); f_020c10ff(cpu); if cpu.returned_to != 0x20c159d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); }
  if cpu.zf { pc = 0x20c15cd; continue; }
  pc = 0x20c15b2; continue;
 }
 0x20c15b2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x20c15cb); f_020c042e(cpu); if cpu.returned_to != 0x20c15cb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x20c15cd; continue;
 }
 0x20c15cd => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c15f9; continue; }
  pc = 0x20c15dd; continue;
 }
 0x20c15dd => {
  { let v = 0xbffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c15f5); f_020c042e(cpu); if cpu.returned_to != 0x20c15f5 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c15f9; continue;
 }
 0x20c15f9 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xffe0u16); }
  if (cpu.sf == cpu.of) { pc = 0x20c1623; continue; }
  pc = 0x20c160a; continue;
 }
 0x20c160a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c16ba; continue;
 }
 0x20c1623 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x20c162c; continue; }
  pc = 0x20c1628; continue;
 }
 0x20c1628 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c162c; continue;
 }
 0x20c162c => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c1639; continue; }
  pc = 0x20c1635; continue;
 }
 0x20c1635 => {
  { let v: u16 = 0x8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20c1639; continue;
 }
 0x20c1639 => {
  { let v: u16 = cpu.r16(0x20c154au32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x20c1653; continue; }
  pc = 0x20c164c; continue;
 }
 0x20c164c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c1653; continue; }
  pc = 0x20c1651; continue;
 }
 0x20c1651 => {
  pc = 0x20c1691; continue;
 }
 0x20c1653 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x20c166d); f_020c0862(cpu); if cpu.returned_to != 0x20c166d { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x20c14f0u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  cpu.push(0x20c1686); f_020c1a60(cpu); if cpu.returned_to != 0x20c1686 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x20c1691; continue;
 }
 0x20c1691 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c169c; continue; }
  pc = 0x20c1695; continue;
 }
 0x20c1695 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c169c; continue; }
  pc = 0x20c169a; continue;
 }
 0x20c169a => {
  pc = 0x20c16ba; continue;
 }
 0x20c169c => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x20c1496u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  cpu.push(0x20c16ba); f_020c1ab2(cpu); if cpu.returned_to != 0x20c16ba { return; }
  pc = 0x20c16ba; continue;
 }
 0x20c16ba => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x20c16cf; continue; }
  pc = 0x20c16c3; continue;
 }
 0x20c16c3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c16d8; continue;
 }
 0x20c16cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c16d8; continue;
 }
 0x20c16d8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c16e5; continue; }
  pc = 0x20c16e0; continue;
 }
 0x20c16e0 => {
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c16e5; continue;
 }
 0x20c16e5 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x20c1742; continue; }
  pc = 0x20c16ea; continue;
 }
 0x20c16ea => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x20c1708; continue; }
  pc = 0x20c16f9; continue;
 }
 0x20c16f9 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c1714; continue;
 }
 0x20c1708 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c1714; continue;
 }
 0x20c1714 => {
  if cpu.zf { pc = 0x20c171c; continue; }
  pc = 0x20c1716; continue;
 }
 0x20c1716 => {
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  pc = 0x20c171c; continue;
 }
 0x20c171c => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x20c173d; continue; }
  pc = 0x20c1723; continue;
 }
 0x20c1723 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20c173d; continue; }
  pc = 0x20c1727; continue;
 }
 0x20c1727 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x20c173d; continue; }
  pc = 0x20c172f; continue;
 }
 0x20c172f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x20c1742; continue;
 }
 0x20c173d => {
  cpu.push(0x20c1742); f_020c068d(cpu); if cpu.returned_to != 0x20c1742 { return; }
  pc = 0x20c1742; continue;
 }
 0x20c1742 => {
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
pub fn f_020c1753(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1753); }
 let mut pc: u32 = 0x20c1753;
 loop { match pc {
 0x20c1753 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x20c175b); f_020c155c(cpu); if cpu.returned_to != 0x20c175b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c175d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c175d); }
 let mut pc: u32 = 0x20c175d;
 loop { match pc {
 0x20c175d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  cpu.push(0x20c1768); f_020c155c(cpu); if cpu.returned_to != 0x20c1768 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c176a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c176a); }
 let mut pc: u32 = 0x20c176a;
 loop { match pc {
 0x20c176a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x20c1775); f_020c155c(cpu); if cpu.returned_to != 0x20c1775 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1778); }
 let mut pc: u32 = 0x20c1778;
 loop { match pc {
 0x20c1778 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x67cc487bu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c17a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c17a0); }
 let mut pc: u32 = 0x20c17a0;
 loop { match pc {
 0x20c17a0 => {
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
  if !cpu.zf { pc = 0x20c17c2; continue; }
  pc = 0x20c17b9; continue;
 }
 0x20c17b9 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x20c17c2; continue; }
  pc = 0x20c17bf; continue;
 }
 0x20c17bf => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  pc = 0x20c17c2; continue;
 }
 0x20c17c2 => {
  if cpu.zf { pc = 0x20c182a; continue; }
  pc = 0x20c17c4; continue;
 }
 0x20c17c4 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x20c17e3; continue; }
  pc = 0x20c17c9; continue;
 }
 0x20c17c9 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c17df); f_020c042e(cpu); if cpu.returned_to != 0x20c17df { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c17e3; continue;
 }
 0x20c17e3 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x20c17f2; continue;
 }
 0x20c17f2 => {
  { cpu.sub16((cpu.ebx as u16), 0x3fffu16); }
  if cpu.cf { pc = 0x20c1803; continue; }
  pc = 0x20c17f9; continue;
 }
 0x20c17f9 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20c17f2; continue;
 }
 0x20c1803 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c1809; continue; }
  pc = 0x20c1807; continue;
 }
 0x20c1807 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c1809; continue;
 }
 0x20c1809 => {
  if !cpu.zf { pc = 0x20c180f; continue; }
  pc = 0x20c180b; continue;
 }
 0x20c180b => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x20c181f; continue;
 }
 0x20c180f => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x20c181b; continue; }
  pc = 0x20c1813; continue;
 }
 0x20c1813 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20c180f; continue;
 }
 0x20c181b => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x20c181f; continue;
 }
 0x20c181f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20c182c; continue;
 }
 0x20c182a => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x20c182c; continue;
 }
 0x20c182c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x20c1847); f_020c0862(cpu); if cpu.returned_to != 0x20c1847 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xa6829a79u32; cpu.ecx = v; }
  { let v: u32 = 0x1bf21f8cu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40030000u32; cpu.esi = v; }
  cpu.push(0x20c1865); f_020c045e(cpu); if cpu.returned_to != 0x20c1865 { return; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = 0xec96f0d6u32; cpu.ecx = v; }
  { let v: u32 = 0x9d7bfdbu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffa0000u32; cpu.esi = v; }
  cpu.push(0x20c188b); f_020c0862(cpu); if cpu.returned_to != 0x20c188b { return; }
  { let v: u32 = 0xe6d5051au32; cpu.ecx = v; }
  { let v: u32 = 0xe536e187u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40010000u32; cpu.esi = v; }
  cpu.push(0x20c18a6); f_020c045e(cpu); if cpu.returned_to != 0x20c18a6 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x20c18ba); f_020c0862(cpu); if cpu.returned_to != 0x20c18ba { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x20c18d8); f_020c045e(cpu); if cpu.returned_to != 0x20c18d8 { return; }
  { let a = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x20c18f6); f_020c045e(cpu); if cpu.returned_to != 0x20c18f6 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c1908); f_020c068d(cpu); if cpu.returned_to != 0x20c1908 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x20c1778u32; cpu.esi = v; }
  pc = 0x20c191b; continue;
 }
 0x20c191b => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20c1953; continue; }
  pc = 0x20c191f; continue;
 }
 0x20c191f => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x20c194e; continue; }
  pc = 0x20c1923; continue;
 }
 0x20c1923 => {
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
  cpu.push(0x20c1943); f_020c0862(cpu); if cpu.returned_to != 0x20c1943 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x20c194e; continue;
 }
 0x20c194e => {
  { let v: u32 = 0xau32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20c191b; continue;
 }
 0x20c1953 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.add16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.esi as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c196d); f_020c042e(cpu); if cpu.returned_to != 0x20c196d { return; }
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
pub fn f_020c1974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1974); }
 let mut pc: u32 = 0x20c1974;
 loop { match pc {
 0x20c1974 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x20c19ba; continue; }
  pc = 0x20c1986; continue;
 }
 0x20c1986 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x20c19af; continue; }
  pc = 0x20c198a; continue;
 }
 0x20c198a => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c1a3a; continue; }
  pc = 0x20c1993; continue;
 }
 0x20c1993 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c19a6; continue;
 }
 0x20c19a6 => {
  { let v: u8 = cpu.or8(cpu.r8(0x7u32.wrapping_add(cpu.eax)), 0xc0u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c1a3a; continue;
 }
 0x20c19af => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x20c1a3a; continue; }
  pc = 0x20c19ba; continue;
 }
 0x20c19ba => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x7fffu16); }
  if cpu.zf { cpu.poll(); pc = 0x20c19a6; continue; }
  pc = 0x20c19c6; continue;
 }
 0x20c19c6 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { cpu.poll(); pc = 0x20c19a6; continue; }
  pc = 0x20c19cb; continue;
 }
 0x20c19cb => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20c19d7; continue; }
  pc = 0x20c19d0; continue;
 }
 0x20c19d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x20c1993; continue; }
  pc = 0x20c19d7; continue;
 }
 0x20c19d7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x20c19ed; continue; }
  pc = 0x20c19e7; continue;
 }
 0x20c19e7 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x20c19ed; continue;
 }
 0x20c19ed => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x20c1a20; continue; }
  pc = 0x20c1a02; continue;
 }
 0x20c1a02 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x20c1a03; continue;
 }
 0x20c1a03 => {
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x20c1a15; continue; }
  pc = 0x20c1a0a; continue;
 }
 0x20c1a0a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x20c1a03; continue;
 }
 0x20c1a15 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x20c1a29; continue;
 }
 0x20c1a20 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x20c1a32; continue; }
  pc = 0x20c1a24; continue;
 }
 0x20c1a24 => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  pc = 0x20c1a29; continue;
 }
 0x20c1a29 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.edx); cpu.eax = v; }
  pc = 0x20c1a32; continue;
 }
 0x20c1a32 => {
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x20c1a3a; continue;
 }
 0x20c1a3a => {
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
pub fn f_020c1a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1a40); }
 let mut pc: u32 = 0x20c1a40;
 loop { match pc {
 0x20c1a40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20c1a53); f_020c1a60(cpu); if cpu.returned_to != 0x20c1a53 { return; }
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
pub fn f_020c1a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1a60); }
 let mut pc: u32 = 0x20c1a60;
 loop { match pc {
 0x20c1a60 => {
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
  pc = 0x20c1a77; continue;
 }
 0x20c1a77 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x20c1a8c); f_020c0862(cpu); if cpu.returned_to != 0x20c1a8c { return; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x20c1aa6); f_020c045e(cpu); if cpu.returned_to != 0x20c1aa6 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x20c1a77; continue; }
  pc = 0x20c1aab; continue;
 }
 0x20c1aab => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1ab2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1ab2); }
 let mut pc: u32 = 0x20c1ab2;
 loop { match pc {
 0x20c1ab2 => {
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
  cpu.push(0x20c1ad2); f_020c0862(cpu); if cpu.returned_to != 0x20c1ad2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x20c1ad9); f_020c1a60(cpu); if cpu.returned_to != 0x20c1ad9 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x20c1aed); f_020c0862(cpu); if cpu.returned_to != 0x20c1aed { return; }
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
pub fn f_020c1afa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1afa); }
 let mut pc: u32 = 0x20c1afa;
 loop { match pc {
 0x20c1afa => {
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
  if cpu.zf { pc = 0x20c1b6a; continue; }
  pc = 0x20c1b0d; continue;
 }
 0x20c1b0d => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x20c1b6a; continue; }
  pc = 0x20c1b19; continue;
 }
 0x20c1b19 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, 0x666u32); }
  if !cpu.zf { pc = 0x20c1b6a; continue; }
  pc = 0x20c1b2d; continue;
 }
 0x20c1b2d => {
  { let v: u8 = 0x22u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x20c1b6a; continue; }
  pc = 0x20c1b3e; continue;
 }
 0x20c1b3e => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x217c8e5u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u32 = 0x22d22d4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c1bc4; continue;
 }
 0x20c1b6a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20c1b76; continue; }
  pc = 0x20c1b71; continue;
 }
 0x20c1b71 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x20c1bc2; continue; }
  pc = 0x20c1b76; continue;
 }
 0x20c1b76 => {
  { let v: u32 = 0xa00u32; cpu.eax = v; }
  { let v: u32 = 0x216b31cu32; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c1bc2; continue; }
  pc = 0x20c1b8a; continue;
 }
 0x20c1b8a => {
  { let v: u32 = 0x20becbcu32; cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  cpu.push(0x20c1b9b); f_020aeebe(cpu); if cpu.returned_to != 0x20c1b9b { return; }
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
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x217c8e4u32, v); }
  pc = 0x20c1bc2; continue;
 }
 0x20c1bc2 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c1bc4; continue;
 }
 0x20c1bc4 => {
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
pub fn f_020c1bce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1bce); }
 let mut pc: u32 = 0x20c1bce;
 loop { match pc {
 0x20c1bce => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x217c8e5u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20c1bfd; continue; }
  pc = 0x20c1bdd; continue;
 }
 0x20c1bdd => {
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
 0x20c1bfd => {
  { cpu.sub8(cpu.r8(0x217c8e4u32), 0x0u8); }
  if cpu.zf { pc = 0x20c1c21; continue; }
  pc = 0x20c1c06; continue;
 }
 0x20c1c06 => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3eu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = (cpu.ebx as u8) & 0xfbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x20c1c21; continue;
 }
 0x20c1c21 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1c27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1c27); }
 let mut pc: u32 = 0x20c1c27;
 loop { match pc {
 0x20c1c27 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1c28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1c28); }
 let mut pc: u32 = 0x20c1c28;
 loop { match pc {
 0x20c1c28 => {
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
  if cpu.zf { pc = 0x20c1c42; continue; }
  pc = 0x20c1c40; continue;
 }
 0x20c1c40 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c1c42; continue;
 }
 0x20c1c42 => {
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
pub fn f_020c1c4f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c1c4f); } r_020c1c4f(cpu, 0x20c1c4f); }
pub fn f_020c1c56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c1c56); } r_020c1c4f(cpu, 0x20c1c56); }
fn r_020c1c4f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c1c4f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20c1c56; continue; }
  pc = 0x20c1c53; continue;
 }
 0x20c1c53 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c1c55; continue;
 }
 0x20c1c55 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c1c56 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x20c1c55; continue; }
  f_020c1c5a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1c5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1c5a); }
 let mut pc: u32 = 0x20c1c5a;
 loop { match pc {
 0x20c1c5a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20c1c69); f_020af0fe(cpu); if cpu.returned_to != 0x20c1c69 { return; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if !cpu.cf { pc = 0x20c1cb4; continue; }
  pc = 0x20c1c71; continue;
 }
 0x20c1c71 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b947u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c1c9a; continue; }
  pc = 0x20c1c7d; continue;
 }
 0x20c1c7d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if !cpu.zf { pc = 0x20c1c8a; continue; }
  pc = 0x20c1c86; continue;
 }
 0x20c1c86 => {
  { let v: u8 = 0xeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c1c9a; continue;
 }
 0x20c1c8a => {
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c1c93; continue; }
  pc = 0x20c1c8f; continue;
 }
 0x20c1c8f => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c1c9a; continue;
 }
 0x20c1c93 => {
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c1c9a; continue; }
  pc = 0x20c1c98; continue;
 }
 0x20c1c98 => {
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c1c9a; continue;
 }
 0x20c1c9a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x13u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c1ca5; continue; }
  pc = 0x20c1ca3; continue;
 }
 0x20c1ca3 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c1ca5; continue;
 }
 0x20c1ca5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x217c8f1u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x20c1cbe; continue;
 }
 0x20c1cb4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x20c1cbe; continue;
 }
 0x20c1cbe => {
  cpu.push(0x20c1cc3); f_020af0db(cpu); if cpu.returned_to != 0x20c1cc3 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1ccb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1ccb); }
 let mut pc: u32 = 0x20c1ccb;
 loop { match pc {
 0x20c1ccb => {
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
pub fn f_020c1cf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1cf2); }
 let mut pc: u32 = 0x20c1cf2;
 loop { match pc {
 0x20c1cf2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x217c90cu32)); }
  if cpu.cf { pc = 0x20c1d05; continue; }
  pc = 0x20c1cff; continue;
 }
 0x20c1cff => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c1d05 => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c1d3b; continue; }
  pc = 0x20c1d0a; continue;
 }
 0x20c1d0a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217c960u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x20c1d3b; continue; }
  pc = 0x20c1d1e; continue;
 }
 0x20c1d1e => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c1d2d); f_020c1ccb(cpu); if cpu.returned_to != 0x20c1d2d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c1d3b; continue; }
  pc = 0x20c1d31; continue;
 }
 0x20c1d31 => {
  { let v: u32 = cpu.r32(0x217c960u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x20c1d3b; continue;
 }
 0x20c1d3b => {
  { let v: u32 = cpu.r32(0x217c960u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1d47); }
 let mut pc: u32 = 0x20c1d47;
 loop { match pc {
 0x20c1d47 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217c960u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1d5c); }
 let mut pc: u32 = 0x20c1d5c;
 loop { match pc {
 0x20c1d5c => {
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
  if (cpu.sf != cpu.of) { pc = 0x20c1d7e; continue; }
  pc = 0x20c1d6f; continue;
 }
 0x20c1d6f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20c1d78); f_020c1d47(cpu); if cpu.returned_to != 0x20c1d78 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c1d7e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x20c1d88); f_020af0db(cpu); if cpu.returned_to != 0x20c1d88 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1d91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1d91); }
 let mut pc: u32 = 0x20c1d91;
 loop { match pc {
 0x20c1d91 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x217b920u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c1dbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1dbb); }
 let mut pc: u32 = 0x20c1dbb;
 loop { match pc {
 0x20c1dbb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x20c1de0; continue; }
  pc = 0x20c1dc8; continue;
 }
 0x20c1dc8 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x20c1dd2); f_020af0db(cpu); if cpu.returned_to != 0x20c1dd2 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1dd6; continue;
 }
 0x20c1dd6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20c1e59; continue;
 }
 0x20c1de0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c1ded; continue; }
  pc = 0x20c1de6; continue;
 }
 0x20c1de6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c1ded); f_020c1e5f(cpu); if cpu.returned_to != 0x20c1ded { return; }
  pc = 0x20c1ded; continue;
 }
 0x20c1ded => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x20c1e2a; continue; }
  pc = 0x20c1df7; continue;
 }
 0x20c1df7 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x20c1e2a; continue; }
  pc = 0x20c1e03; continue;
 }
 0x20c1e03 => {
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
  if !cpu.zf { pc = 0x20c1e2a; continue; }
  pc = 0x20c1e1f; continue;
 }
 0x20c1e1f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c1e26); f_020c1ed6(cpu); if cpu.returned_to != 0x20c1e26 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20c1dd6; continue; }
  pc = 0x20c1e2a; continue;
 }
 0x20c1e2a => {
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
  if !cpu.zf { pc = 0x20c1e4a; continue; }
  pc = 0x20c1e45; continue;
 }
 0x20c1e45 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x20c1e55; continue; }
  pc = 0x20c1e4a; continue;
 }
 0x20c1e4a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c1e51); f_020c1ed6(cpu); if cpu.returned_to != 0x20c1e51 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20c1dd6; continue; }
  pc = 0x20c1e55; continue;
 }
 0x20c1e55 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c1e59; continue;
 }
 0x20c1e59 => {
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
pub fn f_020c1e5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1e5f); }
 let mut pc: u32 = 0x20c1e5f;
 loop { match pc {
 0x20c1e5f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x20c1e68); f_020c5648(cpu); if cpu.returned_to != 0x20c1e68 { return; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c1e94; continue; }
  pc = 0x20c1e6e; continue;
 }
 0x20c1e6e => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x20c1e7f; continue; }
  pc = 0x20c1e76; continue;
 }
 0x20c1e76 => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1e94; continue;
 }
 0x20c1e7f => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c1e8d; continue; }
  pc = 0x20c1e84; continue;
 }
 0x20c1e84 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1e94; continue;
 }
 0x20c1e8d => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1e94; continue;
 }
 0x20c1e94 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x20c1e9c); f_020bd474(cpu); if cpu.returned_to != 0x20c1e9c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c1ec3; continue; }
  pc = 0x20c1ea3; continue;
 }
 0x20c1ea3 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1ec7; continue;
 }
 0x20c1ec3 => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x20c1ec7; continue;
 }
 0x20c1ec7 => {
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
pub fn f_020c1ed6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1ed6); }
 let mut pc: u32 = 0x20c1ed6;
 loop { match pc {
 0x20c1ed6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x20c1f62; continue; }
  pc = 0x20c1eec; continue;
 }
 0x20c1eec => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f00; continue;
 }
 0x20c1f00 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f0b; continue;
 }
 0x20c1f0b => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  pc = 0x20c1f10; continue;
 }
 0x20c1f10 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f18; continue;
 }
 0x20c1f18 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f20; continue;
 }
 0x20c1f20 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c1f2c); f_020c5679(cpu); if cpu.returned_to != 0x20c1f2c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20c1f40; continue; }
  pc = 0x20c1f33; continue;
 }
 0x20c1f33 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c1f5c; continue;
 }
 0x20c1f40 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c1f5c; continue; }
  pc = 0x20c1f44; continue;
 }
 0x20c1f44 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x20c1f4e); f_020af0db(cpu); if cpu.returned_to != 0x20c1f4e { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c1f5c; continue;
 }
 0x20c1f5c => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x20c1f10; continue;
 }
 0x20c1f62 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f68; continue;
 }
 0x20c1f68 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x20u8); }
  if !cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f72; continue;
 }
 0x20c1f72 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c1f8a; continue; }
  pc = 0x20c1f79; continue;
 }
 0x20c1f79 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c1f8a); f_020b1ab3(cpu); if cpu.returned_to != 0x20c1f8a { return; }
  pc = 0x20c1f8a; continue;
 }
 0x20c1f8a => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20c1f9a; continue; }
  pc = 0x20c1f8f; continue;
 }
 0x20c1f8f => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c1f9a; continue;
 }
 0x20c1f9a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
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
pub fn o_020c1fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c1fb0); }
 let mut pc: u32 = 0x20c1fb0;
 loop { match pc {
 0x20c1fb0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x5u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x217c964u32), 0xffffu32); }
  if !cpu.zf { pc = 0x20c1fee; continue; }
  pc = 0x20c1fc3; continue;
 }
 0x20c1fc3 => {
  cpu.push(0x20c1fc8); f_020c5ea0(cpu); if cpu.returned_to != 0x20c1fc8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x217c968u32; cpu.edx = v; }
  if cpu.zf { pc = 0x20c2034; continue; }
  pc = 0x20c1fd1; continue;
 }
 0x20c1fd1 => {
  { let v = 0x20b1c74u32; cpu.push(v); }
  cpu.push(0x20c1fdb); f_020c625c(cpu); if cpu.returned_to != 0x20c1fdb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217c964u32, v); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { let v: u32 = 0x217c997u32; cpu.edx = v; }
  if cpu.zf { pc = 0x20c2034; continue; }
  pc = 0x20c1fee; continue;
 }
 0x20c1fee => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x20c8ef8u32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x20c918cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x217c964u32); cpu.push(v); }
  cpu.push(0x20c2014); f_020c6464(cpu); if cpu.returned_to != 0x20c2014 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); }
  { let v: u32 = 0x217c9b9u32; cpu.edx = v; }
  if !cpu.zf { pc = 0x20c2034; continue; }
  pc = 0x20c2026; continue;
 }
 0x20c2026 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u8 = 0xe9u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.poll(); pc = cpu.eax; continue;
 }
 0x20c2034 => {
  cpu.unimplemented(0x20c2034, "Int3");
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20c203b); f_020c6218(cpu); if cpu.returned_to != 0x20c203b { return; }
  cpu.unimplemented(0x20c203b, "Hlt");
  f_020c203c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c203c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c203c); }
 let mut pc: u32 = 0x20c203c;
 loop { match pc {
 0x20c203c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c205a; continue; }
  pc = 0x20c2047; continue;
 }
 0x20c2047 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x20c205a; continue;
 }
 0x20c205a => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c2071; continue; }
  pc = 0x20c205e; continue;
 }
 0x20c205e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2071 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c207a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c207a); }
 let mut pc: u32 = 0x20c207a;
 loop { match pc {
 0x20c207a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x20c2090; continue; }
  pc = 0x20c2084; continue;
 }
 0x20c2084 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2090 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2094(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2094); }
 let mut pc: u32 = 0x20c2094;
 loop { match pc {
 0x20c2094 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x20c20b4; continue; }
  pc = 0x20c209a; continue;
 }
 0x20c209a => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x22d2354u32, v); }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22d2355u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x22d2356u32, v); }
  pc = 0x20c20c1; continue;
 }
 0x20c20b4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22d2354u32, v); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x22d2355u32, v); }
  pc = 0x20c20c1; continue;
 }
 0x20c20c1 => {
  { let v: u32 = 0x22d2354u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c20c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c20c8); }
 let mut pc: u32 = 0x20c20c8;
 loop { match pc {
 0x20c20c8 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.push(0x20c20cf); f_020c20f6(cpu); if cpu.returned_to != 0x20c20cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c20d8; continue; }
  pc = 0x20c20d3; continue;
 }
 0x20c20d3 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x20c20f3; continue; }
  pc = 0x20c20d8; continue;
 }
 0x20c20d8 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if !cpu.zf { pc = 0x20c20e9; continue; }
  pc = 0x20c20e1; continue;
 }
 0x20c20e1 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if cpu.zf { pc = 0x20c20f3; continue; }
  pc = 0x20c20e9; continue;
 }
 0x20c20e9 => {
  { let v: u32 = 0x1u32; cpu.w32(0x217c9ecu32, v); }
  pc = 0x20c20f3; continue;
 }
 0x20c20f3 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c20f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c20f6); }
 let mut pc: u32 = 0x20c20f6;
 loop { match pc {
 0x20c20f6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c219c; continue; }
  pc = 0x20c210c; continue;
 }
 0x20c210c => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c219c; continue; }
  pc = 0x20c2115; continue;
 }
 0x20c2115 => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20c2125); f_020aedc0(cpu); if cpu.returned_to != 0x20c2125 { return; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x2511u32; cpu.ecx = v; }
  cpu.push(0x20c213a); f_020aedc0(cpu); if cpu.returned_to != 0x20c213a { return; }
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
  cpu.push(0x20c216e); f_020c65d4(cpu); if cpu.returned_to != 0x20c216e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x20c21f8; continue; }
  pc = 0x20c2180; continue;
 }
 0x20c2180 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x20c2191; continue;
 }
 0x20c2191 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2480u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x20c21fc; continue;
 }
 0x20c219c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c21f8; continue; }
  pc = 0x20c21a8; continue;
 }
 0x20c21a8 => {
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x20c21b8); f_020aedc0(cpu); if cpu.returned_to != 0x20c21b8 { return; }
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
  if !cpu.zf { pc = 0x20c21f8; continue; }
  pc = 0x20c21e4; continue;
 }
 0x20c21e4 => {
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2191; continue;
 }
 0x20c21f8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c21fc; continue;
 }
 0x20c21fc => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2203); }
 let mut pc: u32 = 0x20c2203;
 loop { match pc {
 0x20c2203 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c2261; continue; }
  pc = 0x20c2213; continue;
 }
 0x20c2213 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c226c; continue; }
  pc = 0x20c221d; continue;
 }
 0x20c221d => {
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c223e; continue; }
  pc = 0x20c2229; continue;
 }
 0x20c2229 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  pc = 0x20c2240; continue;
 }
 0x20c223e => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x20c2240; continue;
 }
 0x20c2240 => {
  if cpu.zf { pc = 0x20c226c; continue; }
  pc = 0x20c2242; continue;
 }
 0x20c2242 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x20c225a; continue; }
  pc = 0x20c2248; continue;
 }
 0x20c2248 => {
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
 0x20c225a => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2261 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20c226c; continue;
 }
 0x20c226c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2270); }
 let mut pc: u32 = 0x20c2270;
 loop { match pc {
 0x20c2270 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c22bb; continue; }
  pc = 0x20c227c; continue;
 }
 0x20c227c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c22bd; continue; }
  pc = 0x20c2284; continue;
 }
 0x20c2284 => {
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c22a2; continue; }
  pc = 0x20c228f; continue;
 }
 0x20c228f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  pc = 0x20c22a2; continue;
 }
 0x20c22a2 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c22bd; continue; }
  pc = 0x20c22a6; continue;
 }
 0x20c22a6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20c22bd; continue; }
  pc = 0x20c22ab; continue;
 }
 0x20c22ab => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c22bb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c22bd; continue;
 }
 0x20c22bd => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2308); }
 let mut pc: u32 = 0x20c2308;
 loop { match pc {
 0x20c2308 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.cf { pc = 0x20c2320; continue; }
  pc = 0x20c2319; continue;
 }
 0x20c2319 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x20c2382; continue;
 }
 0x20c2320 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c2382; continue; }
  pc = 0x20c2325; continue;
 }
 0x20c2325 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  if cpu.zf { pc = 0x20c2342; continue; }
  pc = 0x20c2329; continue;
 }
 0x20c2329 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x20c2344; continue;
 }
 0x20c2342 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x20c2344; continue;
 }
 0x20c2344 => {
  if cpu.zf { pc = 0x20c234b; continue; }
  pc = 0x20c2346; continue;
 }
 0x20c2346 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x20c2382; continue;
 }
 0x20c234b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c236d; continue; }
  pc = 0x20c2350; continue;
 }
 0x20c2350 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c2369; continue; }
  pc = 0x20c2354; continue;
 }
 0x20c2354 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x20c236b; continue;
 }
 0x20c2369 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x20c236b; continue;
 }
 0x20c236b => {
  if !cpu.zf { cpu.poll(); pc = 0x20c234b; continue; }
  pc = 0x20c236d; continue;
 }
 0x20c236d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20c2382; continue;
 }
 0x20c2382 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217c9ecu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c238e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c238e); }
 let mut pc: u32 = 0x20c238e;
 loop { match pc {
 0x20c238e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217420du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c23df; continue; }
  pc = 0x20c23bc; continue;
 }
 0x20c23bc => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x216b32cu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20c23df; continue; }
  pc = 0x20c23d8; continue;
 }
 0x20c23d8 => {
  { let v: u8 = cpu.r8(0x217ba10u32) | 0x1u8; cpu.w8(0x217ba10u32, v); }
  pc = 0x20c23df; continue;
 }
 0x20c23df => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c23e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c23e3); }
 let mut pc: u32 = 0x20c23e3;
 loop { match pc {
 0x20c23e3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x20c23eb); f_020c23ed(cpu); if cpu.returned_to != 0x20c23eb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c23ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c23ed); }
 let mut pc: u32 = 0x20c23ed;
 loop { match pc {
 0x20c23ed => {
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
  if cpu.zf { pc = 0x20c2419; continue; }
  pc = 0x20c2412; continue;
 }
 0x20c2412 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c2419 => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x20c2425; continue; }
  pc = 0x20c241e; continue;
 }
 0x20c241e => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c2425 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c2431; continue; }
  pc = 0x20c242a; continue;
 }
 0x20c242a => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c2431 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x20c243d; continue; }
  pc = 0x20c2436; continue;
 }
 0x20c2436 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c243d => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x20c2449; continue; }
  pc = 0x20c2442; continue;
 }
 0x20c2442 => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c2449 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c2453; continue; }
  pc = 0x20c244e; continue;
 }
 0x20c244e => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x20c2453; continue;
 }
 0x20c2453 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217caf0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x20c2477; continue; }
  pc = 0x20c246a; continue;
 }
 0x20c246a => {
  { let v = cpu.rf64(0x216b6a4u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20c24b9; continue;
 }
 0x20c2477 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x20c2486; continue; }
  pc = 0x20c247c; continue;
 }
 0x20c247c => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c24b9; continue;
 }
 0x20c2486 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x20c249a; continue; }
  pc = 0x20c248b; continue;
 }
 0x20c248b => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c24b9; continue;
 }
 0x20c249a => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x20c24ae; continue; }
  pc = 0x20c249f; continue;
 }
 0x20c249f => {
  { let v: u32 = cpu.r32(0x216b6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x216b6a8u32); cpu.eax = v; }
  pc = 0x20c24b6; continue;
 }
 0x20c24ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x20c24b6; continue;
 }
 0x20c24b6 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c24b9; continue;
 }
 0x20c24b9 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x20c24c1); f_020c6743(cpu); if cpu.returned_to != 0x20c24c1 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c24c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c24c8); }
 let mut pc: u32 = 0x20c24c8;
 loop { match pc {
 0x20c24c8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.cf { pc = 0x20c250a; continue; }
  pc = 0x20c24d9; continue;
 }
 0x20c24d9 => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x20c24e5; continue; }
  pc = 0x20c24de; continue;
 }
 0x20c24de => {
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c24f6; continue; }
  pc = 0x20c24e3; continue;
 }
 0x20c24e3 => {
  pc = 0x20c250a; continue;
 }
 0x20c24e5 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c252e; continue;
 }
 0x20c24f6 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20c250a; continue; }
  pc = 0x20c2500; continue;
 }
 0x20c2500 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c2536; continue;
 }
 0x20c250a => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x81u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.edx, 0x6u32); }
  if !cpu.zf { pc = 0x20c2529; continue; }
  pc = 0x20c251a; continue;
 }
 0x20c251a => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20c2529; continue; }
  pc = 0x20c2524; continue;
 }
 0x20c2524 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x11u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x20c2529; continue;
 }
 0x20c2529 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20c252e; continue;
 }
 0x20c252e => {
  cpu.push(0x20c2533); f_020c23e3(cpu); if cpu.returned_to != 0x20c2533 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20c2536; continue;
 }
 0x20c2536 => {
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
pub fn f_020c2544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2544); }
 let mut pc: u32 = 0x20c2544;
 loop { match pc {
 0x20c2544 => {
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_sub(0xcu8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x29b3020cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.call(0x20c25a4); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2744); }
 let mut pc: u32 = 0x20c2744;
 loop { match pc {
 0x20c2744 => {
  { let v: u8 = cpu.r8(0x35a3020cu32) ^ 0xcu8; cpu.w8(0x35a3020cu32, v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x20cu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cf = true; }
  { let v: u32 = cpu.eax ^ 0x362b020cu32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (cpu.cf || cpu.zf) { pc = 0x20c27a8; continue; }
  pc = 0x20c2772; continue;
 }
 0x20c2772 => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.stos(1, false); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) / (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.set_st(0, v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.unimplemented(0x20c27a1, "Aaa");
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x20c27a4, "Aaa");
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c27a8; continue;
 }
 0x20c27a8 => {
  { let a = cpu.r8(0x0u32.wrapping_add(cpu.edi)); let b = ((cpu.edx >> 8) as u8); { let v: u8 = b; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.eax as u8), 0x37u8); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  cpu.unimplemented(0x20c27c5, "Aaa");
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[0])), (cpu.ecx as u8)); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.ecx as u8)); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let a = cpu.ebp; let b = cpu.eax; { let v: u32 = b; cpu.ebp = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), (cpu.ecx as u8)); }
  cpu.unimplemented(0x20c27e4, "Aad");
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), cpu.ecx); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8((cpu.eax as u8), 0x39u8); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c283b; continue; }
  pc = 0x20c2802; continue;
 }
 0x20c2802 => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sar8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 12u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))), cpu.ecx); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.edi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  if !cpu.pf() { pc = 0x20c2860; continue; }
  pc = 0x20c2826; continue;
 }
 0x20c2826 => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x3au8); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.call(0x20c2840); return;
 }
 0x20c283b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)))); }
  cpu.unimplemented(0x20c2848, "Bound");
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3bu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.fstatus(); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c2860; continue;
 }
 0x20c2860 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0xcu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1)))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0xd9020c3cu32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xcu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x60020c3du32).wrapping_add((cpu.ebx as u8)); cpu.w8(0x60020c3du32, v); }
  { cpu.sub32(cpu.eax, 0x20cu32); }
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
  { cpu.movs(1, false); }
  { cpu.sub32(cpu.eax, 0x3dc9020cu32); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x20c28ac); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
