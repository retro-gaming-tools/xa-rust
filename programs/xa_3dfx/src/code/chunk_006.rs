#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010bef5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef5b); }
 let mut pc: u32 = 0x10bef5b;
 loop { match pc {
 0x10bef5b => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef5e); }
 let mut pc: u32 = 0x10bef5e;
 loop { match pc {
 0x10bef5e => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef61); }
 let mut pc: u32 = 0x10bef61;
 loop { match pc {
 0x10bef61 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef64); }
 let mut pc: u32 = 0x10bef64;
 loop { match pc {
 0x10bef64 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef67); }
 let mut pc: u32 = 0x10bef67;
 loop { match pc {
 0x10bef67 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef6a); }
 let mut pc: u32 = 0x10bef6a;
 loop { match pc {
 0x10bef6a => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef6d); }
 let mut pc: u32 = 0x10bef6d;
 loop { match pc {
 0x10bef6d => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef70); }
 let mut pc: u32 = 0x10bef70;
 loop { match pc {
 0x10bef70 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef73); }
 let mut pc: u32 = 0x10bef73;
 loop { match pc {
 0x10bef73 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef76); }
 let mut pc: u32 = 0x10bef76;
 loop { match pc {
 0x10bef76 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef79); }
 let mut pc: u32 = 0x10bef79;
 loop { match pc {
 0x10bef79 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef7c); }
 let mut pc: u32 = 0x10bef7c;
 loop { match pc {
 0x10bef7c => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef7f); }
 let mut pc: u32 = 0x10bef7f;
 loop { match pc {
 0x10bef7f => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef82); }
 let mut pc: u32 = 0x10bef82;
 loop { match pc {
 0x10bef82 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef85); }
 let mut pc: u32 = 0x10bef85;
 loop { match pc {
 0x10bef85 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef88); }
 let mut pc: u32 = 0x10bef88;
 loop { match pc {
 0x10bef88 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef8b); }
 let mut pc: u32 = 0x10bef8b;
 loop { match pc {
 0x10bef8b => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef8e); }
 let mut pc: u32 = 0x10bef8e;
 loop { match pc {
 0x10bef8e => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef91); }
 let mut pc: u32 = 0x10bef91;
 loop { match pc {
 0x10bef91 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef94); }
 let mut pc: u32 = 0x10bef94;
 loop { match pc {
 0x10bef94 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef97); }
 let mut pc: u32 = 0x10bef97;
 loop { match pc {
 0x10bef97 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef9a); }
 let mut pc: u32 = 0x10bef9a;
 loop { match pc {
 0x10bef9a => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef9d); }
 let mut pc: u32 = 0x10bef9d;
 loop { match pc {
 0x10bef9d => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befa0); }
 let mut pc: u32 = 0x10befa0;
 loop { match pc {
 0x10befa0 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befa3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befa3); }
 let mut pc: u32 = 0x10befa3;
 loop { match pc {
 0x10befa3 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befa6); }
 let mut pc: u32 = 0x10befa6;
 loop { match pc {
 0x10befa6 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befa9); }
 let mut pc: u32 = 0x10befa9;
 loop { match pc {
 0x10befa9 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befac); }
 let mut pc: u32 = 0x10befac;
 loop { match pc {
 0x10befac => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befaf); }
 let mut pc: u32 = 0x10befaf;
 loop { match pc {
 0x10befaf => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befb2); }
 let mut pc: u32 = 0x10befb2;
 loop { match pc {
 0x10befb2 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befb5); }
 let mut pc: u32 = 0x10befb5;
 loop { match pc {
 0x10befb5 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befb8); }
 let mut pc: u32 = 0x10befb8;
 loop { match pc {
 0x10befb8 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befbb); }
 let mut pc: u32 = 0x10befbb;
 loop { match pc {
 0x10befbb => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befbe); }
 let mut pc: u32 = 0x10befbe;
 loop { match pc {
 0x10befbe => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befc1); }
 let mut pc: u32 = 0x10befc1;
 loop { match pc {
 0x10befc1 => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befc4); }
 let mut pc: u32 = 0x10befc4;
 loop { match pc {
 0x10befc4 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befc7); }
 let mut pc: u32 = 0x10befc7;
 loop { match pc {
 0x10befc7 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befca); }
 let mut pc: u32 = 0x10befca;
 loop { match pc {
 0x10befca => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befcd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befcd); }
 let mut pc: u32 = 0x10befcd;
 loop { match pc {
 0x10befcd => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befd0); }
 let mut pc: u32 = 0x10befd0;
 loop { match pc {
 0x10befd0 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befd3); }
 let mut pc: u32 = 0x10befd3;
 loop { match pc {
 0x10befd3 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befd6); }
 let mut pc: u32 = 0x10befd6;
 loop { match pc {
 0x10befd6 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befd9); }
 let mut pc: u32 = 0x10befd9;
 loop { match pc {
 0x10befd9 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befdc); }
 let mut pc: u32 = 0x10befdc;
 loop { match pc {
 0x10befdc => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befdf); }
 let mut pc: u32 = 0x10befdf;
 loop { match pc {
 0x10befdf => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befe2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befe2); }
 let mut pc: u32 = 0x10befe2;
 loop { match pc {
 0x10befe2 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befe5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befe5); }
 let mut pc: u32 = 0x10befe5;
 loop { match pc {
 0x10befe5 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befe8); }
 let mut pc: u32 = 0x10befe8;
 loop { match pc {
 0x10befe8 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befeb); }
 let mut pc: u32 = 0x10befeb;
 loop { match pc {
 0x10befeb => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010befee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10befee); }
 let mut pc: u32 = 0x10befee;
 loop { match pc {
 0x10befee => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beff1); }
 let mut pc: u32 = 0x10beff1;
 loop { match pc {
 0x10beff1 => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beff4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beff4); }
 let mut pc: u32 = 0x10beff4;
 loop { match pc {
 0x10beff4 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beff7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beff7); }
 let mut pc: u32 = 0x10beff7;
 loop { match pc {
 0x10beff7 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beffa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beffa); }
 let mut pc: u32 = 0x10beffa;
 loop { match pc {
 0x10beffa => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beffd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beffd); }
 let mut pc: u32 = 0x10beffd;
 loop { match pc {
 0x10beffd => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf000); }
 let mut pc: u32 = 0x10bf000;
 loop { match pc {
 0x10bf000 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf003(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf003); }
 let mut pc: u32 = 0x10bf003;
 loop { match pc {
 0x10bf003 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf006(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf006); }
 let mut pc: u32 = 0x10bf006;
 loop { match pc {
 0x10bf006 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf009(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf009); }
 let mut pc: u32 = 0x10bf009;
 loop { match pc {
 0x10bf009 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf00c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf00c); }
 let mut pc: u32 = 0x10bf00c;
 loop { match pc {
 0x10bf00c => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf00f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf00f); }
 let mut pc: u32 = 0x10bf00f;
 loop { match pc {
 0x10bf00f => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf012(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf012); }
 let mut pc: u32 = 0x10bf012;
 loop { match pc {
 0x10bf012 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10bf061; continue; }
  pc = 0x10bf01c; continue;
 }
 0x10bf01c => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x10bf061; continue; }
  pc = 0x10bf021; continue;
 }
 0x10bf021 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf061; continue; }
  pc = 0x10bf036; continue;
 }
 0x10bf036 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf045; continue; }
  pc = 0x10bf040; continue;
 }
 0x10bf040 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x10bf045; continue;
 }
 0x10bf045 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x10bf065; continue; }
  pc = 0x10bf04b; continue;
 }
 0x10bf04b => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf045; continue; }
  pc = 0x10bf05e; continue;
 }
 0x10bf05e => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf061; continue;
 }
 0x10bf061 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10bf0b0; continue;
 }
 0x10bf065 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x10bf093; continue; }
  pc = 0x10bf075; continue;
 }
 0x10bf075 => {
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
  pc = 0x10bf0a8; continue;
 }
 0x10bf093 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x10bf0a8; continue;
 }
 0x10bf0a8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x10bf0b0; continue;
 }
 0x10bf0b0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10bf0b4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf0ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf0ba); }
 let mut pc: u32 = 0x10bf0ba;
 loop { match pc {
 0x10bf0b5 => {
  pc = 0x10bf1c0; continue;
 }
 0x10bf0ba => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x10bf0b5; continue; }
  pc = 0x10bf0c4; continue;
 }
 0x10bf0c4 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10bf0b5; continue; }
  pc = 0x10bf0cf; continue;
 }
 0x10bf0cf => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x10bf0fc; continue; }
  pc = 0x10bf0dd; continue;
 }
 0x10bf0dd => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x10bf0e5; continue; }
  pc = 0x10bf0e2; continue;
 }
 0x10bf0e2 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf0e5; continue;
 }
 0x10bf0e5 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf178; continue;
 }
 0x10bf0fc => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10bf113; continue; }
  pc = 0x10bf105; continue;
 }
 0x10bf105 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf178; continue; }
  pc = 0x10bf10a; continue;
 }
 0x10bf10a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10bf178; continue; }
  pc = 0x10bf111; continue;
 }
 0x10bf111 => {
  pc = 0x10bf122; continue;
 }
 0x10bf113 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10bf178; continue; }
  pc = 0x10bf11a; continue;
 }
 0x10bf11a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf178; continue; }
  pc = 0x10bf122; continue;
 }
 0x10bf122 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x10bf157; continue; }
  pc = 0x10bf132; continue;
 }
 0x10bf132 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf13f; continue; }
  pc = 0x10bf13d; continue;
 }
 0x10bf13d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x10bf13f; continue;
 }
 0x10bf13f => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x10bf143; continue;
 }
 0x10bf143 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10bf178; continue; }
  pc = 0x10bf14a; continue;
 }
 0x10bf14a => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bf157; continue; }
  pc = 0x10bf14f; continue;
 }
 0x10bf14f => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10bf143; continue; }
  pc = 0x10bf157; continue;
 }
 0x10bf157 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10bf161; continue; }
  pc = 0x10bf15e; continue;
 }
 0x10bf15e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x10bf161; continue;
 }
 0x10bf161 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10bf178; continue; }
  pc = 0x10bf165; continue;
 }
 0x10bf165 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10bf178; continue; }
  pc = 0x10bf16c; continue;
 }
 0x10bf16c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x10bf178; continue; }
  pc = 0x10bf173; continue;
 }
 0x10bf173 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x10bf161; continue;
 }
 0x10bf178 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10bf197; continue; }
  pc = 0x10bf187; continue;
 }
 0x10bf187 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x10bf193; continue; }
  pc = 0x10bf190; continue;
 }
 0x10bf190 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf193; continue;
 }
 0x10bf193 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x10bf1a8; continue;
 }
 0x10bf197 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf1a8; continue;
 }
 0x10bf1a8 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x10bf1b8; continue; }
  pc = 0x10bf1b0; continue;
 }
 0x10bf1b0 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x10bf1b8; continue; }
  pc = 0x10bf1b5; continue;
 }
 0x10bf1b5 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf1b8; continue;
 }
 0x10bf1b8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x10bf1c0; continue; }
  pc = 0x10bf1bd; continue;
 }
 0x10bf1bd => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10bf1c0; continue;
 }
 0x10bf1c0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10bf1c4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf1c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf1c5); }
 let mut pc: u32 = 0x10bf1c5;
 loop { match pc {
 0x10bf1c5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x117b208u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x10bf1db; continue; }
  pc = 0x10bf1d2; continue;
 }
 0x10bf1d2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117b208u32, v); }
  pc = 0x10bf1db; continue;
 }
 0x10bf1db => {
  { cpu.sub32(cpu.eax, cpu.r32(0x12d183cu32)); }
  if !cpu.zf { pc = 0x10bf1eb; continue; }
  pc = 0x10bf1e3; continue;
 }
 0x10bf1e3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d183cu32, v); }
  pc = 0x10bf1eb; continue;
 }
 0x10bf1eb => {
  { let v: u32 = cpu.r32(0x117b204u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10bf1fe; continue; }
  pc = 0x10bf1f5; continue;
 }
 0x10bf1f5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117b204u32, v); }
  pc = 0x10bf1fe; continue;
 }
 0x10bf1fe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bf20b; continue; }
  pc = 0x10bf208; continue;
 }
 0x10bf208 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x10bf20b; continue;
 }
 0x10bf20b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf212; continue; }
  pc = 0x10bf20f; continue;
 }
 0x10bf20f => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bf212; continue;
 }
 0x10bf212 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf216(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bf216); } r_010bf216(cpu, 0x10bf216); }
pub fn f_010bf435(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bf435); } r_010bf216(cpu, 0x10bf435); }
pub fn f_010bf506(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bf506); } r_010bf216(cpu, 0x10bf506); }
fn r_010bf216(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10bf216 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x117b204u32); cpu.edx = v; }
  pc = 0x10bf221; continue;
 }
 0x10bf221 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bf500; continue; }
  pc = 0x10bf229; continue;
 }
 0x10bf229 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x10bf26d; continue; }
  pc = 0x10bf237; continue;
 }
 0x10bf237 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x10bf244); f_010bf1c5(cpu); if cpu.returned_to != 0x10bf244 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10bf25d; continue; }
  pc = 0x10bf24d; continue;
 }
 0x10bf24d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x10bf221; continue;
 }
 0x10bf25d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x10bf221; continue;
 }
 0x10bf26d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x10bf221; continue;
 }
 0x10bf435 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x10bf449); f_010bf216(cpu); if cpu.returned_to != 0x10bf449 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10bf464; continue; }
  pc = 0x10bf454; continue;
 }
 0x10bf454 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bf464; continue;
 }
 0x10bf464 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf481; continue; }
  pc = 0x10bf468; continue;
 }
 0x10bf468 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10bf4fd; continue;
 }
 0x10bf481 => {
  { cpu.and16(cpu.r16(0x12d24c6u32), 0xfff0u16); }
  if cpu.zf { pc = 0x10bf496; continue; }
  pc = 0x10bf48c; continue;
 }
 0x10bf48c => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x12d24c4u32, v); }
  pc = 0x10bf496; continue;
 }
 0x10bf496 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf4fb; continue; }
  pc = 0x10bf49f; continue;
 }
 0x10bf49f => {
  { let v: u32 = cpu.r32(0x12d24c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10bf4fb; continue; }
  pc = 0x10bf4b9; continue;
 }
 0x10bf4b9 => {
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
  if (cpu.sf != cpu.of) { pc = 0x10bf4fb; continue; }
  pc = 0x10bf4dd; continue;
 }
 0x10bf4dd => {
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
  pc = 0x10bf4fd; continue;
 }
 0x10bf4fb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bf4fd; continue;
 }
 0x10bf4fd => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10bf500; continue;
 }
 0x10bf500 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bf506 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117ccb0u32), 0x0u32); }
  if !cpu.zf { pc = 0x10bf519; continue; }
  pc = 0x10bf515; continue;
 }
 0x10bf515 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x10bf4fd; continue;
 }
 0x10bf519 => {
  { cpu.sub32(cpu.r32(0x117ba1cu32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x10bf515; continue; }
  pc = 0x10bf522; continue;
 }
 0x10bf522 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10bf529); f_010bf702(cpu); if cpu.returned_to != 0x10bf529 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x10bf4fd; continue; }
  pc = 0x10bf52d; continue;
 }
 0x10bf52d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bf54e; continue; }
  pc = 0x10bf539; continue;
 }
 0x10bf539 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10bf541); f_010bf435(cpu); if cpu.returned_to != 0x10bf541 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf569; continue; }
  pc = 0x10bf547; continue;
 }
 0x10bf547 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10bf569; continue;
 }
 0x10bf54e => {
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
  pc = 0x10bf569; continue;
 }
 0x10bf569 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x10bf515; continue; }
  pc = 0x10bf56d; continue;
 }
 0x10bf56d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10bf515; continue; }
  pc = 0x10bf57a; continue;
 }
 0x10bf57a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x10bf515; continue; }
  pc = 0x10bf582; continue;
 }
 0x10bf582 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10bf58b); f_010bf371(cpu); if cpu.returned_to != 0x10bf58b { return; }
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
  cpu.push(0x10bf5ac); f_010b00ed(cpu); if cpu.returned_to != 0x10bf5ac { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10bf4fd; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf272(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf272); }
 let mut pc: u32 = 0x10bf272;
 loop { match pc {
 0x10bf272 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x10bf280); f_010bf216(cpu); if cpu.returned_to != 0x10bf280 { return; }
  { let v: u32 = cpu.r32(0x117b204u32); cpu.edx = v; }
  pc = 0x10bf361; continue;
 }
 0x10bf28b => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf369; continue; }
  pc = 0x10bf293; continue;
 }
 0x10bf293 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x10bf35e; continue; }
  pc = 0x10bf29e; continue;
 }
 0x10bf29e => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x10bf35e; continue; }
  pc = 0x10bf2a8; continue;
 }
 0x10bf2a8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10bf2af); f_010bf1c5(cpu); if cpu.returned_to != 0x10bf2af { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10bf369; continue; }
  pc = 0x10bf2bc; continue;
 }
 0x10bf2bc => {
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
  if cpu.zf { pc = 0x10bf300; continue; }
  pc = 0x10bf2f0; continue;
 }
 0x10bf2f0 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bf300; continue;
 }
 0x10bf300 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf36b; continue; }
  pc = 0x10bf304; continue;
 }
 0x10bf304 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10bf31b); f_010bf371(cpu); if cpu.returned_to != 0x10bf31b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x10bf357; continue; }
  pc = 0x10bf330; continue;
 }
 0x10bf330 => {
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
  cpu.push(0x10bf355); f_010b00ed(cpu); if cpu.returned_to != 0x10bf355 { return; }
  pc = 0x10bf35a; continue;
 }
 0x10bf357 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bf35a; continue;
 }
 0x10bf35a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10bf36b; continue;
 }
 0x10bf35e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x10bf361; continue;
 }
 0x10bf361 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf28b; continue; }
  pc = 0x10bf369; continue;
 }
 0x10bf369 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10bf36b; continue;
 }
 0x10bf36b => {
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
pub fn f_010bf371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf371); }
 let mut pc: u32 = 0x10bf371;
 loop { match pc {
 0x10bf371 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117b204u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10bf387; continue;
 }
 0x10bf37e => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x10bf38b; continue; }
  pc = 0x10bf382; continue;
 }
 0x10bf382 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10bf387; continue;
 }
 0x10bf387 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf37e; continue; }
  pc = 0x10bf38b; continue;
 }
 0x10bf38b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bf39a; continue; }
  pc = 0x10bf395; continue;
 }
 0x10bf395 => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bf3a0; continue;
 }
 0x10bf39a => {
  { let v: u32 = cpu.edx; cpu.w32(0x117b204u32, v); }
  pc = 0x10bf3a0; continue;
 }
 0x10bf3a0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf3a7; continue; }
  pc = 0x10bf3a4; continue;
 }
 0x10bf3a4 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bf3a7; continue;
 }
 0x10bf3a7 => {
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
pub fn f_010bf3e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf3e5); }
 let mut pc: u32 = 0x10bf3e5;
 loop { match pc {
 0x10bf3e5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x117b204u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bf430; continue; }
  pc = 0x10bf3f1; continue;
 }
 0x10bf3f1 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bf423; continue; }
  pc = 0x10bf407; continue;
 }
 0x10bf407 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10bf423; continue; }
  pc = 0x10bf40c; continue;
 }
 0x10bf40c => {
  { cpu.sub8(cpu.r8(0x117ba4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x10bf423; continue; }
  pc = 0x10bf415; continue;
 }
 0x10bf415 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x10bf41a, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117ba1cu32, v); }
  pc = 0x10bf423; continue;
 }
 0x10bf423 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x117ba1cu32)); }
  if !cpu.zf { pc = 0x10bf430; continue; }
  pc = 0x10bf42b; continue;
 }
 0x10bf42b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bf430 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf5b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf5b6); }
 let mut pc: u32 = 0x10bf5b6;
 loop { match pc {
 0x10bf5b6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bf5d1; continue; }
  pc = 0x10bf5c8; continue;
 }
 0x10bf5c8 => {
  { cpu.sub8(cpu.r8(0x117ba4bu32), 0x0u8); }
  if cpu.zf { pc = 0x10bf5dd; continue; }
  pc = 0x10bf5d1; continue;
 }
 0x10bf5d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x10bf5ea; continue; }
  pc = 0x10bf5dd; continue;
 }
 0x10bf5dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10bf5e5); f_010bf506(cpu); if cpu.returned_to != 0x10bf5e5 { return; }
  pc = 0x10bf6f9; continue;
 }
 0x10bf5ea => {
  { cpu.sub32(cpu.r32(0x117ccb0u32), 0x0u32); }
  if !cpu.zf { pc = 0x10bf5fa; continue; }
  pc = 0x10bf5f3; continue;
 }
 0x10bf5f3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bf6f9; continue;
 }
 0x10bf5fa => {
  { cpu.sub32(cpu.r32(0x117ba1cu32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf603; continue;
 }
 0x10bf603 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10bf60a); f_010bf702(cpu); if cpu.returned_to != 0x10bf60a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bf6f9; continue; }
  pc = 0x10bf612; continue;
 }
 0x10bf612 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bf63a; continue; }
  pc = 0x10bf61e; continue;
 }
 0x10bf61e => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10bf63a; continue; }
  pc = 0x10bf623; continue;
 }
 0x10bf623 => {
  { cpu.sub8(cpu.r8(0x117ba4cu32), 0x0u8); }
  if !cpu.zf { pc = 0x10bf63a; continue; }
  pc = 0x10bf62c; continue;
 }
 0x10bf62c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x10bf631, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117ba1cu32, v); }
  pc = 0x10bf63a; continue;
 }
 0x10bf63a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117ba1cu32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x10bf64e; continue; }
  pc = 0x10bf649; continue;
 }
 0x10bf649 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x10bf64e; continue;
 }
 0x10bf64e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10bf655); f_010c6379(cpu); if cpu.returned_to != 0x10bf655 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf65a; continue;
 }
 0x10bf65a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf661; continue;
 }
 0x10bf661 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf665; continue;
 }
 0x10bf665 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf671; continue;
 }
 0x10bf671 => {
  { let v: u32 = cpu.r32(0x117b204u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10bf68e; continue;
 }
 0x10bf67c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10bf692; continue; }
  pc = 0x10bf683; continue;
 }
 0x10bf683 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10bf68b; continue; }
  pc = 0x10bf687; continue;
 }
 0x10bf687 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x10bf692; continue; }
  pc = 0x10bf68b; continue;
 }
 0x10bf68b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x10bf68e; continue;
 }
 0x10bf68e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf67c; continue; }
  pc = 0x10bf692; continue;
 }
 0x10bf692 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bf6bc; continue; }
  pc = 0x10bf696; continue;
 }
 0x10bf696 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10bf6bc; continue; }
  pc = 0x10bf6a1; continue;
 }
 0x10bf6a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10bf6db; continue;
 }
 0x10bf6bc => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x10bf5f3; continue; }
  pc = 0x10bf6c6; continue;
 }
 0x10bf6c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10bf6d4); f_010bf371(cpu); if cpu.returned_to != 0x10bf6d4 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10bf6db; continue;
 }
 0x10bf6db => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bf6f4); f_010b00ed(cpu); if cpu.returned_to != 0x10bf6f4 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10bf6f9; continue;
 }
 0x10bf6f9 => {
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
pub fn f_010bf702(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf702); }
 let mut pc: u32 = 0x10bf702;
 loop { match pc {
 0x10bf702 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10bf719; continue; }
  pc = 0x10bf713; continue;
 }
 0x10bf713 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bf719 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bf72e; continue; }
  pc = 0x10bf725; continue;
 }
 0x10bf725 => {
  { cpu.sub8(cpu.r8(0x117ba4bu32), 0x0u8); }
  if cpu.zf { pc = 0x10bf73a; continue; }
  pc = 0x10bf72e; continue;
 }
 0x10bf72e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x10bf73f; continue; }
  pc = 0x10bf73a; continue;
 }
 0x10bf73a => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x10bf746; continue;
 }
 0x10bf73f => {
  cpu.push(0x10bf744); f_010bf3e5(cpu); if cpu.returned_to != 0x10bf744 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x10bf746; continue;
 }
 0x10bf746 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x10bf713; continue; }
  pc = 0x10bf74f; continue;
 }
 0x10bf74f => {
  { let v: u32 = cpu.r32(0x117ccb4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x10bf75e; continue; }
  pc = 0x10bf759; continue;
 }
 0x10bf759 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10bf75e; continue;
 }
 0x10bf75e => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x10bf713; continue; }
  pc = 0x10bf76a; continue;
 }
 0x10bf76a => {
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
pub fn f_010bf77f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf77f); }
 let mut pc: u32 = 0x10bf77f;
 loop { match pc {
 0x10bf77f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf782(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf782); }
 let mut pc: u32 = 0x10bf782;
 loop { match pc {
 0x10bf782 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bf789); f_010bf891(cpu); if cpu.returned_to != 0x10bf789 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x10bf794; continue;
 }
 0x10bf794 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x10bf79d; continue; }
  pc = 0x10bf799; continue;
 }
 0x10bf799 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x10bf79d; continue;
 }
 0x10bf79d => {
  if cpu.cf { pc = 0x10bf7a5; continue; }
  pc = 0x10bf79f; continue;
 }
 0x10bf79f => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10bf794; continue;
 }
 0x10bf7a5 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x10bf7a8; continue;
 }
 0x10bf7a8 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x10bf7b1; continue; }
  pc = 0x10bf7ad; continue;
 }
 0x10bf7ad => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x10bf7b1; continue;
 }
 0x10bf7b1 => {
  if !cpu.cf { pc = 0x10bf7b9; continue; }
  pc = 0x10bf7b3; continue;
 }
 0x10bf7b3 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10bf7a8; continue;
 }
 0x10bf7b9 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf7bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf7bc); }
 let mut pc: u32 = 0x10bf7bc;
 loop { match pc {
 0x10bf7bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0xfff00000u32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ 0x100000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 20u32); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x433u32); cpu.ebp = v; }
  if cpu.zf { pc = 0x10bf81a; continue; }
  pc = 0x10bf7e5; continue;
 }
 0x10bf7e5 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bf7f0; continue; }
  pc = 0x10bf7e7; continue;
 }
 0x10bf7e7 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10bf7e7; continue; }
  pc = 0x10bf7ee; continue;
 }
 0x10bf7ee => {
  pc = 0x10bf81a; continue;
 }
 0x10bf7f0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x10bf7f4; continue;
 }
 0x10bf7f4 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10bf7f4; continue; }
  pc = 0x10bf7ff; continue;
 }
 0x10bf7ff => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x10bf81a; continue; }
  pc = 0x10bf806; continue;
 }
 0x10bf806 => {
  if !cpu.zf { pc = 0x10bf814; continue; }
  pc = 0x10bf808; continue;
 }
 0x10bf808 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x10bf814; continue; }
  pc = 0x10bf80c; continue;
 }
 0x10bf80c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x10bf81a; continue; }
  pc = 0x10bf814; continue;
 }
 0x10bf814 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x10bf81a; continue;
 }
 0x10bf81a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf827(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf827); }
 let mut pc: u32 = 0x10bf827;
 loop { match pc {
 0x10bf827 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x10bf83c; continue;
 }
 0x10bf83c => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x10bf84f; continue; }
  pc = 0x10bf848; continue;
 }
 0x10bf848 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x10bf851; continue; }
  pc = 0x10bf84c; continue;
 }
 0x10bf84c => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x10bf867; continue;
 }
 0x10bf84f => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x10bf851; continue;
 }
 0x10bf851 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x10bf860, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x10bf865, "Aam");
  pc = 0x10bf867; continue;
 }
 0x10bf867 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x10bf88b; continue; }
  pc = 0x10bf879; continue;
 }
 0x10bf879 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x10bf88b; continue; }
  pc = 0x10bf87f; continue;
 }
 0x10bf87f => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x10bf88b; continue; }
  pc = 0x10bf885; continue;
 }
 0x10bf885 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10bf83c; continue; }
  pc = 0x10bf88b; continue;
 }
 0x10bf88b => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf891(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf891); }
 let mut pc: u32 = 0x10bf891;
 loop { match pc {
 0x10bf891 => {
  cpu.push(0x10bf896); f_010bf946(cpu); if cpu.returned_to != 0x10bf896 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[4]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10bf8c3, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10bf8d4, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x10bf8da; continue; }
  pc = 0x10bf8e5; continue;
 }
 0x10bf8da => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x10bf8da; continue; }
  pc = 0x10bf8e5; continue;
 }
 0x10bf8e5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10bf8f3); cpu.call(0x0); if cpu.returned_to != 0x10bf8f3 { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10bf900); cpu.call(0x1f44176); if cpu.returned_to != 0x10bf900 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x10bf905, "Aam");
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x5af34e72u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x7au32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x7au32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0xffffff8du32.wrapping_add(cpu.esi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0xffffff8du32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x86f2a4c6u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x86f2a4c6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax & cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.r32(0x78u32.wrapping_add(cpu.edi)), 69u32); cpu.w32(0x78u32.wrapping_add(cpu.edi), v); }
  cpu.unimplemented(0x10bf928, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x10bf938); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf946(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf946); }
 let mut pc: u32 = 0x10bf946;
 loop { match pc {
 0x10bf946 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf948); }
 let mut pc: u32 = 0x10bf948;
 loop { match pc {
 0x10bf948 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x116b3d4u32; cpu.eax = v; }
  cpu.push(0x10bf958); f_010be875(cpu); if cpu.returned_to != 0x10bf958 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf95a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bf95a); } r_010bf95a(cpu, 0x10bf95a); }
pub fn f_010bf9e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bf9e4); } r_010bf95a(cpu, 0x10bf9e4); }
fn r_010bf95a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10bf95a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10bf960, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x10bf965, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x10bf9e6; continue; }
  pc = 0x10bf975; continue;
 }
 0x10bf975 => {
  if cpu.cf { pc = 0x10bf9ea; continue; }
  pc = 0x10bf977; continue;
 }
 0x10bf977 => {
  if cpu.zf { pc = 0x10bf9ee; continue; }
  pc = 0x10bf979; continue;
 }
 0x10bf979 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bf9f2; continue; }
  pc = 0x10bf97b; continue;
 }
 0x10bf97b => {
  if cpu.sf { pc = 0x10bf9f6; continue; }
  pc = 0x10bf97d; continue;
 }
 0x10bf97d => {
  if cpu.pf() { f_010bf97f(cpu); return; }
  f_010bf97f(cpu); return;
 }
 0x10bf9e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x10bfa2c; continue; }
  pc = 0x10bf9ed; continue;
 }
 0x10bf9e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bf9ea => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_010bfa60(cpu); return; }
  pc = 0x10bf9ec; continue;
 }
 0x10bf9ec => {
  cpu.unimplemented(0x10bf9ec, "Aas");
  pc = 0x10bf9ed; continue;
 }
 0x10bf9ed => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x10bfa23; continue; }
  pc = 0x10bf9ff; continue;
 }
 0x10bf9ee => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x10bfa23; continue; }
  pc = 0x10bf9ff; continue;
 }
 0x10bf9f2 => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x10bfa23; continue; }
  pc = 0x10bf9ff; continue;
 }
 0x10bf9f6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x10bfa23; continue; }
  pc = 0x10bf9ff; continue;
 }
 0x10bf9ff => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x10bfa23; continue; }
  pc = 0x10bfa07; continue;
 }
 0x10bfa07 => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x10bfa2c; continue; }
  pc = 0x10bfa0f; continue;
 }
 0x10bfa0f => {
  { let v: u32 = cpu.edx.wrapping_sub(0x70000000u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bfa23 => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bfa2c => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf97f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf97f); }
 let mut pc: u32 = 0x10bf97f;
 loop { match pc {
 0x10bf97f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10bf995; continue;
 }
 0x10bf995 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10bf95au32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf995; continue; }
  pc = 0x10bf9b5; continue;
 }
 0x10bf9b5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10bf9b5; continue; }
  pc = 0x10bf9bf; continue;
 }
 0x10bf9bf => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bf9c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bf9c9); }
 let mut pc: u32 = 0x10bf9c9;
 loop { match pc {
 0x10bf9c9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x10bf9db; continue; }
  pc = 0x10bf9d1; continue;
 }
 0x10bf9d1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10bf9db; continue; }
  pc = 0x10bf9d5; continue;
 }
 0x10bf9d5 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10bf9db; continue;
 }
 0x10bf9db => {
  cpu.push(0x10bf9e0); f_010bf97f(cpu); if cpu.returned_to != 0x10bf9e0 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa30); }
 let mut pc: u32 = 0x10bfa30;
 loop { match pc {
 0x10bfa30 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.df = false; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa54); }
 let mut pc: u32 = 0x10bfa54;
 loop { match pc {
 0x10bfa54 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa58); }
 let mut pc: u32 = 0x10bfa58;
 loop { match pc {
 0x10bfa58 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa5c); }
 let mut pc: u32 = 0x10bfa5c;
 loop { match pc {
 0x10bfa5c => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa60); }
 let mut pc: u32 = 0x10bfa60;
 loop { match pc {
 0x10bfa60 => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa64); }
 let mut pc: u32 = 0x10bfa64;
 loop { match pc {
 0x10bfa64 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa68); }
 let mut pc: u32 = 0x10bfa68;
 loop { match pc {
 0x10bfa68 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa6c); }
 let mut pc: u32 = 0x10bfa6c;
 loop { match pc {
 0x10bfa6c => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa70); }
 let mut pc: u32 = 0x10bfa70;
 loop { match pc {
 0x10bfa70 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa74); }
 let mut pc: u32 = 0x10bfa74;
 loop { match pc {
 0x10bfa74 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa78); }
 let mut pc: u32 = 0x10bfa78;
 loop { match pc {
 0x10bfa78 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa7c); }
 let mut pc: u32 = 0x10bfa7c;
 loop { match pc {
 0x10bfa7c => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa80); }
 let mut pc: u32 = 0x10bfa80;
 loop { match pc {
 0x10bfa80 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa84); }
 let mut pc: u32 = 0x10bfa84;
 loop { match pc {
 0x10bfa84 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa88); }
 let mut pc: u32 = 0x10bfa88;
 loop { match pc {
 0x10bfa88 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa8c); }
 let mut pc: u32 = 0x10bfa8c;
 loop { match pc {
 0x10bfa8c => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa90); }
 let mut pc: u32 = 0x10bfa90;
 loop { match pc {
 0x10bfa90 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa94); }
 let mut pc: u32 = 0x10bfa94;
 loop { match pc {
 0x10bfa94 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa98); }
 let mut pc: u32 = 0x10bfa98;
 loop { match pc {
 0x10bfa98 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfa9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfa9c); }
 let mut pc: u32 = 0x10bfa9c;
 loop { match pc {
 0x10bfa9c => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaa0); }
 let mut pc: u32 = 0x10bfaa0;
 loop { match pc {
 0x10bfaa0 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaa4); }
 let mut pc: u32 = 0x10bfaa4;
 loop { match pc {
 0x10bfaa4 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaa8); }
 let mut pc: u32 = 0x10bfaa8;
 loop { match pc {
 0x10bfaa8 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaac); }
 let mut pc: u32 = 0x10bfaac;
 loop { match pc {
 0x10bfaac => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfab0); }
 let mut pc: u32 = 0x10bfab0;
 loop { match pc {
 0x10bfab0 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfab4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfab4); }
 let mut pc: u32 = 0x10bfab4;
 loop { match pc {
 0x10bfab4 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfab8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfab8); }
 let mut pc: u32 = 0x10bfab8;
 loop { match pc {
 0x10bfab8 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfabc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfabc); }
 let mut pc: u32 = 0x10bfabc;
 loop { match pc {
 0x10bfabc => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfac0); }
 let mut pc: u32 = 0x10bfac0;
 loop { match pc {
 0x10bfac0 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfac4); }
 let mut pc: u32 = 0x10bfac4;
 loop { match pc {
 0x10bfac4 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfac8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfac8); }
 let mut pc: u32 = 0x10bfac8;
 loop { match pc {
 0x10bfac8 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfacc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfacc); }
 let mut pc: u32 = 0x10bfacc;
 loop { match pc {
 0x10bfacc => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfad0); }
 let mut pc: u32 = 0x10bfad0;
 loop { match pc {
 0x10bfad0 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfad4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfad4); }
 let mut pc: u32 = 0x10bfad4;
 loop { match pc {
 0x10bfad4 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfad8); }
 let mut pc: u32 = 0x10bfad8;
 loop { match pc {
 0x10bfad8 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfadc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfadc); }
 let mut pc: u32 = 0x10bfadc;
 loop { match pc {
 0x10bfadc => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfae0); }
 let mut pc: u32 = 0x10bfae0;
 loop { match pc {
 0x10bfae0 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfae4); }
 let mut pc: u32 = 0x10bfae4;
 loop { match pc {
 0x10bfae4 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfae8); }
 let mut pc: u32 = 0x10bfae8;
 loop { match pc {
 0x10bfae8 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaec); }
 let mut pc: u32 = 0x10bfaec;
 loop { match pc {
 0x10bfaec => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaf0); }
 let mut pc: u32 = 0x10bfaf0;
 loop { match pc {
 0x10bfaf0 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaf4); }
 let mut pc: u32 = 0x10bfaf4;
 loop { match pc {
 0x10bfaf4 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfaf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfaf8); }
 let mut pc: u32 = 0x10bfaf8;
 loop { match pc {
 0x10bfaf8 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfafc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfafc); }
 let mut pc: u32 = 0x10bfafc;
 loop { match pc {
 0x10bfafc => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb00); }
 let mut pc: u32 = 0x10bfb00;
 loop { match pc {
 0x10bfb00 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb04); }
 let mut pc: u32 = 0x10bfb04;
 loop { match pc {
 0x10bfb04 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb08); }
 let mut pc: u32 = 0x10bfb08;
 loop { match pc {
 0x10bfb08 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb0c); }
 let mut pc: u32 = 0x10bfb0c;
 loop { match pc {
 0x10bfb0c => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb10); }
 let mut pc: u32 = 0x10bfb10;
 loop { match pc {
 0x10bfb10 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb14); }
 let mut pc: u32 = 0x10bfb14;
 loop { match pc {
 0x10bfb14 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb18); }
 let mut pc: u32 = 0x10bfb18;
 loop { match pc {
 0x10bfb18 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb1c); }
 let mut pc: u32 = 0x10bfb1c;
 loop { match pc {
 0x10bfb1c => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb20); }
 let mut pc: u32 = 0x10bfb20;
 loop { match pc {
 0x10bfb20 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb24); }
 let mut pc: u32 = 0x10bfb24;
 loop { match pc {
 0x10bfb24 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb28); }
 let mut pc: u32 = 0x10bfb28;
 loop { match pc {
 0x10bfb28 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb2c); }
 let mut pc: u32 = 0x10bfb2c;
 loop { match pc {
 0x10bfb2c => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb30); }
 let mut pc: u32 = 0x10bfb30;
 loop { match pc {
 0x10bfb30 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb34); }
 let mut pc: u32 = 0x10bfb34;
 loop { match pc {
 0x10bfb34 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb38); }
 let mut pc: u32 = 0x10bfb38;
 loop { match pc {
 0x10bfb38 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb3c); }
 let mut pc: u32 = 0x10bfb3c;
 loop { match pc {
 0x10bfb3c => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb40); }
 let mut pc: u32 = 0x10bfb40;
 loop { match pc {
 0x10bfb40 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb44); }
 let mut pc: u32 = 0x10bfb44;
 loop { match pc {
 0x10bfb44 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb48); }
 let mut pc: u32 = 0x10bfb48;
 loop { match pc {
 0x10bfb48 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb4c); }
 let mut pc: u32 = 0x10bfb4c;
 loop { match pc {
 0x10bfb4c => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb50); }
 let mut pc: u32 = 0x10bfb50;
 loop { match pc {
 0x10bfb50 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb54); }
 let mut pc: u32 = 0x10bfb54;
 loop { match pc {
 0x10bfb54 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb58); }
 let mut pc: u32 = 0x10bfb58;
 loop { match pc {
 0x10bfb58 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb5c); }
 let mut pc: u32 = 0x10bfb5c;
 loop { match pc {
 0x10bfb5c => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb60); }
 let mut pc: u32 = 0x10bfb60;
 loop { match pc {
 0x10bfb60 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb64); }
 let mut pc: u32 = 0x10bfb64;
 loop { match pc {
 0x10bfb64 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb68); }
 let mut pc: u32 = 0x10bfb68;
 loop { match pc {
 0x10bfb68 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb6c); }
 let mut pc: u32 = 0x10bfb6c;
 loop { match pc {
 0x10bfb6c => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb70); }
 let mut pc: u32 = 0x10bfb70;
 loop { match pc {
 0x10bfb70 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb74); }
 let mut pc: u32 = 0x10bfb74;
 loop { match pc {
 0x10bfb74 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb78); }
 let mut pc: u32 = 0x10bfb78;
 loop { match pc {
 0x10bfb78 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb7c); }
 let mut pc: u32 = 0x10bfb7c;
 loop { match pc {
 0x10bfb7c => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb80); }
 let mut pc: u32 = 0x10bfb80;
 loop { match pc {
 0x10bfb80 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb84); }
 let mut pc: u32 = 0x10bfb84;
 loop { match pc {
 0x10bfb84 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb88); }
 let mut pc: u32 = 0x10bfb88;
 loop { match pc {
 0x10bfb88 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb8c); }
 let mut pc: u32 = 0x10bfb8c;
 loop { match pc {
 0x10bfb8c => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb90); }
 let mut pc: u32 = 0x10bfb90;
 loop { match pc {
 0x10bfb90 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb94); }
 let mut pc: u32 = 0x10bfb94;
 loop { match pc {
 0x10bfb94 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb98); }
 let mut pc: u32 = 0x10bfb98;
 loop { match pc {
 0x10bfb98 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfb9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfb9c); }
 let mut pc: u32 = 0x10bfb9c;
 loop { match pc {
 0x10bfb9c => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfba0); }
 let mut pc: u32 = 0x10bfba0;
 loop { match pc {
 0x10bfba0 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfba4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfba4); }
 let mut pc: u32 = 0x10bfba4;
 loop { match pc {
 0x10bfba4 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfba8); }
 let mut pc: u32 = 0x10bfba8;
 loop { match pc {
 0x10bfba8 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbac); }
 let mut pc: u32 = 0x10bfbac;
 loop { match pc {
 0x10bfbac => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbb0); }
 let mut pc: u32 = 0x10bfbb0;
 loop { match pc {
 0x10bfbb0 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbb4); }
 let mut pc: u32 = 0x10bfbb4;
 loop { match pc {
 0x10bfbb4 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbb8); }
 let mut pc: u32 = 0x10bfbb8;
 loop { match pc {
 0x10bfbb8 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbbc); }
 let mut pc: u32 = 0x10bfbbc;
 loop { match pc {
 0x10bfbbc => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbc0); }
 let mut pc: u32 = 0x10bfbc0;
 loop { match pc {
 0x10bfbc0 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbc4); }
 let mut pc: u32 = 0x10bfbc4;
 loop { match pc {
 0x10bfbc4 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbc8); }
 let mut pc: u32 = 0x10bfbc8;
 loop { match pc {
 0x10bfbc8 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbcc); }
 let mut pc: u32 = 0x10bfbcc;
 loop { match pc {
 0x10bfbcc => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbd0); }
 let mut pc: u32 = 0x10bfbd0;
 loop { match pc {
 0x10bfbd0 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbd4); }
 let mut pc: u32 = 0x10bfbd4;
 loop { match pc {
 0x10bfbd4 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbd8); }
 let mut pc: u32 = 0x10bfbd8;
 loop { match pc {
 0x10bfbd8 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbdc); }
 let mut pc: u32 = 0x10bfbdc;
 loop { match pc {
 0x10bfbdc => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbe0); }
 let mut pc: u32 = 0x10bfbe0;
 loop { match pc {
 0x10bfbe0 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbe4); }
 let mut pc: u32 = 0x10bfbe4;
 loop { match pc {
 0x10bfbe4 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbe8); }
 let mut pc: u32 = 0x10bfbe8;
 loop { match pc {
 0x10bfbe8 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbec); }
 let mut pc: u32 = 0x10bfbec;
 loop { match pc {
 0x10bfbec => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbf0); }
 let mut pc: u32 = 0x10bfbf0;
 loop { match pc {
 0x10bfbf0 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbf4); }
 let mut pc: u32 = 0x10bfbf4;
 loop { match pc {
 0x10bfbf4 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbf8); }
 let mut pc: u32 = 0x10bfbf8;
 loop { match pc {
 0x10bfbf8 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfbfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfbfc); }
 let mut pc: u32 = 0x10bfbfc;
 loop { match pc {
 0x10bfbfc => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc00); }
 let mut pc: u32 = 0x10bfc00;
 loop { match pc {
 0x10bfc00 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc04); }
 let mut pc: u32 = 0x10bfc04;
 loop { match pc {
 0x10bfc04 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc08); }
 let mut pc: u32 = 0x10bfc08;
 loop { match pc {
 0x10bfc08 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc0c); }
 let mut pc: u32 = 0x10bfc0c;
 loop { match pc {
 0x10bfc0c => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc10); }
 let mut pc: u32 = 0x10bfc10;
 loop { match pc {
 0x10bfc10 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc14); }
 let mut pc: u32 = 0x10bfc14;
 loop { match pc {
 0x10bfc14 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc18); }
 let mut pc: u32 = 0x10bfc18;
 loop { match pc {
 0x10bfc18 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc1c); }
 let mut pc: u32 = 0x10bfc1c;
 loop { match pc {
 0x10bfc1c => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc20); }
 let mut pc: u32 = 0x10bfc20;
 loop { match pc {
 0x10bfc20 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc24); }
 let mut pc: u32 = 0x10bfc24;
 loop { match pc {
 0x10bfc24 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc28); }
 let mut pc: u32 = 0x10bfc28;
 loop { match pc {
 0x10bfc28 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc2c); }
 let mut pc: u32 = 0x10bfc2c;
 loop { match pc {
 0x10bfc2c => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc30); }
 let mut pc: u32 = 0x10bfc30;
 loop { match pc {
 0x10bfc30 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc34); }
 let mut pc: u32 = 0x10bfc34;
 loop { match pc {
 0x10bfc34 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc38); }
 let mut pc: u32 = 0x10bfc38;
 loop { match pc {
 0x10bfc38 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc3c); }
 let mut pc: u32 = 0x10bfc3c;
 loop { match pc {
 0x10bfc3c => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc40); }
 let mut pc: u32 = 0x10bfc40;
 loop { match pc {
 0x10bfc40 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc44); }
 let mut pc: u32 = 0x10bfc44;
 loop { match pc {
 0x10bfc44 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc48); }
 let mut pc: u32 = 0x10bfc48;
 loop { match pc {
 0x10bfc48 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc4c); }
 let mut pc: u32 = 0x10bfc4c;
 loop { match pc {
 0x10bfc4c => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc50); }
 let mut pc: u32 = 0x10bfc50;
 loop { match pc {
 0x10bfc50 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc54); }
 let mut pc: u32 = 0x10bfc54;
 loop { match pc {
 0x10bfc54 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc58); }
 let mut pc: u32 = 0x10bfc58;
 loop { match pc {
 0x10bfc58 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc5c); }
 let mut pc: u32 = 0x10bfc5c;
 loop { match pc {
 0x10bfc5c => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc60); }
 let mut pc: u32 = 0x10bfc60;
 loop { match pc {
 0x10bfc60 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc64); }
 let mut pc: u32 = 0x10bfc64;
 loop { match pc {
 0x10bfc64 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc68); }
 let mut pc: u32 = 0x10bfc68;
 loop { match pc {
 0x10bfc68 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc6c); }
 let mut pc: u32 = 0x10bfc6c;
 loop { match pc {
 0x10bfc6c => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc70); }
 let mut pc: u32 = 0x10bfc70;
 loop { match pc {
 0x10bfc70 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc74); }
 let mut pc: u32 = 0x10bfc74;
 loop { match pc {
 0x10bfc74 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc78); }
 let mut pc: u32 = 0x10bfc78;
 loop { match pc {
 0x10bfc78 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc7c); }
 let mut pc: u32 = 0x10bfc7c;
 loop { match pc {
 0x10bfc7c => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc80); }
 let mut pc: u32 = 0x10bfc80;
 loop { match pc {
 0x10bfc80 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc84); }
 let mut pc: u32 = 0x10bfc84;
 loop { match pc {
 0x10bfc84 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc88); }
 let mut pc: u32 = 0x10bfc88;
 loop { match pc {
 0x10bfc88 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc8c); }
 let mut pc: u32 = 0x10bfc8c;
 loop { match pc {
 0x10bfc8c => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc90); }
 let mut pc: u32 = 0x10bfc90;
 loop { match pc {
 0x10bfc90 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc94); }
 let mut pc: u32 = 0x10bfc94;
 loop { match pc {
 0x10bfc94 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc98); }
 let mut pc: u32 = 0x10bfc98;
 loop { match pc {
 0x10bfc98 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfc9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfc9c); }
 let mut pc: u32 = 0x10bfc9c;
 loop { match pc {
 0x10bfc9c => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfca0); }
 let mut pc: u32 = 0x10bfca0;
 loop { match pc {
 0x10bfca0 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfca4); }
 let mut pc: u32 = 0x10bfca4;
 loop { match pc {
 0x10bfca4 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfca8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfca8); }
 let mut pc: u32 = 0x10bfca8;
 loop { match pc {
 0x10bfca8 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcac); }
 let mut pc: u32 = 0x10bfcac;
 loop { match pc {
 0x10bfcac => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcb0); }
 let mut pc: u32 = 0x10bfcb0;
 loop { match pc {
 0x10bfcb0 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcb4); }
 let mut pc: u32 = 0x10bfcb4;
 loop { match pc {
 0x10bfcb4 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcb8); }
 let mut pc: u32 = 0x10bfcb8;
 loop { match pc {
 0x10bfcb8 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcbc); }
 let mut pc: u32 = 0x10bfcbc;
 loop { match pc {
 0x10bfcbc => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcc0); }
 let mut pc: u32 = 0x10bfcc0;
 loop { match pc {
 0x10bfcc0 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcc4); }
 let mut pc: u32 = 0x10bfcc4;
 loop { match pc {
 0x10bfcc4 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcc8); }
 let mut pc: u32 = 0x10bfcc8;
 loop { match pc {
 0x10bfcc8 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfccc); }
 let mut pc: u32 = 0x10bfccc;
 loop { match pc {
 0x10bfccc => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcd0); }
 let mut pc: u32 = 0x10bfcd0;
 loop { match pc {
 0x10bfcd0 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcd4); }
 let mut pc: u32 = 0x10bfcd4;
 loop { match pc {
 0x10bfcd4 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcd8); }
 let mut pc: u32 = 0x10bfcd8;
 loop { match pc {
 0x10bfcd8 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcdc); }
 let mut pc: u32 = 0x10bfcdc;
 loop { match pc {
 0x10bfcdc => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfce0); }
 let mut pc: u32 = 0x10bfce0;
 loop { match pc {
 0x10bfce0 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfce4); }
 let mut pc: u32 = 0x10bfce4;
 loop { match pc {
 0x10bfce4 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfce8); }
 let mut pc: u32 = 0x10bfce8;
 loop { match pc {
 0x10bfce8 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcec); }
 let mut pc: u32 = 0x10bfcec;
 loop { match pc {
 0x10bfcec => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcf0); }
 let mut pc: u32 = 0x10bfcf0;
 loop { match pc {
 0x10bfcf0 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcf4); }
 let mut pc: u32 = 0x10bfcf4;
 loop { match pc {
 0x10bfcf4 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcf8); }
 let mut pc: u32 = 0x10bfcf8;
 loop { match pc {
 0x10bfcf8 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfcfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfcfc); }
 let mut pc: u32 = 0x10bfcfc;
 loop { match pc {
 0x10bfcfc => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd00); }
 let mut pc: u32 = 0x10bfd00;
 loop { match pc {
 0x10bfd00 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd04); }
 let mut pc: u32 = 0x10bfd04;
 loop { match pc {
 0x10bfd04 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd08); }
 let mut pc: u32 = 0x10bfd08;
 loop { match pc {
 0x10bfd08 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd0c); }
 let mut pc: u32 = 0x10bfd0c;
 loop { match pc {
 0x10bfd0c => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd10); }
 let mut pc: u32 = 0x10bfd10;
 loop { match pc {
 0x10bfd10 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd14); }
 let mut pc: u32 = 0x10bfd14;
 loop { match pc {
 0x10bfd14 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd18); }
 let mut pc: u32 = 0x10bfd18;
 loop { match pc {
 0x10bfd18 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd1c); }
 let mut pc: u32 = 0x10bfd1c;
 loop { match pc {
 0x10bfd1c => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd20); }
 let mut pc: u32 = 0x10bfd20;
 loop { match pc {
 0x10bfd20 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd24); }
 let mut pc: u32 = 0x10bfd24;
 loop { match pc {
 0x10bfd24 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd28); }
 let mut pc: u32 = 0x10bfd28;
 loop { match pc {
 0x10bfd28 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd2c); }
 let mut pc: u32 = 0x10bfd2c;
 loop { match pc {
 0x10bfd2c => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd30); }
 let mut pc: u32 = 0x10bfd30;
 loop { match pc {
 0x10bfd30 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd34); }
 let mut pc: u32 = 0x10bfd34;
 loop { match pc {
 0x10bfd34 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd38); }
 let mut pc: u32 = 0x10bfd38;
 loop { match pc {
 0x10bfd38 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd3c); }
 let mut pc: u32 = 0x10bfd3c;
 loop { match pc {
 0x10bfd3c => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd40); }
 let mut pc: u32 = 0x10bfd40;
 loop { match pc {
 0x10bfd40 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd44); }
 let mut pc: u32 = 0x10bfd44;
 loop { match pc {
 0x10bfd44 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd48); }
 let mut pc: u32 = 0x10bfd48;
 loop { match pc {
 0x10bfd48 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd4c); }
 let mut pc: u32 = 0x10bfd4c;
 loop { match pc {
 0x10bfd4c => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd50); }
 let mut pc: u32 = 0x10bfd50;
 loop { match pc {
 0x10bfd50 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd54); }
 let mut pc: u32 = 0x10bfd54;
 loop { match pc {
 0x10bfd54 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd58); }
 let mut pc: u32 = 0x10bfd58;
 loop { match pc {
 0x10bfd58 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd5c); }
 let mut pc: u32 = 0x10bfd5c;
 loop { match pc {
 0x10bfd5c => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd60); }
 let mut pc: u32 = 0x10bfd60;
 loop { match pc {
 0x10bfd60 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd64); }
 let mut pc: u32 = 0x10bfd64;
 loop { match pc {
 0x10bfd64 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd68); }
 let mut pc: u32 = 0x10bfd68;
 loop { match pc {
 0x10bfd68 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd6c); }
 let mut pc: u32 = 0x10bfd6c;
 loop { match pc {
 0x10bfd6c => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd70); }
 let mut pc: u32 = 0x10bfd70;
 loop { match pc {
 0x10bfd70 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd74); }
 let mut pc: u32 = 0x10bfd74;
 loop { match pc {
 0x10bfd74 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd78); }
 let mut pc: u32 = 0x10bfd78;
 loop { match pc {
 0x10bfd78 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd7c); }
 let mut pc: u32 = 0x10bfd7c;
 loop { match pc {
 0x10bfd7c => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd80); }
 let mut pc: u32 = 0x10bfd80;
 loop { match pc {
 0x10bfd80 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd84); }
 let mut pc: u32 = 0x10bfd84;
 loop { match pc {
 0x10bfd84 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd88); }
 let mut pc: u32 = 0x10bfd88;
 loop { match pc {
 0x10bfd88 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd8c); }
 let mut pc: u32 = 0x10bfd8c;
 loop { match pc {
 0x10bfd8c => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd90); }
 let mut pc: u32 = 0x10bfd90;
 loop { match pc {
 0x10bfd90 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd94); }
 let mut pc: u32 = 0x10bfd94;
 loop { match pc {
 0x10bfd94 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd98); }
 let mut pc: u32 = 0x10bfd98;
 loop { match pc {
 0x10bfd98 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfd9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfd9c); }
 let mut pc: u32 = 0x10bfd9c;
 loop { match pc {
 0x10bfd9c => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfda0); }
 let mut pc: u32 = 0x10bfda0;
 loop { match pc {
 0x10bfda0 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfda4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfda4); }
 let mut pc: u32 = 0x10bfda4;
 loop { match pc {
 0x10bfda4 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfda8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfda8); }
 let mut pc: u32 = 0x10bfda8;
 loop { match pc {
 0x10bfda8 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfdac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfdac); }
 let mut pc: u32 = 0x10bfdac;
 loop { match pc {
 0x10bfdac => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bfdb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bfdb0); }
 let mut pc: u32 = 0x10bfdb0;
 loop { match pc {
 0x10bfdb0 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
