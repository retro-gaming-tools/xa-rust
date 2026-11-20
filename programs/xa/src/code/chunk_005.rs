#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_030ae0a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0a6); }
 let mut pc: u32 = 0x30ae0a6;
 loop { match pc {
 0x30ae0a6 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0a9); }
 let mut pc: u32 = 0x30ae0a9;
 loop { match pc {
 0x30ae0a9 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0ac); }
 let mut pc: u32 = 0x30ae0ac;
 loop { match pc {
 0x30ae0ac => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0af); }
 let mut pc: u32 = 0x30ae0af;
 loop { match pc {
 0x30ae0af => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0b2); }
 let mut pc: u32 = 0x30ae0b2;
 loop { match pc {
 0x30ae0b2 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0b5); }
 let mut pc: u32 = 0x30ae0b5;
 loop { match pc {
 0x30ae0b5 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0b8); }
 let mut pc: u32 = 0x30ae0b8;
 loop { match pc {
 0x30ae0b8 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0bb); }
 let mut pc: u32 = 0x30ae0bb;
 loop { match pc {
 0x30ae0bb => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0be); }
 let mut pc: u32 = 0x30ae0be;
 loop { match pc {
 0x30ae0be => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0c1); }
 let mut pc: u32 = 0x30ae0c1;
 loop { match pc {
 0x30ae0c1 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0c4); }
 let mut pc: u32 = 0x30ae0c4;
 loop { match pc {
 0x30ae0c4 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0c7); }
 let mut pc: u32 = 0x30ae0c7;
 loop { match pc {
 0x30ae0c7 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0ca); }
 let mut pc: u32 = 0x30ae0ca;
 loop { match pc {
 0x30ae0ca => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0cd); }
 let mut pc: u32 = 0x30ae0cd;
 loop { match pc {
 0x30ae0cd => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0d0); }
 let mut pc: u32 = 0x30ae0d0;
 loop { match pc {
 0x30ae0d0 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0d3); }
 let mut pc: u32 = 0x30ae0d3;
 loop { match pc {
 0x30ae0d3 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0d6); }
 let mut pc: u32 = 0x30ae0d6;
 loop { match pc {
 0x30ae0d6 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0d9); }
 let mut pc: u32 = 0x30ae0d9;
 loop { match pc {
 0x30ae0d9 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0dc); }
 let mut pc: u32 = 0x30ae0dc;
 loop { match pc {
 0x30ae0dc => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0df); }
 let mut pc: u32 = 0x30ae0df;
 loop { match pc {
 0x30ae0df => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0e2); }
 let mut pc: u32 = 0x30ae0e2;
 loop { match pc {
 0x30ae0e2 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0e5); }
 let mut pc: u32 = 0x30ae0e5;
 loop { match pc {
 0x30ae0e5 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0e8); }
 let mut pc: u32 = 0x30ae0e8;
 loop { match pc {
 0x30ae0e8 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0eb); }
 let mut pc: u32 = 0x30ae0eb;
 loop { match pc {
 0x30ae0eb => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0ee); }
 let mut pc: u32 = 0x30ae0ee;
 loop { match pc {
 0x30ae0ee => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0f1); }
 let mut pc: u32 = 0x30ae0f1;
 loop { match pc {
 0x30ae0f1 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0f4); }
 let mut pc: u32 = 0x30ae0f4;
 loop { match pc {
 0x30ae0f4 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0f7); }
 let mut pc: u32 = 0x30ae0f7;
 loop { match pc {
 0x30ae0f7 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0fa); }
 let mut pc: u32 = 0x30ae0fa;
 loop { match pc {
 0x30ae0fa => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae0fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae0fd); }
 let mut pc: u32 = 0x30ae0fd;
 loop { match pc {
 0x30ae0fd => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae100); }
 let mut pc: u32 = 0x30ae100;
 loop { match pc {
 0x30ae100 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae103(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae103); }
 let mut pc: u32 = 0x30ae103;
 loop { match pc {
 0x30ae103 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae106(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae106); }
 let mut pc: u32 = 0x30ae106;
 loop { match pc {
 0x30ae106 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae109(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae109); }
 let mut pc: u32 = 0x30ae109;
 loop { match pc {
 0x30ae109 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae10c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae10c); }
 let mut pc: u32 = 0x30ae10c;
 loop { match pc {
 0x30ae10c => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae10f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae10f); }
 let mut pc: u32 = 0x30ae10f;
 loop { match pc {
 0x30ae10f => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae112(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae112); }
 let mut pc: u32 = 0x30ae112;
 loop { match pc {
 0x30ae112 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae115); }
 let mut pc: u32 = 0x30ae115;
 loop { match pc {
 0x30ae115 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae118(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae118); }
 let mut pc: u32 = 0x30ae118;
 loop { match pc {
 0x30ae118 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae11b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae11b); }
 let mut pc: u32 = 0x30ae11b;
 loop { match pc {
 0x30ae11b => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae11e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae11e); }
 let mut pc: u32 = 0x30ae11e;
 loop { match pc {
 0x30ae11e => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae121(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae121); }
 let mut pc: u32 = 0x30ae121;
 loop { match pc {
 0x30ae121 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae124); }
 let mut pc: u32 = 0x30ae124;
 loop { match pc {
 0x30ae124 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae127); }
 let mut pc: u32 = 0x30ae127;
 loop { match pc {
 0x30ae127 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae12a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae12a); }
 let mut pc: u32 = 0x30ae12a;
 loop { match pc {
 0x30ae12a => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae12d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae12d); }
 let mut pc: u32 = 0x30ae12d;
 loop { match pc {
 0x30ae12d => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae130); }
 let mut pc: u32 = 0x30ae130;
 loop { match pc {
 0x30ae130 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae133); }
 let mut pc: u32 = 0x30ae133;
 loop { match pc {
 0x30ae133 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae136(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae136); }
 let mut pc: u32 = 0x30ae136;
 loop { match pc {
 0x30ae136 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae139(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae139); }
 let mut pc: u32 = 0x30ae139;
 loop { match pc {
 0x30ae139 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae13c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae13c); }
 let mut pc: u32 = 0x30ae13c;
 loop { match pc {
 0x30ae13c => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae13f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae13f); }
 let mut pc: u32 = 0x30ae13f;
 loop { match pc {
 0x30ae13f => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae142(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae142); }
 let mut pc: u32 = 0x30ae142;
 loop { match pc {
 0x30ae142 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae145(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae145); }
 let mut pc: u32 = 0x30ae145;
 loop { match pc {
 0x30ae145 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae148(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae148); }
 let mut pc: u32 = 0x30ae148;
 loop { match pc {
 0x30ae148 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae14b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae14b); }
 let mut pc: u32 = 0x30ae14b;
 loop { match pc {
 0x30ae14b => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae14e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae14e); }
 let mut pc: u32 = 0x30ae14e;
 loop { match pc {
 0x30ae14e => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae151); }
 let mut pc: u32 = 0x30ae151;
 loop { match pc {
 0x30ae151 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae154); }
 let mut pc: u32 = 0x30ae154;
 loop { match pc {
 0x30ae154 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae157(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae157); }
 let mut pc: u32 = 0x30ae157;
 loop { match pc {
 0x30ae157 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae15a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae15a); }
 let mut pc: u32 = 0x30ae15a;
 loop { match pc {
 0x30ae15a => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae15d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae15d); }
 let mut pc: u32 = 0x30ae15d;
 loop { match pc {
 0x30ae15d => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae160); }
 let mut pc: u32 = 0x30ae160;
 loop { match pc {
 0x30ae160 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae163(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae163); }
 let mut pc: u32 = 0x30ae163;
 loop { match pc {
 0x30ae163 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae166(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae166); }
 let mut pc: u32 = 0x30ae166;
 loop { match pc {
 0x30ae166 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae169(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae169); }
 let mut pc: u32 = 0x30ae169;
 loop { match pc {
 0x30ae169 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae16c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae16c); }
 let mut pc: u32 = 0x30ae16c;
 loop { match pc {
 0x30ae16c => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae16f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae16f); }
 let mut pc: u32 = 0x30ae16f;
 loop { match pc {
 0x30ae16f => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae172(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae172); }
 let mut pc: u32 = 0x30ae172;
 loop { match pc {
 0x30ae172 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae175(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae175); }
 let mut pc: u32 = 0x30ae175;
 loop { match pc {
 0x30ae175 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae178); }
 let mut pc: u32 = 0x30ae178;
 loop { match pc {
 0x30ae178 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae17b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae17b); }
 let mut pc: u32 = 0x30ae17b;
 loop { match pc {
 0x30ae17b => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae17e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae17e); }
 let mut pc: u32 = 0x30ae17e;
 loop { match pc {
 0x30ae17e => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae181); }
 let mut pc: u32 = 0x30ae181;
 loop { match pc {
 0x30ae181 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae184(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae184); }
 let mut pc: u32 = 0x30ae184;
 loop { match pc {
 0x30ae184 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae187(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae187); }
 let mut pc: u32 = 0x30ae187;
 loop { match pc {
 0x30ae187 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae18a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae18a); }
 let mut pc: u32 = 0x30ae18a;
 loop { match pc {
 0x30ae18a => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae18d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae18d); }
 let mut pc: u32 = 0x30ae18d;
 loop { match pc {
 0x30ae18d => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae190); }
 let mut pc: u32 = 0x30ae190;
 loop { match pc {
 0x30ae190 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae193(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae193); }
 let mut pc: u32 = 0x30ae193;
 loop { match pc {
 0x30ae193 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae196(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae196); }
 let mut pc: u32 = 0x30ae196;
 loop { match pc {
 0x30ae196 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae199(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae199); }
 let mut pc: u32 = 0x30ae199;
 loop { match pc {
 0x30ae199 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae19c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae19c); }
 let mut pc: u32 = 0x30ae19c;
 loop { match pc {
 0x30ae19c => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae19f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae19f); }
 let mut pc: u32 = 0x30ae19f;
 loop { match pc {
 0x30ae19f => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1a2); }
 let mut pc: u32 = 0x30ae1a2;
 loop { match pc {
 0x30ae1a2 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1a5); }
 let mut pc: u32 = 0x30ae1a5;
 loop { match pc {
 0x30ae1a5 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1a8); }
 let mut pc: u32 = 0x30ae1a8;
 loop { match pc {
 0x30ae1a8 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ab); }
 let mut pc: u32 = 0x30ae1ab;
 loop { match pc {
 0x30ae1ab => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ae); }
 let mut pc: u32 = 0x30ae1ae;
 loop { match pc {
 0x30ae1ae => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1b1); }
 let mut pc: u32 = 0x30ae1b1;
 loop { match pc {
 0x30ae1b1 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1b4); }
 let mut pc: u32 = 0x30ae1b4;
 loop { match pc {
 0x30ae1b4 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1b7); }
 let mut pc: u32 = 0x30ae1b7;
 loop { match pc {
 0x30ae1b7 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ba); }
 let mut pc: u32 = 0x30ae1ba;
 loop { match pc {
 0x30ae1ba => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1bd); }
 let mut pc: u32 = 0x30ae1bd;
 loop { match pc {
 0x30ae1bd => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1c0); }
 let mut pc: u32 = 0x30ae1c0;
 loop { match pc {
 0x30ae1c0 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1c3); }
 let mut pc: u32 = 0x30ae1c3;
 loop { match pc {
 0x30ae1c3 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1c6); }
 let mut pc: u32 = 0x30ae1c6;
 loop { match pc {
 0x30ae1c6 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1c9); }
 let mut pc: u32 = 0x30ae1c9;
 loop { match pc {
 0x30ae1c9 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1cc); }
 let mut pc: u32 = 0x30ae1cc;
 loop { match pc {
 0x30ae1cc => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1cf); }
 let mut pc: u32 = 0x30ae1cf;
 loop { match pc {
 0x30ae1cf => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1d2); }
 let mut pc: u32 = 0x30ae1d2;
 loop { match pc {
 0x30ae1d2 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1d5); }
 let mut pc: u32 = 0x30ae1d5;
 loop { match pc {
 0x30ae1d5 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1d8); }
 let mut pc: u32 = 0x30ae1d8;
 loop { match pc {
 0x30ae1d8 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1db); }
 let mut pc: u32 = 0x30ae1db;
 loop { match pc {
 0x30ae1db => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1de); }
 let mut pc: u32 = 0x30ae1de;
 loop { match pc {
 0x30ae1de => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1e1); }
 let mut pc: u32 = 0x30ae1e1;
 loop { match pc {
 0x30ae1e1 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1e4); }
 let mut pc: u32 = 0x30ae1e4;
 loop { match pc {
 0x30ae1e4 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1e7); }
 let mut pc: u32 = 0x30ae1e7;
 loop { match pc {
 0x30ae1e7 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ea); }
 let mut pc: u32 = 0x30ae1ea;
 loop { match pc {
 0x30ae1ea => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ed); }
 let mut pc: u32 = 0x30ae1ed;
 loop { match pc {
 0x30ae1ed => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1f0); }
 let mut pc: u32 = 0x30ae1f0;
 loop { match pc {
 0x30ae1f0 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1f3); }
 let mut pc: u32 = 0x30ae1f3;
 loop { match pc {
 0x30ae1f3 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1f6); }
 let mut pc: u32 = 0x30ae1f6;
 loop { match pc {
 0x30ae1f6 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1f9); }
 let mut pc: u32 = 0x30ae1f9;
 loop { match pc {
 0x30ae1f9 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1fc); }
 let mut pc: u32 = 0x30ae1fc;
 loop { match pc {
 0x30ae1fc => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae1ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae1ff); }
 let mut pc: u32 = 0x30ae1ff;
 loop { match pc {
 0x30ae1ff => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae202(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae202); }
 let mut pc: u32 = 0x30ae202;
 loop { match pc {
 0x30ae202 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae205(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae205); }
 let mut pc: u32 = 0x30ae205;
 loop { match pc {
 0x30ae205 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae208(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae208); }
 let mut pc: u32 = 0x30ae208;
 loop { match pc {
 0x30ae208 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae20b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae20b); }
 let mut pc: u32 = 0x30ae20b;
 loop { match pc {
 0x30ae20b => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae20e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae20e); }
 let mut pc: u32 = 0x30ae20e;
 loop { match pc {
 0x30ae20e => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae211(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae211); }
 let mut pc: u32 = 0x30ae211;
 loop { match pc {
 0x30ae211 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae214(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae214); }
 let mut pc: u32 = 0x30ae214;
 loop { match pc {
 0x30ae214 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae217(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae217); }
 let mut pc: u32 = 0x30ae217;
 loop { match pc {
 0x30ae217 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae21a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae21a); }
 let mut pc: u32 = 0x30ae21a;
 loop { match pc {
 0x30ae21a => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae21d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae21d); }
 let mut pc: u32 = 0x30ae21d;
 loop { match pc {
 0x30ae21d => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae220); }
 let mut pc: u32 = 0x30ae220;
 loop { match pc {
 0x30ae220 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae223(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae223); }
 let mut pc: u32 = 0x30ae223;
 loop { match pc {
 0x30ae223 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae226); }
 let mut pc: u32 = 0x30ae226;
 loop { match pc {
 0x30ae226 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae229(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae229); }
 let mut pc: u32 = 0x30ae229;
 loop { match pc {
 0x30ae229 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae22c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae22c); }
 let mut pc: u32 = 0x30ae22c;
 loop { match pc {
 0x30ae22c => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae22f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae22f); }
 let mut pc: u32 = 0x30ae22f;
 loop { match pc {
 0x30ae22f => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae232); }
 let mut pc: u32 = 0x30ae232;
 loop { match pc {
 0x30ae232 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae235(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae235); }
 let mut pc: u32 = 0x30ae235;
 loop { match pc {
 0x30ae235 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae238(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae238); }
 let mut pc: u32 = 0x30ae238;
 loop { match pc {
 0x30ae238 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae23b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae23b); }
 let mut pc: u32 = 0x30ae23b;
 loop { match pc {
 0x30ae23b => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae23e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae23e); }
 let mut pc: u32 = 0x30ae23e;
 loop { match pc {
 0x30ae23e => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae241(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae241); }
 let mut pc: u32 = 0x30ae241;
 loop { match pc {
 0x30ae241 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae244); }
 let mut pc: u32 = 0x30ae244;
 loop { match pc {
 0x30ae244 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae247); }
 let mut pc: u32 = 0x30ae247;
 loop { match pc {
 0x30ae247 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae24a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae24a); }
 let mut pc: u32 = 0x30ae24a;
 loop { match pc {
 0x30ae24a => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae24d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae24d); }
 let mut pc: u32 = 0x30ae24d;
 loop { match pc {
 0x30ae24d => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae250); }
 let mut pc: u32 = 0x30ae250;
 loop { match pc {
 0x30ae250 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae253(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae253); }
 let mut pc: u32 = 0x30ae253;
 loop { match pc {
 0x30ae253 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae256); }
 let mut pc: u32 = 0x30ae256;
 loop { match pc {
 0x30ae256 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae259(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae259); }
 let mut pc: u32 = 0x30ae259;
 loop { match pc {
 0x30ae259 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae25c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae25c); }
 let mut pc: u32 = 0x30ae25c;
 loop { match pc {
 0x30ae25c => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae25f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae25f); }
 let mut pc: u32 = 0x30ae25f;
 loop { match pc {
 0x30ae25f => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae262(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae262); }
 let mut pc: u32 = 0x30ae262;
 loop { match pc {
 0x30ae262 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae265); }
 let mut pc: u32 = 0x30ae265;
 loop { match pc {
 0x30ae265 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae268); }
 let mut pc: u32 = 0x30ae268;
 loop { match pc {
 0x30ae268 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae26b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae26b); }
 let mut pc: u32 = 0x30ae26b;
 loop { match pc {
 0x30ae26b => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae26e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae26e); }
 let mut pc: u32 = 0x30ae26e;
 loop { match pc {
 0x30ae26e => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae271(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae271); }
 let mut pc: u32 = 0x30ae271;
 loop { match pc {
 0x30ae271 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae274(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae274); }
 let mut pc: u32 = 0x30ae274;
 loop { match pc {
 0x30ae274 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae277(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae277); }
 let mut pc: u32 = 0x30ae277;
 loop { match pc {
 0x30ae277 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae27a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae27a); }
 let mut pc: u32 = 0x30ae27a;
 loop { match pc {
 0x30ae27a => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae27d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae27d); }
 let mut pc: u32 = 0x30ae27d;
 loop { match pc {
 0x30ae27d => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae280); }
 let mut pc: u32 = 0x30ae280;
 loop { match pc {
 0x30ae280 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae283(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae283); }
 let mut pc: u32 = 0x30ae283;
 loop { match pc {
 0x30ae283 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae286(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae286); }
 let mut pc: u32 = 0x30ae286;
 loop { match pc {
 0x30ae286 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae289(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae289); }
 let mut pc: u32 = 0x30ae289;
 loop { match pc {
 0x30ae289 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae28c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae28c); }
 let mut pc: u32 = 0x30ae28c;
 loop { match pc {
 0x30ae28c => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae28f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae28f); }
 let mut pc: u32 = 0x30ae28f;
 loop { match pc {
 0x30ae28f => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae292(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae292); }
 let mut pc: u32 = 0x30ae292;
 loop { match pc {
 0x30ae292 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae295(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae295); }
 let mut pc: u32 = 0x30ae295;
 loop { match pc {
 0x30ae295 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae298); }
 let mut pc: u32 = 0x30ae298;
 loop { match pc {
 0x30ae298 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae29b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae29b); }
 let mut pc: u32 = 0x30ae29b;
 loop { match pc {
 0x30ae29b => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae29e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae29e); }
 let mut pc: u32 = 0x30ae29e;
 loop { match pc {
 0x30ae29e => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2a1); }
 let mut pc: u32 = 0x30ae2a1;
 loop { match pc {
 0x30ae2a1 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2a4); }
 let mut pc: u32 = 0x30ae2a4;
 loop { match pc {
 0x30ae2a4 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2a7); }
 let mut pc: u32 = 0x30ae2a7;
 loop { match pc {
 0x30ae2a7 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2aa); }
 let mut pc: u32 = 0x30ae2aa;
 loop { match pc {
 0x30ae2aa => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2ad); }
 let mut pc: u32 = 0x30ae2ad;
 loop { match pc {
 0x30ae2ad => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2b0); }
 let mut pc: u32 = 0x30ae2b0;
 loop { match pc {
 0x30ae2b0 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2b3); }
 let mut pc: u32 = 0x30ae2b3;
 loop { match pc {
 0x30ae2b3 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2b6); }
 let mut pc: u32 = 0x30ae2b6;
 loop { match pc {
 0x30ae2b6 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2b9); }
 let mut pc: u32 = 0x30ae2b9;
 loop { match pc {
 0x30ae2b9 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2bc); }
 let mut pc: u32 = 0x30ae2bc;
 loop { match pc {
 0x30ae2bc => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2bf); }
 let mut pc: u32 = 0x30ae2bf;
 loop { match pc {
 0x30ae2bf => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2c2); }
 let mut pc: u32 = 0x30ae2c2;
 loop { match pc {
 0x30ae2c2 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2c5); }
 let mut pc: u32 = 0x30ae2c5;
 loop { match pc {
 0x30ae2c5 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2c8); }
 let mut pc: u32 = 0x30ae2c8;
 loop { match pc {
 0x30ae2c8 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2cb); }
 let mut pc: u32 = 0x30ae2cb;
 loop { match pc {
 0x30ae2cb => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2ce); }
 let mut pc: u32 = 0x30ae2ce;
 loop { match pc {
 0x30ae2ce => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2d1); }
 let mut pc: u32 = 0x30ae2d1;
 loop { match pc {
 0x30ae2d1 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2d4); }
 let mut pc: u32 = 0x30ae2d4;
 loop { match pc {
 0x30ae2d4 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2d7); }
 let mut pc: u32 = 0x30ae2d7;
 loop { match pc {
 0x30ae2d7 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2da); }
 let mut pc: u32 = 0x30ae2da;
 loop { match pc {
 0x30ae2da => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2dd); }
 let mut pc: u32 = 0x30ae2dd;
 loop { match pc {
 0x30ae2dd => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2e0); }
 let mut pc: u32 = 0x30ae2e0;
 loop { match pc {
 0x30ae2e0 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2e3); }
 let mut pc: u32 = 0x30ae2e3;
 loop { match pc {
 0x30ae2e3 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2e6); }
 let mut pc: u32 = 0x30ae2e6;
 loop { match pc {
 0x30ae2e6 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2e9); }
 let mut pc: u32 = 0x30ae2e9;
 loop { match pc {
 0x30ae2e9 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2ec); }
 let mut pc: u32 = 0x30ae2ec;
 loop { match pc {
 0x30ae2ec => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2ef); }
 let mut pc: u32 = 0x30ae2ef;
 loop { match pc {
 0x30ae2ef => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2f2); }
 let mut pc: u32 = 0x30ae2f2;
 loop { match pc {
 0x30ae2f2 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2f5); }
 let mut pc: u32 = 0x30ae2f5;
 loop { match pc {
 0x30ae2f5 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2f8); }
 let mut pc: u32 = 0x30ae2f8;
 loop { match pc {
 0x30ae2f8 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2fb); }
 let mut pc: u32 = 0x30ae2fb;
 loop { match pc {
 0x30ae2fb => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae2fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae2fe); }
 let mut pc: u32 = 0x30ae2fe;
 loop { match pc {
 0x30ae2fe => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30ae305); f_030ae40d(cpu); if cpu.returned_to != 0x30ae305 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30ae310; continue;
 }
 0x30ae310 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x30ae319; continue; }
  pc = 0x30ae315; continue;
 }
 0x30ae315 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x30ae319; continue;
 }
 0x30ae319 => {
  if cpu.cf { pc = 0x30ae321; continue; }
  pc = 0x30ae31b; continue;
 }
 0x30ae31b => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x30ae310; continue;
 }
 0x30ae321 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x30ae324; continue;
 }
 0x30ae324 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x30ae32d; continue; }
  pc = 0x30ae329; continue;
 }
 0x30ae329 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x30ae32d; continue;
 }
 0x30ae32d => {
  if !cpu.cf { pc = 0x30ae335; continue; }
  pc = 0x30ae32f; continue;
 }
 0x30ae32f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x30ae324; continue;
 }
 0x30ae335 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae338); }
 let mut pc: u32 = 0x30ae338;
 loop { match pc {
 0x30ae338 => {
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
  if cpu.zf { pc = 0x30ae396; continue; }
  pc = 0x30ae361; continue;
 }
 0x30ae361 => {
  if (cpu.cf || cpu.zf) { pc = 0x30ae36c; continue; }
  pc = 0x30ae363; continue;
 }
 0x30ae363 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30ae363; continue; }
  pc = 0x30ae36a; continue;
 }
 0x30ae36a => {
  pc = 0x30ae396; continue;
 }
 0x30ae36c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x30ae370; continue;
 }
 0x30ae370 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30ae370; continue; }
  pc = 0x30ae37b; continue;
 }
 0x30ae37b => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x30ae396; continue; }
  pc = 0x30ae382; continue;
 }
 0x30ae382 => {
  if !cpu.zf { pc = 0x30ae390; continue; }
  pc = 0x30ae384; continue;
 }
 0x30ae384 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x30ae390; continue; }
  pc = 0x30ae388; continue;
 }
 0x30ae388 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x30ae396; continue; }
  pc = 0x30ae390; continue;
 }
 0x30ae390 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x30ae396; continue;
 }
 0x30ae396 => {
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
pub fn f_030ae3a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae3a3); }
 let mut pc: u32 = 0x30ae3a3;
 loop { match pc {
 0x30ae3a3 => {
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
  pc = 0x30ae3b8; continue;
 }
 0x30ae3b8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x30ae3cb; continue; }
  pc = 0x30ae3c4; continue;
 }
 0x30ae3c4 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x30ae3cd; continue; }
  pc = 0x30ae3c8; continue;
 }
 0x30ae3c8 => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x30ae3e3; continue;
 }
 0x30ae3cb => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x30ae3cd; continue;
 }
 0x30ae3cd => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x30ae3dc, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x30ae3e1, "Aam");
  pc = 0x30ae3e3; continue;
 }
 0x30ae3e3 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x30ae407; continue; }
  pc = 0x30ae3f5; continue;
 }
 0x30ae3f5 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x30ae407; continue; }
  pc = 0x30ae3fb; continue;
 }
 0x30ae3fb => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x30ae407; continue; }
  pc = 0x30ae401; continue;
 }
 0x30ae401 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30ae3b8; continue; }
  pc = 0x30ae407; continue;
 }
 0x30ae407 => {
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
pub fn f_030ae40d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae40d); }
 let mut pc: u32 = 0x30ae40d;
 loop { match pc {
 0x30ae40d => {
  cpu.push(0x30ae412); f_030ae4c2(cpu); if cpu.returned_to != 0x30ae412 { return; }
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
  cpu.unimplemented(0x30ae43f, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x30ae450, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x30ae456; continue; }
  pc = 0x30ae461; continue;
 }
 0x30ae456 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x30ae456; continue; }
  pc = 0x30ae461; continue;
 }
 0x30ae461 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x30ae46f); cpu.call(0x0); if cpu.returned_to != 0x30ae46f { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x30ae47c); cpu.call(0x3f32cf2); if cpu.returned_to != 0x30ae47c { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x30ae481, "Aam");
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
  cpu.unimplemented(0x30ae4a4, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x30ae4b4); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae4c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae4c2); }
 let mut pc: u32 = 0x30ae4c2;
 loop { match pc {
 0x30ae4c2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae4c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae4c4); }
 let mut pc: u32 = 0x30ae4c4;
 loop { match pc {
 0x30ae4c4 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x314b6bcu32; cpu.eax = v; }
  cpu.push(0x30ae4d4); f_030adb62(cpu); if cpu.returned_to != 0x30ae4d4 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae4d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30ae4d6); } r_030ae4d6(cpu, 0x30ae4d6); }
pub fn f_030ae560(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30ae560); } r_030ae4d6(cpu, 0x30ae560); }
fn r_030ae4d6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30ae4d6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x30ae4dc, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x30ae4e1, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x30ae562; continue; }
  pc = 0x30ae4f1; continue;
 }
 0x30ae4f1 => {
  if cpu.cf { pc = 0x30ae566; continue; }
  pc = 0x30ae4f3; continue;
 }
 0x30ae4f3 => {
  if cpu.zf { pc = 0x30ae56a; continue; }
  pc = 0x30ae4f5; continue;
 }
 0x30ae4f5 => {
  if (cpu.cf || cpu.zf) { pc = 0x30ae56e; continue; }
  pc = 0x30ae4f7; continue;
 }
 0x30ae4f7 => {
  if cpu.sf { pc = 0x30ae572; continue; }
  pc = 0x30ae4f9; continue;
 }
 0x30ae4f9 => {
  if cpu.pf() { f_030ae4fb(cpu); return; }
  f_030ae4fb(cpu); return;
 }
 0x30ae560 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x30ae5a8; continue; }
  pc = 0x30ae569; continue;
 }
 0x30ae562 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30ae566 => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_030ae5dc(cpu); return; }
  pc = 0x30ae568; continue;
 }
 0x30ae568 => {
  cpu.unimplemented(0x30ae568, "Aas");
  pc = 0x30ae569; continue;
 }
 0x30ae569 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x30ae59f; continue; }
  pc = 0x30ae57b; continue;
 }
 0x30ae56a => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x30ae59f; continue; }
  pc = 0x30ae57b; continue;
 }
 0x30ae56e => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x30ae59f; continue; }
  pc = 0x30ae57b; continue;
 }
 0x30ae572 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x30ae59f; continue; }
  pc = 0x30ae57b; continue;
 }
 0x30ae57b => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x30ae59f; continue; }
  pc = 0x30ae583; continue;
 }
 0x30ae583 => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x30ae5a8; continue; }
  pc = 0x30ae58b; continue;
 }
 0x30ae58b => {
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
 0x30ae59f => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30ae5a8 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae4fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae4fb); }
 let mut pc: u32 = 0x30ae4fb;
 loop { match pc {
 0x30ae4fb => {
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
  pc = 0x30ae511; continue;
 }
 0x30ae511 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x30ae4d6u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30ae511; continue; }
  pc = 0x30ae531; continue;
 }
 0x30ae531 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x30ae531; continue; }
  pc = 0x30ae53b; continue;
 }
 0x30ae53b => {
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
pub fn f_030ae545(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae545); }
 let mut pc: u32 = 0x30ae545;
 loop { match pc {
 0x30ae545 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x30ae557; continue; }
  pc = 0x30ae54d; continue;
 }
 0x30ae54d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30ae557; continue; }
  pc = 0x30ae551; continue;
 }
 0x30ae551 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x30ae557; continue;
 }
 0x30ae557 => {
  cpu.push(0x30ae55c); f_030ae4fb(cpu); if cpu.returned_to != 0x30ae55c { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5ac); }
 let mut pc: u32 = 0x30ae5ac;
 loop { match pc {
 0x30ae5ac => {
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
pub fn f_030ae5d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5d0); }
 let mut pc: u32 = 0x30ae5d0;
 loop { match pc {
 0x30ae5d0 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5d4); }
 let mut pc: u32 = 0x30ae5d4;
 loop { match pc {
 0x30ae5d4 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5d8); }
 let mut pc: u32 = 0x30ae5d8;
 loop { match pc {
 0x30ae5d8 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5dc); }
 let mut pc: u32 = 0x30ae5dc;
 loop { match pc {
 0x30ae5dc => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5e0); }
 let mut pc: u32 = 0x30ae5e0;
 loop { match pc {
 0x30ae5e0 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5e4); }
 let mut pc: u32 = 0x30ae5e4;
 loop { match pc {
 0x30ae5e4 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5e8); }
 let mut pc: u32 = 0x30ae5e8;
 loop { match pc {
 0x30ae5e8 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5ec); }
 let mut pc: u32 = 0x30ae5ec;
 loop { match pc {
 0x30ae5ec => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5f0); }
 let mut pc: u32 = 0x30ae5f0;
 loop { match pc {
 0x30ae5f0 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5f4); }
 let mut pc: u32 = 0x30ae5f4;
 loop { match pc {
 0x30ae5f4 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5f8); }
 let mut pc: u32 = 0x30ae5f8;
 loop { match pc {
 0x30ae5f8 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae5fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae5fc); }
 let mut pc: u32 = 0x30ae5fc;
 loop { match pc {
 0x30ae5fc => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae600); }
 let mut pc: u32 = 0x30ae600;
 loop { match pc {
 0x30ae600 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae604(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae604); }
 let mut pc: u32 = 0x30ae604;
 loop { match pc {
 0x30ae604 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae608(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae608); }
 let mut pc: u32 = 0x30ae608;
 loop { match pc {
 0x30ae608 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae60c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae60c); }
 let mut pc: u32 = 0x30ae60c;
 loop { match pc {
 0x30ae60c => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae610); }
 let mut pc: u32 = 0x30ae610;
 loop { match pc {
 0x30ae610 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae614); }
 let mut pc: u32 = 0x30ae614;
 loop { match pc {
 0x30ae614 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae618(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae618); }
 let mut pc: u32 = 0x30ae618;
 loop { match pc {
 0x30ae618 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae61c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae61c); }
 let mut pc: u32 = 0x30ae61c;
 loop { match pc {
 0x30ae61c => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae620); }
 let mut pc: u32 = 0x30ae620;
 loop { match pc {
 0x30ae620 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae624(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae624); }
 let mut pc: u32 = 0x30ae624;
 loop { match pc {
 0x30ae624 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae628(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae628); }
 let mut pc: u32 = 0x30ae628;
 loop { match pc {
 0x30ae628 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae62c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae62c); }
 let mut pc: u32 = 0x30ae62c;
 loop { match pc {
 0x30ae62c => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae630); }
 let mut pc: u32 = 0x30ae630;
 loop { match pc {
 0x30ae630 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae634); }
 let mut pc: u32 = 0x30ae634;
 loop { match pc {
 0x30ae634 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae638); }
 let mut pc: u32 = 0x30ae638;
 loop { match pc {
 0x30ae638 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae63c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae63c); }
 let mut pc: u32 = 0x30ae63c;
 loop { match pc {
 0x30ae63c => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae640); }
 let mut pc: u32 = 0x30ae640;
 loop { match pc {
 0x30ae640 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae644(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae644); }
 let mut pc: u32 = 0x30ae644;
 loop { match pc {
 0x30ae644 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae648(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae648); }
 let mut pc: u32 = 0x30ae648;
 loop { match pc {
 0x30ae648 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae64c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae64c); }
 let mut pc: u32 = 0x30ae64c;
 loop { match pc {
 0x30ae64c => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae650); }
 let mut pc: u32 = 0x30ae650;
 loop { match pc {
 0x30ae650 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae654(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae654); }
 let mut pc: u32 = 0x30ae654;
 loop { match pc {
 0x30ae654 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae658(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae658); }
 let mut pc: u32 = 0x30ae658;
 loop { match pc {
 0x30ae658 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae65c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae65c); }
 let mut pc: u32 = 0x30ae65c;
 loop { match pc {
 0x30ae65c => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae660); }
 let mut pc: u32 = 0x30ae660;
 loop { match pc {
 0x30ae660 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae664(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae664); }
 let mut pc: u32 = 0x30ae664;
 loop { match pc {
 0x30ae664 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae668(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae668); }
 let mut pc: u32 = 0x30ae668;
 loop { match pc {
 0x30ae668 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae66c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae66c); }
 let mut pc: u32 = 0x30ae66c;
 loop { match pc {
 0x30ae66c => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae670); }
 let mut pc: u32 = 0x30ae670;
 loop { match pc {
 0x30ae670 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae674); }
 let mut pc: u32 = 0x30ae674;
 loop { match pc {
 0x30ae674 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae678); }
 let mut pc: u32 = 0x30ae678;
 loop { match pc {
 0x30ae678 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae67c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae67c); }
 let mut pc: u32 = 0x30ae67c;
 loop { match pc {
 0x30ae67c => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae680); }
 let mut pc: u32 = 0x30ae680;
 loop { match pc {
 0x30ae680 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae684(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae684); }
 let mut pc: u32 = 0x30ae684;
 loop { match pc {
 0x30ae684 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae688); }
 let mut pc: u32 = 0x30ae688;
 loop { match pc {
 0x30ae688 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae68c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae68c); }
 let mut pc: u32 = 0x30ae68c;
 loop { match pc {
 0x30ae68c => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae690); }
 let mut pc: u32 = 0x30ae690;
 loop { match pc {
 0x30ae690 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae694(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae694); }
 let mut pc: u32 = 0x30ae694;
 loop { match pc {
 0x30ae694 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae698); }
 let mut pc: u32 = 0x30ae698;
 loop { match pc {
 0x30ae698 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae69c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae69c); }
 let mut pc: u32 = 0x30ae69c;
 loop { match pc {
 0x30ae69c => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6a0); }
 let mut pc: u32 = 0x30ae6a0;
 loop { match pc {
 0x30ae6a0 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6a4); }
 let mut pc: u32 = 0x30ae6a4;
 loop { match pc {
 0x30ae6a4 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6a8); }
 let mut pc: u32 = 0x30ae6a8;
 loop { match pc {
 0x30ae6a8 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6ac); }
 let mut pc: u32 = 0x30ae6ac;
 loop { match pc {
 0x30ae6ac => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6b0); }
 let mut pc: u32 = 0x30ae6b0;
 loop { match pc {
 0x30ae6b0 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6b4); }
 let mut pc: u32 = 0x30ae6b4;
 loop { match pc {
 0x30ae6b4 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6b8); }
 let mut pc: u32 = 0x30ae6b8;
 loop { match pc {
 0x30ae6b8 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6bc); }
 let mut pc: u32 = 0x30ae6bc;
 loop { match pc {
 0x30ae6bc => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6c0); }
 let mut pc: u32 = 0x30ae6c0;
 loop { match pc {
 0x30ae6c0 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6c4); }
 let mut pc: u32 = 0x30ae6c4;
 loop { match pc {
 0x30ae6c4 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6c8); }
 let mut pc: u32 = 0x30ae6c8;
 loop { match pc {
 0x30ae6c8 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6cc); }
 let mut pc: u32 = 0x30ae6cc;
 loop { match pc {
 0x30ae6cc => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6d0); }
 let mut pc: u32 = 0x30ae6d0;
 loop { match pc {
 0x30ae6d0 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6d4); }
 let mut pc: u32 = 0x30ae6d4;
 loop { match pc {
 0x30ae6d4 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6d8); }
 let mut pc: u32 = 0x30ae6d8;
 loop { match pc {
 0x30ae6d8 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6dc); }
 let mut pc: u32 = 0x30ae6dc;
 loop { match pc {
 0x30ae6dc => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6e0); }
 let mut pc: u32 = 0x30ae6e0;
 loop { match pc {
 0x30ae6e0 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6e4); }
 let mut pc: u32 = 0x30ae6e4;
 loop { match pc {
 0x30ae6e4 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6e8); }
 let mut pc: u32 = 0x30ae6e8;
 loop { match pc {
 0x30ae6e8 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6ec); }
 let mut pc: u32 = 0x30ae6ec;
 loop { match pc {
 0x30ae6ec => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6f0); }
 let mut pc: u32 = 0x30ae6f0;
 loop { match pc {
 0x30ae6f0 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6f4); }
 let mut pc: u32 = 0x30ae6f4;
 loop { match pc {
 0x30ae6f4 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6f8); }
 let mut pc: u32 = 0x30ae6f8;
 loop { match pc {
 0x30ae6f8 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae6fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae6fc); }
 let mut pc: u32 = 0x30ae6fc;
 loop { match pc {
 0x30ae6fc => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae700); }
 let mut pc: u32 = 0x30ae700;
 loop { match pc {
 0x30ae700 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae704); }
 let mut pc: u32 = 0x30ae704;
 loop { match pc {
 0x30ae704 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae708(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae708); }
 let mut pc: u32 = 0x30ae708;
 loop { match pc {
 0x30ae708 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae70c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae70c); }
 let mut pc: u32 = 0x30ae70c;
 loop { match pc {
 0x30ae70c => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae710); }
 let mut pc: u32 = 0x30ae710;
 loop { match pc {
 0x30ae710 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae714(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae714); }
 let mut pc: u32 = 0x30ae714;
 loop { match pc {
 0x30ae714 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae718); }
 let mut pc: u32 = 0x30ae718;
 loop { match pc {
 0x30ae718 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae71c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae71c); }
 let mut pc: u32 = 0x30ae71c;
 loop { match pc {
 0x30ae71c => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae720); }
 let mut pc: u32 = 0x30ae720;
 loop { match pc {
 0x30ae720 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae724(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae724); }
 let mut pc: u32 = 0x30ae724;
 loop { match pc {
 0x30ae724 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae728); }
 let mut pc: u32 = 0x30ae728;
 loop { match pc {
 0x30ae728 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae72c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae72c); }
 let mut pc: u32 = 0x30ae72c;
 loop { match pc {
 0x30ae72c => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ae730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ae730); }
 let mut pc: u32 = 0x30ae730;
 loop { match pc {
 0x30ae730 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
