#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_040adf54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf54); }
 let mut pc: u32 = 0x40adf54;
 loop { match pc {
 0x40adf54 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf58); }
 let mut pc: u32 = 0x40adf58;
 loop { match pc {
 0x40adf58 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf5c); }
 let mut pc: u32 = 0x40adf5c;
 loop { match pc {
 0x40adf5c => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf60); }
 let mut pc: u32 = 0x40adf60;
 loop { match pc {
 0x40adf60 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf64); }
 let mut pc: u32 = 0x40adf64;
 loop { match pc {
 0x40adf64 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf68); }
 let mut pc: u32 = 0x40adf68;
 loop { match pc {
 0x40adf68 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf6c); }
 let mut pc: u32 = 0x40adf6c;
 loop { match pc {
 0x40adf6c => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf70); }
 let mut pc: u32 = 0x40adf70;
 loop { match pc {
 0x40adf70 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf74); }
 let mut pc: u32 = 0x40adf74;
 loop { match pc {
 0x40adf74 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf78); }
 let mut pc: u32 = 0x40adf78;
 loop { match pc {
 0x40adf78 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf7c); }
 let mut pc: u32 = 0x40adf7c;
 loop { match pc {
 0x40adf7c => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf80); }
 let mut pc: u32 = 0x40adf80;
 loop { match pc {
 0x40adf80 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf84); }
 let mut pc: u32 = 0x40adf84;
 loop { match pc {
 0x40adf84 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf88); }
 let mut pc: u32 = 0x40adf88;
 loop { match pc {
 0x40adf88 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf8c); }
 let mut pc: u32 = 0x40adf8c;
 loop { match pc {
 0x40adf8c => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf90); }
 let mut pc: u32 = 0x40adf90;
 loop { match pc {
 0x40adf90 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf94); }
 let mut pc: u32 = 0x40adf94;
 loop { match pc {
 0x40adf94 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf98); }
 let mut pc: u32 = 0x40adf98;
 loop { match pc {
 0x40adf98 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf9c); }
 let mut pc: u32 = 0x40adf9c;
 loop { match pc {
 0x40adf9c => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfa0); }
 let mut pc: u32 = 0x40adfa0;
 loop { match pc {
 0x40adfa0 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfa4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfa4); }
 let mut pc: u32 = 0x40adfa4;
 loop { match pc {
 0x40adfa4 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfa8); }
 let mut pc: u32 = 0x40adfa8;
 loop { match pc {
 0x40adfa8 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfac); }
 let mut pc: u32 = 0x40adfac;
 loop { match pc {
 0x40adfac => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfb0); }
 let mut pc: u32 = 0x40adfb0;
 loop { match pc {
 0x40adfb0 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfb4); }
 let mut pc: u32 = 0x40adfb4;
 loop { match pc {
 0x40adfb4 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfb8); }
 let mut pc: u32 = 0x40adfb8;
 loop { match pc {
 0x40adfb8 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfbc); }
 let mut pc: u32 = 0x40adfbc;
 loop { match pc {
 0x40adfbc => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfc0); }
 let mut pc: u32 = 0x40adfc0;
 loop { match pc {
 0x40adfc0 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfc4); }
 let mut pc: u32 = 0x40adfc4;
 loop { match pc {
 0x40adfc4 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfc8); }
 let mut pc: u32 = 0x40adfc8;
 loop { match pc {
 0x40adfc8 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfcc); }
 let mut pc: u32 = 0x40adfcc;
 loop { match pc {
 0x40adfcc => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfd0); }
 let mut pc: u32 = 0x40adfd0;
 loop { match pc {
 0x40adfd0 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfd4); }
 let mut pc: u32 = 0x40adfd4;
 loop { match pc {
 0x40adfd4 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfd8); }
 let mut pc: u32 = 0x40adfd8;
 loop { match pc {
 0x40adfd8 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfdc); }
 let mut pc: u32 = 0x40adfdc;
 loop { match pc {
 0x40adfdc => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfe0); }
 let mut pc: u32 = 0x40adfe0;
 loop { match pc {
 0x40adfe0 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfe4); }
 let mut pc: u32 = 0x40adfe4;
 loop { match pc {
 0x40adfe4 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfe8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfe8); }
 let mut pc: u32 = 0x40adfe8;
 loop { match pc {
 0x40adfe8 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adfec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adfec); }
 let mut pc: u32 = 0x40adfec;
 loop { match pc {
 0x40adfec => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adff0); }
 let mut pc: u32 = 0x40adff0;
 loop { match pc {
 0x40adff0 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adff4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adff4); }
 let mut pc: u32 = 0x40adff4;
 loop { match pc {
 0x40adff4 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adff8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adff8); }
 let mut pc: u32 = 0x40adff8;
 loop { match pc {
 0x40adff8 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adffc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adffc); }
 let mut pc: u32 = 0x40adffc;
 loop { match pc {
 0x40adffc => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae000); }
 let mut pc: u32 = 0x40ae000;
 loop { match pc {
 0x40ae000 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae004); }
 let mut pc: u32 = 0x40ae004;
 loop { match pc {
 0x40ae004 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae008); }
 let mut pc: u32 = 0x40ae008;
 loop { match pc {
 0x40ae008 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae00c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae00c); }
 let mut pc: u32 = 0x40ae00c;
 loop { match pc {
 0x40ae00c => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae010); }
 let mut pc: u32 = 0x40ae010;
 loop { match pc {
 0x40ae010 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae014); }
 let mut pc: u32 = 0x40ae014;
 loop { match pc {
 0x40ae014 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae018); }
 let mut pc: u32 = 0x40ae018;
 loop { match pc {
 0x40ae018 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae01c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae01c); }
 let mut pc: u32 = 0x40ae01c;
 loop { match pc {
 0x40ae01c => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae020); }
 let mut pc: u32 = 0x40ae020;
 loop { match pc {
 0x40ae020 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae024); }
 let mut pc: u32 = 0x40ae024;
 loop { match pc {
 0x40ae024 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae028(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae028); }
 let mut pc: u32 = 0x40ae028;
 loop { match pc {
 0x40ae028 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae02c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae02c); }
 let mut pc: u32 = 0x40ae02c;
 loop { match pc {
 0x40ae02c => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae030); }
 let mut pc: u32 = 0x40ae030;
 loop { match pc {
 0x40ae030 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae034(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae034); }
 let mut pc: u32 = 0x40ae034;
 loop { match pc {
 0x40ae034 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae038); }
 let mut pc: u32 = 0x40ae038;
 loop { match pc {
 0x40ae038 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae03c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae03c); }
 let mut pc: u32 = 0x40ae03c;
 loop { match pc {
 0x40ae03c => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae040); }
 let mut pc: u32 = 0x40ae040;
 loop { match pc {
 0x40ae040 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae044(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae044); }
 let mut pc: u32 = 0x40ae044;
 loop { match pc {
 0x40ae044 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae048(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae048); }
 let mut pc: u32 = 0x40ae048;
 loop { match pc {
 0x40ae048 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae04c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae04c); }
 let mut pc: u32 = 0x40ae04c;
 loop { match pc {
 0x40ae04c => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae050); }
 let mut pc: u32 = 0x40ae050;
 loop { match pc {
 0x40ae050 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae054); }
 let mut pc: u32 = 0x40ae054;
 loop { match pc {
 0x40ae054 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae058); }
 let mut pc: u32 = 0x40ae058;
 loop { match pc {
 0x40ae058 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae05c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae05c); }
 let mut pc: u32 = 0x40ae05c;
 loop { match pc {
 0x40ae05c => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae060); }
 let mut pc: u32 = 0x40ae060;
 loop { match pc {
 0x40ae060 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae064(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae064); }
 let mut pc: u32 = 0x40ae064;
 loop { match pc {
 0x40ae064 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae068(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae068); }
 let mut pc: u32 = 0x40ae068;
 loop { match pc {
 0x40ae068 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae06c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae06c); }
 let mut pc: u32 = 0x40ae06c;
 loop { match pc {
 0x40ae06c => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae070); }
 let mut pc: u32 = 0x40ae070;
 loop { match pc {
 0x40ae070 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae074(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae074); }
 let mut pc: u32 = 0x40ae074;
 loop { match pc {
 0x40ae074 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae078); }
 let mut pc: u32 = 0x40ae078;
 loop { match pc {
 0x40ae078 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae07c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae07c); }
 let mut pc: u32 = 0x40ae07c;
 loop { match pc {
 0x40ae07c => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae080); }
 let mut pc: u32 = 0x40ae080;
 loop { match pc {
 0x40ae080 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae084); }
 let mut pc: u32 = 0x40ae084;
 loop { match pc {
 0x40ae084 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae088(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae088); }
 let mut pc: u32 = 0x40ae088;
 loop { match pc {
 0x40ae088 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae08c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae08c); }
 let mut pc: u32 = 0x40ae08c;
 loop { match pc {
 0x40ae08c => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae090); }
 let mut pc: u32 = 0x40ae090;
 loop { match pc {
 0x40ae090 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae094(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae094); }
 let mut pc: u32 = 0x40ae094;
 loop { match pc {
 0x40ae094 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae098(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae098); }
 let mut pc: u32 = 0x40ae098;
 loop { match pc {
 0x40ae098 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae09c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae09c); }
 let mut pc: u32 = 0x40ae09c;
 loop { match pc {
 0x40ae09c => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0a0); }
 let mut pc: u32 = 0x40ae0a0;
 loop { match pc {
 0x40ae0a0 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0a4); }
 let mut pc: u32 = 0x40ae0a4;
 loop { match pc {
 0x40ae0a4 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0a8); }
 let mut pc: u32 = 0x40ae0a8;
 loop { match pc {
 0x40ae0a8 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0ac); }
 let mut pc: u32 = 0x40ae0ac;
 loop { match pc {
 0x40ae0ac => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0b0); }
 let mut pc: u32 = 0x40ae0b0;
 loop { match pc {
 0x40ae0b0 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0b4); }
 let mut pc: u32 = 0x40ae0b4;
 loop { match pc {
 0x40ae0b4 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0b8); }
 let mut pc: u32 = 0x40ae0b8;
 loop { match pc {
 0x40ae0b8 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0bc); }
 let mut pc: u32 = 0x40ae0bc;
 loop { match pc {
 0x40ae0bc => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0c0); }
 let mut pc: u32 = 0x40ae0c0;
 loop { match pc {
 0x40ae0c0 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0c4); }
 let mut pc: u32 = 0x40ae0c4;
 loop { match pc {
 0x40ae0c4 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0c8); }
 let mut pc: u32 = 0x40ae0c8;
 loop { match pc {
 0x40ae0c8 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0cc); }
 let mut pc: u32 = 0x40ae0cc;
 loop { match pc {
 0x40ae0cc => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0d0); }
 let mut pc: u32 = 0x40ae0d0;
 loop { match pc {
 0x40ae0d0 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0d4); }
 let mut pc: u32 = 0x40ae0d4;
 loop { match pc {
 0x40ae0d4 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0d8); }
 let mut pc: u32 = 0x40ae0d8;
 loop { match pc {
 0x40ae0d8 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0dc); }
 let mut pc: u32 = 0x40ae0dc;
 loop { match pc {
 0x40ae0dc => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0e0); }
 let mut pc: u32 = 0x40ae0e0;
 loop { match pc {
 0x40ae0e0 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0e4); }
 let mut pc: u32 = 0x40ae0e4;
 loop { match pc {
 0x40ae0e4 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0e8); }
 let mut pc: u32 = 0x40ae0e8;
 loop { match pc {
 0x40ae0e8 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0ec); }
 let mut pc: u32 = 0x40ae0ec;
 loop { match pc {
 0x40ae0ec => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0f0); }
 let mut pc: u32 = 0x40ae0f0;
 loop { match pc {
 0x40ae0f0 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0f4); }
 let mut pc: u32 = 0x40ae0f4;
 loop { match pc {
 0x40ae0f4 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0f8); }
 let mut pc: u32 = 0x40ae0f8;
 loop { match pc {
 0x40ae0f8 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae0fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae0fc); }
 let mut pc: u32 = 0x40ae0fc;
 loop { match pc {
 0x40ae0fc => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae100); }
 let mut pc: u32 = 0x40ae100;
 loop { match pc {
 0x40ae100 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae104(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae104); }
 let mut pc: u32 = 0x40ae104;
 loop { match pc {
 0x40ae104 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae108(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae108); }
 let mut pc: u32 = 0x40ae108;
 loop { match pc {
 0x40ae108 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae10c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae10c); }
 let mut pc: u32 = 0x40ae10c;
 loop { match pc {
 0x40ae10c => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae110); }
 let mut pc: u32 = 0x40ae110;
 loop { match pc {
 0x40ae110 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae114(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae114); }
 let mut pc: u32 = 0x40ae114;
 loop { match pc {
 0x40ae114 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae118(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae118); }
 let mut pc: u32 = 0x40ae118;
 loop { match pc {
 0x40ae118 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae11c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae11c); }
 let mut pc: u32 = 0x40ae11c;
 loop { match pc {
 0x40ae11c => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae120); }
 let mut pc: u32 = 0x40ae120;
 loop { match pc {
 0x40ae120 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae124(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae124); }
 let mut pc: u32 = 0x40ae124;
 loop { match pc {
 0x40ae124 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae128(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae128); }
 let mut pc: u32 = 0x40ae128;
 loop { match pc {
 0x40ae128 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae12c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae12c); }
 let mut pc: u32 = 0x40ae12c;
 loop { match pc {
 0x40ae12c => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae130); }
 let mut pc: u32 = 0x40ae130;
 loop { match pc {
 0x40ae130 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae134(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae134); }
 let mut pc: u32 = 0x40ae134;
 loop { match pc {
 0x40ae134 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae138(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae138); }
 let mut pc: u32 = 0x40ae138;
 loop { match pc {
 0x40ae138 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae13c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae13c); }
 let mut pc: u32 = 0x40ae13c;
 loop { match pc {
 0x40ae13c => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae140); }
 let mut pc: u32 = 0x40ae140;
 loop { match pc {
 0x40ae140 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae144(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae144); }
 let mut pc: u32 = 0x40ae144;
 loop { match pc {
 0x40ae144 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae148(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae148); }
 let mut pc: u32 = 0x40ae148;
 loop { match pc {
 0x40ae148 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae14c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae14c); }
 let mut pc: u32 = 0x40ae14c;
 loop { match pc {
 0x40ae14c => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae150); }
 let mut pc: u32 = 0x40ae150;
 loop { match pc {
 0x40ae150 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae154); }
 let mut pc: u32 = 0x40ae154;
 loop { match pc {
 0x40ae154 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae158(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae158); }
 let mut pc: u32 = 0x40ae158;
 loop { match pc {
 0x40ae158 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae15c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae15c); }
 let mut pc: u32 = 0x40ae15c;
 loop { match pc {
 0x40ae15c => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae160); }
 let mut pc: u32 = 0x40ae160;
 loop { match pc {
 0x40ae160 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae164); }
 let mut pc: u32 = 0x40ae164;
 loop { match pc {
 0x40ae164 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae168); }
 let mut pc: u32 = 0x40ae168;
 loop { match pc {
 0x40ae168 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae16c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae16c); }
 let mut pc: u32 = 0x40ae16c;
 loop { match pc {
 0x40ae16c => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae170); }
 let mut pc: u32 = 0x40ae170;
 loop { match pc {
 0x40ae170 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae174(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae174); }
 let mut pc: u32 = 0x40ae174;
 loop { match pc {
 0x40ae174 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae178(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae178); }
 let mut pc: u32 = 0x40ae178;
 loop { match pc {
 0x40ae178 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae17c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae17c); }
 let mut pc: u32 = 0x40ae17c;
 loop { match pc {
 0x40ae17c => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae180); }
 let mut pc: u32 = 0x40ae180;
 loop { match pc {
 0x40ae180 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae184(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae184); }
 let mut pc: u32 = 0x40ae184;
 loop { match pc {
 0x40ae184 => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae188(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae188); }
 let mut pc: u32 = 0x40ae188;
 loop { match pc {
 0x40ae188 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae18c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae18c); }
 let mut pc: u32 = 0x40ae18c;
 loop { match pc {
 0x40ae18c => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae190); }
 let mut pc: u32 = 0x40ae190;
 loop { match pc {
 0x40ae190 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae194(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae194); }
 let mut pc: u32 = 0x40ae194;
 loop { match pc {
 0x40ae194 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae198(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae198); }
 let mut pc: u32 = 0x40ae198;
 loop { match pc {
 0x40ae198 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae19c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae19c); }
 let mut pc: u32 = 0x40ae19c;
 loop { match pc {
 0x40ae19c => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1a0); }
 let mut pc: u32 = 0x40ae1a0;
 loop { match pc {
 0x40ae1a0 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1a4); }
 let mut pc: u32 = 0x40ae1a4;
 loop { match pc {
 0x40ae1a4 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1a8); }
 let mut pc: u32 = 0x40ae1a8;
 loop { match pc {
 0x40ae1a8 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1ac); }
 let mut pc: u32 = 0x40ae1ac;
 loop { match pc {
 0x40ae1ac => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1b0); }
 let mut pc: u32 = 0x40ae1b0;
 loop { match pc {
 0x40ae1b0 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1b4); }
 let mut pc: u32 = 0x40ae1b4;
 loop { match pc {
 0x40ae1b4 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1b8); }
 let mut pc: u32 = 0x40ae1b8;
 loop { match pc {
 0x40ae1b8 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1bc); }
 let mut pc: u32 = 0x40ae1bc;
 loop { match pc {
 0x40ae1bc => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1c0); }
 let mut pc: u32 = 0x40ae1c0;
 loop { match pc {
 0x40ae1c0 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1c4); }
 let mut pc: u32 = 0x40ae1c4;
 loop { match pc {
 0x40ae1c4 => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1c8); }
 let mut pc: u32 = 0x40ae1c8;
 loop { match pc {
 0x40ae1c8 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1cc); }
 let mut pc: u32 = 0x40ae1cc;
 loop { match pc {
 0x40ae1cc => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1d0); }
 let mut pc: u32 = 0x40ae1d0;
 loop { match pc {
 0x40ae1d0 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1d4); }
 let mut pc: u32 = 0x40ae1d4;
 loop { match pc {
 0x40ae1d4 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1d8); }
 let mut pc: u32 = 0x40ae1d8;
 loop { match pc {
 0x40ae1d8 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1dc); }
 let mut pc: u32 = 0x40ae1dc;
 loop { match pc {
 0x40ae1dc => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1e0); }
 let mut pc: u32 = 0x40ae1e0;
 loop { match pc {
 0x40ae1e0 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1e4); }
 let mut pc: u32 = 0x40ae1e4;
 loop { match pc {
 0x40ae1e4 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1e8); }
 let mut pc: u32 = 0x40ae1e8;
 loop { match pc {
 0x40ae1e8 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1ec); }
 let mut pc: u32 = 0x40ae1ec;
 loop { match pc {
 0x40ae1ec => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae1f0); }
 let mut pc: u32 = 0x40ae1f0;
 loop { match pc {
 0x40ae1f0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x40addf0u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae206(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae206); }
 let mut pc: u32 = 0x40ae206;
 loop { match pc {
 0x40ae206 => {
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
  cpu.flat_ds(0x40ae244); cpu.push(0x40ae249); f_040ae1f0(cpu); if cpu.returned_to != 0x40ae249 { return; }
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
  cpu.flat_ds(0x40ae297); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae298(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae298); }
 let mut pc: u32 = 0x40ae298;
 loop { match pc {
 0x40ae298 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x40ae2a9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae366); }
 let mut pc: u32 = 0x40ae366;
 loop { match pc {
 0x40ae366 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415c744u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae375(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae375); }
 let mut pc: u32 = 0x40ae375;
 loop { match pc {
 0x40ae375 => {
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
  if cpu.cf { pc = 0x40ae3b4; continue; }
  pc = 0x40ae397; continue;
 }
 0x40ae397 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ae3c1; continue; }
  pc = 0x40ae399; continue;
 }
 0x40ae399 => {
  { cpu.sub8((cpu.edx as u8), 0x66u8); }
  if cpu.cf { pc = 0x40ae3aa; continue; }
  pc = 0x40ae39e; continue;
 }
 0x40ae39e => {
  if (cpu.cf || cpu.zf) { pc = 0x40ae3d6; continue; }
  pc = 0x40ae3a0; continue;
 }
 0x40ae3a0 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.zf { pc = 0x40ae3c1; continue; }
  pc = 0x40ae3a5; continue;
 }
 0x40ae3a5 => {
  pc = 0x40ae453; continue;
 }
 0x40ae3aa => {
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.zf { pc = 0x40ae3d1; continue; }
  pc = 0x40ae3af; continue;
 }
 0x40ae3af => {
  pc = 0x40ae453; continue;
 }
 0x40ae3b4 => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  if cpu.cf { pc = 0x40ae453; continue; }
  pc = 0x40ae3bd; continue;
 }
 0x40ae3bd => {
  if (cpu.cf || cpu.zf) { pc = 0x40ae3d1; continue; }
  pc = 0x40ae3bf; continue;
 }
 0x40ae3bf => {
  pc = 0x40ae3d6; continue;
 }
 0x40ae3c1 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x40ae3ce; continue; }
  pc = 0x40ae3c7; continue;
 }
 0x40ae3c7 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40ae3ce; continue;
 }
 0x40ae3ce => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40ae3d1; continue;
 }
 0x40ae3d1 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x40ae3d6; continue;
 }
 0x40ae3d6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40ae3f7; continue; }
  pc = 0x40ae3f0; continue;
 }
 0x40ae3f0 => {
  { let v: u32 = 0x6u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40ae3f7; continue;
 }
 0x40ae3f7 => {
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
  cpu.push(0x40ae41f); f_040b4f16(cpu); if cpu.returned_to != 0x40ae41f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2au32); }
  if !cpu.zf { pc = 0x40ae42c; continue; }
  pc = 0x40ae428; continue;
 }
 0x40ae428 => {
  { let v: u8 = 0x2au8; cpu.w8(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x40ae42c; continue;
 }
 0x40ae42c => {
  { let v: u8 = 0x0u8; cpu.w8(0xffu32.wrapping_add(cpu.esi), v); }
  pc = 0x40ae433; continue;
 }
 0x40ae433 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x40ae43f; continue; }
  pc = 0x40ae43c; continue;
 }
 0x40ae43c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae433; continue;
 }
 0x40ae43f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ae448); f_040ae45f(cpu); if cpu.returned_to != 0x40ae448 { return; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40ae453; continue;
 }
 0x40ae453 => {
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
pub fn f_040ae45f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae45f); }
 let mut pc: u32 = 0x40ae45f;
 loop { match pc {
 0x40ae45f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if !cpu.zf { pc = 0x40ae4bc; continue; }
  pc = 0x40ae470; continue;
 }
 0x40ae470 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40ae486; continue; }
  pc = 0x40ae475; continue;
 }
 0x40ae475 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x65u32); }
  if cpu.zf { pc = 0x40ae486; continue; }
  pc = 0x40ae47e; continue;
 }
 0x40ae47e => {
  { cpu.sub32(cpu.edx, 0x45u32); }
  if cpu.zf { pc = 0x40ae486; continue; }
  pc = 0x40ae483; continue;
 }
 0x40ae483 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40ae470; continue;
 }
 0x40ae486 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x40ae49c; continue; }
  pc = 0x40ae497; continue;
 }
 0x40ae497 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if !cpu.zf { pc = 0x40ae4a8; continue; }
  pc = 0x40ae49c; continue;
 }
 0x40ae49c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x40ae4a8; continue; }
  pc = 0x40ae4a5; continue;
 }
 0x40ae4a5 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40ae49c; continue;
 }
 0x40ae4a8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x40ae4b2; continue; }
  pc = 0x40ae4b1; continue;
 }
 0x40ae4b1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40ae4b2; continue;
 }
 0x40ae4b2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40ae4b2; continue; }
  pc = 0x40ae4bc; continue;
 }
 0x40ae4bc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae4be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae4be); }
 let mut pc: u32 = 0x40ae4be;
 loop { match pc {
 0x40ae4be => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x40ae4c8); f_0408cccf(cpu); if cpu.returned_to != 0x40ae4c8 { return; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae4cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4cc); } r_040ae4cc(cpu, 0x40ae4cc); }
pub fn f_040ae4d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4d0); } r_040ae4cc(cpu, 0x40ae4d0); }
pub fn f_040ae4d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4d4); } r_040ae4cc(cpu, 0x40ae4d4); }
pub fn f_040ae4d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4d6); } r_040ae4cc(cpu, 0x40ae4d6); }
pub fn f_040ae4d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4d8); } r_040ae4cc(cpu, 0x40ae4d8); }
pub fn f_040ae4da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae4da); } r_040ae4cc(cpu, 0x40ae4da); }
pub fn f_040ae526(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae526); } r_040ae4cc(cpu, 0x40ae526); }
pub fn f_040ae528(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae528); } r_040ae4cc(cpu, 0x40ae528); }
pub fn f_040ae52c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae52c); } r_040ae4cc(cpu, 0x40ae52c); }
pub fn f_040ae52e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae52e); } r_040ae4cc(cpu, 0x40ae52e); }
fn r_040ae4cc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40ae4cc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40ae4d0; continue;
 }
 0x40ae4d0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40ae4d4; continue;
 }
 0x40ae4d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40ae4d6; continue;
 }
 0x40ae4d6 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ae4d8; continue;
 }
 0x40ae4d8 => {
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
  cpu.flat_ds(0x40ae53a); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40ae4da => {
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
  cpu.flat_ds(0x40ae53a); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40ae526 => {
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
  cpu.flat_ds(0x40ae53a); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40ae528 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40ae52c; continue;
 }
 0x40ae52c => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ae52e; continue;
 }
 0x40ae52e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x40ae53a); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae576(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae576); } r_040ae576(cpu, 0x40ae576); }
pub fn f_040ae57a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae57a); } r_040ae576(cpu, 0x40ae57a); }
pub fn f_040ae584(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae584); } r_040ae576(cpu, 0x40ae584); }
pub fn f_040ae5e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae5e4); } r_040ae576(cpu, 0x40ae5e4); }
pub fn f_040ae644(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae644); } r_040ae576(cpu, 0x40ae644); }
fn r_040ae576(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40ae576 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40ae57a; continue;
 }
 0x40ae57a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let a = cpu.ebx; let b = cpu.ebx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x40ae584; continue;
 }
 0x40ae584 => {
  pc = 0x40ae591; continue;
 }
 0x40ae591 => {
  pc = 0x40ae59d; continue;
 }
 0x40ae59d => {
  pc = 0x40ae5a9; continue;
 }
 0x40ae5a9 => {
  pc = 0x40ae5b5; continue;
 }
 0x40ae5b5 => {
  pc = 0x40ae5c1; continue;
 }
 0x40ae5c1 => {
  pc = 0x40ae5cd; continue;
 }
 0x40ae5cd => {
  pc = 0x40ae5d9; continue;
 }
 0x40ae5d9 => {
  pc = 0x40ae5e5; continue;
 }
 0x40ae5e4 => {
  pc = 0x40ae5f1; continue;
 }
 0x40ae5e5 => {
  pc = 0x40ae5f1; continue;
 }
 0x40ae5f1 => {
  pc = 0x40ae5fd; continue;
 }
 0x40ae5fd => {
  pc = 0x40ae609; continue;
 }
 0x40ae609 => {
  pc = 0x40ae615; continue;
 }
 0x40ae615 => {
  pc = 0x40ae621; continue;
 }
 0x40ae621 => {
  pc = 0x40ae62d; continue;
 }
 0x40ae62d => {
  pc = 0x40ae639; continue;
 }
 0x40ae639 => {
  pc = 0x40ae645; continue;
 }
 0x40ae644 => {
  { let v: u32 = cpu.eax ^ 0x27040aecu32; cpu.eax = v; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x8f040aeeu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(8)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x40ae659); return;
 }
 0x40ae645 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x8f040aeeu32.wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(8)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x40ae659); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae744(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae744); } r_040ae744(cpu, 0x40ae744); }
pub fn f_040ae844(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae844); } r_040ae744(cpu, 0x40ae844); }
fn r_040ae744(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40ae744 => {
  cpu.unimplemented(0x40ae744, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae749, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae74d, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae751, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae765, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae76d, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae771, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x28040afcu32.wrapping_add(cpu.edx.wrapping_mul(2)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae785, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae789, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae78d, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae791, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae795, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae79d, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7a5, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7ad, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7b1, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7b5, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7c5, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(2)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7cd, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0xa1040afeu32.wrapping_add(cpu.edx.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7e5, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(8)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae7f1, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae801, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae805, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebp.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0xaf40c04u32); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae811, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0xc040afbu32.wrapping_add(cpu.eax.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae819, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae821, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae825, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae82d, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae831, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae839, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40ae841, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ae85c); cpu.push(0x40ae861); f_040ae8d8(cpu); if cpu.returned_to != 0x40ae861 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x40ae864); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40ae844 => {
  {  }
  { cpu.pushad(); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ae85c); cpu.push(0x40ae861); f_040ae8d8(cpu); if cpu.returned_to != 0x40ae861 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x40ae864); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae8b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ae8b4); }
 let mut pc: u32 = 0x40ae8b4;
 loop { match pc {
 0x40ae8b4 => {
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
  cpu.flat_ds(0x40ae8cb); cpu.push(0x40ae8d0); f_040ae8d8(cpu); if cpu.returned_to != 0x40ae8d0 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  cpu.flat_ds(0x40ae8d6); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ae8d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ae8d8); } r_040ae8d8(cpu, 0x40ae8d8); }
pub fn f_040af5d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40af5d9); } r_040ae8d8(cpu, 0x40af5d9); }
pub fn f_040af7f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40af7f4); } r_040ae8d8(cpu, 0x40af7f4); }
pub fn f_040af8fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40af8fe); } r_040ae8d8(cpu, 0x40af8fe); }
pub fn f_040afa16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40afa16); } r_040ae8d8(cpu, 0x40afa16); }
pub fn f_040afa9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40afa9f); } r_040ae8d8(cpu, 0x40afa9f); }
pub fn f_040afca8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40afca8); } r_040ae8d8(cpu, 0x40afca8); }
pub fn f_040afe5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40afe5d); } r_040ae8d8(cpu, 0x40afe5d); }
fn r_040ae8d8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40ae8d8 => {
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (cpu.cf || cpu.zf) { pc = 0x40ae964; continue; }
  pc = 0x40ae8f2; continue;
 }
 0x40ae8f2 => {
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0xf8u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if !cpu.zf { pc = 0x40aea0b; continue; }
  pc = 0x40ae900; continue;
 }
 0x40ae900 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0xc0u8); }
  if !cpu.cf { pc = 0x40ae9c1; continue; }
  pc = 0x40ae911; continue;
 }
 0x40ae911 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0x1807u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x40ae584u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.flat_ds(0x40ae922); cpu.push(0x40ae92a); cpu.call(t); if cpu.returned_to != 0x40ae92a { return; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x38u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x40ae939); cpu.push(0x40ae93e); f_0409db58(cpu); if cpu.returned_to != 0x40ae93e { return; }
  { let v: u32 = 0x415c748u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi & 0x3800u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 11u32); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = cpu.r32(0x40ae644u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40ae95e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9bu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40ae8f2; continue; }
  pc = 0x40ae964; continue;
 }
 0x40ae964 => {
  if cpu.zf { pc = 0x40ae98a; continue; }
  pc = 0x40ae966; continue;
 }
 0x40ae966 => {
  { cpu.sub8((cpu.eax as u8), 0x26u8); }
  if cpu.zf { pc = 0x40ae9a8; continue; }
  pc = 0x40ae96a; continue;
 }
 0x40ae96a => {
  { cpu.sub8((cpu.eax as u8), 0x2eu8); }
  if cpu.zf { pc = 0x40ae98d; continue; }
  pc = 0x40ae96e; continue;
 }
 0x40ae96e => {
  { cpu.sub8((cpu.eax as u8), 0x36u8); }
  if cpu.zf { pc = 0x40ae99f; continue; }
  pc = 0x40ae972; continue;
 }
 0x40ae972 => {
  { cpu.sub8((cpu.eax as u8), 0x3eu8); }
  if cpu.zf { pc = 0x40ae996; continue; }
  pc = 0x40ae976; continue;
 }
 0x40ae976 => {
  { cpu.sub8((cpu.eax as u8), 0x64u8); }
  if cpu.zf { pc = 0x40ae9b1; continue; }
  pc = 0x40ae97a; continue;
 }
 0x40ae97a => {
  { cpu.sub8((cpu.eax as u8), 0x65u8); }
  if cpu.zf { pc = 0x40ae9b9; continue; }
  pc = 0x40ae97e; continue;
 }
 0x40ae97e => {
  { cpu.sub8((cpu.eax as u8), 0x66u8); }
  if cpu.zf { pc = 0x40ae98a; continue; }
  pc = 0x40ae982; continue;
 }
 0x40ae982 => {
  { cpu.sub8((cpu.eax as u8), 0x67u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40ae8f2; continue; }
  pc = 0x40ae98a; continue;
 }
 0x40ae98a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae98d => {
  { let v: u16 = cpu.r16(0x3cu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae996 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae99f => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae9a8 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae9b1 => {
  { let v: u16 = (cpu.segments[4] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae9b9 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40ae95e; continue;
 }
 0x40ae9c1 => {
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ae9c4); cpu.push(0x40ae9c9); f_0409db58(cpu); if cpu.returned_to != 0x40ae9c9 { return; }
  { let v: u32 = 0x415c748u32; cpu.ebp = v; }
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
  if !cpu.zf { pc = 0x40ae9f4; continue; }
  pc = 0x40ae9f0; continue;
 }
 0x40ae9f0 => {
  { let v: u16 = 0x1u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40ae9f4; continue;
 }
 0x40ae9f4 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.esi & 0x7u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = cpu.r32(0x40ae744u32.wrapping_add(cpu.edx.wrapping_mul(4))); continue;
 }
 0x40aea0b => {
  cpu.flat_ds(0x40aea0b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aea7a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x40ae576u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40aea91; continue;
 }
 0x40aea91 => {
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aeacc => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) | cpu.r8(0x40ae576u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aec30 => {
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aec35 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aec3b); cpu.push(0x40aec40); f_040b0826(cpu); if cpu.returned_to != 0x40aec40 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aec4b); cpu.push(0x40aec50); f_040afffc(cpu); if cpu.returned_to != 0x40aec50 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aec54 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aec5a); cpu.push(0x40aec5f); f_040b0826(cpu); if cpu.returned_to != 0x40aec5f { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aec6a); cpu.push(0x40aec6f); f_040b0430(cpu); if cpu.returned_to != 0x40aec6f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aec73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aec79); cpu.push(0x40aec7e); f_040b0826(cpu); if cpu.returned_to != 0x40aec7e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aec85); cpu.push(0x40aec8a); f_040b01c2(cpu); if cpu.returned_to != 0x40aec8a { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40aec8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aec95); cpu.push(0x40aec9a); f_040b0826(cpu); if cpu.returned_to != 0x40aec9a { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aeca1); cpu.push(0x40aeca6); f_040b01c2(cpu); if cpu.returned_to != 0x40aeca6 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40aecab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aecb1); cpu.push(0x40aecb6); f_040b0826(cpu); if cpu.returned_to != 0x40aecb6 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aecc1); cpu.push(0x40aecc6); f_040affee(cpu); if cpu.returned_to != 0x40aecc6 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aecca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aecd0); cpu.push(0x40aecd5); f_040b0826(cpu); if cpu.returned_to != 0x40aecd5 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aece0); cpu.push(0x40aece5); f_040affee(cpu); if cpu.returned_to != 0x40aece5 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aece9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aecef); cpu.push(0x40aecf4); f_040b0826(cpu); if cpu.returned_to != 0x40aecf4 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aecff); cpu.push(0x40aed04); f_040b025b(cpu); if cpu.returned_to != 0x40aed04 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aed08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aed0e); cpu.push(0x40aed13); f_040b0826(cpu); if cpu.returned_to != 0x40aed13 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aed1e); cpu.push(0x40aed23); f_040b025b(cpu); if cpu.returned_to != 0x40aed23 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aed27 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40aed57); cpu.push(0x40aed5c); f_040b0826(cpu); if cpu.returned_to != 0x40aed5c { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aed61 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40aed65); cpu.push(0x40aed6a); f_040b06e0(cpu); if cpu.returned_to != 0x40aed6a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aed72 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40aed76); cpu.push(0x40aed7b); f_040b06e0(cpu); if cpu.returned_to != 0x40aed7b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40aed83 => {
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
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aeda1 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16) & 0x300u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x300u16); }
  if !cpu.zf { pc = 0x40aedc0; continue; }
  pc = 0x40aedb4; continue;
 }
 0x40aedb4 => {
  { let v: u32 = 0x40ae8d8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40aeddc; continue;
 }
 0x40aedc0 => {
  { cpu.sub16((cpu.eax as u16), 0x200u16); }
  if !cpu.zf { pc = 0x40aedd2; continue; }
  pc = 0x40aedc6; continue;
 }
 0x40aedc6 => {
  { let v: u32 = 0x40aea48u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40aeddc; continue;
 }
 0x40aedd2 => {
  { let v: u32 = 0x40aea0cu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40aeddc; continue;
 }
 0x40aeddc => {
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aede1 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aedf5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aee03 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee09); cpu.push(0x40aee0e); f_040b061a(cpu); if cpu.returned_to != 0x40aee0e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aee19); cpu.push(0x40aee1e); f_040afffc(cpu); if cpu.returned_to != 0x40aee1e { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aee22 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee28); cpu.push(0x40aee2d); f_040b061a(cpu); if cpu.returned_to != 0x40aee2d { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aee38); cpu.push(0x40aee3d); f_040b0430(cpu); if cpu.returned_to != 0x40aee3d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aee41 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee47); cpu.push(0x40aee4c); f_040b061a(cpu); if cpu.returned_to != 0x40aee4c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee53); cpu.push(0x40aee58); f_040b01c2(cpu); if cpu.returned_to != 0x40aee58 { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40aee5d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee63); cpu.push(0x40aee68); f_040b061a(cpu); if cpu.returned_to != 0x40aee68 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee6f); cpu.push(0x40aee74); f_040b01c2(cpu); if cpu.returned_to != 0x40aee74 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40aee79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee7f); cpu.push(0x40aee84); f_040b061a(cpu); if cpu.returned_to != 0x40aee84 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aee8f); cpu.push(0x40aee94); f_040affee(cpu); if cpu.returned_to != 0x40aee94 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aee98 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aee9e); cpu.push(0x40aeea3); f_040b061a(cpu); if cpu.returned_to != 0x40aeea3 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aeeae); cpu.push(0x40aeeb3); f_040affee(cpu); if cpu.returned_to != 0x40aeeb3 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aeeb7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aeebd); cpu.push(0x40aeec2); f_040b061a(cpu); if cpu.returned_to != 0x40aeec2 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aeecd); cpu.push(0x40aeed2); f_040b025b(cpu); if cpu.returned_to != 0x40aeed2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aeed6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aeedc); cpu.push(0x40aeee1); f_040b061a(cpu); if cpu.returned_to != 0x40aeee1 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aeeec); cpu.push(0x40aeef1); f_040b025b(cpu); if cpu.returned_to != 0x40aeef1 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aeef5 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40aef25); cpu.push(0x40aef2a); f_040b061a(cpu); if cpu.returned_to != 0x40aef2a { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aef2f => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40aef49); cpu.push(0x40aef4e); f_040b08eb(cpu); if cpu.returned_to != 0x40aef4e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40aef52); cpu.push(0x40aef57); f_040b05ab(cpu); if cpu.returned_to != 0x40aef57 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aef6b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40aef79); cpu.push(0x40aef7e); f_040b08eb(cpu); if cpu.returned_to != 0x40aef7e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40aef82); cpu.push(0x40aef87); f_040b05ab(cpu); if cpu.returned_to != 0x40aef87 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40aef8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40aef99); cpu.push(0x40aef9e); f_040b0660(cpu); if cpu.returned_to != 0x40aef9e { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aefa9); cpu.push(0x40aefae); f_040afffc(cpu); if cpu.returned_to != 0x40aefae { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aefb2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40aefbc); cpu.push(0x40aefc1); f_040b0660(cpu); if cpu.returned_to != 0x40aefc1 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40aefcc); cpu.push(0x40aefd1); f_040b0430(cpu); if cpu.returned_to != 0x40aefd1 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aefd5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40aefdf); cpu.push(0x40aefe4); f_040b0660(cpu); if cpu.returned_to != 0x40aefe4 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40aefeb); cpu.push(0x40aeff0); f_040b01c2(cpu); if cpu.returned_to != 0x40aeff0 { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40aeff5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40aefff); cpu.push(0x40af004); f_040b0660(cpu); if cpu.returned_to != 0x40af004 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af00b); cpu.push(0x40af010); f_040b01c2(cpu); if cpu.returned_to != 0x40af010 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40af015 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40af01f); cpu.push(0x40af024); f_040b0660(cpu); if cpu.returned_to != 0x40af024 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af02f); cpu.push(0x40af034); f_040affee(cpu); if cpu.returned_to != 0x40af034 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af038 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40af042); cpu.push(0x40af047); f_040b0660(cpu); if cpu.returned_to != 0x40af047 { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af052); cpu.push(0x40af057); f_040affee(cpu); if cpu.returned_to != 0x40af057 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af05b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40af065); cpu.push(0x40af06a); f_040b0660(cpu); if cpu.returned_to != 0x40af06a { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af075); cpu.push(0x40af07a); f_040b025b(cpu); if cpu.returned_to != 0x40af07a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af07e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  cpu.flat_ds(0x40af088); cpu.push(0x40af08d); f_040b0660(cpu); if cpu.returned_to != 0x40af08d { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af098); cpu.push(0x40af09d); f_040b025b(cpu); if cpu.returned_to != 0x40af09d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af0a1 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af0e8 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af11c); cpu.push(0x40af121); f_040b0660(cpu); if cpu.returned_to != 0x40af121 { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af126 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af12a); cpu.push(0x40af12f); f_040b075f(cpu); if cpu.returned_to != 0x40af12f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af13b => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40af159 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af15d); cpu.push(0x40af162); f_040b075f(cpu); if cpu.returned_to != 0x40af162 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40af16e => {
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
  pc = 0x40af19c; continue;
 }
 0x40af19c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x40af19c; continue; }
  pc = 0x40af1ae; continue;
 }
 0x40af1ae => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af1b5 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  pc = 0x40af1c7; continue;
 }
 0x40af1c7 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.ebx)) as u32; cpu.ebx = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x40af1c7; continue; }
  pc = 0x40af1d9; continue;
 }
 0x40af1d9 => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40afacb; continue;
 }
 0x40af1e0 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af1ee => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af1f6); cpu.push(0x40af1fb); f_040b061a(cpu); if cpu.returned_to != 0x40af1fb { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af206); cpu.push(0x40af20b); f_040afffc(cpu); if cpu.returned_to != 0x40af20b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af20f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af217); cpu.push(0x40af21c); f_040b061a(cpu); if cpu.returned_to != 0x40af21c { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af227); cpu.push(0x40af22c); f_040b0430(cpu); if cpu.returned_to != 0x40af22c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af230 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af238); cpu.push(0x40af23d); f_040b061a(cpu); if cpu.returned_to != 0x40af23d { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af244); cpu.push(0x40af249); f_040b01c2(cpu); if cpu.returned_to != 0x40af249 { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40af24e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af256); cpu.push(0x40af25b); f_040b061a(cpu); if cpu.returned_to != 0x40af25b { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af262); cpu.push(0x40af267); f_040b01c2(cpu); if cpu.returned_to != 0x40af267 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40af26c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af274); cpu.push(0x40af279); f_040b061a(cpu); if cpu.returned_to != 0x40af279 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af284); cpu.push(0x40af289); f_040affee(cpu); if cpu.returned_to != 0x40af289 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af28d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af295); cpu.push(0x40af29a); f_040b061a(cpu); if cpu.returned_to != 0x40af29a { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af2a5); cpu.push(0x40af2aa); f_040affee(cpu); if cpu.returned_to != 0x40af2aa { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af2ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af2b6); cpu.push(0x40af2bb); f_040b061a(cpu); if cpu.returned_to != 0x40af2bb { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af2c6); cpu.push(0x40af2cb); f_040b025b(cpu); if cpu.returned_to != 0x40af2cb { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af2cf => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af2d7); cpu.push(0x40af2dc); f_040b061a(cpu); if cpu.returned_to != 0x40af2dc { return; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af2e7); cpu.push(0x40af2ec); f_040b025b(cpu); if cpu.returned_to != 0x40af2ec { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af2f0 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af322); cpu.push(0x40af327); f_040b061a(cpu); if cpu.returned_to != 0x40af327 { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af32c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40af346); cpu.push(0x40af34b); f_040b08eb(cpu); if cpu.returned_to != 0x40af34b { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af34f); cpu.push(0x40af354); f_040b05ab(cpu); if cpu.returned_to != 0x40af354 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x40af35f; continue; }
  pc = 0x40af35b; continue;
 }
 0x40af35b => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40af35f; continue;
 }
 0x40af35f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af374 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40af382); cpu.push(0x40af387); f_040b08eb(cpu); if cpu.returned_to != 0x40af387 { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af38b); cpu.push(0x40af390); f_040b05ab(cpu); if cpu.returned_to != 0x40af390 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x40af39b; continue; }
  pc = 0x40af397; continue;
 }
 0x40af397 => {
  { let v: u16 = 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40af39b; continue;
 }
 0x40af39b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40af3a4 => {
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af3a9 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af3d9); cpu.push(0x40af3de); f_040b062b(cpu); if cpu.returned_to != 0x40af3de { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af3e3 => {
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40af3e8 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40af3f6); cpu.push(0x40af3fb); f_040b08eb(cpu); if cpu.returned_to != 0x40af3fb { return; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af3ff); cpu.push(0x40af404); f_040b05b0(cpu); if cpu.returned_to != 0x40af404 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40af40c => {
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af411 => {
  cpu.poll(); pc = cpu.r32(0x40af419u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40af439 => {
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af43e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af44a); cpu.push(0x40af44f); f_040afffc(cpu); if cpu.returned_to != 0x40af44f { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af453 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af45f); cpu.push(0x40af464); f_040b0430(cpu); if cpu.returned_to != 0x40af464 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af468 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af470); cpu.push(0x40af475); f_040b01c2(cpu); if cpu.returned_to != 0x40af475 { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40af47a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af482); cpu.push(0x40af487); f_040b01c2(cpu); if cpu.returned_to != 0x40af487 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40af48c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af498); cpu.push(0x40af49d); f_040affee(cpu); if cpu.returned_to != 0x40af49d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af4a1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af4ad); cpu.push(0x40af4b2); f_040affee(cpu); if cpu.returned_to != 0x40af4b2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af4b6 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af4c2); cpu.push(0x40af4c7); f_040b025b(cpu); if cpu.returned_to != 0x40af4c7 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af4cb => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40af4d7); cpu.push(0x40af4dc); f_040b025b(cpu); if cpu.returned_to != 0x40af4dc { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af4e0 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af528 => {
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
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af550 => {
  cpu.poll(); pc = cpu.r32(0x40af558u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40af578 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af585 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af592 => {
  { let v: u32 = cpu.r32(0x40ae4ccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x40ae4d0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae4d4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.flat_ds(0x40af5b9); cpu.push(0x40af5be); f_040b01c2(cpu); if cpu.returned_to != 0x40af5be { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40af5c3 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af5c7); cpu.push(0x40af5cc); f_040b0d72(cpu); if cpu.returned_to != 0x40af5cc { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af5d1 => {
  cpu.poll(); pc = cpu.r32(0x40af5d9u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40af5d9 => {
  { cpu.cf = true; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0x4u8); }
  { let v: u32 = cpu.esi ^ 0xf6c6040au32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0xaf75004u32); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x95u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3])), 0xaf40c04u32); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af5f9 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af63c => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xcd1b8afeu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xd49a784bu32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af681 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x5c17f0bcu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb8aa3b29u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af6c6 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xc900daa2u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af70b => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xfbcff799u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x9a209a84u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffdu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af750 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = 0xd1cf79acu32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xb17217f7u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3ffeu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af795 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae528u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af7ec => {
  cpu.poll(); pc = cpu.r32(0x40af7f4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40af7f4 => {
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0xc6040afeu32.wrapping_add(cpu.edi.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.dec8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xb4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = false; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x8d040af8u32.wrapping_add(cpu.edx.wrapping_mul(8)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  cpu.unimplemented(0x40af816, "Das");
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0xe8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.pf() { pc = 0x40af836; continue; }
  pc = 0x40af81b; continue;
 }
 0x40af814 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af818); cpu.push(0x40af81d); f_040b1398(cpu); if cpu.returned_to != 0x40af81d { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af81b => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af821 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af85f); cpu.push(0x40af864); f_040b0e79(cpu); if cpu.returned_to != 0x40af864 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af836 => {
  { let v: u32 = cpu.port_in(0xau16, 4) as u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af85f); cpu.push(0x40af864); f_040b0e79(cpu); if cpu.returned_to != 0x40af864 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af868 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af86c); cpu.push(0x40af871); f_040b1362(cpu); if cpu.returned_to != 0x40af871 { return; }
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af8b4 => {
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af8d5 => {
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af8f6 => {
  cpu.poll(); pc = cpu.r32(0x40af8feu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40af8fe => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.cf = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x6d040affu32.wrapping_add(cpu.edx.wrapping_mul(2)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xf4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x8d1c2f44u32.wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0xe8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.adc16((cpu.eax as u16), cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x663e07e0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x663e07e0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x40ae57au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af91e => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af926); cpu.push(0x40af92b); f_040b0c91(cpu); if cpu.returned_to != 0x40af92b { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x40ae57au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af948 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40af986); cpu.push(0x40af98b); f_040b0e60(cpu); if cpu.returned_to != 0x40af98b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af98f => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af993); cpu.push(0x40af998); f_040b156c(cpu); if cpu.returned_to != 0x40af998 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40af99c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v = cpu.r32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af9ac); cpu.push(0x40af9b1); f_040b134b(cpu); if cpu.returned_to != 0x40af9b1 { return; }
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af9ea); cpu.push(0x40af9ef); f_040b1355(cpu); if cpu.returned_to != 0x40af9ef { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40af9f4 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40af9f8); cpu.push(0x40af9fd); f_040b134b(cpu); if cpu.returned_to != 0x40af9fd { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afa01 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.flat_ds(0x40afa05); cpu.push(0x40afa0a); f_040b1355(cpu); if cpu.returned_to != 0x40afa0a { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afa0e => {
  cpu.poll(); pc = cpu.r32(0x40afa16u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40afa16 => {
  { let v: u8 = (cpu.eax as u8) | 0xf4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa1f, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa23, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa27, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa2b, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa2f, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afa33, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  cpu.unimplemented(0x40afa38, "Das");
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x3eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afa8d); cpu.push(0x40afa92); f_040b01c2(cpu); if cpu.returned_to != 0x40afa92 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40afa36 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afa8d); cpu.push(0x40afa92); f_040b01c2(cpu); if cpu.returned_to != 0x40afa92 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40afa97 => {
  cpu.poll(); pc = cpu.r32(0x40afa9fu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40afa9f => {
  { let v: u8 = (cpu.eax as u8) | 0xf4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afaa4, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edi.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afab0, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afab4, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afab8, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afabc, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp)) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40afabf => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0x7f00u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40afacb => {
  { let v: u16 = 0x33fu16; cpu.w16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x40ae8d8u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40afaf1 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afafd); cpu.push(0x40afb02); f_040afffc(cpu); if cpu.returned_to != 0x40afb02 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb06 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afb12); cpu.push(0x40afb17); f_040b0430(cpu); if cpu.returned_to != 0x40afb17 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb1b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afb27); cpu.push(0x40afb2c); f_040affee(cpu); if cpu.returned_to != 0x40afb2c { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb30 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afb3c); cpu.push(0x40afb41); f_040affee(cpu); if cpu.returned_to != 0x40afb41 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb45 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afb51); cpu.push(0x40afb56); f_040b025b(cpu); if cpu.returned_to != 0x40afb56 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb5a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afb66); cpu.push(0x40afb6b); f_040b025b(cpu); if cpu.returned_to != 0x40afb6b { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afb6f => {
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40afb8e => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40afbba => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afbc2); cpu.push(0x40afbc7); f_040b01c2(cpu); if cpu.returned_to != 0x40afbc7 { return; }
  cpu.poll(); pc = 0x40aeacc; continue;
 }
 0x40afbcc => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afbd4); cpu.push(0x40afbd9); f_040b01c2(cpu); if cpu.returned_to != 0x40afbd9 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40afbde => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40aea91; continue;
 }
 0x40afc0a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afc16); cpu.push(0x40afc1b); f_040afffc(cpu); if cpu.returned_to != 0x40afc1b { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afc55 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afc61); cpu.push(0x40afc66); f_040b0430(cpu); if cpu.returned_to != 0x40afc66 { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afca0 => {
  cpu.poll(); pc = cpu.r32(0x40afca8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40afca8 => {
  { let v: u8 = (cpu.eax as u8) | 0xf4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(8)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcb1, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcb5, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcb9, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcbd, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcc1, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afcc5, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  cpu.unimplemented(0x40afcca, "Das");
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x3eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afd1f); cpu.push(0x40afd24); f_040b01c2(cpu); if cpu.returned_to != 0x40afd24 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40afcc8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x70u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x74u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x6cu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40afd1f); cpu.push(0x40afd24); f_040b01c2(cpu); if cpu.returned_to != 0x40afd24 { return; }
  cpu.poll(); pc = 0x40aea7a; continue;
 }
 0x40afd29 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afd35); cpu.push(0x40afd3a); f_040affee(cpu); if cpu.returned_to != 0x40afd3a { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afd74 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afd80); cpu.push(0x40afd85); f_040affee(cpu); if cpu.returned_to != 0x40afd85 { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afdbf => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afdcb); cpu.push(0x40afdd0); f_040b025b(cpu); if cpu.returned_to != 0x40afdd0 { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afe0a => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  cpu.flat_ds(0x40afe16); cpu.push(0x40afe1b); f_040b025b(cpu); if cpu.returned_to != 0x40afe1b { return; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afe55 => {
  cpu.poll(); pc = cpu.r32(0x40afe5du32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40afe5d => {
  { let v = cpu.st(7) / cpu.st(0); cpu.set_st(7, v); cpu.fpop(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe62, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe66, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe6a, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe6e, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe72, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe76, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40afe7a, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x2e1c2f54u32.wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x40afebd); cpu.push(0x40afec2); f_040b0a78(cpu); if cpu.returned_to != 0x40afec2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afe7d => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.ebx as u16)); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.flat_ds(0x40afebd); cpu.push(0x40afec2); f_040b0a78(cpu); if cpu.returned_to != 0x40afec2 { return; }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40afec6 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x40ae4d8u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x40ae4dau32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) & 0xc7ffu16; cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.ebx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x40ae526u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
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
  cpu.flat_ds(0x40aff21); cpu.push(0x40aff26); f_040b05ab(cpu); if cpu.returned_to != 0x40aff26 { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aff2b => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  cpu.flat_ds(0x40aff33); cpu.push(0x40aff38); f_040b0c91(cpu); if cpu.returned_to != 0x40aff38 { return; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xb8ffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), cpu.r8(0x40ae57au32.wrapping_add(cpu.eax))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); continue;
 }
 0x40aff55 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40aff63); cpu.push(0x40aff68); f_040b08eb(cpu); if cpu.returned_to != 0x40aff68 { return; }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40aff6d => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r16(0x40ae4d6u32.wrapping_add(cpu.edi)) as u32; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.flat_ds(0x40aff7f); cpu.push(0x40aff84); f_040b05ab(cpu); if cpu.returned_to != 0x40aff84 { return; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x40affac; continue; }
  pc = 0x40aff91; continue;
 }
 0x40aff91 => {
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x40aff9d; continue; }
  pc = 0x40aff95; continue;
 }
 0x40aff95 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40affaa; continue;
 }
 0x40aff9d => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40affaa; continue;
 }
 0x40affaa => {
  pc = 0x40affd2; continue;
 }
 0x40affac => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x8000u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edx as u16), (cpu.eax as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x40affd2; continue; }
  pc = 0x40affc2; continue;
 }
 0x40affc2 => {
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x40affce; continue; }
  pc = 0x40affc6; continue;
 }
 0x40affc6 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40affd2; continue;
 }
 0x40affce => {
  { let v: u16 = 0x7fffu16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x40affd2; continue;
 }
 0x40affd2 => {
  { let v: u16 = (cpu.edx as u16) | (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 0x40affde => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x40ae8d8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aea0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aea0c); }
 let mut pc: u32 = 0x40aea0c;
 loop { match pc {
 0x40aea0c => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 25u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x40aea3a; continue; }
  pc = 0x40aea1d; continue;
 }
 0x40aea1d => {
  if !cpu.zf { pc = 0x40aea26; continue; }
  pc = 0x40aea1f; continue;
 }
 0x40aea1f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x40aea26; continue; }
  pc = 0x40aea24; continue;
 }
 0x40aea24 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x40aea26; continue;
 }
 0x40aea26 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x100u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x40aea3a; continue; }
  pc = 0x40aea2f; continue;
 }
 0x40aea2f => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40aea3a; continue;
 }
 0x40aea3a => {
  { let v: u32 = cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_040ae8d8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aea48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aea48); }
 let mut pc: u32 = 0x40aea48;
 loop { match pc {
 0x40aea48 => {
  { let v: u32 = 0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = 0xfffff800u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 22u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x40aea73; continue; }
  pc = 0x40aea58; continue;
 }
 0x40aea58 => {
  if !cpu.zf { pc = 0x40aea5c; continue; }
  pc = 0x40aea5a; continue;
 }
 0x40aea5a => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x40aea5c; continue;
 }
 0x40aea5c => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x800u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  if !cpu.cf { pc = 0x40aea73; continue; }
  pc = 0x40aea68; continue;
 }
 0x40aea68 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.ebx))); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40aea73; continue;
 }
 0x40aea73 => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  f_040ae8d8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeae8); }
 let mut pc: u32 = 0x40aeae8;
 loop { match pc {
 0x40aeae8 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])) | (cpu.eax as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3]), v); }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40aeb35; continue; }
  pc = 0x40aeaf2; continue;
 }
 0x40aeaf2 => {
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
  let t = cpu.r32(0x415b845u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40aeb0f); cpu.push(0x40aeb15); cpu.call(t); if cpu.returned_to != 0x40aeb15 { return; }
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
  pc = 0x40aeb35; continue;
 }
 0x40aeb35 => {
  cpu.flat_ds(0x40aeb35); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb36); }
 let mut pc: u32 = 0x40aeb36;
 loop { match pc {
 0x40aeb36 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb3c); }
 let mut pc: u32 = 0x40aeb3c;
 loop { match pc {
 0x40aeb3c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb42); }
 let mut pc: u32 = 0x40aeb42;
 loop { match pc {
 0x40aeb42 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb48); }
 let mut pc: u32 = 0x40aeb48;
 loop { match pc {
 0x40aeb48 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb4e); }
 let mut pc: u32 = 0x40aeb4e;
 loop { match pc {
 0x40aeb4e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb56); }
 let mut pc: u32 = 0x40aeb56;
 loop { match pc {
 0x40aeb56 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb5c); }
 let mut pc: u32 = 0x40aeb5c;
 loop { match pc {
 0x40aeb5c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb62); }
 let mut pc: u32 = 0x40aeb62;
 loop { match pc {
 0x40aeb62 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb68); }
 let mut pc: u32 = 0x40aeb68;
 loop { match pc {
 0x40aeb68 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb70); }
 let mut pc: u32 = 0x40aeb70;
 loop { match pc {
 0x40aeb70 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb78); }
 let mut pc: u32 = 0x40aeb78;
 loop { match pc {
 0x40aeb78 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb80); }
 let mut pc: u32 = 0x40aeb80;
 loop { match pc {
 0x40aeb80 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb88); }
 let mut pc: u32 = 0x40aeb88;
 loop { match pc {
 0x40aeb88 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb92); }
 let mut pc: u32 = 0x40aeb92;
 loop { match pc {
 0x40aeb92 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeb9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeb9c); }
 let mut pc: u32 = 0x40aeb9c;
 loop { match pc {
 0x40aeb9c => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aeba4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aeba4); }
 let mut pc: u32 = 0x40aeba4;
 loop { match pc {
 0x40aeba4 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as i8 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebac); }
 let mut pc: u32 = 0x40aebac;
 loop { match pc {
 0x40aebac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebb5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebb5); }
 let mut pc: u32 = 0x40aebb5;
 loop { match pc {
 0x40aebb5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x24u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebbe); }
 let mut pc: u32 = 0x40aebbe;
 loop { match pc {
 0x40aebbe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebc7); }
 let mut pc: u32 = 0x40aebc7;
 loop { match pc {
 0x40aebc7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebd0); }
 let mut pc: u32 = 0x40aebd0;
 loop { match pc {
 0x40aebd0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebdb); }
 let mut pc: u32 = 0x40aebdb;
 loop { match pc {
 0x40aebdb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebe6); }
 let mut pc: u32 = 0x40aebe6;
 loop { match pc {
 0x40aebe6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebef); }
 let mut pc: u32 = 0x40aebef;
 loop { match pc {
 0x40aebef => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aebf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aebf8); }
 let mut pc: u32 = 0x40aebf8;
 loop { match pc {
 0x40aebf8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) & 0x7u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 3u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0x18u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  let t = cpu.r32(0x40ae5e4u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.push(0x40aec13); cpu.call(t); if cpu.returned_to != 0x40aec13 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x38u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x40aec2e; continue; }
  pc = 0x40aec1f; continue;
 }
 0x40aec1f => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.edx = v; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  pc = 0x40aec2e; continue;
 }
 0x40aec2e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040af419(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40af419); }
 let mut pc: u32 = 0x40af419;
 loop { match pc {
 0x40af419 => {
  { cpu.sub32(cpu.esp, cpu.esi); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af41e, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af422, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af426, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af42a, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af42e, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af432, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af436, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(8)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x40af441); cpu.call(0x0); if cpu.returned_to != 0x40af441 { return; }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8du8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x8du8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  cpu.unimplemented(0x40af448, "Das");
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0xe8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), ((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040af558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40af558); }
 let mut pc: u32 = 0x40af558;
 loop { match pc {
 0x40af54f => {
  cpu.poll(); pc = cpu.r32(0x0u32.wrapping_add(cpu.esi)); continue;
 }
 0x40af558 => {
  if cpu.sf { cpu.poll(); pc = 0x40af54f; continue; }
  pc = 0x40af55a; continue;
 }
 0x40af55a => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0xc040af5u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af561, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af565, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(8)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.cf = !cpu.cf; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af571, "Hlt");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40af575, "Hlt");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x801c2f44u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x40af588; continue; }
  pc = 0x40af57f; continue;
 }
 0x40af57f => {
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x53u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.call(0x40af582); return;
 }
 0x40af588 => {
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x80u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.pushad(); }
  { let v: u32 = cpu.or32(cpu.r32(0xffffffe9u32.wrapping_add(cpu.edi)), cpu.edi); cpu.w32(0xffffffe9u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.call(0x40af58f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040affee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40affee); } r_040affee(cpu, 0x40affee); }
pub fn f_040afffc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40afffc); } r_040affee(cpu, 0x40afffc); }
fn r_040affee(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40affee => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b0003; continue;
 }
 0x40afffc => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b0003; continue;
 }
 0x40b0003 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40b0019); f_040b0056(cpu); if cpu.returned_to != 0x40b0019 { return; }
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
pub fn f_040b0026(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0026); }
 let mut pc: u32 = 0x40b0026;
 loop { match pc {
 0x40b0026 => {
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
  cpu.push(0x40b0047); f_040b0056(cpu); if cpu.returned_to != 0x40b0047 { return; }
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
pub fn f_040b0056(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0056); }
 let mut pc: u32 = 0x40b0056;
 loop { match pc {
 0x40b0056 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b0079; continue; }
  pc = 0x40b005a; continue;
 }
 0x40b005a => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b0079; continue; }
  pc = 0x40b005e; continue;
 }
 0x40b005e => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0076; continue; }
  pc = 0x40b0063; continue;
 }
 0x40b0063 => {
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16) | (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40b0075; continue; }
  pc = 0x40b0073; continue;
 }
 0x40b0073 => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x40b0075; continue;
 }
 0x40b0075 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0076 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b0079; continue;
 }
 0x40b0079 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40b008a; continue; }
  pc = 0x40b007d; continue;
 }
 0x40b007d => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x40b008a; continue; }
  pc = 0x40b0081; continue;
 }
 0x40b0081 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x40b008a; continue; }
  pc = 0x40b0089; continue;
 }
 0x40b0089 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b008a => {
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
  if cpu.zf { pc = 0x40b00d9; continue; }
  pc = 0x40b00bb; continue;
 }
 0x40b00bb => {
  if !cpu.cf { pc = 0x40b00c5; continue; }
  pc = 0x40b00bd; continue;
 }
 0x40b00bd => {
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.esi; let b = cpu.edx; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x40b00c5; continue;
 }
 0x40b00c5 => {
  { cpu.sub16((cpu.ecx as u16), 0x40u16); }
  if (cpu.cf || cpu.zf) { pc = 0x40b00d9; continue; }
  pc = 0x40b00cb; continue;
 }
 0x40b00cb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b00d9 => {
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.sf { pc = 0x40b00ee; continue; }
  pc = 0x40b00df; continue;
 }
 0x40b00df => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, 0x0u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x40b00ee; continue;
 }
 0x40b00ee => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b0121; continue; }
  pc = 0x40b00f5; continue;
 }
 0x40b00f5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x40b0113; continue; }
  pc = 0x40b00fd; continue;
 }
 0x40b00fd => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x40b010f; continue; }
  pc = 0x40b010b; continue;
 }
 0x40b010b => {
  { let v: u32 = cpu.edi | cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x40b010f; continue;
 }
 0x40b010f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x40b0113; continue;
 }
 0x40b0113 => {
  { let v: u32 = cpu.shrd32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.edi | cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x40b0121; continue;
 }
 0x40b0121 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x40b0152; continue; }
  pc = 0x40b012a; continue;
 }
 0x40b012a => {
  { cpu.sub8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x40b0143; continue; }
  pc = 0x40b012f; continue;
 }
 0x40b012f => {
  { cpu.and32(cpu.edi, 0x7fffffffu32); }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = cpu.adc8(((cpu.ecx >> 8) as u8), 0x0u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b0143; continue;
 }
 0x40b0143 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ 0x80000000u32; cpu.ebp = v; }
  pc = 0x40b0152; continue;
 }
 0x40b0152 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40b01a9; continue; }
  pc = 0x40b015a; continue;
 }
 0x40b015a => {
  { let v: u16 = cpu.or16((cpu.ebp as u16), (cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b01ab; continue; }
  pc = 0x40b015f; continue;
 }
 0x40b015f => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x0u8); }
  if !cpu.zf { pc = 0x40b0172; continue; }
  pc = 0x40b0164; continue;
 }
 0x40b0164 => {
  { let v: u32 = cpu.rol32(cpu.edi, 1u32); cpu.edi = v; }
  { let v: u32 = cpu.ror32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x40b0168; continue;
 }
 0x40b0168 => {
  { let v: u16 = cpu.dec16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b01ab; continue; }
  pc = 0x40b016c; continue;
 }
 0x40b016c => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x40b0168; continue; }
  pc = 0x40b0172; continue;
 }
 0x40b0172 => {
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x40b01b5; continue; }
  pc = 0x40b017f; continue;
 }
 0x40b017f => {
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x40b01a7; continue; }
  pc = 0x40b0186; continue;
 }
 0x40b0186 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  if !cpu.zf { pc = 0x40b018e; continue; }
  pc = 0x40b018a; continue;
 }
 0x40b018a => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b018e; continue;
 }
 0x40b018e => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x40b01a7; continue; }
  pc = 0x40b0196; continue;
 }
 0x40b0196 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ebp as u16)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ebp as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x40b01b5; continue; }
  pc = 0x40b01a7; continue;
 }
 0x40b01a7 => {
  pc = 0x40b01ab; continue;
 }
 0x40b01a9 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x40b01ab; continue;
 }
 0x40b01ab => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.rcr16((cpu.ebp as u16), 1u32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b01b5 => {
  { let v: u16 = 0x7fffu16; cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x40b01ab; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b01c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b01c2); }
 let mut pc: u32 = 0x40b01c2;
 loop { match pc {
 0x40b01c2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b01da; continue; }
  pc = 0x40b01d1; continue;
 }
 0x40b01d1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.esi = v; }
  if !cpu.zf { pc = 0x40b022d; continue; }
  pc = 0x40b01da; continue;
 }
 0x40b01da => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16) | 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.inc16((cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b01f0; continue; }
  pc = 0x40b01e7; continue;
 }
 0x40b01e7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.esi = v; }
  if !cpu.zf { pc = 0x40b022d; continue; }
  pc = 0x40b01f0; continue;
 }
 0x40b01f0 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.edi, cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.sf { pc = 0x40b0235; continue; }
  pc = 0x40b01ff; continue;
 }
 0x40b01ff => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.esi as u16), cpu.r16(0x8u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x40b0215; continue; }
  pc = 0x40b0209; continue;
 }
 0x40b0209 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x40b0215; continue; }
  pc = 0x40b0211; continue;
 }
 0x40b0211 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  pc = 0x40b0215; continue;
 }
 0x40b0215 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  if cpu.zf { pc = 0x40b0228; continue; }
  pc = 0x40b021c; continue;
 }
 0x40b021c => {
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.eax; cpu.esi = v; }
  pc = 0x40b0220; continue;
 }
 0x40b0220 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, 0x0u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x40b0228; continue;
 }
 0x40b0228 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b022d => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0235 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x40b0220; continue; }
  pc = 0x40b0258; continue;
 }
 0x40b0258 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b025b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b025b); }
 let mut pc: u32 = 0x40b025b;
 loop { match pc {
 0x40b025b => {
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
  cpu.push(0x40b0278); f_040b0285(cpu); if cpu.returned_to != 0x40b0278 { return; }
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
pub fn f_040b0285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0285); }
 let mut pc: u32 = 0x40b0285;
 loop { match pc {
 0x40b0285 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40b02d0; continue; }
  pc = 0x40b0289; continue;
 }
 0x40b0289 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x40b02d0; continue; }
  pc = 0x40b028d; continue;
 }
 0x40b028d => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x40b02d0; continue; }
  pc = 0x40b0295; continue;
 }
 0x40b0295 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b02ba; continue; }
  pc = 0x40b0299; continue;
 }
 0x40b0299 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b02ba; continue; }
  pc = 0x40b029d; continue;
 }
 0x40b029d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b02ba; continue; }
  pc = 0x40b02a4; continue;
 }
 0x40b02a4 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x40b02ad); f_040aeae8(cpu); if cpu.returned_to != 0x40b02ad { return; }
  { let v: u32 = 0xc0000000u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b02cf; continue;
 }
 0x40b02ba => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x83u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x40b02c3); f_040aeae8(cpu); if cpu.returned_to != 0x40b02c3 { return; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b02cf; continue;
 }
 0x40b02cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b02d0 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b02e1; continue; }
  pc = 0x40b02d4; continue;
 }
 0x40b02d4 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b02e1; continue; }
  pc = 0x40b02d8; continue;
 }
 0x40b02d8 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b02de; continue; }
  pc = 0x40b02dd; continue;
 }
 0x40b02dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b02de => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b02e1; continue;
 }
 0x40b02e1 => {
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
  if !cpu.zf { pc = 0x40b031c; continue; }
  pc = 0x40b0312; continue;
 }
 0x40b0312 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.sf { cpu.poll(); pc = 0x40b0312; continue; }
  pc = 0x40b031c; continue;
 }
 0x40b031c => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b032b; continue; }
  pc = 0x40b0321; continue;
 }
 0x40b0321 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.sf { cpu.poll(); pc = 0x40b0321; continue; }
  pc = 0x40b032b; continue;
 }
 0x40b032b => {
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.ecx as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.add16((cpu.edi as u16), 0x3fffu16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x40b034e; continue; }
  pc = 0x40b0335; continue;
 }
 0x40b0335 => {
  { cpu.sub16((cpu.edi as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x40b034e; continue; }
  pc = 0x40b033c; continue;
 }
 0x40b033c => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u16 = 0x7fffu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x40b0426; continue;
 }
 0x40b034e => {
  { cpu.sub16((cpu.edi as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x40b035f; continue; }
  pc = 0x40b0354; continue;
 }
 0x40b0354 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x40b0426; continue;
 }
 0x40b035f => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b0371; continue; }
  pc = 0x40b036e; continue;
 }
 0x40b036e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40b0371; continue;
 }
 0x40b0371 => {
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
  if cpu.zf { pc = 0x40b0392; continue; }
  pc = 0x40b038d; continue;
 }
 0x40b038d => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  pc = 0x40b0392; continue;
 }
 0x40b0392 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.sbb32(cpu.edi, cpu.edx); cpu.edi = v; }
  if cpu.zf { pc = 0x40b03ac; continue; }
  pc = 0x40b039a; continue;
 }
 0x40b039a => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b039a; continue; }
  pc = 0x40b03ac; continue;
 }
 0x40b03ac => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b03c1; continue; }
  pc = 0x40b03b7; continue;
 }
 0x40b03b7 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b03c1; continue;
 }
 0x40b03c1 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40b03f8; continue; }
  pc = 0x40b03cc; continue;
 }
 0x40b03cc => {
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
  if cpu.zf { pc = 0x40b03f8; continue; }
  pc = 0x40b03e1; continue;
 }
 0x40b03e1 => {
  { let v: u32 = cpu.sub32(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)), 0x1u32); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)), 0x0u32); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.edi, 0x0u32); cpu.edi = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b03e1; continue; }
  pc = 0x40b03f8; continue;
 }
 0x40b03f8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.dec16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x40b040a; continue; }
  pc = 0x40b0405; continue;
 }
 0x40b0405 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x40b040a; continue;
 }
 0x40b040a => {
  { let v: u16 = cpu.or16((cpu.edi as u16), (cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b0426; continue; }
  pc = 0x40b040f; continue;
 }
 0x40b040f => {
  if !cpu.zf { pc = 0x40b0415; continue; }
  pc = 0x40b0411; continue;
 }
 0x40b0411 => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b041b; continue;
 }
 0x40b0415 => {
  { let v: u16 = cpu.neg16((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b041b; continue;
 }
 0x40b041b => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.edi as u16).wrapping_sub((cpu.edi as u16)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x40b0426; continue;
 }
 0x40b0426 => {
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
pub fn f_040b0430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0430); }
 let mut pc: u32 = 0x40b0430;
 loop { match pc {
 0x40b0430 => {
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
  cpu.push(0x40b044d); f_040b045a(cpu); if cpu.returned_to != 0x40b044d { return; }
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
pub fn f_040b045a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b045a); }
 let mut pc: u32 = 0x40b045a;
 loop { match pc {
 0x40b045a => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b046b; continue; }
  pc = 0x40b045e; continue;
 }
 0x40b045e => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b046b; continue; }
  pc = 0x40b0462; continue;
 }
 0x40b0462 => {
  { let v: u16 = cpu.add16((cpu.esi as u16), (cpu.esi as u16)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0468; continue; }
  pc = 0x40b0467; continue;
 }
 0x40b0467 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0468 => {
  { let v: u16 = cpu.rcr16((cpu.esi as u16), 1u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b046b; continue;
 }
 0x40b046b => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40b0482; continue; }
  pc = 0x40b046f; continue;
 }
 0x40b046f => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x40b0482; continue; }
  pc = 0x40b0473; continue;
 }
 0x40b0473 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x40b0482; continue; }
  pc = 0x40b047b; continue;
 }
 0x40b047b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0482 => {
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
  if cpu.cf { pc = 0x40b04b2; continue; }
  pc = 0x40b04a6; continue;
 }
 0x40b04a6 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.cf { pc = 0x40b04b2; continue; }
  pc = 0x40b04ad; continue;
 }
 0x40b04ad => {
  pc = 0x40b0549; continue;
 }
 0x40b04b2 => {
  { cpu.sub16((cpu.ecx as u16), 0xffc0u16); }
  if (cpu.sf == cpu.of) { pc = 0x40b04c3; continue; }
  pc = 0x40b04b8; continue;
 }
 0x40b04b8 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b053f; continue;
 }
 0x40b04c3 => {
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
  if cpu.sf { pc = 0x40b04fa; continue; }
  pc = 0x40b04f2; continue;
 }
 0x40b04f2 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b04fa; continue;
 }
 0x40b04fa => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.cf { pc = 0x40b0526; continue; }
  pc = 0x40b04fe; continue;
 }
 0x40b04fe => {
  if !cpu.zf { pc = 0x40b050d; continue; }
  pc = 0x40b0500; continue;
 }
 0x40b0500 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u8 = (!cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x40b050d; continue; }
  pc = 0x40b0509; continue;
 }
 0x40b0509 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  pc = 0x40b050d; continue;
 }
 0x40b050d => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x40b0526; continue; }
  pc = 0x40b0515; continue;
 }
 0x40b0515 => {
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x40b0549; continue; }
  pc = 0x40b0526; continue;
 }
 0x40b0526 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b053f; continue; }
  pc = 0x40b052b; continue;
 }
 0x40b052b => {
  if !cpu.zf { pc = 0x40b0531; continue; }
  pc = 0x40b052d; continue;
 }
 0x40b052d => {
  { let v: u8 = 0x1u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b0534; continue;
 }
 0x40b0531 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b0534; continue;
 }
 0x40b0534 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ebx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b053f; continue;
 }
 0x40b053f => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0549 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40b053f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0556(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b0556); } r_040b0556(cpu, 0x40b0556); }
pub fn f_040b05ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b05ab); } r_040b0556(cpu, 0x40b05ab); }
pub fn f_040b05b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b05b0); } r_040b0556(cpu, 0x40b05b0); }
fn r_040b0556(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b0556 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0xa0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x40b05b3; continue;
 }
 0x40b05ab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x1fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x40b05b3; continue;
 }
 0x40b05b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x40b05b3; continue;
 }
 0x40b05b3 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shr8(((cpu.ebx >> 8) as u8), 1u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ecx as u16), 0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b0615; continue; }
  pc = 0x40b05cb; continue;
 }
 0x40b05cb => {
  { cpu.sub16((cpu.ecx as u16), 0x20u16); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b060d; continue; }
  pc = 0x40b05d1; continue;
 }
 0x40b05d1 => {
  { let v: u8 = (cpu.ebx as u8) & 0x3fu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), (cpu.ebx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b060d; continue; }
  pc = 0x40b05d8; continue;
 }
 0x40b05d8 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.rcr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x40b05ec; continue; }
  pc = 0x40b05e8; continue;
 }
 0x40b05e8 => {
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b05f9; continue;
 }
 0x40b05ec => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.shld32(cpu.ebx, cpu.eax, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40b05f9; continue;
 }
 0x40b05f9 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.rcr8((cpu.ecx as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x40b060a; continue; }
  pc = 0x40b0608; continue;
 }
 0x40b0608 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b060a; continue;
 }
 0x40b060a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b060d => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0615 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b061a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b061a); } r_040b061a(cpu, 0x40b061a); }
pub fn f_040b062b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b062b); } r_040b061a(cpu, 0x40b062b); }
fn r_040b061a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b061a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.sf { pc = 0x40b062a; continue; }
  pc = 0x40b0621; continue;
 }
 0x40b0621 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xbfffu32; cpu.edx = v; }
  pc = 0x40b0633; continue;
 }
 0x40b062a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x40b062b; continue;
 }
 0x40b062b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3fffu32; cpu.edx = v; }
  pc = 0x40b0633; continue;
 }
 0x40b0633 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40b064e; continue; }
  pc = 0x40b0638; continue;
 }
 0x40b0638 => {
  cpu.unimplemented(0x40b0638, "Bsr");
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = ((cpu.ecx >> 8) as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40b0652; continue;
 }
 0x40b064e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b0652; continue;
 }
 0x40b0652 => {
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
pub fn f_040b0660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0660); }
 let mut pc: u32 = 0x40b0660;
 loop { match pc {
 0x40b0660 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 11u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 20u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7ffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b06ab; continue; }
  pc = 0x40b0674; continue;
 }
 0x40b0674 => {
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if cpu.zf { pc = 0x40b0682; continue; }
  pc = 0x40b067b; continue;
 }
 0x40b067b => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3c00u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b06a3; continue;
 }
 0x40b0682 => {
  { let v: u16 = 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b0690; continue; }
  pc = 0x40b068e; continue;
 }
 0x40b068e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b0690; continue;
 }
 0x40b0690 => {
  if cpu.zf { pc = 0x40b06a3; continue; }
  pc = 0x40b0692; continue;
 }
 0x40b0692 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x81u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x40b069c); f_040aeae8(cpu); if cpu.returned_to != 0x40b069c { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx | 0x40000000u32; cpu.edx = v; }
  pc = 0x40b06a3; continue;
 }
 0x40b06a3 => {
  { let v: u32 = cpu.edx | 0x80000000u32; cpu.edx = v; }
  pc = 0x40b06d0; continue;
 }
 0x40b06ab => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b06b7; continue; }
  pc = 0x40b06af; continue;
 }
 0x40b06af => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b06b7; continue; }
  pc = 0x40b06b3; continue;
 }
 0x40b06b3 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b06d0; continue;
 }
 0x40b06b7 => {
  { let v: u16 = 0x3c01u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b06c4; continue; }
  pc = 0x40b06bf; continue;
 }
 0x40b06bf => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x20u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b06c4; continue;
 }
 0x40b06c4 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x40b06d0; continue; }
  pc = 0x40b06c8; continue;
 }
 0x40b06c8 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40b06c4; continue;
 }
 0x40b06d0 => {
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
pub fn f_040b06e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b06e0); }
 let mut pc: u32 = 0x40b06e0;
 loop { match pc {
 0x40b06e0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 25u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x40b070f; continue; }
  pc = 0x40b06f8; continue;
 }
 0x40b06f8 => {
  if !cpu.zf { pc = 0x40b0701; continue; }
  pc = 0x40b06fa; continue;
 }
 0x40b06fa => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0701; continue; }
  pc = 0x40b06ff; continue;
 }
 0x40b06ff => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x40b0701; continue;
 }
 0x40b0701 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x40b070f; continue; }
  pc = 0x40b0708; continue;
 }
 0x40b0708 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b070f; continue;
 }
 0x40b070f => {
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b075b; continue; }
  pc = 0x40b071a; continue;
 }
 0x40b071a => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x40b0732; continue; }
  pc = 0x40b0721; continue;
 }
 0x40b0721 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b075b; continue;
 }
 0x40b0732 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0xc080u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x40b073d; continue; }
  pc = 0x40b0739; continue;
 }
 0x40b0739 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b075b; continue;
 }
 0x40b073d => {
  { cpu.sub16((cpu.ecx as u16), 0xffu16); }
  if (cpu.sf != cpu.of) { pc = 0x40b0750; continue; }
  pc = 0x40b0744; continue;
 }
 0x40b0744 => {
  { let v: u32 = 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b075b; continue;
 }
 0x40b0750 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.ecx, 8u32); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b075b; continue;
 }
 0x40b075b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b075f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b075f); }
 let mut pc: u32 = 0x40b075f;
 loop { match pc {
 0x40b075f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffff800u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 22u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x40b078c; continue; }
  pc = 0x40b0777; continue;
 }
 0x40b0777 => {
  if !cpu.zf { pc = 0x40b077b; continue; }
  pc = 0x40b0779; continue;
 }
 0x40b0779 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  pc = 0x40b077b; continue;
 }
 0x40b077b => {
  { let v: u32 = cpu.add32(cpu.eax, 0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x40b078c; continue; }
  pc = 0x40b0785; continue;
 }
 0x40b0785 => {
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b078c; continue;
 }
 0x40b078c => {
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xc400u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if !cpu.cf { pc = 0x40b07c2; continue; }
  pc = 0x40b07a1; continue;
 }
 0x40b07a1 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b07b1; continue; }
  pc = 0x40b07a6; continue;
 }
 0x40b07a6 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 12u32); cpu.edx = v; }
  pc = 0x40b07bb; continue;
 }
 0x40b07b1 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ecx, 11u32); cpu.edx = v; }
  pc = 0x40b07bb; continue;
 }
 0x40b07bb => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x40b0822; continue;
 }
 0x40b07c2 => {
  { cpu.sub16((cpu.ecx as u16), 0xc400u16); }
  if cpu.cf { pc = 0x40b07fc; continue; }
  pc = 0x40b07c9; continue;
 }
 0x40b07c9 => {
  { cpu.sub16((cpu.ecx as u16), 0xffccu16); }
  if (cpu.sf != cpu.of) { pc = 0x40b07f6; continue; }
  pc = 0x40b07cf; continue;
 }
 0x40b07cf => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x40b07e4; continue; }
  pc = 0x40b07db; continue;
 }
 0x40b07db => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x40b07e4; continue;
 }
 0x40b07e4 => {
  { let v: u32 = cpu.shrd32(cpu.esi, cpu.eax, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x40b07fa; continue;
 }
 0x40b07f6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x40b07fa; continue;
 }
 0x40b07fa => {
  pc = 0x40b0822; continue;
 }
 0x40b07fc => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 11u32); cpu.edx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x7ff00000u32; cpu.edx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x43ffu16); }
  if cpu.zf { pc = 0x40b0822; continue; }
  pc = 0x40b0817; continue;
 }
 0x40b0817 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x84u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x40b0821); f_040aeae8(cpu); if cpu.returned_to != 0x40b0821 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40b0822; continue;
 }
 0x40b0822 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0826); }
 let mut pc: u32 = 0x40b0826;
 loop { match pc {
 0x40b0826 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 23u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0xffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b085f; continue; }
  pc = 0x40b0838; continue;
 }
 0x40b0838 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x40b0847; continue; }
  pc = 0x40b0840; continue;
 }
 0x40b0840 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3f80u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b0855; continue;
 }
 0x40b0847 => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40b0855; continue; }
  pc = 0x40b0850; continue;
 }
 0x40b0850 => {
  { let v: u32 = cpu.eax | 0x40000000u32; cpu.eax = v; }
  pc = 0x40b0855; continue;
 }
 0x40b0855 => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, 0x80000000u32); cpu.eax = v; }
  pc = 0x40b085f; continue;
 }
 0x40b085f => {
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
pub fn f_040b08eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b08eb); }
 let mut pc: u32 = 0x40b08eb;
 loop { match pc {
 0x40b08eb => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x40b0900; continue; }
  pc = 0x40b08fb; continue;
 }
 0x40b08fb => {
  { cpu.sub16((cpu.edx as u16), 0x800u16); }
  pc = 0x40b0900; continue;
 }
 0x40b0900 => {
  if !cpu.zf { pc = 0x40b090b; continue; }
  pc = 0x40b0902; continue;
 }
 0x40b0902 => {
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x40b090b; continue; }
  pc = 0x40b0906; continue;
 }
 0x40b0906 => {
  { let v: u16 = (cpu.edx as u16) ^ 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x40b090b; continue;
 }
 0x40b090b => {
  { cpu.sub16((cpu.edx as u16), 0xc00u16); }
  if !cpu.zf { pc = 0x40b0916; continue; }
  pc = 0x40b0912; continue;
 }
 0x40b0912 => {
  { let v: u16 = 0x400u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x40b0916; continue;
 }
 0x40b0916 => {
  { let v: u16 = (cpu.ebx as u16) & 0x7fffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x403fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b0924; continue; }
  pc = 0x40b0922; continue;
 }
 0x40b0922 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0924 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b097a; continue; }
  pc = 0x40b092c; continue;
 }
 0x40b092c => {
  { let v: u32 = cpu.r32(0x40b086bu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x40b0978; continue; }
  pc = 0x40b0940; continue;
 }
 0x40b0940 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x40b0965; continue; }
  pc = 0x40b0946; continue;
 }
 0x40b0946 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b0978; continue; }
  pc = 0x40b0950; continue;
 }
 0x40b0950 => {
  if !cpu.zf { pc = 0x40b0965; continue; }
  pc = 0x40b0952; continue;
 }
 0x40b0952 => {
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x40b095b; continue; }
  pc = 0x40b0956; continue;
 }
 0x40b0956 => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x40b095d; continue;
 }
 0x40b095b => {
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x40b095d; continue;
 }
 0x40b095d => {
  if cpu.zf { pc = 0x40b0978; continue; }
  pc = 0x40b0963; continue;
 }
 0x40b0963 => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b0965; continue;
 }
 0x40b0965 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b0978; continue; }
  pc = 0x40b0969; continue;
 }
 0x40b0969 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x40b0978; continue; }
  pc = 0x40b0971; continue;
 }
 0x40b0971 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b0978; continue;
 }
 0x40b0978 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b097a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b09dd; continue; }
  pc = 0x40b098a; continue;
 }
 0x40b098a => {
  { let v: u32 = cpu.r32(0x40b086bu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi & cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) ^ cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x40b09da; continue; }
  pc = 0x40b099f; continue;
 }
 0x40b099f => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x40b09ca; continue; }
  pc = 0x40b09a5; continue;
 }
 0x40b09a5 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b09da; continue; }
  pc = 0x40b09b1; continue;
 }
 0x40b09b1 => {
  if !cpu.zf { pc = 0x40b09ca; continue; }
  pc = 0x40b09b3; continue;
 }
 0x40b09b3 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40b09ce; continue; }
  pc = 0x40b09bb; continue;
 }
 0x40b09bb => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x40b09c2; continue; }
  pc = 0x40b09bf; continue;
 }
 0x40b09bf => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x40b09c2; continue;
 }
 0x40b09c2 => {
  if cpu.zf { pc = 0x40b09da; continue; }
  pc = 0x40b09c8; continue;
 }
 0x40b09c8 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40b09ca; continue;
 }
 0x40b09ca => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b09da; continue; }
  pc = 0x40b09ce; continue;
 }
 0x40b09ce => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x40b09da; continue; }
  pc = 0x40b09d3; continue;
 }
 0x40b09d3 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b09da; continue;
 }
 0x40b09da => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b09dd => {
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x40b0a12; continue; }
  pc = 0x40b09e4; continue;
 }
 0x40b09e4 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x40b0a02; continue; }
  pc = 0x40b09ea; continue;
 }
 0x40b09ea => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0a12; continue; }
  pc = 0x40b09f2; continue;
 }
 0x40b09f2 => {
  { cpu.sub32(cpu.esi, 0x80000000u32); }
  if !cpu.zf { pc = 0x40b0a02; continue; }
  pc = 0x40b09fa; continue;
 }
 0x40b09fa => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b0a12; continue; }
  pc = 0x40b0a02; continue;
 }
 0x40b0a02 => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0a12 => {
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
pub fn f_040b0a1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0a1e); }
 let mut pc: u32 = 0x40b0a1e;
 loop { match pc {
 0x40b0a1e => {
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
  cpu.unimplemented(0x40b0a3b, "Aas");
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0xb99d0a6bu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x171bffbu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xe38e2727u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  {  }
  cpu.unimplemented(0x40b0a4f, "Aas");
  { cpu.df = false; }
  { let v = 0x2472dd9cu32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.df = false; }
  { let v: u32 = 0x94ebadc1u32; cpu.edi = v; }
  cpu.unimplemented(0x40b0a5e, "Int3");
  cpu.unimplemented(0x40b0a5f, "Int3");
  cpu.unimplemented(0x40b0a60, "Int3");
  cpu.unimplemented(0x40b0a61, "Int3");
  { cpu.df = false; }
  cpu.unimplemented(0x40b0a63, "Aas");
  { let v: u32 = cpu.adc32(cpu.r32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0xbffdaau32); cpu.w32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x3fu8); }
  cpu.flat_ds(0x40b0a75); f_040b0a78(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0a78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0a78); }
 let mut pc: u32 = 0x40b0a78;
 loop { match pc {
 0x40b0a78 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0ab6; continue; }
  pc = 0x40b0a86; continue;
 }
 0x40b0a86 => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x40b0aa1; continue; }
  pc = 0x40b0a8c; continue;
 }
 0x40b0a8c => {
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b0ab4; continue;
 }
 0x40b0aa1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b0ab4; continue;
 }
 0x40b0ab4 => {
  pc = 0x40b0b2a; continue;
 }
 0x40b0ab6 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0ade; continue; }
  pc = 0x40b0abf; continue;
 }
 0x40b0abf => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b0b2a; continue;
 }
 0x40b0ade => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x40b0ae6); f_040b025b(cpu); if cpu.returned_to != 0x40b0ae6 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b0aed); f_040b0b2e(cpu); if cpu.returned_to != 0x40b0aed { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x40b0b10; continue; }
  pc = 0x40b0af4; continue;
 }
 0x40b0af4 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x40b0b0e; continue; }
  pc = 0x40b0af8; continue;
 }
 0x40b0af8 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x4000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x40b0b0e); f_040b0026(cpu); if cpu.returned_to != 0x40b0b0e { return; }
  pc = 0x40b0b0e; continue;
 }
 0x40b0b0e => {
  pc = 0x40b0b2a; continue;
 }
 0x40b0b10 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b0b2a; continue; }
  pc = 0x40b0b14; continue;
 }
 0x40b0b14 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xc000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x40b0b2a); f_040b0026(cpu); if cpu.returned_to != 0x40b0b2a { return; }
  pc = 0x40b0b2a; continue;
 }
 0x40b0b2a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0b2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0b2e); }
 let mut pc: u32 = 0x40b0b2e;
 loop { match pc {
 0x40b0b2e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if !cpu.zf { pc = 0x40b0b70; continue; }
  pc = 0x40b0b44; continue;
 }
 0x40b0b44 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x80000000u32); }
  if !cpu.zf { pc = 0x40b0b70; continue; }
  pc = 0x40b0b4d; continue;
 }
 0x40b0b4d => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0b70; continue; }
  pc = 0x40b0b52; continue;
 }
 0x40b0b52 => {
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
 0x40b0b70 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x40b0ba6; continue; }
  pc = 0x40b0b7a; continue;
 }
 0x40b0b7a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x40b0b9a); f_040b0285(cpu); if cpu.returned_to != 0x40b0b9a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b0ba6; continue;
 }
 0x40b0ba6 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3ffdu16); }
  if !cpu.zf { pc = 0x40b0bc1; continue; }
  pc = 0x40b0bb0; continue;
 }
 0x40b0bb0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8930a2f4u32); }
  if !cpu.zf { pc = 0x40b0bc1; continue; }
  pc = 0x40b0bba; continue;
 }
 0x40b0bba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xf66ab09bu32); }
  pc = 0x40b0bc1; continue;
 }
 0x40b0bc1 => {
  if (cpu.cf || cpu.zf) { pc = 0x40b0c34; continue; }
  pc = 0x40b0bc3; continue;
 }
 0x40b0bc3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x40b0be1); f_040b0056(cpu); if cpu.returned_to != 0x40b0be1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x40b0c01); f_040b045a(cpu); if cpu.returned_to != 0x40b0c01 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0xbfffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x40b0c17); f_040b0056(cpu); if cpu.returned_to != 0x40b0c17 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b0c27); f_040b0285(cpu); if cpu.returned_to != 0x40b0c27 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b0c34; continue;
 }
 0x40b0c34 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x40b0a1eu32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.push(0x40b0c46); f_040b16aa(cpu); if cpu.returned_to != 0x40b0c46 { return; }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x40b0c62; continue; }
  pc = 0x40b0c4a; continue;
 }
 0x40b0c4a => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0x860a91c1u32; cpu.push(v); }
  { let v = 0x6b9b2c23u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b0c62); f_040b0026(cpu); if cpu.returned_to != 0x40b0c62 { return; }
  pc = 0x40b0c62; continue;
 }
 0x40b0c62 => {
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x40b0c82; continue; }
  pc = 0x40b0c66; continue;
 }
 0x40b0c66 => {
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b0c7e); f_040b0026(cpu); if cpu.returned_to != 0x40b0c7e { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b0c82; continue;
 }
 0x40b0c82 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x40b0c8b; continue; }
  pc = 0x40b0c87; continue;
 }
 0x40b0c87 => {
  { let v: u8 = cpu.xor8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b0c8b; continue;
 }
 0x40b0c8b => {
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
pub fn f_040b0c91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0c91); }
 let mut pc: u32 = 0x40b0c91;
 loop { match pc {
 0x40b0c91 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x40b0ca6; continue; }
  pc = 0x40b0c98; continue;
 }
 0x40b0c98 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0ca6; continue; }
  pc = 0x40b0c9e; continue;
 }
 0x40b0c9e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0ca6; continue; }
  pc = 0x40b0ca3; continue;
 }
 0x40b0ca3 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0ca6 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x40b0cc6; continue; }
  pc = 0x40b0cad; continue;
 }
 0x40b0cad => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0cc6; continue; }
  pc = 0x40b0cb3; continue;
 }
 0x40b0cb3 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0cc6; continue; }
  pc = 0x40b0cb8; continue;
 }
 0x40b0cb8 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b0cc6 => {
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
  cpu.push(0x40b0ce4); f_040b0cf7(cpu); if cpu.returned_to != 0x40b0ce4 { return; }
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
pub fn f_040b0cf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0cf7); }
 let mut pc: u32 = 0x40b0cf7;
 loop { match pc {
 0x40b0cf7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi & 0x7fffu32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0x7fffu32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b0d56; continue; }
  pc = 0x40b0d09; continue;
 }
 0x40b0d09 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x40b0d0b; continue;
 }
 0x40b0d0b => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.zf { pc = 0x40b0d17; continue; }
  pc = 0x40b0d0f; continue;
 }
 0x40b0d0f => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.zf { pc = 0x40b0d19; continue; }
  pc = 0x40b0d17; continue;
 }
 0x40b0d17 => {
  if !cpu.cf { pc = 0x40b0d1e; continue; }
  pc = 0x40b0d19; continue;
 }
 0x40b0d19 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { cpu.cf = true; }
  pc = 0x40b0d1e; continue;
 }
 0x40b0d1e => {
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b0d33; continue; }
  pc = 0x40b0d27; continue;
 }
 0x40b0d27 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.cf { cpu.poll(); pc = 0x40b0d19; continue; }
  pc = 0x40b0d2d; continue;
 }
 0x40b0d2d => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { cpu.poll(); pc = 0x40b0d0b; continue; }
  pc = 0x40b0d31; continue;
 }
 0x40b0d31 => {
  cpu.poll(); pc = 0x40b0d1e; continue;
 }
 0x40b0d33 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b0d3f; continue; }
  pc = 0x40b0d39; continue;
 }
 0x40b0d39 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b0d3f; continue; }
  pc = 0x40b0d3d; continue;
 }
 0x40b0d3d => {
  pc = 0x40b0d54; continue;
 }
 0x40b0d3f => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x40b0d4a; continue; }
  pc = 0x40b0d43; continue;
 }
 0x40b0d43 => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40b0d3f; continue;
 }
 0x40b0d4a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x7fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x40b0d54; continue;
 }
 0x40b0d54 => {
  pc = 0x40b0d5a; continue;
 }
 0x40b0d56 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x40b0d5a; continue;
 }
 0x40b0d5a => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi | cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b0d6a; continue; }
  pc = 0x40b0d68; continue;
 }
 0x40b0d68 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x40b0d6a; continue;
 }
 0x40b0d6a => {
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x40b0d71; continue; }
  pc = 0x40b0d6f; continue;
 }
 0x40b0d6f => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x40b0d71; continue;
 }
 0x40b0d71 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0d72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0d72); }
 let mut pc: u32 = 0x40b0d72;
 loop { match pc {
 0x40b0d72 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x40ae52eu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x40b0d90; continue; }
  pc = 0x40b0d8a; continue;
 }
 0x40b0d8a => {
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b0e03; continue;
 }
 0x40b0d90 => {
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) & (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.rol8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x40b0dc4; continue; }
  pc = 0x40b0daa; continue;
 }
 0x40b0daa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x40b0dc2; continue; }
  pc = 0x40b0db3; continue;
 }
 0x40b0db3 => {
  { let v: u16 = cpu.r16(0x40ae528u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b0dc2; continue;
 }
 0x40b0dc2 => {
  pc = 0x40b0e03; continue;
 }
 0x40b0dc4 => {
  { cpu.sub16((cpu.eax as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x40b0dec; continue; }
  pc = 0x40b0dca; continue;
 }
 0x40b0dca => {
  { let v: u16 = cpu.r16(0x40ae52cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b0de4; continue; }
  pc = 0x40b0ddc; continue;
 }
 0x40b0ddc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  pc = 0x40b0de4; continue;
 }
 0x40b0de4 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x40b0dea; continue; }
  pc = 0x40b0de8; continue;
 }
 0x40b0de8 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b0dea; continue;
 }
 0x40b0dea => {
  pc = 0x40b0e03; continue;
 }
 0x40b0dec => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.ebx)), 0x80u8); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x40b0e03; continue; }
  pc = 0x40b0df4; continue;
 }
 0x40b0df4 => {
  { let v: u16 = cpu.r16(0x40ae52cu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b0e03; continue;
 }
 0x40b0e03 => {
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
pub fn f_040b0e1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0e1a); }
 let mut pc: u32 = 0x40b0e1a;
 loop { match pc {
 0x40b0e1a => {
  cpu.call(0x9ec0f765); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0e38(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b0e38); } r_040b0e38(cpu, 0x40b0e38); }
pub fn f_040b0e79(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b0e79); } r_040b0e38(cpu, 0x40b0e79); }
fn r_040b0e38(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b0e38 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x76bb3fffu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x76bb3fffu32.wrapping_add(cpu.eax), v); }
  if cpu.of { pc = 0x40b0e84; continue; }
  pc = 0x40b0e46; continue;
 }
 0x40b0e46 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xef60c004u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.r32(0x9c041fd0u32.wrapping_add(cpu.ecx))); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0xf4e06c83u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.ror8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 192u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_040b0e60(cpu); return;
 }
 0x40b0e79 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40b0e84); f_040b0ebc(cpu); if cpu.returned_to != 0x40b0e84 { return; }
  pc = 0x40b0e84; continue;
 }
 0x40b0e84 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40b0e8d); f_040b0430(cpu); if cpu.returned_to != 0x40b0e8d { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8aa3b29u32; cpu.edx = v; }
  { let v: u32 = 0x5c17f0bdu32; cpu.eax = v; }
  cpu.push(0x40b0ead); f_040b045a(cpu); if cpu.returned_to != 0x40b0ead { return; }
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
pub fn f_040b0e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0e60); }
 let mut pc: u32 = 0x40b0e60;
 loop { match pc {
 0x40b0e60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3fffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40b0e76); f_040b0026(cpu); if cpu.returned_to != 0x40b0e76 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_040b0e79(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b0ebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b0ebc); }
 let mut pc: u32 = 0x40b0ebc;
 loop { match pc {
 0x40b0ebc => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x40b0ece; continue; }
  pc = 0x40b0ecc; continue;
 }
 0x40b0ecc => {
  pc = 0x40b0ece; continue;
 }
 0x40b0ece => {
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
  cpu.push(0x40b0ef6); f_040b0056(cpu); if cpu.returned_to != 0x40b0ef6 { return; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3ffeu16); }
  if !cpu.zf { pc = 0x40b0f0d; continue; }
  pc = 0x40b0efe; continue;
 }
 0x40b0efe => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xb504f333u32); }
  if !cpu.zf { pc = 0x40b0f0d; continue; }
  pc = 0x40b0f07; continue;
 }
 0x40b0f07 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0xf9de6484u32); }
  pc = 0x40b0f0d; continue;
 }
 0x40b0f0d => {
  if (cpu.cf || cpu.zf) { pc = 0x40b0f29; continue; }
  pc = 0x40b0f0f; continue;
 }
 0x40b0f0f => {
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbffe0000u32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x40b0f27); f_040b0056(cpu); if cpu.returned_to != 0x40b0f27 { return; }
  pc = 0x40b0f35; continue;
 }
 0x40b0f29 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40b0f35; continue;
 }
 0x40b0f35 => {
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
  cpu.push(0x40b0f5b); f_040b0056(cpu); if cpu.returned_to != 0x40b0f5b { return; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x40b0f6d); f_040b0285(cpu); if cpu.returned_to != 0x40b0f6d { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40b0f81); f_040b045a(cpu); if cpu.returned_to != 0x40b0f81 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x40b0e38u32; cpu.ecx = v; }
  cpu.push(0x40b0f9a); f_040b1658(cpu); if cpu.returned_to != 0x40b0f9a { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x40b0e1au32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  cpu.push(0x40b0fb6); f_040b1658(cpu); if cpu.returned_to != 0x40b0fb6 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b0fc7); f_040b0285(cpu); if cpu.returned_to != 0x40b0fc7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x40b0fdb); f_040b045a(cpu); if cpu.returned_to != 0x40b0fdb { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x40b0ff2); f_040b045a(cpu); if cpu.returned_to != 0x40b0ff2 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b1003); f_040b0056(cpu); if cpu.returned_to != 0x40b1003 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b107e; continue; }
  pc = 0x40b1008; continue;
 }
 0x40b1008 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b1014); f_040b061a(cpu); if cpu.returned_to != 0x40b1014 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbff20000u32; cpu.esi = v; }
  { let v: u32 = 0xde8082e3u32; cpu.ecx = v; }
  { let v: u32 = 0x865435cu32; cpu.ebx = v; }
  cpu.push(0x40b1038); f_040b045a(cpu); if cpu.returned_to != 0x40b1038 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b1049); f_040b0056(cpu); if cpu.returned_to != 0x40b1049 { return; }
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
  cpu.push(0x40b106d); f_040b045a(cpu); if cpu.returned_to != 0x40b106d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b107e); f_040b0056(cpu); if cpu.returned_to != 0x40b107e { return; }
  pc = 0x40b107e; continue;
 }
 0x40b107e => {
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
pub fn f_040b108e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b108e); }
 let mut pc: u32 = 0x40b108e;
 loop { match pc {
 0x40b108e => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffff85u32; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x3fceca96u32); }
  cpu.call(0x40b1098); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1142(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b1142); } r_040b1142(cpu, 0x40b1142); }
pub fn f_040b1154(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b1154); } r_040b1142(cpu, 0x40b1154); }
pub fn f_040b1219(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b1219); } r_040b1142(cpu, 0x40b1219); }
fn r_040b1142(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b1142 => {
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
  if (cpu.sf != cpu.of) { pc = 0x40b1176; continue; }
  pc = 0x40b1176; continue;
 }
 0x40b1154 => {
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
  if (cpu.sf != cpu.of) { pc = 0x40b1176; continue; }
  pc = 0x40b1176; continue;
 }
 0x40b1176 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3ffeu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc90fdaa2u32; cpu.ecx = v; }
  { let v: u32 = 0x2168c235u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40b1195); f_040b0cf7(cpu); if cpu.returned_to != 0x40b1195 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); }
  if cpu.zf { pc = 0x40b11c5; continue; }
  pc = 0x40b11aa; continue;
 }
 0x40b11aa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x40b11c3); f_040b0026(cpu); if cpu.returned_to != 0x40b11c3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x40b11c5; continue;
 }
 0x40b11c5 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b11f1; continue; }
  pc = 0x40b11d5; continue;
 }
 0x40b11d5 => {
  { let v = 0xbffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b11ed); f_040b0026(cpu); if cpu.returned_to != 0x40b11ed { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b11f1; continue;
 }
 0x40b11f1 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xffe0u16); }
  if (cpu.sf == cpu.of) { pc = 0x40b121b; continue; }
  pc = 0x40b1202; continue;
 }
 0x40b1202 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b12b2; continue;
 }
 0x40b1219 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b121b; continue;
 }
 0x40b121b => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x40b1224; continue; }
  pc = 0x40b1220; continue;
 }
 0x40b1220 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b1224; continue;
 }
 0x40b1224 => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b1231; continue; }
  pc = 0x40b122d; continue;
 }
 0x40b122d => {
  { let v: u16 = 0x8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x40b1231; continue;
 }
 0x40b1231 => {
  { let v: u16 = cpu.r16(0x40b1142u32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x40b124b; continue; }
  pc = 0x40b1244; continue;
 }
 0x40b1244 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b124b; continue; }
  pc = 0x40b1249; continue;
 }
 0x40b1249 => {
  pc = 0x40b1289; continue;
 }
 0x40b124b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x40b1265); f_040b045a(cpu); if cpu.returned_to != 0x40b1265 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x40b10e8u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  cpu.push(0x40b127e); f_040b1658(cpu); if cpu.returned_to != 0x40b127e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40b1289; continue;
 }
 0x40b1289 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b1294; continue; }
  pc = 0x40b128d; continue;
 }
 0x40b128d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b1294; continue; }
  pc = 0x40b1292; continue;
 }
 0x40b1292 => {
  pc = 0x40b12b2; continue;
 }
 0x40b1294 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x40b108eu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  cpu.push(0x40b12b2); f_040b16aa(cpu); if cpu.returned_to != 0x40b12b2 { return; }
  pc = 0x40b12b2; continue;
 }
 0x40b12b2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x40b12c7; continue; }
  pc = 0x40b12bb; continue;
 }
 0x40b12bb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b12d0; continue;
 }
 0x40b12c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b12d0; continue;
 }
 0x40b12d0 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b12dd; continue; }
  pc = 0x40b12d8; continue;
 }
 0x40b12d8 => {
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b12dd; continue;
 }
 0x40b12dd => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x40b133a; continue; }
  pc = 0x40b12e2; continue;
 }
 0x40b12e2 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x40b1300; continue; }
  pc = 0x40b12f1; continue;
 }
 0x40b12f1 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b130c; continue;
 }
 0x40b1300 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b130c; continue;
 }
 0x40b130c => {
  if cpu.zf { pc = 0x40b1314; continue; }
  pc = 0x40b130e; continue;
 }
 0x40b130e => {
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  pc = 0x40b1314; continue;
 }
 0x40b1314 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x40b1335; continue; }
  pc = 0x40b131b; continue;
 }
 0x40b131b => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40b1335; continue; }
  pc = 0x40b131f; continue;
 }
 0x40b131f => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x40b1335; continue; }
  pc = 0x40b1327; continue;
 }
 0x40b1327 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x40b133a; continue;
 }
 0x40b1335 => {
  cpu.push(0x40b133a); f_040b0285(cpu); if cpu.returned_to != 0x40b133a { return; }
  pc = 0x40b133a; continue;
 }
 0x40b133a => {
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
pub fn f_040b134b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b134b); }
 let mut pc: u32 = 0x40b134b;
 loop { match pc {
 0x40b134b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x40b1353); f_040b1154(cpu); if cpu.returned_to != 0x40b1353 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1355(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1355); }
 let mut pc: u32 = 0x40b1355;
 loop { match pc {
 0x40b1355 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  cpu.push(0x40b1360); f_040b1154(cpu); if cpu.returned_to != 0x40b1360 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1362); }
 let mut pc: u32 = 0x40b1362;
 loop { match pc {
 0x40b1362 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x40b136d); f_040b1154(cpu); if cpu.returned_to != 0x40b136d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1370); }
 let mut pc: u32 = 0x40b1370;
 loop { match pc {
 0x40b1370 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x67cc487bu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1398); }
 let mut pc: u32 = 0x40b1398;
 loop { match pc {
 0x40b1398 => {
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
  if !cpu.zf { pc = 0x40b13ba; continue; }
  pc = 0x40b13b1; continue;
 }
 0x40b13b1 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x40b13ba; continue; }
  pc = 0x40b13b7; continue;
 }
 0x40b13b7 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  pc = 0x40b13ba; continue;
 }
 0x40b13ba => {
  if cpu.zf { pc = 0x40b1422; continue; }
  pc = 0x40b13bc; continue;
 }
 0x40b13bc => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x40b13db; continue; }
  pc = 0x40b13c1; continue;
 }
 0x40b13c1 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b13d7); f_040b0026(cpu); if cpu.returned_to != 0x40b13d7 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b13db; continue;
 }
 0x40b13db => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x40b13ea; continue;
 }
 0x40b13ea => {
  { cpu.sub16((cpu.ebx as u16), 0x3fffu16); }
  if cpu.cf { pc = 0x40b13fb; continue; }
  pc = 0x40b13f1; continue;
 }
 0x40b13f1 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40b13ea; continue;
 }
 0x40b13fb => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b1401; continue; }
  pc = 0x40b13ff; continue;
 }
 0x40b13ff => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b1401; continue;
 }
 0x40b1401 => {
  if !cpu.zf { pc = 0x40b1407; continue; }
  pc = 0x40b1403; continue;
 }
 0x40b1403 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b1417; continue;
 }
 0x40b1407 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x40b1413; continue; }
  pc = 0x40b140b; continue;
 }
 0x40b140b => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40b1407; continue;
 }
 0x40b1413 => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x40b1417; continue;
 }
 0x40b1417 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b1424; continue;
 }
 0x40b1422 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b1424; continue;
 }
 0x40b1424 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x40b143f); f_040b045a(cpu); if cpu.returned_to != 0x40b143f { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xa6829a79u32; cpu.ecx = v; }
  { let v: u32 = 0x1bf21f8cu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40030000u32; cpu.esi = v; }
  cpu.push(0x40b145d); f_040b0056(cpu); if cpu.returned_to != 0x40b145d { return; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = 0xec96f0d6u32; cpu.ecx = v; }
  { let v: u32 = 0x9d7bfdbu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffa0000u32; cpu.esi = v; }
  cpu.push(0x40b1483); f_040b045a(cpu); if cpu.returned_to != 0x40b1483 { return; }
  { let v: u32 = 0xe6d5051au32; cpu.ecx = v; }
  { let v: u32 = 0xe536e187u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40010000u32; cpu.esi = v; }
  cpu.push(0x40b149e); f_040b0056(cpu); if cpu.returned_to != 0x40b149e { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x40b14b2); f_040b045a(cpu); if cpu.returned_to != 0x40b14b2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x40b14d0); f_040b0056(cpu); if cpu.returned_to != 0x40b14d0 { return; }
  { let a = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x40b14ee); f_040b0056(cpu); if cpu.returned_to != 0x40b14ee { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b1500); f_040b0285(cpu); if cpu.returned_to != 0x40b1500 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x40b1370u32; cpu.esi = v; }
  pc = 0x40b1513; continue;
 }
 0x40b1513 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b154b; continue; }
  pc = 0x40b1517; continue;
 }
 0x40b1517 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x40b1546; continue; }
  pc = 0x40b151b; continue;
 }
 0x40b151b => {
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
  cpu.push(0x40b153b); f_040b045a(cpu); if cpu.returned_to != 0x40b153b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x40b1546; continue;
 }
 0x40b1546 => {
  { let v: u32 = 0xau32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40b1513; continue;
 }
 0x40b154b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.add16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.esi as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b1565); f_040b0026(cpu); if cpu.returned_to != 0x40b1565 { return; }
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
pub fn f_040b156c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b156c); }
 let mut pc: u32 = 0x40b156c;
 loop { match pc {
 0x40b156c => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x40b15b2; continue; }
  pc = 0x40b157e; continue;
 }
 0x40b157e => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b15a7; continue; }
  pc = 0x40b1582; continue;
 }
 0x40b1582 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b1632; continue; }
  pc = 0x40b158b; continue;
 }
 0x40b158b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b159e; continue;
 }
 0x40b159e => {
  { let v: u8 = cpu.or8(cpu.r8(0x7u32.wrapping_add(cpu.eax)), 0xc0u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b1632; continue;
 }
 0x40b15a7 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x40b1632; continue; }
  pc = 0x40b15b2; continue;
 }
 0x40b15b2 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x7fffu16); }
  if cpu.zf { cpu.poll(); pc = 0x40b159e; continue; }
  pc = 0x40b15be; continue;
 }
 0x40b15be => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { cpu.poll(); pc = 0x40b159e; continue; }
  pc = 0x40b15c3; continue;
 }
 0x40b15c3 => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b15cf; continue; }
  pc = 0x40b15c8; continue;
 }
 0x40b15c8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x40b158b; continue; }
  pc = 0x40b15cf; continue;
 }
 0x40b15cf => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x40b15e5; continue; }
  pc = 0x40b15df; continue;
 }
 0x40b15df => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x40b15e5; continue;
 }
 0x40b15e5 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x40b1618; continue; }
  pc = 0x40b15fa; continue;
 }
 0x40b15fa => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x40b15fb; continue;
 }
 0x40b15fb => {
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x40b160d; continue; }
  pc = 0x40b1602; continue;
 }
 0x40b1602 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x40b15fb; continue;
 }
 0x40b160d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x40b1621; continue;
 }
 0x40b1618 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40b162a; continue; }
  pc = 0x40b161c; continue;
 }
 0x40b161c => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  pc = 0x40b1621; continue;
 }
 0x40b1621 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.edx); cpu.eax = v; }
  pc = 0x40b162a; continue;
 }
 0x40b162a => {
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x40b1632; continue;
 }
 0x40b1632 => {
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
pub fn f_040b1638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1638); }
 let mut pc: u32 = 0x40b1638;
 loop { match pc {
 0x40b1638 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40b164b); f_040b1658(cpu); if cpu.returned_to != 0x40b164b { return; }
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
pub fn f_040b1658(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1658); }
 let mut pc: u32 = 0x40b1658;
 loop { match pc {
 0x40b1658 => {
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
  pc = 0x40b166f; continue;
 }
 0x40b166f => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x40b1684); f_040b045a(cpu); if cpu.returned_to != 0x40b1684 { return; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x40b169e); f_040b0056(cpu); if cpu.returned_to != 0x40b169e { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40b166f; continue; }
  pc = 0x40b16a3; continue;
 }
 0x40b16a3 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b16aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b16aa); }
 let mut pc: u32 = 0x40b16aa;
 loop { match pc {
 0x40b16aa => {
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
  cpu.push(0x40b16ca); f_040b045a(cpu); if cpu.returned_to != 0x40b16ca { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x40b16d1); f_040b1658(cpu); if cpu.returned_to != 0x40b16d1 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x40b16e5); f_040b045a(cpu); if cpu.returned_to != 0x40b16e5 { return; }
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
pub fn f_040b16f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b16f2); }
 let mut pc: u32 = 0x40b16f2;
 loop { match pc {
 0x40b16f2 => {
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
  if cpu.zf { pc = 0x40b1762; continue; }
  pc = 0x40b1705; continue;
 }
 0x40b1705 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x40b1762; continue; }
  pc = 0x40b1711; continue;
 }
 0x40b1711 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, 0x666u32); }
  if !cpu.zf { pc = 0x40b1762; continue; }
  pc = 0x40b1725; continue;
 }
 0x40b1725 => {
  { let v: u8 = 0x22u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x40b1762; continue; }
  pc = 0x40b1736; continue;
 }
 0x40b1736 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x415c7c5u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u32 = 0x42c61f4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b17bc; continue;
 }
 0x40b1762 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40b176e; continue; }
  pc = 0x40b1769; continue;
 }
 0x40b1769 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x40b17ba; continue; }
  pc = 0x40b176e; continue;
 }
 0x40b176e => {
  { let v: u32 = 0xa00u32; cpu.eax = v; }
  { let v: u32 = 0x414b66cu32; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b17ba; continue; }
  pc = 0x40b1782; continue;
 }
 0x40b1782 => {
  { let v: u32 = 0x40ae8b4u32; cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  cpu.push(0x40b1793); f_0409e9de(cpu); if cpu.returned_to != 0x40b1793 { return; }
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
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x415c7c4u32, v); }
  pc = 0x40b17ba; continue;
 }
 0x40b17ba => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b17bc; continue;
 }
 0x40b17bc => {
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
pub fn f_040b17c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b17c6); }
 let mut pc: u32 = 0x40b17c6;
 loop { match pc {
 0x40b17c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x415c7c5u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40b17f5; continue; }
  pc = 0x40b17d5; continue;
 }
 0x40b17d5 => {
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
 0x40b17f5 => {
  { cpu.sub8(cpu.r8(0x415c7c4u32), 0x0u8); }
  if cpu.zf { pc = 0x40b1819; continue; }
  pc = 0x40b17fe; continue;
 }
 0x40b17fe => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3eu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = (cpu.ebx as u8) & 0xfbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x40b1819; continue;
 }
 0x40b1819 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b181f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b181f); }
 let mut pc: u32 = 0x40b181f;
 loop { match pc {
 0x40b181f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1820); }
 let mut pc: u32 = 0x40b1820;
 loop { match pc {
 0x40b1820 => {
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
  if cpu.zf { pc = 0x40b183a; continue; }
  pc = 0x40b1838; continue;
 }
 0x40b1838 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b183a; continue;
 }
 0x40b183a => {
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
pub fn f_040b184e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b184e); }
 let mut pc: u32 = 0x40b184e;
 loop { match pc {
 0x40b184d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b184e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x40b184d; continue; }
  f_040b1852(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1852(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1852); }
 let mut pc: u32 = 0x40b1852;
 loop { match pc {
 0x40b1852 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x40b1861); f_0409ec1e(cpu); if cpu.returned_to != 0x40b1861 { return; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if !cpu.cf { pc = 0x40b18ac; continue; }
  pc = 0x40b1869; continue;
 }
 0x40b1869 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b843u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b1892; continue; }
  pc = 0x40b1875; continue;
 }
 0x40b1875 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if !cpu.zf { pc = 0x40b1882; continue; }
  pc = 0x40b187e; continue;
 }
 0x40b187e => {
  { let v: u8 = 0xeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b1892; continue;
 }
 0x40b1882 => {
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b188b; continue; }
  pc = 0x40b1887; continue;
 }
 0x40b1887 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b1892; continue;
 }
 0x40b188b => {
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b1892; continue; }
  pc = 0x40b1890; continue;
 }
 0x40b1890 => {
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b1892; continue;
 }
 0x40b1892 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x13u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b189d; continue; }
  pc = 0x40b189b; continue;
 }
 0x40b189b => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b189d; continue;
 }
 0x40b189d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x415c7d1u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x40b18b6; continue;
 }
 0x40b18ac => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x40b18b6; continue;
 }
 0x40b18b6 => {
  cpu.push(0x40b18bb); f_0409ebfb(cpu); if cpu.returned_to != 0x40b18bb { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b18c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b18c3); }
 let mut pc: u32 = 0x40b18c3;
 loop { match pc {
 0x40b18c3 => {
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
pub fn f_040b18ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b18ea); }
 let mut pc: u32 = 0x40b18ea;
 loop { match pc {
 0x40b18ea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x415c7ecu32)); }
  if cpu.cf { pc = 0x40b18fd; continue; }
  pc = 0x40b18f7; continue;
 }
 0x40b18f7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b18fd => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b1933; continue; }
  pc = 0x40b1902; continue;
 }
 0x40b1902 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415c840u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x40b1933; continue; }
  pc = 0x40b1916; continue;
 }
 0x40b1916 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b1925); f_040b18c3(cpu); if cpu.returned_to != 0x40b1925 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b1933; continue; }
  pc = 0x40b1929; continue;
 }
 0x40b1929 => {
  { let v: u32 = cpu.r32(0x415c840u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40b1933; continue;
 }
 0x40b1933 => {
  { let v: u32 = cpu.r32(0x415c840u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b193f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b193f); }
 let mut pc: u32 = 0x40b193f;
 loop { match pc {
 0x40b193f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415c840u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1954); }
 let mut pc: u32 = 0x40b1954;
 loop { match pc {
 0x40b1954 => {
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
  if (cpu.sf != cpu.of) { pc = 0x40b1976; continue; }
  pc = 0x40b1967; continue;
 }
 0x40b1967 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x40b1970); f_040b193f(cpu); if cpu.returned_to != 0x40b1970 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b1976 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x40b1980); f_0409ebfb(cpu); if cpu.returned_to != 0x40b1980 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1989); }
 let mut pc: u32 = 0x40b1989;
 loop { match pc {
 0x40b1989 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x415b81cu32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b19b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b19b3); }
 let mut pc: u32 = 0x40b19b3;
 loop { match pc {
 0x40b19b3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x40b19d8; continue; }
  pc = 0x40b19c0; continue;
 }
 0x40b19c0 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x40b19ca); f_0409ebfb(cpu); if cpu.returned_to != 0x40b19ca { return; }
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x40b19ce; continue;
 }
 0x40b19ce => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40b1a51; continue;
 }
 0x40b19d8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b19e5; continue; }
  pc = 0x40b19de; continue;
 }
 0x40b19de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b19e5); f_040b1a57(cpu); if cpu.returned_to != 0x40b19e5 { return; }
  pc = 0x40b19e5; continue;
 }
 0x40b19e5 => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x40b1a22; continue; }
  pc = 0x40b19ef; continue;
 }
 0x40b19ef => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x40b1a22; continue; }
  pc = 0x40b19fb; continue;
 }
 0x40b19fb => {
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
  if !cpu.zf { pc = 0x40b1a22; continue; }
  pc = 0x40b1a17; continue;
 }
 0x40b1a17 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b1a1e); f_040accda(cpu); if cpu.returned_to != 0x40b1a1e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b19ce; continue; }
  pc = 0x40b1a22; continue;
 }
 0x40b1a22 => {
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
  if !cpu.zf { pc = 0x40b1a42; continue; }
  pc = 0x40b1a3d; continue;
 }
 0x40b1a3d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x40b1a4d; continue; }
  pc = 0x40b1a42; continue;
 }
 0x40b1a42 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b1a49); f_040accda(cpu); if cpu.returned_to != 0x40b1a49 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b19ce; continue; }
  pc = 0x40b1a4d; continue;
 }
 0x40b1a4d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b1a51; continue;
 }
 0x40b1a51 => {
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
pub fn f_040b1a57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1a57); }
 let mut pc: u32 = 0x40b1a57;
 loop { match pc {
 0x40b1a57 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x40b1a60); f_040b5150(cpu); if cpu.returned_to != 0x40b1a60 { return; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b1a8c; continue; }
  pc = 0x40b1a66; continue;
 }
 0x40b1a66 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x40b1a77; continue; }
  pc = 0x40b1a6e; continue;
 }
 0x40b1a6e => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b1a8c; continue;
 }
 0x40b1a77 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b1a85; continue; }
  pc = 0x40b1a7c; continue;
 }
 0x40b1a7c => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b1a8c; continue;
 }
 0x40b1a85 => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b1a8c; continue;
 }
 0x40b1a8c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x40b1a94); f_040ad06c(cpu); if cpu.returned_to != 0x40b1a94 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b1abb; continue; }
  pc = 0x40b1a9b; continue;
 }
 0x40b1a9b => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x40b1abf; continue;
 }
 0x40b1abb => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x40b1abf; continue;
 }
 0x40b1abf => {
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
pub fn f_040b1ace(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1ace); }
 let mut pc: u32 = 0x40b1ace;
 loop { match pc {
 0x40b1ace => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b1aec; continue; }
  pc = 0x40b1ad9; continue;
 }
 0x40b1ad9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x40b1aec; continue;
 }
 0x40b1aec => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b1b03; continue; }
  pc = 0x40b1af0; continue;
 }
 0x40b1af0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b1b03 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1b0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1b0c); }
 let mut pc: u32 = 0x40b1b0c;
 loop { match pc {
 0x40b1b0c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x40b1b22; continue; }
  pc = 0x40b1b16; continue;
 }
 0x40b1b16 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b1b22 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1b26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1b26); }
 let mut pc: u32 = 0x40b1b26;
 loop { match pc {
 0x40b1b26 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x40b1b46; continue; }
  pc = 0x40b1b2c; continue;
 }
 0x40b1b2c => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x42c6274u32, v); }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x42c6275u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x42c6276u32, v); }
  pc = 0x40b1b53; continue;
 }
 0x40b1b46 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x42c6274u32, v); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x42c6275u32, v); }
  pc = 0x40b1b53; continue;
 }
 0x40b1b53 => {
  { let v: u32 = 0x42c6274u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1b5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1b5a); }
 let mut pc: u32 = 0x40b1b5a;
 loop { match pc {
 0x40b1b5a => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.push(0x40b1b61); f_040b1b88(cpu); if cpu.returned_to != 0x40b1b61 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b1b6a; continue; }
  pc = 0x40b1b65; continue;
 }
 0x40b1b65 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x40b1b85; continue; }
  pc = 0x40b1b6a; continue;
 }
 0x40b1b6a => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if !cpu.zf { pc = 0x40b1b7b; continue; }
  pc = 0x40b1b73; continue;
 }
 0x40b1b73 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if cpu.zf { pc = 0x40b1b85; continue; }
  pc = 0x40b1b7b; continue;
 }
 0x40b1b7b => {
  { let v: u32 = 0x1u32; cpu.w32(0x415c844u32, v); }
  pc = 0x40b1b85; continue;
 }
 0x40b1b85 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1b88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1b88); }
 let mut pc: u32 = 0x40b1b88;
 loop { match pc {
 0x40b1b88 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b1c2e; continue; }
  pc = 0x40b1b9e; continue;
 }
 0x40b1b9e => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b1c2e; continue; }
  pc = 0x40b1ba7; continue;
 }
 0x40b1ba7 => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x40b1bb7); f_0409e8e0(cpu); if cpu.returned_to != 0x40b1bb7 { return; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x2511u32; cpu.ecx = v; }
  cpu.push(0x40b1bcc); f_0409e8e0(cpu); if cpu.returned_to != 0x40b1bcc { return; }
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
  cpu.push(0x40b1c00); f_040b5181(cpu); if cpu.returned_to != 0x40b1c00 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x40b1c8a; continue; }
  pc = 0x40b1c12; continue;
 }
 0x40b1c12 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x40b1c23; continue;
 }
 0x40b1c23 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x42c6288u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x40b1c8e; continue;
 }
 0x40b1c2e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b1c8a; continue; }
  pc = 0x40b1c3a; continue;
 }
 0x40b1c3a => {
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x40b1c4a); f_0409e8e0(cpu); if cpu.returned_to != 0x40b1c4a { return; }
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
  if !cpu.zf { pc = 0x40b1c8a; continue; }
  pc = 0x40b1c76; continue;
 }
 0x40b1c76 => {
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x40b1c23; continue;
 }
 0x40b1c8a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b1c8e; continue;
 }
 0x40b1c8e => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1c95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1c95); }
 let mut pc: u32 = 0x40b1c95;
 loop { match pc {
 0x40b1c95 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40b1cf3; continue; }
  pc = 0x40b1ca5; continue;
 }
 0x40b1ca5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40b1cfe; continue; }
  pc = 0x40b1caf; continue;
 }
 0x40b1caf => {
  { let v: u32 = cpu.r32(0x415c844u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b1cd0; continue; }
  pc = 0x40b1cbb; continue;
 }
 0x40b1cbb => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  pc = 0x40b1cd2; continue;
 }
 0x40b1cd0 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x40b1cd2; continue;
 }
 0x40b1cd2 => {
  if cpu.zf { pc = 0x40b1cfe; continue; }
  pc = 0x40b1cd4; continue;
 }
 0x40b1cd4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x40b1cec; continue; }
  pc = 0x40b1cda; continue;
 }
 0x40b1cda => {
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
 0x40b1cec => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b1cf3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40b1cfe; continue;
 }
 0x40b1cfe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1d02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1d02); }
 let mut pc: u32 = 0x40b1d02;
 loop { match pc {
 0x40b1d02 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b1d4d; continue; }
  pc = 0x40b1d0e; continue;
 }
 0x40b1d0e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b1d4f; continue; }
  pc = 0x40b1d16; continue;
 }
 0x40b1d16 => {
  { let v: u32 = cpu.r32(0x415c844u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b1d34; continue; }
  pc = 0x40b1d21; continue;
 }
 0x40b1d21 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  pc = 0x40b1d34; continue;
 }
 0x40b1d34 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b1d4f; continue; }
  pc = 0x40b1d38; continue;
 }
 0x40b1d38 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40b1d4f; continue; }
  pc = 0x40b1d3d; continue;
 }
 0x40b1d3d => {
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
 0x40b1d4d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b1d4f; continue;
 }
 0x40b1d4f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1d9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1d9a); }
 let mut pc: u32 = 0x40b1d9a;
 loop { match pc {
 0x40b1d9a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.cf { pc = 0x40b1db2; continue; }
  pc = 0x40b1dab; continue;
 }
 0x40b1dab => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x40b1e14; continue;
 }
 0x40b1db2 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b1e14; continue; }
  pc = 0x40b1db7; continue;
 }
 0x40b1db7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  if cpu.zf { pc = 0x40b1dd4; continue; }
  pc = 0x40b1dbb; continue;
 }
 0x40b1dbb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x40b1dd6; continue;
 }
 0x40b1dd4 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x40b1dd6; continue;
 }
 0x40b1dd6 => {
  if cpu.zf { pc = 0x40b1ddd; continue; }
  pc = 0x40b1dd8; continue;
 }
 0x40b1dd8 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x40b1e14; continue;
 }
 0x40b1ddd => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b1dff; continue; }
  pc = 0x40b1de2; continue;
 }
 0x40b1de2 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b1dfb; continue; }
  pc = 0x40b1de6; continue;
 }
 0x40b1de6 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x40b1dfd; continue;
 }
 0x40b1dfb => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x40b1dfd; continue;
 }
 0x40b1dfd => {
  if !cpu.zf { cpu.poll(); pc = 0x40b1ddd; continue; }
  pc = 0x40b1dff; continue;
 }
 0x40b1dff => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40b1e14; continue;
 }
 0x40b1e14 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415c844u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1e20); }
 let mut pc: u32 = 0x40b1e20;
 loop { match pc {
 0x40b1e20 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x41544ddu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b1e71; continue; }
  pc = 0x40b1e4e; continue;
 }
 0x40b1e4e => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x414b67cu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40b1e71; continue; }
  pc = 0x40b1e6a; continue;
 }
 0x40b1e6a => {
  { let v: u8 = cpu.r8(0x415b8f0u32) | 0x1u8; cpu.w8(0x415b8f0u32, v); }
  pc = 0x40b1e71; continue;
 }
 0x40b1e71 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1e75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1e75); }
 let mut pc: u32 = 0x40b1e75;
 loop { match pc {
 0x40b1e75 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x40b1e7d); f_040b1e7f(cpu); if cpu.returned_to != 0x40b1e7d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1e7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1e7f); }
 let mut pc: u32 = 0x40b1e7f;
 loop { match pc {
 0x40b1e7f => {
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
  if cpu.zf { pc = 0x40b1eab; continue; }
  pc = 0x40b1ea4; continue;
 }
 0x40b1ea4 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1eab => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x40b1eb7; continue; }
  pc = 0x40b1eb0; continue;
 }
 0x40b1eb0 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1eb7 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b1ec3; continue; }
  pc = 0x40b1ebc; continue;
 }
 0x40b1ebc => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1ec3 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x40b1ecf; continue; }
  pc = 0x40b1ec8; continue;
 }
 0x40b1ec8 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1ecf => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x40b1edb; continue; }
  pc = 0x40b1ed4; continue;
 }
 0x40b1ed4 => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1edb => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b1ee5; continue; }
  pc = 0x40b1ee0; continue;
 }
 0x40b1ee0 => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x40b1ee5; continue;
 }
 0x40b1ee5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415c948u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x40b1f09; continue; }
  pc = 0x40b1efc; continue;
 }
 0x40b1efc => {
  { let v = cpu.rf64(0x414b930u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40b1f4b; continue;
 }
 0x40b1f09 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x40b1f18; continue; }
  pc = 0x40b1f0e; continue;
 }
 0x40b1f0e => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b1f4b; continue;
 }
 0x40b1f18 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x40b1f2c; continue; }
  pc = 0x40b1f1d; continue;
 }
 0x40b1f1d => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b1f4b; continue;
 }
 0x40b1f2c => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x40b1f40; continue; }
  pc = 0x40b1f31; continue;
 }
 0x40b1f31 => {
  { let v: u32 = cpu.r32(0x414b930u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x414b934u32); cpu.eax = v; }
  pc = 0x40b1f48; continue;
 }
 0x40b1f40 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x40b1f48; continue;
 }
 0x40b1f48 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b1f4b; continue;
 }
 0x40b1f4b => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x40b1f53); f_040b52f0(cpu); if cpu.returned_to != 0x40b1f53 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b1f5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b1f5a); }
 let mut pc: u32 = 0x40b1f5a;
 loop { match pc {
 0x40b1f5a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.cf { pc = 0x40b1f9c; continue; }
  pc = 0x40b1f6b; continue;
 }
 0x40b1f6b => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x40b1f77; continue; }
  pc = 0x40b1f70; continue;
 }
 0x40b1f70 => {
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b1f88; continue; }
  pc = 0x40b1f75; continue;
 }
 0x40b1f75 => {
  pc = 0x40b1f9c; continue;
 }
 0x40b1f77 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40b1fc0; continue;
 }
 0x40b1f88 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40b1f9c; continue; }
  pc = 0x40b1f92; continue;
 }
 0x40b1f92 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b1fc8; continue;
 }
 0x40b1f9c => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x81u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.edx, 0x6u32); }
  if !cpu.zf { pc = 0x40b1fbb; continue; }
  pc = 0x40b1fac; continue;
 }
 0x40b1fac => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40b1fbb; continue; }
  pc = 0x40b1fb6; continue;
 }
 0x40b1fb6 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x11u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x40b1fbb; continue;
 }
 0x40b1fbb => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40b1fc0; continue;
 }
 0x40b1fc0 => {
  cpu.push(0x40b1fc5); f_040b1e75(cpu); if cpu.returned_to != 0x40b1fc5 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40b1fc8; continue;
 }
 0x40b1fc8 => {
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
pub fn f_040b1fd8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b1fd8); } r_040b1fd8(cpu, 0x40b1fd8); }
pub fn f_040b21d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b21d8); } r_040b1fd8(cpu, 0x40b21d8); }
pub fn f_040b23d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b23d8); } r_040b1fd8(cpu, 0x40b23d8); }
pub fn f_040b2414(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b2414); } r_040b1fd8(cpu, 0x40b2414); }
fn r_040b1fd8(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b1fd8 => {
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x14u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x28u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x47u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x24u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x24u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(2))); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xa7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xcd040b24u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xcd040b24u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xf2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax & 0x2524040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x255b040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2596040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x25d5040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x2612040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x65040b26u32.wrapping_add(cpu.ecx.wrapping_mul(2)).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(4)).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(4)).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(8)).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(8)).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)).wrapping_add(cpu.bases[0]))); cpu.eax = v; }
  cpu.unimplemented(0x40b20dd, "Daa");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
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
  { let v: u8 = cpu.r8(0x27u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x27u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.eax = v; }
  cpu.unimplemented(0x40b211d, "Daa");
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  cpu.unimplemented(0x40b2121, "Daa");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_sub((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xe5040b28u32.wrapping_add(cpu.esi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xe5040b28u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_sub((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x31u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x88u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_sub(cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x61u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = cpu.r8(0xe9040b2au32.wrapping_add(cpu.esi)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xe9040b2au32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x39u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x94u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xe4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x43u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x97u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xf6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0xbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2da7040bu32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2e56040bu32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.eax = v; }
  cpu.unimplemented(0x40b21cd, "Das");
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(2)))); cpu.eax = v; }
  cpu.unimplemented(0x40b21d1, "Das");
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(8)))); cpu.eax = v; }
  cpu.unimplemented(0x40b21d5, "Das");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x37u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x30u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xbf040b30u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xbf040b30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xe3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x72u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x40b31u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x40b31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xfbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xbau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x69u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x9bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x54u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x40b34u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x40b34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x353c040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xbu32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xda040b35u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xda040b35u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ 0x35f6040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x36u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)))); cpu.eax = v; }
  cpu.unimplemented(0x40b2301, "Aaa");
  { let v: u32 = cpu.eax | cpu.r32(0x40b37u32.wrapping_add(cpu.ebp.wrapping_mul(2))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xf4040b37u32.wrapping_add(cpu.ecx)), ((cpu.edx >> 8) as u8)); cpu.w8(0xf4040b37u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x40b2315, "Aaa");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5du8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x92u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xcau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x74u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x9bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xccu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x75040b3au32).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x75040b3au32, v); }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xaeu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xe6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x22u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xbu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xa7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xeau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x40bu32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b240f; continue; }
  pc = 0x40b23d2; continue;
 }
 0x40b21d8 => {
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x37u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x30u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x30u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xbf040b30u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xbf040b30u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xe3u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) ^ (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x72u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x40b31u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x40b31u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xfbu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x32u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x32u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xbau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x69u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x9bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.edx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x54u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x40b34u32.wrapping_add(cpu.ebp)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x40b34u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ebx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) ^ 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xfu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x353c040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xbu32.wrapping_add(cpu.ebp).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xda040b35u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0xda040b35u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ 0x35f6040bu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x36u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x36u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)))); cpu.eax = v; }
  cpu.unimplemented(0x40b2301, "Aaa");
  { let v: u32 = cpu.eax | cpu.r32(0x40b37u32.wrapping_add(cpu.ebp.wrapping_mul(2))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0xf4040b37u32.wrapping_add(cpu.ecx)), ((cpu.edx >> 8) as u8)); cpu.w8(0xf4040b37u32.wrapping_add(cpu.ecx), v); }
  cpu.unimplemented(0x40b2315, "Aaa");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x5du8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x92u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xcau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x3bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x74u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x9bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xccu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), cpu.ecx); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x4u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x75040b3au32).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x75040b3au32, v); }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xaeu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xe6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2eu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x7fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.edx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x22u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1))).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xbu32.wrapping_add(cpu.esp).wrapping_add(cpu.edi.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xa7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xeau8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x40bu32); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b240f; continue; }
  pc = 0x40b23d2; continue;
 }
 0x40b23d2 => {
  { let v: u32 = cpu.eax | cpu.r32(0x8b040b3du32.wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b23d8; continue; }
  pc = 0x40b23e8; continue;
 }
 0x40b23d7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x8bu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b23d8; continue; }
  pc = 0x40b23e8; continue;
 }
 0x40b23d8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b23d8; continue; }
  pc = 0x40b23e8; continue;
 }
 0x40b23e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b240f => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x75u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x40b23d7; continue; }
  pc = 0x40b2414; continue;
 }
 0x40b2414 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b2414; continue; }
  pc = 0x40b2427; continue;
 }
 0x40b2427 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
