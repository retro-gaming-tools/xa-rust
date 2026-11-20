#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_040ad8c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8c6); }
 let mut pc: u32 = 0x40ad8c6;
 loop { match pc {
 0x40ad8c6 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8c9); }
 let mut pc: u32 = 0x40ad8c9;
 loop { match pc {
 0x40ad8c9 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8cc); }
 let mut pc: u32 = 0x40ad8cc;
 loop { match pc {
 0x40ad8cc => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8cf); }
 let mut pc: u32 = 0x40ad8cf;
 loop { match pc {
 0x40ad8cf => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8d2); }
 let mut pc: u32 = 0x40ad8d2;
 loop { match pc {
 0x40ad8d2 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8d5); }
 let mut pc: u32 = 0x40ad8d5;
 loop { match pc {
 0x40ad8d5 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8d8); }
 let mut pc: u32 = 0x40ad8d8;
 loop { match pc {
 0x40ad8d8 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8db); }
 let mut pc: u32 = 0x40ad8db;
 loop { match pc {
 0x40ad8db => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8de); }
 let mut pc: u32 = 0x40ad8de;
 loop { match pc {
 0x40ad8de => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8e1); }
 let mut pc: u32 = 0x40ad8e1;
 loop { match pc {
 0x40ad8e1 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8e4); }
 let mut pc: u32 = 0x40ad8e4;
 loop { match pc {
 0x40ad8e4 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8e7); }
 let mut pc: u32 = 0x40ad8e7;
 loop { match pc {
 0x40ad8e7 => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ea); }
 let mut pc: u32 = 0x40ad8ea;
 loop { match pc {
 0x40ad8ea => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ed); }
 let mut pc: u32 = 0x40ad8ed;
 loop { match pc {
 0x40ad8ed => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8f0); }
 let mut pc: u32 = 0x40ad8f0;
 loop { match pc {
 0x40ad8f0 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8f3); }
 let mut pc: u32 = 0x40ad8f3;
 loop { match pc {
 0x40ad8f3 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8f6); }
 let mut pc: u32 = 0x40ad8f6;
 loop { match pc {
 0x40ad8f6 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8f9); }
 let mut pc: u32 = 0x40ad8f9;
 loop { match pc {
 0x40ad8f9 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8fc); }
 let mut pc: u32 = 0x40ad8fc;
 loop { match pc {
 0x40ad8fc => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ff); }
 let mut pc: u32 = 0x40ad8ff;
 loop { match pc {
 0x40ad8ff => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad902(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad902); }
 let mut pc: u32 = 0x40ad902;
 loop { match pc {
 0x40ad902 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad905(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad905); }
 let mut pc: u32 = 0x40ad905;
 loop { match pc {
 0x40ad905 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad908(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad908); }
 let mut pc: u32 = 0x40ad908;
 loop { match pc {
 0x40ad908 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad90b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad90b); }
 let mut pc: u32 = 0x40ad90b;
 loop { match pc {
 0x40ad90b => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad90e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad90e); }
 let mut pc: u32 = 0x40ad90e;
 loop { match pc {
 0x40ad90e => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad911(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad911); }
 let mut pc: u32 = 0x40ad911;
 loop { match pc {
 0x40ad911 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad914(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad914); }
 let mut pc: u32 = 0x40ad914;
 loop { match pc {
 0x40ad914 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad917(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad917); }
 let mut pc: u32 = 0x40ad917;
 loop { match pc {
 0x40ad917 => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad91a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad91a); }
 let mut pc: u32 = 0x40ad91a;
 loop { match pc {
 0x40ad91a => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad91d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad91d); }
 let mut pc: u32 = 0x40ad91d;
 loop { match pc {
 0x40ad91d => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad920); }
 let mut pc: u32 = 0x40ad920;
 loop { match pc {
 0x40ad920 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad923(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad923); }
 let mut pc: u32 = 0x40ad923;
 loop { match pc {
 0x40ad923 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad926(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad926); }
 let mut pc: u32 = 0x40ad926;
 loop { match pc {
 0x40ad926 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad929(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad929); }
 let mut pc: u32 = 0x40ad929;
 loop { match pc {
 0x40ad929 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad92c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad92c); }
 let mut pc: u32 = 0x40ad92c;
 loop { match pc {
 0x40ad92c => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad92f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad92f); }
 let mut pc: u32 = 0x40ad92f;
 loop { match pc {
 0x40ad92f => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad932(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad932); }
 let mut pc: u32 = 0x40ad932;
 loop { match pc {
 0x40ad932 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad935(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad935); }
 let mut pc: u32 = 0x40ad935;
 loop { match pc {
 0x40ad935 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad938(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad938); }
 let mut pc: u32 = 0x40ad938;
 loop { match pc {
 0x40ad938 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad93b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad93b); }
 let mut pc: u32 = 0x40ad93b;
 loop { match pc {
 0x40ad93b => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad93e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad93e); }
 let mut pc: u32 = 0x40ad93e;
 loop { match pc {
 0x40ad93e => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad941(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad941); }
 let mut pc: u32 = 0x40ad941;
 loop { match pc {
 0x40ad941 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad944(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad944); }
 let mut pc: u32 = 0x40ad944;
 loop { match pc {
 0x40ad944 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad947); }
 let mut pc: u32 = 0x40ad947;
 loop { match pc {
 0x40ad947 => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad94a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad94a); }
 let mut pc: u32 = 0x40ad94a;
 loop { match pc {
 0x40ad94a => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad94d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad94d); }
 let mut pc: u32 = 0x40ad94d;
 loop { match pc {
 0x40ad94d => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad950); }
 let mut pc: u32 = 0x40ad950;
 loop { match pc {
 0x40ad950 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad953(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad953); }
 let mut pc: u32 = 0x40ad953;
 loop { match pc {
 0x40ad953 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad956(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad956); }
 let mut pc: u32 = 0x40ad956;
 loop { match pc {
 0x40ad956 => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad959(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad959); }
 let mut pc: u32 = 0x40ad959;
 loop { match pc {
 0x40ad959 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad95c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad95c); }
 let mut pc: u32 = 0x40ad95c;
 loop { match pc {
 0x40ad95c => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad95f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad95f); }
 let mut pc: u32 = 0x40ad95f;
 loop { match pc {
 0x40ad95f => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad962(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad962); }
 let mut pc: u32 = 0x40ad962;
 loop { match pc {
 0x40ad962 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad965(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad965); }
 let mut pc: u32 = 0x40ad965;
 loop { match pc {
 0x40ad965 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad968); }
 let mut pc: u32 = 0x40ad968;
 loop { match pc {
 0x40ad968 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad96b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad96b); }
 let mut pc: u32 = 0x40ad96b;
 loop { match pc {
 0x40ad96b => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad96e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad96e); }
 let mut pc: u32 = 0x40ad96e;
 loop { match pc {
 0x40ad96e => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad971(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad971); }
 let mut pc: u32 = 0x40ad971;
 loop { match pc {
 0x40ad971 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad974(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad974); }
 let mut pc: u32 = 0x40ad974;
 loop { match pc {
 0x40ad974 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad977(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad977); }
 let mut pc: u32 = 0x40ad977;
 loop { match pc {
 0x40ad977 => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad97a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad97a); }
 let mut pc: u32 = 0x40ad97a;
 loop { match pc {
 0x40ad97a => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad97d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad97d); }
 let mut pc: u32 = 0x40ad97d;
 loop { match pc {
 0x40ad97d => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad980); }
 let mut pc: u32 = 0x40ad980;
 loop { match pc {
 0x40ad980 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad983(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad983); }
 let mut pc: u32 = 0x40ad983;
 loop { match pc {
 0x40ad983 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad986(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad986); }
 let mut pc: u32 = 0x40ad986;
 loop { match pc {
 0x40ad986 => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad989); }
 let mut pc: u32 = 0x40ad989;
 loop { match pc {
 0x40ad989 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad98c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad98c); }
 let mut pc: u32 = 0x40ad98c;
 loop { match pc {
 0x40ad98c => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad98f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad98f); }
 let mut pc: u32 = 0x40ad98f;
 loop { match pc {
 0x40ad98f => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad992(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad992); }
 let mut pc: u32 = 0x40ad992;
 loop { match pc {
 0x40ad992 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad995(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad995); }
 let mut pc: u32 = 0x40ad995;
 loop { match pc {
 0x40ad995 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad998(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad998); }
 let mut pc: u32 = 0x40ad998;
 loop { match pc {
 0x40ad998 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad99b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad99b); }
 let mut pc: u32 = 0x40ad99b;
 loop { match pc {
 0x40ad99b => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad99e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad99e); }
 let mut pc: u32 = 0x40ad99e;
 loop { match pc {
 0x40ad99e => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9a1); }
 let mut pc: u32 = 0x40ad9a1;
 loop { match pc {
 0x40ad9a1 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9a4); }
 let mut pc: u32 = 0x40ad9a4;
 loop { match pc {
 0x40ad9a4 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9a7); }
 let mut pc: u32 = 0x40ad9a7;
 loop { match pc {
 0x40ad9a7 => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9aa); }
 let mut pc: u32 = 0x40ad9aa;
 loop { match pc {
 0x40ad9aa => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9ad); }
 let mut pc: u32 = 0x40ad9ad;
 loop { match pc {
 0x40ad9ad => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9b0); }
 let mut pc: u32 = 0x40ad9b0;
 loop { match pc {
 0x40ad9b0 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9b3); }
 let mut pc: u32 = 0x40ad9b3;
 loop { match pc {
 0x40ad9b3 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9b6); }
 let mut pc: u32 = 0x40ad9b6;
 loop { match pc {
 0x40ad9b6 => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9b9); }
 let mut pc: u32 = 0x40ad9b9;
 loop { match pc {
 0x40ad9b9 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9bc); }
 let mut pc: u32 = 0x40ad9bc;
 loop { match pc {
 0x40ad9bc => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9bf); }
 let mut pc: u32 = 0x40ad9bf;
 loop { match pc {
 0x40ad9bf => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9c2); }
 let mut pc: u32 = 0x40ad9c2;
 loop { match pc {
 0x40ad9c2 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9c5); }
 let mut pc: u32 = 0x40ad9c5;
 loop { match pc {
 0x40ad9c5 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9c8); }
 let mut pc: u32 = 0x40ad9c8;
 loop { match pc {
 0x40ad9c8 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9cb); }
 let mut pc: u32 = 0x40ad9cb;
 loop { match pc {
 0x40ad9cb => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9ce); }
 let mut pc: u32 = 0x40ad9ce;
 loop { match pc {
 0x40ad9ce => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9d1); }
 let mut pc: u32 = 0x40ad9d1;
 loop { match pc {
 0x40ad9d1 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9d4); }
 let mut pc: u32 = 0x40ad9d4;
 loop { match pc {
 0x40ad9d4 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9d7); }
 let mut pc: u32 = 0x40ad9d7;
 loop { match pc {
 0x40ad9d7 => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9da); }
 let mut pc: u32 = 0x40ad9da;
 loop { match pc {
 0x40ad9da => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9dd); }
 let mut pc: u32 = 0x40ad9dd;
 loop { match pc {
 0x40ad9dd => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9e0); }
 let mut pc: u32 = 0x40ad9e0;
 loop { match pc {
 0x40ad9e0 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9e3); }
 let mut pc: u32 = 0x40ad9e3;
 loop { match pc {
 0x40ad9e3 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9e6); }
 let mut pc: u32 = 0x40ad9e6;
 loop { match pc {
 0x40ad9e6 => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9e9); }
 let mut pc: u32 = 0x40ad9e9;
 loop { match pc {
 0x40ad9e9 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9ec); }
 let mut pc: u32 = 0x40ad9ec;
 loop { match pc {
 0x40ad9ec => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9ef); }
 let mut pc: u32 = 0x40ad9ef;
 loop { match pc {
 0x40ad9ef => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9f2); }
 let mut pc: u32 = 0x40ad9f2;
 loop { match pc {
 0x40ad9f2 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9f5); }
 let mut pc: u32 = 0x40ad9f5;
 loop { match pc {
 0x40ad9f5 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9f8); }
 let mut pc: u32 = 0x40ad9f8;
 loop { match pc {
 0x40ad9f8 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9fb); }
 let mut pc: u32 = 0x40ad9fb;
 loop { match pc {
 0x40ad9fb => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad9fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad9fe); }
 let mut pc: u32 = 0x40ad9fe;
 loop { match pc {
 0x40ad9fe => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada01); }
 let mut pc: u32 = 0x40ada01;
 loop { match pc {
 0x40ada01 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada04); }
 let mut pc: u32 = 0x40ada04;
 loop { match pc {
 0x40ada04 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada07); }
 let mut pc: u32 = 0x40ada07;
 loop { match pc {
 0x40ada07 => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada0a); }
 let mut pc: u32 = 0x40ada0a;
 loop { match pc {
 0x40ada0a => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada0d); }
 let mut pc: u32 = 0x40ada0d;
 loop { match pc {
 0x40ada0d => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada10); }
 let mut pc: u32 = 0x40ada10;
 loop { match pc {
 0x40ada10 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada13); }
 let mut pc: u32 = 0x40ada13;
 loop { match pc {
 0x40ada13 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada16); }
 let mut pc: u32 = 0x40ada16;
 loop { match pc {
 0x40ada16 => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada19); }
 let mut pc: u32 = 0x40ada19;
 loop { match pc {
 0x40ada19 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada1c); }
 let mut pc: u32 = 0x40ada1c;
 loop { match pc {
 0x40ada1c => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada1f); }
 let mut pc: u32 = 0x40ada1f;
 loop { match pc {
 0x40ada1f => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada22); }
 let mut pc: u32 = 0x40ada22;
 loop { match pc {
 0x40ada22 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada25); }
 let mut pc: u32 = 0x40ada25;
 loop { match pc {
 0x40ada25 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada28); }
 let mut pc: u32 = 0x40ada28;
 loop { match pc {
 0x40ada28 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada2b); }
 let mut pc: u32 = 0x40ada2b;
 loop { match pc {
 0x40ada2b => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada2e); }
 let mut pc: u32 = 0x40ada2e;
 loop { match pc {
 0x40ada2e => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada31); }
 let mut pc: u32 = 0x40ada31;
 loop { match pc {
 0x40ada31 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada34); }
 let mut pc: u32 = 0x40ada34;
 loop { match pc {
 0x40ada34 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada37); }
 let mut pc: u32 = 0x40ada37;
 loop { match pc {
 0x40ada37 => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada3a); }
 let mut pc: u32 = 0x40ada3a;
 loop { match pc {
 0x40ada3a => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada3d); }
 let mut pc: u32 = 0x40ada3d;
 loop { match pc {
 0x40ada3d => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada40); }
 let mut pc: u32 = 0x40ada40;
 loop { match pc {
 0x40ada40 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada43); }
 let mut pc: u32 = 0x40ada43;
 loop { match pc {
 0x40ada43 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada46); }
 let mut pc: u32 = 0x40ada46;
 loop { match pc {
 0x40ada46 => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada49); }
 let mut pc: u32 = 0x40ada49;
 loop { match pc {
 0x40ada49 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada4c); }
 let mut pc: u32 = 0x40ada4c;
 loop { match pc {
 0x40ada4c => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada4f); }
 let mut pc: u32 = 0x40ada4f;
 loop { match pc {
 0x40ada4f => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada52); }
 let mut pc: u32 = 0x40ada52;
 loop { match pc {
 0x40ada52 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada55); }
 let mut pc: u32 = 0x40ada55;
 loop { match pc {
 0x40ada55 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada58); }
 let mut pc: u32 = 0x40ada58;
 loop { match pc {
 0x40ada58 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada5b); }
 let mut pc: u32 = 0x40ada5b;
 loop { match pc {
 0x40ada5b => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada5e); }
 let mut pc: u32 = 0x40ada5e;
 loop { match pc {
 0x40ada5e => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada61); }
 let mut pc: u32 = 0x40ada61;
 loop { match pc {
 0x40ada61 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada64); }
 let mut pc: u32 = 0x40ada64;
 loop { match pc {
 0x40ada64 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada67); }
 let mut pc: u32 = 0x40ada67;
 loop { match pc {
 0x40ada67 => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada6a); }
 let mut pc: u32 = 0x40ada6a;
 loop { match pc {
 0x40ada6a => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada6d); }
 let mut pc: u32 = 0x40ada6d;
 loop { match pc {
 0x40ada6d => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada70); }
 let mut pc: u32 = 0x40ada70;
 loop { match pc {
 0x40ada70 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada73); }
 let mut pc: u32 = 0x40ada73;
 loop { match pc {
 0x40ada73 => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada76); }
 let mut pc: u32 = 0x40ada76;
 loop { match pc {
 0x40ada76 => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada79); }
 let mut pc: u32 = 0x40ada79;
 loop { match pc {
 0x40ada79 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada7c); }
 let mut pc: u32 = 0x40ada7c;
 loop { match pc {
 0x40ada7c => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada7f); }
 let mut pc: u32 = 0x40ada7f;
 loop { match pc {
 0x40ada7f => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada82); }
 let mut pc: u32 = 0x40ada82;
 loop { match pc {
 0x40ada82 => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada85); }
 let mut pc: u32 = 0x40ada85;
 loop { match pc {
 0x40ada85 => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada88); }
 let mut pc: u32 = 0x40ada88;
 loop { match pc {
 0x40ada88 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada8b); }
 let mut pc: u32 = 0x40ada8b;
 loop { match pc {
 0x40ada8b => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada8e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada8e); }
 let mut pc: u32 = 0x40ada8e;
 loop { match pc {
 0x40ada8e => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada91); }
 let mut pc: u32 = 0x40ada91;
 loop { match pc {
 0x40ada91 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada94); }
 let mut pc: u32 = 0x40ada94;
 loop { match pc {
 0x40ada94 => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada97(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada97); }
 let mut pc: u32 = 0x40ada97;
 loop { match pc {
 0x40ada97 => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada9a); }
 let mut pc: u32 = 0x40ada9a;
 loop { match pc {
 0x40ada9a => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ada9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ada9d); }
 let mut pc: u32 = 0x40ada9d;
 loop { match pc {
 0x40ada9d => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaa0); }
 let mut pc: u32 = 0x40adaa0;
 loop { match pc {
 0x40adaa0 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaa3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaa3); }
 let mut pc: u32 = 0x40adaa3;
 loop { match pc {
 0x40adaa3 => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaa6); }
 let mut pc: u32 = 0x40adaa6;
 loop { match pc {
 0x40adaa6 => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaa9); }
 let mut pc: u32 = 0x40adaa9;
 loop { match pc {
 0x40adaa9 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaac); }
 let mut pc: u32 = 0x40adaac;
 loop { match pc {
 0x40adaac => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaaf); }
 let mut pc: u32 = 0x40adaaf;
 loop { match pc {
 0x40adaaf => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adab2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adab2); }
 let mut pc: u32 = 0x40adab2;
 loop { match pc {
 0x40adab2 => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adab5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adab5); }
 let mut pc: u32 = 0x40adab5;
 loop { match pc {
 0x40adab5 => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adab8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adab8); }
 let mut pc: u32 = 0x40adab8;
 loop { match pc {
 0x40adab8 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adabb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adabb); }
 let mut pc: u32 = 0x40adabb;
 loop { match pc {
 0x40adabb => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adabe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adabe); }
 let mut pc: u32 = 0x40adabe;
 loop { match pc {
 0x40adabe => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adac1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adac1); }
 let mut pc: u32 = 0x40adac1;
 loop { match pc {
 0x40adac1 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adac4); }
 let mut pc: u32 = 0x40adac4;
 loop { match pc {
 0x40adac4 => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adac7); }
 let mut pc: u32 = 0x40adac7;
 loop { match pc {
 0x40adac7 => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaca); }
 let mut pc: u32 = 0x40adaca;
 loop { match pc {
 0x40adaca => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adacd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adacd); }
 let mut pc: u32 = 0x40adacd;
 loop { match pc {
 0x40adacd => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adad0); }
 let mut pc: u32 = 0x40adad0;
 loop { match pc {
 0x40adad0 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adad3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adad3); }
 let mut pc: u32 = 0x40adad3;
 loop { match pc {
 0x40adad3 => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adad6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adad6); }
 let mut pc: u32 = 0x40adad6;
 loop { match pc {
 0x40adad6 => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adad9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adad9); }
 let mut pc: u32 = 0x40adad9;
 loop { match pc {
 0x40adad9 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adadc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adadc); }
 let mut pc: u32 = 0x40adadc;
 loop { match pc {
 0x40adadc => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adadf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adadf); }
 let mut pc: u32 = 0x40adadf;
 loop { match pc {
 0x40adadf => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adae2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adae2); }
 let mut pc: u32 = 0x40adae2;
 loop { match pc {
 0x40adae2 => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adae5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adae5); }
 let mut pc: u32 = 0x40adae5;
 loop { match pc {
 0x40adae5 => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adae8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adae8); }
 let mut pc: u32 = 0x40adae8;
 loop { match pc {
 0x40adae8 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaeb); }
 let mut pc: u32 = 0x40adaeb;
 loop { match pc {
 0x40adaeb => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaee); }
 let mut pc: u32 = 0x40adaee;
 loop { match pc {
 0x40adaee => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaf1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaf1); }
 let mut pc: u32 = 0x40adaf1;
 loop { match pc {
 0x40adaf1 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaf4); }
 let mut pc: u32 = 0x40adaf4;
 loop { match pc {
 0x40adaf4 => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adaf7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adaf7); }
 let mut pc: u32 = 0x40adaf7;
 loop { match pc {
 0x40adaf7 => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adafa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adafa); }
 let mut pc: u32 = 0x40adafa;
 loop { match pc {
 0x40adafa => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adafd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adafd); }
 let mut pc: u32 = 0x40adafd;
 loop { match pc {
 0x40adafd => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb00); }
 let mut pc: u32 = 0x40adb00;
 loop { match pc {
 0x40adb00 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb03); }
 let mut pc: u32 = 0x40adb03;
 loop { match pc {
 0x40adb03 => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb06); }
 let mut pc: u32 = 0x40adb06;
 loop { match pc {
 0x40adb06 => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb09); }
 let mut pc: u32 = 0x40adb09;
 loop { match pc {
 0x40adb09 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb0c); }
 let mut pc: u32 = 0x40adb0c;
 loop { match pc {
 0x40adb0c => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb0f); }
 let mut pc: u32 = 0x40adb0f;
 loop { match pc {
 0x40adb0f => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb12); }
 let mut pc: u32 = 0x40adb12;
 loop { match pc {
 0x40adb12 => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb15); }
 let mut pc: u32 = 0x40adb15;
 loop { match pc {
 0x40adb15 => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb18); }
 let mut pc: u32 = 0x40adb18;
 loop { match pc {
 0x40adb18 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb1b); }
 let mut pc: u32 = 0x40adb1b;
 loop { match pc {
 0x40adb1b => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb1e); }
 let mut pc: u32 = 0x40adb1e;
 loop { match pc {
 0x40adb1e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40adb25); f_040adc2d(cpu); if cpu.returned_to != 0x40adb25 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x40adb30; continue;
 }
 0x40adb30 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x40adb39; continue; }
  pc = 0x40adb35; continue;
 }
 0x40adb35 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x40adb39; continue;
 }
 0x40adb39 => {
  if cpu.cf { pc = 0x40adb41; continue; }
  pc = 0x40adb3b; continue;
 }
 0x40adb3b => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40adb30; continue;
 }
 0x40adb41 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x40adb44; continue;
 }
 0x40adb44 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x40adb4d; continue; }
  pc = 0x40adb49; continue;
 }
 0x40adb49 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x40adb4d; continue;
 }
 0x40adb4d => {
  if !cpu.cf { pc = 0x40adb55; continue; }
  pc = 0x40adb4f; continue;
 }
 0x40adb4f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40adb44; continue;
 }
 0x40adb55 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adb58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adb58); }
 let mut pc: u32 = 0x40adb58;
 loop { match pc {
 0x40adb58 => {
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
  if cpu.zf { pc = 0x40adbb6; continue; }
  pc = 0x40adb81; continue;
 }
 0x40adb81 => {
  if (cpu.cf || cpu.zf) { pc = 0x40adb8c; continue; }
  pc = 0x40adb83; continue;
 }
 0x40adb83 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40adb83; continue; }
  pc = 0x40adb8a; continue;
 }
 0x40adb8a => {
  pc = 0x40adbb6; continue;
 }
 0x40adb8c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x40adb90; continue;
 }
 0x40adb90 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40adb90; continue; }
  pc = 0x40adb9b; continue;
 }
 0x40adb9b => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x40adbb6; continue; }
  pc = 0x40adba2; continue;
 }
 0x40adba2 => {
  if !cpu.zf { pc = 0x40adbb0; continue; }
  pc = 0x40adba4; continue;
 }
 0x40adba4 => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x40adbb0; continue; }
  pc = 0x40adba8; continue;
 }
 0x40adba8 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x40adbb6; continue; }
  pc = 0x40adbb0; continue;
 }
 0x40adbb0 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x40adbb6; continue;
 }
 0x40adbb6 => {
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
pub fn f_040adbc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adbc3); }
 let mut pc: u32 = 0x40adbc3;
 loop { match pc {
 0x40adbc3 => {
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
  pc = 0x40adbd8; continue;
 }
 0x40adbd8 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x40adbeb; continue; }
  pc = 0x40adbe4; continue;
 }
 0x40adbe4 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x40adbed; continue; }
  pc = 0x40adbe8; continue;
 }
 0x40adbe8 => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x40adc03; continue;
 }
 0x40adbeb => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x40adbed; continue;
 }
 0x40adbed => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x40adbfc, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x40adc01, "Aam");
  pc = 0x40adc03; continue;
 }
 0x40adc03 => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40adc27; continue; }
  pc = 0x40adc15; continue;
 }
 0x40adc15 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40adc27; continue; }
  pc = 0x40adc1b; continue;
 }
 0x40adc1b => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40adc27; continue; }
  pc = 0x40adc21; continue;
 }
 0x40adc21 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40adbd8; continue; }
  pc = 0x40adc27; continue;
 }
 0x40adc27 => {
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
pub fn f_040adc2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adc2d); }
 let mut pc: u32 = 0x40adc2d;
 loop { match pc {
 0x40adc2d => {
  cpu.push(0x40adc32); f_040adce2(cpu); if cpu.returned_to != 0x40adc32 { return; }
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
  cpu.unimplemented(0x40adc5f, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x40adc70, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x40adc76; continue; }
  pc = 0x40adc81; continue;
 }
 0x40adc76 => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x40adc76; continue; }
  pc = 0x40adc81; continue;
 }
 0x40adc81 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x40adc8f); cpu.call(0x0); if cpu.returned_to != 0x40adc8f { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40adc9c); cpu.call(0x4f32512); if cpu.returned_to != 0x40adc9c { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x40adca1, "Aam");
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
  cpu.unimplemented(0x40adcc4, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x40adcd4); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adce2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adce2); }
 let mut pc: u32 = 0x40adce2;
 loop { match pc {
 0x40adce2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adce4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adce4); }
 let mut pc: u32 = 0x40adce4;
 loop { match pc {
 0x40adce4 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x414b648u32; cpu.eax = v; }
  cpu.push(0x40adcf4); f_040ad382(cpu); if cpu.returned_to != 0x40adcf4 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adcf6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40adcf6); } r_040adcf6(cpu, 0x40adcf6); }
pub fn f_040add80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40add80); } r_040adcf6(cpu, 0x40add80); }
fn r_040adcf6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40adcf6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x40adcfc, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x40add01, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x40add82; continue; }
  pc = 0x40add11; continue;
 }
 0x40add11 => {
  if cpu.cf { pc = 0x40add86; continue; }
  pc = 0x40add13; continue;
 }
 0x40add13 => {
  if cpu.zf { pc = 0x40add8a; continue; }
  pc = 0x40add15; continue;
 }
 0x40add15 => {
  if (cpu.cf || cpu.zf) { pc = 0x40add8e; continue; }
  pc = 0x40add17; continue;
 }
 0x40add17 => {
  if cpu.sf { pc = 0x40add92; continue; }
  pc = 0x40add19; continue;
 }
 0x40add19 => {
  if cpu.pf() { f_040add1b(cpu); return; }
  f_040add1b(cpu); return;
 }
 0x40add80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x40addc8; continue; }
  pc = 0x40add89; continue;
 }
 0x40add82 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40add86 => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_040addfc(cpu); return; }
  pc = 0x40add88; continue;
 }
 0x40add88 => {
  cpu.unimplemented(0x40add88, "Aas");
  pc = 0x40add89; continue;
 }
 0x40add89 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x40addbf; continue; }
  pc = 0x40add9b; continue;
 }
 0x40add8a => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x40addbf; continue; }
  pc = 0x40add9b; continue;
 }
 0x40add8e => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x40addbf; continue; }
  pc = 0x40add9b; continue;
 }
 0x40add92 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x40addbf; continue; }
  pc = 0x40add9b; continue;
 }
 0x40add9b => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x40addbf; continue; }
  pc = 0x40adda3; continue;
 }
 0x40adda3 => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x40addc8; continue; }
  pc = 0x40addab; continue;
 }
 0x40addab => {
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
 0x40addbf => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40addc8 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040add1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40add1b); }
 let mut pc: u32 = 0x40add1b;
 loop { match pc {
 0x40add1b => {
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
  pc = 0x40add31; continue;
 }
 0x40add31 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x40adcf6u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40add31; continue; }
  pc = 0x40add51; continue;
 }
 0x40add51 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40add51; continue; }
  pc = 0x40add5b; continue;
 }
 0x40add5b => {
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
pub fn f_040add65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40add65); }
 let mut pc: u32 = 0x40add65;
 loop { match pc {
 0x40add65 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x40add77; continue; }
  pc = 0x40add6d; continue;
 }
 0x40add6d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40add77; continue; }
  pc = 0x40add71; continue;
 }
 0x40add71 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40add77; continue;
 }
 0x40add77 => {
  cpu.push(0x40add7c); f_040add1b(cpu); if cpu.returned_to != 0x40add7c { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040addcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40addcc); }
 let mut pc: u32 = 0x40addcc;
 loop { match pc {
 0x40addcc => {
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
pub fn f_040addf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40addf0); }
 let mut pc: u32 = 0x40addf0;
 loop { match pc {
 0x40addf0 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040addf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40addf4); }
 let mut pc: u32 = 0x40addf4;
 loop { match pc {
 0x40addf4 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040addf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40addf8); }
 let mut pc: u32 = 0x40addf8;
 loop { match pc {
 0x40addf8 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040addfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40addfc); }
 let mut pc: u32 = 0x40addfc;
 loop { match pc {
 0x40addfc => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade00); }
 let mut pc: u32 = 0x40ade00;
 loop { match pc {
 0x40ade00 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade04); }
 let mut pc: u32 = 0x40ade04;
 loop { match pc {
 0x40ade04 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade08); }
 let mut pc: u32 = 0x40ade08;
 loop { match pc {
 0x40ade08 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade0c); }
 let mut pc: u32 = 0x40ade0c;
 loop { match pc {
 0x40ade0c => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade10); }
 let mut pc: u32 = 0x40ade10;
 loop { match pc {
 0x40ade10 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade14); }
 let mut pc: u32 = 0x40ade14;
 loop { match pc {
 0x40ade14 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade18); }
 let mut pc: u32 = 0x40ade18;
 loop { match pc {
 0x40ade18 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade1c); }
 let mut pc: u32 = 0x40ade1c;
 loop { match pc {
 0x40ade1c => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade20); }
 let mut pc: u32 = 0x40ade20;
 loop { match pc {
 0x40ade20 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade24); }
 let mut pc: u32 = 0x40ade24;
 loop { match pc {
 0x40ade24 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade28); }
 let mut pc: u32 = 0x40ade28;
 loop { match pc {
 0x40ade28 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade2c); }
 let mut pc: u32 = 0x40ade2c;
 loop { match pc {
 0x40ade2c => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade30); }
 let mut pc: u32 = 0x40ade30;
 loop { match pc {
 0x40ade30 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade34); }
 let mut pc: u32 = 0x40ade34;
 loop { match pc {
 0x40ade34 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade38); }
 let mut pc: u32 = 0x40ade38;
 loop { match pc {
 0x40ade38 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade3c); }
 let mut pc: u32 = 0x40ade3c;
 loop { match pc {
 0x40ade3c => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade40); }
 let mut pc: u32 = 0x40ade40;
 loop { match pc {
 0x40ade40 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade44); }
 let mut pc: u32 = 0x40ade44;
 loop { match pc {
 0x40ade44 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade48); }
 let mut pc: u32 = 0x40ade48;
 loop { match pc {
 0x40ade48 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade4c); }
 let mut pc: u32 = 0x40ade4c;
 loop { match pc {
 0x40ade4c => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade50); }
 let mut pc: u32 = 0x40ade50;
 loop { match pc {
 0x40ade50 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade54); }
 let mut pc: u32 = 0x40ade54;
 loop { match pc {
 0x40ade54 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade58); }
 let mut pc: u32 = 0x40ade58;
 loop { match pc {
 0x40ade58 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade5c); }
 let mut pc: u32 = 0x40ade5c;
 loop { match pc {
 0x40ade5c => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade60); }
 let mut pc: u32 = 0x40ade60;
 loop { match pc {
 0x40ade60 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade64); }
 let mut pc: u32 = 0x40ade64;
 loop { match pc {
 0x40ade64 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade68); }
 let mut pc: u32 = 0x40ade68;
 loop { match pc {
 0x40ade68 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade6c); }
 let mut pc: u32 = 0x40ade6c;
 loop { match pc {
 0x40ade6c => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade70); }
 let mut pc: u32 = 0x40ade70;
 loop { match pc {
 0x40ade70 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade74); }
 let mut pc: u32 = 0x40ade74;
 loop { match pc {
 0x40ade74 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade78); }
 let mut pc: u32 = 0x40ade78;
 loop { match pc {
 0x40ade78 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade7c); }
 let mut pc: u32 = 0x40ade7c;
 loop { match pc {
 0x40ade7c => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade80); }
 let mut pc: u32 = 0x40ade80;
 loop { match pc {
 0x40ade80 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade84); }
 let mut pc: u32 = 0x40ade84;
 loop { match pc {
 0x40ade84 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade88); }
 let mut pc: u32 = 0x40ade88;
 loop { match pc {
 0x40ade88 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade8c); }
 let mut pc: u32 = 0x40ade8c;
 loop { match pc {
 0x40ade8c => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade90); }
 let mut pc: u32 = 0x40ade90;
 loop { match pc {
 0x40ade90 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade94); }
 let mut pc: u32 = 0x40ade94;
 loop { match pc {
 0x40ade94 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade98); }
 let mut pc: u32 = 0x40ade98;
 loop { match pc {
 0x40ade98 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ade9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ade9c); }
 let mut pc: u32 = 0x40ade9c;
 loop { match pc {
 0x40ade9c => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adea0); }
 let mut pc: u32 = 0x40adea0;
 loop { match pc {
 0x40adea0 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adea4); }
 let mut pc: u32 = 0x40adea4;
 loop { match pc {
 0x40adea4 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adea8); }
 let mut pc: u32 = 0x40adea8;
 loop { match pc {
 0x40adea8 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adeac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adeac); }
 let mut pc: u32 = 0x40adeac;
 loop { match pc {
 0x40adeac => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adeb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adeb0); }
 let mut pc: u32 = 0x40adeb0;
 loop { match pc {
 0x40adeb0 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adeb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adeb4); }
 let mut pc: u32 = 0x40adeb4;
 loop { match pc {
 0x40adeb4 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adeb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adeb8); }
 let mut pc: u32 = 0x40adeb8;
 loop { match pc {
 0x40adeb8 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adebc); }
 let mut pc: u32 = 0x40adebc;
 loop { match pc {
 0x40adebc => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adec0); }
 let mut pc: u32 = 0x40adec0;
 loop { match pc {
 0x40adec0 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adec4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adec4); }
 let mut pc: u32 = 0x40adec4;
 loop { match pc {
 0x40adec4 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adec8); }
 let mut pc: u32 = 0x40adec8;
 loop { match pc {
 0x40adec8 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adecc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adecc); }
 let mut pc: u32 = 0x40adecc;
 loop { match pc {
 0x40adecc => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aded0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aded0); }
 let mut pc: u32 = 0x40aded0;
 loop { match pc {
 0x40aded0 => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aded4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aded4); }
 let mut pc: u32 = 0x40aded4;
 loop { match pc {
 0x40aded4 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aded8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aded8); }
 let mut pc: u32 = 0x40aded8;
 loop { match pc {
 0x40aded8 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adedc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adedc); }
 let mut pc: u32 = 0x40adedc;
 loop { match pc {
 0x40adedc => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adee0); }
 let mut pc: u32 = 0x40adee0;
 loop { match pc {
 0x40adee0 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adee4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adee4); }
 let mut pc: u32 = 0x40adee4;
 loop { match pc {
 0x40adee4 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adee8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adee8); }
 let mut pc: u32 = 0x40adee8;
 loop { match pc {
 0x40adee8 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adeec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adeec); }
 let mut pc: u32 = 0x40adeec;
 loop { match pc {
 0x40adeec => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adef0); }
 let mut pc: u32 = 0x40adef0;
 loop { match pc {
 0x40adef0 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adef4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adef4); }
 let mut pc: u32 = 0x40adef4;
 loop { match pc {
 0x40adef4 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adef8); }
 let mut pc: u32 = 0x40adef8;
 loop { match pc {
 0x40adef8 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adefc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adefc); }
 let mut pc: u32 = 0x40adefc;
 loop { match pc {
 0x40adefc => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf00); }
 let mut pc: u32 = 0x40adf00;
 loop { match pc {
 0x40adf00 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf04); }
 let mut pc: u32 = 0x40adf04;
 loop { match pc {
 0x40adf04 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf08); }
 let mut pc: u32 = 0x40adf08;
 loop { match pc {
 0x40adf08 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf0c); }
 let mut pc: u32 = 0x40adf0c;
 loop { match pc {
 0x40adf0c => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf10); }
 let mut pc: u32 = 0x40adf10;
 loop { match pc {
 0x40adf10 => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf14); }
 let mut pc: u32 = 0x40adf14;
 loop { match pc {
 0x40adf14 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf18); }
 let mut pc: u32 = 0x40adf18;
 loop { match pc {
 0x40adf18 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf1c); }
 let mut pc: u32 = 0x40adf1c;
 loop { match pc {
 0x40adf1c => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf20); }
 let mut pc: u32 = 0x40adf20;
 loop { match pc {
 0x40adf20 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf24); }
 let mut pc: u32 = 0x40adf24;
 loop { match pc {
 0x40adf24 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf28); }
 let mut pc: u32 = 0x40adf28;
 loop { match pc {
 0x40adf28 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf2c); }
 let mut pc: u32 = 0x40adf2c;
 loop { match pc {
 0x40adf2c => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf30); }
 let mut pc: u32 = 0x40adf30;
 loop { match pc {
 0x40adf30 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf34); }
 let mut pc: u32 = 0x40adf34;
 loop { match pc {
 0x40adf34 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf38); }
 let mut pc: u32 = 0x40adf38;
 loop { match pc {
 0x40adf38 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf3c); }
 let mut pc: u32 = 0x40adf3c;
 loop { match pc {
 0x40adf3c => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf40); }
 let mut pc: u32 = 0x40adf40;
 loop { match pc {
 0x40adf40 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf44); }
 let mut pc: u32 = 0x40adf44;
 loop { match pc {
 0x40adf44 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf48); }
 let mut pc: u32 = 0x40adf48;
 loop { match pc {
 0x40adf48 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf4c); }
 let mut pc: u32 = 0x40adf4c;
 loop { match pc {
 0x40adf4c => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040adf50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40adf50); }
 let mut pc: u32 = 0x40adf50;
 loop { match pc {
 0x40adf50 => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
