#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_020bdcd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcd1); }
 let mut pc: u32 = 0x20bdcd1;
 loop { match pc {
 0x20bdcd1 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcd4); }
 let mut pc: u32 = 0x20bdcd4;
 loop { match pc {
 0x20bdcd4 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcd7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcd7); }
 let mut pc: u32 = 0x20bdcd7;
 loop { match pc {
 0x20bdcd7 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcda); }
 let mut pc: u32 = 0x20bdcda;
 loop { match pc {
 0x20bdcda => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcdd); }
 let mut pc: u32 = 0x20bdcdd;
 loop { match pc {
 0x20bdcdd => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdce0); }
 let mut pc: u32 = 0x20bdce0;
 loop { match pc {
 0x20bdce0 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdce3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdce3); }
 let mut pc: u32 = 0x20bdce3;
 loop { match pc {
 0x20bdce3 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdce6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdce6); }
 let mut pc: u32 = 0x20bdce6;
 loop { match pc {
 0x20bdce6 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdce9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdce9); }
 let mut pc: u32 = 0x20bdce9;
 loop { match pc {
 0x20bdce9 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcec); }
 let mut pc: u32 = 0x20bdcec;
 loop { match pc {
 0x20bdcec => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcef); }
 let mut pc: u32 = 0x20bdcef;
 loop { match pc {
 0x20bdcef => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcf2); }
 let mut pc: u32 = 0x20bdcf2;
 loop { match pc {
 0x20bdcf2 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcf5); }
 let mut pc: u32 = 0x20bdcf5;
 loop { match pc {
 0x20bdcf5 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcf8); }
 let mut pc: u32 = 0x20bdcf8;
 loop { match pc {
 0x20bdcf8 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcfb); }
 let mut pc: u32 = 0x20bdcfb;
 loop { match pc {
 0x20bdcfb => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcfe); }
 let mut pc: u32 = 0x20bdcfe;
 loop { match pc {
 0x20bdcfe => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd01); }
 let mut pc: u32 = 0x20bdd01;
 loop { match pc {
 0x20bdd01 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd04); }
 let mut pc: u32 = 0x20bdd04;
 loop { match pc {
 0x20bdd04 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd07); }
 let mut pc: u32 = 0x20bdd07;
 loop { match pc {
 0x20bdd07 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd0a); }
 let mut pc: u32 = 0x20bdd0a;
 loop { match pc {
 0x20bdd0a => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd0d); }
 let mut pc: u32 = 0x20bdd0d;
 loop { match pc {
 0x20bdd0d => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd10); }
 let mut pc: u32 = 0x20bdd10;
 loop { match pc {
 0x20bdd10 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd13); }
 let mut pc: u32 = 0x20bdd13;
 loop { match pc {
 0x20bdd13 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd16); }
 let mut pc: u32 = 0x20bdd16;
 loop { match pc {
 0x20bdd16 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd19); }
 let mut pc: u32 = 0x20bdd19;
 loop { match pc {
 0x20bdd19 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd1c); }
 let mut pc: u32 = 0x20bdd1c;
 loop { match pc {
 0x20bdd1c => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd1f); }
 let mut pc: u32 = 0x20bdd1f;
 loop { match pc {
 0x20bdd1f => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd22); }
 let mut pc: u32 = 0x20bdd22;
 loop { match pc {
 0x20bdd22 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd25); }
 let mut pc: u32 = 0x20bdd25;
 loop { match pc {
 0x20bdd25 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd28); }
 let mut pc: u32 = 0x20bdd28;
 loop { match pc {
 0x20bdd28 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd2b); }
 let mut pc: u32 = 0x20bdd2b;
 loop { match pc {
 0x20bdd2b => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd2e); }
 let mut pc: u32 = 0x20bdd2e;
 loop { match pc {
 0x20bdd2e => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd31); }
 let mut pc: u32 = 0x20bdd31;
 loop { match pc {
 0x20bdd31 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd34); }
 let mut pc: u32 = 0x20bdd34;
 loop { match pc {
 0x20bdd34 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd37); }
 let mut pc: u32 = 0x20bdd37;
 loop { match pc {
 0x20bdd37 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd3a); }
 let mut pc: u32 = 0x20bdd3a;
 loop { match pc {
 0x20bdd3a => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd3d); }
 let mut pc: u32 = 0x20bdd3d;
 loop { match pc {
 0x20bdd3d => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd40); }
 let mut pc: u32 = 0x20bdd40;
 loop { match pc {
 0x20bdd40 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd43); }
 let mut pc: u32 = 0x20bdd43;
 loop { match pc {
 0x20bdd43 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd46); }
 let mut pc: u32 = 0x20bdd46;
 loop { match pc {
 0x20bdd46 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd49); }
 let mut pc: u32 = 0x20bdd49;
 loop { match pc {
 0x20bdd49 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd4c); }
 let mut pc: u32 = 0x20bdd4c;
 loop { match pc {
 0x20bdd4c => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd4f); }
 let mut pc: u32 = 0x20bdd4f;
 loop { match pc {
 0x20bdd4f => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd52); }
 let mut pc: u32 = 0x20bdd52;
 loop { match pc {
 0x20bdd52 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd55); }
 let mut pc: u32 = 0x20bdd55;
 loop { match pc {
 0x20bdd55 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd58); }
 let mut pc: u32 = 0x20bdd58;
 loop { match pc {
 0x20bdd58 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd5b); }
 let mut pc: u32 = 0x20bdd5b;
 loop { match pc {
 0x20bdd5b => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd5e); }
 let mut pc: u32 = 0x20bdd5e;
 loop { match pc {
 0x20bdd5e => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd61); }
 let mut pc: u32 = 0x20bdd61;
 loop { match pc {
 0x20bdd61 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd64); }
 let mut pc: u32 = 0x20bdd64;
 loop { match pc {
 0x20bdd64 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd67); }
 let mut pc: u32 = 0x20bdd67;
 loop { match pc {
 0x20bdd67 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd6a); }
 let mut pc: u32 = 0x20bdd6a;
 loop { match pc {
 0x20bdd6a => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd6d); }
 let mut pc: u32 = 0x20bdd6d;
 loop { match pc {
 0x20bdd6d => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd70); }
 let mut pc: u32 = 0x20bdd70;
 loop { match pc {
 0x20bdd70 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd73); }
 let mut pc: u32 = 0x20bdd73;
 loop { match pc {
 0x20bdd73 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd76); }
 let mut pc: u32 = 0x20bdd76;
 loop { match pc {
 0x20bdd76 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd79); }
 let mut pc: u32 = 0x20bdd79;
 loop { match pc {
 0x20bdd79 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd7c); }
 let mut pc: u32 = 0x20bdd7c;
 loop { match pc {
 0x20bdd7c => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd7f); }
 let mut pc: u32 = 0x20bdd7f;
 loop { match pc {
 0x20bdd7f => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd82); }
 let mut pc: u32 = 0x20bdd82;
 loop { match pc {
 0x20bdd82 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd85); }
 let mut pc: u32 = 0x20bdd85;
 loop { match pc {
 0x20bdd85 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd88); }
 let mut pc: u32 = 0x20bdd88;
 loop { match pc {
 0x20bdd88 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd8b); }
 let mut pc: u32 = 0x20bdd8b;
 loop { match pc {
 0x20bdd8b => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd8e); }
 let mut pc: u32 = 0x20bdd8e;
 loop { match pc {
 0x20bdd8e => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd91); }
 let mut pc: u32 = 0x20bdd91;
 loop { match pc {
 0x20bdd91 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd94); }
 let mut pc: u32 = 0x20bdd94;
 loop { match pc {
 0x20bdd94 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd97); }
 let mut pc: u32 = 0x20bdd97;
 loop { match pc {
 0x20bdd97 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd9a); }
 let mut pc: u32 = 0x20bdd9a;
 loop { match pc {
 0x20bdd9a => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdd9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdd9d); }
 let mut pc: u32 = 0x20bdd9d;
 loop { match pc {
 0x20bdd9d => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdda0); }
 let mut pc: u32 = 0x20bdda0;
 loop { match pc {
 0x20bdda0 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdda3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdda3); }
 let mut pc: u32 = 0x20bdda3;
 loop { match pc {
 0x20bdda3 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdda6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdda6); }
 let mut pc: u32 = 0x20bdda6;
 loop { match pc {
 0x20bdda6 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdda9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdda9); }
 let mut pc: u32 = 0x20bdda9;
 loop { match pc {
 0x20bdda9 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddac); }
 let mut pc: u32 = 0x20bddac;
 loop { match pc {
 0x20bddac => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddaf); }
 let mut pc: u32 = 0x20bddaf;
 loop { match pc {
 0x20bddaf => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddb2); }
 let mut pc: u32 = 0x20bddb2;
 loop { match pc {
 0x20bddb2 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddb5); }
 let mut pc: u32 = 0x20bddb5;
 loop { match pc {
 0x20bddb5 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddb8); }
 let mut pc: u32 = 0x20bddb8;
 loop { match pc {
 0x20bddb8 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddbb); }
 let mut pc: u32 = 0x20bddbb;
 loop { match pc {
 0x20bddbb => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddbe); }
 let mut pc: u32 = 0x20bddbe;
 loop { match pc {
 0x20bddbe => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddc1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddc1); }
 let mut pc: u32 = 0x20bddc1;
 loop { match pc {
 0x20bddc1 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddc4); }
 let mut pc: u32 = 0x20bddc4;
 loop { match pc {
 0x20bddc4 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddc7); }
 let mut pc: u32 = 0x20bddc7;
 loop { match pc {
 0x20bddc7 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddca); }
 let mut pc: u32 = 0x20bddca;
 loop { match pc {
 0x20bddca => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddcd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddcd); }
 let mut pc: u32 = 0x20bddcd;
 loop { match pc {
 0x20bddcd => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddd0); }
 let mut pc: u32 = 0x20bddd0;
 loop { match pc {
 0x20bddd0 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddd3); }
 let mut pc: u32 = 0x20bddd3;
 loop { match pc {
 0x20bddd3 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddd6); }
 let mut pc: u32 = 0x20bddd6;
 loop { match pc {
 0x20bddd6 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddd9); }
 let mut pc: u32 = 0x20bddd9;
 loop { match pc {
 0x20bddd9 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdddc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdddc); }
 let mut pc: u32 = 0x20bdddc;
 loop { match pc {
 0x20bdddc => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdddf); }
 let mut pc: u32 = 0x20bdddf;
 loop { match pc {
 0x20bdddf => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdde2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdde2); }
 let mut pc: u32 = 0x20bdde2;
 loop { match pc {
 0x20bdde2 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdde5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdde5); }
 let mut pc: u32 = 0x20bdde5;
 loop { match pc {
 0x20bdde5 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdde8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdde8); }
 let mut pc: u32 = 0x20bdde8;
 loop { match pc {
 0x20bdde8 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddeb); }
 let mut pc: u32 = 0x20bddeb;
 loop { match pc {
 0x20bddeb => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddee); }
 let mut pc: u32 = 0x20bddee;
 loop { match pc {
 0x20bddee => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddf1); }
 let mut pc: u32 = 0x20bddf1;
 loop { match pc {
 0x20bddf1 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddf4); }
 let mut pc: u32 = 0x20bddf4;
 loop { match pc {
 0x20bddf4 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddf7); }
 let mut pc: u32 = 0x20bddf7;
 loop { match pc {
 0x20bddf7 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddfa); }
 let mut pc: u32 = 0x20bddfa;
 loop { match pc {
 0x20bddfa => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bddfd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bddfd); }
 let mut pc: u32 = 0x20bddfd;
 loop { match pc {
 0x20bddfd => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde00); }
 let mut pc: u32 = 0x20bde00;
 loop { match pc {
 0x20bde00 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde03); }
 let mut pc: u32 = 0x20bde03;
 loop { match pc {
 0x20bde03 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde06); }
 let mut pc: u32 = 0x20bde06;
 loop { match pc {
 0x20bde06 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde09); }
 let mut pc: u32 = 0x20bde09;
 loop { match pc {
 0x20bde09 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde0c); }
 let mut pc: u32 = 0x20bde0c;
 loop { match pc {
 0x20bde0c => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde0f); }
 let mut pc: u32 = 0x20bde0f;
 loop { match pc {
 0x20bde0f => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde12); }
 let mut pc: u32 = 0x20bde12;
 loop { match pc {
 0x20bde12 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde15); }
 let mut pc: u32 = 0x20bde15;
 loop { match pc {
 0x20bde15 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde18); }
 let mut pc: u32 = 0x20bde18;
 loop { match pc {
 0x20bde18 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde1b); }
 let mut pc: u32 = 0x20bde1b;
 loop { match pc {
 0x20bde1b => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde1e); }
 let mut pc: u32 = 0x20bde1e;
 loop { match pc {
 0x20bde1e => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde21); }
 let mut pc: u32 = 0x20bde21;
 loop { match pc {
 0x20bde21 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde24); }
 let mut pc: u32 = 0x20bde24;
 loop { match pc {
 0x20bde24 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde27); }
 let mut pc: u32 = 0x20bde27;
 loop { match pc {
 0x20bde27 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde2a); }
 let mut pc: u32 = 0x20bde2a;
 loop { match pc {
 0x20bde2a => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde2d); }
 let mut pc: u32 = 0x20bde2d;
 loop { match pc {
 0x20bde2d => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde30); }
 let mut pc: u32 = 0x20bde30;
 loop { match pc {
 0x20bde30 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde33); }
 let mut pc: u32 = 0x20bde33;
 loop { match pc {
 0x20bde33 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde36); }
 let mut pc: u32 = 0x20bde36;
 loop { match pc {
 0x20bde36 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde39); }
 let mut pc: u32 = 0x20bde39;
 loop { match pc {
 0x20bde39 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde3c); }
 let mut pc: u32 = 0x20bde3c;
 loop { match pc {
 0x20bde3c => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde3f); }
 let mut pc: u32 = 0x20bde3f;
 loop { match pc {
 0x20bde3f => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde42); }
 let mut pc: u32 = 0x20bde42;
 loop { match pc {
 0x20bde42 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde45); }
 let mut pc: u32 = 0x20bde45;
 loop { match pc {
 0x20bde45 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde48); }
 let mut pc: u32 = 0x20bde48;
 loop { match pc {
 0x20bde48 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde4b); }
 let mut pc: u32 = 0x20bde4b;
 loop { match pc {
 0x20bde4b => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde4e); }
 let mut pc: u32 = 0x20bde4e;
 loop { match pc {
 0x20bde4e => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde51); }
 let mut pc: u32 = 0x20bde51;
 loop { match pc {
 0x20bde51 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde54); }
 let mut pc: u32 = 0x20bde54;
 loop { match pc {
 0x20bde54 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde57); }
 let mut pc: u32 = 0x20bde57;
 loop { match pc {
 0x20bde57 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde5a); }
 let mut pc: u32 = 0x20bde5a;
 loop { match pc {
 0x20bde5a => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde5d); }
 let mut pc: u32 = 0x20bde5d;
 loop { match pc {
 0x20bde5d => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde60); }
 let mut pc: u32 = 0x20bde60;
 loop { match pc {
 0x20bde60 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde63); }
 let mut pc: u32 = 0x20bde63;
 loop { match pc {
 0x20bde63 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde66); }
 let mut pc: u32 = 0x20bde66;
 loop { match pc {
 0x20bde66 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde69); }
 let mut pc: u32 = 0x20bde69;
 loop { match pc {
 0x20bde69 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde6c); }
 let mut pc: u32 = 0x20bde6c;
 loop { match pc {
 0x20bde6c => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde6f); }
 let mut pc: u32 = 0x20bde6f;
 loop { match pc {
 0x20bde6f => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde72); }
 let mut pc: u32 = 0x20bde72;
 loop { match pc {
 0x20bde72 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde75); }
 let mut pc: u32 = 0x20bde75;
 loop { match pc {
 0x20bde75 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde78); }
 let mut pc: u32 = 0x20bde78;
 loop { match pc {
 0x20bde78 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde7b); }
 let mut pc: u32 = 0x20bde7b;
 loop { match pc {
 0x20bde7b => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde7e); }
 let mut pc: u32 = 0x20bde7e;
 loop { match pc {
 0x20bde7e => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde81); }
 let mut pc: u32 = 0x20bde81;
 loop { match pc {
 0x20bde81 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde84); }
 let mut pc: u32 = 0x20bde84;
 loop { match pc {
 0x20bde84 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde87); }
 let mut pc: u32 = 0x20bde87;
 loop { match pc {
 0x20bde87 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde8a); }
 let mut pc: u32 = 0x20bde8a;
 loop { match pc {
 0x20bde8a => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde8d); }
 let mut pc: u32 = 0x20bde8d;
 loop { match pc {
 0x20bde8d => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde90); }
 let mut pc: u32 = 0x20bde90;
 loop { match pc {
 0x20bde90 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde93); }
 let mut pc: u32 = 0x20bde93;
 loop { match pc {
 0x20bde93 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde96); }
 let mut pc: u32 = 0x20bde96;
 loop { match pc {
 0x20bde96 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde99); }
 let mut pc: u32 = 0x20bde99;
 loop { match pc {
 0x20bde99 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde9c); }
 let mut pc: u32 = 0x20bde9c;
 loop { match pc {
 0x20bde9c => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bde9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bde9f); }
 let mut pc: u32 = 0x20bde9f;
 loop { match pc {
 0x20bde9f => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdea2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdea2); }
 let mut pc: u32 = 0x20bdea2;
 loop { match pc {
 0x20bdea2 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdea5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdea5); }
 let mut pc: u32 = 0x20bdea5;
 loop { match pc {
 0x20bdea5 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdea8); }
 let mut pc: u32 = 0x20bdea8;
 loop { match pc {
 0x20bdea8 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeab); }
 let mut pc: u32 = 0x20bdeab;
 loop { match pc {
 0x20bdeab => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeae); }
 let mut pc: u32 = 0x20bdeae;
 loop { match pc {
 0x20bdeae => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeb1); }
 let mut pc: u32 = 0x20bdeb1;
 loop { match pc {
 0x20bdeb1 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeb4); }
 let mut pc: u32 = 0x20bdeb4;
 loop { match pc {
 0x20bdeb4 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeb7); }
 let mut pc: u32 = 0x20bdeb7;
 loop { match pc {
 0x20bdeb7 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeba); }
 let mut pc: u32 = 0x20bdeba;
 loop { match pc {
 0x20bdeba => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdebd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdebd); }
 let mut pc: u32 = 0x20bdebd;
 loop { match pc {
 0x20bdebd => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdec0); }
 let mut pc: u32 = 0x20bdec0;
 loop { match pc {
 0x20bdec0 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdec3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdec3); }
 let mut pc: u32 = 0x20bdec3;
 loop { match pc {
 0x20bdec3 => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdec6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdec6); }
 let mut pc: u32 = 0x20bdec6;
 loop { match pc {
 0x20bdec6 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdec9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdec9); }
 let mut pc: u32 = 0x20bdec9;
 loop { match pc {
 0x20bdec9 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdecc); }
 let mut pc: u32 = 0x20bdecc;
 loop { match pc {
 0x20bdecc => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdecf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdecf); }
 let mut pc: u32 = 0x20bdecf;
 loop { match pc {
 0x20bdecf => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bded2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bded2); }
 let mut pc: u32 = 0x20bded2;
 loop { match pc {
 0x20bded2 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bded5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bded5); }
 let mut pc: u32 = 0x20bded5;
 loop { match pc {
 0x20bded5 => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bded8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bded8); }
 let mut pc: u32 = 0x20bded8;
 loop { match pc {
 0x20bded8 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdedb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdedb); }
 let mut pc: u32 = 0x20bdedb;
 loop { match pc {
 0x20bdedb => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdede(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdede); }
 let mut pc: u32 = 0x20bdede;
 loop { match pc {
 0x20bdede => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdee1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdee1); }
 let mut pc: u32 = 0x20bdee1;
 loop { match pc {
 0x20bdee1 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdee4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdee4); }
 let mut pc: u32 = 0x20bdee4;
 loop { match pc {
 0x20bdee4 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdee7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdee7); }
 let mut pc: u32 = 0x20bdee7;
 loop { match pc {
 0x20bdee7 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeea); }
 let mut pc: u32 = 0x20bdeea;
 loop { match pc {
 0x20bdeea => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeed); }
 let mut pc: u32 = 0x20bdeed;
 loop { match pc {
 0x20bdeed => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdef0); }
 let mut pc: u32 = 0x20bdef0;
 loop { match pc {
 0x20bdef0 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdef3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdef3); }
 let mut pc: u32 = 0x20bdef3;
 loop { match pc {
 0x20bdef3 => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdef6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdef6); }
 let mut pc: u32 = 0x20bdef6;
 loop { match pc {
 0x20bdef6 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdef9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdef9); }
 let mut pc: u32 = 0x20bdef9;
 loop { match pc {
 0x20bdef9 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdefc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdefc); }
 let mut pc: u32 = 0x20bdefc;
 loop { match pc {
 0x20bdefc => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdeff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdeff); }
 let mut pc: u32 = 0x20bdeff;
 loop { match pc {
 0x20bdeff => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf02); }
 let mut pc: u32 = 0x20bdf02;
 loop { match pc {
 0x20bdf02 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf05); }
 let mut pc: u32 = 0x20bdf05;
 loop { match pc {
 0x20bdf05 => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf08); }
 let mut pc: u32 = 0x20bdf08;
 loop { match pc {
 0x20bdf08 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf0b); }
 let mut pc: u32 = 0x20bdf0b;
 loop { match pc {
 0x20bdf0b => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf0e); }
 let mut pc: u32 = 0x20bdf0e;
 loop { match pc {
 0x20bdf0e => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf11); }
 let mut pc: u32 = 0x20bdf11;
 loop { match pc {
 0x20bdf11 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf14); }
 let mut pc: u32 = 0x20bdf14;
 loop { match pc {
 0x20bdf14 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf17); }
 let mut pc: u32 = 0x20bdf17;
 loop { match pc {
 0x20bdf17 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf1a); }
 let mut pc: u32 = 0x20bdf1a;
 loop { match pc {
 0x20bdf1a => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf1d); }
 let mut pc: u32 = 0x20bdf1d;
 loop { match pc {
 0x20bdf1d => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf20); }
 let mut pc: u32 = 0x20bdf20;
 loop { match pc {
 0x20bdf20 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf23); }
 let mut pc: u32 = 0x20bdf23;
 loop { match pc {
 0x20bdf23 => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf26); }
 let mut pc: u32 = 0x20bdf26;
 loop { match pc {
 0x20bdf26 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bdf2d); f_020be035(cpu); if cpu.returned_to != 0x20bdf2d { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x20bdf38; continue;
 }
 0x20bdf38 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x20bdf41; continue; }
  pc = 0x20bdf3d; continue;
 }
 0x20bdf3d => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x20bdf41; continue;
 }
 0x20bdf41 => {
  if cpu.cf { pc = 0x20bdf49; continue; }
  pc = 0x20bdf43; continue;
 }
 0x20bdf43 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20bdf38; continue;
 }
 0x20bdf49 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x20bdf4c; continue;
 }
 0x20bdf4c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x20bdf55; continue; }
  pc = 0x20bdf51; continue;
 }
 0x20bdf51 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x20bdf55; continue;
 }
 0x20bdf55 => {
  if !cpu.cf { pc = 0x20bdf5d; continue; }
  pc = 0x20bdf57; continue;
 }
 0x20bdf57 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20bdf4c; continue;
 }
 0x20bdf5d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdf60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdf60); }
 let mut pc: u32 = 0x20bdf60;
 loop { match pc {
 0x20bdf60 => {
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
  if cpu.zf { pc = 0x20bdfbe; continue; }
  pc = 0x20bdf89; continue;
 }
 0x20bdf89 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bdf94; continue; }
  pc = 0x20bdf8b; continue;
 }
 0x20bdf8b => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20bdf8b; continue; }
  pc = 0x20bdf92; continue;
 }
 0x20bdf92 => {
  pc = 0x20bdfbe; continue;
 }
 0x20bdf94 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x20bdf98; continue;
 }
 0x20bdf98 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20bdf98; continue; }
  pc = 0x20bdfa3; continue;
 }
 0x20bdfa3 => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x20bdfbe; continue; }
  pc = 0x20bdfaa; continue;
 }
 0x20bdfaa => {
  if !cpu.zf { pc = 0x20bdfb8; continue; }
  pc = 0x20bdfac; continue;
 }
 0x20bdfac => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x20bdfb8; continue; }
  pc = 0x20bdfb0; continue;
 }
 0x20bdfb0 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x20bdfbe; continue; }
  pc = 0x20bdfb8; continue;
 }
 0x20bdfb8 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x20bdfbe; continue;
 }
 0x20bdfbe => {
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
pub fn f_020bdfcb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdfcb); }
 let mut pc: u32 = 0x20bdfcb;
 loop { match pc {
 0x20bdfcb => {
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
  pc = 0x20bdfe0; continue;
 }
 0x20bdfe0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20bdff3; continue; }
  pc = 0x20bdfec; continue;
 }
 0x20bdfec => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x20bdff5; continue; }
  pc = 0x20bdff0; continue;
 }
 0x20bdff0 => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x20be00b; continue;
 }
 0x20bdff3 => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x20bdff5; continue;
 }
 0x20bdff5 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x20be004, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x20be009, "Aam");
  pc = 0x20be00b; continue;
 }
 0x20be00b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20be02f; continue; }
  pc = 0x20be01d; continue;
 }
 0x20be01d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20be02f; continue; }
  pc = 0x20be023; continue;
 }
 0x20be023 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20be02f; continue; }
  pc = 0x20be029; continue;
 }
 0x20be029 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20bdfe0; continue; }
  pc = 0x20be02f; continue;
 }
 0x20be02f => {
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
pub fn f_020be035(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be035); }
 let mut pc: u32 = 0x20be035;
 loop { match pc {
 0x20be035 => {
  cpu.push(0x20be03a); f_020be0ea(cpu); if cpu.returned_to != 0x20be03a { return; }
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
  cpu.unimplemented(0x20be067, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x20be078, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x20be07e; continue; }
  pc = 0x20be089; continue;
 }
 0x20be07e => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x20be07e; continue; }
  pc = 0x20be089; continue;
 }
 0x20be089 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x20be097); cpu.call(0x0); if cpu.returned_to != 0x20be097 { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20be0a4); cpu.call(0x2f4291a); if cpu.returned_to != 0x20be0a4 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x20be0a9, "Aam");
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
  cpu.unimplemented(0x20be0cc, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x20be0dc); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be0ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be0ea); }
 let mut pc: u32 = 0x20be0ea;
 loop { match pc {
 0x20be0ea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be0ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be0ec); }
 let mut pc: u32 = 0x20be0ec;
 loop { match pc {
 0x20be0ec => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x216b2f8u32; cpu.eax = v; }
  cpu.push(0x20be0fc); f_020bd78a(cpu); if cpu.returned_to != 0x20be0fc { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be0fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be0fe); } r_020be0fe(cpu, 0x20be0fe); }
pub fn f_020be188(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20be188); } r_020be0fe(cpu, 0x20be188); }
fn r_020be0fe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20be0fe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x20be104, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x20be109, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x20be18a; continue; }
  pc = 0x20be119; continue;
 }
 0x20be119 => {
  if cpu.cf { pc = 0x20be18e; continue; }
  pc = 0x20be11b; continue;
 }
 0x20be11b => {
  if cpu.zf { pc = 0x20be192; continue; }
  pc = 0x20be11d; continue;
 }
 0x20be11d => {
  if (cpu.cf || cpu.zf) { pc = 0x20be196; continue; }
  pc = 0x20be11f; continue;
 }
 0x20be11f => {
  if cpu.sf { pc = 0x20be19a; continue; }
  pc = 0x20be121; continue;
 }
 0x20be121 => {
  if cpu.pf() { f_020be123(cpu); return; }
  f_020be123(cpu); return;
 }
 0x20be188 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x20be1d0; continue; }
  pc = 0x20be191; continue;
 }
 0x20be18a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20be18e => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_020be204(cpu); return; }
  pc = 0x20be190; continue;
 }
 0x20be190 => {
  cpu.unimplemented(0x20be190, "Aas");
  pc = 0x20be191; continue;
 }
 0x20be191 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x20be1c7; continue; }
  pc = 0x20be1a3; continue;
 }
 0x20be192 => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x20be1c7; continue; }
  pc = 0x20be1a3; continue;
 }
 0x20be196 => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x20be1c7; continue; }
  pc = 0x20be1a3; continue;
 }
 0x20be19a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x20be1c7; continue; }
  pc = 0x20be1a3; continue;
 }
 0x20be1a3 => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x20be1c7; continue; }
  pc = 0x20be1ab; continue;
 }
 0x20be1ab => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x20be1d0; continue; }
  pc = 0x20be1b3; continue;
 }
 0x20be1b3 => {
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
 0x20be1c7 => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20be1d0 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be123(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be123); }
 let mut pc: u32 = 0x20be123;
 loop { match pc {
 0x20be123 => {
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
  pc = 0x20be139; continue;
 }
 0x20be139 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x20be0feu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20be139; continue; }
  pc = 0x20be159; continue;
 }
 0x20be159 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20be159; continue; }
  pc = 0x20be163; continue;
 }
 0x20be163 => {
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
pub fn f_020be16d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be16d); }
 let mut pc: u32 = 0x20be16d;
 loop { match pc {
 0x20be16d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x20be17f; continue; }
  pc = 0x20be175; continue;
 }
 0x20be175 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20be17f; continue; }
  pc = 0x20be179; continue;
 }
 0x20be179 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20be17f; continue;
 }
 0x20be17f => {
  cpu.push(0x20be184); f_020be123(cpu); if cpu.returned_to != 0x20be184 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be1d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be1d4); }
 let mut pc: u32 = 0x20be1d4;
 loop { match pc {
 0x20be1d4 => {
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
pub fn f_020be1f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be1f8); }
 let mut pc: u32 = 0x20be1f8;
 loop { match pc {
 0x20be1f8 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be1fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be1fc); }
 let mut pc: u32 = 0x20be1fc;
 loop { match pc {
 0x20be1fc => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be200); }
 let mut pc: u32 = 0x20be200;
 loop { match pc {
 0x20be200 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be204); }
 let mut pc: u32 = 0x20be204;
 loop { match pc {
 0x20be204 => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be208(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be208); }
 let mut pc: u32 = 0x20be208;
 loop { match pc {
 0x20be208 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be20c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be20c); }
 let mut pc: u32 = 0x20be20c;
 loop { match pc {
 0x20be20c => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be210); }
 let mut pc: u32 = 0x20be210;
 loop { match pc {
 0x20be210 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be214(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be214); }
 let mut pc: u32 = 0x20be214;
 loop { match pc {
 0x20be214 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be218(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be218); }
 let mut pc: u32 = 0x20be218;
 loop { match pc {
 0x20be218 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be21c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be21c); }
 let mut pc: u32 = 0x20be21c;
 loop { match pc {
 0x20be21c => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be220); }
 let mut pc: u32 = 0x20be220;
 loop { match pc {
 0x20be220 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be224); }
 let mut pc: u32 = 0x20be224;
 loop { match pc {
 0x20be224 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be228); }
 let mut pc: u32 = 0x20be228;
 loop { match pc {
 0x20be228 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be22c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be22c); }
 let mut pc: u32 = 0x20be22c;
 loop { match pc {
 0x20be22c => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be230); }
 let mut pc: u32 = 0x20be230;
 loop { match pc {
 0x20be230 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be234(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be234); }
 let mut pc: u32 = 0x20be234;
 loop { match pc {
 0x20be234 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be238); }
 let mut pc: u32 = 0x20be238;
 loop { match pc {
 0x20be238 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be23c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be23c); }
 let mut pc: u32 = 0x20be23c;
 loop { match pc {
 0x20be23c => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be240); }
 let mut pc: u32 = 0x20be240;
 loop { match pc {
 0x20be240 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be244); }
 let mut pc: u32 = 0x20be244;
 loop { match pc {
 0x20be244 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be248(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be248); }
 let mut pc: u32 = 0x20be248;
 loop { match pc {
 0x20be248 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be24c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be24c); }
 let mut pc: u32 = 0x20be24c;
 loop { match pc {
 0x20be24c => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be250); }
 let mut pc: u32 = 0x20be250;
 loop { match pc {
 0x20be250 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be254(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be254); }
 let mut pc: u32 = 0x20be254;
 loop { match pc {
 0x20be254 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be258(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be258); }
 let mut pc: u32 = 0x20be258;
 loop { match pc {
 0x20be258 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be25c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be25c); }
 let mut pc: u32 = 0x20be25c;
 loop { match pc {
 0x20be25c => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be260); }
 let mut pc: u32 = 0x20be260;
 loop { match pc {
 0x20be260 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be264(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be264); }
 let mut pc: u32 = 0x20be264;
 loop { match pc {
 0x20be264 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be268); }
 let mut pc: u32 = 0x20be268;
 loop { match pc {
 0x20be268 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be26c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be26c); }
 let mut pc: u32 = 0x20be26c;
 loop { match pc {
 0x20be26c => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be270); }
 let mut pc: u32 = 0x20be270;
 loop { match pc {
 0x20be270 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be274(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be274); }
 let mut pc: u32 = 0x20be274;
 loop { match pc {
 0x20be274 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be278(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be278); }
 let mut pc: u32 = 0x20be278;
 loop { match pc {
 0x20be278 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be27c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be27c); }
 let mut pc: u32 = 0x20be27c;
 loop { match pc {
 0x20be27c => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be280); }
 let mut pc: u32 = 0x20be280;
 loop { match pc {
 0x20be280 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be284(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be284); }
 let mut pc: u32 = 0x20be284;
 loop { match pc {
 0x20be284 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be288(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be288); }
 let mut pc: u32 = 0x20be288;
 loop { match pc {
 0x20be288 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be28c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be28c); }
 let mut pc: u32 = 0x20be28c;
 loop { match pc {
 0x20be28c => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be290); }
 let mut pc: u32 = 0x20be290;
 loop { match pc {
 0x20be290 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be294(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be294); }
 let mut pc: u32 = 0x20be294;
 loop { match pc {
 0x20be294 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be298); }
 let mut pc: u32 = 0x20be298;
 loop { match pc {
 0x20be298 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be29c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be29c); }
 let mut pc: u32 = 0x20be29c;
 loop { match pc {
 0x20be29c => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2a0); }
 let mut pc: u32 = 0x20be2a0;
 loop { match pc {
 0x20be2a0 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2a4); }
 let mut pc: u32 = 0x20be2a4;
 loop { match pc {
 0x20be2a4 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2a8); }
 let mut pc: u32 = 0x20be2a8;
 loop { match pc {
 0x20be2a8 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2ac); }
 let mut pc: u32 = 0x20be2ac;
 loop { match pc {
 0x20be2ac => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2b0); }
 let mut pc: u32 = 0x20be2b0;
 loop { match pc {
 0x20be2b0 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2b4); }
 let mut pc: u32 = 0x20be2b4;
 loop { match pc {
 0x20be2b4 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2b8); }
 let mut pc: u32 = 0x20be2b8;
 loop { match pc {
 0x20be2b8 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2bc); }
 let mut pc: u32 = 0x20be2bc;
 loop { match pc {
 0x20be2bc => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2c0); }
 let mut pc: u32 = 0x20be2c0;
 loop { match pc {
 0x20be2c0 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2c4); }
 let mut pc: u32 = 0x20be2c4;
 loop { match pc {
 0x20be2c4 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2c8); }
 let mut pc: u32 = 0x20be2c8;
 loop { match pc {
 0x20be2c8 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2cc); }
 let mut pc: u32 = 0x20be2cc;
 loop { match pc {
 0x20be2cc => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2d0); }
 let mut pc: u32 = 0x20be2d0;
 loop { match pc {
 0x20be2d0 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2d4); }
 let mut pc: u32 = 0x20be2d4;
 loop { match pc {
 0x20be2d4 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2d8); }
 let mut pc: u32 = 0x20be2d8;
 loop { match pc {
 0x20be2d8 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2dc); }
 let mut pc: u32 = 0x20be2dc;
 loop { match pc {
 0x20be2dc => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2e0); }
 let mut pc: u32 = 0x20be2e0;
 loop { match pc {
 0x20be2e0 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2e4); }
 let mut pc: u32 = 0x20be2e4;
 loop { match pc {
 0x20be2e4 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2e8); }
 let mut pc: u32 = 0x20be2e8;
 loop { match pc {
 0x20be2e8 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2ec); }
 let mut pc: u32 = 0x20be2ec;
 loop { match pc {
 0x20be2ec => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2f0); }
 let mut pc: u32 = 0x20be2f0;
 loop { match pc {
 0x20be2f0 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2f4); }
 let mut pc: u32 = 0x20be2f4;
 loop { match pc {
 0x20be2f4 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2f8); }
 let mut pc: u32 = 0x20be2f8;
 loop { match pc {
 0x20be2f8 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be2fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be2fc); }
 let mut pc: u32 = 0x20be2fc;
 loop { match pc {
 0x20be2fc => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be300); }
 let mut pc: u32 = 0x20be300;
 loop { match pc {
 0x20be300 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be304(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be304); }
 let mut pc: u32 = 0x20be304;
 loop { match pc {
 0x20be304 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be308); }
 let mut pc: u32 = 0x20be308;
 loop { match pc {
 0x20be308 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be30c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be30c); }
 let mut pc: u32 = 0x20be30c;
 loop { match pc {
 0x20be30c => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be310); }
 let mut pc: u32 = 0x20be310;
 loop { match pc {
 0x20be310 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be314(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be314); }
 let mut pc: u32 = 0x20be314;
 loop { match pc {
 0x20be314 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be318(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be318); }
 let mut pc: u32 = 0x20be318;
 loop { match pc {
 0x20be318 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be31c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be31c); }
 let mut pc: u32 = 0x20be31c;
 loop { match pc {
 0x20be31c => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be320); }
 let mut pc: u32 = 0x20be320;
 loop { match pc {
 0x20be320 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be324(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be324); }
 let mut pc: u32 = 0x20be324;
 loop { match pc {
 0x20be324 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be328(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be328); }
 let mut pc: u32 = 0x20be328;
 loop { match pc {
 0x20be328 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be32c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be32c); }
 let mut pc: u32 = 0x20be32c;
 loop { match pc {
 0x20be32c => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be330); }
 let mut pc: u32 = 0x20be330;
 loop { match pc {
 0x20be330 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be334(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be334); }
 let mut pc: u32 = 0x20be334;
 loop { match pc {
 0x20be334 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be338); }
 let mut pc: u32 = 0x20be338;
 loop { match pc {
 0x20be338 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be33c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be33c); }
 let mut pc: u32 = 0x20be33c;
 loop { match pc {
 0x20be33c => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be340); }
 let mut pc: u32 = 0x20be340;
 loop { match pc {
 0x20be340 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be344(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be344); }
 let mut pc: u32 = 0x20be344;
 loop { match pc {
 0x20be344 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be348(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be348); }
 let mut pc: u32 = 0x20be348;
 loop { match pc {
 0x20be348 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be34c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be34c); }
 let mut pc: u32 = 0x20be34c;
 loop { match pc {
 0x20be34c => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be350); }
 let mut pc: u32 = 0x20be350;
 loop { match pc {
 0x20be350 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be354); }
 let mut pc: u32 = 0x20be354;
 loop { match pc {
 0x20be354 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be358(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be358); }
 let mut pc: u32 = 0x20be358;
 loop { match pc {
 0x20be358 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020be35c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20be35c); }
 let mut pc: u32 = 0x20be35c;
 loop { match pc {
 0x20be35c => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
