#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_020b30c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b30c7); }
 let mut pc: u32 = 0x20b30c7;
 loop { match pc {
 0x20b30c7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b30d6); f_020b2b14(cpu); if cpu.returned_to != 0x20b30d6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b30da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b30da); }
 let mut pc: u32 = 0x20b30da;
 loop { match pc {
 0x20b30da => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b30e9); f_020b2b14(cpu); if cpu.returned_to != 0x20b30e9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b314c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b314c); }
 let mut pc: u32 = 0x20b314c;
 loop { match pc {
 0x20b314c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x20b3190; continue; }
  pc = 0x20b315d; continue;
 }
 0x20b315d => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b3173; continue; }
  pc = 0x20b3166; continue;
 }
 0x20b3166 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x20b316f); f_020b30c7(cpu); if cpu.returned_to != 0x20b316f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x20b3173 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x20b3184); f_020b30c7(cpu); if cpu.returned_to != 0x20b3184 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x20b3190 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3198(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3198); }
 let mut pc: u32 = 0x20b3198;
 loop { match pc {
 0x20b3198 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x20b31dc; continue; }
  pc = 0x20b31a9; continue;
 }
 0x20b31a9 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b31bf; continue; }
  pc = 0x20b31b2; continue;
 }
 0x20b31b2 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x20b31bb); f_020b30da(cpu); if cpu.returned_to != 0x20b31bb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x20b31bf => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x20b31d0); f_020b30da(cpu); if cpu.returned_to != 0x20b31d0 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x20b31dc => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3350); }
 let mut pc: u32 = 0x20b3350;
 loop { match pc {
 0x20b3350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217ba24u32, v); }
  cpu.push(0x20b336b); f_020b421b(cpu); if cpu.returned_to != 0x20b336b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b337b; continue; }
  pc = 0x20b3376; continue;
 }
 0x20b3376 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b338c; continue; }
  pc = 0x20b337b; continue;
 }
 0x20b337b => {
  { let v: u32 = 0x3u32; cpu.w32(0x217ba24u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b338c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b3394); f_020b433f(cpu); if cpu.returned_to != 0x20b3394 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x20b33a1); f_020b4315(cpu); if cpu.returned_to != 0x20b33a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b33ad; continue; }
  pc = 0x20b33a8; continue;
 }
 0x20b33a8 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b33bb; continue; }
  pc = 0x20b33ad; continue;
 }
 0x20b33ad => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b33b6); f_020b3740(cpu); if cpu.returned_to != 0x20b33b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b33bd; continue;
 }
 0x20b33bb => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20b33bd; continue;
 }
 0x20b33bd => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b33e0; continue; }
  pc = 0x20b33c5; continue;
 }
 0x20b33c5 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x217ba24u32, v); }
  cpu.push(0x20b33d6); f_020b4345(cpu); if cpu.returned_to != 0x20b33d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b33e0 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b33ea; continue; }
  pc = 0x20b33e5; continue;
 }
 0x20b33e5 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b33ea; continue;
 }
 0x20b33ea => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b33f2); f_020b4291(cpu); if cpu.returned_to != 0x20b33f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x20b3433; continue; }
  pc = 0x20b33f9; continue;
 }
 0x20b33f9 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x20b3418; continue; }
  pc = 0x20b33fd; continue;
 }
 0x20b33fd => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b3418; continue; }
  pc = 0x20b3401; continue;
 }
 0x20b3401 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b340b); f_020b34c0(cpu); if cpu.returned_to != 0x20b340b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x217ba2cu32); cpu.push(0x20b3418); cpu.call(t); if cpu.returned_to != 0x20b3418 { return; }
  pc = 0x20b3418; continue;
 }
 0x20b3418 => {
  { let v: u32 = 0x5u32; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x217ba24u32, v); }
  cpu.push(0x20b3429); f_020b4345(cpu); if cpu.returned_to != 0x20b3429 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3433 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b3439); f_020b4345(cpu); if cpu.returned_to != 0x20b3439 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3450); }
 let mut pc: u32 = 0x20b3450;
 loop { match pc {
 0x20b3450 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217ba24u32, v); }
  cpu.push(0x20b3465); f_020b421b(cpu); if cpu.returned_to != 0x20b3465 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b3473; continue; }
  pc = 0x20b346e; continue;
 }
 0x20b346e => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b3486; continue; }
  pc = 0x20b3473; continue;
 }
 0x20b3473 => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217ba24u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3486 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b348e); f_020b433f(cpu); if cpu.returned_to != 0x20b348e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b34a2; continue; }
  pc = 0x20b3498; continue;
 }
 0x20b3498 => {
  { let v: u32 = 0x5u32; cpu.w32(0x217ba24u32, v); }
  pc = 0x20b34a2; continue;
 }
 0x20b34a2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b34a8); f_020b4345(cpu); if cpu.returned_to != 0x20b34a8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b34c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b34c0); }
 let mut pc: u32 = 0x20b34c0;
 loop { match pc {
 0x20b34c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b34d2); f_020b3500(cpu); if cpu.returned_to != 0x20b34d2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b34e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b34e0); }
 let mut pc: u32 = 0x20b34e0;
 loop { match pc {
 0x20b34e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b34f2); f_020b35a0(cpu); if cpu.returned_to != 0x20b34f2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3500); }
 let mut pc: u32 = 0x20b3500;
 loop { match pc {
 0x20b3500 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x20b3517; continue; }
  pc = 0x20b3513; continue;
 }
 0x20b3513 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x20b3519; continue;
 }
 0x20b3517 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20b3519; continue;
 }
 0x20b3519 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b351d; continue; }
  pc = 0x20b351b; continue;
 }
 0x20b351b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20b351d; continue;
 }
 0x20b351d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x1cu32; cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = 0x601u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20b3585); cpu.call(t); if cpu.returned_to != 0x20b3585 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b35a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b35a0); }
 let mut pc: u32 = 0x20b35a0;
 loop { match pc {
 0x20b35a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x1cu32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { cpu.sub32(cpu.ebp, cpu.esi); }
  if !cpu.cf { pc = 0x20b35d9; continue; }
  pc = 0x20b35d5; continue;
 }
 0x20b35d5 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20b35db; continue;
 }
 0x20b35d9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b35db; continue;
 }
 0x20b35db => {
  if (cpu.cf || cpu.zf) { pc = 0x20b35e1; continue; }
  pc = 0x20b35dd; continue;
 }
 0x20b35dd => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x20b35e3; continue;
 }
 0x20b35e1 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  pc = 0x20b35e3; continue;
 }
 0x20b35e3 => {
  { let v: u32 = 0x600u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20b3626); cpu.call(t); if cpu.returned_to != 0x20b3626 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3640); }
 let mut pc: u32 = 0x20b3640;
 loop { match pc {
 0x20b3640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x101u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20b366c); cpu.call(t); if cpu.returned_to != 0x20b366c { return; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3680); }
 let mut pc: u32 = 0x20b3680;
 loop { match pc {
 0x20b3680 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20b36a7); cpu.call(t); if cpu.returned_to != 0x20b36a7 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20b36b2; continue; }
  pc = 0x20b36ae; continue;
 }
 0x20b36ae => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b36e5; continue;
 }
 0x20b36b2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b36e5; continue;
 }
 0x20b36e5 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b3706; continue; }
  pc = 0x20b36e9; continue;
 }
 0x20b36e9 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b3703); f_020b35a0(cpu); if cpu.returned_to != 0x20b3703 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b3706; continue;
 }
 0x20b3706 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3710); }
 let mut pc: u32 = 0x20b3710;
 loop { match pc {
 0x20b3710 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b3733; continue; }
  pc = 0x20b3719; continue;
 }
 0x20b3719 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b3726); f_020b3500(cpu); if cpu.returned_to != 0x20b3726 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x217ba2cu32); cpu.push(0x20b3733); cpu.call(t); if cpu.returned_to != 0x20b3733 { return; }
  pc = 0x20b3733; continue;
 }
 0x20b3733 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3740); }
 let mut pc: u32 = 0x20b3740;
 loop { match pc {
 0x20b3740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x217ba28u32); cpu.push(0x20b3750); cpu.call(t); if cpu.returned_to != 0x20b3750 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b3768; continue; }
  pc = 0x20b3756; continue;
 }
 0x20b3756 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b3760); f_020b35a0(cpu); if cpu.returned_to != 0x20b3760 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b3768; continue;
 }
 0x20b3768 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3770); }
 let mut pc: u32 = 0x20b3770;
 loop { match pc {
 0x20b3770 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217ba2cu32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217ba2cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b379e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b379e); }
 let mut pc: u32 = 0x20b379e;
 loop { match pc {
 0x20b379e => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217bbccu32), 0x0u32); }
  if !cpu.zf { pc = 0x20b37b2; continue; }
  pc = 0x20b37a8; continue;
 }
 0x20b37a8 => {
  { cpu.sub16(cpu.r16(0x217bbd0u32), 0x0u16); }
  if cpu.zf { pc = 0x20b37bf; continue; }
  pc = 0x20b37b2; continue;
 }
 0x20b37b2 => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x217bbccu32); cpu.push(cpu.segments[1] as u32); cpu.push(0x20b37bd); cpu.call(t); if cpu.returned_to != 0x20b37bd { return; }
  pc = 0x20b3820; continue;
 }
 0x20b37bf => {
  { let v: u32 = cpu.r32(0x217bbbcu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x20b37cb; continue;
 }
 0x20b37cb => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b37d1; continue;
 }
 0x20b37d1 => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub32(cpu.ecx, cpu.r32(0x217bbbcu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x20b37bf; continue; }
  pc = 0x20b37dd; continue;
 }
 0x20b37dd => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b3808; continue; }
  pc = 0x20b37e4; continue;
 }
 0x20b37e4 => {
  { let v: u32 = cpu.r32(0x217bbdeu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x20b37f4; continue; }
  pc = 0x20b37ef; continue;
 }
 0x20b37ef => {
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  pc = 0x20b37f4; continue;
 }
 0x20b37f4 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.cf { pc = 0x20b37fa; continue; }
  pc = 0x20b37f8; continue;
 }
 0x20b37f8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b37fa; continue;
 }
 0x20b37fa => {
  { let v: u32 = 0x3e8u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = 0x1234ddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x20b3808; continue;
 }
 0x20b3808 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217bbe2u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x217bbc4u32)); cpu.eax = v; }
  pc = 0x20b3820; continue;
 }
 0x20b3820 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x217bbc0u32)); }
  if !cpu.cf { pc = 0x20b382d; continue; }
  pc = 0x20b3828; continue;
 }
 0x20b3828 => {
  { let v: u32 = cpu.r32(0x217bbc0u32); cpu.eax = v; }
  pc = 0x20b382d; continue;
 }
 0x20b382d => {
  { let v: u32 = cpu.eax; cpu.w32(0x217bbc0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3834); }
 let mut pc: u32 = 0x20b3834;
 loop { match pc {
 0x20b3834 => {
  { cpu.sub32(cpu.r32(0x217bbc0u32), 0x0u32); }
  if cpu.zf { pc = 0x20b3851; continue; }
  pc = 0x20b383d; continue;
 }
 0x20b383d => {
  cpu.push(0x20b3842); f_020b379e(cpu); if cpu.returned_to != 0x20b3842 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x217bbbcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x217bbc4u32, v); }
  pc = 0x20b3851; continue;
 }
 0x20b3851 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3852(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3852); }
 let mut pc: u32 = 0x20b3852;
 loop { match pc {
 0x20b3852 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x217bbeeu32); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x217bbbcu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x217bbbcu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x217bbe6u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b39ed; continue; }
  pc = 0x20b386e; continue;
 }
 0x20b386e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  { let v = cpu.r16(0x217bbeeu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x217bbe6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x217bbe6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x217c600u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x217c604u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x217c600u32; cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x217bbccu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20b38ae; continue; }
  pc = 0x20b38a4; continue;
 }
 0x20b38a4 => {
  { cpu.sub16(cpu.r16(0x217bbd0u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x20b38d4; continue; }
  pc = 0x20b38ae; continue;
 }
 0x20b38ae => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x217bbccu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20b38b3); cpu.push(cpu.segments[1] as u32); cpu.push(0x20b38b9); cpu.call(t); if cpu.returned_to != 0x20b38b9 { return; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x217bbc8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20b38f5; continue; }
  pc = 0x20b38c7; continue;
 }
 0x20b38c7 => {
  { let v: u32 = cpu.r32(0x217bbe2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217bbc8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20b3915; continue;
 }
 0x20b38d4 => {
  { let v: u32 = cpu.r32(0x217bbbcu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x217bbc8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20b38ef; continue; }
  pc = 0x20b38e2; continue;
 }
 0x20b38e2 => {
  { let v: u32 = cpu.r32(0x217bbe2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217bbc8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20b3915; continue;
 }
 0x20b38ef => {
  { let v: u32 = cpu.r32(0x217bbe2u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x20b38f5; continue;
 }
 0x20b38f5 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x217bbc8u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217bbc8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xf4240u32); }
  if cpu.cf { pc = 0x20b3915; continue; }
  pc = 0x20b3910; continue;
 }
 0x20b3910 => {
  { let v: u32 = 0xf4240u32; cpu.edx = v; }
  pc = 0x20b3915; continue;
 }
 0x20b3915 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x20b391a; continue;
 }
 0x20b391a => {
  { cpu.sub32(cpu.r32(0x217ba7cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if !cpu.zf { pc = 0x20b3947; continue; }
  pc = 0x20b3923; continue;
 }
 0x20b3923 => {
  { let v: u32 = cpu.r32(0x217babcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x20b392b; continue;
 }
 0x20b392b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x217bafcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if cpu.cf { pc = 0x20b3941; continue; }
  pc = 0x20b3933; continue;
 }
 0x20b3933 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x217bafcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x217bb3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x217bb3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20b392b; continue;
 }
 0x20b3941 => {
  { let v: u32 = cpu.eax; cpu.w32(0x217babcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20b3947; continue;
 }
 0x20b3947 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x20b391a; continue; }
  pc = 0x20b394f; continue;
 }
 0x20b394f => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  {  }
  { cpu.sub32(cpu.r32(0x217bbeau32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b3988; continue; }
  pc = 0x20b395d; continue;
 }
 0x20b395d => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x20b3962; continue;
 }
 0x20b3962 => {
  { cpu.sub32(cpu.r32(0x217bb3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x20b3980; continue; }
  pc = 0x20b396b; continue;
 }
 0x20b396b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x217bb3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x217bb3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x217bb7cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba3cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20b3978); cpu.push(0x20b397e); cpu.call(t); if cpu.returned_to != 0x20b397e { return; }
  cpu.poll(); pc = 0x20b3962; continue;
 }
 0x20b3980 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x20b3962; continue; }
  pc = 0x20b3988; continue;
 }
 0x20b3988 => {
  { cpu.sub32(cpu.r32(0x217bb78u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x20b39d1; continue; }
  pc = 0x20b3991; continue;
 }
 0x20b3991 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x217bb78u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x217bb78u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x217c600u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x217c604u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x217bbe6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x217bbe6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x217bbd2u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x217bbd6u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.r16(0x2cu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b39d1 => {
  { let v = cpu.r16(0x217c600u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x217c604u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x217bbe6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x217bbe6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20b39ec); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x20b39ed => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20b39f4); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b39f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b39f5); }
 let mut pc: u32 = 0x20b39f5;
 loop { match pc {
 0x20b39f5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v: u8 = 0x34u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217bbdeu32, v); }
  pc = 0x20b3a08; continue;
 }
 0x20b3a08 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b3a0e; continue;
 }
 0x20b3a0e => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x20b3a1b; continue; }
  pc = 0x20b3a1a; continue;
 }
 0x20b3a1a => {
  {  }
  pc = 0x20b3a1b; continue;
 }
 0x20b3a1b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3a1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3a1f); }
 let mut pc: u32 = 0x20b3a1f;
 loop { match pc {
 0x20b3a1f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xd68du32); }
  if !cpu.cf { pc = 0x20b3a42; continue; }
  pc = 0x20b3a31; continue;
 }
 0x20b3a31 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20bcu32; cpu.ebx = v; }
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x20b3a42; continue;
 }
 0x20b3a42 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b3a48); f_020b39f5(cpu); if cpu.returned_to != 0x20b3a48 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3a4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3a4e); }
 let mut pc: u32 = 0x20b3a4e;
 loop { match pc {
 0x20b3a4e => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x20b3a50); cpu.push(0x20b3a55); f_020ad4e0(cpu); if cpu.returned_to != 0x20b3a55 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x20b3a62; continue;
 }
 0x20b3a62 => {
  { cpu.sub32(cpu.r32(0x217ba7cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x20b3a77; continue; }
  pc = 0x20b3a6b; continue;
 }
 0x20b3a6b => {
  { let v: u32 = cpu.r32(0x217bafcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x20b3a77; continue; }
  pc = 0x20b3a75; continue;
 }
 0x20b3a75 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x20b3a77; continue;
 }
 0x20b3a77 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x20b3a62; continue; }
  pc = 0x20b3a7f; continue;
 }
 0x20b3a7f => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x217bbe2u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x20b3aac; continue; }
  pc = 0x20b3a87; continue;
 }
 0x20b3a87 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x217bbe2u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x20b3a8e); cpu.push(0x20b3a93); f_020b3a1f(cpu); if cpu.returned_to != 0x20b3a93 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x217babcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.flat_ds(0x20b3aa7); cpu.push(0x20b3aac); f_020b3834(cpu); if cpu.returned_to != 0x20b3aac { return; }
  pc = 0x20b3aac; continue;
 }
 0x20b3aac => {
  cpu.flat_ds(0x20b3aac); cpu.push(0x20b3ab1); f_020ad4d0(cpu); if cpu.returned_to != 0x20b3ab1 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.flat_ds(0x20b3ab3); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3bca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3bca); }
 let mut pc: u32 = 0x20b3bca;
 loop { match pc {
 0x20b3bca => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x20b3bcf); cpu.push(0x20b3bd4); f_020b39f5(cpu); if cpu.returned_to != 0x20b3bd4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217bbd2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x217bbd6u32.wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x20b3bfd; continue; }
  pc = 0x20b3bfc; continue;
 }
 0x20b3bfc => {
  {  }
  pc = 0x20b3bfd; continue;
 }
 0x20b3bfd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20b3c00); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3c01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3c01); }
 let mut pc: u32 = 0x20b3c01;
 loop { match pc {
 0x20b3c01 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3c29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3c29); }
 let mut pc: u32 = 0x20b3c29;
 loop { match pc {
 0x20b3c29 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x201u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3c4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c4c); } r_020b3c4c(cpu, 0x20b3c4c); }
pub fn f_020b3c53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c53); } r_020b3c4c(cpu, 0x20b3c53); }
pub fn f_020b3c62(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c62); } r_020b3c4c(cpu, 0x20b3c62); }
pub fn f_020b3c68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c68); } r_020b3c4c(cpu, 0x20b3c68); }
pub fn f_020b3c7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c7e); } r_020b3c4c(cpu, 0x20b3c7e); }
pub fn f_020b3c88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c88); } r_020b3c4c(cpu, 0x20b3c88); }
pub fn f_020b3c8f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3c8f); } r_020b3c4c(cpu, 0x20b3c8f); }
fn r_020b3c4c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b3c4c => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c53 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(3, v as u16); }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = 0x9999u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c62 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x99999999u32; cpu.esp = v; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c68 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffau32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c7e => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c88 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x20b3c8f => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3c96); }
 let mut pc: u32 = 0x20b3c96;
 loop { match pc {
 0x20b3c96 => {
  { let v = (cpu.ebx as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20b3ca0); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3ca1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3ca1); }
 let mut pc: u32 = 0x20b3ca1;
 loop { match pc {
 0x20b3ca1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffe4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.sub32(cpu.r32(0x217bbf2u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b3eb1; continue; }
  pc = 0x20b3cb8; continue;
 }
 0x20b3cb8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.unimplemented(0x20b3d12, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.unimplemented(0x20b3d63, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x217bc00u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x217bc00u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.unimplemented(0x20b3dc6, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x20b3c4cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x20b3c4cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x9u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.unimplemented(0x20b3e29, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x200u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20b3c68u32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20b3c62u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20b3c53u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20b3c88u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20b3c8fu32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20b3c7eu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217bbf2u32, v); }
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x20b3e86; continue; }
  pc = 0x20b3e83; continue;
 }
 0x20b3e83 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x20b3e86; continue;
 }
 0x20b3e86 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x204u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217bbf6u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217bbfau32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  pc = 0x20b3eb1; continue;
 }
 0x20b3eb1 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3eb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3eb7); }
 let mut pc: u32 = 0x20b3eb7;
 loop { match pc {
 0x20b3eb7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b3ef6; continue; }
  pc = 0x20b3ec6; continue;
 }
 0x20b3ec6 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x217bbf2u32)); }
  if !cpu.zf { pc = 0x20b3ef6; continue; }
  pc = 0x20b3ece; continue;
 }
 0x20b3ece => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x20b3ed6; continue; }
  pc = 0x20b3ed3; continue;
 }
 0x20b3ed3 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x20b3ed6; continue;
 }
 0x20b3ed6 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217bbf6u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x217bbfau32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x217bbf2u32, v); }
  pc = 0x20b3ef6; continue;
 }
 0x20b3ef6 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3f00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3f00); }
 let mut pc: u32 = 0x20b3f00;
 loop { match pc {
 0x20b3f00 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x20b3f11; continue; }
  pc = 0x20b3f10; continue;
 }
 0x20b3f10 => {
  {  }
  pc = 0x20b3f11; continue;
 }
 0x20b3f11 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3f15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3f15); }
 let mut pc: u32 = 0x20b3f15;
 loop { match pc {
 0x20b3f15 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffffffbu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.esp); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x14u32); cpu.esp = v; }
  cpu.poll(); pc = cpu.edx; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b3f67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b3f67); }
 let mut pc: u32 = 0x20b3f67;
 loop { match pc {
 0x20b3f67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x20b3f72); cpu.push(0x20b3f77); f_020ad4e0(cpu); if cpu.returned_to != 0x20b3f77 { return; }
  { cpu.df = false; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0xffffffc6u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffeau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.flags() as u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let value = cpu.r16(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2); let v: u16 = value; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x20b3fd6; continue; }
  pc = 0x20b3fb6; continue;
 }
 0x20b3fb6 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b3fd6; continue;
 }
 0x20b3fd6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x32u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x300u32; cpu.eax = v; }
  { let v: u32 = 0x66u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ecx = v; }
  { let v: u32 = 0xffffffc6u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x20b402f; continue; }
  pc = 0x20b4000; continue;
 }
 0x20b4000 => {
  { let v: u16 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffdeu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffcau32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xffffffc6u32.wrapping_add(cpu.ebp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0xffffffd6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20b402f; continue;
 }
 0x20b402f => {
  { let v: u32 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x20b4034); cpu.push(0x20b4039); f_020ad4d0(cpu); if cpu.returned_to != 0x20b4039 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x20b4040); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4041(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4041); }
 let mut pc: u32 = 0x20b4041;
 loop { match pc {
 0x20b4041 => {
  { let v: u32 = cpu.r32(0x217bbe6u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4047(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4047); }
 let mut pc: u32 = 0x20b4047;
 loop { match pc {
 0x20b4047 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b404f); f_020ad4e0(cpu); if cpu.returned_to != 0x20b404f { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x20b4054; continue;
 }
 0x20b4054 => {
  { cpu.sub32(cpu.r32(0x217ba7cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x20b406c; continue; }
  pc = 0x20b405d; continue;
 }
 0x20b405d => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x20b4054; continue; }
  pc = 0x20b4065; continue;
 }
 0x20b4065 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b407f; continue;
 }
 0x20b406c => {
  { let v: u32 = 0x1u32; cpu.w32(0x217ba7cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217ba3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b407f; continue;
 }
 0x20b407f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b4085); f_020ad4d0(cpu); if cpu.returned_to != 0x20b4085 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4088(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4088); }
 let mut pc: u32 = 0x20b4088;
 loop { match pc {
 0x20b4088 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b4090); f_020ad4e0(cpu); if cpu.returned_to != 0x20b4090 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b40a1; continue; }
  pc = 0x20b4098; continue;
 }
 0x20b4098 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let a = cpu.r32(0x217bb7cu32.wrapping_add(cpu.ecx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x217bb7cu32.wrapping_add(cpu.ecx), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x20b40a1; continue;
 }
 0x20b40a1 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b40a7); f_020ad4d0(cpu); if cpu.returned_to != 0x20b40a7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b40aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b40aa); }
 let mut pc: u32 = 0x20b40aa;
 loop { match pc {
 0x20b40aa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b40b2); f_020ad4e0(cpu); if cpu.returned_to != 0x20b40b2 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b40c4; continue; }
  pc = 0x20b40ba; continue;
 }
 0x20b40ba => {
  { let v: u32 = 0x0u32; cpu.w32(0x217ba7cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b40c4; continue;
 }
 0x20b40c4 => {
  cpu.push(0x20b40c9); f_020ad4d0(cpu); if cpu.returned_to != 0x20b40c9 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b40cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b40cb); }
 let mut pc: u32 = 0x20b40cb;
 loop { match pc {
 0x20b40cb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b40d1); f_020ad4e0(cpu); if cpu.returned_to != 0x20b40d1 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x20b40d6; continue;
 }
 0x20b40d6 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b40dc); f_020ac410(cpu); if cpu.returned_to != 0x20b40dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b40d6; continue; }
  pc = 0x20b40e4; continue;
 }
 0x20b40e4 => {
  cpu.push(0x20b40e9); f_020ad4d0(cpu); if cpu.returned_to != 0x20b40e9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b40eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b40eb); }
 let mut pc: u32 = 0x20b40eb;
 loop { match pc {
 0x20b40eb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b40f3); f_020ad4e0(cpu); if cpu.returned_to != 0x20b40f3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b410e; continue; }
  pc = 0x20b40fb; continue;
 }
 0x20b40fb => {
  { cpu.sub32(cpu.r32(0x217ba7cu32.wrapping_add(cpu.ecx)), 0x1u32); }
  if !cpu.zf { pc = 0x20b410e; continue; }
  pc = 0x20b4104; continue;
 }
 0x20b4104 => {
  { let v: u32 = 0x2u32; cpu.w32(0x217ba7cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b410e; continue;
 }
 0x20b410e => {
  cpu.push(0x20b4113); f_020ad4d0(cpu); if cpu.returned_to != 0x20b4113 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4115(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4115); }
 let mut pc: u32 = 0x20b4115;
 loop { match pc {
 0x20b4115 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b411b); f_020ad4e0(cpu); if cpu.returned_to != 0x20b411b { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x20b4120; continue;
 }
 0x20b4120 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b4126); f_020ac670(cpu); if cpu.returned_to != 0x20b4126 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b4120; continue; }
  pc = 0x20b412e; continue;
 }
 0x20b412e => {
  cpu.push(0x20b4133); f_020ad4d0(cpu); if cpu.returned_to != 0x20b4133 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4135(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4135); }
 let mut pc: u32 = 0x20b4135;
 loop { match pc {
 0x20b4135 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b413d); f_020ad4e0(cpu); if cpu.returned_to != 0x20b413d { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b4158; continue; }
  pc = 0x20b4145; continue;
 }
 0x20b4145 => {
  { cpu.sub32(cpu.r32(0x217ba7cu32.wrapping_add(cpu.ecx)), 0x2u32); }
  if !cpu.zf { pc = 0x20b4158; continue; }
  pc = 0x20b414e; continue;
 }
 0x20b414e => {
  { let v: u32 = 0x1u32; cpu.w32(0x217ba7cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b4158; continue;
 }
 0x20b4158 => {
  cpu.push(0x20b415d); f_020ad4d0(cpu); if cpu.returned_to != 0x20b415d { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b415f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b415f); }
 let mut pc: u32 = 0x20b415f;
 loop { match pc {
 0x20b415f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b4165); f_020ad4e0(cpu); if cpu.returned_to != 0x20b4165 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x20b416a; continue;
 }
 0x20b416a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b4170); f_020ac540(cpu); if cpu.returned_to != 0x20b4170 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b416a; continue; }
  pc = 0x20b4178; continue;
 }
 0x20b4178 => {
  cpu.push(0x20b417d); f_020ad4d0(cpu); if cpu.returned_to != 0x20b417d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b417f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b417f); }
 let mut pc: u32 = 0x20b417f;
 loop { match pc {
 0x20b417f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b418a); f_020ad4e0(cpu); if cpu.returned_to != 0x20b418a { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217bafcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x217babcu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x20b41a5); f_020b3a4e(cpu); if cpu.returned_to != 0x20b41a5 { return; }
  cpu.push(0x20b41aa); f_020ad4d0(cpu); if cpu.returned_to != 0x20b41aa { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b41af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b41af); }
 let mut pc: u32 = 0x20b41af;
 loop { match pc {
 0x20b41af => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b41b7); f_020ad4e0(cpu); if cpu.returned_to != 0x20b41b7 { return; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x20b41cf); f_020ac980(cpu); if cpu.returned_to != 0x20b41cf { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x20b41d7); f_020ad4d0(cpu); if cpu.returned_to != 0x20b41d7 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b41d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b41d9); }
 let mut pc: u32 = 0x20b41d9;
 loop { match pc {
 0x20b41d9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20b41e1); f_020ad4e0(cpu); if cpu.returned_to != 0x20b41e1 { return; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b41ee; continue; }
  pc = 0x20b41e7; continue;
 }
 0x20b41e7 => {
  { let v: u32 = 0xd68du32; cpu.eax = v; }
  pc = 0x20b4202; continue;
 }
 0x20b41ee => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x2e9cu32; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  pc = 0x20b4202; continue;
 }
 0x20b4202 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x20b420b); f_020ac980(cpu); if cpu.returned_to != 0x20b420b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x20b4213); f_020ad4d0(cpu); if cpu.returned_to != 0x20b4213 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4215(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4215); }
 let mut pc: u32 = 0x20b4215;
 loop { match pc {
 0x20b4215 => {
  { let v: u32 = cpu.r32(0x217bbdeu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b421b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b421b); }
 let mut pc: u32 = 0x20b421b;
 loop { match pc {
 0x20b421b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x20b4230; continue; }
  pc = 0x20b422c; continue;
 }
 0x20b422c => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20b4230; continue;
 }
 0x20b4230 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4237(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4237); }
 let mut pc: u32 = 0x20b4237;
 loop { match pc {
 0x20b4237 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x20b424b; continue; }
  pc = 0x20b4247; continue;
 }
 0x20b4247 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20b424b; continue;
 }
 0x20b424b => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4252(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4252); }
 let mut pc: u32 = 0x20b4252;
 loop { match pc {
 0x20b4252 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x20b427d; continue; }
  pc = 0x20b4263; continue;
 }
 0x20b4263 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x20b4270; continue; }
  pc = 0x20b4269; continue;
 }
 0x20b4269 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b428c; continue;
 }
 0x20b4270 => {
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { cpu.poll(); pc = 0x20b4269; continue; }
  pc = 0x20b4278; continue;
 }
 0x20b4278 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  pc = 0x20b428c; continue;
 }
 0x20b427d => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20b428c; continue;
 }
 0x20b428c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4291(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4291); }
 let mut pc: u32 = 0x20b4291;
 loop { match pc {
 0x20b4291 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x20b429a; continue;
 }
 0x20b429a => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x20b42ac; continue; }
  pc = 0x20b42a7; continue;
 }
 0x20b42a7 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x20b42ac; continue;
 }
 0x20b42ac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x20b42c9; continue; }
  pc = 0x20b42b7; continue;
 }
 0x20b42b7 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b42c9; continue; }
  pc = 0x20b42be; continue;
 }
 0x20b42be => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x20b42c9; continue; }
  pc = 0x20b42c4; continue;
 }
 0x20b42c4 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x20b429a; continue; }
  pc = 0x20b42c9; continue;
 }
 0x20b42c9 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b42d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b42d3); }
 let mut pc: u32 = 0x20b42d3;
 loop { match pc {
 0x20b42d3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x20b42dc; continue;
 }
 0x20b42dc => {
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x20b42ee; continue; }
  pc = 0x20b42e9; continue;
 }
 0x20b42e9 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x20b42ee; continue;
 }
 0x20b42ee => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x20b430b; continue; }
  pc = 0x20b42f9; continue;
 }
 0x20b42f9 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b430b; continue; }
  pc = 0x20b4300; continue;
 }
 0x20b4300 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x20b430b; continue; }
  pc = 0x20b4306; continue;
 }
 0x20b4306 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x20b42dc; continue; }
  pc = 0x20b430b; continue;
 }
 0x20b430b => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4315(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b4315); } r_020b4315(cpu, 0x20b4315); }
pub fn f_020b433f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b433f); } r_020b4315(cpu, 0x20b433f); }
fn r_020b4315(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b4315 => {
  { let v: u16 = 0x4200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20b4319; continue;
 }
 0x20b4319 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xeu32.wrapping_add(cpu.ebp)) as u32; cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b433f => {
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20b4319; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4345(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4345); }
 let mut pc: u32 = 0x20b4345;
 loop { match pc {
 0x20b4345 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4353(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4353); }
 let mut pc: u32 = 0x20b4353;
 loop { match pc {
 0x20b4353 => {
  { let v = 0x217c608u32; cpu.push(v); }
  { let v = 0x217ba3cu32; cpu.push(v); }
  cpu.push(0x20b4362); f_020b35a0(cpu); if cpu.returned_to != 0x20b4362 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x20b4353u32; cpu.push(v); }
  { let v = 0x20b3790u32; cpu.push(v); }
  cpu.push(0x20b4374); f_020b35a0(cpu); if cpu.returned_to != 0x20b4374 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4380); }
 let mut pc: u32 = 0x20b4380;
 loop { match pc {
 0x20b4380 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c70cu32), 0x0u32); }
  if cpu.zf { pc = 0x20b4403; continue; }
  pc = 0x20b438c; continue;
 }
 0x20b438c => {
  { let v = 0x20b4380u32; cpu.push(v); }
  { let v = 0x20b5070u32; cpu.push(v); }
  cpu.push(0x20b439b); f_020b3500(cpu); if cpu.returned_to != 0x20b439b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x22d1e80u32; cpu.push(v); }
  cpu.push(0x20b43aa); f_020b34c0(cpu); if cpu.returned_to != 0x20b43aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x22d1fc0u32; cpu.push(v); }
  cpu.push(0x20b43b9); f_020b34c0(cpu); if cpu.returned_to != 0x20b43b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x22d1ec0u32; cpu.push(v); }
  cpu.push(0x20b43cb); f_020b34c0(cpu); if cpu.returned_to != 0x20b43cb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x22d2010u32; cpu.push(v); }
  cpu.push(0x20b43da); f_020b34c0(cpu); if cpu.returned_to != 0x20b43da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c708u32; cpu.push(v); }
  cpu.push(0x20b43e9); f_020b34c0(cpu); if cpu.returned_to != 0x20b43e9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2028u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20b43fa); f_020b34c0(cpu); if cpu.returned_to != 0x20b43fa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c70cu32, v); }
  pc = 0x20b4403; continue;
 }
 0x20b4403 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4410); }
 let mut pc: u32 = 0x20b4410;
 loop { match pc {
 0x20b4410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x20b441d); f_020b379e(cpu); if cpu.returned_to != 0x20b441d { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x20b441f; continue;
 }
 0x20b441f => {
  cpu.push(0x20b4424); f_020b379e(cpu); if cpu.returned_to != 0x20b4424 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x20b441f; continue; }
  pc = 0x20b4428; continue;
 }
 0x20b4428 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4430); }
 let mut pc: u32 = 0x20b4430;
 loop { match pc {
 0x20b4430 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20b4445; continue; }
  pc = 0x20b443b; continue;
 }
 0x20b443b => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x217c608u32, v); }
  pc = 0x20b448b; continue;
 }
 0x20b4445 => {
  { let v: u32 = 0x217c608u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b444c; continue;
 }
 0x20b444c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b444c; continue; }
  pc = 0x20b4456; continue;
 }
 0x20b4456 => {
  { let v: u32 = 0x217c608u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b448b; continue; }
  pc = 0x20b446b; continue;
 }
 0x20b446b => {
  { let v: u8 = cpu.r8(0x217c607u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x20b448b; continue; }
  pc = 0x20b4476; continue;
 }
 0x20b4476 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3au8); }
  if cpu.zf { pc = 0x20b448b; continue; }
  pc = 0x20b447b; continue;
 }
 0x20b447b => {
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x217c608u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x217c609u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b448b; continue;
 }
 0x20b448b => {
  { let v: u32 = 0x217c608u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b44a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b44a0); }
 let mut pc: u32 = 0x20b44a0;
 loop { match pc {
 0x20b44a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x20b44a6; continue;
 }
 0x20b44a6 => {
  { let v: u32 = cpu.r32(0x22d1e80u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b44b9; continue; }
  pc = 0x20b44b0; continue;
 }
 0x20b44b0 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b44b6); f_020ac0d0(cpu); if cpu.returned_to != 0x20b44b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b44b9; continue;
 }
 0x20b44b9 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xfffffffcu32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b44a6; continue; }
  pc = 0x20b44c1; continue;
 }
 0x20b44c1 => {
  cpu.push(0x20b44c6); f_020ac390(cpu); if cpu.returned_to != 0x20b44c6 { return; }
  cpu.push(0x20b44cb); f_020b3bca(cpu); if cpu.returned_to != 0x20b44cb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b44d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b44d0); }
 let mut pc: u32 = 0x20b44d0;
 loop { match pc {
 0x20b3ab4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x217bbeeu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x217bbeeu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x20b3aca); cpu.push(0x20b3acf); f_020b4353(cpu); if cpu.returned_to != 0x20b3acf { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x217bbe6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x217bbeau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x217bbe2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x217bbf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x217ba7cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x217babcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x217bafcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x217bb3cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = 0x1684u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = (cpu.edi as u16); cpu.set_segment(0, v as u16); }
  { cpu.interrupt(0x2f); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x217bbccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x217bbceu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217bbd0u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x200u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217bbd2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x217bbd6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x217bbdau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x20b3852u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2u32; cpu.w32(0x217bab8u32.wrapping_add(cpu.bases[3]), v); }
  { let v = 0xd68du32; cpu.push(v); }
  { let v = 0x3cu32; cpu.push(v); }
  cpu.flat_ds(0x20b3bb1); cpu.push(0x20b3bb6); f_020ac980(cpu); if cpu.returned_to != 0x20b3bb6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x20b3bc4; continue; }
  pc = 0x20b3bc3; continue;
 }
 0x20b3bc3 => {
  {  }
  pc = 0x20b3bc4; continue;
 }
 0x20b3bc4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20b3bc9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b44d0 => {
  cpu.flat_ds(0x20b44d0); cpu.push(0x20b44d5); f_020b5070(cpu); if cpu.returned_to != 0x20b44d5 { return; }
  { let v = 0xc8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x20b44dc); cpu.push(0x20b44e1); f_020ad4f0(cpu); if cpu.returned_to != 0x20b44e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.flat_ds(0x20b44e8); cpu.push(0x20b44ed); f_020ad4f0(cpu); if cpu.returned_to != 0x20b44ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.flat_ds(0x20b44f7); cpu.push(0x20b44fc); f_020ad4f0(cpu); if cpu.returned_to != 0x20b44fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.flat_ds(0x20b4503); cpu.push(0x20b4508); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4508 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.flat_ds(0x20b450f); cpu.push(0x20b4514); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4514 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  cpu.flat_ds(0x20b451b); cpu.push(0x20b4520); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4520 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x28fu32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  cpu.flat_ds(0x20b452a); cpu.push(0x20b452f); f_020ad4f0(cpu); if cpu.returned_to != 0x20b452f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.flat_ds(0x20b4536); cpu.push(0x20b453b); f_020ad4f0(cpu); if cpu.returned_to != 0x20b453b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.flat_ds(0x20b4542); cpu.push(0x20b4547); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4547 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  cpu.flat_ds(0x20b454e); cpu.push(0x20b4553); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4553 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.flat_ds(0x20b455a); cpu.push(0x20b455f); f_020ad4f0(cpu); if cpu.returned_to != 0x20b455f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x78u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.flat_ds(0x20b4566); cpu.push(0x20b456b); f_020ad4f0(cpu); if cpu.returned_to != 0x20b456b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.flat_ds(0x20b4572); cpu.push(0x20b4577); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4577 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xdu32; cpu.push(v); }
  cpu.flat_ds(0x20b457e); cpu.push(0x20b4583); f_020ad4f0(cpu); if cpu.returned_to != 0x20b4583 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  cpu.flat_ds(0x20b458a); cpu.push(0x20b458f); f_020ad4f0(cpu); if cpu.returned_to != 0x20b458f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  cpu.flat_ds(0x20b4596); cpu.push(0x20b459b); f_020ad4f0(cpu); if cpu.returned_to != 0x20b459b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.flat_ds(0x20b45a2); cpu.push(0x20b45a7); f_020ad4f0(cpu); if cpu.returned_to != 0x20b45a7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.flat_ds(0x20b45ae); cpu.push(0x20b45b3); f_020ad4f0(cpu); if cpu.returned_to != 0x20b45b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.flat_ds(0x20b45ba); cpu.push(0x20b45bf); f_020ad4f0(cpu); if cpu.returned_to != 0x20b45bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x13u32; cpu.push(v); }
  cpu.flat_ds(0x20b45c6); cpu.push(0x20b45cb); f_020ad4f0(cpu); if cpu.returned_to != 0x20b45cb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b45d0; continue;
 }
 0x20b45d0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d1e7cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b45d0; continue; }
  pc = 0x20b45e0; continue;
 }
 0x20b45e0 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x22d1ec0u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20b3ab4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b45f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b45f0); }
 let mut pc: u32 = 0x20b45f0;
 loop { match pc {
 0x20b45f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  if cpu.zf { pc = 0x20b468e; continue; }
  pc = 0x20b4614; continue;
 }
 0x20b4614 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba30u32); cpu.push(0x20b461b); cpu.call(t); if cpu.returned_to != 0x20b461b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b468e; continue; }
  pc = 0x20b4621; continue;
 }
 0x20b4621 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b463b; continue; }
  pc = 0x20b4636; continue;
 }
 0x20b4636 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x20b463b; continue;
 }
 0x20b463b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x3au32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x303u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3au32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  cpu.push(0x20b4668); f_020ad040(cpu); if cpu.returned_to != 0x20b4668 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b4677; continue; }
  pc = 0x20b4670; continue;
 }
 0x20b4670 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b4677 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x22d202cu32; cpu.edi = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x22d202cu32; cpu.eax = v; }
  { cpu.movs(4, true); }
  pc = 0x20b468e; continue;
 }
 0x20b468e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b46a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b46a0); }
 let mut pc: u32 = 0x20b46a0;
 loop { match pc {
 0x20b46a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b46b8; continue; }
  pc = 0x20b46ae; continue;
 }
 0x20b46ae => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x20b46b5); cpu.call(t); if cpu.returned_to != 0x20b46b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b46b8; continue;
 }
 0x20b46b8 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b46c9; continue; }
  pc = 0x20b46c0; continue;
 }
 0x20b46c0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b46c6); f_020ac410(cpu); if cpu.returned_to != 0x20b46c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b46c9; continue;
 }
 0x20b46c9 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b46f2; continue; }
  pc = 0x20b46cf; continue;
 }
 0x20b46cf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b46e0; continue; }
  pc = 0x20b46d7; continue;
 }
 0x20b46d7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b46dd); f_020ad190(cpu); if cpu.returned_to != 0x20b46dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b46e0; continue;
 }
 0x20b46e0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b46ef); f_020ad040(cpu); if cpu.returned_to != 0x20b46ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20b46f2; continue;
 }
 0x20b46f2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x20b4700); f_020ad310(cpu); if cpu.returned_to != 0x20b4700 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4713); f_020b3640(cpu); if cpu.returned_to != 0x20b4713 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b471c); f_020b3710(cpu); if cpu.returned_to != 0x20b471c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b4721; continue;
 }
 0x20b4721 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x22d1e80u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20b4731; continue; }
  pc = 0x20b4729; continue;
 }
 0x20b4729 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d1e80u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b4731; continue;
 }
 0x20b4731 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4721; continue; }
  pc = 0x20b4739; continue;
 }
 0x20b4739 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4740); }
 let mut pc: u32 = 0x20b4740;
 loop { match pc {
 0x20b4740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x20b474f); f_020b3740(cpu); if cpu.returned_to != 0x20b474f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b4775; continue; }
  pc = 0x20b4758; continue;
 }
 0x20b4758 => {
  { let v: u32 = 0x216acf0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4764; continue;
 }
 0x20b4764 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4764; continue; }
  pc = 0x20b476e; continue;
 }
 0x20b476e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b4775 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xfu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b478f); f_020b3680(cpu); if cpu.returned_to != 0x20b478f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b47bc; continue; }
  pc = 0x20b4796; continue;
 }
 0x20b4796 => {
  { let v: u32 = 0x216ad1cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b47a2; continue;
 }
 0x20b47a2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b47a2; continue; }
  pc = 0x20b47ac; continue;
 }
 0x20b47ac => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b47b2); f_020b3710(cpu); if cpu.returned_to != 0x20b47b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b47bc => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x216ad38u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b47e9; continue;
 }
 0x20b47e9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b47f7; continue; }
  pc = 0x20b47f1; continue;
 }
 0x20b47f1 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b47f7; continue; }
  pc = 0x20b47f5; continue;
 }
 0x20b47f5 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b47f7; continue;
 }
 0x20b47f7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4804; continue; }
  pc = 0x20b47fc; continue;
 }
 0x20b47fc => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4804; continue; }
  pc = 0x20b4801; continue;
 }
 0x20b4801 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4804; continue;
 }
 0x20b4804 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4813; continue; }
  pc = 0x20b4808; continue;
 }
 0x20b4808 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4813; continue; }
  pc = 0x20b480c; continue;
 }
 0x20b480c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4813; continue; }
  pc = 0x20b480f; continue;
 }
 0x20b480f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b47e9; continue;
 }
 0x20b4813 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b481a; continue; }
  pc = 0x20b4818; continue;
 }
 0x20b4818 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b481a; continue;
 }
 0x20b481a => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b482a; continue; }
  pc = 0x20b481e; continue;
 }
 0x20b481e => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b48ae; continue;
 }
 0x20b482a => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x216ad40u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x20b4837; continue;
 }
 0x20b4837 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4845; continue; }
  pc = 0x20b483f; continue;
 }
 0x20b483f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4845; continue; }
  pc = 0x20b4843; continue;
 }
 0x20b4843 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4845; continue;
 }
 0x20b4845 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4852; continue; }
  pc = 0x20b484a; continue;
 }
 0x20b484a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4852; continue; }
  pc = 0x20b484f; continue;
 }
 0x20b484f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4852; continue;
 }
 0x20b4852 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4861; continue; }
  pc = 0x20b4856; continue;
 }
 0x20b4856 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4861; continue; }
  pc = 0x20b485a; continue;
 }
 0x20b485a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4861; continue; }
  pc = 0x20b485d; continue;
 }
 0x20b485d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4837; continue;
 }
 0x20b4861 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4868; continue; }
  pc = 0x20b4866; continue;
 }
 0x20b4866 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4868; continue;
 }
 0x20b4868 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20b48a7; continue; }
  pc = 0x20b486c; continue;
 }
 0x20b486c => {
  { let v: u32 = 0x216ad48u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4878; continue;
 }
 0x20b4878 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4878; continue; }
  pc = 0x20b4882; continue;
 }
 0x20b4882 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b4894); f_020b3640(cpu); if cpu.returned_to != 0x20b4894 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b489d); f_020b3710(cpu); if cpu.returned_to != 0x20b489d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b48a7 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b48ae; continue;
 }
 0x20b48ae => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b48b2; continue;
 }
 0x20b48b2 => {
  { cpu.sub32(cpu.r32(0x22d1e80u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x20b48ca; continue; }
  pc = 0x20b48bb; continue;
 }
 0x20b48bb => {
  { let v: u32 = cpu.ebp; cpu.w32(0x22d1e80u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b48d3; continue;
 }
 0x20b48ca => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { cpu.poll(); pc = 0x20b48b2; continue; }
  pc = 0x20b48d3; continue;
 }
 0x20b48d3 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x20b4912; continue; }
  pc = 0x20b48d8; continue;
 }
 0x20b48d8 => {
  { let v: u32 = 0x216ad60u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b48e4; continue;
 }
 0x20b48e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b48e4; continue; }
  pc = 0x20b48ee; continue;
 }
 0x20b48ee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b48ff); f_020b3640(cpu); if cpu.returned_to != 0x20b48ff { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4908); f_020b3710(cpu); if cpu.returned_to != 0x20b4908 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b4912 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x66u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b4930); f_020ad3d0(cpu); if cpu.returned_to != 0x20b4930 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x36u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x34u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x20b4950); f_020ad310(cpu); if cpu.returned_to != 0x20b4950 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x300u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b4970); f_020ad040(cpu); if cpu.returned_to != 0x20b4970 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b498b; continue; }
  pc = 0x20b497f; continue;
 }
 0x20b497f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b4a17; continue;
 }
 0x20b498b => {
  { let v = 0x20b5010u32; cpu.push(v); }
  cpu.push(0x20b4995); f_020acb50(cpu); if cpu.returned_to != 0x20b4995 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b49eb; continue; }
  pc = 0x20b49a0; continue;
 }
 0x20b49a0 => {
  { let v: u32 = 0x216ad78u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b49ac; continue;
 }
 0x20b49ac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b49ac; continue; }
  pc = 0x20b49b6; continue;
 }
 0x20b49b6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x20b49c4); f_020ad310(cpu); if cpu.returned_to != 0x20b49c4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b49d8); f_020b3640(cpu); if cpu.returned_to != 0x20b49d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b49e1); f_020b3710(cpu); if cpu.returned_to != 0x20b49e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b49eb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b49f2); f_020aca40(cpu); if cpu.returned_to != 0x20b49f2 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b4a08); f_020ac8c0(cpu); if cpu.returned_to != 0x20b4a08 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b4a14); f_020ac670(cpu); if cpu.returned_to != 0x20b4a14 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b4a17; continue;
 }
 0x20b4a17 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4a20); }
 let mut pc: u32 = 0x20b4a20;
 loop { match pc {
 0x20b4a20 => {
  { let v: u32 = 0x22d1ec0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4a30); }
 let mut pc: u32 = 0x20b4a30;
 loop { match pc {
 0x20b4a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d1fc0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d1fc0u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4a50); }
 let mut pc: u32 = 0x20b4a50;
 loop { match pc {
 0x20b4a50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x390u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = 0x118u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4ab2); f_020b421b(cpu); if cpu.returned_to != 0x20b4ab2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b4b2a; continue; }
  pc = 0x20b4ac1; continue;
 }
 0x20b4ac1 => {
  { cpu.sub8(cpu.r8(0x217c608u32), 0x0u8); }
  if !cpu.zf { pc = 0x20b4ad1; continue; }
  pc = 0x20b4aca; continue;
 }
 0x20b4aca => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b4edd; continue;
 }
 0x20b4ad1 => {
  { let v: u32 = 0x217c608u32; cpu.esi = v; }
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4adf; continue;
 }
 0x20b4adf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4adf; continue; }
  pc = 0x20b4ae9; continue;
 }
 0x20b4ae9 => {
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b4aff; continue;
 }
 0x20b4aff => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4aff; continue; }
  pc = 0x20b4b05; continue;
 }
 0x20b4b05 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x228u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b4b14); f_020b421b(cpu); if cpu.returned_to != 0x20b4b14 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b4b2a; continue; }
  pc = 0x20b4b23; continue;
 }
 0x20b4b23 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b4edd; continue;
 }
 0x20b4b2a => {
  { let v: u32 = 0x10cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b4b3f; continue;
 }
 0x20b4b3f => {
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20b4b5e); f_020b4ef0(cpu); if cpu.returned_to != 0x20b4b5e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b4e6a; continue; }
  pc = 0x20b4b66; continue;
 }
 0x20b4b66 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x20b4b98; continue; }
  pc = 0x20b4b80; continue;
 }
 0x20b4b80 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x20b4b98; continue; }
  pc = 0x20b4b8a; continue;
 }
 0x20b4b8a => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x325u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b4b80; continue; }
  pc = 0x20b4b98; continue;
 }
 0x20b4b98 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20b4ba7; continue;
 }
 0x20b4b9c => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x20b4bbe; continue; }
  pc = 0x20b4ba6; continue;
 }
 0x20b4ba6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b4ba7; continue;
 }
 0x20b4ba7 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x20b4b9c; continue; }
  pc = 0x20b4bbe; continue;
 }
 0x20b4bbe => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x20b4bd4; continue;
 }
 0x20b4bc9 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x20b4beb; continue; }
  pc = 0x20b4bd3; continue;
 }
 0x20b4bd3 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b4bd4; continue;
 }
 0x20b4bd4 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x20b4bc9; continue; }
  pc = 0x20b4beb; continue;
 }
 0x20b4beb => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x20b4c02; continue;
 }
 0x20b4bf7 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x20b4c19; continue; }
  pc = 0x20b4c01; continue;
 }
 0x20b4c01 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b4c02; continue;
 }
 0x20b4c02 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x20b4bf7; continue; }
  pc = 0x20b4c19; continue;
 }
 0x20b4c19 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { cpu.poll(); pc = 0x20b4b3f; continue; }
  pc = 0x20b4c3d; continue;
 }
 0x20b4c3d => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x3bu8); }
  if cpu.zf { cpu.poll(); pc = 0x20b4b3f; continue; }
  pc = 0x20b4c49; continue;
 }
 0x20b4c49 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x216aca4u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4c55; continue;
 }
 0x20b4c55 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4c63; continue; }
  pc = 0x20b4c5d; continue;
 }
 0x20b4c5d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4c63; continue; }
  pc = 0x20b4c61; continue;
 }
 0x20b4c61 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4c63; continue;
 }
 0x20b4c63 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4c70; continue; }
  pc = 0x20b4c68; continue;
 }
 0x20b4c68 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4c70; continue; }
  pc = 0x20b4c6d; continue;
 }
 0x20b4c6d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4c70; continue;
 }
 0x20b4c70 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4c7f; continue; }
  pc = 0x20b4c74; continue;
 }
 0x20b4c74 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4c7f; continue; }
  pc = 0x20b4c78; continue;
 }
 0x20b4c78 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4c7f; continue; }
  pc = 0x20b4c7b; continue;
 }
 0x20b4c7b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4c55; continue;
 }
 0x20b4c7f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4c86; continue; }
  pc = 0x20b4c84; continue;
 }
 0x20b4c84 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4c86; continue;
 }
 0x20b4c86 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b4ca4; continue; }
  pc = 0x20b4c8a; continue;
 }
 0x20b4c8a => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4c95; continue;
 }
 0x20b4c95 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4c95; continue; }
  pc = 0x20b4c9f; continue;
 }
 0x20b4c9f => {
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4ca4 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x216acacu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4cb0; continue;
 }
 0x20b4cb0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4cbe; continue; }
  pc = 0x20b4cb8; continue;
 }
 0x20b4cb8 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4cbe; continue; }
  pc = 0x20b4cbc; continue;
 }
 0x20b4cbc => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4cbe; continue;
 }
 0x20b4cbe => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4ccb; continue; }
  pc = 0x20b4cc3; continue;
 }
 0x20b4cc3 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4ccb; continue; }
  pc = 0x20b4cc8; continue;
 }
 0x20b4cc8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4ccb; continue;
 }
 0x20b4ccb => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4cda; continue; }
  pc = 0x20b4ccf; continue;
 }
 0x20b4ccf => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4cda; continue; }
  pc = 0x20b4cd3; continue;
 }
 0x20b4cd3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4cda; continue; }
  pc = 0x20b4cd6; continue;
 }
 0x20b4cd6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4cb0; continue;
 }
 0x20b4cda => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4ce1; continue; }
  pc = 0x20b4cdf; continue;
 }
 0x20b4cdf => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4ce1; continue;
 }
 0x20b4ce1 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b4cfa; continue; }
  pc = 0x20b4ce5; continue;
 }
 0x20b4ce5 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4ceb; continue;
 }
 0x20b4ceb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4ceb; continue; }
  pc = 0x20b4cf5; continue;
 }
 0x20b4cf5 => {
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4cfa => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x216acb4u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4d06; continue;
 }
 0x20b4d06 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4d14; continue; }
  pc = 0x20b4d0e; continue;
 }
 0x20b4d0e => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4d14; continue; }
  pc = 0x20b4d12; continue;
 }
 0x20b4d12 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4d14; continue;
 }
 0x20b4d14 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4d21; continue; }
  pc = 0x20b4d19; continue;
 }
 0x20b4d19 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4d21; continue; }
  pc = 0x20b4d1e; continue;
 }
 0x20b4d1e => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4d21; continue;
 }
 0x20b4d21 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4d30; continue; }
  pc = 0x20b4d25; continue;
 }
 0x20b4d25 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4d30; continue; }
  pc = 0x20b4d29; continue;
 }
 0x20b4d29 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4d30; continue; }
  pc = 0x20b4d2c; continue;
 }
 0x20b4d2c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4d06; continue;
 }
 0x20b4d30 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4d37; continue; }
  pc = 0x20b4d35; continue;
 }
 0x20b4d35 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4d37; continue;
 }
 0x20b4d37 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b4d55; continue; }
  pc = 0x20b4d3b; continue;
 }
 0x20b4d3b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4d45); f_020b4f70(cpu); if cpu.returned_to != 0x20b4d45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x100u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4d55 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216acbcu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4d61; continue;
 }
 0x20b4d61 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4d6f; continue; }
  pc = 0x20b4d69; continue;
 }
 0x20b4d69 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4d6f; continue; }
  pc = 0x20b4d6d; continue;
 }
 0x20b4d6d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4d6f; continue;
 }
 0x20b4d6f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4d7c; continue; }
  pc = 0x20b4d74; continue;
 }
 0x20b4d74 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4d7c; continue; }
  pc = 0x20b4d79; continue;
 }
 0x20b4d79 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4d7c; continue;
 }
 0x20b4d7c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4d8b; continue; }
  pc = 0x20b4d80; continue;
 }
 0x20b4d80 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4d8b; continue; }
  pc = 0x20b4d84; continue;
 }
 0x20b4d84 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4d8b; continue; }
  pc = 0x20b4d87; continue;
 }
 0x20b4d87 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4d61; continue;
 }
 0x20b4d8b => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4d92; continue; }
  pc = 0x20b4d90; continue;
 }
 0x20b4d90 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4d92; continue;
 }
 0x20b4d92 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b4db0; continue; }
  pc = 0x20b4d96; continue;
 }
 0x20b4d96 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4da0); f_020b4f70(cpu); if cpu.returned_to != 0x20b4da0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x102u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4db0 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x216acc0u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4dbc; continue;
 }
 0x20b4dbc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4dca; continue; }
  pc = 0x20b4dc4; continue;
 }
 0x20b4dc4 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4dca; continue; }
  pc = 0x20b4dc8; continue;
 }
 0x20b4dc8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4dca; continue;
 }
 0x20b4dca => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4dd7; continue; }
  pc = 0x20b4dcf; continue;
 }
 0x20b4dcf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4dd7; continue; }
  pc = 0x20b4dd4; continue;
 }
 0x20b4dd4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4dd7; continue;
 }
 0x20b4dd7 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4de6; continue; }
  pc = 0x20b4ddb; continue;
 }
 0x20b4ddb => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4de6; continue; }
  pc = 0x20b4ddf; continue;
 }
 0x20b4ddf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4de6; continue; }
  pc = 0x20b4de2; continue;
 }
 0x20b4de2 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4dbc; continue;
 }
 0x20b4de6 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4ded; continue; }
  pc = 0x20b4deb; continue;
 }
 0x20b4deb => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4ded; continue;
 }
 0x20b4ded => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b4e0b; continue; }
  pc = 0x20b4df1; continue;
 }
 0x20b4df1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4dfb); f_020b4f70(cpu); if cpu.returned_to != 0x20b4dfb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x104u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4e0b => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v: u32 = 0x216acccu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b4e17; continue;
 }
 0x20b4e17 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4e25; continue; }
  pc = 0x20b4e1f; continue;
 }
 0x20b4e1f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4e25; continue; }
  pc = 0x20b4e23; continue;
 }
 0x20b4e23 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4e25; continue;
 }
 0x20b4e25 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4e32; continue; }
  pc = 0x20b4e2a; continue;
 }
 0x20b4e2a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4e32; continue; }
  pc = 0x20b4e2f; continue;
 }
 0x20b4e2f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b4e32; continue;
 }
 0x20b4e32 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b4e41; continue; }
  pc = 0x20b4e36; continue;
 }
 0x20b4e36 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b4e41; continue; }
  pc = 0x20b4e3a; continue;
 }
 0x20b4e3a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b4e41; continue; }
  pc = 0x20b4e3d; continue;
 }
 0x20b4e3d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b4e17; continue;
 }
 0x20b4e41 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b4e48; continue; }
  pc = 0x20b4e46; continue;
 }
 0x20b4e46 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b4e48; continue;
 }
 0x20b4e48 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4b3f; continue; }
  pc = 0x20b4e50; continue;
 }
 0x20b4e50 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b4e5a); f_020b4f70(cpu); if cpu.returned_to != 0x20b4e5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x106u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b4b3f; continue;
 }
 0x20b4e6a => {
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b4e77); f_020b4345(cpu); if cpu.returned_to != 0x20b4e77 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b4ea3; continue; }
  pc = 0x20b4e91; continue;
 }
 0x20b4e91 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b4ec8; continue; }
  pc = 0x20b4ea3; continue;
 }
 0x20b4ea3 => {
  { let v: u32 = 0x216acd8u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b4eaf; continue;
 }
 0x20b4eaf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b4eaf; continue; }
  pc = 0x20b4eb9; continue;
 }
 0x20b4eb9 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b4ec8 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x20b4edd; continue;
 }
 0x20b4edd => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4ef0); }
 let mut pc: u32 = 0x20b4ef0;
 loop { match pc {
 0x20b4ef0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x20b4f57; continue; }
  pc = 0x20b4f05; continue;
 }
 0x20b4f05 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x20b4f2f; continue; }
  pc = 0x20b4f0b; continue;
 }
 0x20b4f0b => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b4f25); f_020b4291(cpu); if cpu.returned_to != 0x20b4f25 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b4f57; continue; }
  pc = 0x20b4f2f; continue;
 }
 0x20b4f2f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xdu8); }
  if cpu.zf { pc = 0x20b4f50; continue; }
  pc = 0x20b4f40; continue;
 }
 0x20b4f40 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if !cpu.zf { pc = 0x20b4f4c; continue; }
  pc = 0x20b4f46; continue;
 }
 0x20b4f46 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b4f57; continue;
 }
 0x20b4f4c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x20b4f50; continue;
 }
 0x20b4f50 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x20b4f05; continue; }
  pc = 0x20b4f57; continue;
 }
 0x20b4f57 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b4f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b4f70); }
 let mut pc: u32 = 0x20b4f70;
 loop { match pc {
 0x20b4f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b4fd8; continue;
 }
 0x20b4f91 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b4fa1; continue; }
  pc = 0x20b4f99; continue;
 }
 0x20b4f99 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b4fa1; continue; }
  pc = 0x20b4f9e; continue;
 }
 0x20b4f9e => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20b4fa1; continue;
 }
 0x20b4fa1 => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x20b4fab; continue; }
  pc = 0x20b4fa6; continue;
 }
 0x20b4fa6 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b4fd6; continue;
 }
 0x20b4fab => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x20b4fd6; continue; }
  pc = 0x20b4fb0; continue;
 }
 0x20b4fb0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b4fd2; continue; }
  pc = 0x20b4fb6; continue;
 }
 0x20b4fb6 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x216ac90u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20b4fcd; continue; }
  pc = 0x20b4fbe; continue;
 }
 0x20b4fbe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b4fd2; continue;
 }
 0x20b4fcd => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b4fb6; continue; }
  pc = 0x20b4fd2; continue;
 }
 0x20b4fd2 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x20b4fec; continue; }
  pc = 0x20b4fd6; continue;
 }
 0x20b4fd6 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x20b4fd8; continue;
 }
 0x20b4fd8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x20b4f91; continue; }
  pc = 0x20b4fec; continue;
 }
 0x20b4fec => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b4ffd; continue; }
  pc = 0x20b4ff4; continue;
 }
 0x20b4ff4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b4ffd; continue;
 }
 0x20b4ffd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5010); }
 let mut pc: u32 = 0x20b5010;
 loop { match pc {
 0x20b5010 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c708u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b505c; continue; }
  pc = 0x20b501b; continue;
 }
 0x20b501b => {
  { let v: u32 = 0x1u32; cpu.w32(0x217c708u32, v); }
  cpu.push(0x20b502a); f_020ad180(cpu); if cpu.returned_to != 0x20b502a { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2028u32, v); }
  cpu.push(0x20b5042); f_020ad040(cpu); if cpu.returned_to != 0x20b5042 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d2028u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x20b5053); f_020ad170(cpu); if cpu.returned_to != 0x20b5053 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x217c708u32, v); }
  pc = 0x20b505c; continue;
 }
 0x20b505c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5070); }
 let mut pc: u32 = 0x20b5070;
 loop { match pc {
 0x20b5070 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c70cu32), 0x0u32); }
  if !cpu.zf { pc = 0x20b50f6; continue; }
  pc = 0x20b507c; continue;
 }
 0x20b507c => {
  { let v = 0x20b4380u32; cpu.push(v); }
  { let v = 0x20b5070u32; cpu.push(v); }
  cpu.push(0x20b508b); f_020b35a0(cpu); if cpu.returned_to != 0x20b508b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x22d1e80u32; cpu.push(v); }
  cpu.push(0x20b509a); f_020b34e0(cpu); if cpu.returned_to != 0x20b509a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x22d1fc0u32; cpu.push(v); }
  cpu.push(0x20b50a9); f_020b34e0(cpu); if cpu.returned_to != 0x20b50a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x22d1ec0u32; cpu.push(v); }
  cpu.push(0x20b50bb); f_020b34e0(cpu); if cpu.returned_to != 0x20b50bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x22d2010u32; cpu.push(v); }
  cpu.push(0x20b50ca); f_020b34e0(cpu); if cpu.returned_to != 0x20b50ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c708u32; cpu.push(v); }
  cpu.push(0x20b50d9); f_020b34e0(cpu); if cpu.returned_to != 0x20b50d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2028u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20b50ed); f_020b34e0(cpu); if cpu.returned_to != 0x20b50ed { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c70cu32, v); }
  pc = 0x20b50f6; continue;
 }
 0x20b50f6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5100); }
 let mut pc: u32 = 0x20b5100;
 loop { match pc {
 0x20b5100 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c794u32), 0x0u32); }
  if cpu.zf { pc = 0x20b51b4; continue; }
  pc = 0x20b5110; continue;
 }
 0x20b5110 => {
  { let v = 0x20b5100u32; cpu.push(v); }
  { let v = 0x20b6e80u32; cpu.push(v); }
  cpu.push(0x20b511f); f_020b3500(cpu); if cpu.returned_to != 0x20b511f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x217c710u32; cpu.push(v); }
  cpu.push(0x20b5131); f_020b34c0(cpu); if cpu.returned_to != 0x20b5131 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c790u32; cpu.push(v); }
  cpu.push(0x20b5140); f_020b34c0(cpu); if cpu.returned_to != 0x20b5140 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2058u32; cpu.push(v); }
  cpu.push(0x20b514f); f_020b34c0(cpu); if cpu.returned_to != 0x20b514f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x22d2044u32; cpu.push(v); }
  cpu.push(0x20b515e); f_020b34c0(cpu); if cpu.returned_to != 0x20b515e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2060u32; cpu.push(v); }
  cpu.push(0x20b516d); f_020b34c0(cpu); if cpu.returned_to != 0x20b516d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2064u32; cpu.push(v); }
  cpu.push(0x20b517c); f_020b34c0(cpu); if cpu.returned_to != 0x20b517c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2054u32; cpu.push(v); }
  cpu.push(0x20b518b); f_020b34c0(cpu); if cpu.returned_to != 0x20b518b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d205cu32; cpu.push(v); }
  cpu.push(0x20b519a); f_020b34c0(cpu); if cpu.returned_to != 0x20b519a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2050u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20b51ab); f_020b34c0(cpu); if cpu.returned_to != 0x20b51ab { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c794u32, v); }
  pc = 0x20b51b4; continue;
 }
 0x20b51b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b51c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b51c0); }
 let mut pc: u32 = 0x20b51c0;
 loop { match pc {
 0x20b51c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b51d6; continue; }
  pc = 0x20b51d1; continue;
 }
 0x20b51d1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b51d6; continue;
 }
 0x20b51d6 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b51df; continue; }
  pc = 0x20b51da; continue;
 }
 0x20b51da => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b51df; continue;
 }
 0x20b51df => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20b51ef; continue; }
  pc = 0x20b51e6; continue;
 }
 0x20b51e6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b51ef; continue;
 }
 0x20b51ef => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b51fc; continue; }
  pc = 0x20b51f7; continue;
 }
 0x20b51f7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b51fc; continue;
 }
 0x20b51fc => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5360); }
 let mut pc: u32 = 0x20b5360;
 loop { match pc {
 0x20b5360 => {
  { let v: u32 = 0xbu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x5394020bu32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  {  }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b53a0; continue; }
  pc = 0x20b5385; continue;
 }
 0x20b5385 => {
  cpu.poll(); pc = cpu.r32(0x20b5360u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x20b538d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x20b53a0; continue;
 }
 0x20b5394 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  pc = 0x20b53a0; continue;
 }
 0x20b539b => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x20b53a0; continue;
 }
 0x20b53a0 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x44u32.wrapping_add(cpu.ecx))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x22d1fd8u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 15u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x20b53f7; continue; }
  pc = 0x20b53e2; continue;
 }
 0x20b53e2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x20b53f7; continue; }
  pc = 0x20b53ed; continue;
 }
 0x20b53ed => {
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x20b53f7; continue; }
  pc = 0x20b53f4; continue;
 }
 0x20b53f4 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x20b53f7; continue;
 }
 0x20b53f7 => {
  { let v: u32 = cpu.ebx.wrapping_add(0xffu32); cpu.ebx = v; }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5410); }
 let mut pc: u32 = 0x20b5410;
 loop { match pc {
 0x20b5410 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5420); }
 let mut pc: u32 = 0x20b5420;
 loop { match pc {
 0x20b5420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b545b; continue; }
  pc = 0x20b5437; continue;
 }
 0x20b5437 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b5439; continue;
 }
 0x20b5439 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20b544d; continue; }
  pc = 0x20b5444; continue;
 }
 0x20b5444 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b544a); f_020b69f0(cpu); if cpu.returned_to != 0x20b544a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b544d; continue;
 }
 0x20b544d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8a8u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b5439; continue; }
  pc = 0x20b545b; continue;
 }
 0x20b545b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5460); }
 let mut pc: u32 = 0x20b5460;
 loop { match pc {
 0x20b5460 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b546b; continue; }
  pc = 0x20b5468; continue;
 }
 0x20b5468 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b546b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x868u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5480); }
 let mut pc: u32 = 0x20b5480;
 loop { match pc {
 0x20b5480 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b549b; continue; }
  pc = 0x20b5488; continue;
 }
 0x20b5488 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x868u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b549b; continue;
 }
 0x20b549b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b54a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b54a0); }
 let mut pc: u32 = 0x20b54a0;
 loop { match pc {
 0x20b54a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b54bb; continue; }
  pc = 0x20b54a9; continue;
 }
 0x20b54a9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20b54bb; continue;
 }
 0x20b54bb => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5500); }
 let mut pc: u32 = 0x20b5500;
 loop { match pc {
 0x20b5500 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b5546; continue; }
  pc = 0x20b550a; continue;
 }
 0x20b550a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20b5546; continue; }
  pc = 0x20b5512; continue;
 }
 0x20b5512 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x20b5546; continue; }
  pc = 0x20b5517; continue;
 }
 0x20b5517 => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b5536; continue; }
  pc = 0x20b552f; continue;
 }
 0x20b552f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x20b5536); cpu.call(t); if cpu.returned_to != 0x20b5536 { return; }
  pc = 0x20b5536; continue;
 }
 0x20b5536 => {
  { cpu.sub32(cpu.r32(0x864u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b5546; continue; }
  pc = 0x20b553f; continue;
 }
 0x20b553f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x20b5546); cpu.call(t); if cpu.returned_to != 0x20b5546 { return; }
  pc = 0x20b5546; continue;
 }
 0x20b5546 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5550); }
 let mut pc: u32 = 0x20b5550;
 loop { match pc {
 0x20b5550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b55a4; continue; }
  pc = 0x20b555d; continue;
 }
 0x20b555d => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x8u32); }
  if !cpu.zf { pc = 0x20b55a4; continue; }
  pc = 0x20b5563; continue;
 }
 0x20b5563 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b55a4; continue; }
  pc = 0x20b5574; continue;
 }
 0x20b5574 => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b559a); f_020ad040(cpu); if cpu.returned_to != 0x20b559a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b55a4; continue;
 }
 0x20b55a4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b55d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b55d0); }
 let mut pc: u32 = 0x20b55d0;
 loop { match pc {
 0x20b55d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b564a; continue; }
  pc = 0x20b55de; continue;
 }
 0x20b55de => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20b564a; continue; }
  pc = 0x20b55e4; continue;
 }
 0x20b55e4 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b564a; continue; }
  pc = 0x20b55f2; continue;
 }
 0x20b55f2 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b564a; continue; }
  pc = 0x20b55f8; continue;
 }
 0x20b55f8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20b564a; continue; }
  pc = 0x20b561a; continue;
 }
 0x20b561a => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5640); f_020ad040(cpu); if cpu.returned_to != 0x20b5640 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b564a; continue;
 }
 0x20b564a => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5660); }
 let mut pc: u32 = 0x20b5660;
 loop { match pc {
 0x20b5660 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5676; continue; }
  pc = 0x20b5668; continue;
 }
 0x20b5668 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b5676; continue;
 }
 0x20b5676 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5680); }
 let mut pc: u32 = 0x20b5680;
 loop { match pc {
 0x20b5680 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b568f; continue; }
  pc = 0x20b5688; continue;
 }
 0x20b5688 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b568f; continue;
 }
 0x20b568f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5690); }
 let mut pc: u32 = 0x20b5690;
 loop { match pc {
 0x20b5690 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b569e; continue; }
  pc = 0x20b5698; continue;
 }
 0x20b5698 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b569e => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b56f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b56f0); }
 let mut pc: u32 = 0x20b56f0;
 loop { match pc {
 0x20b56f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b570d; continue; }
  pc = 0x20b56fc; continue;
 }
 0x20b56fc => {
  { cpu.sub32(cpu.edx, cpu.r32(0x54u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b570d; continue; }
  pc = 0x20b5701; continue;
 }
 0x20b5701 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b570a); f_020b69f0(cpu); if cpu.returned_to != 0x20b570a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b570d; continue;
 }
 0x20b570d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5710); }
 let mut pc: u32 = 0x20b5710;
 loop { match pc {
 0x20b5710 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5719; continue; }
  pc = 0x20b5718; continue;
 }
 0x20b5718 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5719 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5720); }
 let mut pc: u32 = 0x20b5720;
 loop { match pc {
 0x20b5720 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b573d; continue; }
  pc = 0x20b572c; continue;
 }
 0x20b572c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b573d; continue; }
  pc = 0x20b5731; continue;
 }
 0x20b5731 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b573a); f_020b69f0(cpu); if cpu.returned_to != 0x20b573a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b573d; continue;
 }
 0x20b573d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5740); }
 let mut pc: u32 = 0x20b5740;
 loop { match pc {
 0x20b5740 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5749; continue; }
  pc = 0x20b5748; continue;
 }
 0x20b5748 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5749 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5750); }
 let mut pc: u32 = 0x20b5750;
 loop { match pc {
 0x20b5750 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b575f; continue; }
  pc = 0x20b5758; continue;
 }
 0x20b5758 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b575f; continue;
 }
 0x20b575f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5760); }
 let mut pc: u32 = 0x20b5760;
 loop { match pc {
 0x20b5760 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5769; continue; }
  pc = 0x20b5768; continue;
 }
 0x20b5768 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5769 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5770); }
 let mut pc: u32 = 0x20b5770;
 loop { match pc {
 0x20b5770 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b579e; continue; }
  pc = 0x20b577d; continue;
 }
 0x20b577d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x44u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20b578b; continue; }
  pc = 0x20b5782; continue;
 }
 0x20b5782 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x48u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b579e; continue; }
  pc = 0x20b578b; continue;
 }
 0x20b578b => {
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b579b); f_020b69f0(cpu); if cpu.returned_to != 0x20b579b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b579e; continue;
 }
 0x20b579e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b57a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b57a0); }
 let mut pc: u32 = 0x20b57a0;
 loop { match pc {
 0x20b57a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b57c4; continue; }
  pc = 0x20b57a8; continue;
 }
 0x20b57a8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b57c4; continue;
 }
 0x20b57c4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b57d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b57d0); }
 let mut pc: u32 = 0x20b57d0;
 loop { match pc {
 0x20b57d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b57d9; continue; }
  pc = 0x20b57d8; continue;
 }
 0x20b57d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b57d9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b57e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b57e0); }
 let mut pc: u32 = 0x20b57e0;
 loop { match pc {
 0x20b57e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b58c4; continue; }
  pc = 0x20b57ed; continue;
 }
 0x20b57ed => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xfffffffeu32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d1fd4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x2b11u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b5883; continue; }
  pc = 0x20b587e; continue;
 }
 0x20b587e => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x20b588c; continue; }
  pc = 0x20b5883; continue;
 }
 0x20b5883 => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b5893; continue;
 }
 0x20b588c => {
  { let v: u32 = 0x40u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b5893; continue;
 }
 0x20b5893 => {
  { let v: u32 = 0x0u32; cpu.w32(0x85cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x860u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b58c1); f_020b69f0(cpu); if cpu.returned_to != 0x20b58c1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b58c4; continue;
 }
 0x20b58c4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b58d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b58d0); }
 let mut pc: u32 = 0x20b58d0;
 loop { match pc {
 0x20b58d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b58df; continue; }
  pc = 0x20b58d8; continue;
 }
 0x20b58d8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b58df; continue;
 }
 0x20b58df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b58e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b58e0); }
 let mut pc: u32 = 0x20b58e0;
 loop { match pc {
 0x20b58e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x20b58ec); f_020ad4e0(cpu); if cpu.returned_to != 0x20b58ec { return; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b590c; continue; }
  pc = 0x20b58f5; continue;
 }
 0x20b58f5 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x20b58f8; continue;
 }
 0x20b58f8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x20b590c; continue; }
  pc = 0x20b58fe; continue;
 }
 0x20b58fe => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b58f8; continue; }
  pc = 0x20b590c; continue;
 }
 0x20b590c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x64u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20b5932; continue; }
  pc = 0x20b5911; continue;
 }
 0x20b5911 => {
  { let v: u32 = 0x216aef4u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b591d; continue;
 }
 0x20b591d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b591d; continue; }
  pc = 0x20b5927; continue;
 }
 0x20b5927 => {
  cpu.push(0x20b592c); f_020ad4d0(cpu); if cpu.returned_to != 0x20b592c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5932 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5943); f_020aba90(cpu); if cpu.returned_to != 0x20b5943 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x20b594b); f_020ad4d0(cpu); if cpu.returned_to != 0x20b594b { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5960); }
 let mut pc: u32 = 0x20b5960;
 loop { match pc {
 0x20b5960 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b5989; continue; }
  pc = 0x20b5971; continue;
 }
 0x20b5971 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x20b5974; continue;
 }
 0x20b5974 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x4u32); }
  if !cpu.zf { pc = 0x20b597b; continue; }
  pc = 0x20b597a; continue;
 }
 0x20b597a => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20b597b; continue;
 }
 0x20b597b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b5974; continue; }
  pc = 0x20b5989; continue;
 }
 0x20b5989 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5990); }
 let mut pc: u32 = 0x20b5990;
 loop { match pc {
 0x20b5990 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b599d); f_020ac0d0(cpu); if cpu.returned_to != 0x20b599d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b59b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b59b0); }
 let mut pc: u32 = 0x20b59b0;
 loop { match pc {
 0x20b59b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x216aeccu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b59c7); f_020acc50(cpu); if cpu.returned_to != 0x20b59c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b59eb; continue; }
  pc = 0x20b59ce; continue;
 }
 0x20b59ce => {
  { let v: u32 = 0x216aed4u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b59da; continue;
 }
 0x20b59da => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b59da; continue; }
  pc = 0x20b59e4; continue;
 }
 0x20b59e4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b5a20; continue;
 }
 0x20b59eb => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5a00); f_020abec0(cpu); if cpu.returned_to != 0x20b5a00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5a1e; continue; }
  pc = 0x20b5a10; continue;
 }
 0x20b5a10 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5a1e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b5a20; continue;
 }
 0x20b5a20 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5a30); }
 let mut pc: u32 = 0x20b5a30;
 loop { match pc {
 0x20b5a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5a41); f_020ac180(cpu); if cpu.returned_to != 0x20b5a41 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5a69; continue; }
  pc = 0x20b5a4a; continue;
 }
 0x20b5a4a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5a55); f_020b5b50(cpu); if cpu.returned_to != 0x20b5a55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5a67; continue; }
  pc = 0x20b5a5e; continue;
 }
 0x20b5a5e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5a64); f_020ac0d0(cpu); if cpu.returned_to != 0x20b5a64 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b5a67; continue;
 }
 0x20b5a67 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b5a69; continue;
 }
 0x20b5a69 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5a70); }
 let mut pc: u32 = 0x20b5a70;
 loop { match pc {
 0x20b5a70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5a88); f_020b3350(cpu); if cpu.returned_to != 0x20b5a88 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5af6; continue; }
  pc = 0x20b5a91; continue;
 }
 0x20b5a91 => {
  { cpu.sub8(cpu.r8(0x217c608u32), 0x0u8); }
  if !cpu.zf { pc = 0x20b5ab7; continue; }
  pc = 0x20b5a9a; continue;
 }
 0x20b5a9a => {
  { let v: u32 = 0x216aeb4u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5aa6; continue;
 }
 0x20b5aa6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5aa6; continue; }
  pc = 0x20b5ab0; continue;
 }
 0x20b5ab0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b5b42; continue;
 }
 0x20b5ab7 => {
  { let v: u32 = 0x217c608u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5ac0; continue;
 }
 0x20b5ac0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5ac0; continue; }
  pc = 0x20b5aca; continue;
 }
 0x20b5aca => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b5adb; continue;
 }
 0x20b5adb => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5adb; continue; }
  pc = 0x20b5ae1; continue;
 }
 0x20b5ae1 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5aed); f_020b3350(cpu); if cpu.returned_to != 0x20b5aed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x20b5a9a; continue; }
  pc = 0x20b5af6; continue;
 }
 0x20b5af6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5b02); f_020ac180(cpu); if cpu.returned_to != 0x20b5b02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20b5b0d); f_020b3710(cpu); if cpu.returned_to != 0x20b5b0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20b5b20; continue; }
  pc = 0x20b5b14; continue;
 }
 0x20b5b14 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5b20 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5b2e); f_020b5b50(cpu); if cpu.returned_to != 0x20b5b2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5b40; continue; }
  pc = 0x20b5b37; continue;
 }
 0x20b5b37 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b5b3d); f_020ac0d0(cpu); if cpu.returned_to != 0x20b5b3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b5b40; continue;
 }
 0x20b5b40 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b5b42; continue;
 }
 0x20b5b42 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b5b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b5b50); }
 let mut pc: u32 = 0x20b5b50;
 loop { match pc {
 0x20b5b50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x20b5b5c); f_020b6e80(cpu); if cpu.returned_to != 0x20b5b5c { return; }
  { let v = 0x90u32; cpu.push(v); }
  cpu.push(0x20b5b66); f_020b3740(cpu); if cpu.returned_to != 0x20b5b66 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5b8c; continue; }
  pc = 0x20b5b6f; continue;
 }
 0x20b5b6f => {
  { let v: u32 = 0x216adbcu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5b7b; continue;
 }
 0x20b5b7b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5b7b; continue; }
  pc = 0x20b5b85; continue;
 }
 0x20b5b85 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b6182; continue;
 }
 0x20b5b8c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216ade4u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b5be3; continue; }
  pc = 0x20b5bb8; continue;
 }
 0x20b5bb8 => {
  { let v: u32 = 0x216adecu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5bc6; continue;
 }
 0x20b5bc6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5bc6; continue; }
  pc = 0x20b5bd0; continue;
 }
 0x20b5bd0 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b5bd6); f_020b3710(cpu); if cpu.returned_to != 0x20b5bd6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5be3 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5bf4); f_020ad040(cpu); if cpu.returned_to != 0x20b5bf4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x3cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x6cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x58u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x68u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b5cb3; continue; }
  pc = 0x20b5c7d; continue;
 }
 0x20b5c7d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5c96); f_020b6230(cpu); if cpu.returned_to != 0x20b5c96 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5cb3; continue; }
  pc = 0x20b5c9d; continue;
 }
 0x20b5c9d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x20b5cb3; continue;
 }
 0x20b5cb3 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b5cfc; continue; }
  pc = 0x20b5cba; continue;
 }
 0x20b5cba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b5cc3); f_020ac290(cpu); if cpu.returned_to != 0x20b5cc3 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5cfc; continue; }
  pc = 0x20b5ccc; continue;
 }
 0x20b5ccc => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x20b5ce1); f_020b6230(cpu); if cpu.returned_to != 0x20b5ce1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5cfc; continue; }
  pc = 0x20b5ce8; continue;
 }
 0x20b5ce8 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x20b5cfc; continue;
 }
 0x20b5cfc => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20b5d97; continue; }
  pc = 0x20b5d08; continue;
 }
 0x20b5d08 => {
  { cpu.sub32(cpu.r32(0x22d2004u32), 0x1u32); }
  if !cpu.zf { pc = 0x20b5d97; continue; }
  pc = 0x20b5d15; continue;
 }
 0x20b5d15 => {
  { let v: u32 = cpu.edi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b5d83; continue;
 }
 0x20b5d1f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20b5d4b; continue; }
  pc = 0x20b5d3d; continue;
 }
 0x20b5d3d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x20b5d4b; continue;
 }
 0x20b5d4b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5d52); f_020b6230(cpu); if cpu.returned_to != 0x20b5d52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5d6f; continue; }
  pc = 0x20b5d59; continue;
 }
 0x20b5d59 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x20b5d97; continue;
 }
 0x20b5d6f => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x18u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b5d83; continue;
 }
 0x20b5d83 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b5d1f; continue; }
  pc = 0x20b5d97; continue;
 }
 0x20b5d97 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b5dc7; continue; }
  pc = 0x20b5d9e; continue;
 }
 0x20b5d9e => {
  { let v: u32 = 0x216ae04u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5daa; continue;
 }
 0x20b5daa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5daa; continue; }
  pc = 0x20b5db4; continue;
 }
 0x20b5db4 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5dba); f_020b3710(cpu); if cpu.returned_to != 0x20b5dba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5dc7 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { cpu.sub32(cpu.r32(0x22d2008u32), 0x0u32); }
  if cpu.zf { pc = 0x20b5de5; continue; }
  pc = 0x20b5dde; continue;
 }
 0x20b5dde => {
  { let v: u32 = 0x2000u32; cpu.ebx = v; }
  pc = 0x20b5deb; continue;
 }
 0x20b5de5 => {
  { let v: u32 = cpu.r32(0x22d1fc8u32); cpu.ebx = v; }
  pc = 0x20b5deb; continue;
 }
 0x20b5deb => {
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x20b5e04; continue;
 }
 0x20b5e04 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b5e19); f_020b3680(cpu); if cpu.returned_to != 0x20b5e19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5e49; continue; }
  pc = 0x20b5e20; continue;
 }
 0x20b5e20 => {
  { let v: u32 = 0x216ae28u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5e2c; continue;
 }
 0x20b5e2c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5e2c; continue; }
  pc = 0x20b5e36; continue;
 }
 0x20b5e36 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5e3c); f_020b3710(cpu); if cpu.returned_to != 0x20b5e3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5e49 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b5e61; continue; }
  pc = 0x20b5e50; continue;
 }
 0x20b5e50 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5e5e); f_020b3640(cpu); if cpu.returned_to != 0x20b5e5e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20b5e61; continue;
 }
 0x20b5e61 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xf0000u32; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xf0000u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5e04; continue; }
  pc = 0x20b5e95; continue;
 }
 0x20b5e95 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5e9b); f_020b6360(cpu); if cpu.returned_to != 0x20b5e9b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5ec9; continue; }
  pc = 0x20b5ea2; continue;
 }
 0x20b5ea2 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5eb3); f_020b3640(cpu); if cpu.returned_to != 0x20b5eb3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5ebc); f_020b3710(cpu); if cpu.returned_to != 0x20b5ebc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5ec9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b5edb); f_020ad040(cpu); if cpu.returned_to != 0x20b5edb { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x44u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b5f18); f_020b3740(cpu); if cpu.returned_to != 0x20b5f18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b5f93; continue; }
  pc = 0x20b5f22; continue;
 }
 0x20b5f22 => {
  { let v: u32 = 0x216ae48u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5f2e; continue;
 }
 0x20b5f2e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5f2e; continue; }
  pc = 0x20b5f38; continue;
 }
 0x20b5f38 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b5f4d; continue; }
  pc = 0x20b5f44; continue;
 }
 0x20b5f44 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b5f4a); f_020ad190(cpu); if cpu.returned_to != 0x20b5f4a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b5f4d; continue;
 }
 0x20b5f4d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5f5f); f_020ad040(cpu); if cpu.returned_to != 0x20b5f5f { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b5f7d); f_020b3640(cpu); if cpu.returned_to != 0x20b5f7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b5f86); f_020b3710(cpu); if cpu.returned_to != 0x20b5f86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b5f93 => {
  { let v: u32 = cpu.r32(0x22d1fd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b5fa7); f_020b3740(cpu); if cpu.returned_to != 0x20b5fa7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b6032; continue; }
  pc = 0x20b5fb5; continue;
 }
 0x20b5fb5 => {
  { let v: u32 = 0x216ae6cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b5fc1; continue;
 }
 0x20b5fc1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b5fc1; continue; }
  pc = 0x20b5fcb; continue;
 }
 0x20b5fcb => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b5fe0; continue; }
  pc = 0x20b5fd7; continue;
 }
 0x20b5fd7 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b5fdd); f_020ad190(cpu); if cpu.returned_to != 0x20b5fdd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b5fe0; continue;
 }
 0x20b5fe0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b5ff2); f_020ad040(cpu); if cpu.returned_to != 0x20b5ff2 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b6010); f_020b3640(cpu); if cpu.returned_to != 0x20b6010 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b601c); f_020b3710(cpu); if cpu.returned_to != 0x20b601c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b6025); f_020b3710(cpu); if cpu.returned_to != 0x20b6025 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b6032 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6087; continue; }
  pc = 0x20b603f; continue;
 }
 0x20b603f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x216ae94u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffff75cu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b603f; continue; }
  pc = 0x20b6087; continue;
 }
 0x20b6087 => {
  { let v = 0x20b67e0u32; cpu.push(v); }
  cpu.push(0x20b6091); f_020acb50(cpu); if cpu.returned_to != 0x20b6091 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b6129; continue; }
  pc = 0x20b60a0; continue;
 }
 0x20b60a0 => {
  { let v: u32 = 0x216ae9cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b60ac; continue;
 }
 0x20b60ac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b60ac; continue; }
  pc = 0x20b60b6; continue;
 }
 0x20b60b6 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b60cb; continue; }
  pc = 0x20b60c2; continue;
 }
 0x20b60c2 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b60c8); f_020ad190(cpu); if cpu.returned_to != 0x20b60c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b60cb; continue;
 }
 0x20b60cb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b60dd); f_020ad040(cpu); if cpu.returned_to != 0x20b60dd { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b60fb); f_020b3640(cpu); if cpu.returned_to != 0x20b60fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b6107); f_020b3710(cpu); if cpu.returned_to != 0x20b6107 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b6113); f_020b3710(cpu); if cpu.returned_to != 0x20b6113 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b611c); f_020b3710(cpu); if cpu.returned_to != 0x20b611c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b6129 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b6130); f_020aca40(cpu); if cpu.returned_to != 0x20b6130 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d1fc0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b6142); f_020ac8c0(cpu); if cpu.returned_to != 0x20b6142 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b614e); f_020ac670(cpu); if cpu.returned_to != 0x20b614e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20b6190u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b6167); f_020c4566(cpu); if cpu.returned_to != 0x20b6167 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b6172); f_020c44d3(cpu); if cpu.returned_to != 0x20b6172 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b617d); f_020c44d3(cpu); if cpu.returned_to != 0x20b617d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20b6182; continue;
 }
 0x20b6182 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6190); }
 let mut pc: u32 = 0x20b6190;
 loop { match pc {
 0x20b6190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b61e6; continue; }
  pc = 0x20b619f; continue;
 }
 0x20b619f => {
  if cpu.zf { pc = 0x20b61bd; continue; }
  pc = 0x20b61a1; continue;
 }
 0x20b61a1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b61b3); f_020ad040(cpu); if cpu.returned_to != 0x20b61b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b61bd; continue;
 }
 0x20b61bd => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b61c3); f_020c4566(cpu); if cpu.returned_to != 0x20b61c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b61ce); f_020c44d3(cpu); if cpu.returned_to != 0x20b61ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b61d9); f_020c44d3(cpu); if cpu.returned_to != 0x20b61d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x20b61e3); f_020acf90(cpu); if cpu.returned_to != 0x20b61e3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b61e6; continue;
 }
 0x20b61e6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b61ef); f_020ac410(cpu); if cpu.returned_to != 0x20b61ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b6203); f_020b3640(cpu); if cpu.returned_to != 0x20b6203 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b620f); f_020b3710(cpu); if cpu.returned_to != 0x20b620f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b621b); f_020b3710(cpu); if cpu.returned_to != 0x20b621b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b6224); f_020b3710(cpu); if cpu.returned_to != 0x20b6224 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6230); }
 let mut pc: u32 = 0x20b6230;
 loop { match pc {
 0x20b6230 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x20b62d2; continue; }
  pc = 0x20b6259; continue;
 }
 0x20b6259 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b627e; continue; }
  pc = 0x20b6277; continue;
 }
 0x20b6277 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b627e; continue;
 }
 0x20b627e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b6291; continue; }
  pc = 0x20b6288; continue;
 }
 0x20b6288 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6291; continue;
 }
 0x20b6291 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b62a5; continue; }
  pc = 0x20b629c; continue;
 }
 0x20b629c => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b62a5; continue;
 }
 0x20b62a5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b62b9; continue; }
  pc = 0x20b62b0; continue;
 }
 0x20b62b0 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b62b9; continue;
 }
 0x20b62b9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b62bb; continue;
 }
 0x20b62bb => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b62ca; continue; }
  pc = 0x20b62c2; continue;
 }
 0x20b62c2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x20b62ca; continue;
 }
 0x20b62ca => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b62bb; continue; }
  pc = 0x20b62d2; continue;
 }
 0x20b62d2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x22d1fe4u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20b6309; continue; }
  pc = 0x20b62ee; continue;
 }
 0x20b62ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x20b6323; continue;
 }
 0x20b6309 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x20b6323; continue;
 }
 0x20b6323 => {
  cpu.push(0x20b6328); f_020ad040(cpu); if cpu.returned_to != 0x20b6328 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6340(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b6340); } r_020b6340(cpu, 0x20b6340); }
pub fn f_020b6350(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b6350); } r_020b6340(cpu, 0x20b6350); }
pub fn f_020b6360(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b6360); } r_020b6340(cpu, 0x20b6360); }
fn r_020b6340(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b6340 => {
  { let v: u32 = cpu.adc32(cpu.esp, cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ 0x57020b64u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[4]))); cpu.eax = v; }
  if !cpu.sf { pc = 0x20b63b2; continue; }
  pc = 0x20b634e; continue;
 }
 0x20b634e => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x20b6350; continue;
 }
 0x20b6350 => {
  cpu.unimplemented(0x20b6350, "Frstor");
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20b6360; continue;
 }
 0x20b6360 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b63f3; continue; }
  pc = 0x20b637a; continue;
 }
 0x20b637a => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b63b2; continue; }
  pc = 0x20b6383; continue;
 }
 0x20b6383 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b6385; continue;
 }
 0x20b6385 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8a4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x4u32); }
  if !cpu.zf { pc = 0x20b63a5; continue; }
  pc = 0x20b639e; continue;
 }
 0x20b639e => {
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20b63a5; continue;
 }
 0x20b63a5 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b6385; continue; }
  pc = 0x20b63b2; continue;
 }
 0x20b63b2 => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b63d4; continue; }
  pc = 0x20b63b8; continue;
 }
 0x20b63b8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b63ca); f_020ad040(cpu); if cpu.returned_to != 0x20b63ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b63d4; continue;
 }
 0x20b63d4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b63da); f_020c4566(cpu); if cpu.returned_to != 0x20b63da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b63e5); f_020c44d3(cpu); if cpu.returned_to != 0x20b63e5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b63f0); f_020c44d3(cpu); if cpu.returned_to != 0x20b63f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b63f3; continue;
 }
 0x20b63f3 => {
  { let v: u32 = cpu.r32(0x22d1fdcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d1fe0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b6499; continue; }
  pc = 0x20b640b; continue;
 }
 0x20b640b => {
  cpu.poll(); pc = cpu.r32(0x20b6340u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x20b6413 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6499; continue;
 }
 0x20b6435 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6499; continue;
 }
 0x20b6457 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6499; continue;
 }
 0x20b6479 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6499; continue;
 }
 0x20b6499 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b649e; continue;
 }
 0x20b649e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x20b64ac; continue; }
  pc = 0x20b64a7; continue;
 }
 0x20b64a7 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x20b64b4; continue;
 }
 0x20b64ac => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b649e; continue; }
  pc = 0x20b64b4; continue;
 }
 0x20b64b4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x22d1fc4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20b64dd; continue; }
  pc = 0x20b64d4; continue;
 }
 0x20b64d4 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b64ff; continue; }
  pc = 0x20b64d6; continue;
 }
 0x20b64d6 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20b651d; continue; }
  pc = 0x20b64db; continue;
 }
 0x20b64db => {
  pc = 0x20b653b; continue;
 }
 0x20b64dd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b653b; continue; }
  pc = 0x20b64e1; continue;
 }
 0x20b64e1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x20b65ca; continue;
 }
 0x20b64ff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x20b65ca; continue;
 }
 0x20b651d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x20b65ca; continue;
 }
 0x20b653b => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x7fffffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d1fc4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b65a0; continue;
 }
 0x20b65a0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b65ab; continue; }
  pc = 0x20b65a9; continue;
 }
 0x20b65a9 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b65ab; continue;
 }
 0x20b65ab => {
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b65be; continue; }
  pc = 0x20b65af; continue;
 }
 0x20b65af => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b65bc; continue; }
  pc = 0x20b65ba; continue;
 }
 0x20b65ba => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b65bc; continue;
 }
 0x20b65bc => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x20b65be; continue;
 }
 0x20b65be => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b65a0; continue; }
  pc = 0x20b65c7; continue;
 }
 0x20b65c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x20b65ca; continue;
 }
 0x20b65ca => {
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b6614; continue; }
  pc = 0x20b65d5; continue;
 }
 0x20b65d5 => {
  cpu.poll(); pc = cpu.r32(0x20b6350u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x20b65dd => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b6614; continue;
 }
 0x20b65ed => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b6614; continue;
 }
 0x20b65fd => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b660d; continue;
 }
 0x20b6606 => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b660d; continue;
 }
 0x20b660d => {
  { let v: u32 = 0x2u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b6614; continue;
 }
 0x20b6614 => {
  { let v: u32 = cpu.r32(0x22d1fccu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x22d2008u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b6699; continue; }
  pc = 0x20b6665; continue;
 }
 0x20b6665 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6672; continue; }
  pc = 0x20b666d; continue;
 }
 0x20b666d => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x20b6672; continue;
 }
 0x20b6672 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6699; continue; }
  pc = 0x20b667c; continue;
 }
 0x20b667c => {
  { let v: u32 = 0x216ad90u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b6688; continue;
 }
 0x20b6688 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b6688; continue; }
  pc = 0x20b6692; continue;
 }
 0x20b6692 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b67d7; continue;
 }
 0x20b6699 => {
  { let v: u32 = 0x7fffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x20b66c5; continue;
 }
 0x20b66a5 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b66b0; continue; }
  pc = 0x20b66ae; continue;
 }
 0x20b66ae => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b66b0; continue;
 }
 0x20b66b0 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b66c3; continue; }
  pc = 0x20b66b4; continue;
 }
 0x20b66b4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b66bf; continue; }
  pc = 0x20b66bd; continue;
 }
 0x20b66bd => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b66bf; continue;
 }
 0x20b66bf => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x20b66c3; continue;
 }
 0x20b66c3 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  pc = 0x20b66c5; continue;
 }
 0x20b66c5 => {
  { let v: u32 = cpu.r32(0x22d1fc8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b66a5; continue; }
  pc = 0x20b66d7; continue;
 }
 0x20b66d7 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x20b66e5; continue; }
  pc = 0x20b66e2; continue;
 }
 0x20b66e2 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b66e5; continue;
 }
 0x20b66e5 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x14u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { pc = 0x20b66ed; continue; }
  pc = 0x20b66ea; continue;
 }
 0x20b66ea => {
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b66ed; continue;
 }
 0x20b66ed => {
  { let v: u32 = cpu.r32(0x22d2008u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b670a; continue; }
  pc = 0x20b66fa; continue;
 }
 0x20b66fa => {
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xff000u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  pc = 0x20b670a; continue;
 }
 0x20b670a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b67d2; continue; }
  pc = 0x20b6756; continue;
 }
 0x20b6756 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b677c; continue; }
  pc = 0x20b675f; continue;
 }
 0x20b675f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b6761; continue;
 }
 0x20b6761 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8a4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b6761; continue; }
  pc = 0x20b677c; continue;
 }
 0x20b677c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b6782); f_020c4566(cpu); if cpu.returned_to != 0x20b6782 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b678d); f_020c44d3(cpu); if cpu.returned_to != 0x20b678d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b6798); f_020c44d3(cpu); if cpu.returned_to != 0x20b6798 { return; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20b67d2; continue; }
  pc = 0x20b67a2; continue;
 }
 0x20b67a2 => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x401u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b67c8); f_020ad040(cpu); if cpu.returned_to != 0x20b67c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b67d2; continue;
 }
 0x20b67d2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b67d7; continue;
 }
 0x20b67d7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b67e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b67e0); }
 let mut pc: u32 = 0x20b67e0;
 loop { match pc {
 0x20b67e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x217c790u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b69dd; continue; }
  pc = 0x20b67f6; continue;
 }
 0x20b67f6 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d1fe8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c790u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2058u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b681e; continue; }
  pc = 0x20b6814; continue;
 }
 0x20b6814 => {
  cpu.push(0x20b6819); f_020ad180(cpu); if cpu.returned_to != 0x20b6819 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2060u32, v); }
  pc = 0x20b681e; continue;
 }
 0x20b681e => {
  { let v: u32 = cpu.r32(0x22d2058u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20b685b; continue; }
  pc = 0x20b6830; continue;
 }
 0x20b6830 => {
  { cpu.sub32(cpu.r32(0x22d1fe8u32), 0x0u32); }
  if cpu.zf { pc = 0x20b6848; continue; }
  pc = 0x20b6839; continue;
 }
 0x20b6839 => {
  { let v: u32 = cpu.r32(0x22d2060u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b6845); f_020ad170(cpu); if cpu.returned_to != 0x20b6845 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b6848; continue;
 }
 0x20b6848 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c790u32, v); }
  pc = 0x20b69dd; continue;
 }
 0x20b685b => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d205cu32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b6871; continue; }
  pc = 0x20b686c; continue;
 }
 0x20b686c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20b68aa; continue; }
  pc = 0x20b6871; continue;
 }
 0x20b6871 => {
  { let v: u32 = cpu.r32(0x22d1fe8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2054u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b6890; continue; }
  pc = 0x20b6881; continue;
 }
 0x20b6881 => {
  { let v: u32 = cpu.r32(0x22d2060u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b688d); f_020ad170(cpu); if cpu.returned_to != 0x20b688d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b6890; continue;
 }
 0x20b6890 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x217c790u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2054u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x20b68aa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b68b3); f_020c4566(cpu); if cpu.returned_to != 0x20b68b3 { return; }
  { let v: u32 = cpu.r32(0x22d2058u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d2064u32, v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2054u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2050u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b6911; continue; }
  pc = 0x20b68d8; continue;
 }
 0x20b68d8 => {
  { let v: u32 = cpu.r32(0x22d2050u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x20b68ee; continue; }
  pc = 0x20b68e3; continue;
 }
 0x20b68e3 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x22d2064u32)); cpu.w32(0x22d2064u32, v); }
  cpu.push(0x20b68ee); f_020b6ce0(cpu); if cpu.returned_to != 0x20b68ee { return; }
  pc = 0x20b68ee; continue;
 }
 0x20b68ee => {
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x22d2050u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2054u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d2050u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x20b68d8; continue; }
  pc = 0x20b6911; continue;
 }
 0x20b6911 => {
  { let v: u32 = cpu.r32(0x22d2058u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2064u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d205cu32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b692f); f_020c44d3(cpu); if cpu.returned_to != 0x20b692f { return; }
  { let v: u32 = cpu.r32(0x22d2058u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x20b6971; continue; }
  pc = 0x20b693f; continue;
 }
 0x20b693f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x22d2044u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x22d205cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = 0x405u32; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x22d2048u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x22d204au32, v); }
  cpu.push(0x20b696e); f_020ad040(cpu); if cpu.returned_to != 0x20b696e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20b6971; continue;
 }
 0x20b6971 => {
  { let v: u32 = cpu.r32(0x22d2058u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b698d; continue; }
  pc = 0x20b6984; continue;
 }
 0x20b6984 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b69b7; continue;
 }
 0x20b698d => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x20b69b7; continue; }
  pc = 0x20b699b; continue;
 }
 0x20b699b => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b69b7; continue; }
  pc = 0x20b69a1; continue;
 }
 0x20b69a1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b69b1); f_020ad040(cpu); if cpu.returned_to != 0x20b69b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b69b7; continue;
 }
 0x20b69b7 => {
  { cpu.sub32(cpu.r32(0x22d1fe8u32), 0x0u32); }
  if cpu.zf { pc = 0x20b69cf; continue; }
  pc = 0x20b69c0; continue;
 }
 0x20b69c0 => {
  { let v: u32 = cpu.r32(0x22d2060u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b69cc); f_020ad170(cpu); if cpu.returned_to != 0x20b69cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b69cf; continue;
 }
 0x20b69cf => {
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x22d2054u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x217c790u32, v); }
  pc = 0x20b69dd; continue;
 }
 0x20b69dd => {
  { let v: u32 = cpu.esi; cpu.w32(0x22d2054u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b69f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b69f0); }
 let mut pc: u32 = 0x20b69f0;
 loop { match pc {
 0x20b69f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6a0c; continue; }
  pc = 0x20b6a03; continue;
 }
 0x20b6a03 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b6a17; continue;
 }
 0x20b6a0c => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20b6a17; continue; }
  pc = 0x20b6a10; continue;
 }
 0x20b6a10 => {
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b6a17; continue;
 }
 0x20b6a17 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6a28; continue; }
  pc = 0x20b6a1f; continue;
 }
 0x20b6a1f => {
  { let v: u32 = 0x7fu32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b6a33; continue;
 }
 0x20b6a28 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20b6a33; continue; }
  pc = 0x20b6a2c; continue;
 }
 0x20b6a2c => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b6a33; continue;
 }
 0x20b6a33 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x6cu32.wrapping_add(cpu.eax))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b6a62; continue; }
  pc = 0x20b6a5c; continue;
 }
 0x20b6a5c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6a62; continue;
 }
 0x20b6a62 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b6a71; continue; }
  pc = 0x20b6a69; continue;
 }
 0x20b6a69 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6a71; continue;
 }
 0x20b6a71 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x20b6a7e; continue; }
  pc = 0x20b6a79; continue;
 }
 0x20b6a79 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x20b6ac7; continue; }
  pc = 0x20b6a7e; continue;
 }
 0x20b6a7e => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x217c710u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x217c710u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x458u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b6cd7; continue;
 }
 0x20b6ac7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b6ad6; continue; }
  pc = 0x20b6acf; continue;
 }
 0x20b6acf => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b6ad6; continue;
 }
 0x20b6ad6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x20b6b04; continue; }
  pc = 0x20b6ae1; continue;
 }
 0x20b6ae1 => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x20b6b04; continue; }
  pc = 0x20b6ae6; continue;
 }
 0x20b6ae6 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b6af3; continue; }
  pc = 0x20b6aea; continue;
 }
 0x20b6aea => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x20b6c3c; continue; }
  pc = 0x20b6af3; continue;
 }
 0x20b6af3 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x20b6b04; continue; }
  pc = 0x20b6afb; continue;
 }
 0x20b6afb => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x20b6c3c; continue; }
  pc = 0x20b6b04; continue;
 }
 0x20b6b04 => {
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x458u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x217c710u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x217c710u32.wrapping_add(cpu.edi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x48u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x20b6bdf; continue; }
  pc = 0x20b6b3c; continue;
 }
 0x20b6b3c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0x0u32); cpu.ecx = v; }
  pc = 0x20b6b4d; continue;
 }
 0x20b6b4d => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b6b4d; continue; }
  pc = 0x20b6b83; continue;
 }
 0x20b6b83 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  pc = 0x20b6ba7; continue;
 }
 0x20b6ba7 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x20b6cd7; continue; }
  pc = 0x20b6bdd; continue;
 }
 0x20b6bdd => {
  cpu.poll(); pc = 0x20b6ba7; continue;
 }
 0x20b6bdf => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  pc = 0x20b6c00; continue;
 }
 0x20b6c00 => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, 0x8000u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b6cd7; continue; }
  pc = 0x20b6c3a; continue;
 }
 0x20b6c3a => {
  cpu.poll(); pc = 0x20b6c00; continue;
 }
 0x20b6c3c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b6ca7; continue; }
  pc = 0x20b6c4e; continue;
 }
 0x20b6c4e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x0u32); cpu.edx = v; }
  pc = 0x20b6c59; continue;
 }
 0x20b6c59 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b6c59; continue; }
  pc = 0x20b6c74; continue;
 }
 0x20b6c74 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x20b6c8e; continue;
 }
 0x20b6c8e => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x100u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20b6cd7; continue; }
  pc = 0x20b6ca5; continue;
 }
 0x20b6ca5 => {
  cpu.poll(); pc = 0x20b6c8e; continue;
 }
 0x20b6ca7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x20b6cbc; continue;
 }
 0x20b6cbc => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b6cbc; continue; }
  pc = 0x20b6cd7; continue;
 }
 0x20b6cd7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6ce0); }
 let mut pc: u32 = 0x20b6ce0;
 loop { match pc {
 0x20b6ce0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x20b6df6; continue; }
  pc = 0x20b6cf2; continue;
 }
 0x20b6cf2 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x20b6cf4; continue;
 }
 0x20b6cf4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x20b6df6; continue; }
  pc = 0x20b6d00; continue;
 }
 0x20b6d00 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20b6d2e; continue; }
  pc = 0x20b6d18; continue;
 }
 0x20b6d18 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b6d2e; continue; }
  pc = 0x20b6d20; continue;
 }
 0x20b6d20 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.eax))); }
  if !cpu.cf { pc = 0x20b6d2e; continue; }
  pc = 0x20b6d25; continue;
 }
 0x20b6d25 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b6d2e; continue;
 }
 0x20b6d2e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b6dd4; continue; }
  pc = 0x20b6d36; continue;
 }
 0x20b6d36 => {
  { cpu.sub32(cpu.r32(0x860u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b6d46; continue; }
  pc = 0x20b6d3f; continue;
 }
 0x20b6d3f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x20b6d46); cpu.call(t); if cpu.returned_to != 0x20b6d46 { return; }
  pc = 0x20b6d46; continue;
 }
 0x20b6d46 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x20b6d6a; continue; }
  pc = 0x20b6d4c; continue;
 }
 0x20b6d4c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x20b6cf4; continue; }
  pc = 0x20b6d62; continue;
 }
 0x20b6d62 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x20b6cf4; continue;
 }
 0x20b6d6a => {
  { cpu.sub32(cpu.r32(0x858u32.wrapping_add(cpu.esi)), 0x2u32); }
  if !cpu.zf { pc = 0x20b6dae; continue; }
  pc = 0x20b6d73; continue;
 }
 0x20b6d73 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20b6dae; continue; }
  pc = 0x20b6d88; continue;
 }
 0x20b6d88 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20b6d97; continue; }
  pc = 0x20b6d8e; continue;
 }
 0x20b6d8e => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b6df6; continue;
 }
 0x20b6d97 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x20b6da6; continue; }
  pc = 0x20b6d9d; continue;
 }
 0x20b6d9d => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b6df6; continue;
 }
 0x20b6da6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x20b6cf4; continue;
 }
 0x20b6dae => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b6df6; continue; }
  pc = 0x20b6dc6; continue;
 }
 0x20b6dc6 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x20b6dcd); cpu.call(t); if cpu.returned_to != 0x20b6dcd { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b6dd4 => {
  { cpu.sub32(cpu.r32(0x85cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b6de4; continue; }
  pc = 0x20b6ddd; continue;
 }
 0x20b6ddd => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x85cu32.wrapping_add(cpu.esi)); cpu.push(0x20b6de4); cpu.call(t); if cpu.returned_to != 0x20b6de4 { return; }
  pc = 0x20b6de4; continue;
 }
 0x20b6de4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b6dec); f_020c4370(cpu); if cpu.returned_to != 0x20b6dec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.poll(); pc = 0x20b6cf4; continue;
 }
 0x20b6df6 => {
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
pub fn f_020b6e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6e00); }
 let mut pc: u32 = 0x20b6e00;
 loop { match pc {
 0x20b6e00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b6e27; continue; }
  pc = 0x20b6e0b; continue;
 }
 0x20b6e0b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b6e1d); f_020ad040(cpu); if cpu.returned_to != 0x20b6e1d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b6e27; continue;
 }
 0x20b6e27 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6e80); }
 let mut pc: u32 = 0x20b6e80;
 loop { match pc {
 0x20b6e80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c794u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b6f3c; continue; }
  pc = 0x20b6e90; continue;
 }
 0x20b6e90 => {
  { let v = 0x20b5100u32; cpu.push(v); }
  { let v = 0x20b6e80u32; cpu.push(v); }
  cpu.push(0x20b6e9f); f_020b35a0(cpu); if cpu.returned_to != 0x20b6e9f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x217c710u32; cpu.push(v); }
  cpu.push(0x20b6eb1); f_020b34e0(cpu); if cpu.returned_to != 0x20b6eb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c790u32; cpu.push(v); }
  cpu.push(0x20b6ec0); f_020b34e0(cpu); if cpu.returned_to != 0x20b6ec0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2058u32; cpu.push(v); }
  cpu.push(0x20b6ecf); f_020b34e0(cpu); if cpu.returned_to != 0x20b6ecf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x22d2044u32; cpu.push(v); }
  cpu.push(0x20b6ede); f_020b34e0(cpu); if cpu.returned_to != 0x20b6ede { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2060u32; cpu.push(v); }
  cpu.push(0x20b6eed); f_020b34e0(cpu); if cpu.returned_to != 0x20b6eed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2064u32; cpu.push(v); }
  cpu.push(0x20b6efc); f_020b34e0(cpu); if cpu.returned_to != 0x20b6efc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2054u32; cpu.push(v); }
  cpu.push(0x20b6f0b); f_020b34e0(cpu); if cpu.returned_to != 0x20b6f0b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d205cu32; cpu.push(v); }
  cpu.push(0x20b6f1a); f_020b34e0(cpu); if cpu.returned_to != 0x20b6f1a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2050u32; cpu.push(v); }
  cpu.push(0x20b6f29); f_020b34e0(cpu); if cpu.returned_to != 0x20b6f29 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20b6f36); f_020c4592(cpu); if cpu.returned_to != 0x20b6f36 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c794u32, v); }
  pc = 0x20b6f3c; continue;
 }
 0x20b6f3c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6f40); }
 let mut pc: u32 = 0x20b6f40;
 loop { match pc {
 0x20b6f40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c798u32), 0x0u32); }
  if cpu.zf { pc = 0x20b6f66; continue; }
  pc = 0x20b6f4c; continue;
 }
 0x20b6f4c => {
  { let v = 0x20b6f40u32; cpu.push(v); }
  { let v = 0x20b7720u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20b6f5d); f_020b3500(cpu); if cpu.returned_to != 0x20b6f5d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c798u32, v); }
  pc = 0x20b6f66; continue;
 }
 0x20b6f66 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6f70); }
 let mut pc: u32 = 0x20b6f70;
 loop { match pc {
 0x20b6f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b6f8b; continue; }
  pc = 0x20b6f79; continue;
 }
 0x20b6f79 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x888u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x888u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20b6f8b; continue;
 }
 0x20b6f8b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b6f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b6f90); }
 let mut pc: u32 = 0x20b6f90;
 loop { match pc {
 0x20b6f90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b6fa5; continue; }
  pc = 0x20b6f9f; continue;
 }
 0x20b6f9f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b6fa5 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x216af1cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b6fb1; continue;
 }
 0x20b6fb1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b6fbf; continue; }
  pc = 0x20b6fb9; continue;
 }
 0x20b6fb9 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b6fbf; continue; }
  pc = 0x20b6fbd; continue;
 }
 0x20b6fbd => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b6fbf; continue;
 }
 0x20b6fbf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b6fcc; continue; }
  pc = 0x20b6fc4; continue;
 }
 0x20b6fc4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b6fcc; continue; }
  pc = 0x20b6fc9; continue;
 }
 0x20b6fc9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b6fcc; continue;
 }
 0x20b6fcc => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b6fdb; continue; }
  pc = 0x20b6fd0; continue;
 }
 0x20b6fd0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b6fdb; continue; }
  pc = 0x20b6fd4; continue;
 }
 0x20b6fd4 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b6fdb; continue; }
  pc = 0x20b6fd7; continue;
 }
 0x20b6fd7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b6fb1; continue;
 }
 0x20b6fdb => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b6fe2; continue; }
  pc = 0x20b6fe0; continue;
 }
 0x20b6fe0 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b6fe2; continue;
 }
 0x20b6fe2 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b6fea; continue; }
  pc = 0x20b6fe6; continue;
 }
 0x20b6fe6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b704d; continue;
 }
 0x20b6fea => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216af28u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x20b6ff7; continue;
 }
 0x20b6ff7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7005; continue; }
  pc = 0x20b6fff; continue;
 }
 0x20b6fff => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7005; continue; }
  pc = 0x20b7003; continue;
 }
 0x20b7003 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7005; continue;
 }
 0x20b7005 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7012; continue; }
  pc = 0x20b700a; continue;
 }
 0x20b700a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7012; continue; }
  pc = 0x20b700f; continue;
 }
 0x20b700f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b7012; continue;
 }
 0x20b7012 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b7021; continue; }
  pc = 0x20b7016; continue;
 }
 0x20b7016 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b7021; continue; }
  pc = 0x20b701a; continue;
 }
 0x20b701a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b7021; continue; }
  pc = 0x20b701d; continue;
 }
 0x20b701d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b6ff7; continue;
 }
 0x20b7021 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b7028; continue; }
  pc = 0x20b7026; continue;
 }
 0x20b7026 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7028; continue;
 }
 0x20b7028 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20b7048; continue; }
  pc = 0x20b702c; continue;
 }
 0x20b702c => {
  { let v: u32 = 0x216af78u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b7038; continue;
 }
 0x20b7038 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7038; continue; }
  pc = 0x20b7042; continue;
 }
 0x20b7042 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7048 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b704d; continue;
 }
 0x20b704d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x20b7058; continue; }
  pc = 0x20b7051; continue;
 }
 0x20b7051 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20b7095; continue; }
  pc = 0x20b7056; continue;
 }
 0x20b7056 => {
  pc = 0x20b70a9; continue;
 }
 0x20b7058 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x898u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b7093); f_020b7458(cpu); if cpu.returned_to != 0x20b7093 { return; }
  pc = 0x20b70a6; continue;
 }
 0x20b7095 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b70a6); f_020b72d0(cpu); if cpu.returned_to != 0x20b70a6 { return; }
  pc = 0x20b70a6; continue;
 }
 0x20b70a6 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b70a9; continue;
 }
 0x20b70a9 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b70ce; continue; }
  pc = 0x20b70b2; continue;
 }
 0x20b70b2 => {
  { let v: u32 = 0x216afa0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b70be; continue;
 }
 0x20b70be => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b70be; continue; }
  pc = 0x20b70c8; continue;
 }
 0x20b70c8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b70ce => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b70e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b70e0); }
 let mut pc: u32 = 0x20b70e0;
 loop { match pc {
 0x20b70e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x217c798u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b710e; continue; }
  pc = 0x20b70f1; continue;
 }
 0x20b70f1 => {
  { let v = 0x20b6f40u32; cpu.push(v); }
  { let v = 0x20b7720u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20b7105); f_020b35a0(cpu); if cpu.returned_to != 0x20b7105 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c798u32, v); }
  pc = 0x20b710e; continue;
 }
 0x20b710e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20b7119; continue; }
  pc = 0x20b7112; continue;
 }
 0x20b7112 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7119 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x216af1cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x20b7125; continue;
 }
 0x20b7125 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7133; continue; }
  pc = 0x20b712d; continue;
 }
 0x20b712d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7133; continue; }
  pc = 0x20b7131; continue;
 }
 0x20b7131 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7133; continue;
 }
 0x20b7133 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7140; continue; }
  pc = 0x20b7138; continue;
 }
 0x20b7138 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7140; continue; }
  pc = 0x20b713d; continue;
 }
 0x20b713d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b7140; continue;
 }
 0x20b7140 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b714f; continue; }
  pc = 0x20b7144; continue;
 }
 0x20b7144 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b714f; continue; }
  pc = 0x20b7148; continue;
 }
 0x20b7148 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b714f; continue; }
  pc = 0x20b714b; continue;
 }
 0x20b714b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b7125; continue;
 }
 0x20b714f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b7156; continue; }
  pc = 0x20b7154; continue;
 }
 0x20b7154 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7156; continue;
 }
 0x20b7156 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b715e; continue; }
  pc = 0x20b715a; continue;
 }
 0x20b715a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b71c2; continue;
 }
 0x20b715e => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216af28u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  pc = 0x20b716b; continue;
 }
 0x20b716b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7179; continue; }
  pc = 0x20b7173; continue;
 }
 0x20b7173 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7179; continue; }
  pc = 0x20b7177; continue;
 }
 0x20b7177 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7179; continue;
 }
 0x20b7179 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7186; continue; }
  pc = 0x20b717e; continue;
 }
 0x20b717e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7186; continue; }
  pc = 0x20b7183; continue;
 }
 0x20b7183 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b7186; continue;
 }
 0x20b7186 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b7195; continue; }
  pc = 0x20b718a; continue;
 }
 0x20b718a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b7195; continue; }
  pc = 0x20b718e; continue;
 }
 0x20b718e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b7195; continue; }
  pc = 0x20b7191; continue;
 }
 0x20b7191 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b716b; continue;
 }
 0x20b7195 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b719c; continue; }
  pc = 0x20b719a; continue;
 }
 0x20b719a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b719c; continue;
 }
 0x20b719c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20b71bd; continue; }
  pc = 0x20b71a0; continue;
 }
 0x20b71a0 => {
  { let v: u32 = 0x216af30u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b71ac; continue;
 }
 0x20b71ac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b71ac; continue; }
  pc = 0x20b71b6; continue;
 }
 0x20b71b6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b71bd => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x20b71c2; continue;
 }
 0x20b71c2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b71cc); f_020abd10(cpu); if cpu.returned_to != 0x20b71cc { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b728a; continue; }
  pc = 0x20b71db; continue;
 }
 0x20b71db => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b71e1); f_020aba90(cpu); if cpu.returned_to != 0x20b71e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x888u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x20b71f9; continue; }
  pc = 0x20b71f2; continue;
 }
 0x20b71f2 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x20b7244; continue; }
  pc = 0x20b71f7; continue;
 }
 0x20b71f7 => {
  pc = 0x20b7262; continue;
 }
 0x20b71f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x898u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x20b7410u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b7237); f_020a9ec0(cpu); if cpu.returned_to != 0x20b7237 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b7242); f_020b7458(cpu); if cpu.returned_to != 0x20b7242 { return; }
  pc = 0x20b725f; continue;
 }
 0x20b7244 => {
  { let v = 0x20b7290u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b7255); f_020a9ec0(cpu); if cpu.returned_to != 0x20b7255 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b725f); f_020b72d0(cpu); if cpu.returned_to != 0x20b725f { return; }
  pc = 0x20b725f; continue;
 }
 0x20b725f => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b7262; continue;
 }
 0x20b7262 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b7288; continue; }
  pc = 0x20b726b; continue;
 }
 0x20b726b => {
  { let v: u32 = 0x216af58u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b7277; continue;
 }
 0x20b7277 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7277; continue; }
  pc = 0x20b7281; continue;
 }
 0x20b7281 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7288 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b728a; continue;
 }
 0x20b728a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7290); }
 let mut pc: u32 = 0x20b7290;
 loop { match pc {
 0x20b7290 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b72a5; continue; }
  pc = 0x20b729e; continue;
 }
 0x20b729e => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x20b72a5); cpu.call(t); if cpu.returned_to != 0x20b72a5 { return; }
  pc = 0x20b72a5; continue;
 }
 0x20b72a5 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b72b7; continue; }
  pc = 0x20b72ae; continue;
 }
 0x20b72ae => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b72b4); f_020abb40(cpu); if cpu.returned_to != 0x20b72b4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b72b7; continue;
 }
 0x20b72b7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b72d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b72d0); }
 let mut pc: u32 = 0x20b72d0;
 loop { match pc {
 0x20b72d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x216af0cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x20b72e4; continue;
 }
 0x20b72e4 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b72ed; continue;
 }
 0x20b72ed => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b72fb; continue; }
  pc = 0x20b72f5; continue;
 }
 0x20b72f5 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b72fb; continue; }
  pc = 0x20b72f9; continue;
 }
 0x20b72f9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b72fb; continue;
 }
 0x20b72fb => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b7308; continue; }
  pc = 0x20b7300; continue;
 }
 0x20b7300 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b7308; continue; }
  pc = 0x20b7305; continue;
 }
 0x20b7305 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b7308; continue;
 }
 0x20b7308 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b7317; continue; }
  pc = 0x20b730c; continue;
 }
 0x20b730c => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b7317; continue; }
  pc = 0x20b7310; continue;
 }
 0x20b7310 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b7317; continue; }
  pc = 0x20b7313; continue;
 }
 0x20b7313 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b72ed; continue;
 }
 0x20b7317 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b731e; continue; }
  pc = 0x20b731c; continue;
 }
 0x20b731c => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b731e; continue;
 }
 0x20b731e => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20b7334; continue; }
  pc = 0x20b7322; continue;
 }
 0x20b7322 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x20b72e4; continue;
 }
 0x20b7334 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x20b7348; continue; }
  pc = 0x20b733b; continue;
 }
 0x20b733b => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x20b7348; continue; }
  pc = 0x20b7342; continue;
 }
 0x20b7342 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x20b7382; continue;
 }
 0x20b7348 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x20b735c; continue; }
  pc = 0x20b734f; continue;
 }
 0x20b734f => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x20b735c; continue; }
  pc = 0x20b7356; continue;
 }
 0x20b7356 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x20b7382; continue;
 }
 0x20b735c => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x20b7370; continue; }
  pc = 0x20b7363; continue;
 }
 0x20b7363 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x20b7370; continue; }
  pc = 0x20b736a; continue;
 }
 0x20b736a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x20b7382; continue;
 }
 0x20b7370 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x20b738b; continue; }
  pc = 0x20b7377; continue;
 }
 0x20b7377 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x20b738b; continue; }
  pc = 0x20b737e; continue;
 }
 0x20b737e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x20b7382; continue;
 }
 0x20b7382 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b7388); f_020ab7d0(cpu); if cpu.returned_to != 0x20b7388 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20b738b; continue;
 }
 0x20b738b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b7395); f_020ab450(cpu); if cpu.returned_to != 0x20b7395 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x20b739f; continue;
 }
 0x20b739f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216af14u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b73ab; continue;
 }
 0x20b73ab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b73b9; continue; }
  pc = 0x20b73b3; continue;
 }
 0x20b73b3 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b73b9; continue; }
  pc = 0x20b73b7; continue;
 }
 0x20b73b7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b73b9; continue;
 }
 0x20b73b9 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b73c6; continue; }
  pc = 0x20b73be; continue;
 }
 0x20b73be => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b73c6; continue; }
  pc = 0x20b73c3; continue;
 }
 0x20b73c3 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b73c6; continue;
 }
 0x20b73c6 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b73d5; continue; }
  pc = 0x20b73ca; continue;
 }
 0x20b73ca => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b73d5; continue; }
  pc = 0x20b73ce; continue;
 }
 0x20b73ce => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20b73d5; continue; }
  pc = 0x20b73d1; continue;
 }
 0x20b73d1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b73ab; continue;
 }
 0x20b73d5 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b73dc; continue; }
  pc = 0x20b73da; continue;
 }
 0x20b73da => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b73dc; continue;
 }
 0x20b73dc => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20b73f2; continue; }
  pc = 0x20b73e0; continue;
 }
 0x20b73e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20b739f; continue;
 }
 0x20b73f2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b7400); f_020ab8a0(cpu); if cpu.returned_to != 0x20b7400 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7410); }
 let mut pc: u32 = 0x20b7410;
 loop { match pc {
 0x20b7410 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b741c); f_020b7458(cpu); if cpu.returned_to != 0x20b741c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7430(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b7430); } r_020b7430(cpu, 0x20b7430); }
pub fn f_020b7458(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b7458); } r_020b7430(cpu, 0x20b7458); }
fn r_020b7430(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b7430 => {
  { let v: u32 = cpu.xor32(cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1))), 0xffffff91u32); cpu.w32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  if cpu.zf { pc = 0x20b7442; continue; }
  pc = 0x20b7437; continue;
 }
 0x20b7437 => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), cpu.r8(0xa7020b76u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if (cpu.cf || cpu.zf) { pc = 0x20b744a; continue; }
  pc = 0x20b743f; continue;
 }
 0x20b743f => {
  { let v: u8 = cpu.add8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x20b744e; continue; }
  pc = 0x20b7443; continue;
 }
 0x20b7442 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.cmps(4, 0); }
  if (cpu.cf || cpu.zf) { pc = 0x20b7452; continue; }
  pc = 0x20b7447; continue;
 }
 0x20b7443 => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), cpu.r8(0x31020b76u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x20b7456; continue; }
  pc = 0x20b744b; continue;
 }
 0x20b7447 => {
  { let v: u8 = cpu.add8(((cpu.edx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.zf { pc = 0x20b7456; continue; }
  pc = 0x20b744b; continue;
 }
 0x20b744a => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20b745a; continue; }
  pc = 0x20b744f; continue;
 }
 0x20b744b => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x75u32.wrapping_add(cpu.eax))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20b744e; continue;
 }
 0x20b744e => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  if cpu.cf { pc = 0x20b74c7; continue; }
  pc = 0x20b7452; continue;
 }
 0x20b744f => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x75u32.wrapping_add(cpu.edx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b7452; continue;
 }
 0x20b7452 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x76u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b7456; continue;
 }
 0x20b7456 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x20b7458; continue;
 }
 0x20b7458 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x20b745a; continue;
 }
 0x20b745a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b7466; continue;
 }
 0x20b7466 => {
  { let v: u32 = cpu.r32(0x88cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b76a7; continue; }
  pc = 0x20b7476; continue;
 }
 0x20b7476 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x20b7430u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x20b7483 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b7489); f_020b76d0(cpu); if cpu.returned_to != 0x20b7489 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7491 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b76a7; continue; }
  pc = 0x20b749e; continue;
 }
 0x20b749e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b74a4); f_020b7710(cpu); if cpu.returned_to != 0x20b74a4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b74b5); f_020ab8a0(cpu); if cpu.returned_to != 0x20b74b5 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v = cpu.edi; cpu.div32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b74d4); f_020ab450(cpu); if cpu.returned_to != 0x20b74d4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b74e1); f_020ab7d0(cpu); if cpu.returned_to != 0x20b74e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b76a2; continue; }
  pc = 0x20b74ec; continue;
 }
 0x20b74c7 => {
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.r32(0x53500cc4u32.wrapping_add(cpu.ebx)), 0xff3f7ce8u32); }
  { let v: u32 = cpu.inc32(cpu.r32(0x6a08c4u32.wrapping_add(cpu.ebx))); cpu.w32(0x6a08c4u32.wrapping_add(cpu.ebx), v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b74e1); f_020ab7d0(cpu); if cpu.returned_to != 0x20b74e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b76a2; continue; }
  pc = 0x20b74ec; continue;
 }
 0x20b74ec => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b74f2); f_020ab720(cpu); if cpu.returned_to != 0x20b74f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b76a2; continue;
 }
 0x20b74fa => {
  { let v: u32 = cpu.r32(0x898u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b76a7; continue; }
  pc = 0x20b7509; continue;
 }
 0x20b7509 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x20b7522; continue; }
  pc = 0x20b7513; continue;
 }
 0x20b7513 => {
  { let v: u32 = 0x1u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b76a7; continue;
 }
 0x20b7522 => {
  { let v: u32 = 0x0u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b76a7; continue;
 }
 0x20b7531 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x890u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b76a7; continue;
 }
 0x20b7548 => {
  { let v: u32 = cpu.r32(0x894u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffu32); }
  if cpu.zf { pc = 0x20b7567; continue; }
  pc = 0x20b7556; continue;
 }
 0x20b7556 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b76a7; continue; }
  pc = 0x20b7567; continue;
 }
 0x20b7567 => {
  { let v: u32 = cpu.r32(0x890u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x20b76a7; continue;
 }
 0x20b7572 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b76a7; continue; }
  pc = 0x20b757f; continue;
 }
 0x20b757f => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x20b75a5; continue; }
  pc = 0x20b7585; continue;
 }
 0x20b7585 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b758f); f_020ab7d0(cpu); if cpu.returned_to != 0x20b758f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7a12000u32; cpu.eax = v; }
  pc = 0x20b75c3; continue;
 }
 0x20b75a5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b75af); f_020ab7d0(cpu); if cpu.returned_to != 0x20b75af { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xf424000u32; cpu.eax = v; }
  pc = 0x20b75c3; continue;
 }
 0x20b75c3 => {
  { let v = cpu.edi; cpu.div32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b75cf); f_020ab450(cpu); if cpu.returned_to != 0x20b75cf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b75e2); f_020b7710(cpu); if cpu.returned_to != 0x20b75e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b75f3); f_020ab8a0(cpu); if cpu.returned_to != 0x20b75f3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b76a2; continue; }
  pc = 0x20b75fe; continue;
 }
 0x20b75fe => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b7604); f_020ab720(cpu); if cpu.returned_to != 0x20b7604 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b76a2; continue;
 }
 0x20b760c => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20b76a7; continue; }
  pc = 0x20b7619; continue;
 }
 0x20b7619 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b761f); f_020b7710(cpu); if cpu.returned_to != 0x20b761f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b7630); f_020ab8a0(cpu); if cpu.returned_to != 0x20b7630 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b763d); f_020ab450(cpu); if cpu.returned_to != 0x20b763d { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x20b7655; continue; }
  pc = 0x20b7648; continue;
 }
 0x20b7648 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x20b7655; continue; }
  pc = 0x20b764f; continue;
 }
 0x20b764f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x20b768c; continue;
 }
 0x20b7655 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x20b7668; continue; }
  pc = 0x20b765b; continue;
 }
 0x20b765b => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x20b7668; continue; }
  pc = 0x20b7662; continue;
 }
 0x20b7662 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x20b768c; continue;
 }
 0x20b7668 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x20b767b; continue; }
  pc = 0x20b766e; continue;
 }
 0x20b766e => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x20b767b; continue; }
  pc = 0x20b7675; continue;
 }
 0x20b7675 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x20b768c; continue;
 }
 0x20b767b => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x20b7695; continue; }
  pc = 0x20b7681; continue;
 }
 0x20b7681 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x20b7695; continue; }
  pc = 0x20b7688; continue;
 }
 0x20b7688 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x20b768c; continue;
 }
 0x20b768c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b7692); f_020ab7d0(cpu); if cpu.returned_to != 0x20b7692 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20b7695; continue;
 }
 0x20b7695 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b76a2; continue; }
  pc = 0x20b7699; continue;
 }
 0x20b7699 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b769f); f_020ab720(cpu); if cpu.returned_to != 0x20b769f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b76a2; continue;
 }
 0x20b76a2 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x20b76a7; continue;
 }
 0x20b76a7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b76ad); f_020b7710(cpu); if cpu.returned_to != 0x20b76ad { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x20b7466; continue; }
  pc = 0x20b76c3; continue;
 }
 0x20b76c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b76d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b76d0); }
 let mut pc: u32 = 0x20b76d0;
 loop { match pc {
 0x20b76d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20b76e6; continue; }
  pc = 0x20b76df; continue;
 }
 0x20b76df => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x20b76e6); cpu.call(t); if cpu.returned_to != 0x20b76e6 { return; }
  pc = 0x20b76e6; continue;
 }
 0x20b76e6 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b76f8; continue; }
  pc = 0x20b76ef; continue;
 }
 0x20b76ef => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b76f5); f_020abb40(cpu); if cpu.returned_to != 0x20b76f5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b76f8; continue;
 }
 0x20b76f8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7710); }
 let mut pc: u32 = 0x20b7710;
 loop { match pc {
 0x20b7710 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7720); }
 let mut pc: u32 = 0x20b7720;
 loop { match pc {
 0x20b7720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c798u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b7749; continue; }
  pc = 0x20b772c; continue;
 }
 0x20b772c => {
  { let v = 0x20b6f40u32; cpu.push(v); }
  { let v = 0x20b7720u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20b7740); f_020b35a0(cpu); if cpu.returned_to != 0x20b7740 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c798u32, v); }
  pc = 0x20b7749; continue;
 }
 0x20b7749 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7750); }
 let mut pc: u32 = 0x20b7750;
 loop { match pc {
 0x20b7750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c820u32), 0x0u32); }
  if cpu.zf { pc = 0x20b784f; continue; }
  pc = 0x20b7760; continue;
 }
 0x20b7760 => {
  { let v = 0x20b7750u32; cpu.push(v); }
  { let v = 0x20baf30u32; cpu.push(v); }
  cpu.push(0x20b776f); f_020b3500(cpu); if cpu.returned_to != 0x20b776f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x217c79cu32; cpu.push(v); }
  cpu.push(0x20b7781); f_020b34c0(cpu); if cpu.returned_to != 0x20b7781 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c81cu32; cpu.push(v); }
  cpu.push(0x20b7790); f_020b34c0(cpu); if cpu.returned_to != 0x20b7790 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d207cu32; cpu.push(v); }
  cpu.push(0x20b779f); f_020b34c0(cpu); if cpu.returned_to != 0x20b779f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2080u32; cpu.push(v); }
  cpu.push(0x20b77ae); f_020b34c0(cpu); if cpu.returned_to != 0x20b77ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2088u32; cpu.push(v); }
  cpu.push(0x20b77bd); f_020b34c0(cpu); if cpu.returned_to != 0x20b77bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2098u32; cpu.push(v); }
  cpu.push(0x20b77cc); f_020b34c0(cpu); if cpu.returned_to != 0x20b77cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d208cu32; cpu.push(v); }
  cpu.push(0x20b77db); f_020b34c0(cpu); if cpu.returned_to != 0x20b77db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2090u32; cpu.push(v); }
  cpu.push(0x20b77ea); f_020b34c0(cpu); if cpu.returned_to != 0x20b77ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2078u32; cpu.push(v); }
  cpu.push(0x20b77f9); f_020b34c0(cpu); if cpu.returned_to != 0x20b77f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2094u32; cpu.push(v); }
  cpu.push(0x20b7808); f_020b34c0(cpu); if cpu.returned_to != 0x20b7808 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2074u32; cpu.push(v); }
  cpu.push(0x20b7817); f_020b34c0(cpu); if cpu.returned_to != 0x20b7817 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2070u32; cpu.push(v); }
  cpu.push(0x20b7826); f_020b34c0(cpu); if cpu.returned_to != 0x20b7826 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2068u32; cpu.push(v); }
  cpu.push(0x20b7835); f_020b34c0(cpu); if cpu.returned_to != 0x20b7835 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d206cu32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20b7846); f_020b34c0(cpu); if cpu.returned_to != 0x20b7846 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c820u32, v); }
  pc = 0x20b784f; continue;
 }
 0x20b784f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7860); }
 let mut pc: u32 = 0x20b7860;
 loop { match pc {
 0x20b7860 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b7884); f_020bac00(cpu); if cpu.returned_to != 0x20b7884 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x20b7898); f_020bae80(cpu); if cpu.returned_to != 0x20b7898 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20b78aa; continue; }
  pc = 0x20b78a3; continue;
 }
 0x20b78a3 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x20b78ac; continue;
 }
 0x20b78aa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x20b78ac; continue;
 }
 0x20b78ac => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b78de); f_020bae80(cpu); if cpu.returned_to != 0x20b78de { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7a10); }
 let mut pc: u32 = 0x20b7a10;
 loop { match pc {
 0x20b7a10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7a41; continue; }
  pc = 0x20b7a1d; continue;
 }
 0x20b7a1d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x20b7a41; continue; }
  pc = 0x20b7a39; continue;
 }
 0x20b7a39 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x20b7a41; continue; }
  pc = 0x20b7a3e; continue;
 }
 0x20b7a3e => {
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b7a41; continue;
 }
 0x20b7a41 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7a50); }
 let mut pc: u32 = 0x20b7a50;
 loop { match pc {
 0x20b7a50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x20b7b5a; continue; }
  pc = 0x20b7a75; continue;
 }
 0x20b7a75 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xe4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x20b7a9e); f_020bad00(cpu); if cpu.returned_to != 0x20b7a9e { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7b28; continue; }
  pc = 0x20b7ab5; continue;
 }
 0x20b7ab5 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b7abe; continue;
 }
 0x20b7abe => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x20b7b09; continue; }
  pc = 0x20b7ac4; continue;
 }
 0x20b7ac4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20b7ac9; continue;
 }
 0x20b7ac9 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b7b02; continue; }
  pc = 0x20b7ad4; continue;
 }
 0x20b7ad4 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x20b7b02; continue; }
  pc = 0x20b7adf; continue;
 }
 0x20b7adf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b7af5); f_020ba250(cpu); if cpu.returned_to != 0x20b7af5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b7b02; continue;
 }
 0x20b7b02 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7ac9; continue; }
  pc = 0x20b7b09; continue;
 }
 0x20b7b09 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7abe; continue; }
  pc = 0x20b7b28; continue;
 }
 0x20b7b28 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b7b53; continue; }
  pc = 0x20b7b3f; continue;
 }
 0x20b7b3f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x20b7b53; continue; }
  pc = 0x20b7b49; continue;
 }
 0x20b7b49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b7b50); f_020b9eb0(cpu); if cpu.returned_to != 0x20b7b50 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b7b53; continue;
 }
 0x20b7b53 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b7b5a; continue;
 }
 0x20b7b5a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7b70); }
 let mut pc: u32 = 0x20b7b70;
 loop { match pc {
 0x20b7b70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x20b7b99; continue;
 }
 0x20b7b99 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x20b7bb8; continue; }
  pc = 0x20b7b9e; continue;
 }
 0x20b7b9e => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.zf { pc = 0x20b7bb8; continue; }
  pc = 0x20b7ba5; continue;
 }
 0x20b7ba5 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x20b7bb8; continue; }
  pc = 0x20b7baa; continue;
 }
 0x20b7baa => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b7bb8; continue; }
  pc = 0x20b7bb4; continue;
 }
 0x20b7bb4 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b7bb8; continue;
 }
 0x20b7bb8 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b7b99; continue; }
  pc = 0x20b7bc1; continue;
 }
 0x20b7bc1 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b7bf8; continue; }
  pc = 0x20b7bc6; continue;
 }
 0x20b7bc6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x20b7bd7; continue;
 }
 0x20b7bd7 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x20b7bef; continue; }
  pc = 0x20b7bdc; continue;
 }
 0x20b7bdc => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x24u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b7bef; continue; }
  pc = 0x20b7be1; continue;
 }
 0x20b7be1 => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b7bef; continue; }
  pc = 0x20b7beb; continue;
 }
 0x20b7beb => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x20b7bef; continue;
 }
 0x20b7bef => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b7bd7; continue; }
  pc = 0x20b7bf8; continue;
 }
 0x20b7bf8 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b7c0f; continue; }
  pc = 0x20b7bfd; continue;
 }
 0x20b7bfd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b7ce0; continue;
 }
 0x20b7c0f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b7c22); f_020bad00(cpu); if cpu.returned_to != 0x20b7c22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7ca6; continue; }
  pc = 0x20b7c37; continue;
 }
 0x20b7c37 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b7c40; continue;
 }
 0x20b7c40 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x20b7c87; continue; }
  pc = 0x20b7c46; continue;
 }
 0x20b7c46 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20b7c4b; continue;
 }
 0x20b7c4b => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b7c80; continue; }
  pc = 0x20b7c56; continue;
 }
 0x20b7c56 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x20b7c80; continue; }
  pc = 0x20b7c5f; continue;
 }
 0x20b7c5f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b7c73); f_020ba250(cpu); if cpu.returned_to != 0x20b7c73 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b7c80; continue;
 }
 0x20b7c80 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7c4b; continue; }
  pc = 0x20b7c87; continue;
 }
 0x20b7c87 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7c40; continue; }
  pc = 0x20b7ca6; continue;
 }
 0x20b7ca6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x124u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xe4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xa4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b7ce0; continue;
 }
 0x20b7ce0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7cf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7cf0); }
 let mut pc: u32 = 0x20b7cf0;
 loop { match pc {
 0x20b7cf0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7cff; continue; }
  pc = 0x20b7cf8; continue;
 }
 0x20b7cf8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b7cff; continue;
 }
 0x20b7cff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7d00); }
 let mut pc: u32 = 0x20b7d00;
 loop { match pc {
 0x20b7d00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b7d0b; continue; }
  pc = 0x20b7d08; continue;
 }
 0x20b7d08 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7d0b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6dcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7d20); }
 let mut pc: u32 = 0x20b7d20;
 loop { match pc {
 0x20b7d20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b7d3b; continue; }
  pc = 0x20b7d28; continue;
 }
 0x20b7d28 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6dcu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b7d3b; continue;
 }
 0x20b7d3b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7d40); }
 let mut pc: u32 = 0x20b7d40;
 loop { match pc {
 0x20b7d40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1a8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1a8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7d60); }
 let mut pc: u32 = 0x20b7d60;
 loop { match pc {
 0x20b7d60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1a4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1a4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7da0); }
 let mut pc: u32 = 0x20b7da0;
 loop { match pc {
 0x20b7da0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7db5; continue; }
  pc = 0x20b7da9; continue;
 }
 0x20b7da9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20b7db5; continue;
 }
 0x20b7db5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7e00); }
 let mut pc: u32 = 0x20b7e00;
 loop { match pc {
 0x20b7e00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20b7e87; continue; }
  pc = 0x20b7e14; continue;
 }
 0x20b7e14 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b7e87; continue; }
  pc = 0x20b7e1f; continue;
 }
 0x20b7e1f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b7e43; continue; }
  pc = 0x20b7e30; continue;
 }
 0x20b7e30 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x20b7e32; continue;
 }
 0x20b7e32 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.zf { pc = 0x20b7e43; continue; }
  pc = 0x20b7e3b; continue;
 }
 0x20b7e3b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b7e32; continue; }
  pc = 0x20b7e43; continue;
 }
 0x20b7e43 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x20b7e87; continue; }
  pc = 0x20b7e47; continue;
 }
 0x20b7e47 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d1ffcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20b7e87; continue; }
  pc = 0x20b7e71; continue;
 }
 0x20b7e71 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.edx = v; }
  pc = 0x20b7e76; continue;
 }
 0x20b7e76 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7e76; continue; }
  pc = 0x20b7e87; continue;
 }
 0x20b7e87 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7e90); }
 let mut pc: u32 = 0x20b7e90;
 loop { match pc {
 0x20b7e90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b7efd; continue; }
  pc = 0x20b7e9c; continue;
 }
 0x20b7e9c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d1ff8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b7edd; continue; }
  pc = 0x20b7ebb; continue;
 }
 0x20b7ebb => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x20b7ed2; continue; }
  pc = 0x20b7ec7; continue;
 }
 0x20b7ec7 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x68u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x20b7ed2; continue; }
  pc = 0x20b7ecf; continue;
 }
 0x20b7ecf => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b7ed2; continue;
 }
 0x20b7ed2 => {
  { let v: u32 = cpu.r32(0x22d1ff8u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b7ebb; continue; }
  pc = 0x20b7edd; continue;
 }
 0x20b7edd => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20b7ee3; continue; }
  pc = 0x20b7ee1; continue;
 }
 0x20b7ee1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b7ee3; continue;
 }
 0x20b7ee3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b7eed; continue; }
  pc = 0x20b7eeb; continue;
 }
 0x20b7eeb => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b7eed; continue;
 }
 0x20b7eed => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b7ef7; continue; }
  pc = 0x20b7ef5; continue;
 }
 0x20b7ef5 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b7ef7; continue;
 }
 0x20b7ef7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b7efd; continue;
 }
 0x20b7efd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b7f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b7f10); }
 let mut pc: u32 = 0x20b7f10;
 loop { match pc {
 0x20b7f10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b7f1d; continue; }
  pc = 0x20b7f19; continue;
 }
 0x20b7f19 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b7f1d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x40u32); cpu.ecx = v; }
  pc = 0x20b7f29; continue;
 }
 0x20b7f29 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20b7f32; continue; }
  pc = 0x20b7f31; continue;
 }
 0x20b7f31 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b7f32; continue;
 }
 0x20b7f32 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b7f29; continue; }
  pc = 0x20b7f39; continue;
 }
 0x20b7f39 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8200); }
 let mut pc: u32 = 0x20b8200;
 loop { match pc {
 0x20b8200 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8209; continue; }
  pc = 0x20b8208; continue;
 }
 0x20b8208 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8209 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8210); }
 let mut pc: u32 = 0x20b8210;
 loop { match pc {
 0x20b8210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b8289; continue; }
  pc = 0x20b821f; continue;
 }
 0x20b821f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x20b8240; continue; }
  pc = 0x20b8236; continue;
 }
 0x20b8236 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8240 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b8249; continue; }
  pc = 0x20b8244; continue;
 }
 0x20b8244 => {
  { let v: u32 = cpu.esi; cpu.w32(0x3cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b8276; continue;
 }
 0x20b8249 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b8263; continue; }
  pc = 0x20b8261; continue;
 }
 0x20b8261 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b8263; continue;
 }
 0x20b8263 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b8276; continue;
 }
 0x20b8276 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b827c); f_020b9d90(cpu); if cpu.returned_to != 0x20b827c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b8289; continue;
 }
 0x20b8289 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8290); }
 let mut pc: u32 = 0x20b8290;
 loop { match pc {
 0x20b8290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b82f1; continue; }
  pc = 0x20b829e; continue;
 }
 0x20b829e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x20b82eb; continue; }
  pc = 0x20b82b5; continue;
 }
 0x20b82b5 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b82be; continue; }
  pc = 0x20b82b9; continue;
 }
 0x20b82b9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b82eb; continue;
 }
 0x20b82be => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b82d8; continue; }
  pc = 0x20b82d6; continue;
 }
 0x20b82d6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x20b82d8; continue;
 }
 0x20b82d8 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b82eb; continue;
 }
 0x20b82eb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b82f1; continue;
 }
 0x20b82f1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8300); }
 let mut pc: u32 = 0x20b8300;
 loop { match pc {
 0x20b8300 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b830f; continue; }
  pc = 0x20b8308; continue;
 }
 0x20b8308 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b830f; continue;
 }
 0x20b830f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8310); }
 let mut pc: u32 = 0x20b8310;
 loop { match pc {
 0x20b8310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b8342; continue; }
  pc = 0x20b831a; continue;
 }
 0x20b831a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20b8342; continue; }
  pc = 0x20b8322; continue;
 }
 0x20b8322 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x20b8342; continue; }
  pc = 0x20b8327; continue;
 }
 0x20b8327 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b832d); f_020a93e0(cpu); if cpu.returned_to != 0x20b832d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b8342; continue; }
  pc = 0x20b833e; continue;
 }
 0x20b833e => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x20b8342); cpu.call(t); if cpu.returned_to != 0x20b8342 { return; }
  pc = 0x20b8342; continue;
 }
 0x20b8342 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8350); }
 let mut pc: u32 = 0x20b8350;
 loop { match pc {
 0x20b8350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20b83ab; continue; }
  pc = 0x20b835c; continue;
 }
 0x20b835c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x8u32); }
  if !cpu.zf { pc = 0x20b83ab; continue; }
  pc = 0x20b8362; continue;
 }
 0x20b8362 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b8369; continue;
 }
 0x20b8369 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b8389; continue; }
  pc = 0x20b8372; continue;
 }
 0x20b8372 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b8378); f_020a7480(cpu); if cpu.returned_to != 0x20b8378 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b8383; continue; }
  pc = 0x20b8381; continue;
 }
 0x20b8381 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20b8383; continue;
 }
 0x20b8383 => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.esi), v); }
  pc = 0x20b8389; continue;
 }
 0x20b8389 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b8369; continue; }
  pc = 0x20b8392; continue;
 }
 0x20b8392 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b8394; continue;
 }
 0x20b8394 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b839b); f_020b9eb0(cpu); if cpu.returned_to != 0x20b839b { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b8394; continue; }
  pc = 0x20b83a4; continue;
 }
 0x20b83a4 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20b83ab; continue;
 }
 0x20b83ab => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b83b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b83b0); }
 let mut pc: u32 = 0x20b83b0;
 loop { match pc {
 0x20b83b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b844c; continue; }
  pc = 0x20b83c0; continue;
 }
 0x20b83c0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.zf { pc = 0x20b844c; continue; }
  pc = 0x20b83ca; continue;
 }
 0x20b83ca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x40u32); cpu.ebp = v; }
  cpu.push(0x20b83dc); f_020ba190(cpu); if cpu.returned_to != 0x20b83dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x20b83e2; continue;
 }
 0x20b83e2 => {
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b8405; continue; }
  pc = 0x20b83f3; continue;
 }
 0x20b83f3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b8402); f_020bad00(cpu); if cpu.returned_to != 0x20b8402 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20b8405; continue;
 }
 0x20b8405 => {
  { cpu.sub32(cpu.r32(0x1d8u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b8416; continue; }
  pc = 0x20b840e; continue;
 }
 0x20b840e => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x20b8416; continue;
 }
 0x20b8416 => {
  { cpu.sub32(cpu.r32(0x258u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b8431; continue; }
  pc = 0x20b841f; continue;
 }
 0x20b841f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b842e); f_020bad00(cpu); if cpu.returned_to != 0x20b842e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20b8431; continue;
 }
 0x20b8431 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b8445; continue; }
  pc = 0x20b843a; continue;
 }
 0x20b843a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b8442); f_020a73c0(cpu); if cpu.returned_to != 0x20b8442 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x20b8445; continue;
 }
 0x20b8445 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x20b83e2; continue; }
  pc = 0x20b844c; continue;
 }
 0x20b844c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8460); }
 let mut pc: u32 = 0x20b8460;
 loop { match pc {
 0x20b8460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b8491; continue; }
  pc = 0x20b8469; continue;
 }
 0x20b8469 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if cpu.zf { pc = 0x20b8491; continue; }
  pc = 0x20b846f; continue;
 }
 0x20b846f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8475); f_020a93e0(cpu); if cpu.returned_to != 0x20b8475 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b847e); f_020b9df0(cpu); if cpu.returned_to != 0x20b847e { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b8491; continue;
 }
 0x20b8491 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b84a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b84a0); }
 let mut pc: u32 = 0x20b84a0;
 loop { match pc {
 0x20b84a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b84b6; continue; }
  pc = 0x20b84af; continue;
 }
 0x20b84af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b88b7; continue;
 }
 0x20b84b6 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20b84cc); f_020baa70(cpu); if cpu.returned_to != 0x20b84cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b84f3; continue; }
  pc = 0x20b84d3; continue;
 }
 0x20b84d3 => {
  { let v: u32 = 0x216b1a0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b84df; continue;
 }
 0x20b84df => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b84df; continue; }
  pc = 0x20b84e9; continue;
 }
 0x20b84e9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b84f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0xff00u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 24u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff000000u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 24u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ebp); }
  if !cpu.cf { pc = 0x20b85f2; continue; }
  pc = 0x20b8550; continue;
 }
 0x20b8550 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216b1b8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8566; continue; }
  pc = 0x20b8564; continue;
 }
 0x20b8564 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8566; continue;
 }
 0x20b8566 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8571; continue; }
  pc = 0x20b856a; continue;
 }
 0x20b856a => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b8571; continue;
 }
 0x20b8571 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216b1c0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8587; continue; }
  pc = 0x20b8585; continue;
 }
 0x20b8585 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8587; continue;
 }
 0x20b8587 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8592; continue; }
  pc = 0x20b858b; continue;
 }
 0x20b858b => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b8592; continue;
 }
 0x20b8592 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216b1c8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b85a8; continue; }
  pc = 0x20b85a6; continue;
 }
 0x20b85a6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b85a8; continue;
 }
 0x20b85a8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b85b3; continue; }
  pc = 0x20b85ac; continue;
 }
 0x20b85ac => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b85b3; continue;
 }
 0x20b85b3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx & 0xff00u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 24u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x20b8550; continue; }
  pc = 0x20b85f2; continue;
 }
 0x20b85f2 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20b861c; continue; }
  pc = 0x20b85fc; continue;
 }
 0x20b85fc => {
  { let v: u32 = 0x216b1a0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8608; continue;
 }
 0x20b8608 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8608; continue; }
  pc = 0x20b8612; continue;
 }
 0x20b8612 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b861c => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b864e); f_020b9df0(cpu); if cpu.returned_to != 0x20b864e { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x22d1ff4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x4cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x50u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b88b2; continue; }
  pc = 0x20b86a5; continue;
 }
 0x20b86a5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b86ae); f_020babc0(cpu); if cpu.returned_to != 0x20b86ae { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20b86c2; continue; }
  pc = 0x20b86bb; continue;
 }
 0x20b86bb => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x20b86d6; continue;
 }
 0x20b86c2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b86d0); f_020babc0(cpu); if cpu.returned_to != 0x20b86d0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x20b86d6; continue;
 }
 0x20b86d6 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x22d209cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x22d209cu32; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b87aa; continue; }
  pc = 0x20b870a; continue;
 }
 0x20b870a => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b870e; continue;
 }
 0x20b870e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x20b87aa; continue; }
  pc = 0x20b871c; continue;
 }
 0x20b871c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.push(0x20b8745); cpu.call(t); if cpu.returned_to != 0x20b8745 { return; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8753; continue; }
  pc = 0x20b874c; continue;
 }
 0x20b874c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.poll(); pc = 0x20b870e; continue;
 }
 0x20b8753 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x22d209cu32); cpu.eax = v; }
  pc = 0x20b876f; continue;
 }
 0x20b8763 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b876f; continue;
 }
 0x20b876f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x20b8763; continue; }
  pc = 0x20b8779; continue;
 }
 0x20b8779 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.cf { pc = 0x20b879d; continue; }
  pc = 0x20b8788; continue;
 }
 0x20b8788 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x20b870e; continue;
 }
 0x20b879d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x20b870e; continue;
 }
 0x20b87aa => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x20b88b2; continue; }
  pc = 0x20b87b5; continue;
 }
 0x20b87b5 => {
  cpu.push(0x20b87ba); f_020acf80(cpu); if cpu.returned_to != 0x20b87ba { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b87e1; continue; }
  pc = 0x20b87be; continue;
 }
 0x20b87be => {
  { let v: u32 = 0x216b1d0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b87ca; continue;
 }
 0x20b87ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b87ca; continue; }
  pc = 0x20b87d4; continue;
 }
 0x20b87d4 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b87e1 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b8837; continue; }
  pc = 0x20b87ff; continue;
 }
 0x20b87ff => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8820); f_020ad040(cpu); if cpu.returned_to != 0x20b8820 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b8837; continue;
 }
 0x20b8837 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = 0x503u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b8878); f_020ad040(cpu); if cpu.returned_to != 0x20b8878 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x20b88b2; continue; }
  pc = 0x20b888d; continue;
 }
 0x20b888d => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.push(0x20b88a5); f_020b88c0(cpu); if cpu.returned_to != 0x20b88a5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b88b2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b88b7; continue;
 }
 0x20b88b7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b88c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b88c0); }
 let mut pc: u32 = 0x20b88c0;
 loop { match pc {
 0x20b88c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = 0x22d1ec0u32; cpu.push(v); }
  { let v = 0x216b16cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20b88e4); cpu.call(t); if cpu.returned_to != 0x20b88e4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b88f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b88f0); }
 let mut pc: u32 = 0x20b88f0;
 loop { match pc {
 0x20b88f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b8909; continue; }
  pc = 0x20b88f9; continue;
 }
 0x20b88f9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b88ff); f_020a93e0(cpu); if cpu.returned_to != 0x20b88ff { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b8909; continue;
 }
 0x20b8909 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8910); }
 let mut pc: u32 = 0x20b8910;
 loop { match pc {
 0x20b8910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x20b891c); f_020ad4e0(cpu); if cpu.returned_to != 0x20b891c { return; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b893c; continue; }
  pc = 0x20b8925; continue;
 }
 0x20b8925 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x20b8928; continue;
 }
 0x20b8928 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x20b893c; continue; }
  pc = 0x20b892e; continue;
 }
 0x20b892e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b8928; continue; }
  pc = 0x20b893c; continue;
 }
 0x20b893c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20b8962; continue; }
  pc = 0x20b8941; continue;
 }
 0x20b8941 => {
  { let v: u32 = 0x216b150u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b894d; continue;
 }
 0x20b894d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b894d; continue; }
  pc = 0x20b8957; continue;
 }
 0x20b8957 => {
  cpu.push(0x20b895c); f_020ad4d0(cpu); if cpu.returned_to != 0x20b895c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8962 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b8985); f_020b9df0(cpu); if cpu.returned_to != 0x20b8985 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b8994); f_020ad4d0(cpu); if cpu.returned_to != 0x20b8994 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b89a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b89a0); }
 let mut pc: u32 = 0x20b89a0;
 loop { match pc {
 0x20b89a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b89ae; continue;
 }
 0x20b89ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b89ae; continue; }
  pc = 0x20b89b8; continue;
 }
 0x20b89b8 => {
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b8a11; continue; }
  pc = 0x20b89d0; continue;
 }
 0x20b89d0 => {
  { let v: u8 = cpu.r8(0x217c79cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x20b89fd; continue; }
  pc = 0x20b89db; continue;
 }
 0x20b89db => {
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b8a11; continue; }
  pc = 0x20b89f1; continue;
 }
 0x20b89f1 => {
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x217c79cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b89fd => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x20b8a0e; continue; }
  pc = 0x20b8a02; continue;
 }
 0x20b8a02 => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x217c79cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8a0e => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20b89d0; continue; }
  pc = 0x20b8a11; continue;
 }
 0x20b8a11 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8a20); }
 let mut pc: u32 = 0x20b8a20;
 loop { match pc {
 0x20b8a20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x112u32); }
  if cpu.cf { pc = 0x20b8ac4; continue; }
  pc = 0x20b8a3a; continue;
 }
 0x20b8a3a => {
  { let v: u32 = cpu.edx.wrapping_add(0xbau32); cpu.edx = v; }
  { let v: u32 = 0x216b114u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b8a47; continue;
 }
 0x20b8a47 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8a55; continue; }
  pc = 0x20b8a4f; continue;
 }
 0x20b8a4f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8a55; continue; }
  pc = 0x20b8a53; continue;
 }
 0x20b8a53 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8a55; continue;
 }
 0x20b8a55 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8a62; continue; }
  pc = 0x20b8a5a; continue;
 }
 0x20b8a5a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8a62; continue; }
  pc = 0x20b8a5f; continue;
 }
 0x20b8a5f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b8a62; continue;
 }
 0x20b8a62 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b8a6e; continue; }
  pc = 0x20b8a66; continue;
 }
 0x20b8a66 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b8a6e; continue; }
  pc = 0x20b8a6a; continue;
 }
 0x20b8a6a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b8a47; continue;
 }
 0x20b8a6e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8a75; continue; }
  pc = 0x20b8a73; continue;
 }
 0x20b8a73 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8a75; continue;
 }
 0x20b8a75 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8a82; continue; }
  pc = 0x20b8a79; continue;
 }
 0x20b8a79 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8a82 => {
  { let v: u32 = 0x216b128u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b8a89; continue;
 }
 0x20b8a89 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8a97; continue; }
  pc = 0x20b8a91; continue;
 }
 0x20b8a91 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8a97; continue; }
  pc = 0x20b8a95; continue;
 }
 0x20b8a95 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8a97; continue;
 }
 0x20b8a97 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8aa4; continue; }
  pc = 0x20b8a9c; continue;
 }
 0x20b8a9c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8aa4; continue; }
  pc = 0x20b8aa1; continue;
 }
 0x20b8aa1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b8aa4; continue;
 }
 0x20b8aa4 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b8ab0; continue; }
  pc = 0x20b8aa8; continue;
 }
 0x20b8aa8 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b8ab0; continue; }
  pc = 0x20b8aac; continue;
 }
 0x20b8aac => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b8a89; continue;
 }
 0x20b8ab0 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8ab7; continue; }
  pc = 0x20b8ab5; continue;
 }
 0x20b8ab5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8ab7; continue;
 }
 0x20b8ab7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8ac4; continue; }
  pc = 0x20b8abb; continue;
 }
 0x20b8abb => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8ac4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  if cpu.zf { pc = 0x20b8b7a; continue; }
  pc = 0x20b8ae1; continue;
 }
 0x20b8ae1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x20b8b7a; continue; }
  pc = 0x20b8aea; continue;
 }
 0x20b8aea => {
  { let v: u32 = 0x216b144u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b8af1; continue;
 }
 0x20b8af1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8aff; continue; }
  pc = 0x20b8af9; continue;
 }
 0x20b8af9 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8aff; continue; }
  pc = 0x20b8afd; continue;
 }
 0x20b8afd => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8aff; continue;
 }
 0x20b8aff => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8b0c; continue; }
  pc = 0x20b8b04; continue;
 }
 0x20b8b04 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8b0c; continue; }
  pc = 0x20b8b09; continue;
 }
 0x20b8b09 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b8b0c; continue;
 }
 0x20b8b0c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b8b18; continue; }
  pc = 0x20b8b10; continue;
 }
 0x20b8b10 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b8b18; continue; }
  pc = 0x20b8b14; continue;
 }
 0x20b8b14 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b8af1; continue;
 }
 0x20b8b18 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8b1f; continue; }
  pc = 0x20b8b1d; continue;
 }
 0x20b8b1d => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8b1f; continue;
 }
 0x20b8b1f => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8b38; continue; }
  pc = 0x20b8b23; continue;
 }
 0x20b8b23 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x5000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8b7a; continue; }
  pc = 0x20b8b2f; continue;
 }
 0x20b8b2f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8b38 => {
  { let v: u32 = 0x216b148u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b8b3f; continue;
 }
 0x20b8b3f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8b4d; continue; }
  pc = 0x20b8b47; continue;
 }
 0x20b8b47 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8b4d; continue; }
  pc = 0x20b8b4b; continue;
 }
 0x20b8b4b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8b4d; continue;
 }
 0x20b8b4d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20b8b5a; continue; }
  pc = 0x20b8b52; continue;
 }
 0x20b8b52 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b8b5a; continue; }
  pc = 0x20b8b57; continue;
 }
 0x20b8b57 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b8b5a; continue;
 }
 0x20b8b5a => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b8b66; continue; }
  pc = 0x20b8b5e; continue;
 }
 0x20b8b5e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b8b66; continue; }
  pc = 0x20b8b62; continue;
 }
 0x20b8b62 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20b8b3f; continue;
 }
 0x20b8b66 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20b8b6d; continue; }
  pc = 0x20b8b6b; continue;
 }
 0x20b8b6b => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20b8b6d; continue;
 }
 0x20b8b6d => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b8b7a; continue; }
  pc = 0x20b8b71; continue;
 }
 0x20b8b71 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8b7a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8b80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8b80); }
 let mut pc: u32 = 0x20b8b80;
 loop { match pc {
 0x20b8b80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20b8b8d); f_020ac0d0(cpu); if cpu.returned_to != 0x20b8b8d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8ba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8ba0); }
 let mut pc: u32 = 0x20b8ba0;
 loop { match pc {
 0x20b8ba0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x216b0ecu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8bb7); f_020acc50(cpu); if cpu.returned_to != 0x20b8bb7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8bdb; continue; }
  pc = 0x20b8bbe; continue;
 }
 0x20b8bbe => {
  { let v: u32 = 0x216b0f4u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8bca; continue;
 }
 0x20b8bca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8bca; continue; }
  pc = 0x20b8bd4; continue;
 }
 0x20b8bd4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b8c10; continue;
 }
 0x20b8bdb => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8bf0); f_020a98c0(cpu); if cpu.returned_to != 0x20b8bf0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8c0e; continue; }
  pc = 0x20b8c00; continue;
 }
 0x20b8c00 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8c0e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b8c10; continue;
 }
 0x20b8c10 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8c20); }
 let mut pc: u32 = 0x20b8c20;
 loop { match pc {
 0x20b8c20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8c31); f_020ac180(cpu); if cpu.returned_to != 0x20b8c31 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b8c59; continue; }
  pc = 0x20b8c3a; continue;
 }
 0x20b8c3a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8c45); f_020b8db0(cpu); if cpu.returned_to != 0x20b8c45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8c57; continue; }
  pc = 0x20b8c4e; continue;
 }
 0x20b8c4e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8c54); f_020ac0d0(cpu); if cpu.returned_to != 0x20b8c54 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b8c57; continue;
 }
 0x20b8c57 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b8c59; continue;
 }
 0x20b8c59 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8c60); }
 let mut pc: u32 = 0x20b8c60;
 loop { match pc {
 0x20b8c60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x180u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x194u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x108u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20b8c84); f_020b3350(cpu); if cpu.returned_to != 0x20b8c84 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8d42; continue; }
  pc = 0x20b8c91; continue;
 }
 0x20b8c91 => {
  { cpu.sub8(cpu.r8(0x217c608u32), 0x0u8); }
  if cpu.zf { pc = 0x20b8d2a; continue; }
  pc = 0x20b8c9e; continue;
 }
 0x20b8c9e => {
  { let v: u32 = 0x217c608u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8ca7; continue;
 }
 0x20b8ca7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8ca7; continue; }
  pc = 0x20b8cb1; continue;
 }
 0x20b8cb1 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b8cc2; continue;
 }
 0x20b8cc2 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8cc2; continue; }
  pc = 0x20b8cc8; continue;
 }
 0x20b8cc8 => {
  { let v: u32 = 0x217c79cu32; cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8cd6; continue;
 }
 0x20b8cd6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8cd6; continue; }
  pc = 0x20b8ce0; continue;
 }
 0x20b8ce0 => {
  { let v: u32 = 0x217c608u32; cpu.esi = v; }
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8cec; continue;
 }
 0x20b8cec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8cec; continue; }
  pc = 0x20b8cf6; continue;
 }
 0x20b8cf6 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b8d0f; continue;
 }
 0x20b8d0f => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8d0f; continue; }
  pc = 0x20b8d15; continue;
 }
 0x20b8d15 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8d21); f_020b3350(cpu); if cpu.returned_to != 0x20b8d21 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8d42; continue; }
  pc = 0x20b8d2a; continue;
 }
 0x20b8d2a => {
  { let v: u32 = 0x216b0d4u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8d36; continue;
 }
 0x20b8d36 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8d36; continue; }
  pc = 0x20b8d40; continue;
 }
 0x20b8d40 => {
  pc = 0x20b8d80; continue;
 }
 0x20b8d42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8d4e); f_020ac180(cpu); if cpu.returned_to != 0x20b8d4e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20b8d59); f_020b3710(cpu); if cpu.returned_to != 0x20b8d59 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b8d80; continue; }
  pc = 0x20b8d60; continue;
 }
 0x20b8d60 => {
  { let v: u32 = cpu.r32(0x198u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8d6e); f_020b8db0(cpu); if cpu.returned_to != 0x20b8d6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8d80; continue; }
  pc = 0x20b8d77; continue;
 }
 0x20b8d77 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8d7d); f_020ac0d0(cpu); if cpu.returned_to != 0x20b8d7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b8d80; continue;
 }
 0x20b8d80 => {
  { cpu.sub8(cpu.r8(0x100u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x20b8da2; continue; }
  pc = 0x20b8d8a; continue;
 }
 0x20b8d8a => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x217c79cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8d98; continue;
 }
 0x20b8d98 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8d98; continue; }
  pc = 0x20b8da2; continue;
 }
 0x20b8da2 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x180u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b8db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b8db0); }
 let mut pc: u32 = 0x20b8db0;
 loop { match pc {
 0x20b8db0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x20b8dbc); f_020baf30(cpu); if cpu.returned_to != 0x20b8dbc { return; }
  { let v = 0x1d8u32; cpu.push(v); }
  cpu.push(0x20b8dc6); f_020b3740(cpu); if cpu.returned_to != 0x20b8dc6 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b8dec; continue; }
  pc = 0x20b8dcf; continue;
 }
 0x20b8dcf => {
  { let v: u32 = 0x216aff8u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8ddb; continue;
 }
 0x20b8ddb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8ddb; continue; }
  pc = 0x20b8de5; continue;
 }
 0x20b8de5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b947a; continue;
 }
 0x20b8dec => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216b020u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x20b8e44; continue; }
  pc = 0x20b8e19; continue;
 }
 0x20b8e19 => {
  { let v: u32 = 0x216b028u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8e27; continue;
 }
 0x20b8e27 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8e27; continue; }
  pc = 0x20b8e31; continue;
 }
 0x20b8e31 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8e37); f_020b3710(cpu); if cpu.returned_to != 0x20b8e37 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b8e44 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b8e56); f_020ad040(cpu); if cpu.returned_to != 0x20b8e56 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  if cpu.zf { pc = 0x20b8eee; continue; }
  pc = 0x20b8e9f; continue;
 }
 0x20b8e9f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20b8eee; continue; }
  pc = 0x20b8ea4; continue;
 }
 0x20b8ea4 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba30u32); cpu.push(0x20b8eab); cpu.call(t); if cpu.returned_to != 0x20b8eab { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b8eee; continue; }
  pc = 0x20b8eb1; continue;
 }
 0x20b8eb1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20b8eee; continue; }
  pc = 0x20b8eb6; continue;
 }
 0x20b8eb6 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b8ed0; continue; }
  pc = 0x20b8ecb; continue;
 }
 0x20b8ecb => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x20b8ed0; continue;
 }
 0x20b8ed0 => {
  { let v: u32 = cpu.r32(0x22d2070u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x20b8eee; continue;
 }
 0x20b8eee => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x20b8f46; continue; }
  pc = 0x20b8f07; continue;
 }
 0x20b8f07 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20b8f46; continue; }
  pc = 0x20b8f0c; continue;
 }
 0x20b8f0c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x217c79cu32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b8f1c; continue;
 }
 0x20b8f1c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8f1c; continue; }
  pc = 0x20b8f26; continue;
 }
 0x20b8f26 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b8f3e; continue;
 }
 0x20b8f3e => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b8f3e; continue; }
  pc = 0x20b8f44; continue;
 }
 0x20b8f44 => {
  pc = 0x20b8f50; continue;
 }
 0x20b8f46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b8f50; continue;
 }
 0x20b8f50 => {
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b8fb7; continue; }
  pc = 0x20b8f83; continue;
 }
 0x20b8f83 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b8f9a); f_020b94f0(cpu); if cpu.returned_to != 0x20b8f9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b8fb7; continue; }
  pc = 0x20b8fa1; continue;
 }
 0x20b8fa1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x20b8fb7; continue;
 }
 0x20b8fb7 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b9000; continue; }
  pc = 0x20b8fbe; continue;
 }
 0x20b8fbe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b8fc7); f_020ac290(cpu); if cpu.returned_to != 0x20b8fc7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b9000; continue; }
  pc = 0x20b8fd0; continue;
 }
 0x20b8fd0 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x20b8fe5); f_020b94f0(cpu); if cpu.returned_to != 0x20b8fe5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b9000; continue; }
  pc = 0x20b8fec; continue;
 }
 0x20b8fec => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x20b9000; continue;
 }
 0x20b9000 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20b909b; continue; }
  pc = 0x20b900c; continue;
 }
 0x20b900c => {
  { cpu.sub32(cpu.r32(0x22d2004u32), 0x1u32); }
  if !cpu.zf { pc = 0x20b909b; continue; }
  pc = 0x20b9019; continue;
 }
 0x20b9019 => {
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b9087; continue;
 }
 0x20b9023 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b904f; continue; }
  pc = 0x20b9041; continue;
 }
 0x20b9041 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x20b904f; continue;
 }
 0x20b904f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9056); f_020b94f0(cpu); if cpu.returned_to != 0x20b9056 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b9073; continue; }
  pc = 0x20b905d; continue;
 }
 0x20b905d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x20b909b; continue;
 }
 0x20b9073 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b9087; continue;
 }
 0x20b9087 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b9023; continue; }
  pc = 0x20b909b; continue;
 }
 0x20b909b => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b90cb; continue; }
  pc = 0x20b90a2; continue;
 }
 0x20b90a2 => {
  { let v: u32 = 0x216b040u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b90ae; continue;
 }
 0x20b90ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b90ae; continue; }
  pc = 0x20b90b8; continue;
 }
 0x20b90b8 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b90be); f_020b3710(cpu); if cpu.returned_to != 0x20b90be { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b90cb => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x22d2010u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b90ed); f_020ad040(cpu); if cpu.returned_to != 0x20b90ed { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x501u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b910f); f_020ad040(cpu); if cpu.returned_to != 0x20b910f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x20b9162; continue; }
  pc = 0x20b911a; continue;
 }
 0x20b911a => {
  { let v: u32 = 0x216b060u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b9126; continue;
 }
 0x20b9126 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9126; continue; }
  pc = 0x20b9130; continue;
 }
 0x20b9130 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b9142); f_020ad040(cpu); if cpu.returned_to != 0x20b9142 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b9155); f_020b3710(cpu); if cpu.returned_to != 0x20b9155 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b9162 => {
  { let v: u32 = cpu.r32(0x22d1ff0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b9181); f_020b3740(cpu); if cpu.returned_to != 0x20b9181 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b91d3; continue; }
  pc = 0x20b918b; continue;
 }
 0x20b918b => {
  { let v: u32 = 0x216b08cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b9197; continue;
 }
 0x20b9197 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9197; continue; }
  pc = 0x20b91a1; continue;
 }
 0x20b91a1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b91b3); f_020ad040(cpu); if cpu.returned_to != 0x20b91b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b91c6); f_020b3710(cpu); if cpu.returned_to != 0x20b91c6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b91d3 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b9227; continue; }
  pc = 0x20b91e0; continue;
 }
 0x20b91e0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20b91e2; continue;
 }
 0x20b91e2 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x216b0b4u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b91e2; continue; }
  pc = 0x20b9227; continue;
 }
 0x20b9227 => {
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d1fecu32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x1b4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b927c; continue;
 }
 0x20b927c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xe0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x120u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x160u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b927c; continue; }
  pc = 0x20b92a1; continue;
 }
 0x20b92a1 => {
  { let v = 0x20b95d0u32; cpu.push(v); }
  cpu.push(0x20b92ab); f_020acb50(cpu); if cpu.returned_to != 0x20b92ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b930a; continue; }
  pc = 0x20b92b6; continue;
 }
 0x20b92b6 => {
  { let v: u32 = 0x216b0bcu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b92c2; continue;
 }
 0x20b92c2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b92c2; continue; }
  pc = 0x20b92cc; continue;
 }
 0x20b92cc => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b92de); f_020ad040(cpu); if cpu.returned_to != 0x20b92de { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b92f4); f_020b3710(cpu); if cpu.returned_to != 0x20b92f4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b92fd); f_020b3710(cpu); if cpu.returned_to != 0x20b92fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b930a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b9311); f_020aca40(cpu); if cpu.returned_to != 0x20b9311 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20b9490u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b9328; continue;
 }
 0x20b9328 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9337); f_020bad00(cpu); if cpu.returned_to != 0x20b9337 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9347); f_020bad00(cpu); if cpu.returned_to != 0x20b9347 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9358); f_020bad00(cpu); if cpu.returned_to != 0x20b9358 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x70u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9365); f_020bad00(cpu); if cpu.returned_to != 0x20b9365 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9372); f_020bad00(cpu); if cpu.returned_to != 0x20b9372 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d1ff4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9384); f_020bad00(cpu); if cpu.returned_to != 0x20b9384 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9392); f_020bad00(cpu); if cpu.returned_to != 0x20b9392 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93a0); f_020bad00(cpu); if cpu.returned_to != 0x20b93a0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93ad); f_020bad00(cpu); if cpu.returned_to != 0x20b93ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = 0x5bu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93bb); f_020bad00(cpu); if cpu.returned_to != 0x20b93bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x5du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93c8); f_020bad00(cpu); if cpu.returned_to != 0x20b93c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93d5); f_020bad00(cpu); if cpu.returned_to != 0x20b93d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x65u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93e2); f_020bad00(cpu); if cpu.returned_to != 0x20b93e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b93ef); f_020bad00(cpu); if cpu.returned_to != 0x20b93ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d2000u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b9402); f_020bad00(cpu); if cpu.returned_to != 0x20b9402 { return; }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20b943e; continue; }
  pc = 0x20b940f; continue;
 }
 0x20b940f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b942f); f_020ad040(cpu); if cpu.returned_to != 0x20b942f { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b943e; continue;
 }
 0x20b943e => {
  { cpu.and16((cpu.esi as u16), 0x3u16); }
  if !cpu.zf { pc = 0x20b944f; continue; }
  pc = 0x20b9445; continue;
 }
 0x20b9445 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x20b944c); f_020acf90(cpu); if cpu.returned_to != 0x20b944c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b944f; continue;
 }
 0x20b944f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9328; continue; }
  pc = 0x20b9459; continue;
 }
 0x20b9459 => {
  { let v: u32 = cpu.r32(0x22d1fecu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b9469); f_020ac8c0(cpu); if cpu.returned_to != 0x20b9469 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b9475); f_020ac670(cpu); if cpu.returned_to != 0x20b9475 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20b947a; continue;
 }
 0x20b947a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b9490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b9490); }
 let mut pc: u32 = 0x20b9490;
 loop { match pc {
 0x20b9490 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b94bf; continue; }
  pc = 0x20b94a1; continue;
 }
 0x20b94a1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b94a3; continue;
 }
 0x20b94a3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x20b94af); f_020a9280(cpu); if cpu.returned_to != 0x20b94af { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b94a3; continue; }
  pc = 0x20b94bf; continue;
 }
 0x20b94bf => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b94c8); f_020ac410(cpu); if cpu.returned_to != 0x20b94c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20b94d4); f_020b3710(cpu); if cpu.returned_to != 0x20b94d4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b94dd); f_020b3710(cpu); if cpu.returned_to != 0x20b94dd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b94f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b94f0); }
 let mut pc: u32 = 0x20b94f0;
 loop { match pc {
 0x20b94f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x20b9597; continue; }
  pc = 0x20b9519; continue;
 }
 0x20b9519 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b9540; continue; }
  pc = 0x20b9539; continue;
 }
 0x20b9539 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b9540; continue;
 }
 0x20b9540 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b9554; continue; }
  pc = 0x20b954b; continue;
 }
 0x20b954b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b9554; continue;
 }
 0x20b9554 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b9569; continue; }
  pc = 0x20b9560; continue;
 }
 0x20b9560 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b9569; continue;
 }
 0x20b9569 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b957e; continue; }
  pc = 0x20b9575; continue;
 }
 0x20b9575 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b957e; continue;
 }
 0x20b957e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b9580; continue;
 }
 0x20b9580 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b958f; continue; }
  pc = 0x20b9587; continue;
 }
 0x20b9587 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x20b958f; continue;
 }
 0x20b958f => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9580; continue; }
  pc = 0x20b9597; continue;
 }
 0x20b9597 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b95bd); f_020ad040(cpu); if cpu.returned_to != 0x20b95bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b95d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b95d0); }
 let mut pc: u32 = 0x20b95d0;
 loop { match pc {
 0x20b95d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b9d7a; continue; }
  pc = 0x20b95e9; continue;
 }
 0x20b95e9 => {
  { cpu.sub32(cpu.r32(0x217c81cu32), 0x0u32); }
  if !cpu.zf { pc = 0x20b9d7a; continue; }
  pc = 0x20b95f6; continue;
 }
 0x20b95f6 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217c81cu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d2088u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d207cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b9d25; continue; }
  pc = 0x20b961a; continue;
 }
 0x20b961a => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x20b9d01; continue; }
  pc = 0x20b9629; continue;
 }
 0x20b9629 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d2098u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9643; continue;
 }
 0x20b9643 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b9c39; continue; }
  pc = 0x20b9654; continue;
 }
 0x20b9654 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b96fb; continue; }
  pc = 0x20b9668; continue;
 }
 0x20b9668 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2080u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x20b9675; continue;
 }
 0x20b9675 => {
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b96e5; continue; }
  pc = 0x20b968d; continue;
 }
 0x20b968d => {
  { let v: u32 = cpu.r32(0x65cu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x65cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x20b96e5; continue; }
  pc = 0x20b969f; continue;
 }
 0x20b969f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b96be); f_020ba250(cpu); if cpu.returned_to != 0x20b96be { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x20b96fb; continue; }
  pc = 0x20b96e5; continue;
 }
 0x20b96e5 => {
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d2080u32, v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9675; continue; }
  pc = 0x20b96fb; continue;
 }
 0x20b96fb => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b9bac; continue; }
  pc = 0x20b970f; continue;
 }
 0x20b970f => {
  { cpu.sub32(cpu.r32(0x22d2098u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b9b8a; continue; }
  pc = 0x20b971c; continue;
 }
 0x20b971c => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2094u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.cf { pc = 0x20b9b8a; continue; }
  pc = 0x20b973a; continue;
 }
 0x20b973a => {
  { cpu.sub32(cpu.eax, 0xf7u32); }
  if cpu.cf { pc = 0x20b9753; continue; }
  pc = 0x20b9741; continue;
 }
 0x20b9741 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b993f; continue; }
  pc = 0x20b9747; continue;
 }
 0x20b9747 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { pc = 0x20b9763; continue; }
  pc = 0x20b974e; continue;
 }
 0x20b974e => {
  pc = 0x20b99de; continue;
 }
 0x20b9753 => {
  { cpu.sub32(cpu.eax, 0xf0u32); }
  if cpu.zf { pc = 0x20b993f; continue; }
  pc = 0x20b975e; continue;
 }
 0x20b975e => {
  pc = 0x20b99de; continue;
 }
 0x20b9763 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x18u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2074u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b977b); f_020bac00(cpu); if cpu.returned_to != 0x20b977b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2070u32, v); }
  { let v: u32 = cpu.r32(0x22d2074u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x51u32); }
  if cpu.cf { pc = 0x20b97a1; continue; }
  pc = 0x20b978d; continue;
 }
 0x20b978d => {
  if (cpu.cf || cpu.zf) { pc = 0x20b9844; continue; }
  pc = 0x20b9793; continue;
 }
 0x20b9793 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if cpu.zf { pc = 0x20b988b; continue; }
  pc = 0x20b979c; continue;
 }
 0x20b979c => {
  pc = 0x20b992c; continue;
 }
 0x20b97a1 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x20b992c; continue; }
  pc = 0x20b97aa; continue;
 }
 0x20b97aa => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d2098u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b97ca; continue; }
  pc = 0x20b97c1; continue;
 }
 0x20b97c1 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x20b9808; continue; }
  pc = 0x20b97ca; continue;
 }
 0x20b97ca => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20b992c; continue; }
  pc = 0x20b97f8; continue;
 }
 0x20b97f8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x20b9929; continue;
 }
 0x20b9808 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b9811); f_020a93e0(cpu); if cpu.returned_to != 0x20b9811 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b992c; continue; }
  pc = 0x20b982b; continue;
 }
 0x20b982b => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x20b9831); cpu.call(t); if cpu.returned_to != 0x20b9831 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2070u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b9844 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d2090u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2070u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b988b => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2090u32, v); }
  { let v: u32 = 0xf42400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d1fecu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x22d2090u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d208cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20b98e6; continue; }
  pc = 0x20b98d2; continue;
 }
 0x20b98d2 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2090u32, v); }
  { let v: u8 = cpu.r8(0x22d2090u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x20b98ee; continue;
 }
 0x20b98e6 => {
  { let v: u8 = cpu.r8(0x22d2090u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x20b98ee; continue;
 }
 0x20b98ee => {
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b992c; continue; }
  pc = 0x20b991d; continue;
 }
 0x20b991d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x20b9929; continue;
 }
 0x20b9929 => {
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.push(0x20b992c); cpu.call(t); if cpu.returned_to != 0x20b992c { return; }
  pc = 0x20b992c; continue;
 }
 0x20b992c => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2070u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b993f => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = 0x22d2068u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2068u32, v); }
  cpu.push(0x20b9957); f_020bac00(cpu); if cpu.returned_to != 0x20b9957 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d2068u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2070u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2070u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20b9985); f_020bae80(cpu); if cpu.returned_to != 0x20b9985 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20b9997; continue; }
  pc = 0x20b9990; continue;
 }
 0x20b9990 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x20b9999; continue;
 }
 0x20b9997 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x20b9999; continue;
 }
 0x20b9999 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b99c3); f_020bae80(cpu); if cpu.returned_to != 0x20b99c3 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d2070u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b99de => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d206cu32, v); }
  { let v: u32 = cpu.r32(0x22d2094u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d2078u32, v); }
  { let v: u32 = cpu.r32(0x22d2094u32) & 0xf0u32; cpu.w32(0x22d2094u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b9a25); f_020ba250(cpu); if cpu.returned_to != 0x20b9a25 { return; }
  { let v: u32 = cpu.r32(0x22d2094u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if cpu.zf { pc = 0x20b9ab7; continue; }
  pc = 0x20b9a3a; continue;
 }
 0x20b9a3a => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d206cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x20b970f; continue; }
  pc = 0x20b9a50; continue;
 }
 0x20b9a50 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x20b9a80; continue; }
  pc = 0x20b9a63; continue;
 }
 0x20b9a63 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b9a99; continue; }
  pc = 0x20b9a65; continue;
 }
 0x20b9a65 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x20b9a77; continue; }
  pc = 0x20b9a6c; continue;
 }
 0x20b9a6c => {
  if (cpu.cf || cpu.zf) { pc = 0x20b9aa0; continue; }
  pc = 0x20b9a6e; continue;
 }
 0x20b9a6e => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x20b9a99; continue; }
  pc = 0x20b9a75; continue;
 }
 0x20b9a75 => {
  pc = 0x20b9aa7; continue;
 }
 0x20b9a77 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x20b9aa0; continue; }
  pc = 0x20b9a7e; continue;
 }
 0x20b9a7e => {
  pc = 0x20b9aa7; continue;
 }
 0x20b9a80 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x20b9a92; continue; }
  pc = 0x20b9a87; continue;
 }
 0x20b9a87 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b9a99; continue; }
  pc = 0x20b9a89; continue;
 }
 0x20b9a89 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x20b9a99; continue; }
  pc = 0x20b9a90; continue;
 }
 0x20b9a90 => {
  pc = 0x20b9aa7; continue;
 }
 0x20b9a92 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x20b9aa7; continue; }
  pc = 0x20b9a99; continue;
 }
 0x20b9a99 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x20b9aa9; continue;
 }
 0x20b9aa0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x20b9aa9; continue;
 }
 0x20b9aa7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b9aa9; continue;
 }
 0x20b9aa9 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b9ab7 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2080u32, v); }
  pc = 0x20b9ac7; continue;
 }
 0x20b9ac7 => {
  { cpu.sub32(cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9ae8; continue; }
  pc = 0x20b9ad1; continue;
 }
 0x20b9ad1 => {
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d2080u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9ac7; continue; }
  pc = 0x20b9ae8; continue;
 }
 0x20b9ae8 => {
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if !cpu.zf { pc = 0x20b9b31; continue; }
  pc = 0x20b9af3; continue;
 }
 0x20b9af3 => {
  { let v: u32 = 0x216afd8u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20b9aff; continue;
 }
 0x20b9aff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9aff; continue; }
  pc = 0x20b9b09; continue;
 }
 0x20b9b09 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x20b9b17); f_020a93e0(cpu); if cpu.returned_to != 0x20b9b17 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x217c81cu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9d7a; continue;
 }
 0x20b9b31 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x558u32.wrapping_add(cpu.eax))); cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d2078u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20b9b6f); f_020bac00(cpu); if cpu.returned_to != 0x20b9b6f { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x65cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x20b970f; continue;
 }
 0x20b9b8a => {
  { cpu.sub32(cpu.r32(0x22d2098u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b9bac; continue; }
  pc = 0x20b9b93; continue;
 }
 0x20b9b93 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9bac; continue;
 }
 0x20b9bac => {
  { cpu.sub32(cpu.r32(0x22d2098u32), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9643; continue; }
  pc = 0x20b9bb9; continue;
 }
 0x20b9bb9 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9643; continue; }
  pc = 0x20b9bd4; continue;
 }
 0x20b9bd4 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x20b9bf7; continue; }
  pc = 0x20b9be9; continue;
 }
 0x20b9be9 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9bf7; continue;
 }
 0x20b9bf7 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x20b9643; continue; }
  pc = 0x20b9c06; continue;
 }
 0x20b9c06 => {
  { let v = 0x22d2084u32; cpu.push(v); }
  { let v = 0x22d2080u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b9c16); f_020a7f40(cpu); if cpu.returned_to != 0x20b9c16 { return; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d2080u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d2084u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x20b9c34); cpu.call(t); if cpu.returned_to != 0x20b9c34 { return; }
  cpu.poll(); pc = 0x20b9643; continue;
 }
 0x20b9c39 => {
  { cpu.sub32(cpu.r32(0x22d2098u32), 0x0u32); }
  if !cpu.zf { pc = 0x20b9d01; continue; }
  pc = 0x20b9c46; continue;
 }
 0x20b9c46 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b9cab; continue; }
  pc = 0x20b9c4e; continue;
 }
 0x20b9c4e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9c5c; continue;
 }
 0x20b9c5c => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x20b9c97; continue; }
  pc = 0x20b9c6b; continue;
 }
 0x20b9c6b => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b9c84; continue; }
  pc = 0x20b9c7c; continue;
 }
 0x20b9c7c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9c8a; continue;
 }
 0x20b9c84 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9c8a; continue;
 }
 0x20b9c8a => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9c5c; continue; }
  pc = 0x20b9c97; continue;
 }
 0x20b9c97 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x38u32.wrapping_add(cpu.eax)), 0x7u8); }
  if !cpu.zf { pc = 0x20b9cab; continue; }
  pc = 0x20b9ca2; continue;
 }
 0x20b9ca2 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b9ca8); f_020b9d90(cpu); if cpu.returned_to != 0x20b9ca8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20b9cab; continue;
 }
 0x20b9cab => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20b9d01; continue; }
  pc = 0x20b9cb8; continue;
 }
 0x20b9cb8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9cc6; continue;
 }
 0x20b9cc6 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x20b9d01; continue; }
  pc = 0x20b9cd5; continue;
 }
 0x20b9cd5 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b9cee; continue; }
  pc = 0x20b9ce6; continue;
 }
 0x20b9ce6 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9cf4; continue;
 }
 0x20b9cee => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9cf4; continue;
 }
 0x20b9cf4 => {
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9cc6; continue; }
  pc = 0x20b9d01; continue;
 }
 0x20b9d01 => {
  { let v: u32 = cpu.r32(0x22d2088u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x22d207cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x71cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d2088u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d207cu32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x20b961a; continue; }
  pc = 0x20b9d25; continue;
 }
 0x20b9d25 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b9d72; continue; }
  pc = 0x20b9d32; continue;
 }
 0x20b9d32 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20b9d57); f_020ad040(cpu); if cpu.returned_to != 0x20b9d57 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20b9d72; continue;
 }
 0x20b9d72 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217c81cu32, v); }
  pc = 0x20b9d7a; continue;
 }
 0x20b9d7a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b9d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b9d90); }
 let mut pc: u32 = 0x20b9d90;
 loop { match pc {
 0x20b9d90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b9d9b; continue;
 }
 0x20b9d9b => {
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9db9; continue; }
  pc = 0x20b9da6; continue;
 }
 0x20b9da6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20b9db6); f_020ba250(cpu); if cpu.returned_to != 0x20b9db6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9db9; continue;
 }
 0x20b9db9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9d9b; continue; }
  pc = 0x20b9dc2; continue;
 }
 0x20b9dc2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b9dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b9dd0); }
 let mut pc: u32 = 0x20b9dd0;
 loop { match pc {
 0x20b9dd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b9ddb); f_020b9df0(cpu); if cpu.returned_to != 0x20b9ddb { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b9df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b9df0); }
 let mut pc: u32 = 0x20b9df0;
 loop { match pc {
 0x20b9df0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b9dfb; continue;
 }
 0x20b9dfb => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b9dfb; continue; }
  pc = 0x20b9e0a; continue;
 }
 0x20b9e0a => {
  { let v: u32 = 0x480u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x20b9e35; continue;
 }
 0x20b9e35 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9e35; continue; }
  pc = 0x20b9e46; continue;
 }
 0x20b9e46 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x20b9e4e; continue;
 }
 0x20b9e4e => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b9e4e; continue; }
  pc = 0x20b9e5f; continue;
 }
 0x20b9e5f => {
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x68u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x7a1200u32; cpu.w32(0x74u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b9eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b9eb0); }
 let mut pc: u32 = 0x20b9eb0;
 loop { match pc {
 0x20b9eb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9ee3; continue; }
  pc = 0x20b9ed0; continue;
 }
 0x20b9ed0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9ee0); f_020ba250(cpu); if cpu.returned_to != 0x20b9ee0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9ee3; continue;
 }
 0x20b9ee3 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xd8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9f0a; continue; }
  pc = 0x20b9ef7; continue;
 }
 0x20b9ef7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9f07); f_020ba250(cpu); if cpu.returned_to != 0x20b9f07 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9f0a; continue;
 }
 0x20b9f0a => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9f36; continue; }
  pc = 0x20b9f1e; continue;
 }
 0x20b9f1e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9f33); f_020ba250(cpu); if cpu.returned_to != 0x20b9f33 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9f36; continue;
 }
 0x20b9f36 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9f5d; continue; }
  pc = 0x20b9f4a; continue;
 }
 0x20b9f4a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9f5a); f_020ba250(cpu); if cpu.returned_to != 0x20b9f5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9f5d; continue;
 }
 0x20b9f5d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9f84; continue; }
  pc = 0x20b9f71; continue;
 }
 0x20b9f71 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6fu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9f81); f_020ba250(cpu); if cpu.returned_to != 0x20b9f81 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9f84; continue;
 }
 0x20b9f84 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x258u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9fab; continue; }
  pc = 0x20b9f98; continue;
 }
 0x20b9f98 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9fa8); f_020ba250(cpu); if cpu.returned_to != 0x20b9fa8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9fab; continue;
 }
 0x20b9fab => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x358u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9fd2; continue; }
  pc = 0x20b9fbf; continue;
 }
 0x20b9fbf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9fcf); f_020ba250(cpu); if cpu.returned_to != 0x20b9fcf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9fd2; continue;
 }
 0x20b9fd2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b9ff9; continue; }
  pc = 0x20b9fe6; continue;
 }
 0x20b9fe6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20b9ff6); f_020ba250(cpu); if cpu.returned_to != 0x20b9ff6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20b9ff9; continue;
 }
 0x20b9ff9 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba020; continue; }
  pc = 0x20ba00d; continue;
 }
 0x20ba00d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba01d); f_020ba250(cpu); if cpu.returned_to != 0x20ba01d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba020; continue;
 }
 0x20ba020 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x418u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba047; continue; }
  pc = 0x20ba034; continue;
 }
 0x20ba034 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba044); f_020ba250(cpu); if cpu.returned_to != 0x20ba044 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba047; continue;
 }
 0x20ba047 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba06e; continue; }
  pc = 0x20ba05b; continue;
 }
 0x20ba05b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba06b); f_020ba250(cpu); if cpu.returned_to != 0x20ba06b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba06e; continue;
 }
 0x20ba06e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x498u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba095; continue; }
  pc = 0x20ba082; continue;
 }
 0x20ba082 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba092); f_020ba250(cpu); if cpu.returned_to != 0x20ba092 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba095; continue;
 }
 0x20ba095 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba0bc; continue; }
  pc = 0x20ba0a9; continue;
 }
 0x20ba0a9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5du32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba0b9); f_020ba250(cpu); if cpu.returned_to != 0x20ba0b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba0bc; continue;
 }
 0x20ba0bc => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba0e2; continue; }
  pc = 0x20ba0d0; continue;
 }
 0x20ba0d0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba0df); f_020ba250(cpu); if cpu.returned_to != 0x20ba0df { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  pc = 0x20ba0e2; continue;
 }
 0x20ba0e2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ba0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ba0f0); }
 let mut pc: u32 = 0x20ba0f0;
 loop { match pc {
 0x20ba0f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x20ba107; continue;
 }
 0x20ba107 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x20ba132; continue; }
  pc = 0x20ba111; continue;
 }
 0x20ba111 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba125); f_020ba250(cpu); if cpu.returned_to != 0x20ba125 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20ba132; continue;
 }
 0x20ba132 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x20ba107; continue; }
  pc = 0x20ba139; continue;
 }
 0x20ba139 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20ba17d; continue; }
  pc = 0x20ba145; continue;
 }
 0x20ba145 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ba166); f_020ad040(cpu); if cpu.returned_to != 0x20ba166 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20ba17d; continue;
 }
 0x20ba17d => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ba190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ba190); }
 let mut pc: u32 = 0x20ba190;
 loop { match pc {
 0x20ba190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  pc = 0x20ba1a5; continue;
 }
 0x20ba1a5 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba1d2; continue; }
  pc = 0x20ba1b0; continue;
 }
 0x20ba1b0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x20ba1c5); f_020ba250(cpu); if cpu.returned_to != 0x20ba1c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20ba1d2; continue;
 }
 0x20ba1d2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x20ba1a5; continue; }
  pc = 0x20ba1d9; continue;
 }
 0x20ba1d9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20ba227; continue; }
  pc = 0x20ba1ef; continue;
 }
 0x20ba1ef => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ba210); f_020ad040(cpu); if cpu.returned_to != 0x20ba210 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20ba227; continue;
 }
 0x20ba227 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20ba23e; continue; }
  pc = 0x20ba22b; continue;
 }
 0x20ba22b => {
  cpu.push(0x20ba230); f_020acf80(cpu); if cpu.returned_to != 0x20ba230 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ba23e; continue; }
  pc = 0x20ba234; continue;
 }
 0x20ba234 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x20ba23b); f_020acf90(cpu); if cpu.returned_to != 0x20ba23b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20ba23e; continue;
 }
 0x20ba23e => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ba250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ba250); }
 let mut pc: u32 = 0x20ba250;
 loop { match pc {
 0x20ba250 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xf0u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0xb0u32); }
  if cpu.zf { pc = 0x20ba2a4; continue; }
  pc = 0x20ba294; continue;
 }
 0x20ba294 => {
  { cpu.sub32(cpu.edx, 0xc0u32); }
  if cpu.zf { pc = 0x20ba2a4; continue; }
  pc = 0x20ba29c; continue;
 }
 0x20ba29c => {
  { cpu.sub32(cpu.edx, 0xe0u32); }
  if !cpu.zf { pc = 0x20ba2c0; continue; }
  pc = 0x20ba2a4; continue;
 }
 0x20ba2a4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ba2bd); f_020ba6c0(cpu); if cpu.returned_to != 0x20ba2bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20ba2c0; continue;
 }
 0x20ba2c0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xb0u32); }
  if !cpu.zf { pc = 0x20ba3f7; continue; }
  pc = 0x20ba2ce; continue;
 }
 0x20ba2ce => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20ba2f5; continue; }
  pc = 0x20ba2d5; continue;
 }
 0x20ba2d5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba2f5; continue; }
  pc = 0x20ba2e9; continue;
 }
 0x20ba2e9 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2d8u32.wrapping_add(cpu.eax), v); }
  pc = 0x20ba2f5; continue;
 }
 0x20ba2f5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, 0x6fu32); }
  if cpu.cf { pc = 0x20ba34c; continue; }
  pc = 0x20ba314; continue;
 }
 0x20ba314 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba580; continue; }
  pc = 0x20ba31a; continue;
 }
 0x20ba31a => {
  { cpu.sub32(cpu.ebp, 0x75u32); }
  if cpu.cf { pc = 0x20ba33c; continue; }
  pc = 0x20ba31f; continue;
 }
 0x20ba31f => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba4f9; continue; }
  pc = 0x20ba325; continue;
 }
 0x20ba325 => {
  { cpu.sub32(cpu.ebp, 0x76u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20ba468; continue; }
  pc = 0x20ba32e; continue;
 }
 0x20ba32e => {
  { cpu.sub32(cpu.ebp, 0x77u32); }
  if cpu.zf { pc = 0x20ba4a8; continue; }
  pc = 0x20ba337; continue;
 }
 0x20ba337 => {
  pc = 0x20ba3f7; continue;
 }
 0x20ba33c => {
  { cpu.sub32(cpu.ebp, 0x73u32); }
  if cpu.cf { pc = 0x20ba3f7; continue; }
  pc = 0x20ba345; continue;
 }
 0x20ba345 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba37a; continue; }
  pc = 0x20ba347; continue;
 }
 0x20ba347 => {
  pc = 0x20ba4c4; continue;
 }
 0x20ba34c => {
  { cpu.sub32(cpu.ebp, 0x6cu32); }
  if cpu.cf { pc = 0x20ba361; continue; }
  pc = 0x20ba351; continue;
 }
 0x20ba351 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba393; continue; }
  pc = 0x20ba353; continue;
 }
 0x20ba353 => {
  { cpu.sub32(cpu.ebp, 0x6du32); }
  if (cpu.cf || cpu.zf) { pc = 0x20ba56e; continue; }
  pc = 0x20ba35c; continue;
 }
 0x20ba35c => {
  pc = 0x20ba5ad; continue;
 }
 0x20ba361 => {
  { cpu.sub32(cpu.ebp, 0x6u32); }
  if cpu.cf { pc = 0x20ba3f7; continue; }
  pc = 0x20ba36a; continue;
 }
 0x20ba36a => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba3b5; continue; }
  pc = 0x20ba36c; continue;
 }
 0x20ba36c => {
  { cpu.sub32(cpu.ebp, 0x7u32); }
  if cpu.zf { pc = 0x20ba43b; continue; }
  pc = 0x20ba375; continue;
 }
 0x20ba375 => {
  pc = 0x20ba3f7; continue;
 }
 0x20ba37a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x20ba3f7; continue;
 }
 0x20ba393 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20ba3f7; continue; }
  pc = 0x20ba39d; continue;
 }
 0x20ba39d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.push(0x20ba3a7); cpu.call(t); if cpu.returned_to != 0x20ba3a7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x20ba3f7; continue;
 }
 0x20ba3b5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba3c8); f_020ba250(cpu); if cpu.returned_to != 0x20ba3c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x65u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba3de); f_020ba250(cpu); if cpu.returned_to != 0x20ba3de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x26u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba3f4); f_020ba250(cpu); if cpu.returned_to != 0x20ba3f4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x20ba3f7; continue;
 }
 0x20ba3f7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x20ba413; continue; }
  pc = 0x20ba40a; continue;
 }
 0x20ba40a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x64u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba413; continue;
 }
 0x20ba413 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x90u32); }
  if !cpu.zf { pc = 0x20ba632; continue; }
  pc = 0x20ba430; continue;
 }
 0x20ba430 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x20ba640; continue;
 }
 0x20ba43b => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x1b4u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v: u32 = 0x3f01u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20ba460; continue; }
  pc = 0x20ba45b; continue;
 }
 0x20ba45b => {
  { let v: u32 = 0x7fu32; cpu.edi = v; }
  pc = 0x20ba460; continue;
 }
 0x20ba460 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20ba3f7; continue; }
  pc = 0x20ba464; continue;
 }
 0x20ba464 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.poll(); pc = 0x20ba3f7; continue;
 }
 0x20ba468 => {
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba493; continue;
 }
 0x20ba493 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.push(0x20ba4a0); cpu.call(t); if cpu.returned_to != 0x20ba4a0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba4a8 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba4b2; continue;
 }
 0x20ba4b2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x20ba4bc); cpu.call(t); if cpu.returned_to != 0x20ba4bc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba4c4 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ba4c8; continue;
 }
 0x20ba4c8 => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba4da; continue; }
  pc = 0x20ba4d1; continue;
 }
 0x20ba4d1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20ba4c8; continue; }
  pc = 0x20ba4da; continue;
 }
 0x20ba4da => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba4e3; continue;
 }
 0x20ba4e3 => {
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x78u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba4f9 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20ba6af; continue; }
  pc = 0x20ba502; continue;
 }
 0x20ba502 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x20ba50a; continue;
 }
 0x20ba50a => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20ba51b; continue; }
  pc = 0x20ba513; continue;
 }
 0x20ba513 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20ba50a; continue; }
  pc = 0x20ba51b; continue;
 }
 0x20ba51b => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba524; continue;
 }
 0x20ba524 => {
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20ba541; continue; }
  pc = 0x20ba533; continue;
 }
 0x20ba533 => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba541 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20ba55c; continue; }
  pc = 0x20ba54e; continue;
 }
 0x20ba54e => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba55c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba56e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba575); f_020a7e80(cpu); if cpu.returned_to != 0x20ba575 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba580 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba58a; continue;
 }
 0x20ba58a => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x20ba59e; continue; }
  pc = 0x20ba58f; continue;
 }
 0x20ba58f => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba59e => {
  { let v: u32 = 0x2u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba5ad => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x20ba5f5; continue; }
  pc = 0x20ba5b7; continue;
 }
 0x20ba5b7 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba5c1; continue;
 }
 0x20ba5c1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ba5cb); f_020a7480(cpu); if cpu.returned_to != 0x20ba5cb { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba5d8; continue;
 }
 0x20ba5d8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba5e0); f_020a72f0(cpu); if cpu.returned_to != 0x20ba5e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x60u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba5f5 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba601; continue;
 }
 0x20ba601 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba60c); f_020ba0f0(cpu); if cpu.returned_to != 0x20ba60c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20ba61c); f_020a73c0(cpu); if cpu.returned_to != 0x20ba61c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ba627); f_020a72f0(cpu); if cpu.returned_to != 0x20ba627 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba632 => {
  { cpu.sub32(cpu.edx, 0x80u32); }
  if !cpu.zf { pc = 0x20ba640; continue; }
  pc = 0x20ba63a; continue;
 }
 0x20ba63a => {
  { let v: u32 = cpu.dec32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x20ba640; continue;
 }
 0x20ba640 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xe4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if !cpu.zf { pc = 0x20ba66d; continue; }
  pc = 0x20ba65f; continue;
 }
 0x20ba65f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x218u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x20ba6af; continue; }
  pc = 0x20ba66d; continue;
 }
 0x20ba66d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x20ba695; continue; }
  pc = 0x20ba67a; continue;
 }
 0x20ba67a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.push(0x20ba691); cpu.call(t); if cpu.returned_to != 0x20ba691 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ba6af; continue; }
  pc = 0x20ba695; continue;
 }
 0x20ba695 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ba6ac); f_020bad00(cpu); if cpu.returned_to != 0x20ba6ac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20ba6af; continue;
 }
 0x20ba6af => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ba6c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ba6c0); }
 let mut pc: u32 = 0x20ba6c0;
 loop { match pc {
 0x20ba6c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xf0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, 0xc0u32); }
  if cpu.cf { pc = 0x20ba6fd; continue; }
  pc = 0x20ba6ef; continue;
 }
 0x20ba6ef => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba709; continue; }
  pc = 0x20ba6f1; continue;
 }
 0x20ba6f1 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x20ba714; continue; }
  pc = 0x20ba6f9; continue;
 }
 0x20ba6f9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba6fd => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x20ba72a; continue; }
  pc = 0x20ba705; continue;
 }
 0x20ba705 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba709 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba714 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba72a => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x20ba77b; continue; }
  pc = 0x20ba72f; continue;
 }
 0x20ba72f => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba887; continue; }
  pc = 0x20ba735; continue;
 }
 0x20ba735 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x20ba765; continue; }
  pc = 0x20ba73a; continue;
 }
 0x20ba73a => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba7f6; continue; }
  pc = 0x20ba740; continue;
 }
 0x20ba740 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x20ba758; continue; }
  pc = 0x20ba745; continue;
 }
 0x20ba745 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba812; continue; }
  pc = 0x20ba74b; continue;
 }
 0x20ba74b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x20ba820; continue; }
  pc = 0x20ba754; continue;
 }
 0x20ba754 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba758 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x20ba804; continue; }
  pc = 0x20ba761; continue;
 }
 0x20ba761 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba765 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x20ba76e; continue; }
  pc = 0x20ba76a; continue;
 }
 0x20ba76a => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba7cc; continue; }
  pc = 0x20ba76c; continue;
 }
 0x20ba76c => {
  pc = 0x20ba7da; continue;
 }
 0x20ba76e => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x20ba7e8; continue; }
  pc = 0x20ba777; continue;
 }
 0x20ba777 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba77b => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x20ba7ab; continue; }
  pc = 0x20ba780; continue;
 }
 0x20ba780 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba84c; continue; }
  pc = 0x20ba786; continue;
 }
 0x20ba786 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x20ba79e; continue; }
  pc = 0x20ba78b; continue;
 }
 0x20ba78b => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba86a; continue; }
  pc = 0x20ba791; continue;
 }
 0x20ba791 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x20ba879; continue; }
  pc = 0x20ba79a; continue;
 }
 0x20ba79a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba79e => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x20ba85b; continue; }
  pc = 0x20ba7a7; continue;
 }
 0x20ba7a7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7ab => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x20ba7c3; continue; }
  pc = 0x20ba7b0; continue;
 }
 0x20ba7b0 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba895; continue; }
  pc = 0x20ba7b6; continue;
 }
 0x20ba7b6 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x20ba83d; continue; }
  pc = 0x20ba7bf; continue;
 }
 0x20ba7bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7c3 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20ba82e; continue; }
  pc = 0x20ba7c8; continue;
 }
 0x20ba7c8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7cc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7da => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7e8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba7f6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba804 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba812 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba820 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba82e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba83d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba84c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba85b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba86a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba879 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba887 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba895 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ba8b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ba8b0); }
 let mut pc: u32 = 0x20ba8b0;
 loop { match pc {
 0x20ba8b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.cf { pc = 0x20ba8e9; continue; }
  pc = 0x20ba8da; continue;
 }
 0x20ba8da => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba8f6; continue; }
  pc = 0x20ba8dc; continue;
 }
 0x20ba8dc => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x20ba8fb; continue; }
  pc = 0x20ba8e4; continue;
 }
 0x20ba8e4 => {
  pc = 0x20baa5a; continue;
 }
 0x20ba8e9 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x20ba90c; continue; }
  pc = 0x20ba8f1; continue;
 }
 0x20ba8f1 => {
  pc = 0x20baa5a; continue;
 }
 0x20ba8f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba8fb => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba90c => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x20ba970; continue; }
  pc = 0x20ba911; continue;
 }
 0x20ba911 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baa47; continue; }
  pc = 0x20ba917; continue;
 }
 0x20ba917 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x20ba94f; continue; }
  pc = 0x20ba91c; continue;
 }
 0x20ba91c => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba9e8; continue; }
  pc = 0x20ba922; continue;
 }
 0x20ba922 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x20ba93e; continue; }
  pc = 0x20ba927; continue;
 }
 0x20ba927 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba9fa; continue; }
  pc = 0x20ba92d; continue;
 }
 0x20ba92d => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x20baa03; continue; }
  pc = 0x20ba936; continue;
 }
 0x20ba936 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba93e => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x20ba9f1; continue; }
  pc = 0x20ba947; continue;
 }
 0x20ba947 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba94f => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x20ba95f; continue; }
  pc = 0x20ba954; continue;
 }
 0x20ba954 => {
  if (cpu.cf || cpu.zf) { pc = 0x20ba9cd; continue; }
  pc = 0x20ba95a; continue;
 }
 0x20ba95a => {
  pc = 0x20ba9d6; continue;
 }
 0x20ba95f => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x20ba9df; continue; }
  pc = 0x20ba968; continue;
 }
 0x20ba968 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba970 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x20ba9a8; continue; }
  pc = 0x20ba975; continue;
 }
 0x20ba975 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baa20; continue; }
  pc = 0x20ba97b; continue;
 }
 0x20ba97b => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x20ba997; continue; }
  pc = 0x20ba980; continue;
 }
 0x20ba980 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baa34; continue; }
  pc = 0x20ba986; continue;
 }
 0x20ba986 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x20baa3e; continue; }
  pc = 0x20ba98f; continue;
 }
 0x20ba98f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba997 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x20baa2a; continue; }
  pc = 0x20ba9a0; continue;
 }
 0x20ba9a0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9a8 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x20ba9c0; continue; }
  pc = 0x20ba9ad; continue;
 }
 0x20ba9ad => {
  if (cpu.cf || cpu.zf) { pc = 0x20baa50; continue; }
  pc = 0x20ba9b3; continue;
 }
 0x20ba9b3 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x20baa16; continue; }
  pc = 0x20ba9b8; continue;
 }
 0x20ba9b8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9c0 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20baa0c; continue; }
  pc = 0x20ba9c5; continue;
 }
 0x20ba9c5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9cd => {
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9d6 => {
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9df => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9e8 => {
  { let v: u32 = cpu.r32(0x180u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9f1 => {
  { let v: u32 = cpu.r32(0x1c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ba9fa => {
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa03 => {
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa0c => {
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa16 => {
  { let v: u32 = cpu.r32(0x2c0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa20 => {
  { let v: u32 = cpu.r32(0x300u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa2a => {
  { let v: u32 = cpu.r32(0x340u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa34 => {
  { let v: u32 = cpu.r32(0x380u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa3e => {
  { let v: u32 = cpu.r32(0x3c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa47 => {
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa50 => {
  { let v: u32 = cpu.r32(0x440u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baa5a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020baa70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20baa70); }
 let mut pc: u32 = 0x20baa70;
 loop { match pc {
 0x20baa70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x20baa81; continue;
 }
 0x20baa81 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x216afc0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20baa99; continue; }
  pc = 0x20baa97; continue;
 }
 0x20baa97 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20baa99; continue;
 }
 0x20baa99 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20baabb; continue; }
  pc = 0x20baa9d; continue;
 }
 0x20baa9d => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216afc8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20baab3; continue; }
  pc = 0x20baab1; continue;
 }
 0x20baab1 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20baab3; continue;
 }
 0x20baab3 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20babae; continue; }
  pc = 0x20baabb; continue;
 }
 0x20baabb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp & 0xffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi & 0xff00u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 24u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xff0000u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x216afd0u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bab07; continue; }
  pc = 0x20bab05; continue;
 }
 0x20bab05 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bab07; continue;
 }
 0x20bab07 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20baa81; continue; }
  pc = 0x20bab0f; continue;
 }
 0x20bab0f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216afc0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bab25; continue; }
  pc = 0x20bab23; continue;
 }
 0x20bab23 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bab25; continue;
 }
 0x20bab25 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bab38; continue; }
  pc = 0x20bab29; continue;
 }
 0x20bab29 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20babae; continue; }
  pc = 0x20bab31; continue;
 }
 0x20bab31 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20babb0; continue;
 }
 0x20bab38 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x20babae; continue; }
  pc = 0x20bab45; continue;
 }
 0x20bab45 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216afd0u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bab5c; continue; }
  pc = 0x20bab5a; continue;
 }
 0x20bab5a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bab5c; continue;
 }
 0x20bab5c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bab70; continue; }
  pc = 0x20bab60; continue;
 }
 0x20bab60 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20bab70; continue; }
  pc = 0x20bab66; continue;
 }
 0x20bab66 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bab70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp & 0xffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi & 0xff00u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 24u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xff0000u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.cf { cpu.poll(); pc = 0x20bab45; continue; }
  pc = 0x20babae; continue;
 }
 0x20babae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20babb0; continue;
 }
 0x20babb0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020babc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20babc0); }
 let mut pc: u32 = 0x20babc0;
 loop { match pc {
 0x20babc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bac00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bac00); }
 let mut pc: u32 = 0x20bac00;
 loop { match pc {
 0x20bac00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x20bac0e; continue;
 }
 0x20bac0e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx | cpu.esi; cpu.ecx = v; }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x20bac29; continue; }
  pc = 0x20bac26; continue;
 }
 0x20bac26 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20bac0e; continue; }
  pc = 0x20bac29; continue;
 }
 0x20bac29 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bac30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bac30); }
 let mut pc: u32 = 0x20bac30;
 loop { match pc {
 0x20bac30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bac80; continue; }
  pc = 0x20bac48; continue;
 }
 0x20bac48 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bac69); f_020ad040(cpu); if cpu.returned_to != 0x20bac69 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bac80; continue;
 }
 0x20bac80 => {
  { cpu.sub32(cpu.edi, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20bac8f; continue; }
  pc = 0x20bac88; continue;
 }
 0x20bac88 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x20bac91; continue;
 }
 0x20bac8f => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x20bac91; continue;
 }
 0x20bac91 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bacf5; continue; }
  pc = 0x20bacbd; continue;
 }
 0x20bacbd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bacde); f_020ad040(cpu); if cpu.returned_to != 0x20bacde { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bacf5; continue;
 }
 0x20bacf5 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bad00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bad00); }
 let mut pc: u32 = 0x20bad00;
 loop { match pc {
 0x20bad00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d200cu32); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20bad40; continue; }
  pc = 0x20bad22; continue;
 }
 0x20bad22 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x20bad3b; continue; }
  pc = 0x20bad30; continue;
 }
 0x20bad30 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if !cpu.zf { pc = 0x20bad40; continue; }
  pc = 0x20bad37; continue;
 }
 0x20bad37 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20bad40; continue; }
  pc = 0x20bad3b; continue;
 }
 0x20bad3b => {
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x20bad40; continue;
 }
 0x20bad40 => {
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  pc = 0x20bad58; continue;
 }
 0x20bad58 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20bae72; continue; }
  pc = 0x20bad62; continue;
 }
 0x20bad62 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.cf { pc = 0x20bad8a; continue; }
  pc = 0x20bad6a; continue;
 }
 0x20bad6a => {
  if (cpu.cf || cpu.zf) { pc = 0x20bada6; continue; }
  pc = 0x20bad6c; continue;
 }
 0x20bad6c => {
  { cpu.sub32(cpu.esi, 0xd0u32); }
  if cpu.cf { pc = 0x20bad80; continue; }
  pc = 0x20bad74; continue;
 }
 0x20bad74 => {
  if (cpu.cf || cpu.zf) { pc = 0x20badad; continue; }
  pc = 0x20bad76; continue;
 }
 0x20bad76 => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x20bada6; continue; }
  pc = 0x20bad7e; continue;
 }
 0x20bad7e => {
  pc = 0x20badb4; continue;
 }
 0x20bad80 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.zf { pc = 0x20badad; continue; }
  pc = 0x20bad88; continue;
 }
 0x20bad88 => {
  pc = 0x20badb4; continue;
 }
 0x20bad8a => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.cf { pc = 0x20bad9e; continue; }
  pc = 0x20bad92; continue;
 }
 0x20bad92 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bada6; continue; }
  pc = 0x20bad94; continue;
 }
 0x20bad94 => {
  { cpu.sub32(cpu.esi, 0xa0u32); }
  if cpu.zf { pc = 0x20bada6; continue; }
  pc = 0x20bad9c; continue;
 }
 0x20bad9c => {
  pc = 0x20badb4; continue;
 }
 0x20bad9e => {
  { cpu.sub32(cpu.esi, 0x80u32); }
  if !cpu.zf { pc = 0x20badb4; continue; }
  pc = 0x20bada6; continue;
 }
 0x20bada6 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x20badb6; continue;
 }
 0x20badad => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x20badb6; continue;
 }
 0x20badb4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20badb6; continue;
 }
 0x20badb6 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20bae08; continue; }
  pc = 0x20badc7; continue;
 }
 0x20badc7 => {
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bae08; continue; }
  pc = 0x20badd0; continue;
 }
 0x20badd0 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20badf1); f_020ad040(cpu); if cpu.returned_to != 0x20badf1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bae08; continue;
 }
 0x20bae08 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x20bae67; continue; }
  pc = 0x20bae49; continue;
 }
 0x20bae49 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  pc = 0x20bae67; continue;
 }
 0x20bae67 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx))); cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x20bad58; continue;
 }
 0x20bae72 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bae80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bae80); }
 let mut pc: u32 = 0x20bae80;
 loop { match pc {
 0x20bae80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20baec9; continue; }
  pc = 0x20bae91; continue;
 }
 0x20bae91 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20baeb2); f_020ad040(cpu); if cpu.returned_to != 0x20baeb2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20baec9; continue;
 }
 0x20baec9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020baed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20baed0); }
 let mut pc: u32 = 0x20baed0;
 loop { match pc {
 0x20baed0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x20baeff; continue; }
  pc = 0x20baee0; continue;
 }
 0x20baee0 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baf19; continue; }
  pc = 0x20baee2; continue;
 }
 0x20baee2 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x20baef5; continue; }
  pc = 0x20baee9; continue;
 }
 0x20baee9 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baf1f; continue; }
  pc = 0x20baeeb; continue;
 }
 0x20baeeb => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x20baf19; continue; }
  pc = 0x20baef2; continue;
 }
 0x20baef2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baef5 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x20baf1f; continue; }
  pc = 0x20baefc; continue;
 }
 0x20baefc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baeff => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x20baf12; continue; }
  pc = 0x20baf06; continue;
 }
 0x20baf06 => {
  if (cpu.cf || cpu.zf) { pc = 0x20baf19; continue; }
  pc = 0x20baf08; continue;
 }
 0x20baf08 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x20baf19; continue; }
  pc = 0x20baf0f; continue;
 }
 0x20baf0f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baf12 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x20baf25; continue; }
  pc = 0x20baf19; continue;
 }
 0x20baf19 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baf1f => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20baf25 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020baf30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20baf30); }
 let mut pc: u32 = 0x20baf30;
 loop { match pc {
 0x20baf30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c820u32), 0x0u32); }
  if !cpu.zf { pc = 0x20bb041; continue; }
  pc = 0x20baf40; continue;
 }
 0x20baf40 => {
  { let v = 0x20b7750u32; cpu.push(v); }
  { let v = 0x20baf30u32; cpu.push(v); }
  cpu.push(0x20baf4f); f_020b35a0(cpu); if cpu.returned_to != 0x20baf4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x217c79cu32; cpu.push(v); }
  cpu.push(0x20baf61); f_020b34e0(cpu); if cpu.returned_to != 0x20baf61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c81cu32; cpu.push(v); }
  cpu.push(0x20baf70); f_020b34e0(cpu); if cpu.returned_to != 0x20baf70 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d207cu32; cpu.push(v); }
  cpu.push(0x20baf7f); f_020b34e0(cpu); if cpu.returned_to != 0x20baf7f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2080u32; cpu.push(v); }
  cpu.push(0x20baf8e); f_020b34e0(cpu); if cpu.returned_to != 0x20baf8e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2084u32; cpu.push(v); }
  cpu.push(0x20baf9d); f_020b34e0(cpu); if cpu.returned_to != 0x20baf9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2088u32; cpu.push(v); }
  cpu.push(0x20bafac); f_020b34e0(cpu); if cpu.returned_to != 0x20bafac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2098u32; cpu.push(v); }
  cpu.push(0x20bafbb); f_020b34e0(cpu); if cpu.returned_to != 0x20bafbb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d208cu32; cpu.push(v); }
  cpu.push(0x20bafca); f_020b34e0(cpu); if cpu.returned_to != 0x20bafca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2090u32; cpu.push(v); }
  cpu.push(0x20bafd9); f_020b34e0(cpu); if cpu.returned_to != 0x20bafd9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2078u32; cpu.push(v); }
  cpu.push(0x20bafe8); f_020b34e0(cpu); if cpu.returned_to != 0x20bafe8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2094u32; cpu.push(v); }
  cpu.push(0x20baff7); f_020b34e0(cpu); if cpu.returned_to != 0x20baff7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2074u32; cpu.push(v); }
  cpu.push(0x20bb006); f_020b34e0(cpu); if cpu.returned_to != 0x20bb006 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2070u32; cpu.push(v); }
  cpu.push(0x20bb015); f_020b34e0(cpu); if cpu.returned_to != 0x20bb015 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d2068u32; cpu.push(v); }
  cpu.push(0x20bb024); f_020b34e0(cpu); if cpu.returned_to != 0x20bb024 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d206cu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20bb038); f_020b34e0(cpu); if cpu.returned_to != 0x20bb038 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c820u32, v); }
  pc = 0x20bb041; continue;
 }
 0x20bb041 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bb050(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bb050); } r_020bb050(cpu, 0x20bb050); }
pub fn f_020bb250(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bb250); } r_020bb050(cpu, 0x20bb250); }
fn r_020bb050(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20bb050 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1e000000u32).wrapping_add((cpu.ebx as u8)); cpu.w8(0x1e000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x41000000u32).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x41000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x20bb0da; continue; }
  pc = 0x20bb0da; continue;
 }
 0x20bb0da => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.pf() { pc = 0x20bb0de; continue; }
  pc = 0x20bb0de; continue;
 }
 0x20bb0de => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x92000000u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x92000000u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xa4000000u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0xa4000000u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb9000000u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb9000000u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x15000001u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x15000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x37000001u32).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x37000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xd2000001u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xd2000001u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x20bb14c, "Outsb");
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xba000002u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0xba000002u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xdc000004u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0xdc000004u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5740000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(5); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (cpu.sf == cpu.of) { pc = 0x20bb198; continue; }
  pc = 0x20bb192; continue;
 }
 0x20bb192 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x20bb19c; continue; }
  pc = 0x20bb196; continue;
 }
 0x20bb196 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20bb198; continue;
 }
 0x20bb198 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20bb19c; continue;
 }
 0x20bb19c => {
  { let v: u8 = 0x7u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xa9000008u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb8000009u32).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb8000009u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4000000bu32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x4000000bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | 0xe910000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x20bb1cc, "Outsd");
  cpu.unimplemented(0x20bb1cd, "Sldt");
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x13u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17200000u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x2300001bu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x2300001bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1ede0000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xe3000024u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xe3000024u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ebp ^ cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20bb258; continue;
 }
 0x20bb250 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c830u32), 0x0u32); }
  if cpu.zf { pc = 0x20bb276; continue; }
  pc = 0x20bb25c; continue;
 }
 0x20bb258 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x20bb276; continue; }
  pc = 0x20bb25c; continue;
 }
 0x20bb25c => {
  { let v = 0x20bb250u32; cpu.push(v); }
  { let v = 0x20bbcc0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20bb26d); f_020b3500(cpu); if cpu.returned_to != 0x20bb26d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c830u32, v); }
  pc = 0x20bb276; continue;
 }
 0x20bb276 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bb280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bb280); }
 let mut pc: u32 = 0x20bb280;
 loop { match pc {
 0x20bb280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20bb2de; continue; }
  pc = 0x20bb28c; continue;
 }
 0x20bb28c => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb2b7; continue; }
  pc = 0x20bb298; continue;
 }
 0x20bb298 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x20bb29a; continue;
 }
 0x20bb29a => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x20bb2aa); f_020abb40(cpu); if cpu.returned_to != 0x20bb2aa { return; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20bb29a; continue; }
  pc = 0x20bb2b7; continue;
 }
 0x20bb2b7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bb2c3); f_020a7930(cpu); if cpu.returned_to != 0x20bb2c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bb2d2); f_020a7820(cpu); if cpu.returned_to != 0x20bb2d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bb2db); f_020b3710(cpu); if cpu.returned_to != 0x20bb2db { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20bb2de; continue;
 }
 0x20bb2de => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bb4f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bb4f0); }
 let mut pc: u32 = 0x20bb4f0;
 loop { match pc {
 0x20bb4f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xf0u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xfu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.zf { pc = 0x20bb526; continue; }
  pc = 0x20bb516; continue;
 }
 0x20bb516 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.zf { pc = 0x20bb526; continue; }
  pc = 0x20bb51e; continue;
 }
 0x20bb51e => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if !cpu.zf { pc = 0x20bb53d; continue; }
  pc = 0x20bb526; continue;
 }
 0x20bb526 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bb53a); f_020bbaf0(cpu); if cpu.returned_to != 0x20bb53a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x20bb53d; continue;
 }
 0x20bb53d => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if !cpu.zf { pc = 0x20bb551; continue; }
  pc = 0x20bb545; continue;
 }
 0x20bb545 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20bb551; continue; }
  pc = 0x20bb54c; continue;
 }
 0x20bb54c => {
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x20bb551; continue;
 }
 0x20bb551 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.cf { pc = 0x20bb587; continue; }
  pc = 0x20bb562; continue;
 }
 0x20bb562 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bb5aa; continue; }
  pc = 0x20bb564; continue;
 }
 0x20bb564 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.cf { pc = 0x20bb908; continue; }
  pc = 0x20bb570; continue;
 }
 0x20bb570 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bb6c8; continue; }
  pc = 0x20bb576; continue;
 }
 0x20bb576 => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if cpu.zf { pc = 0x20bb6e8; continue; }
  pc = 0x20bb582; continue;
 }
 0x20bb582 => {
  pc = 0x20bb908; continue;
 }
 0x20bb587 => {
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x20bb908; continue; }
  pc = 0x20bb593; continue;
 }
 0x20bb593 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bb8a5; continue; }
  pc = 0x20bb599; continue;
 }
 0x20bb599 => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if cpu.zf { pc = 0x20bb733; continue; }
  pc = 0x20bb5a5; continue;
 }
 0x20bb5a5 => {
  pc = 0x20bb908; continue;
 }
 0x20bb5aa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7u32); }
  if cpu.cf { pc = 0x20bb5d4; continue; }
  pc = 0x20bb5b3; continue;
 }
 0x20bb5b3 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bb5e1; continue; }
  pc = 0x20bb5b5; continue;
 }
 0x20bb5b5 => {
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x20bb908; continue; }
  pc = 0x20bb5be; continue;
 }
 0x20bb5be => {
  { cpu.sub32(cpu.edx, 0xbu32); }
  if (cpu.cf || cpu.zf) { pc = 0x20bb5e1; continue; }
  pc = 0x20bb5c3; continue;
 }
 0x20bb5c3 => {
  { cpu.sub32(cpu.edx, 0x7bu32); }
  if cpu.zf { pc = 0x20bb677; continue; }
  pc = 0x20bb5cc; continue;
 }
 0x20bb5cc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x20bb5d4 => {
  { cpu.sub32(cpu.edx, 0x6u32); }
  if cpu.zf { pc = 0x20bb62c; continue; }
  pc = 0x20bb5d9; continue;
 }
 0x20bb5d9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x20bb5e1 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb5f1; continue;
 }
 0x20bb5f1 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x20bb5f3; continue;
 }
 0x20bb5f3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x20bb618; continue; }
  pc = 0x20bb5fb; continue;
 }
 0x20bb5fb => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bb607); f_020ab040(cpu); if cpu.returned_to != 0x20bb607 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20bb618; continue; }
  pc = 0x20bb60f; continue;
 }
 0x20bb60f => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bb618); f_020bba50(cpu); if cpu.returned_to != 0x20bb618 { return; }
  pc = 0x20bb618; continue;
 }
 0x20bb618 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb62a; continue;
 }
 0x20bb62a => {
  cpu.poll(); pc = 0x20bb5f3; continue;
 }
 0x20bb62c => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb63c; continue;
 }
 0x20bb63c => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x20bb63e; continue;
 }
 0x20bb63e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x20bb663; continue; }
  pc = 0x20bb646; continue;
 }
 0x20bb646 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bb652); f_020ab040(cpu); if cpu.returned_to != 0x20bb652 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20bb663; continue; }
  pc = 0x20bb65a; continue;
 }
 0x20bb65a => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bb663); f_020bb940(cpu); if cpu.returned_to != 0x20bb663 { return; }
  pc = 0x20bb663; continue;
 }
 0x20bb663 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb675; continue;
 }
 0x20bb675 => {
  cpu.poll(); pc = 0x20bb63e; continue;
 }
 0x20bb677 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb687; continue;
 }
 0x20bb687 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20bb689; continue;
 }
 0x20bb689 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20bb6b4; continue; }
  pc = 0x20bb691; continue;
 }
 0x20bb691 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bb69d); f_020ab040(cpu); if cpu.returned_to != 0x20bb69d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20bb6b4; continue; }
  pc = 0x20bb6a5; continue;
 }
 0x20bb6a5 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bb6b1); f_020ab510(cpu); if cpu.returned_to != 0x20bb6b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20bb6b4; continue;
 }
 0x20bb6b4 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb6c6; continue;
 }
 0x20bb6c6 => {
  cpu.poll(); pc = 0x20bb689; continue;
 }
 0x20bb6c8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x20bb6da); f_020bbac0(cpu); if cpu.returned_to != 0x20bb6da { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x20bb6e8 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb6f8; continue;
 }
 0x20bb6f8 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x20bb6fa; continue;
 }
 0x20bb6fa => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x20bb71f; continue; }
  pc = 0x20bb702; continue;
 }
 0x20bb702 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bb70e); f_020ab040(cpu); if cpu.returned_to != 0x20bb70e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20bb71f; continue; }
  pc = 0x20bb716; continue;
 }
 0x20bb716 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bb71f); f_020bb940(cpu); if cpu.returned_to != 0x20bb71f { return; }
  pc = 0x20bb71f; continue;
 }
 0x20bb71f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb731; continue;
 }
 0x20bb731 => {
  cpu.poll(); pc = 0x20bb6fa; continue;
 }
 0x20bb733 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if !cpu.zf { pc = 0x20bb74f; continue; }
  pc = 0x20bb738; continue;
 }
 0x20bb738 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x20bb749); f_020bbac0(cpu); if cpu.returned_to != 0x20bb749 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bb74f; continue;
 }
 0x20bb74f => {
  { cpu.sub32(cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x20bb908; continue; }
  pc = 0x20bb75d; continue;
 }
 0x20bb75d => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb78d; continue; }
  pc = 0x20bb769; continue;
 }
 0x20bb769 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x20bb76b; continue;
 }
 0x20bb76b => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bb777); f_020ab040(cpu); if cpu.returned_to != 0x20bb777 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20bb78d; continue; }
  pc = 0x20bb77f; continue;
 }
 0x20bb77f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20bb76b; continue; }
  pc = 0x20bb78d; continue;
 }
 0x20bb78d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x20bb7d0; continue; }
  pc = 0x20bb795; continue;
 }
 0x20bb795 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb7c0; continue; }
  pc = 0x20bb7a2; continue;
 }
 0x20bb7a2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20bb7a4; continue;
 }
 0x20bb7a4 => {
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { pc = 0x20bb7b2; continue; }
  pc = 0x20bb7ae; continue;
 }
 0x20bb7ae => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x20bb7b2; continue;
 }
 0x20bb7b2 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20bb7a4; continue; }
  pc = 0x20bb7c0; continue;
 }
 0x20bb7c0 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20bb7cd); f_020ab510(cpu); if cpu.returned_to != 0x20bb7cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20bb7d0; continue;
 }
 0x20bb7d0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x558u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d8u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x618u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x698u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x698u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x658u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  cpu.push(0x20bb832); f_020aba90(cpu); if cpu.returned_to != 0x20bb832 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20bb851); f_020ab7d0(cpu); if cpu.returned_to != 0x20bb851 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bb875); f_020ab8a0(cpu); if cpu.returned_to != 0x20bb875 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bb881); f_020bb940(cpu); if cpu.returned_to != 0x20bb881 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bb88a); f_020bba50(cpu); if cpu.returned_to != 0x20bb88a { return; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20bb897); f_020ab720(cpu); if cpu.returned_to != 0x20bb897 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x20bb8a5 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x20bb908; continue; }
  pc = 0x20bb8aa; continue;
 }
 0x20bb8aa => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb908; continue; }
  pc = 0x20bb8b6; continue;
 }
 0x20bb8b6 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20bb8b8; continue;
 }
 0x20bb8b8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20bb8fa; continue; }
  pc = 0x20bb8c0; continue;
 }
 0x20bb8c0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x558u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20bb8fa; continue; }
  pc = 0x20bb8cc; continue;
 }
 0x20bb8cc => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bb8d8); f_020ab040(cpu); if cpu.returned_to != 0x20bb8d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20bb8fa; continue; }
  pc = 0x20bb8e0; continue;
 }
 0x20bb8e0 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bb8ec); f_020ab510(cpu); if cpu.returned_to != 0x20bb8ec { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x20bb8fa => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20bb8b8; continue; }
  pc = 0x20bb908; continue;
 }
 0x20bb908 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bb910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bb910); }
 let mut pc: u32 = 0x20bb910;
 loop { match pc {
 0x20bb910 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20bb92a); f_020bbac0(cpu); if cpu.returned_to != 0x20bb92a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bb940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bb940); }
 let mut pc: u32 = 0x20bb940;
 loop { match pc {
 0x20bb940 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x598u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb996; continue; }
  pc = 0x20bb98a; continue;
 }
 0x20bb98a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1fffu32); cpu.eax = v; }
  pc = 0x20bb9ac; continue;
 }
 0x20bb996 => {
  if (cpu.sf == cpu.of) { pc = 0x20bb9a8; continue; }
  pc = 0x20bb998; continue;
 }
 0x20bb998 => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x20bb9ac; continue;
 }
 0x20bb9a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20bb9ac; continue;
 }
 0x20bb9ac => {
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20bb9be; continue; }
  pc = 0x20bb9bc; continue;
 }
 0x20bb9bc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20bb9be; continue;
 }
 0x20bb9be => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20bb9c8; continue; }
  pc = 0x20bb9c3; continue;
 }
 0x20bb9c3 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x20bb9c8; continue;
 }
 0x20bb9c8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20bb050u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20bb050u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20bb050u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 13u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 13u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x20bba19; continue; }
  pc = 0x20bba02; continue;
 }
 0x20bba02 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20bba20; continue;
 }
 0x20bba19 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20bba20; continue;
 }
 0x20bba20 => {
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bba39); f_020ab450(cpu); if cpu.returned_to != 0x20bba39 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
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
pub fn f_020bba50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bba50); }
 let mut pc: u32 = 0x20bba50;
 loop { match pc {
 0x20bba50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2d4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x354u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x618u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bba9d); f_020ab390(cpu); if cpu.returned_to != 0x20bba9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x314u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bbab3); f_020ab2d0(cpu); if cpu.returned_to != 0x20bbab3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_020bbac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbac0); }
 let mut pc: u32 = 0x20bbac0;
 loop { match pc {
 0x20bbac0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20bbac6; continue;
 }
 0x20bbac6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20bbae2; continue; }
  pc = 0x20bbacd; continue;
 }
 0x20bbacd => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x20bbad6; continue; }
  pc = 0x20bbad1; continue;
 }
 0x20bbad1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20bbae4; continue; }
  pc = 0x20bbad6; continue;
 }
 0x20bbad6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20bbac6; continue; }
  pc = 0x20bbae2; continue;
 }
 0x20bbae2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20bbae4; continue;
 }
 0x20bbae4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbaf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbaf0); }
 let mut pc: u32 = 0x20bbaf0;
 loop { match pc {
 0x20bbaf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xf0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, 0xc0u32); }
  if cpu.cf { pc = 0x20bbb2d; continue; }
  pc = 0x20bbb1f; continue;
 }
 0x20bbb1f => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbb39; continue; }
  pc = 0x20bbb21; continue;
 }
 0x20bbb21 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x20bbb44; continue; }
  pc = 0x20bbb29; continue;
 }
 0x20bbb29 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb2d => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x20bbb5a; continue; }
  pc = 0x20bbb35; continue;
 }
 0x20bbb35 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb39 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb44 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb5a => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.cf { pc = 0x20bbbab; continue; }
  pc = 0x20bbb5f; continue;
 }
 0x20bbb5f => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc91; continue; }
  pc = 0x20bbb65; continue;
 }
 0x20bbb65 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x20bbb95; continue; }
  pc = 0x20bbb6a; continue;
 }
 0x20bbb6a => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc0f; continue; }
  pc = 0x20bbb70; continue;
 }
 0x20bbb70 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x20bbb88; continue; }
  pc = 0x20bbb75; continue;
 }
 0x20bbb75 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc2b; continue; }
  pc = 0x20bbb7b; continue;
 }
 0x20bbb7b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x20bbc39; continue; }
  pc = 0x20bbb84; continue;
 }
 0x20bbb84 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb88 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x20bbc1d; continue; }
  pc = 0x20bbb91; continue;
 }
 0x20bbb91 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbb95 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x20bbb9e; continue; }
  pc = 0x20bbb9a; continue;
 }
 0x20bbb9a => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbbf3; continue; }
  pc = 0x20bbb9c; continue;
 }
 0x20bbb9c => {
  pc = 0x20bbc01; continue;
 }
 0x20bbb9e => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.zf { pc = 0x20bbc9f; continue; }
  pc = 0x20bbba7; continue;
 }
 0x20bbba7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbbab => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.cf { pc = 0x20bbbdb; continue; }
  pc = 0x20bbbb0; continue;
 }
 0x20bbbb0 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc56; continue; }
  pc = 0x20bbbb6; continue;
 }
 0x20bbbb6 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.cf { pc = 0x20bbbce; continue; }
  pc = 0x20bbbbb; continue;
 }
 0x20bbbbb => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc74; continue; }
  pc = 0x20bbbc1; continue;
 }
 0x20bbbc1 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x20bbc83; continue; }
  pc = 0x20bbbca; continue;
 }
 0x20bbbca => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbbce => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x20bbc65; continue; }
  pc = 0x20bbbd7; continue;
 }
 0x20bbbd7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbbdb => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20bbcb8; continue; }
  pc = 0x20bbbe4; continue;
 }
 0x20bbbe4 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bbc47; continue; }
  pc = 0x20bbbe6; continue;
 }
 0x20bbbe6 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.zf { pc = 0x20bbcad; continue; }
  pc = 0x20bbbef; continue;
 }
 0x20bbbef => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbbf3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc01 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc0f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc1d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc2b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc39 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc47 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc56 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc65 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc74 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc83 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc91 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbc9f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbcad => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  pc = 0x20bbcb8; continue;
 }
 0x20bbcb8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbcc0); }
 let mut pc: u32 = 0x20bbcc0;
 loop { match pc {
 0x20bbcc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c830u32), 0x0u32); }
  if !cpu.zf { pc = 0x20bbce9; continue; }
  pc = 0x20bbccc; continue;
 }
 0x20bbccc => {
  { let v = 0x20bb250u32; cpu.push(v); }
  { let v = 0x20bbcc0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20bbce0); f_020b35a0(cpu); if cpu.returned_to != 0x20bbce0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c830u32, v); }
  pc = 0x20bbce9; continue;
 }
 0x20bbce9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbcf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbcf0); }
 let mut pc: u32 = 0x20bbcf0;
 loop { match pc {
 0x20bbcf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bbcfb); f_020a6a30(cpu); if cpu.returned_to != 0x20bbcfb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x20bbd1b; continue; }
  pc = 0x20bbd03; continue;
 }
 0x20bbd03 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20bbd18); f_020a6420(cpu); if cpu.returned_to != 0x20bbd18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20bbd1b; continue;
 }
 0x20bbd1b => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bbd21); f_020a6a30(cpu); if cpu.returned_to != 0x20bbd21 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbd30); }
 let mut pc: u32 = 0x20bbd30;
 loop { match pc {
 0x20bbd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbd3c); f_020a6a30(cpu); if cpu.returned_to != 0x20bbd3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20bbd70; continue; }
  pc = 0x20bbd44; continue;
 }
 0x20bbd44 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbd4a); f_020a6640(cpu); if cpu.returned_to != 0x20bbd4a { return; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20bbd70); f_020bc900(cpu); if cpu.returned_to != 0x20bbd70 { return; }
  pc = 0x20bbd70; continue;
 }
 0x20bbd70 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbd76); f_020a6a30(cpu); if cpu.returned_to != 0x20bbd76 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbd80); }
 let mut pc: u32 = 0x20bbd80;
 loop { match pc {
 0x20bbd80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbd8c); f_020a6a30(cpu); if cpu.returned_to != 0x20bbd8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bbdcc; continue; }
  pc = 0x20bbd93; continue;
 }
 0x20bbd93 => {
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20bbdb3); f_020bc900(cpu); if cpu.returned_to != 0x20bbdb3 { return; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20bbdcc); f_020bc900(cpu); if cpu.returned_to != 0x20bbdcc { return; }
  pc = 0x20bbdcc; continue;
 }
 0x20bbdcc => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbdd2); f_020a6a30(cpu); if cpu.returned_to != 0x20bbdd2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbde0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbde0); }
 let mut pc: u32 = 0x20bbde0;
 loop { match pc {
 0x20bbde0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbdf2); f_020a6a30(cpu); if cpu.returned_to != 0x20bbdf2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bbe5f; continue; }
  pc = 0x20bbdf9; continue;
 }
 0x20bbdf9 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x20bbe17; continue; }
  pc = 0x20bbdfe; continue;
 }
 0x20bbdfe => {
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20bbe17); f_020bc900(cpu); if cpu.returned_to != 0x20bbe17 { return; }
  pc = 0x20bbe17; continue;
 }
 0x20bbe17 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if cpu.zf { pc = 0x20bbe5f; continue; }
  pc = 0x20bbe1f; continue;
 }
 0x20bbe1f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x84u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20bbe43); f_020bc790(cpu); if cpu.returned_to != 0x20bbe43 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20bbe4d); f_020bc790(cpu); if cpu.returned_to != 0x20bbe4d { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20bbe5f); f_020bc900(cpu); if cpu.returned_to != 0x20bbe5f { return; }
  pc = 0x20bbe5f; continue;
 }
 0x20bbe5f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bbe65); f_020a6a30(cpu); if cpu.returned_to != 0x20bbe65 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbe70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bbe70); } r_020bbe70(cpu, 0x20bbe70); }
pub fn f_020bbea0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20bbea0); } r_020bbe70(cpu, 0x20bbea0); }
fn r_020bbe70(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20bbe70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bbe7a); f_020a6a30(cpu); if cpu.returned_to != 0x20bbe7a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20bbe92; continue; }
  pc = 0x20bbe82; continue;
 }
 0x20bbe82 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20bc330; continue;
 }
 0x20bbe92 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbea0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bbeaa); f_020a6a30(cpu); if cpu.returned_to != 0x20bbeaa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20bbebf; continue; }
  pc = 0x20bbeb2; continue;
 }
 0x20bbeb2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20bc330; continue;
 }
 0x20bbebf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bc330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u16 = 0xbu16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20bc389); f_020bc900(cpu); if cpu.returned_to != 0x20bc389 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc422; continue; }
  pc = 0x20bc391; continue;
 }
 0x20bc391 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20bc3f6; continue; }
  pc = 0x20bc395; continue;
 }
 0x20bc395 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r8(0xau32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x20bc422; continue;
 }
 0x20bc3f6 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bc422; continue;
 }
 0x20bc422 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_020bbf80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbf80); }
 let mut pc: u32 = 0x20bbf80;
 loop { match pc {
 0x20bbf80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bbf9a); f_020a6a30(cpu); if cpu.returned_to != 0x20bbf9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bbfab; continue; }
  pc = 0x20bbfa1; continue;
 }
 0x20bbfa1 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.cf { pc = 0x20bbfab; continue; }
  pc = 0x20bbfa6; continue;
 }
 0x20bbfa6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x20bbfc5; continue; }
  pc = 0x20bbfab; continue;
 }
 0x20bbfab => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20bbfb5; continue; }
  pc = 0x20bbfaf; continue;
 }
 0x20bbfaf => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x20bbfb5; continue;
 }
 0x20bbfb5 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20bbfdb; continue; }
  pc = 0x20bbfb9; continue;
 }
 0x20bbfb9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbfc5 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20bbfcf; continue; }
  pc = 0x20bbfc9; continue;
 }
 0x20bbfc9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x20bbfcf; continue;
 }
 0x20bbfcf => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20bbfdb; continue; }
  pc = 0x20bbfd3; continue;
 }
 0x20bbfd3 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bbfdb; continue;
 }
 0x20bbfdb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bbfe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bbfe0); }
 let mut pc: u32 = 0x20bbfe0;
 loop { match pc {
 0x20bbfe0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bbfea); f_020a6a30(cpu); if cpu.returned_to != 0x20bbfea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bbff2; continue; }
  pc = 0x20bbff1; continue;
 }
 0x20bbff1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bbff2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc000); }
 let mut pc: u32 = 0x20bc000;
 loop { match pc {
 0x20bc000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bc059; continue; }
  pc = 0x20bc009; continue;
 }
 0x20bc009 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x20bc010); f_020bc170(cpu); if cpu.returned_to != 0x20bc010 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc04b; continue; }
  pc = 0x20bc016; continue;
 }
 0x20bc016 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20bc031; continue; }
  pc = 0x20bc01c; continue;
 }
 0x20bc01c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x20bc026); f_020bc630(cpu); if cpu.returned_to != 0x20bc026 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc04b; continue; }
  pc = 0x20bc02a; continue;
 }
 0x20bc02a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bc031; continue;
 }
 0x20bc031 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x20bc03a; continue; }
  pc = 0x20bc036; continue;
 }
 0x20bc036 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bc03a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bc04b => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bc059; continue;
 }
 0x20bc059 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc060); }
 let mut pc: u32 = 0x20bc060;
 loop { match pc {
 0x20bc060 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bc07c; continue; }
  pc = 0x20bc069; continue;
 }
 0x20bc069 => {
  { let v: u32 = 0x5u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x20bc075); f_020bc430(cpu); if cpu.returned_to != 0x20bc075 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bc07c; continue;
 }
 0x20bc07c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc0a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc0a0); }
 let mut pc: u32 = 0x20bc0a0;
 loop { match pc {
 0x20bc0a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20bc0ea; continue; }
  pc = 0x20bc0aa; continue;
 }
 0x20bc0aa => {
  { let v: u32 = cpu.r32(0x217c834u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bc0dd; continue; }
  pc = 0x20bc0b4; continue;
 }
 0x20bc0b4 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217c834u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20bc0dd; continue; }
  pc = 0x20bc0c1; continue;
 }
 0x20bc0c1 => {
  { let v: u32 = cpu.r32(0x22d22a4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bc0da); f_020b3640(cpu); if cpu.returned_to != 0x20bc0da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20bc0dd; continue;
 }
 0x20bc0dd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bc0e7); f_020b3710(cpu); if cpu.returned_to != 0x20bc0e7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20bc0ea; continue;
 }
 0x20bc0ea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc0f0); }
 let mut pc: u32 = 0x20bc0f0;
 loop { match pc {
 0x20bc0f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0x1a8u32; cpu.push(v); }
  cpu.push(0x20bc102); f_020b3740(cpu); if cpu.returned_to != 0x20bc102 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc15a; continue; }
  pc = 0x20bc10b; continue;
 }
 0x20bc10b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20bc112); f_020bc4e0(cpu); if cpu.returned_to != 0x20bc112 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if !cpu.zf { pc = 0x20bc128; continue; }
  pc = 0x20bc11b; continue;
 }
 0x20bc11b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bc121); f_020b3710(cpu); if cpu.returned_to != 0x20bc121 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x20bc15a; continue;
 }
 0x20bc128 => {
  { let v: u32 = 0x1a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x20bc15a; continue;
 }
 0x20bc15a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc170); }
 let mut pc: u32 = 0x20bc170;
 loop { match pc {
 0x20bc170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u16 = 0x5u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20bc1c1); f_020bc900(cpu); if cpu.returned_to != 0x20bc1c1 { return; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x20bc1cf; continue; }
  pc = 0x20bc1c9; continue;
 }
 0x20bc1c9 => {
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x20bc1d3; continue; }
  pc = 0x20bc1cf; continue;
 }
 0x20bc1cf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20bc1df; continue;
 }
 0x20bc1d3 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20bc1df; continue;
 }
 0x20bc1df => {
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
pub fn f_020bc1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc1f0); }
 let mut pc: u32 = 0x20bc1f0;
 loop { match pc {
 0x20bc1f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20bc20a); f_020bc900(cpu); if cpu.returned_to != 0x20bc20a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc430); }
 let mut pc: u32 = 0x20bc430;
 loop { match pc {
 0x20bc430 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bc485); f_020bc900(cpu); if cpu.returned_to != 0x20bc485 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc4e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc4e0); }
 let mut pc: u32 = 0x20bc4e0;
 loop { match pc {
 0x20bc4e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xa4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x1500u32; cpu.edx = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u32 = 0x2fu32; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x74u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20bc559); cpu.call(t); if cpu.returned_to != 0x20bc559 { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x20bc569; continue; }
  pc = 0x20bc55f; continue;
 }
 0x20bc55f => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x20bc61c; continue;
 }
 0x20bc569 => {
  { cpu.sub32(cpu.r32(0x217c834u32), 0x0u32); }
  if !cpu.zf { pc = 0x20bc5a7; continue; }
  pc = 0x20bc572; continue;
 }
 0x20bc572 => {
  { let v = 0x22d22a4u32; cpu.push(v); }
  { let v = 0x22d229cu32; cpu.push(v); }
  { let v = 0x22d22a0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x20bc588); f_020b3680(cpu); if cpu.returned_to != 0x20bc588 { return; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20bc5a7; continue; }
  pc = 0x20bc595; continue;
 }
 0x20bc595 => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xa4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bc5a7 => {
  { let v: u32 = 0x150du32; cpu.ecx = v; }
  { let v: u32 = 0x300u32; cpu.ebp = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x2fu32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20bc602); cpu.call(t); if cpu.returned_to != 0x20bc602 { return; }
  { let v: u32 = cpu.r32(0x217c834u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c834u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20bc61c; continue;
 }
 0x20bc61c => {
  { let v: u32 = cpu.add32(cpu.esp, 0xa4u32); cpu.esp = v; }
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
pub fn f_020bc630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc630); }
 let mut pc: u32 = 0x20bc630;
 loop { match pc {
 0x20bc630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d22a0u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x21u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x20bc691); f_020bc900(cpu); if cpu.returned_to != 0x20bc691 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc753; continue; }
  pc = 0x20bc699; continue;
 }
 0x20bc699 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20bc720; continue;
 }
 0x20bc6c1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x22d229cu32); cpu.ebx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x20bc6f3); f_020bc900(cpu); if cpu.returned_to != 0x20bc6f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bc753; continue; }
  pc = 0x20bc6f7; continue;
 }
 0x20bc6f7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20bc709); f_020bc860(cpu); if cpu.returned_to != 0x20bc709 { return; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20bc720; continue;
 }
 0x20bc720 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x20bc6c1; continue; }
  pc = 0x20bc72d; continue;
 }
 0x20bc72d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x20bc742); f_020bc860(cpu); if cpu.returned_to != 0x20bc742 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20bc755; continue;
 }
 0x20bc753 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bc755; continue;
 }
 0x20bc755 => {
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
pub fn f_020bc760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc760); }
 let mut pc: u32 = 0x20bc760;
 loop { match pc {
 0x20bc760 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc790); }
 let mut pc: u32 = 0x20bc790;
 loop { match pc {
 0x20bc790 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xea60u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0xffffff6au32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc800); }
 let mut pc: u32 = 0x20bc800;
 loop { match pc {
 0x20bc800 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc860); }
 let mut pc: u32 = 0x20bc860;
 loop { match pc {
 0x20bc860 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc8d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc8d0); }
 let mut pc: u32 = 0x20bc8d0;
 loop { match pc {
 0x20bc8d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bc8f1); f_020bc900(cpu); if cpu.returned_to != 0x20bc8f1 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc900); }
 let mut pc: u32 = 0x20bc900;
 loop { match pc {
 0x20bc900 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x70u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x6cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x1510u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x3u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x54u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20bc9a2); cpu.call(t); if cpu.returned_to != 0x20bc9a2 { return; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0x80u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bc9c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bc9c0); }
 let mut pc: u32 = 0x20bc9c0;
 loop { match pc {
 0x20bc9c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2eu32.wrapping_add(cpu.edx), v); }
  let t = cpu.r32(0x217ba34u32); cpu.push(0x20bc9fa); cpu.call(t); if cpu.returned_to != 0x20bc9fa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bca00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bca00); }
 let mut pc: u32 = 0x20bca00;
 loop { match pc {
 0x20bca00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c84cu32), 0x0u32); }
  if cpu.zf { pc = 0x20bca26; continue; }
  pc = 0x20bca0c; continue;
 }
 0x20bca0c => {
  { let v = 0x20bca00u32; cpu.push(v); }
  { let v = 0x20bd160u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20bca1d); f_020b3500(cpu); if cpu.returned_to != 0x20bca1d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c84cu32, v); }
  pc = 0x20bca26; continue;
 }
 0x20bca26 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bca30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bca30); }
 let mut pc: u32 = 0x20bca30;
 loop { match pc {
 0x20bca30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bca3e; continue; }
  pc = 0x20bca38; continue;
 }
 0x20bca38 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bca3e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bca50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bca50); }
 let mut pc: u32 = 0x20bca50;
 loop { match pc {
 0x20bca50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bca5c); f_020a5fa0(cpu); if cpu.returned_to != 0x20bca5c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bcad1; continue; }
  pc = 0x20bca6b; continue;
 }
 0x20bca6b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bca76); f_020a5de0(cpu); if cpu.returned_to != 0x20bca76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bca8b; continue; }
  pc = 0x20bca7d; continue;
 }
 0x20bca7d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20bca83); f_020a5ef0(cpu); if cpu.returned_to != 0x20bca83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bca8b => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20bcacf; continue; }
  pc = 0x20bca92; continue;
 }
 0x20bca92 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20bcacf; continue; }
  pc = 0x20bca99; continue;
 }
 0x20bca99 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20bcacf; continue; }
  pc = 0x20bcaa1; continue;
 }
 0x20bcaa1 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bcaa8; continue; }
  pc = 0x20bcaa3; continue;
 }
 0x20bcaa3 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  pc = 0x20bcacd; continue;
 }
 0x20bcaa8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcab1); f_020a8f20(cpu); if cpu.returned_to != 0x20bcab1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { cpu.poll(); pc = 0x20bcaa8; continue; }
  pc = 0x20bcab9; continue;
 }
 0x20bcab9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcabe => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcac7); f_020ab040(cpu); if cpu.returned_to != 0x20bcac7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  pc = 0x20bcacd; continue;
 }
 0x20bcacd => {
  if cpu.zf { cpu.poll(); pc = 0x20bcabe; continue; }
  pc = 0x20bcacf; continue;
 }
 0x20bcacf => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20bcad1; continue;
 }
 0x20bcad1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bcae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bcae0); }
 let mut pc: u32 = 0x20bcae0;
 loop { match pc {
 0x20bcae0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bcb23; continue; }
  pc = 0x20bcae9; continue;
 }
 0x20bcae9 => {
  cpu.push(0x20bcaee); f_020ad4e0(cpu); if cpu.returned_to != 0x20bcaee { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20bcb1e; continue; }
  pc = 0x20bcaf5; continue;
 }
 0x20bcaf5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20bcb17; continue; }
  pc = 0x20bcafd; continue;
 }
 0x20bcafd => {
  if (cpu.cf || cpu.zf) { pc = 0x20bcb06; continue; }
  pc = 0x20bcaff; continue;
 }
 0x20bcaff => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20bcb0e; continue; }
  pc = 0x20bcb04; continue;
 }
 0x20bcb04 => {
  pc = 0x20bcb17; continue;
 }
 0x20bcb06 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcb0c); f_020a9280(cpu); if cpu.returned_to != 0x20bcb0c { return; }
  pc = 0x20bcb14; continue;
 }
 0x20bcb0e => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcb14); f_020ab510(cpu); if cpu.returned_to != 0x20bcb14 { return; }
  pc = 0x20bcb14; continue;
 }
 0x20bcb14 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20bcb17; continue;
 }
 0x20bcb17 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bcb1e; continue;
 }
 0x20bcb1e => {
  cpu.push(0x20bcb23); f_020ad4d0(cpu); if cpu.returned_to != 0x20bcb23 { return; }
  pc = 0x20bcb23; continue;
 }
 0x20bcb23 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bcb30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bcb30); }
 let mut pc: u32 = 0x20bcb30;
 loop { match pc {
 0x20bcb30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bcc92; continue; }
  pc = 0x20bcb40; continue;
 }
 0x20bcb40 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x20bcb4f; continue; }
  pc = 0x20bcb46; continue;
 }
 0x20bcb46 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bcb4c); f_020a5d30(cpu); if cpu.returned_to != 0x20bcb4c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20bcb4f; continue;
 }
 0x20bcb4f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20bcc92; continue; }
  pc = 0x20bcb5b; continue;
 }
 0x20bcb5b => {
  if (cpu.cf || cpu.zf) { pc = 0x20bcb6b; continue; }
  pc = 0x20bcb5d; continue;
 }
 0x20bcb5d => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20bcbf9; continue; }
  pc = 0x20bcb66; continue;
 }
 0x20bcb66 => {
  pc = 0x20bcc92; continue;
 }
 0x20bcb6b => {
  { let v: u32 = cpu.r32(0x22d22acu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcb77); f_020a9710(cpu); if cpu.returned_to != 0x20bcb77 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bcc94; continue; }
  pc = 0x20bcb85; continue;
 }
 0x20bcb85 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcb90); f_020a9540(cpu); if cpu.returned_to != 0x20bcb90 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20bcbab; continue; }
  pc = 0x20bcb98; continue;
 }
 0x20bcb98 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bcba1); f_020a9660(cpu); if cpu.returned_to != 0x20bcba1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcbab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20bcbb7); f_020a7750(cpu); if cpu.returned_to != 0x20bcbb7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x20bd120u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcbc8); f_020a7b50(cpu); if cpu.returned_to != 0x20bcbc8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bcbd9); f_020a9020(cpu); if cpu.returned_to != 0x20bcbd9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20bcbec); f_020a9490(cpu); if cpu.returned_to != 0x20bcbec { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcbf9 => {
  { let v: u32 = cpu.r32(0x22d22a8u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bcc05); f_020abd10(cpu); if cpu.returned_to != 0x20bcc05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bcc94; continue; }
  pc = 0x20bcc13; continue;
 }
 0x20bcc13 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcc19); f_020aba90(cpu); if cpu.returned_to != 0x20bcc19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcc2a); f_020ab970(cpu); if cpu.returned_to != 0x20bcc2a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bcc44; continue; }
  pc = 0x20bcc31; continue;
 }
 0x20bcc31 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcc3a); f_020abb40(cpu); if cpu.returned_to != 0x20bcc3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcc44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcc50); f_020a9ce0(cpu); if cpu.returned_to != 0x20bcc50 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x20bd0e0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bcc61); f_020a9ec0(cpu); if cpu.returned_to != 0x20bcc61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bcc72); f_020ab210(cpu); if cpu.returned_to != 0x20bcc72 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20bcc85); f_020ab720(cpu); if cpu.returned_to != 0x20bcc85 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcc92 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20bcc94; continue;
 }
 0x20bcc94 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bcca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bcca0); }
 let mut pc: u32 = 0x20bcca0;
 loop { match pc {
 0x20bcca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20bcd3d; continue; }
  pc = 0x20bccae; continue;
 }
 0x20bccae => {
  cpu.push(0x20bccb3); f_020ad4e0(cpu); if cpu.returned_to != 0x20bccb3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20bcce3; continue; }
  pc = 0x20bccba; continue;
 }
 0x20bccba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x20bccdc; continue; }
  pc = 0x20bccc2; continue;
 }
 0x20bccc2 => {
  if (cpu.cf || cpu.zf) { pc = 0x20bcccb; continue; }
  pc = 0x20bccc4; continue;
 }
 0x20bccc4 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20bccd3; continue; }
  pc = 0x20bccc9; continue;
 }
 0x20bccc9 => {
  pc = 0x20bccdc; continue;
 }
 0x20bcccb => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bccd1); f_020a9280(cpu); if cpu.returned_to != 0x20bccd1 { return; }
  pc = 0x20bccd9; continue;
 }
 0x20bccd3 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bccd9); f_020ab510(cpu); if cpu.returned_to != 0x20bccd9 { return; }
  pc = 0x20bccd9; continue;
 }
 0x20bccd9 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20bccdc; continue;
 }
 0x20bccdc => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bcce3; continue;
 }
 0x20bcce3 => {
  cpu.push(0x20bcce8); f_020ad4d0(cpu); if cpu.returned_to != 0x20bcce8 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20bccfd; continue; }
  pc = 0x20bccee; continue;
 }
 0x20bccee => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20bccf4); f_020b3710(cpu); if cpu.returned_to != 0x20bccf4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bccfd; continue;
 }
 0x20bccfd => {
  { let v: u32 = cpu.r32(0x217c848u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x20bcd10; continue; }
  pc = 0x20bcd06; continue;
 }
 0x20bcd06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217c848u32, v); }
  pc = 0x20bcd34; continue;
 }
 0x20bcd10 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bcd34; continue; }
  pc = 0x20bcd14; continue;
 }
 0x20bcd14 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x20bcd2d; continue; }
  pc = 0x20bcd1b; continue;
 }
 0x20bcd1b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bcd27); f_020b3710(cpu); if cpu.returned_to != 0x20bcd27 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcd2d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcd14; continue; }
  pc = 0x20bcd34; continue;
 }
 0x20bcd34 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bcd3a); f_020b3710(cpu); if cpu.returned_to != 0x20bcd3a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20bcd3d; continue;
 }
 0x20bcd3d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bcd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bcd40); }
 let mut pc: u32 = 0x20bcd40;
 loop { match pc {
 0x20bcd40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x20bcd4e); f_020b3740(cpu); if cpu.returned_to != 0x20bcd4e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bcd77; continue; }
  pc = 0x20bcd5a; continue;
 }
 0x20bcd5a => {
  { let v: u32 = 0x216b23cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20bcd66; continue;
 }
 0x20bcd66 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcd66; continue; }
  pc = 0x20bcd70; continue;
 }
 0x20bcd70 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bcfc0; continue;
 }
 0x20bcd77 => {
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcda5); f_020b3350(cpu); if cpu.returned_to != 0x20bcda5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bcdd8; continue; }
  pc = 0x20bcdaf; continue;
 }
 0x20bcdaf => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20bcdb5); f_020a5ef0(cpu); if cpu.returned_to != 0x20bcdb5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x216b24cu32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20bcdc4; continue;
 }
 0x20bcdc4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcdc4; continue; }
  pc = 0x20bcdce; continue;
 }
 0x20bcdce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcdd8 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x216b25cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x20bcde4; continue;
 }
 0x20bcde4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bcdf2; continue; }
  pc = 0x20bcdec; continue;
 }
 0x20bcdec => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bcdf2; continue; }
  pc = 0x20bcdf0; continue;
 }
 0x20bcdf0 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bcdf2; continue;
 }
 0x20bcdf2 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bcdff; continue; }
  pc = 0x20bcdf7; continue;
 }
 0x20bcdf7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bcdff; continue; }
  pc = 0x20bcdfc; continue;
 }
 0x20bcdfc => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20bcdff; continue;
 }
 0x20bcdff => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20bce0e; continue; }
  pc = 0x20bce03; continue;
 }
 0x20bce03 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20bce0e; continue; }
  pc = 0x20bce07; continue;
 }
 0x20bce07 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20bce0e; continue; }
  pc = 0x20bce0a; continue;
 }
 0x20bce0a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20bcde4; continue;
 }
 0x20bce0e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bce15; continue; }
  pc = 0x20bce13; continue;
 }
 0x20bce13 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bce15; continue;
 }
 0x20bce15 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bce25; continue; }
  pc = 0x20bce19; continue;
 }
 0x20bce19 => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bcf30; continue;
 }
 0x20bce25 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x216b268u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x20bce35; continue;
 }
 0x20bce35 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bce43; continue; }
  pc = 0x20bce3d; continue;
 }
 0x20bce3d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bce43; continue; }
  pc = 0x20bce41; continue;
 }
 0x20bce41 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bce43; continue;
 }
 0x20bce43 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bce50; continue; }
  pc = 0x20bce48; continue;
 }
 0x20bce48 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bce50; continue; }
  pc = 0x20bce4d; continue;
 }
 0x20bce4d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20bce50; continue;
 }
 0x20bce50 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20bce5f; continue; }
  pc = 0x20bce54; continue;
 }
 0x20bce54 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20bce5f; continue; }
  pc = 0x20bce58; continue;
 }
 0x20bce58 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20bce5f; continue; }
  pc = 0x20bce5b; continue;
 }
 0x20bce5b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20bce35; continue;
 }
 0x20bce5f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bce66; continue; }
  pc = 0x20bce64; continue;
 }
 0x20bce64 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bce66; continue;
 }
 0x20bce66 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bce76; continue; }
  pc = 0x20bce6a; continue;
 }
 0x20bce6a => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bcf30; continue;
 }
 0x20bce76 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x216b270u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x20bce83; continue;
 }
 0x20bce83 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bce91; continue; }
  pc = 0x20bce8b; continue;
 }
 0x20bce8b => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bce91; continue; }
  pc = 0x20bce8f; continue;
 }
 0x20bce8f => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bce91; continue;
 }
 0x20bce91 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bce9e; continue; }
  pc = 0x20bce96; continue;
 }
 0x20bce96 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bce9e; continue; }
  pc = 0x20bce9b; continue;
 }
 0x20bce9b => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20bce9e; continue;
 }
 0x20bce9e => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20bcead; continue; }
  pc = 0x20bcea2; continue;
 }
 0x20bcea2 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20bcead; continue; }
  pc = 0x20bcea6; continue;
 }
 0x20bcea6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20bcead; continue; }
  pc = 0x20bcea9; continue;
 }
 0x20bcea9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20bce83; continue;
 }
 0x20bcead => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bceb4; continue; }
  pc = 0x20bceb2; continue;
 }
 0x20bceb2 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bceb4; continue;
 }
 0x20bceb4 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20bcefd; continue; }
  pc = 0x20bceb8; continue;
 }
 0x20bceb8 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x216b278u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x20bcec8; continue;
 }
 0x20bcec8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bced6; continue; }
  pc = 0x20bced0; continue;
 }
 0x20bced0 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bced6; continue; }
  pc = 0x20bced4; continue;
 }
 0x20bced4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bced6; continue;
 }
 0x20bced6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x20bcee3; continue; }
  pc = 0x20bcedb; continue;
 }
 0x20bcedb => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bcee3; continue; }
  pc = 0x20bcee0; continue;
 }
 0x20bcee0 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20bcee3; continue;
 }
 0x20bcee3 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20bcef2; continue; }
  pc = 0x20bcee7; continue;
 }
 0x20bcee7 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20bcef2; continue; }
  pc = 0x20bceeb; continue;
 }
 0x20bceeb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20bcef2; continue; }
  pc = 0x20bceee; continue;
 }
 0x20bceee => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x20bcec8; continue;
 }
 0x20bcef2 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x20bcef9; continue; }
  pc = 0x20bcef7; continue;
 }
 0x20bcef7 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bcef9; continue;
 }
 0x20bcef9 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20bcf29; continue; }
  pc = 0x20bcefd; continue;
 }
 0x20bcefd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20bcf06); f_020a5ef0(cpu); if cpu.returned_to != 0x20bcf06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x216b280u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20bcf15; continue;
 }
 0x20bcf15 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcf15; continue; }
  pc = 0x20bcf1f; continue;
 }
 0x20bcf1f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcf29 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bcf30; continue;
 }
 0x20bcf30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x20bcf6b; continue; }
  pc = 0x20bcf39; continue;
 }
 0x20bcf39 => {
  { cpu.sub32(cpu.r32(0x22d22acu32), 0x0u32); }
  if !cpu.zf { pc = 0x20bcf6b; continue; }
  pc = 0x20bcf42; continue;
 }
 0x20bcf42 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20bcf48); f_020a5ef0(cpu); if cpu.returned_to != 0x20bcf48 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x216b294u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20bcf57; continue;
 }
 0x20bcf57 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcf57; continue; }
  pc = 0x20bcf61; continue;
 }
 0x20bcf61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcf6b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x20bcfa9; continue; }
  pc = 0x20bcf74; continue;
 }
 0x20bcf74 => {
  { cpu.sub32(cpu.r32(0x22d22a8u32), 0x0u32); }
  if !cpu.zf { pc = 0x20bcfa9; continue; }
  pc = 0x20bcf7d; continue;
 }
 0x20bcf7d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20bcf86); f_020a5ef0(cpu); if cpu.returned_to != 0x20bcf86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x216b2b0u32; cpu.esi = v; }
  { let v: u32 = 0x22d1ec0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20bcf95; continue;
 }
 0x20bcf95 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bcf95; continue; }
  pc = 0x20bcf9f; continue;
 }
 0x20bcf9f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20bcfa9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x217c848u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20bcfc0; continue;
 }
 0x20bcfc0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd0e0); }
 let mut pc: u32 = 0x20bd0e0;
 loop { match pc {
 0x20bd0e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bd0ec); f_020a9bd0(cpu); if cpu.returned_to != 0x20bd0ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20bd0fe); f_020a9bd0(cpu); if cpu.returned_to != 0x20bd0fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20bd10e); f_020abb40(cpu); if cpu.returned_to != 0x20bd10e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd120); }
 let mut pc: u32 = 0x20bd120;
 loop { match pc {
 0x20bd120 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20bd12c); f_020a7640(cpu); if cpu.returned_to != 0x20bd12c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20bd13e); f_020a7640(cpu); if cpu.returned_to != 0x20bd13e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20bd14e); f_020a9660(cpu); if cpu.returned_to != 0x20bd14e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd160); }
 let mut pc: u32 = 0x20bd160;
 loop { match pc {
 0x20bd160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217c84cu32), 0x0u32); }
  if !cpu.zf { pc = 0x20bd1b6; continue; }
  pc = 0x20bd16c; continue;
 }
 0x20bd16c => {
  { let v = 0x20bca00u32; cpu.push(v); }
  { let v = 0x20bd160u32; cpu.push(v); }
  cpu.push(0x20bd17b); f_020b35a0(cpu); if cpu.returned_to != 0x20bd17b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d22acu32; cpu.push(v); }
  cpu.push(0x20bd18a); f_020b34e0(cpu); if cpu.returned_to != 0x20bd18a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d22a8u32; cpu.push(v); }
  cpu.push(0x20bd199); f_020b34e0(cpu); if cpu.returned_to != 0x20bd199 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217c848u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20bd1ad); f_020b34e0(cpu); if cpu.returned_to != 0x20bd1ad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c84cu32, v); }
  pc = 0x20bd1b6; continue;
 }
 0x20bd1b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd1ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd1ba); }
 let mut pc: u32 = 0x20bd1ba;
 loop { match pc {
 0x20bd1ba => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217b914u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20bd1d1); f_020bd27b(cpu); if cpu.returned_to != 0x20bd1d1 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217b914u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x20bd1f3); f_020bd474(cpu); if cpu.returned_to != 0x20bd1f3 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bd254; continue; }
  pc = 0x20bd1f9; continue;
 }
 0x20bd1f9 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217b914u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b0d8u32, v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x217b0d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x217b918u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x20bd23b); f_020bd27b(cpu); if cpu.returned_to != 0x20bd23b { return; }
  { let v: u32 = cpu.r32(0x217b0d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b0d4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20bd25e; continue;
 }
 0x20bd254 => {
  { let v: u32 = cpu.eax; cpu.w32(0x217b0d8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x217b0d4u32, v); }
  pc = 0x20bd25e; continue;
 }
 0x20bd25e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d2358u32, v); }
  { let v: u32 = cpu.r32(0x217b0d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d235cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_020bd27b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd27b); }
 let mut pc: u32 = 0x20bd27b;
 loop { match pc {
 0x20bd27b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d2360u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20bd292; continue;
 }
 0x20bd292 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.zf { pc = 0x20bd2a0; continue; }
  pc = 0x20bd29b; continue;
 }
 0x20bd29b => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if !cpu.zf { pc = 0x20bd2a3; continue; }
  pc = 0x20bd2a0; continue;
 }
 0x20bd2a0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20bd292; continue;
 }
 0x20bd2a3 => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x20bd375; continue; }
  pc = 0x20bd2ab; continue;
 }
 0x20bd2ab => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if !cpu.zf { pc = 0x20bd2b5; continue; }
  pc = 0x20bd2b2; continue;
 }
 0x20bd2b2 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20bd2b5; continue;
 }
 0x20bd2b5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x20bd2ba; continue;
 }
 0x20bd2ba => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x20bd2e3; continue; }
  pc = 0x20bd2c3; continue;
 }
 0x20bd2c3 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20bd342; continue; }
  pc = 0x20bd2cc; continue;
 }
 0x20bd2cc => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20bd2e3; continue; }
  pc = 0x20bd2d0; continue;
 }
 0x20bd2d0 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x20bd2d9; continue; }
  pc = 0x20bd2d4; continue;
 }
 0x20bd2d4 => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20bd2d6; continue;
 }
 0x20bd2d6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20bd2ba; continue;
 }
 0x20bd2d9 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x20bd2e3; continue; }
  pc = 0x20bd2de; continue;
 }
 0x20bd2de => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x20bd2ba; continue;
 }
 0x20bd2e3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.zf { pc = 0x20bd2f1; continue; }
  pc = 0x20bd2ec; continue;
 }
 0x20bd2ec => {
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if !cpu.zf { pc = 0x20bd2f5; continue; }
  pc = 0x20bd2f1; continue;
 }
 0x20bd2f1 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20bd342; continue; }
  pc = 0x20bd2f5; continue;
 }
 0x20bd2f5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x20bd342; continue; }
  pc = 0x20bd2fb; continue;
 }
 0x20bd2fb => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  if !cpu.zf { pc = 0x20bd333; continue; }
  pc = 0x20bd304; continue;
 }
 0x20bd304 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20bd314; continue; }
  pc = 0x20bd308; continue;
 }
 0x20bd308 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  pc = 0x20bd310; continue;
 }
 0x20bd310 => {
  if !cpu.zf { pc = 0x20bd333; continue; }
  pc = 0x20bd312; continue;
 }
 0x20bd312 => {
  pc = 0x20bd332; continue;
 }
 0x20bd314 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x20bd328; continue; }
  pc = 0x20bd319; continue;
 }
 0x20bd319 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if cpu.zf { pc = 0x20bd332; continue; }
  pc = 0x20bd323; continue;
 }
 0x20bd323 => {
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  cpu.poll(); pc = 0x20bd310; continue;
 }
 0x20bd328 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x20bd333; continue; }
  pc = 0x20bd332; continue;
 }
 0x20bd332 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20bd333; continue;
 }
 0x20bd333 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x20bd2d6; continue; }
  pc = 0x20bd337; continue;
 }
 0x20bd337 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20bd2ba; continue;
 }
 0x20bd342 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20bd368; continue; }
  pc = 0x20bd346; continue;
 }
 0x20bd346 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x20bd35f; continue; }
  pc = 0x20bd35b; continue;
 }
 0x20bd35b => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bd375; continue;
 }
 0x20bd35f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x20bd292; continue;
 }
 0x20bd368 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20bd2a0; continue; }
  pc = 0x20bd375; continue;
 }
 0x20bd375 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d2360u32, v); }
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
pub fn f_020bd38a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd38a); }
 let mut pc: u32 = 0x20bd38a;
 loop { match pc {
 0x20bd38a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x217b928u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x20bd39f); f_020c1d91(cpu); if cpu.returned_to != 0x20bd39f { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x20bd3ab; continue; }
  pc = 0x20bd3a3; continue;
 }
 0x20bd3a3 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x20bd3ad; continue;
 }
 0x20bd3ab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bd3ad; continue;
 }
 0x20bd3ad => {
  { let v: u32 = cpu.r32(0x217b928u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b92cu32, v); }
  cpu.push(0x20bd3bf); f_020c45c1(cpu); if cpu.returned_to != 0x20bd3bf { return; }
  { let v: u32 = cpu.r32(0x22d235cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d2358u32); cpu.eax = v; }
  cpu.push(0x20bd3cf); f_0207e0c8(cpu); if cpu.returned_to != 0x20bd3cf { return; }
  cpu.push(0x20bd3d4); f_020c462b(cpu); if cpu.returned_to != 0x20bd3d4 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd3da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd3da); }
 let mut pc: u32 = 0x20bd3da;
 loop { match pc {
 0x20bd3da => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20bd3e1; continue;
 }
 0x20bd3e1 => {
  { let v: u32 = 0x217ceceu32; cpu.esi = v; }
  { let v: u32 = 0x217cf0au32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bd3f1; continue;
 }
 0x20bd3f1 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x20bd409; continue; }
  pc = 0x20bd3f5; continue;
 }
 0x20bd3f5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x20bd404; continue; }
  pc = 0x20bd3fa; continue;
 }
 0x20bd3fa => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bd404; continue; }
  pc = 0x20bd3ff; continue;
 }
 0x20bd3ff => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bd404; continue;
 }
 0x20bd404 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x20bd3f1; continue;
 }
 0x20bd409 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x20bd41f; continue; }
  pc = 0x20bd40d; continue;
 }
 0x20bd40d => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20bd41a; continue; }
  pc = 0x20bd414; continue;
 }
 0x20bd414 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x20bd417); cpu.push(0x20bd419); cpu.call(t); if cpu.returned_to != 0x20bd419 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x20bd41a; continue;
 }
 0x20bd41a => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bd3e1; continue;
 }
 0x20bd41f => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x20bd424); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd425(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd425); }
 let mut pc: u32 = 0x20bd425;
 loop { match pc {
 0x20bd425 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20bd42d; continue;
 }
 0x20bd42d => {
  { let v: u32 = 0x217cf0au32; cpu.esi = v; }
  { let v: u32 = 0x217cf1cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bd43d; continue;
 }
 0x20bd43d => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x20bd455; continue; }
  pc = 0x20bd441; continue;
 }
 0x20bd441 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x20bd450; continue; }
  pc = 0x20bd446; continue;
 }
 0x20bd446 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bd450; continue; }
  pc = 0x20bd44b; continue;
 }
 0x20bd44b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20bd450; continue;
 }
 0x20bd450 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x20bd43d; continue;
 }
 0x20bd455 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x20bd46f; continue; }
  pc = 0x20bd459; continue;
 }
 0x20bd459 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bd46a; continue; }
  pc = 0x20bd45d; continue;
 }
 0x20bd45d => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20bd46a; continue; }
  pc = 0x20bd464; continue;
 }
 0x20bd464 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x20bd467); cpu.push(0x20bd469); cpu.call(t); if cpu.returned_to != 0x20bd469 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x20bd46a; continue;
 }
 0x20bd46a => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20bd42d; continue;
 }
 0x20bd46f => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x20bd473); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd474); }
 let mut pc: u32 = 0x20bd474;
 loop { match pc {
 0x20bd474 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bd48d; continue; }
  pc = 0x20bd488; continue;
 }
 0x20bd488 => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20bd494; continue; }
  pc = 0x20bd48d; continue;
 }
 0x20bd48d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bd548; continue;
 }
 0x20bd494 => {
  { let v: u32 = 0x217c850u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x20bd4a7; continue;
 }
 0x20bd4a7 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x20bd4b3; continue; }
  pc = 0x20bd4ae; continue;
 }
 0x20bd4ae => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x20bd4b3; continue;
 }
 0x20bd4b3 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x217c858u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x20bd4d3; continue; }
  pc = 0x20bd4bb; continue;
 }
 0x20bd4bb => {
  { let v: u32 = cpu.r32(0x217c854u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20bd50b; continue; }
  pc = 0x20bd4c5; continue;
 }
 0x20bd4c5 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x217c858u32, v); }
  { let v: u32 = cpu.r32(0x217c850u32); cpu.ecx = v; }
  pc = 0x20bd50b; continue;
 }
 0x20bd4d3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x20bd4c5; continue;
 }
 0x20bd4d7 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217c854u32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bd4f0); f_020c465a(cpu); if cpu.returned_to != 0x20bd4f0 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd53e; continue; }
  pc = 0x20bd4f6; continue;
 }
 0x20bd4f6 => {
  { let v: u32 = cpu.r32(0x217c858u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x20bd508; continue; }
  pc = 0x20bd503; continue;
 }
 0x20bd503 => {
  { let v: u32 = cpu.eax; cpu.w32(0x217c858u32, v); }
  pc = 0x20bd508; continue;
 }
 0x20bd508 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x20bd50b; continue;
 }
 0x20bd50b => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20bd4d7; continue; }
  pc = 0x20bd50f; continue;
 }
 0x20bd50f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x20bd520; continue; }
  pc = 0x20bd515; continue;
 }
 0x20bd515 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bd51c); f_020c4bfe(cpu); if cpu.returned_to != 0x20bd51c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd535; continue; }
  pc = 0x20bd520; continue;
 }
 0x20bd520 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bd527); f_020c4dc7(cpu); if cpu.returned_to != 0x20bd527 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bd53e; continue; }
  pc = 0x20bd52b; continue;
 }
 0x20bd52b => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20bd4a7; continue;
 }
 0x20bd535 => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20bd4a7; continue;
 }
 0x20bd53e => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x22d2364u32, v); }
  pc = 0x20bd548; continue;
 }
 0x20bd548 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
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
pub fn f_020bd557(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd557); }
 let mut pc: u32 = 0x20bd557;
 loop { match pc {
 0x20bd557 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bd5d3; continue; }
  pc = 0x20bd566; continue;
 }
 0x20bd566 => {
  { let v: u32 = cpu.r32(0x22d22b0u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20bd579; continue; }
  pc = 0x20bd570; continue;
 }
 0x20bd570 => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bd579; continue; }
  pc = 0x20bd574; continue;
 }
 0x20bd574 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x20bd593; continue; }
  pc = 0x20bd579; continue;
 }
 0x20bd579 => {
  { let v: u32 = cpu.r32(0x217c850u32); cpu.ecx = v; }
  pc = 0x20bd58c; continue;
 }
 0x20bd581 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20bd589; continue; }
  pc = 0x20bd585; continue;
 }
 0x20bd585 => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x20bd593; continue; }
  pc = 0x20bd589; continue;
 }
 0x20bd589 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x20bd58c; continue;
 }
 0x20bd58c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x20bd581; continue; }
  pc = 0x20bd593; continue;
 }
 0x20bd593 => {
  { let v: u32 = 0x217c850u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20bd5a9); f_020c4702(cpu); if cpu.returned_to != 0x20bd5a9 { return; }
  { let v: u32 = cpu.r32(0x217c854u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d22b0u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x20bd5cb; continue; }
  pc = 0x20bd5b9; continue;
 }
 0x20bd5b9 => {
  { let v: u32 = cpu.r32(0x217c858u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x20bd5cb; continue; }
  pc = 0x20bd5c5; continue;
 }
 0x20bd5c5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x217c858u32, v); }
  pc = 0x20bd5cb; continue;
 }
 0x20bd5cb => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x22d2364u32, v); }
  pc = 0x20bd5d3; continue;
 }
 0x20bd5d3 => {
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
pub fn f_020bd5d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd5d9); }
 let mut pc: u32 = 0x20bd5d9;
 loop { match pc {
 0x20bd5d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20bd631; continue; }
  pc = 0x20bd5eb; continue;
 }
 0x20bd5eb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bd631; continue; }
  pc = 0x20bd5ef; continue;
 }
 0x20bd5ef => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x20bd62b; continue;
 }
 0x20bd605 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20bd610); f_020a5856(cpu); if cpu.returned_to != 0x20bd610 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd628; continue; }
  pc = 0x20bd614; continue;
 }
 0x20bd614 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x20bd628; continue; }
  pc = 0x20bd621; continue;
 }
 0x20bd621 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x20bd633; continue;
 }
 0x20bd628 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x20bd62b; continue;
 }
 0x20bd62b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20bd605; continue; }
  pc = 0x20bd631; continue;
 }
 0x20bd631 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20bd633; continue;
 }
 0x20bd633 => {
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
pub fn f_020bd63a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd63a); }
 let mut pc: u32 = 0x20bd63a;
 loop { match pc {
 0x20bd63a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd66d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd66d); }
 let mut pc: u32 = 0x20bd66d;
 loop { match pc {
 0x20bd66d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b734u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x216b73bu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x216b742u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b710u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x216b71cu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x216b728u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20bd6ca); f_020bd63a(cpu); if cpu.returned_to != 0x20bd6ca { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x20bd6d8; continue; }
  pc = 0x20bd6d4; continue;
 }
 0x20bd6d4 => {
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x20bd6d8; continue;
 }
 0x20bd6d8 => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20bd6eb); f_020bd63a(cpu); if cpu.returned_to != 0x20bd6eb { return; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20bd6fe); f_020bd63a(cpu); if cpu.returned_to != 0x20bd6fe { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20bd710); f_020bd63a(cpu); if cpu.returned_to != 0x20bd710 { return; }
  { let v: u8 = 0x20u8; cpu.w8(0x13u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x14u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x13u32); cpu.eax = v; }
  cpu.push(0x20bd738); f_020bd63a(cpu); if cpu.returned_to != 0x20bd738 { return; }
  { let v: u32 = 0x16u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20bd748); f_020bd63a(cpu); if cpu.returned_to != 0x20bd748 { return; }
  { let v: u8 = 0xau8; cpu.w8(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd778); }
 let mut pc: u32 = 0x20bd778;
 loop { match pc {
 0x20bd778 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x22d22b4u32; cpu.edx = v; }
  cpu.push(0x20bd783); f_020bd66d(cpu); if cpu.returned_to != 0x20bd783 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd78a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd78a); }
 let mut pc: u32 = 0x20bd78a;
 loop { match pc {
 0x20bd78a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x20bd797); f_020c4dca(cpu); if cpu.returned_to != 0x20bd797 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd7a4; continue; }
  pc = 0x20bd79b; continue;
 }
 0x20bd79b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20bd7a4); f_020ae005(cpu); if cpu.returned_to != 0x20bd7a4 { return; }
  pc = 0x20bd7a4; continue;
 }
 0x20bd7a4 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd7a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd7a7); }
 let mut pc: u32 = 0x20bd7a7;
 loop { match pc {
 0x20bd7a7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20bd7b3; continue; }
  pc = 0x20bd7ae; continue;
 }
 0x20bd7ae => {
  { let v: u8 = 0x2fu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20bd7c1; continue;
 }
 0x20bd7b3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x37u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20bd7c1; continue;
 }
 0x20bd7c1 => {
  { let v: u8 = 0x63u8; cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd7cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd7cd); }
 let mut pc: u32 = 0x20bd7cd;
 loop { match pc {
 0x20bd7cd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(44); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x20bd7e0); f_020a2492(cpu); if cpu.returned_to != 0x20bd7e0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd7ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd7ec); }
 let mut pc: u32 = 0x20bd7ec;
 loop { match pc {
 0x20bd7ec => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x20bd806); f_020c4f37(cpu); if cpu.returned_to != 0x20bd806 { return; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x20bd816); f_020c4f8a(cpu); if cpu.returned_to != 0x20bd816 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bd81f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bd81f); }
 let mut pc: u32 = 0x20bd81f;
 loop { match pc {
 0x20bd81f => {
  { let v = 0xe4u32; cpu.push(v); }
  cpu.flat_ds(0x20bd824); cpu.push(0x20bd829); f_020ae38a(cpu); if cpu.returned_to != 0x20bd829 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(176); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.cf { pc = 0x20bd85b; continue; }
  pc = 0x20bd847; continue;
 }
 0x20bd847 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.flat_ds(0x20bd84c); cpu.push(0x20bd851); f_020af0db(cpu); if cpu.returned_to != 0x20bd851 { return; }
  pc = 0x20bd851; continue;
 }
 0x20bd851 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20bdb27; continue;
 }
 0x20bd85b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x56u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x5au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5eu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bd86e); cpu.push(0x20bd873); f_020c4dff(cpu); if cpu.returned_to != 0x20bd873 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20bdb27; continue; }
  pc = 0x20bd87f; continue;
 }
 0x20bd87f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x20bd881); cpu.push(0x20bd886); f_020a4a64(cpu); if cpu.returned_to != 0x20bd886 { return; }
  { let v: u32 = 0x9au32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.flat_ds(0x20bd88e); cpu.push(0x20bd893); f_020bd474(cpu); if cpu.returned_to != 0x20bd893 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd8c4; continue; }
  pc = 0x20bd89a; continue;
 }
 0x20bd89a => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xfcu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x20bd8a0); cpu.push(0x20bd8a5); f_020c1d91(cpu); if cpu.returned_to != 0x20bd8a5 { return; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.cf { pc = 0x20bd8b1; continue; }
  pc = 0x20bd8a9; continue;
 }
 0x20bd8a9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x20bd8b3; continue;
 }
 0x20bd8b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bd8b3; continue;
 }
 0x20bd8b3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd8c7; continue; }
  pc = 0x20bd8ba; continue;
 }
 0x20bd8ba => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bd8bd); cpu.push(0x20bd8c2); f_020bd557(cpu); if cpu.returned_to != 0x20bd8c2 { return; }
  cpu.poll(); pc = 0x20bd851; continue;
 }
 0x20bd8c4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bd8c7; continue;
 }
 0x20bd8c7 => {
  { let v: u32 = 0x42u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x4au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x46u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x93u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x20bd8e0); cpu.push(0x20bd8e5); f_020c50aa(cpu); if cpu.returned_to != 0x20bd8e5 { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x217c8ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20bd8e8); cpu.push(0x20bd8ee); cpu.call(t); if cpu.returned_to != 0x20bd8ee { return; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bd8f1); cpu.push(0x20bd8f6); f_020bd474(cpu); if cpu.returned_to != 0x20bd8f6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd946; continue; }
  pc = 0x20bd8fd; continue;
 }
 0x20bd8fd => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x20bd906); cpu.push(0x20bd90b); f_020c1d91(cpu); if cpu.returned_to != 0x20bd90b { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x20bd91d; continue; }
  pc = 0x20bd90f; continue;
 }
 0x20bd90f => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x20bd91f; continue;
 }
 0x20bd91d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20bd91f; continue;
 }
 0x20bd91f => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd949; continue; }
  pc = 0x20bd926; continue;
 }
 0x20bd926 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.flat_ds(0x20bd92b); cpu.push(0x20bd930); f_020af0db(cpu); if cpu.returned_to != 0x20bd930 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bd93c); cpu.push(0x20bd941); f_020af0fe(cpu); if cpu.returned_to != 0x20bd941 { return; }
  pc = 0x20bdb03; continue;
 }
 0x20bd946 => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bd949; continue;
 }
 0x20bd949 => {
  { let v: u32 = cpu.r8(0x217b947u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x20bd96c; continue; }
  pc = 0x20bd955; continue;
 }
 0x20bd955 => {
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x20bd96c; continue; }
  pc = 0x20bd95d; continue;
 }
 0x20bd95d => {
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x20bd96c; continue; }
  pc = 0x20bd965; continue;
 }
 0x20bd965 => {
  { let v: u32 = 0x216b2d0u32; cpu.w32(0x4au32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bd96c; continue;
 }
 0x20bd96c => {
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x3eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bd97b); cpu.push(0x20bd980); f_020b1c90(cpu); if cpu.returned_to != 0x20bd980 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x20bd985); cpu.push(0x20bd98a); f_020af0db(cpu); if cpu.returned_to != 0x20bd98a { return; }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x20bd9dc; continue; }
  pc = 0x20bd992; continue;
 }
 0x20bd992 => {
  { let v: u32 = 0x216b2d3u32; cpu.edx = v; }
  cpu.flat_ds(0x20bd997); cpu.push(0x20bd99c); f_020a20a7(cpu); if cpu.returned_to != 0x20bd99c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bd9bc; continue; }
  pc = 0x20bd9a0; continue;
 }
 0x20bd9a0 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bd9aa); cpu.push(0x20bd9af); f_020bd7cd(cpu); if cpu.returned_to != 0x20bd9af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20bdaa0; continue; }
  pc = 0x20bd9b7; continue;
 }
 0x20bd9b7 => {
  pc = 0x20bdb03; continue;
 }
 0x20bd9bc => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x20bd9c4); cpu.push(0x20bd9c9); f_020af0db(cpu); if cpu.returned_to != 0x20bd9c9 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bd9d2); cpu.push(0x20bd9d7); f_020bd7ec(cpu); if cpu.returned_to != 0x20bd9d7 { return; }
  pc = 0x20bdb00; continue;
 }
 0x20bd9dc => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x20bd9e2); cpu.push(0x20bd9e7); f_020a4a64(cpu); if cpu.returned_to != 0x20bd9e7 { return; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x20bd9fe; continue; }
  pc = 0x20bd9f2; continue;
 }
 0x20bd9f2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x20bd9f7); cpu.push(0x20bd9fc); f_020af0db(cpu); if cpu.returned_to != 0x20bd9fc { return; }
  pc = 0x20bda27; continue;
 }
 0x20bd9fe => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x216b2d8u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x20bda11); cpu.push(0x20bda16); f_020af0db(cpu); if cpu.returned_to != 0x20bda16 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bda1f); cpu.push(0x20bda24); f_020bd7ec(cpu); if cpu.returned_to != 0x20bda24 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bda27; continue;
 }
 0x20bda27 => {
  cpu.flat_ds(0x20bda27); cpu.push(0x20bda2c); f_020b17b7(cpu); if cpu.returned_to != 0x20bda2c { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x20bda3f; continue; }
  pc = 0x20bda31; continue;
 }
 0x20bda31 => {
  cpu.flat_ds(0x20bda31); cpu.push(0x20bda36); f_020b17b7(cpu); if cpu.returned_to != 0x20bda36 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x20bdb03; continue; }
  pc = 0x20bda3f; continue;
 }
 0x20bda3f => {
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x20bda4a); cpu.push(0x20bda4f); f_020af0db(cpu); if cpu.returned_to != 0x20bda4f { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x216b2ddu32; cpu.esi = v; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.flat_ds(0x20bda61); cpu.push(0x20bda66); f_020bd7ec(cpu); if cpu.returned_to != 0x20bda66 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bda69); cpu.push(0x20bda6e); f_020b17b7(cpu); if cpu.returned_to != 0x20bda6e { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x20bda81; continue; }
  pc = 0x20bda73; continue;
 }
 0x20bda73 => {
  cpu.flat_ds(0x20bda73); cpu.push(0x20bda78); f_020b17b7(cpu); if cpu.returned_to != 0x20bda78 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x20bdb03; continue; }
  pc = 0x20bda81; continue;
 }
 0x20bda81 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.flat_ds(0x20bda86); cpu.push(0x20bda8b); f_020af0db(cpu); if cpu.returned_to != 0x20bda8b { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x216b2d3u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x20bda97); cpu.push(0x20bda9c); f_020bd7cd(cpu); if cpu.returned_to != 0x20bda9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20bdb03; continue; }
  pc = 0x20bdaa0; continue;
 }
 0x20bdaa0 => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x20bdaae); cpu.push(0x20bdab3); f_020bd557(cpu); if cpu.returned_to != 0x20bdab3 { return; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x20bdabd); cpu.push(0x20bdac2); f_020c4f37(cpu); if cpu.returned_to != 0x20bdac2 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.flat_ds(0x20bdad1); cpu.push(0x20bdad6); f_020bd7a7(cpu); if cpu.returned_to != 0x20bdad6 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x20bdae4; continue; }
  pc = 0x20bdadd; continue;
 }
 0x20bdadd => {
  { let v: u32 = 0x216b2eau32; cpu.eax = v; }
  pc = 0x20bdae9; continue;
 }
 0x20bdae4 => {
  { let v: u32 = 0x216b2eeu32; cpu.eax = v; }
  pc = 0x20bdae9; continue;
 }
 0x20bdae9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x216b2e2u32; cpu.eax = v; }
  cpu.flat_ds(0x20bdaef); cpu.push(0x20bdaf4); f_020bd5d9(cpu); if cpu.returned_to != 0x20bdaf4 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.flat_ds(0x20bdaf8); cpu.push(0x20bdafd); f_020c5185(cpu); if cpu.returned_to != 0x20bdafd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x20bdb00; continue;
 }
 0x20bdb00 => {
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20bdb03; continue;
 }
 0x20bdb03 => {
  { let v: u32 = cpu.r32(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bdb06); cpu.push(0x20bdb0b); f_020bd557(cpu); if cpu.returned_to != 0x20bdb0b { return; }
  { let v: u32 = cpu.r32(0x66u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bdb0e); cpu.push(0x20bdb13); f_020bd557(cpu); if cpu.returned_to != 0x20bdb13 { return; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x20bdb16); cpu.push(0x20bdb1b); f_020bd557(cpu); if cpu.returned_to != 0x20bdb1b { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x217c8f0u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20bdb1e); cpu.push(0x20bdb24); cpu.call(t); if cpu.returned_to != 0x20bdb24 { return; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x20bdb27; continue;
 }
 0x20bdb27 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x20bdb31); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdb32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdb32); }
 let mut pc: u32 = 0x20bdb32;
 loop { match pc {
 0x20bdb32 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20bdb45); f_020c51a3(cpu); if cpu.returned_to != 0x20bdb45 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdb48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdb48); }
 let mut pc: u32 = 0x20bdb48;
 loop { match pc {
 0x20bdb48 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x20bdb4d); cpu.push(0x20bdb52); f_020bdb80(cpu); if cpu.returned_to != 0x20bdb52 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x14u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x20bdb7f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdb80); }
 let mut pc: u32 = 0x20bdb80;
 loop { match pc {
 0x20bdb80 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x20bdc26u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v = cpu.ebp; cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdbf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdbf2); }
 let mut pc: u32 = 0x20bdbf2;
 loop { match pc {
 0x20bdbf2 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x20bdc26u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  { let v = cpu.r16(0x1eu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(4, v as u16); }
  { let v = cpu.r16(0x22u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc26); }
 let mut pc: u32 = 0x20bdc26;
 loop { match pc {
 0x20bdc26 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc29); }
 let mut pc: u32 = 0x20bdc29;
 loop { match pc {
 0x20bdc29 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc32); }
 let mut pc: u32 = 0x20bdc32;
 loop { match pc {
 0x20bdc32 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc35); }
 let mut pc: u32 = 0x20bdc35;
 loop { match pc {
 0x20bdc35 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc38); }
 let mut pc: u32 = 0x20bdc38;
 loop { match pc {
 0x20bdc38 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc3b); }
 let mut pc: u32 = 0x20bdc3b;
 loop { match pc {
 0x20bdc3b => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc3e); }
 let mut pc: u32 = 0x20bdc3e;
 loop { match pc {
 0x20bdc3e => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc41); }
 let mut pc: u32 = 0x20bdc41;
 loop { match pc {
 0x20bdc41 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc44); }
 let mut pc: u32 = 0x20bdc44;
 loop { match pc {
 0x20bdc44 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc47); }
 let mut pc: u32 = 0x20bdc47;
 loop { match pc {
 0x20bdc47 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc4a); }
 let mut pc: u32 = 0x20bdc4a;
 loop { match pc {
 0x20bdc4a => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc4d); }
 let mut pc: u32 = 0x20bdc4d;
 loop { match pc {
 0x20bdc4d => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc50); }
 let mut pc: u32 = 0x20bdc50;
 loop { match pc {
 0x20bdc50 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc53); }
 let mut pc: u32 = 0x20bdc53;
 loop { match pc {
 0x20bdc53 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc56); }
 let mut pc: u32 = 0x20bdc56;
 loop { match pc {
 0x20bdc56 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc59); }
 let mut pc: u32 = 0x20bdc59;
 loop { match pc {
 0x20bdc59 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc5c); }
 let mut pc: u32 = 0x20bdc5c;
 loop { match pc {
 0x20bdc5c => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc5f); }
 let mut pc: u32 = 0x20bdc5f;
 loop { match pc {
 0x20bdc5f => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc62); }
 let mut pc: u32 = 0x20bdc62;
 loop { match pc {
 0x20bdc62 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc65); }
 let mut pc: u32 = 0x20bdc65;
 loop { match pc {
 0x20bdc65 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc68); }
 let mut pc: u32 = 0x20bdc68;
 loop { match pc {
 0x20bdc68 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc6b); }
 let mut pc: u32 = 0x20bdc6b;
 loop { match pc {
 0x20bdc6b => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc6e); }
 let mut pc: u32 = 0x20bdc6e;
 loop { match pc {
 0x20bdc6e => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc71); }
 let mut pc: u32 = 0x20bdc71;
 loop { match pc {
 0x20bdc71 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc74); }
 let mut pc: u32 = 0x20bdc74;
 loop { match pc {
 0x20bdc74 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc77); }
 let mut pc: u32 = 0x20bdc77;
 loop { match pc {
 0x20bdc77 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc7a); }
 let mut pc: u32 = 0x20bdc7a;
 loop { match pc {
 0x20bdc7a => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc7d); }
 let mut pc: u32 = 0x20bdc7d;
 loop { match pc {
 0x20bdc7d => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc80); }
 let mut pc: u32 = 0x20bdc80;
 loop { match pc {
 0x20bdc80 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc83); }
 let mut pc: u32 = 0x20bdc83;
 loop { match pc {
 0x20bdc83 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc86); }
 let mut pc: u32 = 0x20bdc86;
 loop { match pc {
 0x20bdc86 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc89); }
 let mut pc: u32 = 0x20bdc89;
 loop { match pc {
 0x20bdc89 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc8c); }
 let mut pc: u32 = 0x20bdc8c;
 loop { match pc {
 0x20bdc8c => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc8f); }
 let mut pc: u32 = 0x20bdc8f;
 loop { match pc {
 0x20bdc8f => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc92); }
 let mut pc: u32 = 0x20bdc92;
 loop { match pc {
 0x20bdc92 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc95); }
 let mut pc: u32 = 0x20bdc95;
 loop { match pc {
 0x20bdc95 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc98); }
 let mut pc: u32 = 0x20bdc98;
 loop { match pc {
 0x20bdc98 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc9b); }
 let mut pc: u32 = 0x20bdc9b;
 loop { match pc {
 0x20bdc9b => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdc9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdc9e); }
 let mut pc: u32 = 0x20bdc9e;
 loop { match pc {
 0x20bdc9e => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdca1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdca1); }
 let mut pc: u32 = 0x20bdca1;
 loop { match pc {
 0x20bdca1 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdca4); }
 let mut pc: u32 = 0x20bdca4;
 loop { match pc {
 0x20bdca4 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdca7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdca7); }
 let mut pc: u32 = 0x20bdca7;
 loop { match pc {
 0x20bdca7 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcaa); }
 let mut pc: u32 = 0x20bdcaa;
 loop { match pc {
 0x20bdcaa => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcad); }
 let mut pc: u32 = 0x20bdcad;
 loop { match pc {
 0x20bdcad => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcb0); }
 let mut pc: u32 = 0x20bdcb0;
 loop { match pc {
 0x20bdcb0 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcb3); }
 let mut pc: u32 = 0x20bdcb3;
 loop { match pc {
 0x20bdcb3 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcb6); }
 let mut pc: u32 = 0x20bdcb6;
 loop { match pc {
 0x20bdcb6 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcb9); }
 let mut pc: u32 = 0x20bdcb9;
 loop { match pc {
 0x20bdcb9 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcbc); }
 let mut pc: u32 = 0x20bdcbc;
 loop { match pc {
 0x20bdcbc => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcbf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcbf); }
 let mut pc: u32 = 0x20bdcbf;
 loop { match pc {
 0x20bdcbf => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcc2); }
 let mut pc: u32 = 0x20bdcc2;
 loop { match pc {
 0x20bdcc2 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcc5); }
 let mut pc: u32 = 0x20bdcc5;
 loop { match pc {
 0x20bdcc5 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcc8); }
 let mut pc: u32 = 0x20bdcc8;
 loop { match pc {
 0x20bdcc8 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdccb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdccb); }
 let mut pc: u32 = 0x20bdccb;
 loop { match pc {
 0x20bdccb => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020bdcce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20bdcce); }
 let mut pc: u32 = 0x20bdcce;
 loop { match pc {
 0x20bdcce => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
