#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_050a6810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6810); }
 let mut pc: u32 = 0x50a6810;
 loop { match pc {
 0x50a6810 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6813(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6813); }
 let mut pc: u32 = 0x50a6813;
 loop { match pc {
 0x50a6813 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6816(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6816); }
 let mut pc: u32 = 0x50a6816;
 loop { match pc {
 0x50a6816 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6819(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6819); }
 let mut pc: u32 = 0x50a6819;
 loop { match pc {
 0x50a6819 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a681c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a681c); }
 let mut pc: u32 = 0x50a681c;
 loop { match pc {
 0x50a681c => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a681f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a681f); }
 let mut pc: u32 = 0x50a681f;
 loop { match pc {
 0x50a681f => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6822(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6822); }
 let mut pc: u32 = 0x50a6822;
 loop { match pc {
 0x50a6822 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6825(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6825); }
 let mut pc: u32 = 0x50a6825;
 loop { match pc {
 0x50a6825 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6828(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6828); }
 let mut pc: u32 = 0x50a6828;
 loop { match pc {
 0x50a6828 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a682b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a682b); }
 let mut pc: u32 = 0x50a682b;
 loop { match pc {
 0x50a682b => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a682e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a682e); }
 let mut pc: u32 = 0x50a682e;
 loop { match pc {
 0x50a682e => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6831(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6831); }
 let mut pc: u32 = 0x50a6831;
 loop { match pc {
 0x50a6831 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6834); }
 let mut pc: u32 = 0x50a6834;
 loop { match pc {
 0x50a6834 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6837(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6837); }
 let mut pc: u32 = 0x50a6837;
 loop { match pc {
 0x50a6837 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a683a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a683a); }
 let mut pc: u32 = 0x50a683a;
 loop { match pc {
 0x50a683a => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a683d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a683d); }
 let mut pc: u32 = 0x50a683d;
 loop { match pc {
 0x50a683d => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6840); }
 let mut pc: u32 = 0x50a6840;
 loop { match pc {
 0x50a6840 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6843(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6843); }
 let mut pc: u32 = 0x50a6843;
 loop { match pc {
 0x50a6843 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6846(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6846); }
 let mut pc: u32 = 0x50a6846;
 loop { match pc {
 0x50a6846 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6849(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6849); }
 let mut pc: u32 = 0x50a6849;
 loop { match pc {
 0x50a6849 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a684c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a684c); }
 let mut pc: u32 = 0x50a684c;
 loop { match pc {
 0x50a684c => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a684f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a684f); }
 let mut pc: u32 = 0x50a684f;
 loop { match pc {
 0x50a684f => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6852(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6852); }
 let mut pc: u32 = 0x50a6852;
 loop { match pc {
 0x50a6852 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6855); }
 let mut pc: u32 = 0x50a6855;
 loop { match pc {
 0x50a6855 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6858(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6858); }
 let mut pc: u32 = 0x50a6858;
 loop { match pc {
 0x50a6858 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a685b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a685b); }
 let mut pc: u32 = 0x50a685b;
 loop { match pc {
 0x50a685b => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a685e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a685e); }
 let mut pc: u32 = 0x50a685e;
 loop { match pc {
 0x50a685e => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6861(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6861); }
 let mut pc: u32 = 0x50a6861;
 loop { match pc {
 0x50a6861 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6864); }
 let mut pc: u32 = 0x50a6864;
 loop { match pc {
 0x50a6864 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6867); }
 let mut pc: u32 = 0x50a6867;
 loop { match pc {
 0x50a6867 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a686a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a686a); }
 let mut pc: u32 = 0x50a686a;
 loop { match pc {
 0x50a686a => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a686d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a686d); }
 let mut pc: u32 = 0x50a686d;
 loop { match pc {
 0x50a686d => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6870(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6870); }
 let mut pc: u32 = 0x50a6870;
 loop { match pc {
 0x50a6870 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6873(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6873); }
 let mut pc: u32 = 0x50a6873;
 loop { match pc {
 0x50a6873 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6876(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6876); }
 let mut pc: u32 = 0x50a6876;
 loop { match pc {
 0x50a6876 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6879(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6879); }
 let mut pc: u32 = 0x50a6879;
 loop { match pc {
 0x50a6879 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a687c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a687c); }
 let mut pc: u32 = 0x50a687c;
 loop { match pc {
 0x50a687c => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a687f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a687f); }
 let mut pc: u32 = 0x50a687f;
 loop { match pc {
 0x50a687f => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6882(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6882); }
 let mut pc: u32 = 0x50a6882;
 loop { match pc {
 0x50a6882 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6885(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6885); }
 let mut pc: u32 = 0x50a6885;
 loop { match pc {
 0x50a6885 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6888(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6888); }
 let mut pc: u32 = 0x50a6888;
 loop { match pc {
 0x50a6888 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a688b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a688b); }
 let mut pc: u32 = 0x50a688b;
 loop { match pc {
 0x50a688b => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a688e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a688e); }
 let mut pc: u32 = 0x50a688e;
 loop { match pc {
 0x50a688e => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6891(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6891); }
 let mut pc: u32 = 0x50a6891;
 loop { match pc {
 0x50a6891 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6894(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6894); }
 let mut pc: u32 = 0x50a6894;
 loop { match pc {
 0x50a6894 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6897(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6897); }
 let mut pc: u32 = 0x50a6897;
 loop { match pc {
 0x50a6897 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a689a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a689a); }
 let mut pc: u32 = 0x50a689a;
 loop { match pc {
 0x50a689a => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a689d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a689d); }
 let mut pc: u32 = 0x50a689d;
 loop { match pc {
 0x50a689d => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68a0); }
 let mut pc: u32 = 0x50a68a0;
 loop { match pc {
 0x50a68a0 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68a3); }
 let mut pc: u32 = 0x50a68a3;
 loop { match pc {
 0x50a68a3 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68a6); }
 let mut pc: u32 = 0x50a68a6;
 loop { match pc {
 0x50a68a6 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68a9); }
 let mut pc: u32 = 0x50a68a9;
 loop { match pc {
 0x50a68a9 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68ac); }
 let mut pc: u32 = 0x50a68ac;
 loop { match pc {
 0x50a68ac => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68af); }
 let mut pc: u32 = 0x50a68af;
 loop { match pc {
 0x50a68af => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68b2); }
 let mut pc: u32 = 0x50a68b2;
 loop { match pc {
 0x50a68b2 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68b5); }
 let mut pc: u32 = 0x50a68b5;
 loop { match pc {
 0x50a68b5 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68b8); }
 let mut pc: u32 = 0x50a68b8;
 loop { match pc {
 0x50a68b8 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68bb); }
 let mut pc: u32 = 0x50a68bb;
 loop { match pc {
 0x50a68bb => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68be); }
 let mut pc: u32 = 0x50a68be;
 loop { match pc {
 0x50a68be => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68c1); }
 let mut pc: u32 = 0x50a68c1;
 loop { match pc {
 0x50a68c1 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68c4); }
 let mut pc: u32 = 0x50a68c4;
 loop { match pc {
 0x50a68c4 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68c7); }
 let mut pc: u32 = 0x50a68c7;
 loop { match pc {
 0x50a68c7 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68ca); }
 let mut pc: u32 = 0x50a68ca;
 loop { match pc {
 0x50a68ca => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68cd); }
 let mut pc: u32 = 0x50a68cd;
 loop { match pc {
 0x50a68cd => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68d0); }
 let mut pc: u32 = 0x50a68d0;
 loop { match pc {
 0x50a68d0 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68d3); }
 let mut pc: u32 = 0x50a68d3;
 loop { match pc {
 0x50a68d3 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68d6); }
 let mut pc: u32 = 0x50a68d6;
 loop { match pc {
 0x50a68d6 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68d9); }
 let mut pc: u32 = 0x50a68d9;
 loop { match pc {
 0x50a68d9 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68dc); }
 let mut pc: u32 = 0x50a68dc;
 loop { match pc {
 0x50a68dc => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68df); }
 let mut pc: u32 = 0x50a68df;
 loop { match pc {
 0x50a68df => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68e2); }
 let mut pc: u32 = 0x50a68e2;
 loop { match pc {
 0x50a68e2 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68e5); }
 let mut pc: u32 = 0x50a68e5;
 loop { match pc {
 0x50a68e5 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68e8); }
 let mut pc: u32 = 0x50a68e8;
 loop { match pc {
 0x50a68e8 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68eb); }
 let mut pc: u32 = 0x50a68eb;
 loop { match pc {
 0x50a68eb => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68ee); }
 let mut pc: u32 = 0x50a68ee;
 loop { match pc {
 0x50a68ee => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68f1); }
 let mut pc: u32 = 0x50a68f1;
 loop { match pc {
 0x50a68f1 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68f4); }
 let mut pc: u32 = 0x50a68f4;
 loop { match pc {
 0x50a68f4 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68f7); }
 let mut pc: u32 = 0x50a68f7;
 loop { match pc {
 0x50a68f7 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68fa); }
 let mut pc: u32 = 0x50a68fa;
 loop { match pc {
 0x50a68fa => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a68fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a68fd); }
 let mut pc: u32 = 0x50a68fd;
 loop { match pc {
 0x50a68fd => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6900); }
 let mut pc: u32 = 0x50a6900;
 loop { match pc {
 0x50a6900 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6903(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6903); }
 let mut pc: u32 = 0x50a6903;
 loop { match pc {
 0x50a6903 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6906(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6906); }
 let mut pc: u32 = 0x50a6906;
 loop { match pc {
 0x50a6906 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6909(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6909); }
 let mut pc: u32 = 0x50a6909;
 loop { match pc {
 0x50a6909 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a690c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a690c); }
 let mut pc: u32 = 0x50a690c;
 loop { match pc {
 0x50a690c => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a690f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a690f); }
 let mut pc: u32 = 0x50a690f;
 loop { match pc {
 0x50a690f => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6912(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6912); }
 let mut pc: u32 = 0x50a6912;
 loop { match pc {
 0x50a6912 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6915(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6915); }
 let mut pc: u32 = 0x50a6915;
 loop { match pc {
 0x50a6915 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6918); }
 let mut pc: u32 = 0x50a6918;
 loop { match pc {
 0x50a6918 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a691b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a691b); }
 let mut pc: u32 = 0x50a691b;
 loop { match pc {
 0x50a691b => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a691e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a691e); }
 let mut pc: u32 = 0x50a691e;
 loop { match pc {
 0x50a691e => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6921(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6921); }
 let mut pc: u32 = 0x50a6921;
 loop { match pc {
 0x50a6921 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6924(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6924); }
 let mut pc: u32 = 0x50a6924;
 loop { match pc {
 0x50a6924 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6927(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6927); }
 let mut pc: u32 = 0x50a6927;
 loop { match pc {
 0x50a6927 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a692a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a692a); }
 let mut pc: u32 = 0x50a692a;
 loop { match pc {
 0x50a692a => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a692d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a692d); }
 let mut pc: u32 = 0x50a692d;
 loop { match pc {
 0x50a692d => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6930); }
 let mut pc: u32 = 0x50a6930;
 loop { match pc {
 0x50a6930 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6933(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6933); }
 let mut pc: u32 = 0x50a6933;
 loop { match pc {
 0x50a6933 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6936(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6936); }
 let mut pc: u32 = 0x50a6936;
 loop { match pc {
 0x50a6936 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6939(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6939); }
 let mut pc: u32 = 0x50a6939;
 loop { match pc {
 0x50a6939 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a693c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a693c); }
 let mut pc: u32 = 0x50a693c;
 loop { match pc {
 0x50a693c => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a693f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a693f); }
 let mut pc: u32 = 0x50a693f;
 loop { match pc {
 0x50a693f => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6942(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6942); }
 let mut pc: u32 = 0x50a6942;
 loop { match pc {
 0x50a6942 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6945(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6945); }
 let mut pc: u32 = 0x50a6945;
 loop { match pc {
 0x50a6945 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6948); }
 let mut pc: u32 = 0x50a6948;
 loop { match pc {
 0x50a6948 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a694b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a694b); }
 let mut pc: u32 = 0x50a694b;
 loop { match pc {
 0x50a694b => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a694e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a694e); }
 let mut pc: u32 = 0x50a694e;
 loop { match pc {
 0x50a694e => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6951(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6951); }
 let mut pc: u32 = 0x50a6951;
 loop { match pc {
 0x50a6951 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6954(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6954); }
 let mut pc: u32 = 0x50a6954;
 loop { match pc {
 0x50a6954 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6957(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6957); }
 let mut pc: u32 = 0x50a6957;
 loop { match pc {
 0x50a6957 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a695a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a695a); }
 let mut pc: u32 = 0x50a695a;
 loop { match pc {
 0x50a695a => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a695d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a695d); }
 let mut pc: u32 = 0x50a695d;
 loop { match pc {
 0x50a695d => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6960); }
 let mut pc: u32 = 0x50a6960;
 loop { match pc {
 0x50a6960 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6963(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6963); }
 let mut pc: u32 = 0x50a6963;
 loop { match pc {
 0x50a6963 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6966(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6966); }
 let mut pc: u32 = 0x50a6966;
 loop { match pc {
 0x50a6966 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6969(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6969); }
 let mut pc: u32 = 0x50a6969;
 loop { match pc {
 0x50a6969 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a696c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a696c); }
 let mut pc: u32 = 0x50a696c;
 loop { match pc {
 0x50a696c => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a696f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a696f); }
 let mut pc: u32 = 0x50a696f;
 loop { match pc {
 0x50a696f => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6972(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6972); }
 let mut pc: u32 = 0x50a6972;
 loop { match pc {
 0x50a6972 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6975(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6975); }
 let mut pc: u32 = 0x50a6975;
 loop { match pc {
 0x50a6975 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6978(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6978); }
 let mut pc: u32 = 0x50a6978;
 loop { match pc {
 0x50a6978 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a697b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a697b); }
 let mut pc: u32 = 0x50a697b;
 loop { match pc {
 0x50a697b => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a697e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a697e); }
 let mut pc: u32 = 0x50a697e;
 loop { match pc {
 0x50a697e => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6981(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6981); }
 let mut pc: u32 = 0x50a6981;
 loop { match pc {
 0x50a6981 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6984(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6984); }
 let mut pc: u32 = 0x50a6984;
 loop { match pc {
 0x50a6984 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6987); }
 let mut pc: u32 = 0x50a6987;
 loop { match pc {
 0x50a6987 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a698a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a698a); }
 let mut pc: u32 = 0x50a698a;
 loop { match pc {
 0x50a698a => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a698d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a698d); }
 let mut pc: u32 = 0x50a698d;
 loop { match pc {
 0x50a698d => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6990); }
 let mut pc: u32 = 0x50a6990;
 loop { match pc {
 0x50a6990 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6993(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6993); }
 let mut pc: u32 = 0x50a6993;
 loop { match pc {
 0x50a6993 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6996(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6996); }
 let mut pc: u32 = 0x50a6996;
 loop { match pc {
 0x50a6996 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6999(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6999); }
 let mut pc: u32 = 0x50a6999;
 loop { match pc {
 0x50a6999 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a699c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a699c); }
 let mut pc: u32 = 0x50a699c;
 loop { match pc {
 0x50a699c => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a699f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a699f); }
 let mut pc: u32 = 0x50a699f;
 loop { match pc {
 0x50a699f => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69a2); }
 let mut pc: u32 = 0x50a69a2;
 loop { match pc {
 0x50a69a2 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69a5); }
 let mut pc: u32 = 0x50a69a5;
 loop { match pc {
 0x50a69a5 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69a8); }
 let mut pc: u32 = 0x50a69a8;
 loop { match pc {
 0x50a69a8 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69ab); }
 let mut pc: u32 = 0x50a69ab;
 loop { match pc {
 0x50a69ab => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69ae); }
 let mut pc: u32 = 0x50a69ae;
 loop { match pc {
 0x50a69ae => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x50a69b5); f_050a6abd(cpu); if cpu.returned_to != 0x50a69b5 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x50a69c0; continue;
 }
 0x50a69c0 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x50a69c9; continue; }
  pc = 0x50a69c5; continue;
 }
 0x50a69c5 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x50a69c9; continue;
 }
 0x50a69c9 => {
  if cpu.cf { pc = 0x50a69d1; continue; }
  pc = 0x50a69cb; continue;
 }
 0x50a69cb => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50a69c0; continue;
 }
 0x50a69d1 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x50a69d4; continue;
 }
 0x50a69d4 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x50a69dd; continue; }
  pc = 0x50a69d9; continue;
 }
 0x50a69d9 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x50a69dd; continue;
 }
 0x50a69dd => {
  if !cpu.cf { pc = 0x50a69e5; continue; }
  pc = 0x50a69df; continue;
 }
 0x50a69df => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x50a69d4; continue;
 }
 0x50a69e5 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a69e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a69e8); }
 let mut pc: u32 = 0x50a69e8;
 loop { match pc {
 0x50a69e8 => {
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
  if cpu.zf { pc = 0x50a6a46; continue; }
  pc = 0x50a6a11; continue;
 }
 0x50a6a11 => {
  if (cpu.cf || cpu.zf) { pc = 0x50a6a1c; continue; }
  pc = 0x50a6a13; continue;
 }
 0x50a6a13 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a6a13; continue; }
  pc = 0x50a6a1a; continue;
 }
 0x50a6a1a => {
  pc = 0x50a6a46; continue;
 }
 0x50a6a1c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x50a6a20; continue;
 }
 0x50a6a20 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a6a20; continue; }
  pc = 0x50a6a2b; continue;
 }
 0x50a6a2b => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x50a6a46; continue; }
  pc = 0x50a6a32; continue;
 }
 0x50a6a32 => {
  if !cpu.zf { pc = 0x50a6a40; continue; }
  pc = 0x50a6a34; continue;
 }
 0x50a6a34 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x50a6a40; continue; }
  pc = 0x50a6a38; continue;
 }
 0x50a6a38 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x50a6a46; continue; }
  pc = 0x50a6a40; continue;
 }
 0x50a6a40 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x50a6a46; continue;
 }
 0x50a6a46 => {
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
pub fn f_050a6a53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6a53); }
 let mut pc: u32 = 0x50a6a53;
 loop { match pc {
 0x50a6a53 => {
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
  pc = 0x50a6a68; continue;
 }
 0x50a6a68 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x50a6a7b; continue; }
  pc = 0x50a6a74; continue;
 }
 0x50a6a74 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x50a6a7d; continue; }
  pc = 0x50a6a78; continue;
 }
 0x50a6a78 => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x50a6a93; continue;
 }
 0x50a6a7b => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x50a6a7d; continue;
 }
 0x50a6a7d => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x50a6a8c, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x50a6a91, "Aam");
  pc = 0x50a6a93; continue;
 }
 0x50a6a93 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x50a6ab7; continue; }
  pc = 0x50a6aa5; continue;
 }
 0x50a6aa5 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x50a6ab7; continue; }
  pc = 0x50a6aab; continue;
 }
 0x50a6aab => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x50a6ab7; continue; }
  pc = 0x50a6ab1; continue;
 }
 0x50a6ab1 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50a6a68; continue; }
  pc = 0x50a6ab7; continue;
 }
 0x50a6ab7 => {
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
pub fn f_050a6abd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6abd); }
 let mut pc: u32 = 0x50a6abd;
 loop { match pc {
 0x50a6abd => {
  cpu.push(0x50a6ac2); f_050a6b72(cpu); if cpu.returned_to != 0x50a6ac2 { return; }
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
  cpu.unimplemented(0x50a6aef, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x50a6b00, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x50a6b06; continue; }
  pc = 0x50a6b11; continue;
 }
 0x50a6b06 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x50a6b06; continue; }
  pc = 0x50a6b11; continue;
 }
 0x50a6b11 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x50a6b1f); cpu.call(0x0); if cpu.returned_to != 0x50a6b1f { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x50a6b2c); cpu.call(0x5f2b3a2); if cpu.returned_to != 0x50a6b2c { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x50a6b31, "Aam");
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
  cpu.unimplemented(0x50a6b54, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x50a6b64); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6b72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6b72); }
 let mut pc: u32 = 0x50a6b72;
 loop { match pc {
 0x50a6b72 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6b74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6b74); }
 let mut pc: u32 = 0x50a6b74;
 loop { match pc {
 0x50a6b74 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x513b08cu32; cpu.eax = v; }
  cpu.push(0x50a6b84); f_050a65b2(cpu); if cpu.returned_to != 0x50a6b84 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6b86(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a6b86); } r_050a6b86(cpu, 0x50a6b86); }
pub fn f_050a6c10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50a6c10); } r_050a6b86(cpu, 0x50a6c10); }
fn r_050a6b86(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50a6b86 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x50a6b8c, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x50a6b91, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x50a6c12; continue; }
  pc = 0x50a6ba1; continue;
 }
 0x50a6ba1 => {
  if cpu.cf { pc = 0x50a6c16; continue; }
  pc = 0x50a6ba3; continue;
 }
 0x50a6ba3 => {
  if cpu.zf { pc = 0x50a6c1a; continue; }
  pc = 0x50a6ba5; continue;
 }
 0x50a6ba5 => {
  if (cpu.cf || cpu.zf) { pc = 0x50a6c1e; continue; }
  pc = 0x50a6ba7; continue;
 }
 0x50a6ba7 => {
  if cpu.sf { pc = 0x50a6c22; continue; }
  pc = 0x50a6ba9; continue;
 }
 0x50a6ba9 => {
  if cpu.pf() { f_050a6bab(cpu); return; }
  f_050a6bab(cpu); return;
 }
 0x50a6c10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x50a6c58; continue; }
  pc = 0x50a6c19; continue;
 }
 0x50a6c12 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a6c16 => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_050a6c8c(cpu); return; }
  pc = 0x50a6c18; continue;
 }
 0x50a6c18 => {
  cpu.unimplemented(0x50a6c18, "Aas");
  pc = 0x50a6c19; continue;
 }
 0x50a6c19 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x50a6c4f; continue; }
  pc = 0x50a6c2b; continue;
 }
 0x50a6c1a => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x50a6c4f; continue; }
  pc = 0x50a6c2b; continue;
 }
 0x50a6c1e => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x50a6c4f; continue; }
  pc = 0x50a6c2b; continue;
 }
 0x50a6c22 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x50a6c4f; continue; }
  pc = 0x50a6c2b; continue;
 }
 0x50a6c2b => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x50a6c4f; continue; }
  pc = 0x50a6c33; continue;
 }
 0x50a6c33 => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x50a6c58; continue; }
  pc = 0x50a6c3b; continue;
 }
 0x50a6c3b => {
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
 0x50a6c4f => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50a6c58 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6bab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6bab); }
 let mut pc: u32 = 0x50a6bab;
 loop { match pc {
 0x50a6bab => {
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
  pc = 0x50a6bc1; continue;
 }
 0x50a6bc1 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x50a6b86u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50a6bc1; continue; }
  pc = 0x50a6be1; continue;
 }
 0x50a6be1 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x50a6be1; continue; }
  pc = 0x50a6beb; continue;
 }
 0x50a6beb => {
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
pub fn f_050a6bf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6bf5); }
 let mut pc: u32 = 0x50a6bf5;
 loop { match pc {
 0x50a6bf5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x50a6c07; continue; }
  pc = 0x50a6bfd; continue;
 }
 0x50a6bfd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x50a6c07; continue; }
  pc = 0x50a6c01; continue;
 }
 0x50a6c01 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x50a6c07; continue;
 }
 0x50a6c07 => {
  cpu.push(0x50a6c0c); f_050a6bab(cpu); if cpu.returned_to != 0x50a6c0c { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c5c); }
 let mut pc: u32 = 0x50a6c5c;
 loop { match pc {
 0x50a6c5c => {
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
pub fn f_050a6c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c80); }
 let mut pc: u32 = 0x50a6c80;
 loop { match pc {
 0x50a6c80 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c84); }
 let mut pc: u32 = 0x50a6c84;
 loop { match pc {
 0x50a6c84 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c88); }
 let mut pc: u32 = 0x50a6c88;
 loop { match pc {
 0x50a6c88 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c8c); }
 let mut pc: u32 = 0x50a6c8c;
 loop { match pc {
 0x50a6c8c => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c90); }
 let mut pc: u32 = 0x50a6c90;
 loop { match pc {
 0x50a6c90 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c94); }
 let mut pc: u32 = 0x50a6c94;
 loop { match pc {
 0x50a6c94 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c98); }
 let mut pc: u32 = 0x50a6c98;
 loop { match pc {
 0x50a6c98 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6c9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6c9c); }
 let mut pc: u32 = 0x50a6c9c;
 loop { match pc {
 0x50a6c9c => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ca0); }
 let mut pc: u32 = 0x50a6ca0;
 loop { match pc {
 0x50a6ca0 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ca4); }
 let mut pc: u32 = 0x50a6ca4;
 loop { match pc {
 0x50a6ca4 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ca8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ca8); }
 let mut pc: u32 = 0x50a6ca8;
 loop { match pc {
 0x50a6ca8 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cac); }
 let mut pc: u32 = 0x50a6cac;
 loop { match pc {
 0x50a6cac => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cb0); }
 let mut pc: u32 = 0x50a6cb0;
 loop { match pc {
 0x50a6cb0 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cb4); }
 let mut pc: u32 = 0x50a6cb4;
 loop { match pc {
 0x50a6cb4 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cb8); }
 let mut pc: u32 = 0x50a6cb8;
 loop { match pc {
 0x50a6cb8 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cbc); }
 let mut pc: u32 = 0x50a6cbc;
 loop { match pc {
 0x50a6cbc => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cc0); }
 let mut pc: u32 = 0x50a6cc0;
 loop { match pc {
 0x50a6cc0 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cc4); }
 let mut pc: u32 = 0x50a6cc4;
 loop { match pc {
 0x50a6cc4 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cc8); }
 let mut pc: u32 = 0x50a6cc8;
 loop { match pc {
 0x50a6cc8 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ccc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ccc); }
 let mut pc: u32 = 0x50a6ccc;
 loop { match pc {
 0x50a6ccc => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cd0); }
 let mut pc: u32 = 0x50a6cd0;
 loop { match pc {
 0x50a6cd0 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cd4); }
 let mut pc: u32 = 0x50a6cd4;
 loop { match pc {
 0x50a6cd4 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cd8); }
 let mut pc: u32 = 0x50a6cd8;
 loop { match pc {
 0x50a6cd8 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cdc); }
 let mut pc: u32 = 0x50a6cdc;
 loop { match pc {
 0x50a6cdc => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ce0); }
 let mut pc: u32 = 0x50a6ce0;
 loop { match pc {
 0x50a6ce0 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ce4); }
 let mut pc: u32 = 0x50a6ce4;
 loop { match pc {
 0x50a6ce4 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ce8); }
 let mut pc: u32 = 0x50a6ce8;
 loop { match pc {
 0x50a6ce8 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cec); }
 let mut pc: u32 = 0x50a6cec;
 loop { match pc {
 0x50a6cec => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cf0); }
 let mut pc: u32 = 0x50a6cf0;
 loop { match pc {
 0x50a6cf0 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cf4); }
 let mut pc: u32 = 0x50a6cf4;
 loop { match pc {
 0x50a6cf4 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cf8); }
 let mut pc: u32 = 0x50a6cf8;
 loop { match pc {
 0x50a6cf8 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6cfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6cfc); }
 let mut pc: u32 = 0x50a6cfc;
 loop { match pc {
 0x50a6cfc => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d00); }
 let mut pc: u32 = 0x50a6d00;
 loop { match pc {
 0x50a6d00 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d04); }
 let mut pc: u32 = 0x50a6d04;
 loop { match pc {
 0x50a6d04 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d08); }
 let mut pc: u32 = 0x50a6d08;
 loop { match pc {
 0x50a6d08 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d0c); }
 let mut pc: u32 = 0x50a6d0c;
 loop { match pc {
 0x50a6d0c => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d10); }
 let mut pc: u32 = 0x50a6d10;
 loop { match pc {
 0x50a6d10 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d14); }
 let mut pc: u32 = 0x50a6d14;
 loop { match pc {
 0x50a6d14 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d18); }
 let mut pc: u32 = 0x50a6d18;
 loop { match pc {
 0x50a6d18 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d1c); }
 let mut pc: u32 = 0x50a6d1c;
 loop { match pc {
 0x50a6d1c => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d20); }
 let mut pc: u32 = 0x50a6d20;
 loop { match pc {
 0x50a6d20 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d24); }
 let mut pc: u32 = 0x50a6d24;
 loop { match pc {
 0x50a6d24 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d28); }
 let mut pc: u32 = 0x50a6d28;
 loop { match pc {
 0x50a6d28 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d2c); }
 let mut pc: u32 = 0x50a6d2c;
 loop { match pc {
 0x50a6d2c => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d30); }
 let mut pc: u32 = 0x50a6d30;
 loop { match pc {
 0x50a6d30 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d34); }
 let mut pc: u32 = 0x50a6d34;
 loop { match pc {
 0x50a6d34 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d38); }
 let mut pc: u32 = 0x50a6d38;
 loop { match pc {
 0x50a6d38 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d3c); }
 let mut pc: u32 = 0x50a6d3c;
 loop { match pc {
 0x50a6d3c => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d40); }
 let mut pc: u32 = 0x50a6d40;
 loop { match pc {
 0x50a6d40 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d44); }
 let mut pc: u32 = 0x50a6d44;
 loop { match pc {
 0x50a6d44 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d48); }
 let mut pc: u32 = 0x50a6d48;
 loop { match pc {
 0x50a6d48 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d4c); }
 let mut pc: u32 = 0x50a6d4c;
 loop { match pc {
 0x50a6d4c => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d50); }
 let mut pc: u32 = 0x50a6d50;
 loop { match pc {
 0x50a6d50 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d54); }
 let mut pc: u32 = 0x50a6d54;
 loop { match pc {
 0x50a6d54 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d58); }
 let mut pc: u32 = 0x50a6d58;
 loop { match pc {
 0x50a6d58 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d5c); }
 let mut pc: u32 = 0x50a6d5c;
 loop { match pc {
 0x50a6d5c => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d60); }
 let mut pc: u32 = 0x50a6d60;
 loop { match pc {
 0x50a6d60 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d64); }
 let mut pc: u32 = 0x50a6d64;
 loop { match pc {
 0x50a6d64 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d68); }
 let mut pc: u32 = 0x50a6d68;
 loop { match pc {
 0x50a6d68 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d6c); }
 let mut pc: u32 = 0x50a6d6c;
 loop { match pc {
 0x50a6d6c => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d70); }
 let mut pc: u32 = 0x50a6d70;
 loop { match pc {
 0x50a6d70 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d74); }
 let mut pc: u32 = 0x50a6d74;
 loop { match pc {
 0x50a6d74 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d78); }
 let mut pc: u32 = 0x50a6d78;
 loop { match pc {
 0x50a6d78 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d7c); }
 let mut pc: u32 = 0x50a6d7c;
 loop { match pc {
 0x50a6d7c => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d80); }
 let mut pc: u32 = 0x50a6d80;
 loop { match pc {
 0x50a6d80 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d84); }
 let mut pc: u32 = 0x50a6d84;
 loop { match pc {
 0x50a6d84 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d88); }
 let mut pc: u32 = 0x50a6d88;
 loop { match pc {
 0x50a6d88 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d8c); }
 let mut pc: u32 = 0x50a6d8c;
 loop { match pc {
 0x50a6d8c => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d90); }
 let mut pc: u32 = 0x50a6d90;
 loop { match pc {
 0x50a6d90 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d94); }
 let mut pc: u32 = 0x50a6d94;
 loop { match pc {
 0x50a6d94 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d98); }
 let mut pc: u32 = 0x50a6d98;
 loop { match pc {
 0x50a6d98 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6d9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6d9c); }
 let mut pc: u32 = 0x50a6d9c;
 loop { match pc {
 0x50a6d9c => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6da0); }
 let mut pc: u32 = 0x50a6da0;
 loop { match pc {
 0x50a6da0 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6da4); }
 let mut pc: u32 = 0x50a6da4;
 loop { match pc {
 0x50a6da4 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6da8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6da8); }
 let mut pc: u32 = 0x50a6da8;
 loop { match pc {
 0x50a6da8 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dac); }
 let mut pc: u32 = 0x50a6dac;
 loop { match pc {
 0x50a6dac => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6db0); }
 let mut pc: u32 = 0x50a6db0;
 loop { match pc {
 0x50a6db0 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6db4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6db4); }
 let mut pc: u32 = 0x50a6db4;
 loop { match pc {
 0x50a6db4 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6db8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6db8); }
 let mut pc: u32 = 0x50a6db8;
 loop { match pc {
 0x50a6db8 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dbc); }
 let mut pc: u32 = 0x50a6dbc;
 loop { match pc {
 0x50a6dbc => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dc0); }
 let mut pc: u32 = 0x50a6dc0;
 loop { match pc {
 0x50a6dc0 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dc4); }
 let mut pc: u32 = 0x50a6dc4;
 loop { match pc {
 0x50a6dc4 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dc8); }
 let mut pc: u32 = 0x50a6dc8;
 loop { match pc {
 0x50a6dc8 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dcc); }
 let mut pc: u32 = 0x50a6dcc;
 loop { match pc {
 0x50a6dcc => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dd0); }
 let mut pc: u32 = 0x50a6dd0;
 loop { match pc {
 0x50a6dd0 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dd4); }
 let mut pc: u32 = 0x50a6dd4;
 loop { match pc {
 0x50a6dd4 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dd8); }
 let mut pc: u32 = 0x50a6dd8;
 loop { match pc {
 0x50a6dd8 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ddc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ddc); }
 let mut pc: u32 = 0x50a6ddc;
 loop { match pc {
 0x50a6ddc => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6de0); }
 let mut pc: u32 = 0x50a6de0;
 loop { match pc {
 0x50a6de0 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6de4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6de4); }
 let mut pc: u32 = 0x50a6de4;
 loop { match pc {
 0x50a6de4 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6de8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6de8); }
 let mut pc: u32 = 0x50a6de8;
 loop { match pc {
 0x50a6de8 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dec); }
 let mut pc: u32 = 0x50a6dec;
 loop { match pc {
 0x50a6dec => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6df0); }
 let mut pc: u32 = 0x50a6df0;
 loop { match pc {
 0x50a6df0 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6df4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6df4); }
 let mut pc: u32 = 0x50a6df4;
 loop { match pc {
 0x50a6df4 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6df8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6df8); }
 let mut pc: u32 = 0x50a6df8;
 loop { match pc {
 0x50a6df8 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6dfc); }
 let mut pc: u32 = 0x50a6dfc;
 loop { match pc {
 0x50a6dfc => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e00); }
 let mut pc: u32 = 0x50a6e00;
 loop { match pc {
 0x50a6e00 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e04); }
 let mut pc: u32 = 0x50a6e04;
 loop { match pc {
 0x50a6e04 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e08); }
 let mut pc: u32 = 0x50a6e08;
 loop { match pc {
 0x50a6e08 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e0c); }
 let mut pc: u32 = 0x50a6e0c;
 loop { match pc {
 0x50a6e0c => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e10); }
 let mut pc: u32 = 0x50a6e10;
 loop { match pc {
 0x50a6e10 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e14); }
 let mut pc: u32 = 0x50a6e14;
 loop { match pc {
 0x50a6e14 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e18); }
 let mut pc: u32 = 0x50a6e18;
 loop { match pc {
 0x50a6e18 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e1c); }
 let mut pc: u32 = 0x50a6e1c;
 loop { match pc {
 0x50a6e1c => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e20); }
 let mut pc: u32 = 0x50a6e20;
 loop { match pc {
 0x50a6e20 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e24); }
 let mut pc: u32 = 0x50a6e24;
 loop { match pc {
 0x50a6e24 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e28); }
 let mut pc: u32 = 0x50a6e28;
 loop { match pc {
 0x50a6e28 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e2c); }
 let mut pc: u32 = 0x50a6e2c;
 loop { match pc {
 0x50a6e2c => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e30); }
 let mut pc: u32 = 0x50a6e30;
 loop { match pc {
 0x50a6e30 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e34); }
 let mut pc: u32 = 0x50a6e34;
 loop { match pc {
 0x50a6e34 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e38); }
 let mut pc: u32 = 0x50a6e38;
 loop { match pc {
 0x50a6e38 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e3c); }
 let mut pc: u32 = 0x50a6e3c;
 loop { match pc {
 0x50a6e3c => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e40); }
 let mut pc: u32 = 0x50a6e40;
 loop { match pc {
 0x50a6e40 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e44); }
 let mut pc: u32 = 0x50a6e44;
 loop { match pc {
 0x50a6e44 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e48); }
 let mut pc: u32 = 0x50a6e48;
 loop { match pc {
 0x50a6e48 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e4c); }
 let mut pc: u32 = 0x50a6e4c;
 loop { match pc {
 0x50a6e4c => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e50); }
 let mut pc: u32 = 0x50a6e50;
 loop { match pc {
 0x50a6e50 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e54); }
 let mut pc: u32 = 0x50a6e54;
 loop { match pc {
 0x50a6e54 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e58); }
 let mut pc: u32 = 0x50a6e58;
 loop { match pc {
 0x50a6e58 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e5c); }
 let mut pc: u32 = 0x50a6e5c;
 loop { match pc {
 0x50a6e5c => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e60); }
 let mut pc: u32 = 0x50a6e60;
 loop { match pc {
 0x50a6e60 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e64); }
 let mut pc: u32 = 0x50a6e64;
 loop { match pc {
 0x50a6e64 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e68); }
 let mut pc: u32 = 0x50a6e68;
 loop { match pc {
 0x50a6e68 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e6c); }
 let mut pc: u32 = 0x50a6e6c;
 loop { match pc {
 0x50a6e6c => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e70); }
 let mut pc: u32 = 0x50a6e70;
 loop { match pc {
 0x50a6e70 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e74); }
 let mut pc: u32 = 0x50a6e74;
 loop { match pc {
 0x50a6e74 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e78); }
 let mut pc: u32 = 0x50a6e78;
 loop { match pc {
 0x50a6e78 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e7c); }
 let mut pc: u32 = 0x50a6e7c;
 loop { match pc {
 0x50a6e7c => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e80); }
 let mut pc: u32 = 0x50a6e80;
 loop { match pc {
 0x50a6e80 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e84); }
 let mut pc: u32 = 0x50a6e84;
 loop { match pc {
 0x50a6e84 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e88); }
 let mut pc: u32 = 0x50a6e88;
 loop { match pc {
 0x50a6e88 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e8c); }
 let mut pc: u32 = 0x50a6e8c;
 loop { match pc {
 0x50a6e8c => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e90); }
 let mut pc: u32 = 0x50a6e90;
 loop { match pc {
 0x50a6e90 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e94); }
 let mut pc: u32 = 0x50a6e94;
 loop { match pc {
 0x50a6e94 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e98); }
 let mut pc: u32 = 0x50a6e98;
 loop { match pc {
 0x50a6e98 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6e9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6e9c); }
 let mut pc: u32 = 0x50a6e9c;
 loop { match pc {
 0x50a6e9c => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ea0); }
 let mut pc: u32 = 0x50a6ea0;
 loop { match pc {
 0x50a6ea0 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ea4); }
 let mut pc: u32 = 0x50a6ea4;
 loop { match pc {
 0x50a6ea4 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ea8); }
 let mut pc: u32 = 0x50a6ea8;
 loop { match pc {
 0x50a6ea8 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6eac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6eac); }
 let mut pc: u32 = 0x50a6eac;
 loop { match pc {
 0x50a6eac => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6eb0); }
 let mut pc: u32 = 0x50a6eb0;
 loop { match pc {
 0x50a6eb0 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6eb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6eb4); }
 let mut pc: u32 = 0x50a6eb4;
 loop { match pc {
 0x50a6eb4 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6eb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6eb8); }
 let mut pc: u32 = 0x50a6eb8;
 loop { match pc {
 0x50a6eb8 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ebc); }
 let mut pc: u32 = 0x50a6ebc;
 loop { match pc {
 0x50a6ebc => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ec0); }
 let mut pc: u32 = 0x50a6ec0;
 loop { match pc {
 0x50a6ec0 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ec4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ec4); }
 let mut pc: u32 = 0x50a6ec4;
 loop { match pc {
 0x50a6ec4 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ec8); }
 let mut pc: u32 = 0x50a6ec8;
 loop { match pc {
 0x50a6ec8 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ecc); }
 let mut pc: u32 = 0x50a6ecc;
 loop { match pc {
 0x50a6ecc => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ed0); }
 let mut pc: u32 = 0x50a6ed0;
 loop { match pc {
 0x50a6ed0 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ed4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ed4); }
 let mut pc: u32 = 0x50a6ed4;
 loop { match pc {
 0x50a6ed4 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050a6ed8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50a6ed8); }
 let mut pc: u32 = 0x50a6ed8;
 loop { match pc {
 0x50a6ed8 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
