#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_040a2be7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2be7); }
 let mut pc: u32 = 0x40a2be7;
 loop { match pc {
 0x40a2be7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2bf6); f_040a2634(cpu); if cpu.returned_to != 0x40a2bf6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2bfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2bfa); }
 let mut pc: u32 = 0x40a2bfa;
 loop { match pc {
 0x40a2bfa => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2c09); f_040a2634(cpu); if cpu.returned_to != 0x40a2c09 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2c6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2c6c); }
 let mut pc: u32 = 0x40a2c6c;
 loop { match pc {
 0x40a2c6c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x40a2cb0; continue; }
  pc = 0x40a2c7d; continue;
 }
 0x40a2c7d => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a2c93; continue; }
  pc = 0x40a2c86; continue;
 }
 0x40a2c86 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x40a2c8f); f_040a2be7(cpu); if cpu.returned_to != 0x40a2c8f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x40a2c93 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x40a2ca4); f_040a2be7(cpu); if cpu.returned_to != 0x40a2ca4 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x40a2cb0 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2cb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2cb8); }
 let mut pc: u32 = 0x40a2cb8;
 loop { match pc {
 0x40a2cb8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x40a2cfc; continue; }
  pc = 0x40a2cc9; continue;
 }
 0x40a2cc9 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a2cdf; continue; }
  pc = 0x40a2cd2; continue;
 }
 0x40a2cd2 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x40a2cdb); f_040a2bfa(cpu); if cpu.returned_to != 0x40a2cdb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x40a2cdf => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x40a2cf0); f_040a2bfa(cpu); if cpu.returned_to != 0x40a2cf0 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x40a2cfc => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2e70); }
 let mut pc: u32 = 0x40a2e70;
 loop { match pc {
 0x40a2e70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415b904u32, v); }
  cpu.push(0x40a2e8b); f_040a3d3b(cpu); if cpu.returned_to != 0x40a2e8b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a2e9b; continue; }
  pc = 0x40a2e96; continue;
 }
 0x40a2e96 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a2eac; continue; }
  pc = 0x40a2e9b; continue;
 }
 0x40a2e9b => {
  { let v: u32 = 0x3u32; cpu.w32(0x415b904u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2eac => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a2eb4); f_040a3e5f(cpu); if cpu.returned_to != 0x40a2eb4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x40a2ec1); f_040a3e35(cpu); if cpu.returned_to != 0x40a2ec1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a2ecd; continue; }
  pc = 0x40a2ec8; continue;
 }
 0x40a2ec8 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a2edb; continue; }
  pc = 0x40a2ecd; continue;
 }
 0x40a2ecd => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a2ed6); f_040a3260(cpu); if cpu.returned_to != 0x40a2ed6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a2edd; continue;
 }
 0x40a2edb => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x40a2edd; continue;
 }
 0x40a2edd => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a2f00; continue; }
  pc = 0x40a2ee5; continue;
 }
 0x40a2ee5 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x415b904u32, v); }
  cpu.push(0x40a2ef6); f_040a3e65(cpu); if cpu.returned_to != 0x40a2ef6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2f00 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a2f0a; continue; }
  pc = 0x40a2f05; continue;
 }
 0x40a2f05 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a2f0a; continue;
 }
 0x40a2f0a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a2f12); f_040a3db1(cpu); if cpu.returned_to != 0x40a2f12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x40a2f53; continue; }
  pc = 0x40a2f19; continue;
 }
 0x40a2f19 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x40a2f38; continue; }
  pc = 0x40a2f1d; continue;
 }
 0x40a2f1d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a2f38; continue; }
  pc = 0x40a2f21; continue;
 }
 0x40a2f21 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a2f2b); f_040a2fe0(cpu); if cpu.returned_to != 0x40a2f2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x415b90cu32); cpu.push(0x40a2f38); cpu.call(t); if cpu.returned_to != 0x40a2f38 { return; }
  pc = 0x40a2f38; continue;
 }
 0x40a2f38 => {
  { let v: u32 = 0x5u32; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x415b904u32, v); }
  cpu.push(0x40a2f49); f_040a3e65(cpu); if cpu.returned_to != 0x40a2f49 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2f53 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a2f59); f_040a3e65(cpu); if cpu.returned_to != 0x40a2f59 { return; }
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
pub fn f_040a2f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2f70); }
 let mut pc: u32 = 0x40a2f70;
 loop { match pc {
 0x40a2f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415b904u32, v); }
  cpu.push(0x40a2f85); f_040a3d3b(cpu); if cpu.returned_to != 0x40a2f85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a2f93; continue; }
  pc = 0x40a2f8e; continue;
 }
 0x40a2f8e => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a2fa6; continue; }
  pc = 0x40a2f93; continue;
 }
 0x40a2f93 => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415b904u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2fa6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a2fae); f_040a3e5f(cpu); if cpu.returned_to != 0x40a2fae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a2fc2; continue; }
  pc = 0x40a2fb8; continue;
 }
 0x40a2fb8 => {
  { let v: u32 = 0x5u32; cpu.w32(0x415b904u32, v); }
  pc = 0x40a2fc2; continue;
 }
 0x40a2fc2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a2fc8); f_040a3e65(cpu); if cpu.returned_to != 0x40a2fc8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2fe0); }
 let mut pc: u32 = 0x40a2fe0;
 loop { match pc {
 0x40a2fe0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a2ff2); f_040a3020(cpu); if cpu.returned_to != 0x40a2ff2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3000); }
 let mut pc: u32 = 0x40a3000;
 loop { match pc {
 0x40a3000 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a3012); f_040a30c0(cpu); if cpu.returned_to != 0x40a3012 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3020); }
 let mut pc: u32 = 0x40a3020;
 loop { match pc {
 0x40a3020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x40a3037; continue; }
  pc = 0x40a3033; continue;
 }
 0x40a3033 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x40a3039; continue;
 }
 0x40a3037 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40a3039; continue;
 }
 0x40a3039 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a303d; continue; }
  pc = 0x40a303b; continue;
 }
 0x40a303b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40a303d; continue;
 }
 0x40a303d => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40a30a5); cpu.call(t); if cpu.returned_to != 0x40a30a5 { return; }
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
pub fn f_040a30c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a30c0); }
 let mut pc: u32 = 0x40a30c0;
 loop { match pc {
 0x40a30c0 => {
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
  if !cpu.cf { pc = 0x40a30f9; continue; }
  pc = 0x40a30f5; continue;
 }
 0x40a30f5 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x40a30fb; continue;
 }
 0x40a30f9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a30fb; continue;
 }
 0x40a30fb => {
  if (cpu.cf || cpu.zf) { pc = 0x40a3101; continue; }
  pc = 0x40a30fd; continue;
 }
 0x40a30fd => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x40a3103; continue;
 }
 0x40a3101 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  pc = 0x40a3103; continue;
 }
 0x40a3103 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40a3146); cpu.call(t); if cpu.returned_to != 0x40a3146 { return; }
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
pub fn f_040a3160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3160); }
 let mut pc: u32 = 0x40a3160;
 loop { match pc {
 0x40a3160 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40a318c); cpu.call(t); if cpu.returned_to != 0x40a318c { return; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a31a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a31a0); }
 let mut pc: u32 = 0x40a31a0;
 loop { match pc {
 0x40a31a0 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40a31c7); cpu.call(t); if cpu.returned_to != 0x40a31c7 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40a31d2; continue; }
  pc = 0x40a31ce; continue;
 }
 0x40a31ce => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a3205; continue;
 }
 0x40a31d2 => {
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
  pc = 0x40a3205; continue;
 }
 0x40a3205 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a3226; continue; }
  pc = 0x40a3209; continue;
 }
 0x40a3209 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a3223); f_040a30c0(cpu); if cpu.returned_to != 0x40a3223 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a3226; continue;
 }
 0x40a3226 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3230); }
 let mut pc: u32 = 0x40a3230;
 loop { match pc {
 0x40a3230 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a3253; continue; }
  pc = 0x40a3239; continue;
 }
 0x40a3239 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a3246); f_040a3020(cpu); if cpu.returned_to != 0x40a3246 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x415b90cu32); cpu.push(0x40a3253); cpu.call(t); if cpu.returned_to != 0x40a3253 { return; }
  pc = 0x40a3253; continue;
 }
 0x40a3253 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3260); }
 let mut pc: u32 = 0x40a3260;
 loop { match pc {
 0x40a3260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x415b908u32); cpu.push(0x40a3270); cpu.call(t); if cpu.returned_to != 0x40a3270 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a3288; continue; }
  pc = 0x40a3276; continue;
 }
 0x40a3276 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a3280); f_040a30c0(cpu); if cpu.returned_to != 0x40a3280 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a3288; continue;
 }
 0x40a3288 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3290); }
 let mut pc: u32 = 0x40a3290;
 loop { match pc {
 0x40a3290 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415b90cu32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415b90cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a32be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a32be); }
 let mut pc: u32 = 0x40a32be;
 loop { match pc {
 0x40a32be => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415baacu32), 0x0u32); }
  if !cpu.zf { pc = 0x40a32d2; continue; }
  pc = 0x40a32c8; continue;
 }
 0x40a32c8 => {
  { cpu.sub16(cpu.r16(0x415bab0u32), 0x0u16); }
  if cpu.zf { pc = 0x40a32df; continue; }
  pc = 0x40a32d2; continue;
 }
 0x40a32d2 => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x415baacu32); cpu.push(cpu.segments[1] as u32); cpu.push(0x40a32dd); cpu.call(t); if cpu.returned_to != 0x40a32dd { return; }
  pc = 0x40a3340; continue;
 }
 0x40a32df => {
  { let v: u32 = cpu.r32(0x415ba9cu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x40a32eb; continue;
 }
 0x40a32eb => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a32f1; continue;
 }
 0x40a32f1 => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub32(cpu.ecx, cpu.r32(0x415ba9cu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x40a32df; continue; }
  pc = 0x40a32fd; continue;
 }
 0x40a32fd => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a3328; continue; }
  pc = 0x40a3304; continue;
 }
 0x40a3304 => {
  { let v: u32 = cpu.r32(0x415babeu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x40a3314; continue; }
  pc = 0x40a330f; continue;
 }
 0x40a330f => {
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  pc = 0x40a3314; continue;
 }
 0x40a3314 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.cf { pc = 0x40a331a; continue; }
  pc = 0x40a3318; continue;
 }
 0x40a3318 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a331a; continue;
 }
 0x40a331a => {
  { let v: u32 = 0x3e8u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = 0x1234ddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x40a3328; continue;
 }
 0x40a3328 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415bac2u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x415baa4u32)); cpu.eax = v; }
  pc = 0x40a3340; continue;
 }
 0x40a3340 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x415baa0u32)); }
  if !cpu.cf { pc = 0x40a334d; continue; }
  pc = 0x40a3348; continue;
 }
 0x40a3348 => {
  { let v: u32 = cpu.r32(0x415baa0u32); cpu.eax = v; }
  pc = 0x40a334d; continue;
 }
 0x40a334d => {
  { let v: u32 = cpu.eax; cpu.w32(0x415baa0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3354); }
 let mut pc: u32 = 0x40a3354;
 loop { match pc {
 0x40a3354 => {
  { cpu.sub32(cpu.r32(0x415baa0u32), 0x0u32); }
  if cpu.zf { pc = 0x40a3371; continue; }
  pc = 0x40a335d; continue;
 }
 0x40a335d => {
  cpu.push(0x40a3362); f_040a32be(cpu); if cpu.returned_to != 0x40a3362 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x415ba9cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415baa4u32, v); }
  pc = 0x40a3371; continue;
 }
 0x40a3371 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3372); }
 let mut pc: u32 = 0x40a3372;
 loop { match pc {
 0x40a3372 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x415baceu32); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x415ba9cu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x415ba9cu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x415bac6u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a350d; continue; }
  pc = 0x40a338e; continue;
 }
 0x40a338e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  { let v = cpu.r16(0x415baceu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x415bac6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x415bac6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x415c4e0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x415c4e4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x415c4e0u32; cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x415baacu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40a33ce; continue; }
  pc = 0x40a33c4; continue;
 }
 0x40a33c4 => {
  { cpu.sub16(cpu.r16(0x415bab0u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x40a33f4; continue; }
  pc = 0x40a33ce; continue;
 }
 0x40a33ce => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x415baacu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40a33d3); cpu.push(cpu.segments[1] as u32); cpu.push(0x40a33d9); cpu.call(t); if cpu.returned_to != 0x40a33d9 { return; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x415baa8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40a3415; continue; }
  pc = 0x40a33e7; continue;
 }
 0x40a33e7 => {
  { let v: u32 = cpu.r32(0x415bac2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415baa8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40a3435; continue;
 }
 0x40a33f4 => {
  { let v: u32 = cpu.r32(0x415ba9cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x415baa8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40a340f; continue; }
  pc = 0x40a3402; continue;
 }
 0x40a3402 => {
  { let v: u32 = cpu.r32(0x415bac2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415baa8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40a3435; continue;
 }
 0x40a340f => {
  { let v: u32 = cpu.r32(0x415bac2u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x40a3415; continue;
 }
 0x40a3415 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x415baa8u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415baa8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xf4240u32); }
  if cpu.cf { pc = 0x40a3435; continue; }
  pc = 0x40a3430; continue;
 }
 0x40a3430 => {
  { let v: u32 = 0xf4240u32; cpu.edx = v; }
  pc = 0x40a3435; continue;
 }
 0x40a3435 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x40a343a; continue;
 }
 0x40a343a => {
  { cpu.sub32(cpu.r32(0x415b95cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if !cpu.zf { pc = 0x40a3467; continue; }
  pc = 0x40a3443; continue;
 }
 0x40a3443 => {
  { let v: u32 = cpu.r32(0x415b99cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x40a344b; continue;
 }
 0x40a344b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x415b9dcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if cpu.cf { pc = 0x40a3461; continue; }
  pc = 0x40a3453; continue;
 }
 0x40a3453 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x415b9dcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x415ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x415ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40a344b; continue;
 }
 0x40a3461 => {
  { let v: u32 = cpu.eax; cpu.w32(0x415b99cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40a3467; continue;
 }
 0x40a3467 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x40a343a; continue; }
  pc = 0x40a346f; continue;
 }
 0x40a346f => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  {  }
  { cpu.sub32(cpu.r32(0x415bacau32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a34a8; continue; }
  pc = 0x40a347d; continue;
 }
 0x40a347d => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x40a3482; continue;
 }
 0x40a3482 => {
  { cpu.sub32(cpu.r32(0x415ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x40a34a0; continue; }
  pc = 0x40a348b; continue;
 }
 0x40a348b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x415ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x415ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x415ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b91cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40a3498); cpu.push(0x40a349e); cpu.call(t); if cpu.returned_to != 0x40a349e { return; }
  cpu.poll(); pc = 0x40a3482; continue;
 }
 0x40a34a0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x40a3482; continue; }
  pc = 0x40a34a8; continue;
 }
 0x40a34a8 => {
  { cpu.sub32(cpu.r32(0x415ba58u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x40a34f1; continue; }
  pc = 0x40a34b1; continue;
 }
 0x40a34b1 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x415ba58u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x415ba58u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x415c4e0u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x415c4e4u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x415bac6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x415bac6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x415bab2u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x415bab6u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
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
 0x40a34f1 => {
  { let v = cpu.r16(0x415c4e0u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x415c4e4u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x415bac6u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x415bac6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40a350c); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40a350d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40a3514); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3515(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3515); }
 let mut pc: u32 = 0x40a3515;
 loop { match pc {
 0x40a3515 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v: u8 = 0x34u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415babeu32, v); }
  pc = 0x40a3528; continue;
 }
 0x40a3528 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a352e; continue;
 }
 0x40a352e => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x40a353b; continue; }
  pc = 0x40a353a; continue;
 }
 0x40a353a => {
  {  }
  pc = 0x40a353b; continue;
 }
 0x40a353b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a353f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a353f); }
 let mut pc: u32 = 0x40a353f;
 loop { match pc {
 0x40a353f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xd68du32); }
  if !cpu.cf { pc = 0x40a3562; continue; }
  pc = 0x40a3551; continue;
 }
 0x40a3551 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20bcu32; cpu.ebx = v; }
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x40a3562; continue;
 }
 0x40a3562 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a3568); f_040a3515(cpu); if cpu.returned_to != 0x40a3568 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a356e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a356e); }
 let mut pc: u32 = 0x40a356e;
 loop { match pc {
 0x40a356e => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x40a3570); cpu.push(0x40a3575); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3575 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x40a3582; continue;
 }
 0x40a3582 => {
  { cpu.sub32(cpu.r32(0x415b95cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x40a3597; continue; }
  pc = 0x40a358b; continue;
 }
 0x40a358b => {
  { let v: u32 = cpu.r32(0x415b9dcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x40a3597; continue; }
  pc = 0x40a3595; continue;
 }
 0x40a3595 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x40a3597; continue;
 }
 0x40a3597 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x40a3582; continue; }
  pc = 0x40a359f; continue;
 }
 0x40a359f => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x415bac2u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x40a35cc; continue; }
  pc = 0x40a35a7; continue;
 }
 0x40a35a7 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x415bac2u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x40a35ae); cpu.push(0x40a35b3); f_040a353f(cpu); if cpu.returned_to != 0x40a35b3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x415b99cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.flat_ds(0x40a35c7); cpu.push(0x40a35cc); f_040a3354(cpu); if cpu.returned_to != 0x40a35cc { return; }
  pc = 0x40a35cc; continue;
 }
 0x40a35cc => {
  cpu.flat_ds(0x40a35cc); cpu.push(0x40a35d1); f_0409cfc0(cpu); if cpu.returned_to != 0x40a35d1 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.flat_ds(0x40a35d3); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a36ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a36ea); }
 let mut pc: u32 = 0x40a36ea;
 loop { match pc {
 0x40a36ea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x40a36ef); cpu.push(0x40a36f4); f_040a3515(cpu); if cpu.returned_to != 0x40a36f4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415bab2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x415bab6u32.wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x40a371d; continue; }
  pc = 0x40a371c; continue;
 }
 0x40a371c => {
  {  }
  pc = 0x40a371d; continue;
 }
 0x40a371d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40a3720); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3721(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3721); }
 let mut pc: u32 = 0x40a3721;
 loop { match pc {
 0x40a3721 => {
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
pub fn f_040a3749(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3749); }
 let mut pc: u32 = 0x40a3749;
 loop { match pc {
 0x40a3749 => {
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
pub fn f_040a376c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a376c); } r_040a376c(cpu, 0x40a376c); }
pub fn f_040a3773(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a3773); } r_040a376c(cpu, 0x40a3773); }
pub fn f_040a3782(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a3782); } r_040a376c(cpu, 0x40a3782); }
pub fn f_040a3788(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a3788); } r_040a376c(cpu, 0x40a3788); }
pub fn f_040a379e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a379e); } r_040a376c(cpu, 0x40a379e); }
pub fn f_040a37a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a37a8); } r_040a376c(cpu, 0x40a37a8); }
pub fn f_040a37af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a37af); } r_040a376c(cpu, 0x40a37af); }
fn r_040a376c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a376c => {
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
 0x40a3773 => {
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
 0x40a3782 => {
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
 0x40a3788 => {
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
 0x40a379e => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x40a37a8 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x40a37af => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a37b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a37b6); }
 let mut pc: u32 = 0x40a37b6;
 loop { match pc {
 0x40a37b6 => {
  { let v = (cpu.ebx as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40a37c0); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a37c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a37c1); }
 let mut pc: u32 = 0x40a37c1;
 loop { match pc {
 0x40a37c1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffe4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.sub32(cpu.r32(0x415bad2u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a39d1; continue; }
  pc = 0x40a37d8; continue;
 }
 0x40a37d8 => {
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
  cpu.unimplemented(0x40a3832, "Lar");
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
  cpu.unimplemented(0x40a3883, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x415bae0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x415bae0u32; cpu.eax = v; }
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
  cpu.unimplemented(0x40a38e6, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40a376cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x40a376cu32; cpu.eax = v; }
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
  cpu.unimplemented(0x40a3949, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x200u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40a3788u32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x40a3782u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x40a3773u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x40a37a8u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x40a37afu32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x40a379eu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415bad2u32, v); }
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x40a39a6; continue; }
  pc = 0x40a39a3; continue;
 }
 0x40a39a3 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x40a39a6; continue;
 }
 0x40a39a6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x204u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415bad6u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415badau32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  pc = 0x40a39d1; continue;
 }
 0x40a39d1 => {
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
pub fn f_040a39d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a39d7); }
 let mut pc: u32 = 0x40a39d7;
 loop { match pc {
 0x40a39d7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a3a16; continue; }
  pc = 0x40a39e6; continue;
 }
 0x40a39e6 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x415bad2u32)); }
  if !cpu.zf { pc = 0x40a3a16; continue; }
  pc = 0x40a39ee; continue;
 }
 0x40a39ee => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x40a39f6; continue; }
  pc = 0x40a39f3; continue;
 }
 0x40a39f3 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x40a39f6; continue;
 }
 0x40a39f6 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415bad6u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x415badau32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x415bad2u32, v); }
  pc = 0x40a3a16; continue;
 }
 0x40a3a16 => {
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
pub fn f_040a3a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3a20); }
 let mut pc: u32 = 0x40a3a20;
 loop { match pc {
 0x40a3a20 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x40a3a31; continue; }
  pc = 0x40a3a30; continue;
 }
 0x40a3a30 => {
  {  }
  pc = 0x40a3a31; continue;
 }
 0x40a3a31 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3a35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3a35); }
 let mut pc: u32 = 0x40a3a35;
 loop { match pc {
 0x40a3a35 => {
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
pub fn f_040a3a87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3a87); }
 let mut pc: u32 = 0x40a3a87;
 loop { match pc {
 0x40a3a87 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x40a3a92); cpu.push(0x40a3a97); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3a97 { return; }
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
  if cpu.zf { pc = 0x40a3af6; continue; }
  pc = 0x40a3ad6; continue;
 }
 0x40a3ad6 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a3af6; continue;
 }
 0x40a3af6 => {
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
  if cpu.zf { pc = 0x40a3b4f; continue; }
  pc = 0x40a3b20; continue;
 }
 0x40a3b20 => {
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
  pc = 0x40a3b4f; continue;
 }
 0x40a3b4f => {
  { let v: u32 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x40a3b54); cpu.push(0x40a3b59); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3b59 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x40a3b60); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3b61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3b61); }
 let mut pc: u32 = 0x40a3b61;
 loop { match pc {
 0x40a3b61 => {
  { let v: u32 = cpu.r32(0x415bac6u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3b67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3b67); }
 let mut pc: u32 = 0x40a3b67;
 loop { match pc {
 0x40a3b67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3b6f); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3b6f { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x40a3b74; continue;
 }
 0x40a3b74 => {
  { cpu.sub32(cpu.r32(0x415b95cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40a3b8c; continue; }
  pc = 0x40a3b7d; continue;
 }
 0x40a3b7d => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x40a3b74; continue; }
  pc = 0x40a3b85; continue;
 }
 0x40a3b85 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a3b9f; continue;
 }
 0x40a3b8c => {
  { let v: u32 = 0x1u32; cpu.w32(0x415b95cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415b91cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a3b9f; continue;
 }
 0x40a3b9f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a3ba5); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3ba5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3ba8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3ba8); }
 let mut pc: u32 = 0x40a3ba8;
 loop { match pc {
 0x40a3ba8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3bb0); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3bb0 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a3bc1; continue; }
  pc = 0x40a3bb8; continue;
 }
 0x40a3bb8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let a = cpu.r32(0x415ba5cu32.wrapping_add(cpu.ecx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x415ba5cu32.wrapping_add(cpu.ecx), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x40a3bc1; continue;
 }
 0x40a3bc1 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a3bc7); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3bc7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3bca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3bca); }
 let mut pc: u32 = 0x40a3bca;
 loop { match pc {
 0x40a3bca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3bd2); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3bd2 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a3be4; continue; }
  pc = 0x40a3bda; continue;
 }
 0x40a3bda => {
  { let v: u32 = 0x0u32; cpu.w32(0x415b95cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a3be4; continue;
 }
 0x40a3be4 => {
  cpu.push(0x40a3be9); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3be9 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3beb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3beb); }
 let mut pc: u32 = 0x40a3beb;
 loop { match pc {
 0x40a3beb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3bf1); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3bf1 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x40a3bf6; continue;
 }
 0x40a3bf6 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3bfc); f_0409bf00(cpu); if cpu.returned_to != 0x40a3bfc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a3bf6; continue; }
  pc = 0x40a3c04; continue;
 }
 0x40a3c04 => {
  cpu.push(0x40a3c09); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3c09 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3c0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3c0b); }
 let mut pc: u32 = 0x40a3c0b;
 loop { match pc {
 0x40a3c0b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3c13); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3c13 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a3c2e; continue; }
  pc = 0x40a3c1b; continue;
 }
 0x40a3c1b => {
  { cpu.sub32(cpu.r32(0x415b95cu32.wrapping_add(cpu.ecx)), 0x1u32); }
  if !cpu.zf { pc = 0x40a3c2e; continue; }
  pc = 0x40a3c24; continue;
 }
 0x40a3c24 => {
  { let v: u32 = 0x2u32; cpu.w32(0x415b95cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a3c2e; continue;
 }
 0x40a3c2e => {
  cpu.push(0x40a3c33); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3c33 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3c35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3c35); }
 let mut pc: u32 = 0x40a3c35;
 loop { match pc {
 0x40a3c35 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3c3b); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3c3b { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x40a3c40; continue;
 }
 0x40a3c40 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3c46); f_0409c160(cpu); if cpu.returned_to != 0x40a3c46 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a3c40; continue; }
  pc = 0x40a3c4e; continue;
 }
 0x40a3c4e => {
  cpu.push(0x40a3c53); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3c53 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3c55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3c55); }
 let mut pc: u32 = 0x40a3c55;
 loop { match pc {
 0x40a3c55 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3c5d); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3c5d { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a3c78; continue; }
  pc = 0x40a3c65; continue;
 }
 0x40a3c65 => {
  { cpu.sub32(cpu.r32(0x415b95cu32.wrapping_add(cpu.ecx)), 0x2u32); }
  if !cpu.zf { pc = 0x40a3c78; continue; }
  pc = 0x40a3c6e; continue;
 }
 0x40a3c6e => {
  { let v: u32 = 0x1u32; cpu.w32(0x415b95cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a3c78; continue;
 }
 0x40a3c78 => {
  cpu.push(0x40a3c7d); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3c7d { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3c7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3c7f); }
 let mut pc: u32 = 0x40a3c7f;
 loop { match pc {
 0x40a3c7f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3c85); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3c85 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x40a3c8a; continue;
 }
 0x40a3c8a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a3c90); f_0409c030(cpu); if cpu.returned_to != 0x40a3c90 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a3c8a; continue; }
  pc = 0x40a3c98; continue;
 }
 0x40a3c98 => {
  cpu.push(0x40a3c9d); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3c9d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3c9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3c9f); }
 let mut pc: u32 = 0x40a3c9f;
 loop { match pc {
 0x40a3c9f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a3caa); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3caa { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b9dcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x415b99cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x40a3cc5); f_040a356e(cpu); if cpu.returned_to != 0x40a3cc5 { return; }
  cpu.push(0x40a3cca); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3cca { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3ccf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3ccf); }
 let mut pc: u32 = 0x40a3ccf;
 loop { match pc {
 0x40a3ccf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3cd7); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3cd7 { return; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x40a3cef); f_0409c470(cpu); if cpu.returned_to != 0x40a3cef { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x40a3cf7); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3cf7 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3cf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3cf9); }
 let mut pc: u32 = 0x40a3cf9;
 loop { match pc {
 0x40a3cf9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x40a3d01); f_0409cfd0(cpu); if cpu.returned_to != 0x40a3d01 { return; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a3d0e; continue; }
  pc = 0x40a3d07; continue;
 }
 0x40a3d07 => {
  { let v: u32 = 0xd68du32; cpu.eax = v; }
  pc = 0x40a3d22; continue;
 }
 0x40a3d0e => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x2e9cu32; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  pc = 0x40a3d22; continue;
 }
 0x40a3d22 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x40a3d2b); f_0409c470(cpu); if cpu.returned_to != 0x40a3d2b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x40a3d33); f_0409cfc0(cpu); if cpu.returned_to != 0x40a3d33 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3d35); }
 let mut pc: u32 = 0x40a3d35;
 loop { match pc {
 0x40a3d35 => {
  { let v: u32 = cpu.r32(0x415babeu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3d3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3d3b); }
 let mut pc: u32 = 0x40a3d3b;
 loop { match pc {
 0x40a3d3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x40a3d50; continue; }
  pc = 0x40a3d4c; continue;
 }
 0x40a3d4c => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40a3d50; continue;
 }
 0x40a3d50 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3d57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3d57); }
 let mut pc: u32 = 0x40a3d57;
 loop { match pc {
 0x40a3d57 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x40a3d6b; continue; }
  pc = 0x40a3d67; continue;
 }
 0x40a3d67 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40a3d6b; continue;
 }
 0x40a3d6b => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3d72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3d72); }
 let mut pc: u32 = 0x40a3d72;
 loop { match pc {
 0x40a3d72 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x40a3d9d; continue; }
  pc = 0x40a3d83; continue;
 }
 0x40a3d83 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x40a3d90; continue; }
  pc = 0x40a3d89; continue;
 }
 0x40a3d89 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a3dac; continue;
 }
 0x40a3d90 => {
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { cpu.poll(); pc = 0x40a3d89; continue; }
  pc = 0x40a3d98; continue;
 }
 0x40a3d98 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  pc = 0x40a3dac; continue;
 }
 0x40a3d9d => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40a3dac; continue;
 }
 0x40a3dac => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3db1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3db1); }
 let mut pc: u32 = 0x40a3db1;
 loop { match pc {
 0x40a3db1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x40a3dba; continue;
 }
 0x40a3dba => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a3dcc; continue; }
  pc = 0x40a3dc7; continue;
 }
 0x40a3dc7 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x40a3dcc; continue;
 }
 0x40a3dcc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x40a3de9; continue; }
  pc = 0x40a3dd7; continue;
 }
 0x40a3dd7 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a3de9; continue; }
  pc = 0x40a3dde; continue;
 }
 0x40a3dde => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x40a3de9; continue; }
  pc = 0x40a3de4; continue;
 }
 0x40a3de4 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40a3dba; continue; }
  pc = 0x40a3de9; continue;
 }
 0x40a3de9 => {
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
pub fn f_040a3df3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3df3); }
 let mut pc: u32 = 0x40a3df3;
 loop { match pc {
 0x40a3df3 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x40a3dfc; continue;
 }
 0x40a3dfc => {
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a3e0e; continue; }
  pc = 0x40a3e09; continue;
 }
 0x40a3e09 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x40a3e0e; continue;
 }
 0x40a3e0e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x40a3e2b; continue; }
  pc = 0x40a3e19; continue;
 }
 0x40a3e19 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a3e2b; continue; }
  pc = 0x40a3e20; continue;
 }
 0x40a3e20 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x40a3e2b; continue; }
  pc = 0x40a3e26; continue;
 }
 0x40a3e26 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x40a3dfc; continue; }
  pc = 0x40a3e2b; continue;
 }
 0x40a3e2b => {
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
pub fn f_040a3e35(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a3e35); } r_040a3e35(cpu, 0x40a3e35); }
pub fn f_040a3e5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a3e5f); } r_040a3e35(cpu, 0x40a3e5f); }
fn r_040a3e35(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a3e35 => {
  { let v: u16 = 0x4200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40a3e39; continue;
 }
 0x40a3e39 => {
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
 0x40a3e5f => {
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40a3e39; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3e65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3e65); }
 let mut pc: u32 = 0x40a3e65;
 loop { match pc {
 0x40a3e65 => {
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
pub fn f_040a3e73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3e73); }
 let mut pc: u32 = 0x40a3e73;
 loop { match pc {
 0x40a3e73 => {
  { let v = 0x415c4e8u32; cpu.push(v); }
  { let v = 0x415b91cu32; cpu.push(v); }
  cpu.push(0x40a3e82); f_040a30c0(cpu); if cpu.returned_to != 0x40a3e82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40a3e73u32; cpu.push(v); }
  { let v = 0x40a32b0u32; cpu.push(v); }
  cpu.push(0x40a3e94); f_040a30c0(cpu); if cpu.returned_to != 0x40a3e94 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3ea0); }
 let mut pc: u32 = 0x40a3ea0;
 loop { match pc {
 0x40a3ea0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c5ecu32), 0x0u32); }
  if cpu.zf { pc = 0x40a3f23; continue; }
  pc = 0x40a3eac; continue;
 }
 0x40a3eac => {
  { let v = 0x40a3ea0u32; cpu.push(v); }
  { let v = 0x40a4b90u32; cpu.push(v); }
  cpu.push(0x40a3ebb); f_040a3020(cpu); if cpu.returned_to != 0x40a3ebb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x42c5da0u32; cpu.push(v); }
  cpu.push(0x40a3eca); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3eca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x42c5ee0u32; cpu.push(v); }
  cpu.push(0x40a3ed9); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3ed9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x42c5de0u32; cpu.push(v); }
  cpu.push(0x40a3eeb); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3eeb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x42c5f30u32; cpu.push(v); }
  cpu.push(0x40a3efa); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3efa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c5e8u32; cpu.push(v); }
  cpu.push(0x40a3f09); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3f09 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f48u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40a3f1a); f_040a2fe0(cpu); if cpu.returned_to != 0x40a3f1a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c5ecu32, v); }
  pc = 0x40a3f23; continue;
 }
 0x40a3f23 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3f30); }
 let mut pc: u32 = 0x40a3f30;
 loop { match pc {
 0x40a3f30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x40a3f3d); f_040a32be(cpu); if cpu.returned_to != 0x40a3f3d { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x40a3f3f; continue;
 }
 0x40a3f3f => {
  cpu.push(0x40a3f44); f_040a32be(cpu); if cpu.returned_to != 0x40a3f44 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x40a3f3f; continue; }
  pc = 0x40a3f48; continue;
 }
 0x40a3f48 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3f50); }
 let mut pc: u32 = 0x40a3f50;
 loop { match pc {
 0x40a3f50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40a3f65; continue; }
  pc = 0x40a3f5b; continue;
 }
 0x40a3f5b => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x415c4e8u32, v); }
  pc = 0x40a3fab; continue;
 }
 0x40a3f65 => {
  { let v: u32 = 0x415c4e8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a3f6c; continue;
 }
 0x40a3f6c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a3f6c; continue; }
  pc = 0x40a3f76; continue;
 }
 0x40a3f76 => {
  { let v: u32 = 0x415c4e8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a3fab; continue; }
  pc = 0x40a3f8b; continue;
 }
 0x40a3f8b => {
  { let v: u8 = cpu.r8(0x415c4e7u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x40a3fab; continue; }
  pc = 0x40a3f96; continue;
 }
 0x40a3f96 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3au8); }
  if cpu.zf { pc = 0x40a3fab; continue; }
  pc = 0x40a3f9b; continue;
 }
 0x40a3f9b => {
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x415c4e8u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x415c4e9u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a3fab; continue;
 }
 0x40a3fab => {
  { let v: u32 = 0x415c4e8u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3fc0); }
 let mut pc: u32 = 0x40a3fc0;
 loop { match pc {
 0x40a3fc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x40a3fc6; continue;
 }
 0x40a3fc6 => {
  { let v: u32 = cpu.r32(0x42c5da0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a3fd9; continue; }
  pc = 0x40a3fd0; continue;
 }
 0x40a3fd0 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a3fd6); f_0409bbc0(cpu); if cpu.returned_to != 0x40a3fd6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a3fd9; continue;
 }
 0x40a3fd9 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xfffffffcu32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a3fc6; continue; }
  pc = 0x40a3fe1; continue;
 }
 0x40a3fe1 => {
  cpu.push(0x40a3fe6); f_0409be80(cpu); if cpu.returned_to != 0x40a3fe6 { return; }
  cpu.push(0x40a3feb); f_040a36ea(cpu); if cpu.returned_to != 0x40a3feb { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a3ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a3ff0); }
 let mut pc: u32 = 0x40a3ff0;
 loop { match pc {
 0x40a35d4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415baceu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x415baceu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x40a35ea); cpu.push(0x40a35ef); f_040a3e73(cpu); if cpu.returned_to != 0x40a35ef { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x415bac6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x415bacau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x415bac2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x415bad2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x415b95cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x415b99cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x415b9dcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x415ba1cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = 0x1684u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = (cpu.edi as u16); cpu.set_segment(0, v as u16); }
  { cpu.interrupt(0x2f); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x415baacu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x415baaeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415bab0u32.wrapping_add(cpu.bases[3]), v); }
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
  { let v: u32 = cpu.ebx; cpu.w32(0x415bab2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x415bab6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x415babau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x40a3372u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2u32; cpu.w32(0x415b998u32.wrapping_add(cpu.bases[3]), v); }
  { let v = 0xd68du32; cpu.push(v); }
  { let v = 0x3cu32; cpu.push(v); }
  cpu.flat_ds(0x40a36d1); cpu.push(0x40a36d6); f_0409c470(cpu); if cpu.returned_to != 0x40a36d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x40a36e4; continue; }
  pc = 0x40a36e3; continue;
 }
 0x40a36e3 => {
  {  }
  pc = 0x40a36e4; continue;
 }
 0x40a36e4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40a36e9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a3ff0 => {
  cpu.flat_ds(0x40a3ff0); cpu.push(0x40a3ff5); f_040a4b90(cpu); if cpu.returned_to != 0x40a3ff5 { return; }
  { let v = 0xc8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x40a3ffc); cpu.push(0x40a4001); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4001 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.flat_ds(0x40a4008); cpu.push(0x40a400d); f_0409cfe0(cpu); if cpu.returned_to != 0x40a400d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.flat_ds(0x40a4017); cpu.push(0x40a401c); f_0409cfe0(cpu); if cpu.returned_to != 0x40a401c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.flat_ds(0x40a4023); cpu.push(0x40a4028); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4028 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.flat_ds(0x40a402f); cpu.push(0x40a4034); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4034 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  cpu.flat_ds(0x40a403b); cpu.push(0x40a4040); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4040 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x28fu32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  cpu.flat_ds(0x40a404a); cpu.push(0x40a404f); f_0409cfe0(cpu); if cpu.returned_to != 0x40a404f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.flat_ds(0x40a4056); cpu.push(0x40a405b); f_0409cfe0(cpu); if cpu.returned_to != 0x40a405b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.flat_ds(0x40a4062); cpu.push(0x40a4067); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4067 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  cpu.flat_ds(0x40a406e); cpu.push(0x40a4073); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4073 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.flat_ds(0x40a407a); cpu.push(0x40a407f); f_0409cfe0(cpu); if cpu.returned_to != 0x40a407f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x78u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.flat_ds(0x40a4086); cpu.push(0x40a408b); f_0409cfe0(cpu); if cpu.returned_to != 0x40a408b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.flat_ds(0x40a4092); cpu.push(0x40a4097); f_0409cfe0(cpu); if cpu.returned_to != 0x40a4097 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xdu32; cpu.push(v); }
  cpu.flat_ds(0x40a409e); cpu.push(0x40a40a3); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  cpu.flat_ds(0x40a40aa); cpu.push(0x40a40af); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  cpu.flat_ds(0x40a40b6); cpu.push(0x40a40bb); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.flat_ds(0x40a40c2); cpu.push(0x40a40c7); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.flat_ds(0x40a40ce); cpu.push(0x40a40d3); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.flat_ds(0x40a40da); cpu.push(0x40a40df); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x13u32; cpu.push(v); }
  cpu.flat_ds(0x40a40e6); cpu.push(0x40a40eb); f_0409cfe0(cpu); if cpu.returned_to != 0x40a40eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a40f0; continue;
 }
 0x40a40f0 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c5d9cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a40f0; continue; }
  pc = 0x40a4100; continue;
 }
 0x40a4100 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x42c5de0u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40a35d4; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4110); }
 let mut pc: u32 = 0x40a4110;
 loop { match pc {
 0x40a4110 => {
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
  if cpu.zf { pc = 0x40a41ae; continue; }
  pc = 0x40a4134; continue;
 }
 0x40a4134 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b910u32); cpu.push(0x40a413b); cpu.call(t); if cpu.returned_to != 0x40a413b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a41ae; continue; }
  pc = 0x40a4141; continue;
 }
 0x40a4141 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a415b; continue; }
  pc = 0x40a4156; continue;
 }
 0x40a4156 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x40a415b; continue;
 }
 0x40a415b => {
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
  cpu.push(0x40a4188); f_0409cb30(cpu); if cpu.returned_to != 0x40a4188 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a4197; continue; }
  pc = 0x40a4190; continue;
 }
 0x40a4190 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a4197 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x42c5f4cu32; cpu.edi = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x42c5f4cu32; cpu.eax = v; }
  { cpu.movs(4, true); }
  pc = 0x40a41ae; continue;
 }
 0x40a41ae => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a41c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a41c0); }
 let mut pc: u32 = 0x40a41c0;
 loop { match pc {
 0x40a41c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a41d8; continue; }
  pc = 0x40a41ce; continue;
 }
 0x40a41ce => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x40a41d5); cpu.call(t); if cpu.returned_to != 0x40a41d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a41d8; continue;
 }
 0x40a41d8 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a41e9; continue; }
  pc = 0x40a41e0; continue;
 }
 0x40a41e0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a41e6); f_0409bf00(cpu); if cpu.returned_to != 0x40a41e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a41e9; continue;
 }
 0x40a41e9 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a4212; continue; }
  pc = 0x40a41ef; continue;
 }
 0x40a41ef => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a4200; continue; }
  pc = 0x40a41f7; continue;
 }
 0x40a41f7 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a41fd); f_0409cc80(cpu); if cpu.returned_to != 0x40a41fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a4200; continue;
 }
 0x40a4200 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a420f); f_0409cb30(cpu); if cpu.returned_to != 0x40a420f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a4212; continue;
 }
 0x40a4212 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x40a4220); f_0409ce00(cpu); if cpu.returned_to != 0x40a4220 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a4233); f_040a3160(cpu); if cpu.returned_to != 0x40a4233 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a423c); f_040a3230(cpu); if cpu.returned_to != 0x40a423c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a4241; continue;
 }
 0x40a4241 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42c5da0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40a4251; continue; }
  pc = 0x40a4249; continue;
 }
 0x40a4249 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42c5da0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a4251; continue;
 }
 0x40a4251 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a4241; continue; }
  pc = 0x40a4259; continue;
 }
 0x40a4259 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4260); }
 let mut pc: u32 = 0x40a4260;
 loop { match pc {
 0x40a4260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x40a426f); f_040a3260(cpu); if cpu.returned_to != 0x40a426f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a4295; continue; }
  pc = 0x40a4278; continue;
 }
 0x40a4278 => {
  { let v: u32 = 0x414b040u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a4284; continue;
 }
 0x40a4284 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a4284; continue; }
  pc = 0x40a428e; continue;
 }
 0x40a428e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a4295 => {
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
  cpu.push(0x40a42af); f_040a31a0(cpu); if cpu.returned_to != 0x40a42af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a42dc; continue; }
  pc = 0x40a42b6; continue;
 }
 0x40a42b6 => {
  { let v: u32 = 0x414b06cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a42c2; continue;
 }
 0x40a42c2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a42c2; continue; }
  pc = 0x40a42cc; continue;
 }
 0x40a42cc => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a42d2); f_040a3230(cpu); if cpu.returned_to != 0x40a42d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a42dc => {
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
  { let v: u32 = 0x414b088u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a4309; continue;
 }
 0x40a4309 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4317; continue; }
  pc = 0x40a4311; continue;
 }
 0x40a4311 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4317; continue; }
  pc = 0x40a4315; continue;
 }
 0x40a4315 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4317; continue;
 }
 0x40a4317 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4324; continue; }
  pc = 0x40a431c; continue;
 }
 0x40a431c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4324; continue; }
  pc = 0x40a4321; continue;
 }
 0x40a4321 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a4324; continue;
 }
 0x40a4324 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a4333; continue; }
  pc = 0x40a4328; continue;
 }
 0x40a4328 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a4333; continue; }
  pc = 0x40a432c; continue;
 }
 0x40a432c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a4333; continue; }
  pc = 0x40a432f; continue;
 }
 0x40a432f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4309; continue;
 }
 0x40a4333 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a433a; continue; }
  pc = 0x40a4338; continue;
 }
 0x40a4338 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a433a; continue;
 }
 0x40a433a => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a434a; continue; }
  pc = 0x40a433e; continue;
 }
 0x40a433e => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a43ce; continue;
 }
 0x40a434a => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x414b090u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x40a4357; continue;
 }
 0x40a4357 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4365; continue; }
  pc = 0x40a435f; continue;
 }
 0x40a435f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4365; continue; }
  pc = 0x40a4363; continue;
 }
 0x40a4363 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4365; continue;
 }
 0x40a4365 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4372; continue; }
  pc = 0x40a436a; continue;
 }
 0x40a436a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4372; continue; }
  pc = 0x40a436f; continue;
 }
 0x40a436f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a4372; continue;
 }
 0x40a4372 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a4381; continue; }
  pc = 0x40a4376; continue;
 }
 0x40a4376 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a4381; continue; }
  pc = 0x40a437a; continue;
 }
 0x40a437a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a4381; continue; }
  pc = 0x40a437d; continue;
 }
 0x40a437d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4357; continue;
 }
 0x40a4381 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a4388; continue; }
  pc = 0x40a4386; continue;
 }
 0x40a4386 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4388; continue;
 }
 0x40a4388 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40a43c7; continue; }
  pc = 0x40a438c; continue;
 }
 0x40a438c => {
  { let v: u32 = 0x414b098u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a4398; continue;
 }
 0x40a4398 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a4398; continue; }
  pc = 0x40a43a2; continue;
 }
 0x40a43a2 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a43b4); f_040a3160(cpu); if cpu.returned_to != 0x40a43b4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a43bd); f_040a3230(cpu); if cpu.returned_to != 0x40a43bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a43c7 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a43ce; continue;
 }
 0x40a43ce => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a43d2; continue;
 }
 0x40a43d2 => {
  { cpu.sub32(cpu.r32(0x42c5da0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x40a43ea; continue; }
  pc = 0x40a43db; continue;
 }
 0x40a43db => {
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5da0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a43f3; continue;
 }
 0x40a43ea => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { cpu.poll(); pc = 0x40a43d2; continue; }
  pc = 0x40a43f3; continue;
 }
 0x40a43f3 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x40a4432; continue; }
  pc = 0x40a43f8; continue;
 }
 0x40a43f8 => {
  { let v: u32 = 0x414b0b0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a4404; continue;
 }
 0x40a4404 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a4404; continue; }
  pc = 0x40a440e; continue;
 }
 0x40a440e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a441f); f_040a3160(cpu); if cpu.returned_to != 0x40a441f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a4428); f_040a3230(cpu); if cpu.returned_to != 0x40a4428 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a4432 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x66u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a4450); f_0409cec0(cpu); if cpu.returned_to != 0x40a4450 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x36u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x34u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x40a4470); f_0409ce00(cpu); if cpu.returned_to != 0x40a4470 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x300u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a4490); f_0409cb30(cpu); if cpu.returned_to != 0x40a4490 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a44ab; continue; }
  pc = 0x40a449f; continue;
 }
 0x40a449f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a4537; continue;
 }
 0x40a44ab => {
  { let v = 0x40a4b30u32; cpu.push(v); }
  cpu.push(0x40a44b5); f_0409c640(cpu); if cpu.returned_to != 0x40a44b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a450b; continue; }
  pc = 0x40a44c0; continue;
 }
 0x40a44c0 => {
  { let v: u32 = 0x414b0c8u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a44cc; continue;
 }
 0x40a44cc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a44cc; continue; }
  pc = 0x40a44d6; continue;
 }
 0x40a44d6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x40a44e4); f_0409ce00(cpu); if cpu.returned_to != 0x40a44e4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a44f8); f_040a3160(cpu); if cpu.returned_to != 0x40a44f8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a4501); f_040a3230(cpu); if cpu.returned_to != 0x40a4501 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a450b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a4512); f_0409c530(cpu); if cpu.returned_to != 0x40a4512 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a4528); f_0409c3b0(cpu); if cpu.returned_to != 0x40a4528 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a4534); f_0409c160(cpu); if cpu.returned_to != 0x40a4534 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a4537; continue;
 }
 0x40a4537 => {
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
pub fn f_040a4540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4540); }
 let mut pc: u32 = 0x40a4540;
 loop { match pc {
 0x40a4540 => {
  { let v: u32 = 0x42c5de0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4550); }
 let mut pc: u32 = 0x40a4550;
 loop { match pc {
 0x40a4550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5ee0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5ee0u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4570); }
 let mut pc: u32 = 0x40a4570;
 loop { match pc {
 0x40a4570 => {
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
  cpu.push(0x40a45d2); f_040a3d3b(cpu); if cpu.returned_to != 0x40a45d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a464a; continue; }
  pc = 0x40a45e1; continue;
 }
 0x40a45e1 => {
  { cpu.sub8(cpu.r8(0x415c4e8u32), 0x0u8); }
  if !cpu.zf { pc = 0x40a45f1; continue; }
  pc = 0x40a45ea; continue;
 }
 0x40a45ea => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a49fd; continue;
 }
 0x40a45f1 => {
  { let v: u32 = 0x415c4e8u32; cpu.esi = v; }
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a45ff; continue;
 }
 0x40a45ff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a45ff; continue; }
  pc = 0x40a4609; continue;
 }
 0x40a4609 => {
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a461f; continue;
 }
 0x40a461f => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a461f; continue; }
  pc = 0x40a4625; continue;
 }
 0x40a4625 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x228u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a4634); f_040a3d3b(cpu); if cpu.returned_to != 0x40a4634 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a464a; continue; }
  pc = 0x40a4643; continue;
 }
 0x40a4643 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a49fd; continue;
 }
 0x40a464a => {
  { let v: u32 = 0x10cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a465f; continue;
 }
 0x40a465f => {
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x40a467e); f_040a4a10(cpu); if cpu.returned_to != 0x40a467e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a498a; continue; }
  pc = 0x40a4686; continue;
 }
 0x40a4686 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40a46b8; continue; }
  pc = 0x40a46a0; continue;
 }
 0x40a46a0 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x40a46b8; continue; }
  pc = 0x40a46aa; continue;
 }
 0x40a46aa => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x325u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a46a0; continue; }
  pc = 0x40a46b8; continue;
 }
 0x40a46b8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40a46c7; continue;
 }
 0x40a46bc => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x40a46de; continue; }
  pc = 0x40a46c6; continue;
 }
 0x40a46c6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a46c7; continue;
 }
 0x40a46c7 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x40a46bc; continue; }
  pc = 0x40a46de; continue;
 }
 0x40a46de => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x40a46f4; continue;
 }
 0x40a46e9 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x40a470b; continue; }
  pc = 0x40a46f3; continue;
 }
 0x40a46f3 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a46f4; continue;
 }
 0x40a46f4 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x40a46e9; continue; }
  pc = 0x40a470b; continue;
 }
 0x40a470b => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x40a4722; continue;
 }
 0x40a4717 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x40a4739; continue; }
  pc = 0x40a4721; continue;
 }
 0x40a4721 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a4722; continue;
 }
 0x40a4722 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x40a4717; continue; }
  pc = 0x40a4739; continue;
 }
 0x40a4739 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { cpu.poll(); pc = 0x40a465f; continue; }
  pc = 0x40a475d; continue;
 }
 0x40a475d => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x3bu8); }
  if cpu.zf { cpu.poll(); pc = 0x40a465f; continue; }
  pc = 0x40a4769; continue;
 }
 0x40a4769 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x414aff4u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a4775; continue;
 }
 0x40a4775 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4783; continue; }
  pc = 0x40a477d; continue;
 }
 0x40a477d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4783; continue; }
  pc = 0x40a4781; continue;
 }
 0x40a4781 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4783; continue;
 }
 0x40a4783 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4790; continue; }
  pc = 0x40a4788; continue;
 }
 0x40a4788 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4790; continue; }
  pc = 0x40a478d; continue;
 }
 0x40a478d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a4790; continue;
 }
 0x40a4790 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a479f; continue; }
  pc = 0x40a4794; continue;
 }
 0x40a4794 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a479f; continue; }
  pc = 0x40a4798; continue;
 }
 0x40a4798 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a479f; continue; }
  pc = 0x40a479b; continue;
 }
 0x40a479b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4775; continue;
 }
 0x40a479f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a47a6; continue; }
  pc = 0x40a47a4; continue;
 }
 0x40a47a4 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a47a6; continue;
 }
 0x40a47a6 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a47c4; continue; }
  pc = 0x40a47aa; continue;
 }
 0x40a47aa => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a47b5; continue;
 }
 0x40a47b5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a47b5; continue; }
  pc = 0x40a47bf; continue;
 }
 0x40a47bf => {
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a47c4 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x414affcu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a47d0; continue;
 }
 0x40a47d0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a47de; continue; }
  pc = 0x40a47d8; continue;
 }
 0x40a47d8 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a47de; continue; }
  pc = 0x40a47dc; continue;
 }
 0x40a47dc => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a47de; continue;
 }
 0x40a47de => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a47eb; continue; }
  pc = 0x40a47e3; continue;
 }
 0x40a47e3 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a47eb; continue; }
  pc = 0x40a47e8; continue;
 }
 0x40a47e8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a47eb; continue;
 }
 0x40a47eb => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a47fa; continue; }
  pc = 0x40a47ef; continue;
 }
 0x40a47ef => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a47fa; continue; }
  pc = 0x40a47f3; continue;
 }
 0x40a47f3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a47fa; continue; }
  pc = 0x40a47f6; continue;
 }
 0x40a47f6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a47d0; continue;
 }
 0x40a47fa => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a4801; continue; }
  pc = 0x40a47ff; continue;
 }
 0x40a47ff => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4801; continue;
 }
 0x40a4801 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a481a; continue; }
  pc = 0x40a4805; continue;
 }
 0x40a4805 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a480b; continue;
 }
 0x40a480b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a480b; continue; }
  pc = 0x40a4815; continue;
 }
 0x40a4815 => {
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a481a => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x414b004u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a4826; continue;
 }
 0x40a4826 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4834; continue; }
  pc = 0x40a482e; continue;
 }
 0x40a482e => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4834; continue; }
  pc = 0x40a4832; continue;
 }
 0x40a4832 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4834; continue;
 }
 0x40a4834 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4841; continue; }
  pc = 0x40a4839; continue;
 }
 0x40a4839 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4841; continue; }
  pc = 0x40a483e; continue;
 }
 0x40a483e => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a4841; continue;
 }
 0x40a4841 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a4850; continue; }
  pc = 0x40a4845; continue;
 }
 0x40a4845 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a4850; continue; }
  pc = 0x40a4849; continue;
 }
 0x40a4849 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a4850; continue; }
  pc = 0x40a484c; continue;
 }
 0x40a484c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4826; continue;
 }
 0x40a4850 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a4857; continue; }
  pc = 0x40a4855; continue;
 }
 0x40a4855 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4857; continue;
 }
 0x40a4857 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a4875; continue; }
  pc = 0x40a485b; continue;
 }
 0x40a485b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a4865); f_040a4a90(cpu); if cpu.returned_to != 0x40a4865 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x100u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a4875 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b00cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a4881; continue;
 }
 0x40a4881 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a488f; continue; }
  pc = 0x40a4889; continue;
 }
 0x40a4889 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a488f; continue; }
  pc = 0x40a488d; continue;
 }
 0x40a488d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a488f; continue;
 }
 0x40a488f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a489c; continue; }
  pc = 0x40a4894; continue;
 }
 0x40a4894 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a489c; continue; }
  pc = 0x40a4899; continue;
 }
 0x40a4899 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a489c; continue;
 }
 0x40a489c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a48ab; continue; }
  pc = 0x40a48a0; continue;
 }
 0x40a48a0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a48ab; continue; }
  pc = 0x40a48a4; continue;
 }
 0x40a48a4 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a48ab; continue; }
  pc = 0x40a48a7; continue;
 }
 0x40a48a7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4881; continue;
 }
 0x40a48ab => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a48b2; continue; }
  pc = 0x40a48b0; continue;
 }
 0x40a48b0 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a48b2; continue;
 }
 0x40a48b2 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a48d0; continue; }
  pc = 0x40a48b6; continue;
 }
 0x40a48b6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a48c0); f_040a4a90(cpu); if cpu.returned_to != 0x40a48c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x102u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a48d0 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x414b010u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a48dc; continue;
 }
 0x40a48dc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a48ea; continue; }
  pc = 0x40a48e4; continue;
 }
 0x40a48e4 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a48ea; continue; }
  pc = 0x40a48e8; continue;
 }
 0x40a48e8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a48ea; continue;
 }
 0x40a48ea => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a48f7; continue; }
  pc = 0x40a48ef; continue;
 }
 0x40a48ef => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a48f7; continue; }
  pc = 0x40a48f4; continue;
 }
 0x40a48f4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a48f7; continue;
 }
 0x40a48f7 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a4906; continue; }
  pc = 0x40a48fb; continue;
 }
 0x40a48fb => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a4906; continue; }
  pc = 0x40a48ff; continue;
 }
 0x40a48ff => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a4906; continue; }
  pc = 0x40a4902; continue;
 }
 0x40a4902 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a48dc; continue;
 }
 0x40a4906 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a490d; continue; }
  pc = 0x40a490b; continue;
 }
 0x40a490b => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a490d; continue;
 }
 0x40a490d => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a492b; continue; }
  pc = 0x40a4911; continue;
 }
 0x40a4911 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a491b); f_040a4a90(cpu); if cpu.returned_to != 0x40a491b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x104u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a492b => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v: u32 = 0x414b01cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a4937; continue;
 }
 0x40a4937 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4945; continue; }
  pc = 0x40a493f; continue;
 }
 0x40a493f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4945; continue; }
  pc = 0x40a4943; continue;
 }
 0x40a4943 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4945; continue;
 }
 0x40a4945 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4952; continue; }
  pc = 0x40a494a; continue;
 }
 0x40a494a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4952; continue; }
  pc = 0x40a494f; continue;
 }
 0x40a494f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a4952; continue;
 }
 0x40a4952 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a4961; continue; }
  pc = 0x40a4956; continue;
 }
 0x40a4956 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a4961; continue; }
  pc = 0x40a495a; continue;
 }
 0x40a495a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a4961; continue; }
  pc = 0x40a495d; continue;
 }
 0x40a495d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a4937; continue;
 }
 0x40a4961 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a4968; continue; }
  pc = 0x40a4966; continue;
 }
 0x40a4966 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a4968; continue;
 }
 0x40a4968 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40a465f; continue; }
  pc = 0x40a4970; continue;
 }
 0x40a4970 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a497a); f_040a4a90(cpu); if cpu.returned_to != 0x40a497a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x106u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a465f; continue;
 }
 0x40a498a => {
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a4997); f_040a3e65(cpu); if cpu.returned_to != 0x40a4997 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a49c3; continue; }
  pc = 0x40a49b1; continue;
 }
 0x40a49b1 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a49e8; continue; }
  pc = 0x40a49c3; continue;
 }
 0x40a49c3 => {
  { let v: u32 = 0x414b028u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a49cf; continue;
 }
 0x40a49cf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a49cf; continue; }
  pc = 0x40a49d9; continue;
 }
 0x40a49d9 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a49e8 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x40a49fd; continue;
 }
 0x40a49fd => {
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
pub fn f_040a4a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4a10); }
 let mut pc: u32 = 0x40a4a10;
 loop { match pc {
 0x40a4a10 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x40a4a77; continue; }
  pc = 0x40a4a25; continue;
 }
 0x40a4a25 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x40a4a4f; continue; }
  pc = 0x40a4a2b; continue;
 }
 0x40a4a2b => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a4a45); f_040a3db1(cpu); if cpu.returned_to != 0x40a4a45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a4a77; continue; }
  pc = 0x40a4a4f; continue;
 }
 0x40a4a4f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xdu8); }
  if cpu.zf { pc = 0x40a4a70; continue; }
  pc = 0x40a4a60; continue;
 }
 0x40a4a60 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if !cpu.zf { pc = 0x40a4a6c; continue; }
  pc = 0x40a4a66; continue;
 }
 0x40a4a66 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a4a77; continue;
 }
 0x40a4a6c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x40a4a70; continue;
 }
 0x40a4a70 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x40a4a25; continue; }
  pc = 0x40a4a77; continue;
 }
 0x40a4a77 => {
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
pub fn f_040a4a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4a90); }
 let mut pc: u32 = 0x40a4a90;
 loop { match pc {
 0x40a4a90 => {
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
  pc = 0x40a4af8; continue;
 }
 0x40a4ab1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a4ac1; continue; }
  pc = 0x40a4ab9; continue;
 }
 0x40a4ab9 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4ac1; continue; }
  pc = 0x40a4abe; continue;
 }
 0x40a4abe => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40a4ac1; continue;
 }
 0x40a4ac1 => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x40a4acb; continue; }
  pc = 0x40a4ac6; continue;
 }
 0x40a4ac6 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a4af6; continue;
 }
 0x40a4acb => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x40a4af6; continue; }
  pc = 0x40a4ad0; continue;
 }
 0x40a4ad0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a4af2; continue; }
  pc = 0x40a4ad6; continue;
 }
 0x40a4ad6 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x414afe0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40a4aed; continue; }
  pc = 0x40a4ade; continue;
 }
 0x40a4ade => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a4af2; continue;
 }
 0x40a4aed => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a4ad6; continue; }
  pc = 0x40a4af2; continue;
 }
 0x40a4af2 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x40a4b0c; continue; }
  pc = 0x40a4af6; continue;
 }
 0x40a4af6 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x40a4af8; continue;
 }
 0x40a4af8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x40a4ab1; continue; }
  pc = 0x40a4b0c; continue;
 }
 0x40a4b0c => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a4b1d; continue; }
  pc = 0x40a4b14; continue;
 }
 0x40a4b14 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a4b1d; continue;
 }
 0x40a4b1d => {
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
pub fn f_040a4b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4b30); }
 let mut pc: u32 = 0x40a4b30;
 loop { match pc {
 0x40a4b30 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c5e8u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a4b7c; continue; }
  pc = 0x40a4b3b; continue;
 }
 0x40a4b3b => {
  { let v: u32 = 0x1u32; cpu.w32(0x415c5e8u32, v); }
  cpu.push(0x40a4b4a); f_0409cc70(cpu); if cpu.returned_to != 0x40a4b4a { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f48u32, v); }
  cpu.push(0x40a4b62); f_0409cb30(cpu); if cpu.returned_to != 0x40a4b62 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5f48u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x40a4b73); f_0409cc60(cpu); if cpu.returned_to != 0x40a4b73 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x415c5e8u32, v); }
  pc = 0x40a4b7c; continue;
 }
 0x40a4b7c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4b90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4b90); }
 let mut pc: u32 = 0x40a4b90;
 loop { match pc {
 0x40a4b90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c5ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x40a4c16; continue; }
  pc = 0x40a4b9c; continue;
 }
 0x40a4b9c => {
  { let v = 0x40a3ea0u32; cpu.push(v); }
  { let v = 0x40a4b90u32; cpu.push(v); }
  cpu.push(0x40a4bab); f_040a30c0(cpu); if cpu.returned_to != 0x40a4bab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x42c5da0u32; cpu.push(v); }
  cpu.push(0x40a4bba); f_040a3000(cpu); if cpu.returned_to != 0x40a4bba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x42c5ee0u32; cpu.push(v); }
  cpu.push(0x40a4bc9); f_040a3000(cpu); if cpu.returned_to != 0x40a4bc9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x42c5de0u32; cpu.push(v); }
  cpu.push(0x40a4bdb); f_040a3000(cpu); if cpu.returned_to != 0x40a4bdb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x42c5f30u32; cpu.push(v); }
  cpu.push(0x40a4bea); f_040a3000(cpu); if cpu.returned_to != 0x40a4bea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c5e8u32; cpu.push(v); }
  cpu.push(0x40a4bf9); f_040a3000(cpu); if cpu.returned_to != 0x40a4bf9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f48u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40a4c0d); f_040a3000(cpu); if cpu.returned_to != 0x40a4c0d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c5ecu32, v); }
  pc = 0x40a4c16; continue;
 }
 0x40a4c16 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4c20); }
 let mut pc: u32 = 0x40a4c20;
 loop { match pc {
 0x40a4c20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c674u32), 0x0u32); }
  if cpu.zf { pc = 0x40a4cd4; continue; }
  pc = 0x40a4c30; continue;
 }
 0x40a4c30 => {
  { let v = 0x40a4c20u32; cpu.push(v); }
  { let v = 0x40a69a0u32; cpu.push(v); }
  cpu.push(0x40a4c3f); f_040a3020(cpu); if cpu.returned_to != 0x40a4c3f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x415c5f0u32; cpu.push(v); }
  cpu.push(0x40a4c51); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c51 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c670u32; cpu.push(v); }
  cpu.push(0x40a4c60); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c60 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f78u32; cpu.push(v); }
  cpu.push(0x40a4c6f); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c6f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x42c5f64u32; cpu.push(v); }
  cpu.push(0x40a4c7e); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f80u32; cpu.push(v); }
  cpu.push(0x40a4c8d); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f84u32; cpu.push(v); }
  cpu.push(0x40a4c9c); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4c9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f74u32; cpu.push(v); }
  cpu.push(0x40a4cab); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4cab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f7cu32; cpu.push(v); }
  cpu.push(0x40a4cba); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4cba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f70u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40a4ccb); f_040a2fe0(cpu); if cpu.returned_to != 0x40a4ccb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c674u32, v); }
  pc = 0x40a4cd4; continue;
 }
 0x40a4cd4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4ce0); }
 let mut pc: u32 = 0x40a4ce0;
 loop { match pc {
 0x40a4ce0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a4cf6; continue; }
  pc = 0x40a4cf1; continue;
 }
 0x40a4cf1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a4cf6; continue;
 }
 0x40a4cf6 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a4cff; continue; }
  pc = 0x40a4cfa; continue;
 }
 0x40a4cfa => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a4cff; continue;
 }
 0x40a4cff => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40a4d0f; continue; }
  pc = 0x40a4d06; continue;
 }
 0x40a4d06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a4d0f; continue;
 }
 0x40a4d0f => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a4d1c; continue; }
  pc = 0x40a4d17; continue;
 }
 0x40a4d17 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a4d1c; continue;
 }
 0x40a4d1c => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4e80); }
 let mut pc: u32 = 0x40a4e80;
 loop { match pc {
 0x40a4e80 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a4ec0; continue; }
  pc = 0x40a4ea5; continue;
 }
 0x40a4ea5 => {
  cpu.poll(); pc = cpu.r32(0x40a4e80u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x40a4ead => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x40a4ec0; continue;
 }
 0x40a4eb4 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  pc = 0x40a4ec0; continue;
 }
 0x40a4ebb => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x40a4ec0; continue;
 }
 0x40a4ec0 => {
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
  { let v: u32 = cpu.r32(0x42c5ef8u32); cpu.edx = v; }
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
  if cpu.zf { pc = 0x40a4f17; continue; }
  pc = 0x40a4f02; continue;
 }
 0x40a4f02 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x40a4f17; continue; }
  pc = 0x40a4f0d; continue;
 }
 0x40a4f0d => {
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x40a4f17; continue; }
  pc = 0x40a4f14; continue;
 }
 0x40a4f14 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x40a4f17; continue;
 }
 0x40a4f17 => {
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
pub fn f_040a4f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4f30); }
 let mut pc: u32 = 0x40a4f30;
 loop { match pc {
 0x40a4f30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4f40); }
 let mut pc: u32 = 0x40a4f40;
 loop { match pc {
 0x40a4f40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a4f7b; continue; }
  pc = 0x40a4f57; continue;
 }
 0x40a4f57 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a4f59; continue;
 }
 0x40a4f59 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40a4f6d; continue; }
  pc = 0x40a4f64; continue;
 }
 0x40a4f64 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a4f6a); f_040a6510(cpu); if cpu.returned_to != 0x40a4f6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a4f6d; continue;
 }
 0x40a4f6d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8a8u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a4f59; continue; }
  pc = 0x40a4f7b; continue;
 }
 0x40a4f7b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4f80); }
 let mut pc: u32 = 0x40a4f80;
 loop { match pc {
 0x40a4f80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a4f8b; continue; }
  pc = 0x40a4f88; continue;
 }
 0x40a4f88 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a4f8b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x868u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4fa0); }
 let mut pc: u32 = 0x40a4fa0;
 loop { match pc {
 0x40a4fa0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a4fbb; continue; }
  pc = 0x40a4fa8; continue;
 }
 0x40a4fa8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x868u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a4fbb; continue;
 }
 0x40a4fbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a4fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a4fc0); }
 let mut pc: u32 = 0x40a4fc0;
 loop { match pc {
 0x40a4fc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a4fdb; continue; }
  pc = 0x40a4fc9; continue;
 }
 0x40a4fc9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40a4fdb; continue;
 }
 0x40a4fdb => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5020); }
 let mut pc: u32 = 0x40a5020;
 loop { match pc {
 0x40a5020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a5066; continue; }
  pc = 0x40a502a; continue;
 }
 0x40a502a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40a5066; continue; }
  pc = 0x40a5032; continue;
 }
 0x40a5032 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x40a5066; continue; }
  pc = 0x40a5037; continue;
 }
 0x40a5037 => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a5056; continue; }
  pc = 0x40a504f; continue;
 }
 0x40a504f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x40a5056); cpu.call(t); if cpu.returned_to != 0x40a5056 { return; }
  pc = 0x40a5056; continue;
 }
 0x40a5056 => {
  { cpu.sub32(cpu.r32(0x864u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a5066; continue; }
  pc = 0x40a505f; continue;
 }
 0x40a505f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x40a5066); cpu.call(t); if cpu.returned_to != 0x40a5066 { return; }
  pc = 0x40a5066; continue;
 }
 0x40a5066 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5070); }
 let mut pc: u32 = 0x40a5070;
 loop { match pc {
 0x40a5070 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a50c4; continue; }
  pc = 0x40a507d; continue;
 }
 0x40a507d => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x8u32); }
  if !cpu.zf { pc = 0x40a50c4; continue; }
  pc = 0x40a5083; continue;
 }
 0x40a5083 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a50c4; continue; }
  pc = 0x40a5094; continue;
 }
 0x40a5094 => {
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
  cpu.push(0x40a50ba); f_0409cb30(cpu); if cpu.returned_to != 0x40a50ba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a50c4; continue;
 }
 0x40a50c4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a50f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a50f0); }
 let mut pc: u32 = 0x40a50f0;
 loop { match pc {
 0x40a50f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a516a; continue; }
  pc = 0x40a50fe; continue;
 }
 0x40a50fe => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40a516a; continue; }
  pc = 0x40a5104; continue;
 }
 0x40a5104 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a516a; continue; }
  pc = 0x40a5112; continue;
 }
 0x40a5112 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a516a; continue; }
  pc = 0x40a5118; continue;
 }
 0x40a5118 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40a516a; continue; }
  pc = 0x40a513a; continue;
 }
 0x40a513a => {
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
  cpu.push(0x40a5160); f_0409cb30(cpu); if cpu.returned_to != 0x40a5160 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a516a; continue;
 }
 0x40a516a => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5180); }
 let mut pc: u32 = 0x40a5180;
 loop { match pc {
 0x40a5180 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5196; continue; }
  pc = 0x40a5188; continue;
 }
 0x40a5188 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a5196; continue;
 }
 0x40a5196 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a51a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a51a0); }
 let mut pc: u32 = 0x40a51a0;
 loop { match pc {
 0x40a51a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a51af; continue; }
  pc = 0x40a51a8; continue;
 }
 0x40a51a8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a51af; continue;
 }
 0x40a51af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a51b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a51b0); }
 let mut pc: u32 = 0x40a51b0;
 loop { match pc {
 0x40a51b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a51be; continue; }
  pc = 0x40a51b8; continue;
 }
 0x40a51b8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a51be => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5210); }
 let mut pc: u32 = 0x40a5210;
 loop { match pc {
 0x40a5210 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a522d; continue; }
  pc = 0x40a521c; continue;
 }
 0x40a521c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x54u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a522d; continue; }
  pc = 0x40a5221; continue;
 }
 0x40a5221 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a522a); f_040a6510(cpu); if cpu.returned_to != 0x40a522a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a522d; continue;
 }
 0x40a522d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5230); }
 let mut pc: u32 = 0x40a5230;
 loop { match pc {
 0x40a5230 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5239; continue; }
  pc = 0x40a5238; continue;
 }
 0x40a5238 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5239 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5240); }
 let mut pc: u32 = 0x40a5240;
 loop { match pc {
 0x40a5240 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a525d; continue; }
  pc = 0x40a524c; continue;
 }
 0x40a524c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a525d; continue; }
  pc = 0x40a5251; continue;
 }
 0x40a5251 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a525a); f_040a6510(cpu); if cpu.returned_to != 0x40a525a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a525d; continue;
 }
 0x40a525d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5260); }
 let mut pc: u32 = 0x40a5260;
 loop { match pc {
 0x40a5260 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5269; continue; }
  pc = 0x40a5268; continue;
 }
 0x40a5268 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5269 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5270); }
 let mut pc: u32 = 0x40a5270;
 loop { match pc {
 0x40a5270 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a527f; continue; }
  pc = 0x40a5278; continue;
 }
 0x40a5278 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a527f; continue;
 }
 0x40a527f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5280); }
 let mut pc: u32 = 0x40a5280;
 loop { match pc {
 0x40a5280 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5289; continue; }
  pc = 0x40a5288; continue;
 }
 0x40a5288 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5289 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5290); }
 let mut pc: u32 = 0x40a5290;
 loop { match pc {
 0x40a5290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a52be; continue; }
  pc = 0x40a529d; continue;
 }
 0x40a529d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x44u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40a52ab; continue; }
  pc = 0x40a52a2; continue;
 }
 0x40a52a2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x48u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a52be; continue; }
  pc = 0x40a52ab; continue;
 }
 0x40a52ab => {
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a52bb); f_040a6510(cpu); if cpu.returned_to != 0x40a52bb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a52be; continue;
 }
 0x40a52be => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a52c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a52c0); }
 let mut pc: u32 = 0x40a52c0;
 loop { match pc {
 0x40a52c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a52e4; continue; }
  pc = 0x40a52c8; continue;
 }
 0x40a52c8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a52e4; continue;
 }
 0x40a52e4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a52f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a52f0); }
 let mut pc: u32 = 0x40a52f0;
 loop { match pc {
 0x40a52f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a52f9; continue; }
  pc = 0x40a52f8; continue;
 }
 0x40a52f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a52f9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5300); }
 let mut pc: u32 = 0x40a5300;
 loop { match pc {
 0x40a5300 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a53e4; continue; }
  pc = 0x40a530d; continue;
 }
 0x40a530d => {
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
  { let v: u32 = cpu.r32(0x42c5ef4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x2b11u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a53a3; continue; }
  pc = 0x40a539e; continue;
 }
 0x40a539e => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x40a53ac; continue; }
  pc = 0x40a53a3; continue;
 }
 0x40a53a3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a53b3; continue;
 }
 0x40a53ac => {
  { let v: u32 = 0x40u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a53b3; continue;
 }
 0x40a53b3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x85cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x860u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a53e1); f_040a6510(cpu); if cpu.returned_to != 0x40a53e1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a53e4; continue;
 }
 0x40a53e4 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a53f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a53f0); }
 let mut pc: u32 = 0x40a53f0;
 loop { match pc {
 0x40a53f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a53ff; continue; }
  pc = 0x40a53f8; continue;
 }
 0x40a53f8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a53ff; continue;
 }
 0x40a53ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5400); }
 let mut pc: u32 = 0x40a5400;
 loop { match pc {
 0x40a5400 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x40a540c); f_0409cfd0(cpu); if cpu.returned_to != 0x40a540c { return; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a542c; continue; }
  pc = 0x40a5415; continue;
 }
 0x40a5415 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x40a5418; continue;
 }
 0x40a5418 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x40a542c; continue; }
  pc = 0x40a541e; continue;
 }
 0x40a541e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a5418; continue; }
  pc = 0x40a542c; continue;
 }
 0x40a542c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x64u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x40a5452; continue; }
  pc = 0x40a5431; continue;
 }
 0x40a5431 => {
  { let v: u32 = 0x414b244u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a543d; continue;
 }
 0x40a543d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a543d; continue; }
  pc = 0x40a5447; continue;
 }
 0x40a5447 => {
  cpu.push(0x40a544c); f_0409cfc0(cpu); if cpu.returned_to != 0x40a544c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5452 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5463); f_0409b580(cpu); if cpu.returned_to != 0x40a5463 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x40a546b); f_0409cfc0(cpu); if cpu.returned_to != 0x40a546b { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5480); }
 let mut pc: u32 = 0x40a5480;
 loop { match pc {
 0x40a5480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a54a9; continue; }
  pc = 0x40a5491; continue;
 }
 0x40a5491 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x40a5494; continue;
 }
 0x40a5494 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x4u32); }
  if !cpu.zf { pc = 0x40a549b; continue; }
  pc = 0x40a549a; continue;
 }
 0x40a549a => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40a549b; continue;
 }
 0x40a549b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a5494; continue; }
  pc = 0x40a54a9; continue;
 }
 0x40a54a9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a54b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a54b0); }
 let mut pc: u32 = 0x40a54b0;
 loop { match pc {
 0x40a54b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a54bd); f_0409bbc0(cpu); if cpu.returned_to != 0x40a54bd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a54d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a54d0); }
 let mut pc: u32 = 0x40a54d0;
 loop { match pc {
 0x40a54d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x414b21cu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a54e7); f_0409c740(cpu); if cpu.returned_to != 0x40a54e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a550b; continue; }
  pc = 0x40a54ee; continue;
 }
 0x40a54ee => {
  { let v: u32 = 0x414b224u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a54fa; continue;
 }
 0x40a54fa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a54fa; continue; }
  pc = 0x40a5504; continue;
 }
 0x40a5504 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a5540; continue;
 }
 0x40a550b => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5520); f_0409b9b0(cpu); if cpu.returned_to != 0x40a5520 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a553e; continue; }
  pc = 0x40a5530; continue;
 }
 0x40a5530 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a553e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5540; continue;
 }
 0x40a5540 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5550); }
 let mut pc: u32 = 0x40a5550;
 loop { match pc {
 0x40a5550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5561); f_0409bc70(cpu); if cpu.returned_to != 0x40a5561 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5589; continue; }
  pc = 0x40a556a; continue;
 }
 0x40a556a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5575); f_040a5670(cpu); if cpu.returned_to != 0x40a5575 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5587; continue; }
  pc = 0x40a557e; continue;
 }
 0x40a557e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5584); f_0409bbc0(cpu); if cpu.returned_to != 0x40a5584 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a5587; continue;
 }
 0x40a5587 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a5589; continue;
 }
 0x40a5589 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5590); }
 let mut pc: u32 = 0x40a5590;
 loop { match pc {
 0x40a5590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a55a8); f_040a2e70(cpu); if cpu.returned_to != 0x40a55a8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5616; continue; }
  pc = 0x40a55b1; continue;
 }
 0x40a55b1 => {
  { cpu.sub8(cpu.r8(0x415c4e8u32), 0x0u8); }
  if !cpu.zf { pc = 0x40a55d7; continue; }
  pc = 0x40a55ba; continue;
 }
 0x40a55ba => {
  { let v: u32 = 0x414b204u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a55c6; continue;
 }
 0x40a55c6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a55c6; continue; }
  pc = 0x40a55d0; continue;
 }
 0x40a55d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5662; continue;
 }
 0x40a55d7 => {
  { let v: u32 = 0x415c4e8u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a55e0; continue;
 }
 0x40a55e0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a55e0; continue; }
  pc = 0x40a55ea; continue;
 }
 0x40a55ea => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a55fb; continue;
 }
 0x40a55fb => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a55fb; continue; }
  pc = 0x40a5601; continue;
 }
 0x40a5601 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a560d); f_040a2e70(cpu); if cpu.returned_to != 0x40a560d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x40a55ba; continue; }
  pc = 0x40a5616; continue;
 }
 0x40a5616 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5622); f_0409bc70(cpu); if cpu.returned_to != 0x40a5622 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40a562d); f_040a3230(cpu); if cpu.returned_to != 0x40a562d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40a5640; continue; }
  pc = 0x40a5634; continue;
 }
 0x40a5634 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5640 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a564e); f_040a5670(cpu); if cpu.returned_to != 0x40a564e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5660; continue; }
  pc = 0x40a5657; continue;
 }
 0x40a5657 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a565d); f_0409bbc0(cpu); if cpu.returned_to != 0x40a565d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a5660; continue;
 }
 0x40a5660 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a5662; continue;
 }
 0x40a5662 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a5670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5670); }
 let mut pc: u32 = 0x40a5670;
 loop { match pc {
 0x40a5670 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x40a567c); f_040a69a0(cpu); if cpu.returned_to != 0x40a567c { return; }
  { let v = 0x90u32; cpu.push(v); }
  cpu.push(0x40a5686); f_040a3260(cpu); if cpu.returned_to != 0x40a5686 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a56ac; continue; }
  pc = 0x40a568f; continue;
 }
 0x40a568f => {
  { let v: u32 = 0x414b10cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a569b; continue;
 }
 0x40a569b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a569b; continue; }
  pc = 0x40a56a5; continue;
 }
 0x40a56a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5ca2; continue;
 }
 0x40a56ac => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b134u32; cpu.esi = v; }
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
  if cpu.zf { pc = 0x40a5703; continue; }
  pc = 0x40a56d8; continue;
 }
 0x40a56d8 => {
  { let v: u32 = 0x414b13cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a56e6; continue;
 }
 0x40a56e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a56e6; continue; }
  pc = 0x40a56f0; continue;
 }
 0x40a56f0 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a56f6); f_040a3230(cpu); if cpu.returned_to != 0x40a56f6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5703 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5714); f_0409cb30(cpu); if cpu.returned_to != 0x40a5714 { return; }
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
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x40a57d3; continue; }
  pc = 0x40a579d; continue;
 }
 0x40a579d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a57b6); f_040a5d50(cpu); if cpu.returned_to != 0x40a57b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a57d3; continue; }
  pc = 0x40a57bd; continue;
 }
 0x40a57bd => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x40a57d3; continue;
 }
 0x40a57d3 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a581c; continue; }
  pc = 0x40a57da; continue;
 }
 0x40a57da => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a57e3); f_0409bd80(cpu); if cpu.returned_to != 0x40a57e3 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a581c; continue; }
  pc = 0x40a57ec; continue;
 }
 0x40a57ec => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x40a5801); f_040a5d50(cpu); if cpu.returned_to != 0x40a5801 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a581c; continue; }
  pc = 0x40a5808; continue;
 }
 0x40a5808 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x40a581c; continue;
 }
 0x40a581c => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40a58b7; continue; }
  pc = 0x40a5828; continue;
 }
 0x40a5828 => {
  { cpu.sub32(cpu.r32(0x42c5f24u32), 0x1u32); }
  if !cpu.zf { pc = 0x40a58b7; continue; }
  pc = 0x40a5835; continue;
 }
 0x40a5835 => {
  { let v: u32 = cpu.edi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a58a3; continue;
 }
 0x40a583f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40a586b; continue; }
  pc = 0x40a585d; continue;
 }
 0x40a585d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x40a586b; continue;
 }
 0x40a586b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5872); f_040a5d50(cpu); if cpu.returned_to != 0x40a5872 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a588f; continue; }
  pc = 0x40a5879; continue;
 }
 0x40a5879 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x40a58b7; continue;
 }
 0x40a588f => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x18u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a58a3; continue;
 }
 0x40a58a3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a583f; continue; }
  pc = 0x40a58b7; continue;
 }
 0x40a58b7 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a58e7; continue; }
  pc = 0x40a58be; continue;
 }
 0x40a58be => {
  { let v: u32 = 0x414b154u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a58ca; continue;
 }
 0x40a58ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a58ca; continue; }
  pc = 0x40a58d4; continue;
 }
 0x40a58d4 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a58da); f_040a3230(cpu); if cpu.returned_to != 0x40a58da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a58e7 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { cpu.sub32(cpu.r32(0x42c5f28u32), 0x0u32); }
  if cpu.zf { pc = 0x40a5905; continue; }
  pc = 0x40a58fe; continue;
 }
 0x40a58fe => {
  { let v: u32 = 0x2000u32; cpu.ebx = v; }
  pc = 0x40a590b; continue;
 }
 0x40a5905 => {
  { let v: u32 = cpu.r32(0x42c5ee8u32); cpu.ebx = v; }
  pc = 0x40a590b; continue;
 }
 0x40a590b => {
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40a5924; continue;
 }
 0x40a5924 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a5939); f_040a31a0(cpu); if cpu.returned_to != 0x40a5939 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5969; continue; }
  pc = 0x40a5940; continue;
 }
 0x40a5940 => {
  { let v: u32 = 0x414b178u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a594c; continue;
 }
 0x40a594c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a594c; continue; }
  pc = 0x40a5956; continue;
 }
 0x40a5956 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a595c); f_040a3230(cpu); if cpu.returned_to != 0x40a595c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5969 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5981; continue; }
  pc = 0x40a5970; continue;
 }
 0x40a5970 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a597e); f_040a3160(cpu); if cpu.returned_to != 0x40a597e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x40a5981; continue;
 }
 0x40a5981 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x40a5924; continue; }
  pc = 0x40a59b5; continue;
 }
 0x40a59b5 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a59bb); f_040a5e80(cpu); if cpu.returned_to != 0x40a59bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a59e9; continue; }
  pc = 0x40a59c2; continue;
 }
 0x40a59c2 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a59d3); f_040a3160(cpu); if cpu.returned_to != 0x40a59d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a59dc); f_040a3230(cpu); if cpu.returned_to != 0x40a59dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a59e9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a59fb); f_0409cb30(cpu); if cpu.returned_to != 0x40a59fb { return; }
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
  cpu.push(0x40a5a38); f_040a3260(cpu); if cpu.returned_to != 0x40a5a38 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5ab3; continue; }
  pc = 0x40a5a42; continue;
 }
 0x40a5a42 => {
  { let v: u32 = 0x414b198u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a5a4e; continue;
 }
 0x40a5a4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a5a4e; continue; }
  pc = 0x40a5a58; continue;
 }
 0x40a5a58 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a5a6d; continue; }
  pc = 0x40a5a64; continue;
 }
 0x40a5a64 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5a6a); f_0409cc80(cpu); if cpu.returned_to != 0x40a5a6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a5a6d; continue;
 }
 0x40a5a6d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5a7f); f_0409cb30(cpu); if cpu.returned_to != 0x40a5a7f { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5a9d); f_040a3160(cpu); if cpu.returned_to != 0x40a5a9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5aa6); f_040a3230(cpu); if cpu.returned_to != 0x40a5aa6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5ab3 => {
  { let v: u32 = cpu.r32(0x42c5ef0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5ac7); f_040a3260(cpu); if cpu.returned_to != 0x40a5ac7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a5b52; continue; }
  pc = 0x40a5ad5; continue;
 }
 0x40a5ad5 => {
  { let v: u32 = 0x414b1bcu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a5ae1; continue;
 }
 0x40a5ae1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a5ae1; continue; }
  pc = 0x40a5aeb; continue;
 }
 0x40a5aeb => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a5b00; continue; }
  pc = 0x40a5af7; continue;
 }
 0x40a5af7 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5afd); f_0409cc80(cpu); if cpu.returned_to != 0x40a5afd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a5b00; continue;
 }
 0x40a5b00 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a5b12); f_0409cb30(cpu); if cpu.returned_to != 0x40a5b12 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5b30); f_040a3160(cpu); if cpu.returned_to != 0x40a5b30 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5b3c); f_040a3230(cpu); if cpu.returned_to != 0x40a5b3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5b45); f_040a3230(cpu); if cpu.returned_to != 0x40a5b45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5b52 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a5ba7; continue; }
  pc = 0x40a5b5f; continue;
 }
 0x40a5b5f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x414b1e4u32; cpu.esi = v; }
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a5b5f; continue; }
  pc = 0x40a5ba7; continue;
 }
 0x40a5ba7 => {
  { let v = 0x40a6300u32; cpu.push(v); }
  cpu.push(0x40a5bb1); f_0409c640(cpu); if cpu.returned_to != 0x40a5bb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a5c49; continue; }
  pc = 0x40a5bc0; continue;
 }
 0x40a5bc0 => {
  { let v: u32 = 0x414b1ecu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a5bcc; continue;
 }
 0x40a5bcc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a5bcc; continue; }
  pc = 0x40a5bd6; continue;
 }
 0x40a5bd6 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a5beb; continue; }
  pc = 0x40a5be2; continue;
 }
 0x40a5be2 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5be8); f_0409cc80(cpu); if cpu.returned_to != 0x40a5be8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a5beb; continue;
 }
 0x40a5beb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a5bfd); f_0409cb30(cpu); if cpu.returned_to != 0x40a5bfd { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5c1b); f_040a3160(cpu); if cpu.returned_to != 0x40a5c1b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5c27); f_040a3230(cpu); if cpu.returned_to != 0x40a5c27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5c33); f_040a3230(cpu); if cpu.returned_to != 0x40a5c33 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5c3c); f_040a3230(cpu); if cpu.returned_to != 0x40a5c3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a5c49 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a5c50); f_0409c530(cpu); if cpu.returned_to != 0x40a5c50 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5ee0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a5c62); f_0409c3b0(cpu); if cpu.returned_to != 0x40a5c62 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5c6e); f_0409c160(cpu); if cpu.returned_to != 0x40a5c6e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x40a5cb0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a5c87); f_040b3ffa(cpu); if cpu.returned_to != 0x40a5c87 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5c92); f_040b3f67(cpu); if cpu.returned_to != 0x40a5c92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5c9d); f_040b3f67(cpu); if cpu.returned_to != 0x40a5c9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x40a5ca2; continue;
 }
 0x40a5ca2 => {
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
pub fn f_040a5cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5cb0); }
 let mut pc: u32 = 0x40a5cb0;
 loop { match pc {
 0x40a5cb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a5d06; continue; }
  pc = 0x40a5cbf; continue;
 }
 0x40a5cbf => {
  if cpu.zf { pc = 0x40a5cdd; continue; }
  pc = 0x40a5cc1; continue;
 }
 0x40a5cc1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5cd3); f_0409cb30(cpu); if cpu.returned_to != 0x40a5cd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a5cdd; continue;
 }
 0x40a5cdd => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5ce3); f_040b3ffa(cpu); if cpu.returned_to != 0x40a5ce3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5cee); f_040b3f67(cpu); if cpu.returned_to != 0x40a5cee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5cf9); f_040b3f67(cpu); if cpu.returned_to != 0x40a5cf9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x40a5d03); f_0409ca80(cpu); if cpu.returned_to != 0x40a5d03 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a5d06; continue;
 }
 0x40a5d06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a5d0f); f_0409bf00(cpu); if cpu.returned_to != 0x40a5d0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a5d23); f_040a3160(cpu); if cpu.returned_to != 0x40a5d23 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a5d2f); f_040a3230(cpu); if cpu.returned_to != 0x40a5d2f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5d3b); f_040a3230(cpu); if cpu.returned_to != 0x40a5d3b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a5d44); f_040a3230(cpu); if cpu.returned_to != 0x40a5d44 { return; }
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
pub fn f_040a5d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a5d50); }
 let mut pc: u32 = 0x40a5d50;
 loop { match pc {
 0x40a5d50 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x40a5df2; continue; }
  pc = 0x40a5d79; continue;
 }
 0x40a5d79 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a5d9e; continue; }
  pc = 0x40a5d97; continue;
 }
 0x40a5d97 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5d9e; continue;
 }
 0x40a5d9e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a5db1; continue; }
  pc = 0x40a5da8; continue;
 }
 0x40a5da8 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5db1; continue;
 }
 0x40a5db1 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a5dc5; continue; }
  pc = 0x40a5dbc; continue;
 }
 0x40a5dbc => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5dc5; continue;
 }
 0x40a5dc5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a5dd9; continue; }
  pc = 0x40a5dd0; continue;
 }
 0x40a5dd0 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5dd9; continue;
 }
 0x40a5dd9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5ddb; continue;
 }
 0x40a5ddb => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a5dea; continue; }
  pc = 0x40a5de2; continue;
 }
 0x40a5de2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40a5dea; continue;
 }
 0x40a5dea => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a5ddb; continue; }
  pc = 0x40a5df2; continue;
 }
 0x40a5df2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42c5f04u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40a5e29; continue; }
  pc = 0x40a5e0e; continue;
 }
 0x40a5e0e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x40a5e43; continue;
 }
 0x40a5e29 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x40a5e43; continue;
 }
 0x40a5e43 => {
  cpu.push(0x40a5e48); f_0409cb30(cpu); if cpu.returned_to != 0x40a5e48 { return; }
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
pub fn f_040a5e60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a5e60); } r_040a5e60(cpu, 0x40a5e60); }
pub fn f_040a5e70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a5e70); } r_040a5e60(cpu, 0x40a5e70); }
pub fn f_040a5e80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a5e80); } r_040a5e60(cpu, 0x40a5e80); }
fn r_040a5e60(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a5e60 => {
  { let v: u32 = cpu.ebx ^ cpu.r32(0xau32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x55u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x1d040a60u32.wrapping_add(cpu.edi.wrapping_mul(8))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x26040a61u32.wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5f13; continue; }
  pc = 0x40a5e9a; continue;
 }
 0x40a5e70 => {
  { cpu.df = true; }
  { cpu.pushad(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0xd040a61u32.wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.popad(); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5f13; continue; }
  pc = 0x40a5e9a; continue;
 }
 0x40a5e80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a5f13; continue; }
  pc = 0x40a5e9a; continue;
 }
 0x40a5e9a => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a5ed2; continue; }
  pc = 0x40a5ea3; continue;
 }
 0x40a5ea3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5ea5; continue;
 }
 0x40a5ea5 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8a4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x4u32); }
  if !cpu.zf { pc = 0x40a5ec5; continue; }
  pc = 0x40a5ebe; continue;
 }
 0x40a5ebe => {
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40a5ec5; continue;
 }
 0x40a5ec5 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a5ea5; continue; }
  pc = 0x40a5ed2; continue;
 }
 0x40a5ed2 => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a5ef4; continue; }
  pc = 0x40a5ed8; continue;
 }
 0x40a5ed8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a5eea); f_0409cb30(cpu); if cpu.returned_to != 0x40a5eea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a5ef4; continue;
 }
 0x40a5ef4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5efa); f_040b3ffa(cpu); if cpu.returned_to != 0x40a5efa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5f05); f_040b3f67(cpu); if cpu.returned_to != 0x40a5f05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a5f10); f_040b3f67(cpu); if cpu.returned_to != 0x40a5f10 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a5f13; continue;
 }
 0x40a5f13 => {
  { let v: u32 = cpu.r32(0x42c5efcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f00u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a5fb9; continue; }
  pc = 0x40a5f2b; continue;
 }
 0x40a5f2b => {
  cpu.poll(); pc = cpu.r32(0x40a5e60u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x40a5f33 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5fb9; continue;
 }
 0x40a5f55 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5fb9; continue;
 }
 0x40a5f77 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5fb9; continue;
 }
 0x40a5f99 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a5fb9; continue;
 }
 0x40a5fb9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a5fbe; continue;
 }
 0x40a5fbe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x40a5fcc; continue; }
  pc = 0x40a5fc7; continue;
 }
 0x40a5fc7 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x40a5fd4; continue;
 }
 0x40a5fcc => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a5fbe; continue; }
  pc = 0x40a5fd4; continue;
 }
 0x40a5fd4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x42c5ee4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40a5ffd; continue; }
  pc = 0x40a5ff4; continue;
 }
 0x40a5ff4 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a601f; continue; }
  pc = 0x40a5ff6; continue;
 }
 0x40a5ff6 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40a603d; continue; }
  pc = 0x40a5ffb; continue;
 }
 0x40a5ffb => {
  pc = 0x40a605b; continue;
 }
 0x40a5ffd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a605b; continue; }
  pc = 0x40a6001; continue;
 }
 0x40a6001 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x40a60ea; continue;
 }
 0x40a601f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x40a60ea; continue;
 }
 0x40a603d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x40a60ea; continue;
 }
 0x40a605b => {
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
  { let v: u32 = cpu.r32(0x42c5ee4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a60c0; continue;
 }
 0x40a60c0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a60cb; continue; }
  pc = 0x40a60c9; continue;
 }
 0x40a60c9 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a60cb; continue;
 }
 0x40a60cb => {
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a60de; continue; }
  pc = 0x40a60cf; continue;
 }
 0x40a60cf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a60dc; continue; }
  pc = 0x40a60da; continue;
 }
 0x40a60da => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a60dc; continue;
 }
 0x40a60dc => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x40a60de; continue;
 }
 0x40a60de => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a60c0; continue; }
  pc = 0x40a60e7; continue;
 }
 0x40a60e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x40a60ea; continue;
 }
 0x40a60ea => {
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6134; continue; }
  pc = 0x40a60f5; continue;
 }
 0x40a60f5 => {
  cpu.poll(); pc = cpu.r32(0x40a5e70u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x40a60fd => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6134; continue;
 }
 0x40a610d => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6134; continue;
 }
 0x40a611d => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a612d; continue;
 }
 0x40a6126 => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a612d; continue;
 }
 0x40a612d => {
  { let v: u32 = 0x2u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6134; continue;
 }
 0x40a6134 => {
  { let v: u32 = cpu.r32(0x42c5eecu32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x42c5f28u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a61b9; continue; }
  pc = 0x40a6185; continue;
 }
 0x40a6185 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a6192; continue; }
  pc = 0x40a618d; continue;
 }
 0x40a618d => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x40a6192; continue;
 }
 0x40a6192 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a61b9; continue; }
  pc = 0x40a619c; continue;
 }
 0x40a619c => {
  { let v: u32 = 0x414b0e0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a61a8; continue;
 }
 0x40a61a8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a61a8; continue; }
  pc = 0x40a61b2; continue;
 }
 0x40a61b2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a62f7; continue;
 }
 0x40a61b9 => {
  { let v: u32 = 0x7fffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x40a61e5; continue;
 }
 0x40a61c5 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a61d0; continue; }
  pc = 0x40a61ce; continue;
 }
 0x40a61ce => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a61d0; continue;
 }
 0x40a61d0 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a61e3; continue; }
  pc = 0x40a61d4; continue;
 }
 0x40a61d4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a61df; continue; }
  pc = 0x40a61dd; continue;
 }
 0x40a61dd => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a61df; continue;
 }
 0x40a61df => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x40a61e3; continue;
 }
 0x40a61e3 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  pc = 0x40a61e5; continue;
 }
 0x40a61e5 => {
  { let v: u32 = cpu.r32(0x42c5ee8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a61c5; continue; }
  pc = 0x40a61f7; continue;
 }
 0x40a61f7 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40a6205; continue; }
  pc = 0x40a6202; continue;
 }
 0x40a6202 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6205; continue;
 }
 0x40a6205 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x14u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { pc = 0x40a620d; continue; }
  pc = 0x40a620a; continue;
 }
 0x40a620a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a620d; continue;
 }
 0x40a620d => {
  { let v: u32 = cpu.r32(0x42c5f28u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a622a; continue; }
  pc = 0x40a621a; continue;
 }
 0x40a621a => {
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xff000u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  pc = 0x40a622a; continue;
 }
 0x40a622a => {
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
  if cpu.zf { pc = 0x40a62f2; continue; }
  pc = 0x40a6276; continue;
 }
 0x40a6276 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a629c; continue; }
  pc = 0x40a627f; continue;
 }
 0x40a627f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a6281; continue;
 }
 0x40a6281 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8a4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a6281; continue; }
  pc = 0x40a629c; continue;
 }
 0x40a629c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a62a2); f_040b3ffa(cpu); if cpu.returned_to != 0x40a62a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a62ad); f_040b3f67(cpu); if cpu.returned_to != 0x40a62ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a62b8); f_040b3f67(cpu); if cpu.returned_to != 0x40a62b8 { return; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40a62f2; continue; }
  pc = 0x40a62c2; continue;
 }
 0x40a62c2 => {
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
  cpu.push(0x40a62e8); f_0409cb30(cpu); if cpu.returned_to != 0x40a62e8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a62f2; continue;
 }
 0x40a62f2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a62f7; continue;
 }
 0x40a62f7 => {
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
pub fn f_040a6300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6300); }
 let mut pc: u32 = 0x40a6300;
 loop { match pc {
 0x40a6300 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x415c670u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a64fd; continue; }
  pc = 0x40a6316; continue;
 }
 0x40a6316 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f08u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c670u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f78u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a633e; continue; }
  pc = 0x40a6334; continue;
 }
 0x40a6334 => {
  cpu.push(0x40a6339); f_0409cc70(cpu); if cpu.returned_to != 0x40a6339 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f80u32, v); }
  pc = 0x40a633e; continue;
 }
 0x40a633e => {
  { let v: u32 = cpu.r32(0x42c5f78u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40a637b; continue; }
  pc = 0x40a6350; continue;
 }
 0x40a6350 => {
  { cpu.sub32(cpu.r32(0x42c5f08u32), 0x0u32); }
  if cpu.zf { pc = 0x40a6368; continue; }
  pc = 0x40a6359; continue;
 }
 0x40a6359 => {
  { let v: u32 = cpu.r32(0x42c5f80u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a6365); f_0409cc60(cpu); if cpu.returned_to != 0x40a6365 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a6368; continue;
 }
 0x40a6368 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c670u32, v); }
  pc = 0x40a64fd; continue;
 }
 0x40a637b => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5f7cu32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a6391; continue; }
  pc = 0x40a638c; continue;
 }
 0x40a638c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40a63ca; continue; }
  pc = 0x40a6391; continue;
 }
 0x40a6391 => {
  { let v: u32 = cpu.r32(0x42c5f08u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5f74u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a63b0; continue; }
  pc = 0x40a63a1; continue;
 }
 0x40a63a1 => {
  { let v: u32 = cpu.r32(0x42c5f80u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a63ad); f_0409cc60(cpu); if cpu.returned_to != 0x40a63ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a63b0; continue;
 }
 0x40a63b0 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x415c670u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5f74u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x40a63ca => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a63d3); f_040b3ffa(cpu); if cpu.returned_to != 0x40a63d3 { return; }
  { let v: u32 = cpu.r32(0x42c5f78u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5f84u32, v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5f74u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f70u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a6431; continue; }
  pc = 0x40a63f8; continue;
 }
 0x40a63f8 => {
  { let v: u32 = cpu.r32(0x42c5f70u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x40a640e; continue; }
  pc = 0x40a6403; continue;
 }
 0x40a6403 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42c5f84u32)); cpu.w32(0x42c5f84u32, v); }
  cpu.push(0x40a640e); f_040a6800(cpu); if cpu.returned_to != 0x40a640e { return; }
  pc = 0x40a640e; continue;
 }
 0x40a640e => {
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42c5f70u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5f74u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x42c5f70u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40a63f8; continue; }
  pc = 0x40a6431; continue;
 }
 0x40a6431 => {
  { let v: u32 = cpu.r32(0x42c5f78u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f84u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c5f7cu32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a644f); f_040b3f67(cpu); if cpu.returned_to != 0x40a644f { return; }
  { let v: u32 = cpu.r32(0x42c5f78u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x40a6491; continue; }
  pc = 0x40a645f; continue;
 }
 0x40a645f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x42c5f64u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x42c5f7cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = 0x405u32; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x42c5f68u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x42c5f6au32, v); }
  cpu.push(0x40a648e); f_0409cb30(cpu); if cpu.returned_to != 0x40a648e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a6491; continue;
 }
 0x40a6491 => {
  { let v: u32 = cpu.r32(0x42c5f78u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a64ad; continue; }
  pc = 0x40a64a4; continue;
 }
 0x40a64a4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a64d7; continue;
 }
 0x40a64ad => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x40a64d7; continue; }
  pc = 0x40a64bb; continue;
 }
 0x40a64bb => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a64d7; continue; }
  pc = 0x40a64c1; continue;
 }
 0x40a64c1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a64d1); f_0409cb30(cpu); if cpu.returned_to != 0x40a64d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a64d7; continue;
 }
 0x40a64d7 => {
  { cpu.sub32(cpu.r32(0x42c5f08u32), 0x0u32); }
  if cpu.zf { pc = 0x40a64ef; continue; }
  pc = 0x40a64e0; continue;
 }
 0x40a64e0 => {
  { let v: u32 = cpu.r32(0x42c5f80u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a64ec); f_0409cc60(cpu); if cpu.returned_to != 0x40a64ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a64ef; continue;
 }
 0x40a64ef => {
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42c5f74u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x415c670u32, v); }
  pc = 0x40a64fd; continue;
 }
 0x40a64fd => {
  { let v: u32 = cpu.esi; cpu.w32(0x42c5f74u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6510); }
 let mut pc: u32 = 0x40a6510;
 loop { match pc {
 0x40a6510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a652c; continue; }
  pc = 0x40a6523; continue;
 }
 0x40a6523 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a6537; continue;
 }
 0x40a652c => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40a6537; continue; }
  pc = 0x40a6530; continue;
 }
 0x40a6530 => {
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a6537; continue;
 }
 0x40a6537 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a6548; continue; }
  pc = 0x40a653f; continue;
 }
 0x40a653f => {
  { let v: u32 = 0x7fu32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a6553; continue;
 }
 0x40a6548 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40a6553; continue; }
  pc = 0x40a654c; continue;
 }
 0x40a654c => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a6553; continue;
 }
 0x40a6553 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a6582; continue; }
  pc = 0x40a657c; continue;
 }
 0x40a657c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a6582; continue;
 }
 0x40a6582 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a6591; continue; }
  pc = 0x40a6589; continue;
 }
 0x40a6589 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a6591; continue;
 }
 0x40a6591 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x40a659e; continue; }
  pc = 0x40a6599; continue;
 }
 0x40a6599 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x40a65e7; continue; }
  pc = 0x40a659e; continue;
 }
 0x40a659e => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x415c5f0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x415c5f0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x458u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a67f7; continue;
 }
 0x40a65e7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a65f6; continue; }
  pc = 0x40a65ef; continue;
 }
 0x40a65ef => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a65f6; continue;
 }
 0x40a65f6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x40a6624; continue; }
  pc = 0x40a6601; continue;
 }
 0x40a6601 => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x40a6624; continue; }
  pc = 0x40a6606; continue;
 }
 0x40a6606 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a6613; continue; }
  pc = 0x40a660a; continue;
 }
 0x40a660a => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x40a675c; continue; }
  pc = 0x40a6613; continue;
 }
 0x40a6613 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x40a6624; continue; }
  pc = 0x40a661b; continue;
 }
 0x40a661b => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x40a675c; continue; }
  pc = 0x40a6624; continue;
 }
 0x40a6624 => {
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x458u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x415c5f0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x415c5f0u32.wrapping_add(cpu.edi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x48u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x40a66ff; continue; }
  pc = 0x40a665c; continue;
 }
 0x40a665c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0x0u32); cpu.ecx = v; }
  pc = 0x40a666d; continue;
 }
 0x40a666d => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a666d; continue; }
  pc = 0x40a66a3; continue;
 }
 0x40a66a3 => {
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
  pc = 0x40a66c7; continue;
 }
 0x40a66c7 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a67f7; continue; }
  pc = 0x40a66fd; continue;
 }
 0x40a66fd => {
  cpu.poll(); pc = 0x40a66c7; continue;
 }
 0x40a66ff => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  pc = 0x40a6720; continue;
 }
 0x40a6720 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a67f7; continue; }
  pc = 0x40a675a; continue;
 }
 0x40a675a => {
  cpu.poll(); pc = 0x40a6720; continue;
 }
 0x40a675c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a67c7; continue; }
  pc = 0x40a676e; continue;
 }
 0x40a676e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x0u32); cpu.edx = v; }
  pc = 0x40a6779; continue;
 }
 0x40a6779 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a6779; continue; }
  pc = 0x40a6794; continue;
 }
 0x40a6794 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x40a67ae; continue;
 }
 0x40a67ae => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x100u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40a67f7; continue; }
  pc = 0x40a67c5; continue;
 }
 0x40a67c5 => {
  cpu.poll(); pc = 0x40a67ae; continue;
 }
 0x40a67c7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x40a67dc; continue;
 }
 0x40a67dc => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a67dc; continue; }
  pc = 0x40a67f7; continue;
 }
 0x40a67f7 => {
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
pub fn f_040a6800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6800); }
 let mut pc: u32 = 0x40a6800;
 loop { match pc {
 0x40a6800 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x40a6916; continue; }
  pc = 0x40a6812; continue;
 }
 0x40a6812 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x40a6814; continue;
 }
 0x40a6814 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x40a6916; continue; }
  pc = 0x40a6820; continue;
 }
 0x40a6820 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40a684e; continue; }
  pc = 0x40a6838; continue;
 }
 0x40a6838 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a684e; continue; }
  pc = 0x40a6840; continue;
 }
 0x40a6840 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.eax))); }
  if !cpu.cf { pc = 0x40a684e; continue; }
  pc = 0x40a6845; continue;
 }
 0x40a6845 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a684e; continue;
 }
 0x40a684e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a68f4; continue; }
  pc = 0x40a6856; continue;
 }
 0x40a6856 => {
  { cpu.sub32(cpu.r32(0x860u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a6866; continue; }
  pc = 0x40a685f; continue;
 }
 0x40a685f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x40a6866); cpu.call(t); if cpu.returned_to != 0x40a6866 { return; }
  pc = 0x40a6866; continue;
 }
 0x40a6866 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x40a688a; continue; }
  pc = 0x40a686c; continue;
 }
 0x40a686c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x40a6814; continue; }
  pc = 0x40a6882; continue;
 }
 0x40a6882 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x40a6814; continue;
 }
 0x40a688a => {
  { cpu.sub32(cpu.r32(0x858u32.wrapping_add(cpu.esi)), 0x2u32); }
  if !cpu.zf { pc = 0x40a68ce; continue; }
  pc = 0x40a6893; continue;
 }
 0x40a6893 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40a68ce; continue; }
  pc = 0x40a68a8; continue;
 }
 0x40a68a8 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40a68b7; continue; }
  pc = 0x40a68ae; continue;
 }
 0x40a68ae => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6916; continue;
 }
 0x40a68b7 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40a68c6; continue; }
  pc = 0x40a68bd; continue;
 }
 0x40a68bd => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a6916; continue;
 }
 0x40a68c6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x40a6814; continue;
 }
 0x40a68ce => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a6916; continue; }
  pc = 0x40a68e6; continue;
 }
 0x40a68e6 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x40a68ed); cpu.call(t); if cpu.returned_to != 0x40a68ed { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a68f4 => {
  { cpu.sub32(cpu.r32(0x85cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a6904; continue; }
  pc = 0x40a68fd; continue;
 }
 0x40a68fd => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x85cu32.wrapping_add(cpu.esi)); cpu.push(0x40a6904); cpu.call(t); if cpu.returned_to != 0x40a6904 { return; }
  pc = 0x40a6904; continue;
 }
 0x40a6904 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a690c); f_040b3e04(cpu); if cpu.returned_to != 0x40a690c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.poll(); pc = 0x40a6814; continue;
 }
 0x40a6916 => {
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
pub fn f_040a6920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6920); }
 let mut pc: u32 = 0x40a6920;
 loop { match pc {
 0x40a6920 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a6947; continue; }
  pc = 0x40a692b; continue;
 }
 0x40a692b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a693d); f_0409cb30(cpu); if cpu.returned_to != 0x40a693d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a6947; continue;
 }
 0x40a6947 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a69a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a69a0); }
 let mut pc: u32 = 0x40a69a0;
 loop { match pc {
 0x40a69a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c674u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a6a5c; continue; }
  pc = 0x40a69b0; continue;
 }
 0x40a69b0 => {
  { let v = 0x40a4c20u32; cpu.push(v); }
  { let v = 0x40a69a0u32; cpu.push(v); }
  cpu.push(0x40a69bf); f_040a30c0(cpu); if cpu.returned_to != 0x40a69bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x415c5f0u32; cpu.push(v); }
  cpu.push(0x40a69d1); f_040a3000(cpu); if cpu.returned_to != 0x40a69d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c670u32; cpu.push(v); }
  cpu.push(0x40a69e0); f_040a3000(cpu); if cpu.returned_to != 0x40a69e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f78u32; cpu.push(v); }
  cpu.push(0x40a69ef); f_040a3000(cpu); if cpu.returned_to != 0x40a69ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x42c5f64u32; cpu.push(v); }
  cpu.push(0x40a69fe); f_040a3000(cpu); if cpu.returned_to != 0x40a69fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f80u32; cpu.push(v); }
  cpu.push(0x40a6a0d); f_040a3000(cpu); if cpu.returned_to != 0x40a6a0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f84u32; cpu.push(v); }
  cpu.push(0x40a6a1c); f_040a3000(cpu); if cpu.returned_to != 0x40a6a1c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f74u32; cpu.push(v); }
  cpu.push(0x40a6a2b); f_040a3000(cpu); if cpu.returned_to != 0x40a6a2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f7cu32; cpu.push(v); }
  cpu.push(0x40a6a3a); f_040a3000(cpu); if cpu.returned_to != 0x40a6a3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f70u32; cpu.push(v); }
  cpu.push(0x40a6a49); f_040a3000(cpu); if cpu.returned_to != 0x40a6a49 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40a6a56); f_040b4026(cpu); if cpu.returned_to != 0x40a6a56 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c674u32, v); }
  pc = 0x40a6a5c; continue;
 }
 0x40a6a5c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6a60); }
 let mut pc: u32 = 0x40a6a60;
 loop { match pc {
 0x40a6a60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c678u32), 0x0u32); }
  if cpu.zf { pc = 0x40a6a86; continue; }
  pc = 0x40a6a6c; continue;
 }
 0x40a6a6c => {
  { let v = 0x40a6a60u32; cpu.push(v); }
  { let v = 0x40a7240u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40a6a7d); f_040a3020(cpu); if cpu.returned_to != 0x40a6a7d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c678u32, v); }
  pc = 0x40a6a86; continue;
 }
 0x40a6a86 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6a90); }
 let mut pc: u32 = 0x40a6a90;
 loop { match pc {
 0x40a6a90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a6aab; continue; }
  pc = 0x40a6a99; continue;
 }
 0x40a6a99 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x888u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x888u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40a6aab; continue;
 }
 0x40a6aab => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6ab0); }
 let mut pc: u32 = 0x40a6ab0;
 loop { match pc {
 0x40a6ab0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a6ac5; continue; }
  pc = 0x40a6abf; continue;
 }
 0x40a6abf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6ac5 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x414b26cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a6ad1; continue;
 }
 0x40a6ad1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6adf; continue; }
  pc = 0x40a6ad9; continue;
 }
 0x40a6ad9 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6adf; continue; }
  pc = 0x40a6add; continue;
 }
 0x40a6add => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6adf; continue;
 }
 0x40a6adf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6aec; continue; }
  pc = 0x40a6ae4; continue;
 }
 0x40a6ae4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6aec; continue; }
  pc = 0x40a6ae9; continue;
 }
 0x40a6ae9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6aec; continue;
 }
 0x40a6aec => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6afb; continue; }
  pc = 0x40a6af0; continue;
 }
 0x40a6af0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6afb; continue; }
  pc = 0x40a6af4; continue;
 }
 0x40a6af4 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6afb; continue; }
  pc = 0x40a6af7; continue;
 }
 0x40a6af7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6ad1; continue;
 }
 0x40a6afb => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6b02; continue; }
  pc = 0x40a6b00; continue;
 }
 0x40a6b00 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6b02; continue;
 }
 0x40a6b02 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a6b0a; continue; }
  pc = 0x40a6b06; continue;
 }
 0x40a6b06 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a6b6d; continue;
 }
 0x40a6b0a => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b278u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x40a6b17; continue;
 }
 0x40a6b17 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6b25; continue; }
  pc = 0x40a6b1f; continue;
 }
 0x40a6b1f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6b25; continue; }
  pc = 0x40a6b23; continue;
 }
 0x40a6b23 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6b25; continue;
 }
 0x40a6b25 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6b32; continue; }
  pc = 0x40a6b2a; continue;
 }
 0x40a6b2a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6b32; continue; }
  pc = 0x40a6b2f; continue;
 }
 0x40a6b2f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6b32; continue;
 }
 0x40a6b32 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6b41; continue; }
  pc = 0x40a6b36; continue;
 }
 0x40a6b36 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6b41; continue; }
  pc = 0x40a6b3a; continue;
 }
 0x40a6b3a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6b41; continue; }
  pc = 0x40a6b3d; continue;
 }
 0x40a6b3d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6b17; continue;
 }
 0x40a6b41 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6b48; continue; }
  pc = 0x40a6b46; continue;
 }
 0x40a6b46 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6b48; continue;
 }
 0x40a6b48 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40a6b68; continue; }
  pc = 0x40a6b4c; continue;
 }
 0x40a6b4c => {
  { let v: u32 = 0x414b2c8u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a6b58; continue;
 }
 0x40a6b58 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a6b58; continue; }
  pc = 0x40a6b62; continue;
 }
 0x40a6b62 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6b68 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a6b6d; continue;
 }
 0x40a6b6d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x40a6b78; continue; }
  pc = 0x40a6b71; continue;
 }
 0x40a6b71 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40a6bb5; continue; }
  pc = 0x40a6b76; continue;
 }
 0x40a6b76 => {
  pc = 0x40a6bc9; continue;
 }
 0x40a6b78 => {
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
  cpu.push(0x40a6bb3); f_040a6f78(cpu); if cpu.returned_to != 0x40a6bb3 { return; }
  pc = 0x40a6bc6; continue;
 }
 0x40a6bb5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a6bc6); f_040a6df0(cpu); if cpu.returned_to != 0x40a6bc6 { return; }
  pc = 0x40a6bc6; continue;
 }
 0x40a6bc6 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a6bc9; continue;
 }
 0x40a6bc9 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a6bee; continue; }
  pc = 0x40a6bd2; continue;
 }
 0x40a6bd2 => {
  { let v: u32 = 0x414b2f0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a6bde; continue;
 }
 0x40a6bde => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a6bde; continue; }
  pc = 0x40a6be8; continue;
 }
 0x40a6be8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6bee => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6c00); }
 let mut pc: u32 = 0x40a6c00;
 loop { match pc {
 0x40a6c00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x415c678u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a6c2e; continue; }
  pc = 0x40a6c11; continue;
 }
 0x40a6c11 => {
  { let v = 0x40a6a60u32; cpu.push(v); }
  { let v = 0x40a7240u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40a6c25); f_040a30c0(cpu); if cpu.returned_to != 0x40a6c25 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c678u32, v); }
  pc = 0x40a6c2e; continue;
 }
 0x40a6c2e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40a6c39; continue; }
  pc = 0x40a6c32; continue;
 }
 0x40a6c32 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6c39 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x414b26cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x40a6c45; continue;
 }
 0x40a6c45 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6c53; continue; }
  pc = 0x40a6c4d; continue;
 }
 0x40a6c4d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6c53; continue; }
  pc = 0x40a6c51; continue;
 }
 0x40a6c51 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6c53; continue;
 }
 0x40a6c53 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6c60; continue; }
  pc = 0x40a6c58; continue;
 }
 0x40a6c58 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6c60; continue; }
  pc = 0x40a6c5d; continue;
 }
 0x40a6c5d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6c60; continue;
 }
 0x40a6c60 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6c6f; continue; }
  pc = 0x40a6c64; continue;
 }
 0x40a6c64 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6c6f; continue; }
  pc = 0x40a6c68; continue;
 }
 0x40a6c68 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6c6f; continue; }
  pc = 0x40a6c6b; continue;
 }
 0x40a6c6b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6c45; continue;
 }
 0x40a6c6f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6c76; continue; }
  pc = 0x40a6c74; continue;
 }
 0x40a6c74 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6c76; continue;
 }
 0x40a6c76 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a6c7e; continue; }
  pc = 0x40a6c7a; continue;
 }
 0x40a6c7a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a6ce2; continue;
 }
 0x40a6c7e => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b278u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  pc = 0x40a6c8b; continue;
 }
 0x40a6c8b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6c99; continue; }
  pc = 0x40a6c93; continue;
 }
 0x40a6c93 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6c99; continue; }
  pc = 0x40a6c97; continue;
 }
 0x40a6c97 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6c99; continue;
 }
 0x40a6c99 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6ca6; continue; }
  pc = 0x40a6c9e; continue;
 }
 0x40a6c9e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6ca6; continue; }
  pc = 0x40a6ca3; continue;
 }
 0x40a6ca3 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6ca6; continue;
 }
 0x40a6ca6 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6cb5; continue; }
  pc = 0x40a6caa; continue;
 }
 0x40a6caa => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6cb5; continue; }
  pc = 0x40a6cae; continue;
 }
 0x40a6cae => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6cb5; continue; }
  pc = 0x40a6cb1; continue;
 }
 0x40a6cb1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6c8b; continue;
 }
 0x40a6cb5 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6cbc; continue; }
  pc = 0x40a6cba; continue;
 }
 0x40a6cba => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6cbc; continue;
 }
 0x40a6cbc => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40a6cdd; continue; }
  pc = 0x40a6cc0; continue;
 }
 0x40a6cc0 => {
  { let v: u32 = 0x414b280u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a6ccc; continue;
 }
 0x40a6ccc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a6ccc; continue; }
  pc = 0x40a6cd6; continue;
 }
 0x40a6cd6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6cdd => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x40a6ce2; continue;
 }
 0x40a6ce2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a6cec); f_0409b800(cpu); if cpu.returned_to != 0x40a6cec { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a6daa; continue; }
  pc = 0x40a6cfb; continue;
 }
 0x40a6cfb => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a6d01); f_0409b580(cpu); if cpu.returned_to != 0x40a6d01 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x888u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x40a6d19; continue; }
  pc = 0x40a6d12; continue;
 }
 0x40a6d12 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x40a6d64; continue; }
  pc = 0x40a6d17; continue;
 }
 0x40a6d17 => {
  pc = 0x40a6d82; continue;
 }
 0x40a6d19 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x898u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x40a6f30u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a6d57); f_040999b0(cpu); if cpu.returned_to != 0x40a6d57 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a6d62); f_040a6f78(cpu); if cpu.returned_to != 0x40a6d62 { return; }
  pc = 0x40a6d7f; continue;
 }
 0x40a6d64 => {
  { let v = 0x40a6db0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a6d75); f_040999b0(cpu); if cpu.returned_to != 0x40a6d75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a6d7f); f_040a6df0(cpu); if cpu.returned_to != 0x40a6d7f { return; }
  pc = 0x40a6d7f; continue;
 }
 0x40a6d7f => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a6d82; continue;
 }
 0x40a6d82 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a6da8; continue; }
  pc = 0x40a6d8b; continue;
 }
 0x40a6d8b => {
  { let v: u32 = 0x414b2a8u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a6d97; continue;
 }
 0x40a6d97 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a6d97; continue; }
  pc = 0x40a6da1; continue;
 }
 0x40a6da1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6da8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a6daa; continue;
 }
 0x40a6daa => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6db0); }
 let mut pc: u32 = 0x40a6db0;
 loop { match pc {
 0x40a6db0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a6dc5; continue; }
  pc = 0x40a6dbe; continue;
 }
 0x40a6dbe => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x40a6dc5); cpu.call(t); if cpu.returned_to != 0x40a6dc5 { return; }
  pc = 0x40a6dc5; continue;
 }
 0x40a6dc5 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a6dd7; continue; }
  pc = 0x40a6dce; continue;
 }
 0x40a6dce => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a6dd4); f_0409b630(cpu); if cpu.returned_to != 0x40a6dd4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a6dd7; continue;
 }
 0x40a6dd7 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6df0); }
 let mut pc: u32 = 0x40a6df0;
 loop { match pc {
 0x40a6df0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x414b25cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x40a6e04; continue;
 }
 0x40a6e04 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a6e0d; continue;
 }
 0x40a6e0d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6e1b; continue; }
  pc = 0x40a6e15; continue;
 }
 0x40a6e15 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6e1b; continue; }
  pc = 0x40a6e19; continue;
 }
 0x40a6e19 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6e1b; continue;
 }
 0x40a6e1b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6e28; continue; }
  pc = 0x40a6e20; continue;
 }
 0x40a6e20 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6e28; continue; }
  pc = 0x40a6e25; continue;
 }
 0x40a6e25 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6e28; continue;
 }
 0x40a6e28 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6e37; continue; }
  pc = 0x40a6e2c; continue;
 }
 0x40a6e2c => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6e37; continue; }
  pc = 0x40a6e30; continue;
 }
 0x40a6e30 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6e37; continue; }
  pc = 0x40a6e33; continue;
 }
 0x40a6e33 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6e0d; continue;
 }
 0x40a6e37 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6e3e; continue; }
  pc = 0x40a6e3c; continue;
 }
 0x40a6e3c => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6e3e; continue;
 }
 0x40a6e3e => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40a6e54; continue; }
  pc = 0x40a6e42; continue;
 }
 0x40a6e42 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x40a6e04; continue;
 }
 0x40a6e54 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x40a6e68; continue; }
  pc = 0x40a6e5b; continue;
 }
 0x40a6e5b => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x40a6e68; continue; }
  pc = 0x40a6e62; continue;
 }
 0x40a6e62 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x40a6ea2; continue;
 }
 0x40a6e68 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x40a6e7c; continue; }
  pc = 0x40a6e6f; continue;
 }
 0x40a6e6f => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x40a6e7c; continue; }
  pc = 0x40a6e76; continue;
 }
 0x40a6e76 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x40a6ea2; continue;
 }
 0x40a6e7c => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x40a6e90; continue; }
  pc = 0x40a6e83; continue;
 }
 0x40a6e83 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x40a6e90; continue; }
  pc = 0x40a6e8a; continue;
 }
 0x40a6e8a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x40a6ea2; continue;
 }
 0x40a6e90 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x40a6eab; continue; }
  pc = 0x40a6e97; continue;
 }
 0x40a6e97 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x40a6eab; continue; }
  pc = 0x40a6e9e; continue;
 }
 0x40a6e9e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x40a6ea2; continue;
 }
 0x40a6ea2 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a6ea8); f_0409b2c0(cpu); if cpu.returned_to != 0x40a6ea8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x40a6eab; continue;
 }
 0x40a6eab => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a6eb5); f_0409af40(cpu); if cpu.returned_to != 0x40a6eb5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x40a6ebf; continue;
 }
 0x40a6ebf => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b264u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a6ecb; continue;
 }
 0x40a6ecb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6ed9; continue; }
  pc = 0x40a6ed3; continue;
 }
 0x40a6ed3 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6ed9; continue; }
  pc = 0x40a6ed7; continue;
 }
 0x40a6ed7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6ed9; continue;
 }
 0x40a6ed9 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a6ee6; continue; }
  pc = 0x40a6ede; continue;
 }
 0x40a6ede => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a6ee6; continue; }
  pc = 0x40a6ee3; continue;
 }
 0x40a6ee3 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a6ee6; continue;
 }
 0x40a6ee6 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a6ef5; continue; }
  pc = 0x40a6eea; continue;
 }
 0x40a6eea => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a6ef5; continue; }
  pc = 0x40a6eee; continue;
 }
 0x40a6eee => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40a6ef5; continue; }
  pc = 0x40a6ef1; continue;
 }
 0x40a6ef1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a6ecb; continue;
 }
 0x40a6ef5 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a6efc; continue; }
  pc = 0x40a6efa; continue;
 }
 0x40a6efa => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6efc; continue;
 }
 0x40a6efc => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40a6f12; continue; }
  pc = 0x40a6f00; continue;
 }
 0x40a6f00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x40a6ebf; continue;
 }
 0x40a6f12 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a6f20); f_0409b390(cpu); if cpu.returned_to != 0x40a6f20 { return; }
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
pub fn f_040a6f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a6f30); }
 let mut pc: u32 = 0x40a6f30;
 loop { match pc {
 0x40a6f30 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a6f3c); f_040a6f78(cpu); if cpu.returned_to != 0x40a6f3c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a6f50(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a6f50); } r_040a6f50(cpu, 0x40a6f50); }
pub fn f_040a6f78(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a6f78); } r_040a6f50(cpu, 0x40a6f78); }
fn r_040a6f50(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a6f50 => {
  { let v: u32 = cpu.eax; cpu.w32(0xb1040a6fu32, v); }
  cpu.unimplemented(0x40a6f55, "Outsd");
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.of { pc = 0x40a6f65; continue; }
  pc = 0x40a6f5b; continue;
 }
 0x40a6f5b => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0xc7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.of { pc = 0x40a6f69; continue; }
  pc = 0x40a6f5f; continue;
 }
 0x40a6f5f => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x1au8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.of { pc = 0x40a6f6d; continue; }
  pc = 0x40a6f63; continue;
 }
 0x40a6f63 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0xc7u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f65; continue;
 }
 0x40a6f65 => {
  if !cpu.of { pc = 0x40a6f71; continue; }
  pc = 0x40a6f67; continue;
 }
 0x40a6f67 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x51u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f69; continue;
 }
 0x40a6f69 => {
  if cpu.of { pc = 0x40a6f75; continue; }
  pc = 0x40a6f6b; continue;
 }
 0x40a6f6b => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x68u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f6d; continue;
 }
 0x40a6f6d => {
  if cpu.of { pc = 0x40a6f79; continue; }
  pc = 0x40a6f6f; continue;
 }
 0x40a6f6f => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x92u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f71; continue;
 }
 0x40a6f71 => {
  if cpu.of { pc = 0x40a6f7d; continue; }
  pc = 0x40a6f73; continue;
 }
 0x40a6f73 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x2cu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f75; continue;
 }
 0x40a6f75 => {
  if !cpu.of { pc = 0x40a6f81; continue; }
  pc = 0x40a6f77; continue;
 }
 0x40a6f77 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x53u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a6f79; continue;
 }
 0x40a6f78 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x40a6f79; continue;
 }
 0x40a6f79 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a6f86; continue;
 }
 0x40a6f7d => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) & 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a6f86; continue;
 }
 0x40a6f81 => {
  cpu.unimplemented(0x40a6f81, "Insb");
  { let v: u8 = (cpu.eax as u8) & 0x18u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a6f86; continue;
 }
 0x40a6f86 => {
  { let v: u32 = cpu.r32(0x88cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a71c7; continue; }
  pc = 0x40a6f96; continue;
 }
 0x40a6f96 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x40a6f50u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x40a6fa3 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a6fa9); f_040a71f0(cpu); if cpu.returned_to != 0x40a6fa9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a6fb1 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a71c7; continue; }
  pc = 0x40a6fbe; continue;
 }
 0x40a6fbe => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a6fc4); f_040a7230(cpu); if cpu.returned_to != 0x40a6fc4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a6fd5); f_0409b390(cpu); if cpu.returned_to != 0x40a6fd5 { return; }
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
  cpu.push(0x40a6ff4); f_0409af40(cpu); if cpu.returned_to != 0x40a6ff4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7001); f_0409b2c0(cpu); if cpu.returned_to != 0x40a7001 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a71c2; continue; }
  pc = 0x40a700c; continue;
 }
 0x40a700c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7012); f_0409b210(cpu); if cpu.returned_to != 0x40a7012 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a71c2; continue;
 }
 0x40a701a => {
  { let v: u32 = cpu.r32(0x898u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a71c7; continue; }
  pc = 0x40a7029; continue;
 }
 0x40a7029 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x40a7042; continue; }
  pc = 0x40a7033; continue;
 }
 0x40a7033 => {
  { let v: u32 = 0x1u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a71c7; continue;
 }
 0x40a7042 => {
  { let v: u32 = 0x0u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a71c7; continue;
 }
 0x40a7051 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x890u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a71c7; continue;
 }
 0x40a7068 => {
  { let v: u32 = cpu.r32(0x894u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffu32); }
  if cpu.zf { pc = 0x40a7087; continue; }
  pc = 0x40a7076; continue;
 }
 0x40a7076 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a71c7; continue; }
  pc = 0x40a7087; continue;
 }
 0x40a7087 => {
  { let v: u32 = cpu.r32(0x890u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x40a71c7; continue;
 }
 0x40a7092 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a71c7; continue; }
  pc = 0x40a709f; continue;
 }
 0x40a709f => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x40a70c5; continue; }
  pc = 0x40a70a5; continue;
 }
 0x40a70a5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a70af); f_0409b2c0(cpu); if cpu.returned_to != 0x40a70af { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7a12000u32; cpu.eax = v; }
  pc = 0x40a70e3; continue;
 }
 0x40a70c5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a70cf); f_0409b2c0(cpu); if cpu.returned_to != 0x40a70cf { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xf424000u32; cpu.eax = v; }
  pc = 0x40a70e3; continue;
 }
 0x40a70e3 => {
  { let v = cpu.edi; cpu.div32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a70ef); f_0409af40(cpu); if cpu.returned_to != 0x40a70ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7102); f_040a7230(cpu); if cpu.returned_to != 0x40a7102 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7113); f_0409b390(cpu); if cpu.returned_to != 0x40a7113 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a71c2; continue; }
  pc = 0x40a711e; continue;
 }
 0x40a711e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7124); f_0409b210(cpu); if cpu.returned_to != 0x40a7124 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a71c2; continue;
 }
 0x40a712c => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40a71c7; continue; }
  pc = 0x40a7139; continue;
 }
 0x40a7139 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a713f); f_040a7230(cpu); if cpu.returned_to != 0x40a713f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7150); f_0409b390(cpu); if cpu.returned_to != 0x40a7150 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a715d); f_0409af40(cpu); if cpu.returned_to != 0x40a715d { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x40a7175; continue; }
  pc = 0x40a7168; continue;
 }
 0x40a7168 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x40a7175; continue; }
  pc = 0x40a716f; continue;
 }
 0x40a716f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x40a71ac; continue;
 }
 0x40a7175 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x40a7188; continue; }
  pc = 0x40a717b; continue;
 }
 0x40a717b => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x40a7188; continue; }
  pc = 0x40a7182; continue;
 }
 0x40a7182 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x40a71ac; continue;
 }
 0x40a7188 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x40a719b; continue; }
  pc = 0x40a718e; continue;
 }
 0x40a718e => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x40a719b; continue; }
  pc = 0x40a7195; continue;
 }
 0x40a7195 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x40a71ac; continue;
 }
 0x40a719b => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x40a71b5; continue; }
  pc = 0x40a71a1; continue;
 }
 0x40a71a1 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x40a71b5; continue; }
  pc = 0x40a71a8; continue;
 }
 0x40a71a8 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x40a71ac; continue;
 }
 0x40a71ac => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a71b2); f_0409b2c0(cpu); if cpu.returned_to != 0x40a71b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x40a71b5; continue;
 }
 0x40a71b5 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a71c2; continue; }
  pc = 0x40a71b9; continue;
 }
 0x40a71b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a71bf); f_0409b210(cpu); if cpu.returned_to != 0x40a71bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a71c2; continue;
 }
 0x40a71c2 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x40a71c7; continue;
 }
 0x40a71c7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a71cd); f_040a7230(cpu); if cpu.returned_to != 0x40a71cd { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x40a6f86; continue; }
  pc = 0x40a71e3; continue;
 }
 0x40a71e3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a71f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a71f0); }
 let mut pc: u32 = 0x40a71f0;
 loop { match pc {
 0x40a71f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a7206; continue; }
  pc = 0x40a71ff; continue;
 }
 0x40a71ff => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x40a7206); cpu.call(t); if cpu.returned_to != 0x40a7206 { return; }
  pc = 0x40a7206; continue;
 }
 0x40a7206 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a7218; continue; }
  pc = 0x40a720f; continue;
 }
 0x40a720f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7215); f_0409b630(cpu); if cpu.returned_to != 0x40a7215 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a7218; continue;
 }
 0x40a7218 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7230); }
 let mut pc: u32 = 0x40a7230;
 loop { match pc {
 0x40a7230 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7240); }
 let mut pc: u32 = 0x40a7240;
 loop { match pc {
 0x40a7240 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c678u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a7269; continue; }
  pc = 0x40a724c; continue;
 }
 0x40a724c => {
  { let v = 0x40a6a60u32; cpu.push(v); }
  { let v = 0x40a7240u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40a7260); f_040a30c0(cpu); if cpu.returned_to != 0x40a7260 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c678u32, v); }
  pc = 0x40a7269; continue;
 }
 0x40a7269 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7270); }
 let mut pc: u32 = 0x40a7270;
 loop { match pc {
 0x40a7270 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c700u32), 0x0u32); }
  if cpu.zf { pc = 0x40a736f; continue; }
  pc = 0x40a7280; continue;
 }
 0x40a7280 => {
  { let v = 0x40a7270u32; cpu.push(v); }
  { let v = 0x40aaa50u32; cpu.push(v); }
  cpu.push(0x40a728f); f_040a3020(cpu); if cpu.returned_to != 0x40a728f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x415c67cu32; cpu.push(v); }
  cpu.push(0x40a72a1); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c6fcu32; cpu.push(v); }
  cpu.push(0x40a72b0); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f9cu32; cpu.push(v); }
  cpu.push(0x40a72bf); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fa0u32; cpu.push(v); }
  cpu.push(0x40a72ce); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fa8u32; cpu.push(v); }
  cpu.push(0x40a72dd); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb8u32; cpu.push(v); }
  cpu.push(0x40a72ec); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72ec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5facu32; cpu.push(v); }
  cpu.push(0x40a72fb); f_040a2fe0(cpu); if cpu.returned_to != 0x40a72fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb0u32; cpu.push(v); }
  cpu.push(0x40a730a); f_040a2fe0(cpu); if cpu.returned_to != 0x40a730a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f98u32; cpu.push(v); }
  cpu.push(0x40a7319); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7319 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb4u32; cpu.push(v); }
  cpu.push(0x40a7328); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7328 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f94u32; cpu.push(v); }
  cpu.push(0x40a7337); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7337 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f90u32; cpu.push(v); }
  cpu.push(0x40a7346); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7346 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f88u32; cpu.push(v); }
  cpu.push(0x40a7355); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7355 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f8cu32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40a7366); f_040a2fe0(cpu); if cpu.returned_to != 0x40a7366 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c700u32, v); }
  pc = 0x40a736f; continue;
 }
 0x40a736f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7380); }
 let mut pc: u32 = 0x40a7380;
 loop { match pc {
 0x40a7380 => {
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
  cpu.push(0x40a73a4); f_040aa720(cpu); if cpu.returned_to != 0x40a73a4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x40a73b8); f_040aa9a0(cpu); if cpu.returned_to != 0x40a73b8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a73ca; continue; }
  pc = 0x40a73c3; continue;
 }
 0x40a73c3 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x40a73cc; continue;
 }
 0x40a73ca => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x40a73cc; continue;
 }
 0x40a73cc => {
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
  cpu.push(0x40a73fe); f_040aa9a0(cpu); if cpu.returned_to != 0x40a73fe { return; }
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
pub fn f_040a7530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7530); }
 let mut pc: u32 = 0x40a7530;
 loop { match pc {
 0x40a7530 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a7561; continue; }
  pc = 0x40a753d; continue;
 }
 0x40a753d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x40a7561; continue; }
  pc = 0x40a7559; continue;
 }
 0x40a7559 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x40a7561; continue; }
  pc = 0x40a755e; continue;
 }
 0x40a755e => {
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a7561; continue;
 }
 0x40a7561 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7570); }
 let mut pc: u32 = 0x40a7570;
 loop { match pc {
 0x40a7570 => {
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
  if !cpu.zf { pc = 0x40a767a; continue; }
  pc = 0x40a7595; continue;
 }
 0x40a7595 => {
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
  cpu.push(0x40a75be); f_040aa820(cpu); if cpu.returned_to != 0x40a75be { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a7648; continue; }
  pc = 0x40a75d5; continue;
 }
 0x40a75d5 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a75de; continue;
 }
 0x40a75de => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x40a7629; continue; }
  pc = 0x40a75e4; continue;
 }
 0x40a75e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40a75e9; continue;
 }
 0x40a75e9 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a7622; continue; }
  pc = 0x40a75f4; continue;
 }
 0x40a75f4 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x40a7622; continue; }
  pc = 0x40a75ff; continue;
 }
 0x40a75ff => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7615); f_040a9d70(cpu); if cpu.returned_to != 0x40a7615 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7622; continue;
 }
 0x40a7622 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40a75e9; continue; }
  pc = 0x40a7629; continue;
 }
 0x40a7629 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40a75de; continue; }
  pc = 0x40a7648; continue;
 }
 0x40a7648 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a7673; continue; }
  pc = 0x40a765f; continue;
 }
 0x40a765f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x40a7673; continue; }
  pc = 0x40a7669; continue;
 }
 0x40a7669 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7670); f_040a99d0(cpu); if cpu.returned_to != 0x40a7670 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a7673; continue;
 }
 0x40a7673 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a767a; continue;
 }
 0x40a767a => {
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
pub fn f_040a7690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7690); }
 let mut pc: u32 = 0x40a7690;
 loop { match pc {
 0x40a7690 => {
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
  pc = 0x40a76b9; continue;
 }
 0x40a76b9 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x40a76d8; continue; }
  pc = 0x40a76be; continue;
 }
 0x40a76be => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.zf { pc = 0x40a76d8; continue; }
  pc = 0x40a76c5; continue;
 }
 0x40a76c5 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x40a76d8; continue; }
  pc = 0x40a76ca; continue;
 }
 0x40a76ca => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a76d8; continue; }
  pc = 0x40a76d4; continue;
 }
 0x40a76d4 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a76d8; continue;
 }
 0x40a76d8 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a76b9; continue; }
  pc = 0x40a76e1; continue;
 }
 0x40a76e1 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a7718; continue; }
  pc = 0x40a76e6; continue;
 }
 0x40a76e6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x40a76f7; continue;
 }
 0x40a76f7 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x40a770f; continue; }
  pc = 0x40a76fc; continue;
 }
 0x40a76fc => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x24u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a770f; continue; }
  pc = 0x40a7701; continue;
 }
 0x40a7701 => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a770f; continue; }
  pc = 0x40a770b; continue;
 }
 0x40a770b => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x40a770f; continue;
 }
 0x40a770f => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a76f7; continue; }
  pc = 0x40a7718; continue;
 }
 0x40a7718 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a772f; continue; }
  pc = 0x40a771d; continue;
 }
 0x40a771d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7800; continue;
 }
 0x40a772f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7742); f_040aa820(cpu); if cpu.returned_to != 0x40a7742 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a77c6; continue; }
  pc = 0x40a7757; continue;
 }
 0x40a7757 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a7760; continue;
 }
 0x40a7760 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x40a77a7; continue; }
  pc = 0x40a7766; continue;
 }
 0x40a7766 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40a776b; continue;
 }
 0x40a776b => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a77a0; continue; }
  pc = 0x40a7776; continue;
 }
 0x40a7776 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x40a77a0; continue; }
  pc = 0x40a777f; continue;
 }
 0x40a777f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a7793); f_040a9d70(cpu); if cpu.returned_to != 0x40a7793 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a77a0; continue;
 }
 0x40a77a0 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x40a776b; continue; }
  pc = 0x40a77a7; continue;
 }
 0x40a77a7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a7760; continue; }
  pc = 0x40a77c6; continue;
 }
 0x40a77c6 => {
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
  pc = 0x40a7800; continue;
 }
 0x40a7800 => {
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
pub fn f_040a7810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7810); }
 let mut pc: u32 = 0x40a7810;
 loop { match pc {
 0x40a7810 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a781f; continue; }
  pc = 0x40a7818; continue;
 }
 0x40a7818 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a781f; continue;
 }
 0x40a781f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7820); }
 let mut pc: u32 = 0x40a7820;
 loop { match pc {
 0x40a7820 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a782b; continue; }
  pc = 0x40a7828; continue;
 }
 0x40a7828 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a782b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6dcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7840); }
 let mut pc: u32 = 0x40a7840;
 loop { match pc {
 0x40a7840 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a785b; continue; }
  pc = 0x40a7848; continue;
 }
 0x40a7848 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6dcu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a785b; continue;
 }
 0x40a785b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7860); }
 let mut pc: u32 = 0x40a7860;
 loop { match pc {
 0x40a7860 => {
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
pub fn f_040a7880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7880); }
 let mut pc: u32 = 0x40a7880;
 loop { match pc {
 0x40a7880 => {
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
pub fn f_040a78c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a78c0); }
 let mut pc: u32 = 0x40a78c0;
 loop { match pc {
 0x40a78c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a78d5; continue; }
  pc = 0x40a78c9; continue;
 }
 0x40a78c9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40a78d5; continue;
 }
 0x40a78d5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7920); }
 let mut pc: u32 = 0x40a7920;
 loop { match pc {
 0x40a7920 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40a79a7; continue; }
  pc = 0x40a7934; continue;
 }
 0x40a7934 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a79a7; continue; }
  pc = 0x40a793f; continue;
 }
 0x40a793f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a7963; continue; }
  pc = 0x40a7950; continue;
 }
 0x40a7950 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x40a7952; continue;
 }
 0x40a7952 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.zf { pc = 0x40a7963; continue; }
  pc = 0x40a795b; continue;
 }
 0x40a795b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a7952; continue; }
  pc = 0x40a7963; continue;
 }
 0x40a7963 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x40a79a7; continue; }
  pc = 0x40a7967; continue;
 }
 0x40a7967 => {
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
  { let v: u32 = cpu.r32(0x42c5f1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40a79a7; continue; }
  pc = 0x40a7991; continue;
 }
 0x40a7991 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.edx = v; }
  pc = 0x40a7996; continue;
 }
 0x40a7996 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a7996; continue; }
  pc = 0x40a79a7; continue;
 }
 0x40a79a7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a79b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a79b0); }
 let mut pc: u32 = 0x40a79b0;
 loop { match pc {
 0x40a79b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a7a1d; continue; }
  pc = 0x40a79bc; continue;
 }
 0x40a79bc => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5f18u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a79fd; continue; }
  pc = 0x40a79db; continue;
 }
 0x40a79db => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40a79f2; continue; }
  pc = 0x40a79e7; continue;
 }
 0x40a79e7 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x68u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x40a79f2; continue; }
  pc = 0x40a79ef; continue;
 }
 0x40a79ef => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a79f2; continue;
 }
 0x40a79f2 => {
  { let v: u32 = cpu.r32(0x42c5f18u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a79db; continue; }
  pc = 0x40a79fd; continue;
 }
 0x40a79fd => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40a7a03; continue; }
  pc = 0x40a7a01; continue;
 }
 0x40a7a01 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a7a03; continue;
 }
 0x40a7a03 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a7a0d; continue; }
  pc = 0x40a7a0b; continue;
 }
 0x40a7a0b => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a7a0d; continue;
 }
 0x40a7a0d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a7a17; continue; }
  pc = 0x40a7a15; continue;
 }
 0x40a7a15 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a7a17; continue;
 }
 0x40a7a17 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a7a1d; continue;
 }
 0x40a7a1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7a30); }
 let mut pc: u32 = 0x40a7a30;
 loop { match pc {
 0x40a7a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a7a3d; continue; }
  pc = 0x40a7a39; continue;
 }
 0x40a7a39 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a7a3d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x40u32); cpu.ecx = v; }
  pc = 0x40a7a49; continue;
 }
 0x40a7a49 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40a7a52; continue; }
  pc = 0x40a7a51; continue;
 }
 0x40a7a51 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a7a52; continue;
 }
 0x40a7a52 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a7a49; continue; }
  pc = 0x40a7a59; continue;
 }
 0x40a7a59 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7d20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7d20); }
 let mut pc: u32 = 0x40a7d20;
 loop { match pc {
 0x40a7d20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a7d29; continue; }
  pc = 0x40a7d28; continue;
 }
 0x40a7d28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a7d29 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7d30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7d30); }
 let mut pc: u32 = 0x40a7d30;
 loop { match pc {
 0x40a7d30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a7da9; continue; }
  pc = 0x40a7d3f; continue;
 }
 0x40a7d3f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x40a7d60; continue; }
  pc = 0x40a7d56; continue;
 }
 0x40a7d56 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a7d60 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a7d69; continue; }
  pc = 0x40a7d64; continue;
 }
 0x40a7d64 => {
  { let v: u32 = cpu.esi; cpu.w32(0x3cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7d96; continue;
 }
 0x40a7d69 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a7d83; continue; }
  pc = 0x40a7d81; continue;
 }
 0x40a7d81 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a7d83; continue;
 }
 0x40a7d83 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7d96; continue;
 }
 0x40a7d96 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7d9c); f_040a98b0(cpu); if cpu.returned_to != 0x40a7d9c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a7da9; continue;
 }
 0x40a7da9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7db0); }
 let mut pc: u32 = 0x40a7db0;
 loop { match pc {
 0x40a7db0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a7e11; continue; }
  pc = 0x40a7dbe; continue;
 }
 0x40a7dbe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40a7e0b; continue; }
  pc = 0x40a7dd5; continue;
 }
 0x40a7dd5 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a7dde; continue; }
  pc = 0x40a7dd9; continue;
 }
 0x40a7dd9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7e0b; continue;
 }
 0x40a7dde => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a7df8; continue; }
  pc = 0x40a7df6; continue;
 }
 0x40a7df6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40a7df8; continue;
 }
 0x40a7df8 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7e0b; continue;
 }
 0x40a7e0b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a7e11; continue;
 }
 0x40a7e11 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7e20); }
 let mut pc: u32 = 0x40a7e20;
 loop { match pc {
 0x40a7e20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a7e2f; continue; }
  pc = 0x40a7e28; continue;
 }
 0x40a7e28 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a7e2f; continue;
 }
 0x40a7e2f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7e30); }
 let mut pc: u32 = 0x40a7e30;
 loop { match pc {
 0x40a7e30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a7e62; continue; }
  pc = 0x40a7e3a; continue;
 }
 0x40a7e3a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40a7e62; continue; }
  pc = 0x40a7e42; continue;
 }
 0x40a7e42 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x40a7e62; continue; }
  pc = 0x40a7e47; continue;
 }
 0x40a7e47 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7e4d); f_04098ed0(cpu); if cpu.returned_to != 0x40a7e4d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a7e62; continue; }
  pc = 0x40a7e5e; continue;
 }
 0x40a7e5e => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x40a7e62); cpu.call(t); if cpu.returned_to != 0x40a7e62 { return; }
  pc = 0x40a7e62; continue;
 }
 0x40a7e62 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7e70); }
 let mut pc: u32 = 0x40a7e70;
 loop { match pc {
 0x40a7e70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40a7ecb; continue; }
  pc = 0x40a7e7c; continue;
 }
 0x40a7e7c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x8u32); }
  if !cpu.zf { pc = 0x40a7ecb; continue; }
  pc = 0x40a7e82; continue;
 }
 0x40a7e82 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a7e89; continue;
 }
 0x40a7e89 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a7ea9; continue; }
  pc = 0x40a7e92; continue;
 }
 0x40a7e92 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a7e98); f_04096f70(cpu); if cpu.returned_to != 0x40a7e98 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a7ea3; continue; }
  pc = 0x40a7ea1; continue;
 }
 0x40a7ea1 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40a7ea3; continue;
 }
 0x40a7ea3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.esi), v); }
  pc = 0x40a7ea9; continue;
 }
 0x40a7ea9 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a7e89; continue; }
  pc = 0x40a7eb2; continue;
 }
 0x40a7eb2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a7eb4; continue;
 }
 0x40a7eb4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a7ebb); f_040a99d0(cpu); if cpu.returned_to != 0x40a7ebb { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a7eb4; continue; }
  pc = 0x40a7ec4; continue;
 }
 0x40a7ec4 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40a7ecb; continue;
 }
 0x40a7ecb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7ed0); }
 let mut pc: u32 = 0x40a7ed0;
 loop { match pc {
 0x40a7ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a7f6c; continue; }
  pc = 0x40a7ee0; continue;
 }
 0x40a7ee0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.zf { pc = 0x40a7f6c; continue; }
  pc = 0x40a7eea; continue;
 }
 0x40a7eea => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x40u32); cpu.ebp = v; }
  cpu.push(0x40a7efc); f_040a9cb0(cpu); if cpu.returned_to != 0x40a7efc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x40a7f02; continue;
 }
 0x40a7f02 => {
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a7f25; continue; }
  pc = 0x40a7f13; continue;
 }
 0x40a7f13 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a7f22); f_040aa820(cpu); if cpu.returned_to != 0x40a7f22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a7f25; continue;
 }
 0x40a7f25 => {
  { cpu.sub32(cpu.r32(0x1d8u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a7f36; continue; }
  pc = 0x40a7f2e; continue;
 }
 0x40a7f2e => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x40a7f36; continue;
 }
 0x40a7f36 => {
  { cpu.sub32(cpu.r32(0x258u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a7f51; continue; }
  pc = 0x40a7f3f; continue;
 }
 0x40a7f3f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a7f4e); f_040aa820(cpu); if cpu.returned_to != 0x40a7f4e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a7f51; continue;
 }
 0x40a7f51 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a7f65; continue; }
  pc = 0x40a7f5a; continue;
 }
 0x40a7f5a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a7f62); f_04096eb0(cpu); if cpu.returned_to != 0x40a7f62 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x40a7f65; continue;
 }
 0x40a7f65 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x40a7f02; continue; }
  pc = 0x40a7f6c; continue;
 }
 0x40a7f6c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7f80); }
 let mut pc: u32 = 0x40a7f80;
 loop { match pc {
 0x40a7f80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a7fb1; continue; }
  pc = 0x40a7f89; continue;
 }
 0x40a7f89 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if cpu.zf { pc = 0x40a7fb1; continue; }
  pc = 0x40a7f8f; continue;
 }
 0x40a7f8f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7f95); f_04098ed0(cpu); if cpu.returned_to != 0x40a7f95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a7f9e); f_040a9910(cpu); if cpu.returned_to != 0x40a7f9e { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a7fb1; continue;
 }
 0x40a7fb1 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a7fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a7fc0); }
 let mut pc: u32 = 0x40a7fc0;
 loop { match pc {
 0x40a7fc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a7fd6; continue; }
  pc = 0x40a7fcf; continue;
 }
 0x40a7fcf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a83d7; continue;
 }
 0x40a7fd6 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40a7fec); f_040aa590(cpu); if cpu.returned_to != 0x40a7fec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8013; continue; }
  pc = 0x40a7ff3; continue;
 }
 0x40a7ff3 => {
  { let v: u32 = 0x414b4f0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a7fff; continue;
 }
 0x40a7fff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a7fff; continue; }
  pc = 0x40a8009; continue;
 }
 0x40a8009 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8013 => {
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
  if !cpu.cf { pc = 0x40a8112; continue; }
  pc = 0x40a8070; continue;
 }
 0x40a8070 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b508u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a8086; continue; }
  pc = 0x40a8084; continue;
 }
 0x40a8084 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a8086; continue;
 }
 0x40a8086 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a8091; continue; }
  pc = 0x40a808a; continue;
 }
 0x40a808a => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a8091; continue;
 }
 0x40a8091 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b510u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a80a7; continue; }
  pc = 0x40a80a5; continue;
 }
 0x40a80a5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a80a7; continue;
 }
 0x40a80a7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a80b2; continue; }
  pc = 0x40a80ab; continue;
 }
 0x40a80ab => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a80b2; continue;
 }
 0x40a80b2 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b518u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a80c8; continue; }
  pc = 0x40a80c6; continue;
 }
 0x40a80c6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a80c8; continue;
 }
 0x40a80c8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a80d3; continue; }
  pc = 0x40a80cc; continue;
 }
 0x40a80cc => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a80d3; continue;
 }
 0x40a80d3 => {
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
  if cpu.cf { cpu.poll(); pc = 0x40a8070; continue; }
  pc = 0x40a8112; continue;
 }
 0x40a8112 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40a813c; continue; }
  pc = 0x40a811c; continue;
 }
 0x40a811c => {
  { let v: u32 = 0x414b4f0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8128; continue;
 }
 0x40a8128 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8128; continue; }
  pc = 0x40a8132; continue;
 }
 0x40a8132 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a813c => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a816e); f_040a9910(cpu); if cpu.returned_to != 0x40a816e { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x42c5f14u32); cpu.eax = v; }
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
  if cpu.zf { pc = 0x40a83d2; continue; }
  pc = 0x40a81c5; continue;
 }
 0x40a81c5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a81ce); f_040aa6e0(cpu); if cpu.returned_to != 0x40a81ce { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a81e2; continue; }
  pc = 0x40a81db; continue;
 }
 0x40a81db => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x40a81f6; continue;
 }
 0x40a81e2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a81f0); f_040aa6e0(cpu); if cpu.returned_to != 0x40a81f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x40a81f6; continue;
 }
 0x40a81f6 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x42c5fbcu32; cpu.edi = v; }
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
  { let v: u32 = 0x42c5fbcu32; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a82ca; continue; }
  pc = 0x40a822a; continue;
 }
 0x40a822a => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a822e; continue;
 }
 0x40a822e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x40a82ca; continue; }
  pc = 0x40a823c; continue;
 }
 0x40a823c => {
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
  let t = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.push(0x40a8265); cpu.call(t); if cpu.returned_to != 0x40a8265 { return; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8273; continue; }
  pc = 0x40a826c; continue;
 }
 0x40a826c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.poll(); pc = 0x40a822e; continue;
 }
 0x40a8273 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x42c5fbcu32); cpu.eax = v; }
  pc = 0x40a828f; continue;
 }
 0x40a8283 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a828f; continue;
 }
 0x40a828f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x40a8283; continue; }
  pc = 0x40a8299; continue;
 }
 0x40a8299 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.cf { pc = 0x40a82bd; continue; }
  pc = 0x40a82a8; continue;
 }
 0x40a82a8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x40a822e; continue;
 }
 0x40a82bd => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x40a822e; continue;
 }
 0x40a82ca => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x40a83d2; continue; }
  pc = 0x40a82d5; continue;
 }
 0x40a82d5 => {
  cpu.push(0x40a82da); f_0409ca70(cpu); if cpu.returned_to != 0x40a82da { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8301; continue; }
  pc = 0x40a82de; continue;
 }
 0x40a82de => {
  { let v: u32 = 0x414b520u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a82ea; continue;
 }
 0x40a82ea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a82ea; continue; }
  pc = 0x40a82f4; continue;
 }
 0x40a82f4 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8301 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a8357; continue; }
  pc = 0x40a831f; continue;
 }
 0x40a831f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8340); f_0409cb30(cpu); if cpu.returned_to != 0x40a8340 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a8357; continue;
 }
 0x40a8357 => {
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
  cpu.push(0x40a8398); f_0409cb30(cpu); if cpu.returned_to != 0x40a8398 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x40a83d2; continue; }
  pc = 0x40a83ad; continue;
 }
 0x40a83ad => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.push(0x40a83c5); f_040a83e0(cpu); if cpu.returned_to != 0x40a83c5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a83d2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a83d7; continue;
 }
 0x40a83d7 => {
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
pub fn f_040a83e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a83e0); }
 let mut pc: u32 = 0x40a83e0;
 loop { match pc {
 0x40a83e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = 0x42c5de0u32; cpu.push(v); }
  { let v = 0x414b4bcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40a8404); cpu.call(t); if cpu.returned_to != 0x40a8404 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a8410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8410); }
 let mut pc: u32 = 0x40a8410;
 loop { match pc {
 0x40a8410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a8429; continue; }
  pc = 0x40a8419; continue;
 }
 0x40a8419 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a841f); f_04098ed0(cpu); if cpu.returned_to != 0x40a841f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a8429; continue;
 }
 0x40a8429 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a8430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8430); }
 let mut pc: u32 = 0x40a8430;
 loop { match pc {
 0x40a8430 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x40a843c); f_0409cfd0(cpu); if cpu.returned_to != 0x40a843c { return; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a845c; continue; }
  pc = 0x40a8445; continue;
 }
 0x40a8445 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x40a8448; continue;
 }
 0x40a8448 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x40a845c; continue; }
  pc = 0x40a844e; continue;
 }
 0x40a844e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a8448; continue; }
  pc = 0x40a845c; continue;
 }
 0x40a845c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x40a8482; continue; }
  pc = 0x40a8461; continue;
 }
 0x40a8461 => {
  { let v: u32 = 0x414b4a0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a846d; continue;
 }
 0x40a846d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a846d; continue; }
  pc = 0x40a8477; continue;
 }
 0x40a8477 => {
  cpu.push(0x40a847c); f_0409cfc0(cpu); if cpu.returned_to != 0x40a847c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8482 => {
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
  cpu.push(0x40a84a5); f_040a9910(cpu); if cpu.returned_to != 0x40a84a5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a84b4); f_0409cfc0(cpu); if cpu.returned_to != 0x40a84b4 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a84c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a84c0); }
 let mut pc: u32 = 0x40a84c0;
 loop { match pc {
 0x40a84c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a84ce; continue;
 }
 0x40a84ce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a84ce; continue; }
  pc = 0x40a84d8; continue;
 }
 0x40a84d8 => {
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a8531; continue; }
  pc = 0x40a84f0; continue;
 }
 0x40a84f0 => {
  { let v: u8 = cpu.r8(0x415c67cu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x40a851d; continue; }
  pc = 0x40a84fb; continue;
 }
 0x40a84fb => {
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a8531; continue; }
  pc = 0x40a8511; continue;
 }
 0x40a8511 => {
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x415c67cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a851d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x40a852e; continue; }
  pc = 0x40a8522; continue;
 }
 0x40a8522 => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x415c67cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a852e => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40a84f0; continue; }
  pc = 0x40a8531; continue;
 }
 0x40a8531 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a8540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8540); }
 let mut pc: u32 = 0x40a8540;
 loop { match pc {
 0x40a8540 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x112u32); }
  if cpu.cf { pc = 0x40a85e4; continue; }
  pc = 0x40a855a; continue;
 }
 0x40a855a => {
  { let v: u32 = cpu.edx.wrapping_add(0xbau32); cpu.edx = v; }
  { let v: u32 = 0x414b464u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a8567; continue;
 }
 0x40a8567 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a8575; continue; }
  pc = 0x40a856f; continue;
 }
 0x40a856f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a8575; continue; }
  pc = 0x40a8573; continue;
 }
 0x40a8573 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a8575; continue;
 }
 0x40a8575 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a8582; continue; }
  pc = 0x40a857a; continue;
 }
 0x40a857a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a8582; continue; }
  pc = 0x40a857f; continue;
 }
 0x40a857f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a8582; continue;
 }
 0x40a8582 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a858e; continue; }
  pc = 0x40a8586; continue;
 }
 0x40a8586 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a858e; continue; }
  pc = 0x40a858a; continue;
 }
 0x40a858a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a8567; continue;
 }
 0x40a858e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a8595; continue; }
  pc = 0x40a8593; continue;
 }
 0x40a8593 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a8595; continue;
 }
 0x40a8595 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a85a2; continue; }
  pc = 0x40a8599; continue;
 }
 0x40a8599 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a85a2 => {
  { let v: u32 = 0x414b478u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a85a9; continue;
 }
 0x40a85a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a85b7; continue; }
  pc = 0x40a85b1; continue;
 }
 0x40a85b1 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a85b7; continue; }
  pc = 0x40a85b5; continue;
 }
 0x40a85b5 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a85b7; continue;
 }
 0x40a85b7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a85c4; continue; }
  pc = 0x40a85bc; continue;
 }
 0x40a85bc => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a85c4; continue; }
  pc = 0x40a85c1; continue;
 }
 0x40a85c1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a85c4; continue;
 }
 0x40a85c4 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a85d0; continue; }
  pc = 0x40a85c8; continue;
 }
 0x40a85c8 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a85d0; continue; }
  pc = 0x40a85cc; continue;
 }
 0x40a85cc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a85a9; continue;
 }
 0x40a85d0 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a85d7; continue; }
  pc = 0x40a85d5; continue;
 }
 0x40a85d5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a85d7; continue;
 }
 0x40a85d7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a85e4; continue; }
  pc = 0x40a85db; continue;
 }
 0x40a85db => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a85e4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  if cpu.zf { pc = 0x40a869a; continue; }
  pc = 0x40a8601; continue;
 }
 0x40a8601 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x40a869a; continue; }
  pc = 0x40a860a; continue;
 }
 0x40a860a => {
  { let v: u32 = 0x414b494u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a8611; continue;
 }
 0x40a8611 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a861f; continue; }
  pc = 0x40a8619; continue;
 }
 0x40a8619 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a861f; continue; }
  pc = 0x40a861d; continue;
 }
 0x40a861d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a861f; continue;
 }
 0x40a861f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a862c; continue; }
  pc = 0x40a8624; continue;
 }
 0x40a8624 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a862c; continue; }
  pc = 0x40a8629; continue;
 }
 0x40a8629 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a862c; continue;
 }
 0x40a862c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a8638; continue; }
  pc = 0x40a8630; continue;
 }
 0x40a8630 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a8638; continue; }
  pc = 0x40a8634; continue;
 }
 0x40a8634 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a8611; continue;
 }
 0x40a8638 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a863f; continue; }
  pc = 0x40a863d; continue;
 }
 0x40a863d => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a863f; continue;
 }
 0x40a863f => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a8658; continue; }
  pc = 0x40a8643; continue;
 }
 0x40a8643 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x5000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a869a; continue; }
  pc = 0x40a864f; continue;
 }
 0x40a864f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8658 => {
  { let v: u32 = 0x414b498u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a865f; continue;
 }
 0x40a865f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a866d; continue; }
  pc = 0x40a8667; continue;
 }
 0x40a8667 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a866d; continue; }
  pc = 0x40a866b; continue;
 }
 0x40a866b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a866d; continue;
 }
 0x40a866d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40a867a; continue; }
  pc = 0x40a8672; continue;
 }
 0x40a8672 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a867a; continue; }
  pc = 0x40a8677; continue;
 }
 0x40a8677 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a867a; continue;
 }
 0x40a867a => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a8686; continue; }
  pc = 0x40a867e; continue;
 }
 0x40a867e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a8686; continue; }
  pc = 0x40a8682; continue;
 }
 0x40a8682 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40a865f; continue;
 }
 0x40a8686 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40a868d; continue; }
  pc = 0x40a868b; continue;
 }
 0x40a868b => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40a868d; continue;
 }
 0x40a868d => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a869a; continue; }
  pc = 0x40a8691; continue;
 }
 0x40a8691 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a869a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a86a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a86a0); }
 let mut pc: u32 = 0x40a86a0;
 loop { match pc {
 0x40a86a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40a86ad); f_0409bbc0(cpu); if cpu.returned_to != 0x40a86ad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a86c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a86c0); }
 let mut pc: u32 = 0x40a86c0;
 loop { match pc {
 0x40a86c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x414b43cu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a86d7); f_0409c740(cpu); if cpu.returned_to != 0x40a86d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a86fb; continue; }
  pc = 0x40a86de; continue;
 }
 0x40a86de => {
  { let v: u32 = 0x414b444u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a86ea; continue;
 }
 0x40a86ea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a86ea; continue; }
  pc = 0x40a86f4; continue;
 }
 0x40a86f4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a8730; continue;
 }
 0x40a86fb => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8710); f_040993b0(cpu); if cpu.returned_to != 0x40a8710 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a872e; continue; }
  pc = 0x40a8720; continue;
 }
 0x40a8720 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a872e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a8730; continue;
 }
 0x40a8730 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a8740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8740); }
 let mut pc: u32 = 0x40a8740;
 loop { match pc {
 0x40a8740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a8751); f_0409bc70(cpu); if cpu.returned_to != 0x40a8751 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8779; continue; }
  pc = 0x40a875a; continue;
 }
 0x40a875a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8765); f_040a88d0(cpu); if cpu.returned_to != 0x40a8765 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8777; continue; }
  pc = 0x40a876e; continue;
 }
 0x40a876e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a8774); f_0409bbc0(cpu); if cpu.returned_to != 0x40a8774 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40a8777; continue;
 }
 0x40a8777 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a8779; continue;
 }
 0x40a8779 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a8780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8780); }
 let mut pc: u32 = 0x40a8780;
 loop { match pc {
 0x40a8780 => {
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
  cpu.push(0x40a87a4); f_040a2e70(cpu); if cpu.returned_to != 0x40a87a4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8862; continue; }
  pc = 0x40a87b1; continue;
 }
 0x40a87b1 => {
  { cpu.sub8(cpu.r8(0x415c4e8u32), 0x0u8); }
  if cpu.zf { pc = 0x40a884a; continue; }
  pc = 0x40a87be; continue;
 }
 0x40a87be => {
  { let v: u32 = 0x415c4e8u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a87c7; continue;
 }
 0x40a87c7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a87c7; continue; }
  pc = 0x40a87d1; continue;
 }
 0x40a87d1 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a87e2; continue;
 }
 0x40a87e2 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a87e2; continue; }
  pc = 0x40a87e8; continue;
 }
 0x40a87e8 => {
  { let v: u32 = 0x415c67cu32; cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a87f6; continue;
 }
 0x40a87f6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a87f6; continue; }
  pc = 0x40a8800; continue;
 }
 0x40a8800 => {
  { let v: u32 = 0x415c4e8u32; cpu.esi = v; }
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a880c; continue;
 }
 0x40a880c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a880c; continue; }
  pc = 0x40a8816; continue;
 }
 0x40a8816 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a882f; continue;
 }
 0x40a882f => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a882f; continue; }
  pc = 0x40a8835; continue;
 }
 0x40a8835 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8841); f_040a2e70(cpu); if cpu.returned_to != 0x40a8841 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8862; continue; }
  pc = 0x40a884a; continue;
 }
 0x40a884a => {
  { let v: u32 = 0x414b424u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8856; continue;
 }
 0x40a8856 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8856; continue; }
  pc = 0x40a8860; continue;
 }
 0x40a8860 => {
  pc = 0x40a88a0; continue;
 }
 0x40a8862 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a886e); f_0409bc70(cpu); if cpu.returned_to != 0x40a886e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40a8879); f_040a3230(cpu); if cpu.returned_to != 0x40a8879 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a88a0; continue; }
  pc = 0x40a8880; continue;
 }
 0x40a8880 => {
  { let v: u32 = cpu.r32(0x198u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a888e); f_040a88d0(cpu); if cpu.returned_to != 0x40a888e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a88a0; continue; }
  pc = 0x40a8897; continue;
 }
 0x40a8897 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a889d); f_0409bbc0(cpu); if cpu.returned_to != 0x40a889d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a88a0; continue;
 }
 0x40a88a0 => {
  { cpu.sub8(cpu.r8(0x100u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x40a88c2; continue; }
  pc = 0x40a88aa; continue;
 }
 0x40a88aa => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x415c67cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a88b8; continue;
 }
 0x40a88b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a88b8; continue; }
  pc = 0x40a88c2; continue;
 }
 0x40a88c2 => {
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
pub fn f_040a88d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a88d0); }
 let mut pc: u32 = 0x40a88d0;
 loop { match pc {
 0x40a88d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x40a88dc); f_040aaa50(cpu); if cpu.returned_to != 0x40a88dc { return; }
  { let v = 0x1d8u32; cpu.push(v); }
  cpu.push(0x40a88e6); f_040a3260(cpu); if cpu.returned_to != 0x40a88e6 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a890c; continue; }
  pc = 0x40a88ef; continue;
 }
 0x40a88ef => {
  { let v: u32 = 0x414b348u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a88fb; continue;
 }
 0x40a88fb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a88fb; continue; }
  pc = 0x40a8905; continue;
 }
 0x40a8905 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a8f9a; continue;
 }
 0x40a890c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b370u32; cpu.esi = v; }
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
  if cpu.zf { pc = 0x40a8964; continue; }
  pc = 0x40a8939; continue;
 }
 0x40a8939 => {
  { let v: u32 = 0x414b378u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8947; continue;
 }
 0x40a8947 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8947; continue; }
  pc = 0x40a8951; continue;
 }
 0x40a8951 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a8957); f_040a3230(cpu); if cpu.returned_to != 0x40a8957 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8964 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8976); f_0409cb30(cpu); if cpu.returned_to != 0x40a8976 { return; }
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
  if cpu.zf { pc = 0x40a8a0e; continue; }
  pc = 0x40a89bf; continue;
 }
 0x40a89bf => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40a8a0e; continue; }
  pc = 0x40a89c4; continue;
 }
 0x40a89c4 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b910u32); cpu.push(0x40a89cb); cpu.call(t); if cpu.returned_to != 0x40a89cb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8a0e; continue; }
  pc = 0x40a89d1; continue;
 }
 0x40a89d1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40a8a0e; continue; }
  pc = 0x40a89d6; continue;
 }
 0x40a89d6 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a89f0; continue; }
  pc = 0x40a89eb; continue;
 }
 0x40a89eb => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x40a89f0; continue;
 }
 0x40a89f0 => {
  { let v: u32 = cpu.r32(0x42c5f90u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40a8a0e; continue;
 }
 0x40a8a0e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x40a8a66; continue; }
  pc = 0x40a8a27; continue;
 }
 0x40a8a27 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x40a8a66; continue; }
  pc = 0x40a8a2c; continue;
 }
 0x40a8a2c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x415c67cu32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8a3c; continue;
 }
 0x40a8a3c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8a3c; continue; }
  pc = 0x40a8a46; continue;
 }
 0x40a8a46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a8a5e; continue;
 }
 0x40a8a5e => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8a5e; continue; }
  pc = 0x40a8a64; continue;
 }
 0x40a8a64 => {
  pc = 0x40a8a70; continue;
 }
 0x40a8a66 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a8a70; continue;
 }
 0x40a8a70 => {
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x40a8ad7; continue; }
  pc = 0x40a8aa3; continue;
 }
 0x40a8aa3 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8aba); f_040a9010(cpu); if cpu.returned_to != 0x40a8aba { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8ad7; continue; }
  pc = 0x40a8ac1; continue;
 }
 0x40a8ac1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x40a8ad7; continue;
 }
 0x40a8ad7 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a8b20; continue; }
  pc = 0x40a8ade; continue;
 }
 0x40a8ade => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a8ae7); f_0409bd80(cpu); if cpu.returned_to != 0x40a8ae7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8b20; continue; }
  pc = 0x40a8af0; continue;
 }
 0x40a8af0 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x40a8b05); f_040a9010(cpu); if cpu.returned_to != 0x40a8b05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8b20; continue; }
  pc = 0x40a8b0c; continue;
 }
 0x40a8b0c => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x40a8b20; continue;
 }
 0x40a8b20 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40a8bbb; continue; }
  pc = 0x40a8b2c; continue;
 }
 0x40a8b2c => {
  { cpu.sub32(cpu.r32(0x42c5f24u32), 0x1u32); }
  if !cpu.zf { pc = 0x40a8bbb; continue; }
  pc = 0x40a8b39; continue;
 }
 0x40a8b39 => {
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a8ba7; continue;
 }
 0x40a8b43 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a8b6f; continue; }
  pc = 0x40a8b61; continue;
 }
 0x40a8b61 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x40a8b6f; continue;
 }
 0x40a8b6f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8b76); f_040a9010(cpu); if cpu.returned_to != 0x40a8b76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a8b93; continue; }
  pc = 0x40a8b7d; continue;
 }
 0x40a8b7d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x40a8bbb; continue;
 }
 0x40a8b93 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a8ba7; continue;
 }
 0x40a8ba7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a8b43; continue; }
  pc = 0x40a8bbb; continue;
 }
 0x40a8bbb => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a8beb; continue; }
  pc = 0x40a8bc2; continue;
 }
 0x40a8bc2 => {
  { let v: u32 = 0x414b390u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8bce; continue;
 }
 0x40a8bce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8bce; continue; }
  pc = 0x40a8bd8; continue;
 }
 0x40a8bd8 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8bde); f_040a3230(cpu); if cpu.returned_to != 0x40a8bde { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8beb => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x42c5f30u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a8c0d); f_0409cb30(cpu); if cpu.returned_to != 0x40a8c0d { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x501u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8c2f); f_0409cb30(cpu); if cpu.returned_to != 0x40a8c2f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x40a8c82; continue; }
  pc = 0x40a8c3a; continue;
 }
 0x40a8c3a => {
  { let v: u32 = 0x414b3b0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8c46; continue;
 }
 0x40a8c46 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8c46; continue; }
  pc = 0x40a8c50; continue;
 }
 0x40a8c50 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a8c62); f_0409cb30(cpu); if cpu.returned_to != 0x40a8c62 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a8c75); f_040a3230(cpu); if cpu.returned_to != 0x40a8c75 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8c82 => {
  { let v: u32 = cpu.r32(0x42c5f10u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8ca1); f_040a3260(cpu); if cpu.returned_to != 0x40a8ca1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a8cf3; continue; }
  pc = 0x40a8cab; continue;
 }
 0x40a8cab => {
  { let v: u32 = 0x414b3dcu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8cb7; continue;
 }
 0x40a8cb7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8cb7; continue; }
  pc = 0x40a8cc1; continue;
 }
 0x40a8cc1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a8cd3); f_0409cb30(cpu); if cpu.returned_to != 0x40a8cd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a8ce6); f_040a3230(cpu); if cpu.returned_to != 0x40a8ce6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8cf3 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a8d47; continue; }
  pc = 0x40a8d00; continue;
 }
 0x40a8d00 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40a8d02; continue;
 }
 0x40a8d02 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x414b404u32; cpu.esi = v; }
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a8d02; continue; }
  pc = 0x40a8d47; continue;
 }
 0x40a8d47 => {
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5f0cu32); cpu.ebx = v; }
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
  pc = 0x40a8d9c; continue;
 }
 0x40a8d9c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xe0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x120u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x160u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8d9c; continue; }
  pc = 0x40a8dc1; continue;
 }
 0x40a8dc1 => {
  { let v = 0x40a90f0u32; cpu.push(v); }
  cpu.push(0x40a8dcb); f_0409c640(cpu); if cpu.returned_to != 0x40a8dcb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a8e2a; continue; }
  pc = 0x40a8dd6; continue;
 }
 0x40a8dd6 => {
  { let v: u32 = 0x414b40cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a8de2; continue;
 }
 0x40a8de2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a8de2; continue; }
  pc = 0x40a8dec; continue;
 }
 0x40a8dec => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a8dfe); f_0409cb30(cpu); if cpu.returned_to != 0x40a8dfe { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8e14); f_040a3230(cpu); if cpu.returned_to != 0x40a8e14 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e1d); f_040a3230(cpu); if cpu.returned_to != 0x40a8e1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a8e2a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a8e31); f_0409c530(cpu); if cpu.returned_to != 0x40a8e31 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x40a8fb0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a8e48; continue;
 }
 0x40a8e48 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e57); f_040aa820(cpu); if cpu.returned_to != 0x40a8e57 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e67); f_040aa820(cpu); if cpu.returned_to != 0x40a8e67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e78); f_040aa820(cpu); if cpu.returned_to != 0x40a8e78 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x70u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e85); f_040aa820(cpu); if cpu.returned_to != 0x40a8e85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8e92); f_040aa820(cpu); if cpu.returned_to != 0x40a8e92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5f14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ea4); f_040aa820(cpu); if cpu.returned_to != 0x40a8ea4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8eb2); f_040aa820(cpu); if cpu.returned_to != 0x40a8eb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ec0); f_040aa820(cpu); if cpu.returned_to != 0x40a8ec0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ecd); f_040aa820(cpu); if cpu.returned_to != 0x40a8ecd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = 0x5bu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8edb); f_040aa820(cpu); if cpu.returned_to != 0x40a8edb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x5du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ee8); f_040aa820(cpu); if cpu.returned_to != 0x40a8ee8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ef5); f_040aa820(cpu); if cpu.returned_to != 0x40a8ef5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x65u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8f02); f_040aa820(cpu); if cpu.returned_to != 0x40a8f02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8f0f); f_040aa820(cpu); if cpu.returned_to != 0x40a8f0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5f20u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8f22); f_040aa820(cpu); if cpu.returned_to != 0x40a8f22 { return; }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40a8f5e; continue; }
  pc = 0x40a8f2f; continue;
 }
 0x40a8f2f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a8f4f); f_0409cb30(cpu); if cpu.returned_to != 0x40a8f4f { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a8f5e; continue;
 }
 0x40a8f5e => {
  { cpu.and16((cpu.esi as u16), 0x3u16); }
  if !cpu.zf { pc = 0x40a8f6f; continue; }
  pc = 0x40a8f65; continue;
 }
 0x40a8f65 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x40a8f6c); f_0409ca80(cpu); if cpu.returned_to != 0x40a8f6c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a8f6f; continue;
 }
 0x40a8f6f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a8e48; continue; }
  pc = 0x40a8f79; continue;
 }
 0x40a8f79 => {
  { let v: u32 = cpu.r32(0x42c5f0cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a8f89); f_0409c3b0(cpu); if cpu.returned_to != 0x40a8f89 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a8f95); f_0409c160(cpu); if cpu.returned_to != 0x40a8f95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x40a8f9a; continue;
 }
 0x40a8f9a => {
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
pub fn f_040a8fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a8fb0); }
 let mut pc: u32 = 0x40a8fb0;
 loop { match pc {
 0x40a8fb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a8fdf; continue; }
  pc = 0x40a8fc1; continue;
 }
 0x40a8fc1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a8fc3; continue;
 }
 0x40a8fc3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x40a8fcf); f_04098d70(cpu); if cpu.returned_to != 0x40a8fcf { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a8fc3; continue; }
  pc = 0x40a8fdf; continue;
 }
 0x40a8fdf => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a8fe8); f_0409bf00(cpu); if cpu.returned_to != 0x40a8fe8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40a8ff4); f_040a3230(cpu); if cpu.returned_to != 0x40a8ff4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a8ffd); f_040a3230(cpu); if cpu.returned_to != 0x40a8ffd { return; }
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
pub fn f_040a9010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a9010); }
 let mut pc: u32 = 0x40a9010;
 loop { match pc {
 0x40a9010 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x40a90b7; continue; }
  pc = 0x40a9039; continue;
 }
 0x40a9039 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a9060; continue; }
  pc = 0x40a9059; continue;
 }
 0x40a9059 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a9060; continue;
 }
 0x40a9060 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a9074; continue; }
  pc = 0x40a906b; continue;
 }
 0x40a906b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a9074; continue;
 }
 0x40a9074 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a9089; continue; }
  pc = 0x40a9080; continue;
 }
 0x40a9080 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a9089; continue;
 }
 0x40a9089 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a909e; continue; }
  pc = 0x40a9095; continue;
 }
 0x40a9095 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a909e; continue;
 }
 0x40a909e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a90a0; continue;
 }
 0x40a90a0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a90af; continue; }
  pc = 0x40a90a7; continue;
 }
 0x40a90a7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40a90af; continue;
 }
 0x40a90af => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a90a0; continue; }
  pc = 0x40a90b7; continue;
 }
 0x40a90b7 => {
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
  cpu.push(0x40a90dd); f_0409cb30(cpu); if cpu.returned_to != 0x40a90dd { return; }
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
pub fn f_040a90f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a90f0); }
 let mut pc: u32 = 0x40a90f0;
 loop { match pc {
 0x40a90f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a989a; continue; }
  pc = 0x40a9109; continue;
 }
 0x40a9109 => {
  { cpu.sub32(cpu.r32(0x415c6fcu32), 0x0u32); }
  if !cpu.zf { pc = 0x40a989a; continue; }
  pc = 0x40a9116; continue;
 }
 0x40a9116 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415c6fcu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5fa8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f9cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a9845; continue; }
  pc = 0x40a913a; continue;
 }
 0x40a913a => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x40a9821; continue; }
  pc = 0x40a9149; continue;
 }
 0x40a9149 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5fb8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9163; continue;
 }
 0x40a9163 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a9759; continue; }
  pc = 0x40a9174; continue;
 }
 0x40a9174 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a921b; continue; }
  pc = 0x40a9188; continue;
 }
 0x40a9188 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5fa0u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x40a9195; continue;
 }
 0x40a9195 => {
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a9205; continue; }
  pc = 0x40a91ad; continue;
 }
 0x40a91ad => {
  { let v: u32 = cpu.r32(0x65cu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x65cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40a9205; continue; }
  pc = 0x40a91bf; continue;
 }
 0x40a91bf => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a91de); f_040a9d70(cpu); if cpu.returned_to != 0x40a91de { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x40a921b; continue; }
  pc = 0x40a9205; continue;
 }
 0x40a9205 => {
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5fa0u32, v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a9195; continue; }
  pc = 0x40a921b; continue;
 }
 0x40a921b => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a96cc; continue; }
  pc = 0x40a922f; continue;
 }
 0x40a922f => {
  { cpu.sub32(cpu.r32(0x42c5fb8u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a96aa; continue; }
  pc = 0x40a923c; continue;
 }
 0x40a923c => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5fb4u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.cf { pc = 0x40a96aa; continue; }
  pc = 0x40a925a; continue;
 }
 0x40a925a => {
  { cpu.sub32(cpu.eax, 0xf7u32); }
  if cpu.cf { pc = 0x40a9273; continue; }
  pc = 0x40a9261; continue;
 }
 0x40a9261 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a945f; continue; }
  pc = 0x40a9267; continue;
 }
 0x40a9267 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { pc = 0x40a9283; continue; }
  pc = 0x40a926e; continue;
 }
 0x40a926e => {
  pc = 0x40a94fe; continue;
 }
 0x40a9273 => {
  { cpu.sub32(cpu.eax, 0xf0u32); }
  if cpu.zf { pc = 0x40a945f; continue; }
  pc = 0x40a927e; continue;
 }
 0x40a927e => {
  pc = 0x40a94fe; continue;
 }
 0x40a9283 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x18u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f94u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a929b); f_040aa720(cpu); if cpu.returned_to != 0x40a929b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f90u32, v); }
  { let v: u32 = cpu.r32(0x42c5f94u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x51u32); }
  if cpu.cf { pc = 0x40a92c1; continue; }
  pc = 0x40a92ad; continue;
 }
 0x40a92ad => {
  if (cpu.cf || cpu.zf) { pc = 0x40a9364; continue; }
  pc = 0x40a92b3; continue;
 }
 0x40a92b3 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if cpu.zf { pc = 0x40a93ab; continue; }
  pc = 0x40a92bc; continue;
 }
 0x40a92bc => {
  pc = 0x40a944c; continue;
 }
 0x40a92c1 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x40a944c; continue; }
  pc = 0x40a92ca; continue;
 }
 0x40a92ca => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5fb8u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a92ea; continue; }
  pc = 0x40a92e1; continue;
 }
 0x40a92e1 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x40a9328; continue; }
  pc = 0x40a92ea; continue;
 }
 0x40a92ea => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40a944c; continue; }
  pc = 0x40a9318; continue;
 }
 0x40a9318 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x40a9449; continue;
 }
 0x40a9328 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a9331); f_04098ed0(cpu); if cpu.returned_to != 0x40a9331 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a944c; continue; }
  pc = 0x40a934b; continue;
 }
 0x40a934b => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x40a9351); cpu.call(t); if cpu.returned_to != 0x40a9351 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a9364 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
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
  { let v: u32 = cpu.ecx; cpu.w32(0x42c5fb0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a93ab => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5fb0u32, v); }
  { let v: u32 = 0xf42400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5f0cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x42c5fb0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5facu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40a9406; continue; }
  pc = 0x40a93f2; continue;
 }
 0x40a93f2 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5fb0u32, v); }
  { let v: u8 = cpu.r8(0x42c5fb0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x40a940e; continue;
 }
 0x40a9406 => {
  { let v: u8 = cpu.r8(0x42c5fb0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x40a940e; continue;
 }
 0x40a940e => {
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
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
  if cpu.zf { pc = 0x40a944c; continue; }
  pc = 0x40a943d; continue;
 }
 0x40a943d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x40a9449; continue;
 }
 0x40a9449 => {
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.push(0x40a944c); cpu.call(t); if cpu.returned_to != 0x40a944c { return; }
  pc = 0x40a944c; continue;
 }
 0x40a944c => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a945f => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = 0x42c5f88u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f88u32, v); }
  cpu.push(0x40a9477); f_040aa720(cpu); if cpu.returned_to != 0x40a9477 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c5f88u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f90u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f90u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40a94a5); f_040aa9a0(cpu); if cpu.returned_to != 0x40a94a5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a94b7; continue; }
  pc = 0x40a94b0; continue;
 }
 0x40a94b0 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x40a94b9; continue;
 }
 0x40a94b7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x40a94b9; continue;
 }
 0x40a94b9 => {
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
  cpu.push(0x40a94e3); f_040aa9a0(cpu); if cpu.returned_to != 0x40a94e3 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f90u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a94fe => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5f8cu32, v); }
  { let v: u32 = cpu.r32(0x42c5fb4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5f98u32, v); }
  { let v: u32 = cpu.r32(0x42c5fb4u32) & 0xf0u32; cpu.w32(0x42c5fb4u32, v); }
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
  cpu.push(0x40a9545); f_040a9d70(cpu); if cpu.returned_to != 0x40a9545 { return; }
  { let v: u32 = cpu.r32(0x42c5fb4u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if cpu.zf { pc = 0x40a95d7; continue; }
  pc = 0x40a955a; continue;
 }
 0x40a955a => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5f8cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40a922f; continue; }
  pc = 0x40a9570; continue;
 }
 0x40a9570 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x40a95a0; continue; }
  pc = 0x40a9583; continue;
 }
 0x40a9583 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a95b9; continue; }
  pc = 0x40a9585; continue;
 }
 0x40a9585 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x40a9597; continue; }
  pc = 0x40a958c; continue;
 }
 0x40a958c => {
  if (cpu.cf || cpu.zf) { pc = 0x40a95c0; continue; }
  pc = 0x40a958e; continue;
 }
 0x40a958e => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x40a95b9; continue; }
  pc = 0x40a9595; continue;
 }
 0x40a9595 => {
  pc = 0x40a95c7; continue;
 }
 0x40a9597 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x40a95c0; continue; }
  pc = 0x40a959e; continue;
 }
 0x40a959e => {
  pc = 0x40a95c7; continue;
 }
 0x40a95a0 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x40a95b2; continue; }
  pc = 0x40a95a7; continue;
 }
 0x40a95a7 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a95b9; continue; }
  pc = 0x40a95a9; continue;
 }
 0x40a95a9 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x40a95b9; continue; }
  pc = 0x40a95b0; continue;
 }
 0x40a95b0 => {
  pc = 0x40a95c7; continue;
 }
 0x40a95b2 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x40a95c7; continue; }
  pc = 0x40a95b9; continue;
 }
 0x40a95b9 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x40a95c9; continue;
 }
 0x40a95c0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x40a95c9; continue;
 }
 0x40a95c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a95c9; continue;
 }
 0x40a95c9 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a95d7 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c5fa0u32, v); }
  pc = 0x40a95e7; continue;
 }
 0x40a95e7 => {
  { cpu.sub32(cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9608; continue; }
  pc = 0x40a95f1; continue;
 }
 0x40a95f1 => {
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c5fa0u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a95e7; continue; }
  pc = 0x40a9608; continue;
 }
 0x40a9608 => {
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if !cpu.zf { pc = 0x40a9651; continue; }
  pc = 0x40a9613; continue;
 }
 0x40a9613 => {
  { let v: u32 = 0x414b328u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40a961f; continue;
 }
 0x40a961f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a961f; continue; }
  pc = 0x40a9629; continue;
 }
 0x40a9629 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x40a9637); f_04098ed0(cpu); if cpu.returned_to != 0x40a9637 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x415c6fcu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a989a; continue;
 }
 0x40a9651 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x558u32.wrapping_add(cpu.eax))); cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c5f98u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40a968f); f_040aa720(cpu); if cpu.returned_to != 0x40a968f { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x65cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x40a922f; continue;
 }
 0x40a96aa => {
  { cpu.sub32(cpu.r32(0x42c5fb8u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a96cc; continue; }
  pc = 0x40a96b3; continue;
 }
 0x40a96b3 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a96cc; continue;
 }
 0x40a96cc => {
  { cpu.sub32(cpu.r32(0x42c5fb8u32), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a9163; continue; }
  pc = 0x40a96d9; continue;
 }
 0x40a96d9 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a9163; continue; }
  pc = 0x40a96f4; continue;
 }
 0x40a96f4 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40a9717; continue; }
  pc = 0x40a9709; continue;
 }
 0x40a9709 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9717; continue;
 }
 0x40a9717 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x40a9163; continue; }
  pc = 0x40a9726; continue;
 }
 0x40a9726 => {
  { let v = 0x42c5fa4u32; cpu.push(v); }
  { let v = 0x42c5fa0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a9736); f_04097a30(cpu); if cpu.returned_to != 0x40a9736 { return; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c5fa0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c5fa4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x40a9754); cpu.call(t); if cpu.returned_to != 0x40a9754 { return; }
  cpu.poll(); pc = 0x40a9163; continue;
 }
 0x40a9759 => {
  { cpu.sub32(cpu.r32(0x42c5fb8u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a9821; continue; }
  pc = 0x40a9766; continue;
 }
 0x40a9766 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a97cb; continue; }
  pc = 0x40a976e; continue;
 }
 0x40a976e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a977c; continue;
 }
 0x40a977c => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x40a97b7; continue; }
  pc = 0x40a978b; continue;
 }
 0x40a978b => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a97a4; continue; }
  pc = 0x40a979c; continue;
 }
 0x40a979c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a97aa; continue;
 }
 0x40a97a4 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a97aa; continue;
 }
 0x40a97aa => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x40a977c; continue; }
  pc = 0x40a97b7; continue;
 }
 0x40a97b7 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x38u32.wrapping_add(cpu.eax)), 0x7u8); }
  if !cpu.zf { pc = 0x40a97cb; continue; }
  pc = 0x40a97c2; continue;
 }
 0x40a97c2 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a97c8); f_040a98b0(cpu); if cpu.returned_to != 0x40a97c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a97cb; continue;
 }
 0x40a97cb => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40a9821; continue; }
  pc = 0x40a97d8; continue;
 }
 0x40a97d8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a97e6; continue;
 }
 0x40a97e6 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40a9821; continue; }
  pc = 0x40a97f5; continue;
 }
 0x40a97f5 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a980e; continue; }
  pc = 0x40a9806; continue;
 }
 0x40a9806 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9814; continue;
 }
 0x40a980e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9814; continue;
 }
 0x40a9814 => {
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x40a97e6; continue; }
  pc = 0x40a9821; continue;
 }
 0x40a9821 => {
  { let v: u32 = cpu.r32(0x42c5fa8u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x42c5f9cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x71cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5fa8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5f9cu32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x40a913a; continue; }
  pc = 0x40a9845; continue;
 }
 0x40a9845 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a9892; continue; }
  pc = 0x40a9852; continue;
 }
 0x40a9852 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a9877); f_0409cb30(cpu); if cpu.returned_to != 0x40a9877 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9892; continue;
 }
 0x40a9892 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415c6fcu32, v); }
  pc = 0x40a989a; continue;
 }
 0x40a989a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a98b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a98b0); }
 let mut pc: u32 = 0x40a98b0;
 loop { match pc {
 0x40a98b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a98bb; continue;
 }
 0x40a98bb => {
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a98d9; continue; }
  pc = 0x40a98c6; continue;
 }
 0x40a98c6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40a98d6); f_040a9d70(cpu); if cpu.returned_to != 0x40a98d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a98d9; continue;
 }
 0x40a98d9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a98bb; continue; }
  pc = 0x40a98e2; continue;
 }
 0x40a98e2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a98f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a98f0); }
 let mut pc: u32 = 0x40a98f0;
 loop { match pc {
 0x40a98f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a98fb); f_040a9910(cpu); if cpu.returned_to != 0x40a98fb { return; }
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
pub fn f_040a9910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a9910); }
 let mut pc: u32 = 0x40a9910;
 loop { match pc {
 0x40a9910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a991b; continue;
 }
 0x40a991b => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a991b; continue; }
  pc = 0x40a992a; continue;
 }
 0x40a992a => {
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
  pc = 0x40a9955; continue;
 }
 0x40a9955 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a9955; continue; }
  pc = 0x40a9966; continue;
 }
 0x40a9966 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x40a996e; continue;
 }
 0x40a996e => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a996e; continue; }
  pc = 0x40a997f; continue;
 }
 0x40a997f => {
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
pub fn f_040a99d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a99d0); }
 let mut pc: u32 = 0x40a99d0;
 loop { match pc {
 0x40a99d0 => {
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
  if cpu.zf { pc = 0x40a9a03; continue; }
  pc = 0x40a99f0; continue;
 }
 0x40a99f0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9a00); f_040a9d70(cpu); if cpu.returned_to != 0x40a9a00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9a03; continue;
 }
 0x40a9a03 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xd8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9a2a; continue; }
  pc = 0x40a9a17; continue;
 }
 0x40a9a17 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9a27); f_040a9d70(cpu); if cpu.returned_to != 0x40a9a27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9a2a; continue;
 }
 0x40a9a2a => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9a56; continue; }
  pc = 0x40a9a3e; continue;
 }
 0x40a9a3e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9a53); f_040a9d70(cpu); if cpu.returned_to != 0x40a9a53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9a56; continue;
 }
 0x40a9a56 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9a7d; continue; }
  pc = 0x40a9a6a; continue;
 }
 0x40a9a6a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9a7a); f_040a9d70(cpu); if cpu.returned_to != 0x40a9a7a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9a7d; continue;
 }
 0x40a9a7d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9aa4; continue; }
  pc = 0x40a9a91; continue;
 }
 0x40a9a91 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6fu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9aa1); f_040a9d70(cpu); if cpu.returned_to != 0x40a9aa1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9aa4; continue;
 }
 0x40a9aa4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x258u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9acb; continue; }
  pc = 0x40a9ab8; continue;
 }
 0x40a9ab8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9ac8); f_040a9d70(cpu); if cpu.returned_to != 0x40a9ac8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9acb; continue;
 }
 0x40a9acb => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x358u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9af2; continue; }
  pc = 0x40a9adf; continue;
 }
 0x40a9adf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9aef); f_040a9d70(cpu); if cpu.returned_to != 0x40a9aef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9af2; continue;
 }
 0x40a9af2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9b19; continue; }
  pc = 0x40a9b06; continue;
 }
 0x40a9b06 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9b16); f_040a9d70(cpu); if cpu.returned_to != 0x40a9b16 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9b19; continue;
 }
 0x40a9b19 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9b40; continue; }
  pc = 0x40a9b2d; continue;
 }
 0x40a9b2d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9b3d); f_040a9d70(cpu); if cpu.returned_to != 0x40a9b3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9b40; continue;
 }
 0x40a9b40 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x418u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9b67; continue; }
  pc = 0x40a9b54; continue;
 }
 0x40a9b54 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9b64); f_040a9d70(cpu); if cpu.returned_to != 0x40a9b64 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9b67; continue;
 }
 0x40a9b67 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9b8e; continue; }
  pc = 0x40a9b7b; continue;
 }
 0x40a9b7b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9b8b); f_040a9d70(cpu); if cpu.returned_to != 0x40a9b8b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9b8e; continue;
 }
 0x40a9b8e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x498u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9bb5; continue; }
  pc = 0x40a9ba2; continue;
 }
 0x40a9ba2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9bb2); f_040a9d70(cpu); if cpu.returned_to != 0x40a9bb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9bb5; continue;
 }
 0x40a9bb5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9bdc; continue; }
  pc = 0x40a9bc9; continue;
 }
 0x40a9bc9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5du32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9bd9); f_040a9d70(cpu); if cpu.returned_to != 0x40a9bd9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9bdc; continue;
 }
 0x40a9bdc => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9c02; continue; }
  pc = 0x40a9bf0; continue;
 }
 0x40a9bf0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9bff); f_040a9d70(cpu); if cpu.returned_to != 0x40a9bff { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  pc = 0x40a9c02; continue;
 }
 0x40a9c02 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a9c10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a9c10); }
 let mut pc: u32 = 0x40a9c10;
 loop { match pc {
 0x40a9c10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x40a9c27; continue;
 }
 0x40a9c27 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x40a9c52; continue; }
  pc = 0x40a9c31; continue;
 }
 0x40a9c31 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9c45); f_040a9d70(cpu); if cpu.returned_to != 0x40a9c45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a9c52; continue;
 }
 0x40a9c52 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40a9c27; continue; }
  pc = 0x40a9c59; continue;
 }
 0x40a9c59 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a9c9d; continue; }
  pc = 0x40a9c65; continue;
 }
 0x40a9c65 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a9c86); f_0409cb30(cpu); if cpu.returned_to != 0x40a9c86 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a9c9d; continue;
 }
 0x40a9c9d => {
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
pub fn f_040a9cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a9cb0); }
 let mut pc: u32 = 0x40a9cb0;
 loop { match pc {
 0x40a9cb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  pc = 0x40a9cc5; continue;
 }
 0x40a9cc5 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9cf2; continue; }
  pc = 0x40a9cd0; continue;
 }
 0x40a9cd0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x40a9ce5); f_040a9d70(cpu); if cpu.returned_to != 0x40a9ce5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a9cf2; continue;
 }
 0x40a9cf2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x40a9cc5; continue; }
  pc = 0x40a9cf9; continue;
 }
 0x40a9cf9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a9d47; continue; }
  pc = 0x40a9d0f; continue;
 }
 0x40a9d0f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40a9d30); f_0409cb30(cpu); if cpu.returned_to != 0x40a9d30 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a9d47; continue;
 }
 0x40a9d47 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a9d5e; continue; }
  pc = 0x40a9d4b; continue;
 }
 0x40a9d4b => {
  cpu.push(0x40a9d50); f_0409ca70(cpu); if cpu.returned_to != 0x40a9d50 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a9d5e; continue; }
  pc = 0x40a9d54; continue;
 }
 0x40a9d54 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x40a9d5b); f_0409ca80(cpu); if cpu.returned_to != 0x40a9d5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40a9d5e; continue;
 }
 0x40a9d5e => {
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
pub fn f_040a9d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a9d70); }
 let mut pc: u32 = 0x40a9d70;
 loop { match pc {
 0x40a9d70 => {
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
  if cpu.zf { pc = 0x40a9dc4; continue; }
  pc = 0x40a9db4; continue;
 }
 0x40a9db4 => {
  { cpu.sub32(cpu.edx, 0xc0u32); }
  if cpu.zf { pc = 0x40a9dc4; continue; }
  pc = 0x40a9dbc; continue;
 }
 0x40a9dbc => {
  { cpu.sub32(cpu.edx, 0xe0u32); }
  if !cpu.zf { pc = 0x40a9de0; continue; }
  pc = 0x40a9dc4; continue;
 }
 0x40a9dc4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a9ddd); f_040aa1e0(cpu); if cpu.returned_to != 0x40a9ddd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40a9de0; continue;
 }
 0x40a9de0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xb0u32); }
  if !cpu.zf { pc = 0x40a9f17; continue; }
  pc = 0x40a9dee; continue;
 }
 0x40a9dee => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40a9e15; continue; }
  pc = 0x40a9df5; continue;
 }
 0x40a9df5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9e15; continue; }
  pc = 0x40a9e09; continue;
 }
 0x40a9e09 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2d8u32.wrapping_add(cpu.eax), v); }
  pc = 0x40a9e15; continue;
 }
 0x40a9e15 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, 0x6fu32); }
  if cpu.cf { pc = 0x40a9e6c; continue; }
  pc = 0x40a9e34; continue;
 }
 0x40a9e34 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa0a0; continue; }
  pc = 0x40a9e3a; continue;
 }
 0x40a9e3a => {
  { cpu.sub32(cpu.ebp, 0x75u32); }
  if cpu.cf { pc = 0x40a9e5c; continue; }
  pc = 0x40a9e3f; continue;
 }
 0x40a9e3f => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa019; continue; }
  pc = 0x40a9e45; continue;
 }
 0x40a9e45 => {
  { cpu.sub32(cpu.ebp, 0x76u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a9f88; continue; }
  pc = 0x40a9e4e; continue;
 }
 0x40a9e4e => {
  { cpu.sub32(cpu.ebp, 0x77u32); }
  if cpu.zf { pc = 0x40a9fc8; continue; }
  pc = 0x40a9e57; continue;
 }
 0x40a9e57 => {
  pc = 0x40a9f17; continue;
 }
 0x40a9e5c => {
  { cpu.sub32(cpu.ebp, 0x73u32); }
  if cpu.cf { pc = 0x40a9f17; continue; }
  pc = 0x40a9e65; continue;
 }
 0x40a9e65 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a9e9a; continue; }
  pc = 0x40a9e67; continue;
 }
 0x40a9e67 => {
  pc = 0x40a9fe4; continue;
 }
 0x40a9e6c => {
  { cpu.sub32(cpu.ebp, 0x6cu32); }
  if cpu.cf { pc = 0x40a9e81; continue; }
  pc = 0x40a9e71; continue;
 }
 0x40a9e71 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a9eb3; continue; }
  pc = 0x40a9e73; continue;
 }
 0x40a9e73 => {
  { cpu.sub32(cpu.ebp, 0x6du32); }
  if (cpu.cf || cpu.zf) { pc = 0x40aa08e; continue; }
  pc = 0x40a9e7c; continue;
 }
 0x40a9e7c => {
  pc = 0x40aa0cd; continue;
 }
 0x40a9e81 => {
  { cpu.sub32(cpu.ebp, 0x6u32); }
  if cpu.cf { pc = 0x40a9f17; continue; }
  pc = 0x40a9e8a; continue;
 }
 0x40a9e8a => {
  if (cpu.cf || cpu.zf) { pc = 0x40a9ed5; continue; }
  pc = 0x40a9e8c; continue;
 }
 0x40a9e8c => {
  { cpu.sub32(cpu.ebp, 0x7u32); }
  if cpu.zf { pc = 0x40a9f5b; continue; }
  pc = 0x40a9e95; continue;
 }
 0x40a9e95 => {
  pc = 0x40a9f17; continue;
 }
 0x40a9e9a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a9f17; continue;
 }
 0x40a9eb3 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40a9f17; continue; }
  pc = 0x40a9ebd; continue;
 }
 0x40a9ebd => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.push(0x40a9ec7); cpu.call(t); if cpu.returned_to != 0x40a9ec7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a9f17; continue;
 }
 0x40a9ed5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9ee8); f_040a9d70(cpu); if cpu.returned_to != 0x40a9ee8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x65u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9efe); f_040a9d70(cpu); if cpu.returned_to != 0x40a9efe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x26u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40a9f14); f_040a9d70(cpu); if cpu.returned_to != 0x40a9f14 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x40a9f17; continue;
 }
 0x40a9f17 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x40a9f33; continue; }
  pc = 0x40a9f2a; continue;
 }
 0x40a9f2a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x64u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40a9f33; continue;
 }
 0x40a9f33 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x90u32); }
  if !cpu.zf { pc = 0x40aa152; continue; }
  pc = 0x40a9f50; continue;
 }
 0x40a9f50 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x40aa160; continue;
 }
 0x40a9f5b => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x1b4u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v: u32 = 0x3f01u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a9f80; continue; }
  pc = 0x40a9f7b; continue;
 }
 0x40a9f7b => {
  { let v: u32 = 0x7fu32; cpu.edi = v; }
  pc = 0x40a9f80; continue;
 }
 0x40a9f80 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a9f17; continue; }
  pc = 0x40a9f84; continue;
 }
 0x40a9f84 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.poll(); pc = 0x40a9f17; continue;
 }
 0x40a9f88 => {
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40a9fb3; continue;
 }
 0x40a9fb3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.push(0x40a9fc0); cpu.call(t); if cpu.returned_to != 0x40a9fc0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a9fc8 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40a9fd2; continue;
 }
 0x40a9fd2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x40a9fdc); cpu.call(t); if cpu.returned_to != 0x40a9fdc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a9fe4 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a9fe8; continue;
 }
 0x40a9fe8 => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x40a9ffa; continue; }
  pc = 0x40a9ff1; continue;
 }
 0x40a9ff1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a9fe8; continue; }
  pc = 0x40a9ffa; continue;
 }
 0x40a9ffa => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa003; continue;
 }
 0x40aa003 => {
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
 0x40aa019 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40aa1cf; continue; }
  pc = 0x40aa022; continue;
 }
 0x40aa022 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x40aa02a; continue;
 }
 0x40aa02a => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x40aa03b; continue; }
  pc = 0x40aa033; continue;
 }
 0x40aa033 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40aa02a; continue; }
  pc = 0x40aa03b; continue;
 }
 0x40aa03b => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa044; continue;
 }
 0x40aa044 => {
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40aa061; continue; }
  pc = 0x40aa053; continue;
 }
 0x40aa053 => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa061 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40aa07c; continue; }
  pc = 0x40aa06e; continue;
 }
 0x40aa06e => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa07c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa08e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40aa095); f_04097970(cpu); if cpu.returned_to != 0x40aa095 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa0a0 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa0aa; continue;
 }
 0x40aa0aa => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x40aa0be; continue; }
  pc = 0x40aa0af; continue;
 }
 0x40aa0af => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa0be => {
  { let v: u32 = 0x2u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa0cd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40aa115; continue; }
  pc = 0x40aa0d7; continue;
 }
 0x40aa0d7 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa0e1; continue;
 }
 0x40aa0e1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40aa0eb); f_04096f70(cpu); if cpu.returned_to != 0x40aa0eb { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa0f8; continue;
 }
 0x40aa0f8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40aa100); f_04096de0(cpu); if cpu.returned_to != 0x40aa100 { return; }
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
 0x40aa115 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa121; continue;
 }
 0x40aa121 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40aa12c); f_040a9c10(cpu); if cpu.returned_to != 0x40aa12c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40aa13c); f_04096eb0(cpu); if cpu.returned_to != 0x40aa13c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40aa147); f_04096de0(cpu); if cpu.returned_to != 0x40aa147 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa152 => {
  { cpu.sub32(cpu.edx, 0x80u32); }
  if !cpu.zf { pc = 0x40aa160; continue; }
  pc = 0x40aa15a; continue;
 }
 0x40aa15a => {
  { let v: u32 = cpu.dec32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x40aa160; continue;
 }
 0x40aa160 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xe4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if !cpu.zf { pc = 0x40aa18d; continue; }
  pc = 0x40aa17f; continue;
 }
 0x40aa17f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x218u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x40aa1cf; continue; }
  pc = 0x40aa18d; continue;
 }
 0x40aa18d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40aa1b5; continue; }
  pc = 0x40aa19a; continue;
 }
 0x40aa19a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.push(0x40aa1b1); cpu.call(t); if cpu.returned_to != 0x40aa1b1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40aa1cf; continue; }
  pc = 0x40aa1b5; continue;
 }
 0x40aa1b5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40aa1cc); f_040aa820(cpu); if cpu.returned_to != 0x40aa1cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40aa1cf; continue;
 }
 0x40aa1cf => {
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
pub fn f_040aa1e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa1e0); }
 let mut pc: u32 = 0x40aa1e0;
 loop { match pc {
 0x40aa1e0 => {
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
  if cpu.cf { pc = 0x40aa21d; continue; }
  pc = 0x40aa20f; continue;
 }
 0x40aa20f => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa229; continue; }
  pc = 0x40aa211; continue;
 }
 0x40aa211 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x40aa234; continue; }
  pc = 0x40aa219; continue;
 }
 0x40aa219 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa21d => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x40aa24a; continue; }
  pc = 0x40aa225; continue;
 }
 0x40aa225 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa229 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa234 => {
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
 0x40aa24a => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x40aa29b; continue; }
  pc = 0x40aa24f; continue;
 }
 0x40aa24f => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa3a7; continue; }
  pc = 0x40aa255; continue;
 }
 0x40aa255 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x40aa285; continue; }
  pc = 0x40aa25a; continue;
 }
 0x40aa25a => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa316; continue; }
  pc = 0x40aa260; continue;
 }
 0x40aa260 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x40aa278; continue; }
  pc = 0x40aa265; continue;
 }
 0x40aa265 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa332; continue; }
  pc = 0x40aa26b; continue;
 }
 0x40aa26b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x40aa340; continue; }
  pc = 0x40aa274; continue;
 }
 0x40aa274 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa278 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x40aa324; continue; }
  pc = 0x40aa281; continue;
 }
 0x40aa281 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa285 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x40aa28e; continue; }
  pc = 0x40aa28a; continue;
 }
 0x40aa28a => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa2ec; continue; }
  pc = 0x40aa28c; continue;
 }
 0x40aa28c => {
  pc = 0x40aa2fa; continue;
 }
 0x40aa28e => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x40aa308; continue; }
  pc = 0x40aa297; continue;
 }
 0x40aa297 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa29b => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x40aa2cb; continue; }
  pc = 0x40aa2a0; continue;
 }
 0x40aa2a0 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa36c; continue; }
  pc = 0x40aa2a6; continue;
 }
 0x40aa2a6 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x40aa2be; continue; }
  pc = 0x40aa2ab; continue;
 }
 0x40aa2ab => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa38a; continue; }
  pc = 0x40aa2b1; continue;
 }
 0x40aa2b1 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x40aa399; continue; }
  pc = 0x40aa2ba; continue;
 }
 0x40aa2ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa2be => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x40aa37b; continue; }
  pc = 0x40aa2c7; continue;
 }
 0x40aa2c7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa2cb => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x40aa2e3; continue; }
  pc = 0x40aa2d0; continue;
 }
 0x40aa2d0 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa3b5; continue; }
  pc = 0x40aa2d6; continue;
 }
 0x40aa2d6 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x40aa35d; continue; }
  pc = 0x40aa2df; continue;
 }
 0x40aa2df => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa2e3 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40aa34e; continue; }
  pc = 0x40aa2e8; continue;
 }
 0x40aa2e8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa2ec => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa2fa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa308 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa316 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa324 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa332 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa340 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa34e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa35d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa36c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa37b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa38a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa399 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa3a7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa3b5 => {
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
pub fn f_040aa3d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa3d0); }
 let mut pc: u32 = 0x40aa3d0;
 loop { match pc {
 0x40aa3d0 => {
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
  if cpu.cf { pc = 0x40aa409; continue; }
  pc = 0x40aa3fa; continue;
 }
 0x40aa3fa => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa416; continue; }
  pc = 0x40aa3fc; continue;
 }
 0x40aa3fc => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x40aa41b; continue; }
  pc = 0x40aa404; continue;
 }
 0x40aa404 => {
  pc = 0x40aa57a; continue;
 }
 0x40aa409 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x40aa42c; continue; }
  pc = 0x40aa411; continue;
 }
 0x40aa411 => {
  pc = 0x40aa57a; continue;
 }
 0x40aa416 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa41b => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa42c => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x40aa490; continue; }
  pc = 0x40aa431; continue;
 }
 0x40aa431 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa567; continue; }
  pc = 0x40aa437; continue;
 }
 0x40aa437 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x40aa46f; continue; }
  pc = 0x40aa43c; continue;
 }
 0x40aa43c => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa508; continue; }
  pc = 0x40aa442; continue;
 }
 0x40aa442 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x40aa45e; continue; }
  pc = 0x40aa447; continue;
 }
 0x40aa447 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa51a; continue; }
  pc = 0x40aa44d; continue;
 }
 0x40aa44d => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x40aa523; continue; }
  pc = 0x40aa456; continue;
 }
 0x40aa456 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa45e => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x40aa511; continue; }
  pc = 0x40aa467; continue;
 }
 0x40aa467 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa46f => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x40aa47f; continue; }
  pc = 0x40aa474; continue;
 }
 0x40aa474 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa4ed; continue; }
  pc = 0x40aa47a; continue;
 }
 0x40aa47a => {
  pc = 0x40aa4f6; continue;
 }
 0x40aa47f => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x40aa4ff; continue; }
  pc = 0x40aa488; continue;
 }
 0x40aa488 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa490 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x40aa4c8; continue; }
  pc = 0x40aa495; continue;
 }
 0x40aa495 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa540; continue; }
  pc = 0x40aa49b; continue;
 }
 0x40aa49b => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x40aa4b7; continue; }
  pc = 0x40aa4a0; continue;
 }
 0x40aa4a0 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa554; continue; }
  pc = 0x40aa4a6; continue;
 }
 0x40aa4a6 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x40aa55e; continue; }
  pc = 0x40aa4af; continue;
 }
 0x40aa4af => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4b7 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x40aa54a; continue; }
  pc = 0x40aa4c0; continue;
 }
 0x40aa4c0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4c8 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x40aa4e0; continue; }
  pc = 0x40aa4cd; continue;
 }
 0x40aa4cd => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa570; continue; }
  pc = 0x40aa4d3; continue;
 }
 0x40aa4d3 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x40aa536; continue; }
  pc = 0x40aa4d8; continue;
 }
 0x40aa4d8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4e0 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40aa52c; continue; }
  pc = 0x40aa4e5; continue;
 }
 0x40aa4e5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4ed => {
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4f6 => {
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa4ff => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa508 => {
  { let v: u32 = cpu.r32(0x180u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa511 => {
  { let v: u32 = cpu.r32(0x1c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa51a => {
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa523 => {
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa52c => {
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa536 => {
  { let v: u32 = cpu.r32(0x2c0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa540 => {
  { let v: u32 = cpu.r32(0x300u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa54a => {
  { let v: u32 = cpu.r32(0x340u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa554 => {
  { let v: u32 = cpu.r32(0x380u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa55e => {
  { let v: u32 = cpu.r32(0x3c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa567 => {
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa570 => {
  { let v: u32 = cpu.r32(0x440u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa57a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aa590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa590); }
 let mut pc: u32 = 0x40aa590;
 loop { match pc {
 0x40aa590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x40aa5a1; continue;
 }
 0x40aa5a1 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x414b310u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aa5b9; continue; }
  pc = 0x40aa5b7; continue;
 }
 0x40aa5b7 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aa5b9; continue;
 }
 0x40aa5b9 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40aa5db; continue; }
  pc = 0x40aa5bd; continue;
 }
 0x40aa5bd => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b318u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aa5d3; continue; }
  pc = 0x40aa5d1; continue;
 }
 0x40aa5d1 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aa5d3; continue;
 }
 0x40aa5d3 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40aa6ce; continue; }
  pc = 0x40aa5db; continue;
 }
 0x40aa5db => {
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
  { let v: u32 = 0x414b320u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aa627; continue; }
  pc = 0x40aa625; continue;
 }
 0x40aa625 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aa627; continue;
 }
 0x40aa627 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40aa5a1; continue; }
  pc = 0x40aa62f; continue;
 }
 0x40aa62f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b310u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aa645; continue; }
  pc = 0x40aa643; continue;
 }
 0x40aa643 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aa645; continue;
 }
 0x40aa645 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40aa658; continue; }
  pc = 0x40aa649; continue;
 }
 0x40aa649 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40aa6ce; continue; }
  pc = 0x40aa651; continue;
 }
 0x40aa651 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40aa6d0; continue;
 }
 0x40aa658 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x40aa6ce; continue; }
  pc = 0x40aa665; continue;
 }
 0x40aa665 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b320u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aa67c; continue; }
  pc = 0x40aa67a; continue;
 }
 0x40aa67a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aa67c; continue;
 }
 0x40aa67c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40aa690; continue; }
  pc = 0x40aa680; continue;
 }
 0x40aa680 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40aa690; continue; }
  pc = 0x40aa686; continue;
 }
 0x40aa686 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aa690 => {
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
  if cpu.cf { cpu.poll(); pc = 0x40aa665; continue; }
  pc = 0x40aa6ce; continue;
 }
 0x40aa6ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40aa6d0; continue;
 }
 0x40aa6d0 => {
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
pub fn f_040aa6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa6e0); }
 let mut pc: u32 = 0x40aa6e0;
 loop { match pc {
 0x40aa6e0 => {
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
pub fn f_040aa720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa720); }
 let mut pc: u32 = 0x40aa720;
 loop { match pc {
 0x40aa720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x40aa72e; continue;
 }
 0x40aa72e => {
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
  if cpu.zf { pc = 0x40aa749; continue; }
  pc = 0x40aa746; continue;
 }
 0x40aa746 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40aa72e; continue; }
  pc = 0x40aa749; continue;
 }
 0x40aa749 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aa750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa750); }
 let mut pc: u32 = 0x40aa750;
 loop { match pc {
 0x40aa750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40aa7a0; continue; }
  pc = 0x40aa768; continue;
 }
 0x40aa768 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40aa789); f_0409cb30(cpu); if cpu.returned_to != 0x40aa789 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40aa7a0; continue;
 }
 0x40aa7a0 => {
  { cpu.sub32(cpu.edi, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40aa7af; continue; }
  pc = 0x40aa7a8; continue;
 }
 0x40aa7a8 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x40aa7b1; continue;
 }
 0x40aa7af => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x40aa7b1; continue;
 }
 0x40aa7b1 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40aa815; continue; }
  pc = 0x40aa7dd; continue;
 }
 0x40aa7dd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40aa7fe); f_0409cb30(cpu); if cpu.returned_to != 0x40aa7fe { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40aa815; continue;
 }
 0x40aa815 => {
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
pub fn f_040aa820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa820); }
 let mut pc: u32 = 0x40aa820;
 loop { match pc {
 0x40aa820 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c5f2cu32); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40aa860; continue; }
  pc = 0x40aa842; continue;
 }
 0x40aa842 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x40aa85b; continue; }
  pc = 0x40aa850; continue;
 }
 0x40aa850 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if !cpu.zf { pc = 0x40aa860; continue; }
  pc = 0x40aa857; continue;
 }
 0x40aa857 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40aa860; continue; }
  pc = 0x40aa85b; continue;
 }
 0x40aa85b => {
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x40aa860; continue;
 }
 0x40aa860 => {
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  pc = 0x40aa878; continue;
 }
 0x40aa878 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40aa992; continue; }
  pc = 0x40aa882; continue;
 }
 0x40aa882 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.cf { pc = 0x40aa8aa; continue; }
  pc = 0x40aa88a; continue;
 }
 0x40aa88a => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa8c6; continue; }
  pc = 0x40aa88c; continue;
 }
 0x40aa88c => {
  { cpu.sub32(cpu.esi, 0xd0u32); }
  if cpu.cf { pc = 0x40aa8a0; continue; }
  pc = 0x40aa894; continue;
 }
 0x40aa894 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa8cd; continue; }
  pc = 0x40aa896; continue;
 }
 0x40aa896 => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x40aa8c6; continue; }
  pc = 0x40aa89e; continue;
 }
 0x40aa89e => {
  pc = 0x40aa8d4; continue;
 }
 0x40aa8a0 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.zf { pc = 0x40aa8cd; continue; }
  pc = 0x40aa8a8; continue;
 }
 0x40aa8a8 => {
  pc = 0x40aa8d4; continue;
 }
 0x40aa8aa => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.cf { pc = 0x40aa8be; continue; }
  pc = 0x40aa8b2; continue;
 }
 0x40aa8b2 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aa8c6; continue; }
  pc = 0x40aa8b4; continue;
 }
 0x40aa8b4 => {
  { cpu.sub32(cpu.esi, 0xa0u32); }
  if cpu.zf { pc = 0x40aa8c6; continue; }
  pc = 0x40aa8bc; continue;
 }
 0x40aa8bc => {
  pc = 0x40aa8d4; continue;
 }
 0x40aa8be => {
  { cpu.sub32(cpu.esi, 0x80u32); }
  if !cpu.zf { pc = 0x40aa8d4; continue; }
  pc = 0x40aa8c6; continue;
 }
 0x40aa8c6 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x40aa8d6; continue;
 }
 0x40aa8cd => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x40aa8d6; continue;
 }
 0x40aa8d4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40aa8d6; continue;
 }
 0x40aa8d6 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40aa928; continue; }
  pc = 0x40aa8e7; continue;
 }
 0x40aa8e7 => {
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40aa928; continue; }
  pc = 0x40aa8f0; continue;
 }
 0x40aa8f0 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40aa911); f_0409cb30(cpu); if cpu.returned_to != 0x40aa911 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40aa928; continue;
 }
 0x40aa928 => {
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
  if !cpu.zf { pc = 0x40aa987; continue; }
  pc = 0x40aa969; continue;
 }
 0x40aa969 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  pc = 0x40aa987; continue;
 }
 0x40aa987 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx))); cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x40aa878; continue;
 }
 0x40aa992 => {
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
pub fn f_040aa9a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa9a0); }
 let mut pc: u32 = 0x40aa9a0;
 loop { match pc {
 0x40aa9a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40aa9e9; continue; }
  pc = 0x40aa9b1; continue;
 }
 0x40aa9b1 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40aa9d2); f_0409cb30(cpu); if cpu.returned_to != 0x40aa9d2 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40aa9e9; continue;
 }
 0x40aa9e9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aa9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aa9f0); }
 let mut pc: u32 = 0x40aa9f0;
 loop { match pc {
 0x40aa9f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x40aaa1f; continue; }
  pc = 0x40aaa00; continue;
 }
 0x40aaa00 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aaa39; continue; }
  pc = 0x40aaa02; continue;
 }
 0x40aaa02 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x40aaa15; continue; }
  pc = 0x40aaa09; continue;
 }
 0x40aaa09 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aaa3f; continue; }
  pc = 0x40aaa0b; continue;
 }
 0x40aaa0b => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x40aaa39; continue; }
  pc = 0x40aaa12; continue;
 }
 0x40aaa12 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaa15 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x40aaa3f; continue; }
  pc = 0x40aaa1c; continue;
 }
 0x40aaa1c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaa1f => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x40aaa32; continue; }
  pc = 0x40aaa26; continue;
 }
 0x40aaa26 => {
  if (cpu.cf || cpu.zf) { pc = 0x40aaa39; continue; }
  pc = 0x40aaa28; continue;
 }
 0x40aaa28 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x40aaa39; continue; }
  pc = 0x40aaa2f; continue;
 }
 0x40aaa2f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaa32 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x40aaa45; continue; }
  pc = 0x40aaa39; continue;
 }
 0x40aaa39 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaa3f => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aaa45 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aaa50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aaa50); }
 let mut pc: u32 = 0x40aaa50;
 loop { match pc {
 0x40aaa50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c700u32), 0x0u32); }
  if !cpu.zf { pc = 0x40aab61; continue; }
  pc = 0x40aaa60; continue;
 }
 0x40aaa60 => {
  { let v = 0x40a7270u32; cpu.push(v); }
  { let v = 0x40aaa50u32; cpu.push(v); }
  cpu.push(0x40aaa6f); f_040a30c0(cpu); if cpu.returned_to != 0x40aaa6f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x415c67cu32; cpu.push(v); }
  cpu.push(0x40aaa81); f_040a3000(cpu); if cpu.returned_to != 0x40aaa81 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c6fcu32; cpu.push(v); }
  cpu.push(0x40aaa90); f_040a3000(cpu); if cpu.returned_to != 0x40aaa90 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f9cu32; cpu.push(v); }
  cpu.push(0x40aaa9f); f_040a3000(cpu); if cpu.returned_to != 0x40aaa9f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fa0u32; cpu.push(v); }
  cpu.push(0x40aaaae); f_040a3000(cpu); if cpu.returned_to != 0x40aaaae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fa4u32; cpu.push(v); }
  cpu.push(0x40aaabd); f_040a3000(cpu); if cpu.returned_to != 0x40aaabd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fa8u32; cpu.push(v); }
  cpu.push(0x40aaacc); f_040a3000(cpu); if cpu.returned_to != 0x40aaacc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb8u32; cpu.push(v); }
  cpu.push(0x40aaadb); f_040a3000(cpu); if cpu.returned_to != 0x40aaadb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5facu32; cpu.push(v); }
  cpu.push(0x40aaaea); f_040a3000(cpu); if cpu.returned_to != 0x40aaaea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb0u32; cpu.push(v); }
  cpu.push(0x40aaaf9); f_040a3000(cpu); if cpu.returned_to != 0x40aaaf9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f98u32; cpu.push(v); }
  cpu.push(0x40aab08); f_040a3000(cpu); if cpu.returned_to != 0x40aab08 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5fb4u32; cpu.push(v); }
  cpu.push(0x40aab17); f_040a3000(cpu); if cpu.returned_to != 0x40aab17 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f94u32; cpu.push(v); }
  cpu.push(0x40aab26); f_040a3000(cpu); if cpu.returned_to != 0x40aab26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f90u32; cpu.push(v); }
  cpu.push(0x40aab35); f_040a3000(cpu); if cpu.returned_to != 0x40aab35 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f88u32; cpu.push(v); }
  cpu.push(0x40aab44); f_040a3000(cpu); if cpu.returned_to != 0x40aab44 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5f8cu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40aab58); f_040a3000(cpu); if cpu.returned_to != 0x40aab58 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c700u32, v); }
  pc = 0x40aab61; continue;
 }
 0x40aab61 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aab70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40aab70); } r_040aab70(cpu, 0x40aab70); }
pub fn f_040aad70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40aad70); } r_040aab70(cpu, 0x40aad70); }
fn r_040aab70(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40aab70 => {
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
  if cpu.zf { pc = 0x40aabfa; continue; }
  pc = 0x40aabfa; continue;
 }
 0x40aabfa => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.pf() { pc = 0x40aabfe; continue; }
  pc = 0x40aabfe; continue;
 }
 0x40aabfe => {
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
  cpu.unimplemented(0x40aac6c, "Outsb");
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
  if (cpu.sf == cpu.of) { pc = 0x40aacb8; continue; }
  pc = 0x40aacb2; continue;
 }
 0x40aacb2 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x40aacbc; continue; }
  pc = 0x40aacb6; continue;
 }
 0x40aacb6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40aacb8; continue;
 }
 0x40aacb8 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40aacbc; continue;
 }
 0x40aacbc => {
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
  cpu.unimplemented(0x40aacec, "Outsd");
  cpu.unimplemented(0x40aaced, "Sldt");
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
  pc = 0x40aad78; continue;
 }
 0x40aad70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c710u32), 0x0u32); }
  if cpu.zf { pc = 0x40aad96; continue; }
  pc = 0x40aad7c; continue;
 }
 0x40aad78 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x40aad96; continue; }
  pc = 0x40aad7c; continue;
 }
 0x40aad7c => {
  { let v = 0x40aad70u32; cpu.push(v); }
  { let v = 0x40ab7e0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40aad8d); f_040a3020(cpu); if cpu.returned_to != 0x40aad8d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c710u32, v); }
  pc = 0x40aad96; continue;
 }
 0x40aad96 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040aada0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40aada0); }
 let mut pc: u32 = 0x40aada0;
 loop { match pc {
 0x40aada0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40aadfe; continue; }
  pc = 0x40aadac; continue;
 }
 0x40aadac => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40aadd7; continue; }
  pc = 0x40aadb8; continue;
 }
 0x40aadb8 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x40aadba; continue;
 }
 0x40aadba => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x40aadca); f_0409b630(cpu); if cpu.returned_to != 0x40aadca { return; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40aadba; continue; }
  pc = 0x40aadd7; continue;
 }
 0x40aadd7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40aade3); f_04097420(cpu); if cpu.returned_to != 0x40aade3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40aadf2); f_04097310(cpu); if cpu.returned_to != 0x40aadf2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40aadfb); f_040a3230(cpu); if cpu.returned_to != 0x40aadfb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40aadfe; continue;
 }
 0x40aadfe => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab010); }
 let mut pc: u32 = 0x40ab010;
 loop { match pc {
 0x40ab010 => {
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
  if cpu.zf { pc = 0x40ab046; continue; }
  pc = 0x40ab036; continue;
 }
 0x40ab036 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.zf { pc = 0x40ab046; continue; }
  pc = 0x40ab03e; continue;
 }
 0x40ab03e => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if !cpu.zf { pc = 0x40ab05d; continue; }
  pc = 0x40ab046; continue;
 }
 0x40ab046 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab05a); f_040ab610(cpu); if cpu.returned_to != 0x40ab05a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x40ab05d; continue;
 }
 0x40ab05d => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if !cpu.zf { pc = 0x40ab071; continue; }
  pc = 0x40ab065; continue;
 }
 0x40ab065 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40ab071; continue; }
  pc = 0x40ab06c; continue;
 }
 0x40ab06c => {
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x40ab071; continue;
 }
 0x40ab071 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.cf { pc = 0x40ab0a7; continue; }
  pc = 0x40ab082; continue;
 }
 0x40ab082 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab0ca; continue; }
  pc = 0x40ab084; continue;
 }
 0x40ab084 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.cf { pc = 0x40ab428; continue; }
  pc = 0x40ab090; continue;
 }
 0x40ab090 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab1e8; continue; }
  pc = 0x40ab096; continue;
 }
 0x40ab096 => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if cpu.zf { pc = 0x40ab208; continue; }
  pc = 0x40ab0a2; continue;
 }
 0x40ab0a2 => {
  pc = 0x40ab428; continue;
 }
 0x40ab0a7 => {
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x40ab428; continue; }
  pc = 0x40ab0b3; continue;
 }
 0x40ab0b3 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab3c5; continue; }
  pc = 0x40ab0b9; continue;
 }
 0x40ab0b9 => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if cpu.zf { pc = 0x40ab253; continue; }
  pc = 0x40ab0c5; continue;
 }
 0x40ab0c5 => {
  pc = 0x40ab428; continue;
 }
 0x40ab0ca => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7u32); }
  if cpu.cf { pc = 0x40ab0f4; continue; }
  pc = 0x40ab0d3; continue;
 }
 0x40ab0d3 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab101; continue; }
  pc = 0x40ab0d5; continue;
 }
 0x40ab0d5 => {
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x40ab428; continue; }
  pc = 0x40ab0de; continue;
 }
 0x40ab0de => {
  { cpu.sub32(cpu.edx, 0xbu32); }
  if (cpu.cf || cpu.zf) { pc = 0x40ab101; continue; }
  pc = 0x40ab0e3; continue;
 }
 0x40ab0e3 => {
  { cpu.sub32(cpu.edx, 0x7bu32); }
  if cpu.zf { pc = 0x40ab197; continue; }
  pc = 0x40ab0ec; continue;
 }
 0x40ab0ec => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40ab0f4 => {
  { cpu.sub32(cpu.edx, 0x6u32); }
  if cpu.zf { pc = 0x40ab14c; continue; }
  pc = 0x40ab0f9; continue;
 }
 0x40ab0f9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40ab101 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab111; continue;
 }
 0x40ab111 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x40ab113; continue;
 }
 0x40ab113 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x40ab138; continue; }
  pc = 0x40ab11b; continue;
 }
 0x40ab11b => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab127); f_0409ab30(cpu); if cpu.returned_to != 0x40ab127 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40ab138; continue; }
  pc = 0x40ab12f; continue;
 }
 0x40ab12f => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ab138); f_040ab570(cpu); if cpu.returned_to != 0x40ab138 { return; }
  pc = 0x40ab138; continue;
 }
 0x40ab138 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab14a; continue;
 }
 0x40ab14a => {
  cpu.poll(); pc = 0x40ab113; continue;
 }
 0x40ab14c => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab15c; continue;
 }
 0x40ab15c => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x40ab15e; continue;
 }
 0x40ab15e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x40ab183; continue; }
  pc = 0x40ab166; continue;
 }
 0x40ab166 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab172); f_0409ab30(cpu); if cpu.returned_to != 0x40ab172 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40ab183; continue; }
  pc = 0x40ab17a; continue;
 }
 0x40ab17a => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ab183); f_040ab460(cpu); if cpu.returned_to != 0x40ab183 { return; }
  pc = 0x40ab183; continue;
 }
 0x40ab183 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab195; continue;
 }
 0x40ab195 => {
  cpu.poll(); pc = 0x40ab15e; continue;
 }
 0x40ab197 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab1a7; continue;
 }
 0x40ab1a7 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40ab1a9; continue;
 }
 0x40ab1a9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x40ab1d4; continue; }
  pc = 0x40ab1b1; continue;
 }
 0x40ab1b1 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab1bd); f_0409ab30(cpu); if cpu.returned_to != 0x40ab1bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40ab1d4; continue; }
  pc = 0x40ab1c5; continue;
 }
 0x40ab1c5 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab1d1); f_0409b000(cpu); if cpu.returned_to != 0x40ab1d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40ab1d4; continue;
 }
 0x40ab1d4 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab1e6; continue;
 }
 0x40ab1e6 => {
  cpu.poll(); pc = 0x40ab1a9; continue;
 }
 0x40ab1e8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x40ab1fa); f_040ab5e0(cpu); if cpu.returned_to != 0x40ab1fa { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40ab208 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab218; continue;
 }
 0x40ab218 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x40ab21a; continue;
 }
 0x40ab21a => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x40ab23f; continue; }
  pc = 0x40ab222; continue;
 }
 0x40ab222 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab22e); f_0409ab30(cpu); if cpu.returned_to != 0x40ab22e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40ab23f; continue; }
  pc = 0x40ab236; continue;
 }
 0x40ab236 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ab23f); f_040ab460(cpu); if cpu.returned_to != 0x40ab23f { return; }
  pc = 0x40ab23f; continue;
 }
 0x40ab23f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab251; continue;
 }
 0x40ab251 => {
  cpu.poll(); pc = 0x40ab21a; continue;
 }
 0x40ab253 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if !cpu.zf { pc = 0x40ab26f; continue; }
  pc = 0x40ab258; continue;
 }
 0x40ab258 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x40ab269); f_040ab5e0(cpu); if cpu.returned_to != 0x40ab269 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ab26f; continue;
 }
 0x40ab26f => {
  { cpu.sub32(cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x40ab428; continue; }
  pc = 0x40ab27d; continue;
 }
 0x40ab27d => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab2ad; continue; }
  pc = 0x40ab289; continue;
 }
 0x40ab289 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x40ab28b; continue;
 }
 0x40ab28b => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab297); f_0409ab30(cpu); if cpu.returned_to != 0x40ab297 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40ab2ad; continue; }
  pc = 0x40ab29f; continue;
 }
 0x40ab29f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40ab28b; continue; }
  pc = 0x40ab2ad; continue;
 }
 0x40ab2ad => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x40ab2f0; continue; }
  pc = 0x40ab2b5; continue;
 }
 0x40ab2b5 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab2e0; continue; }
  pc = 0x40ab2c2; continue;
 }
 0x40ab2c2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40ab2c4; continue;
 }
 0x40ab2c4 => {
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { pc = 0x40ab2d2; continue; }
  pc = 0x40ab2ce; continue;
 }
 0x40ab2ce => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x40ab2d2; continue;
 }
 0x40ab2d2 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40ab2c4; continue; }
  pc = 0x40ab2e0; continue;
 }
 0x40ab2e0 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ab2ed); f_0409b000(cpu); if cpu.returned_to != 0x40ab2ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40ab2f0; continue;
 }
 0x40ab2f0 => {
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
  cpu.push(0x40ab352); f_0409b580(cpu); if cpu.returned_to != 0x40ab352 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ab371); f_0409b2c0(cpu); if cpu.returned_to != 0x40ab371 { return; }
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
  cpu.push(0x40ab395); f_0409b390(cpu); if cpu.returned_to != 0x40ab395 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ab3a1); f_040ab460(cpu); if cpu.returned_to != 0x40ab3a1 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ab3aa); f_040ab570(cpu); if cpu.returned_to != 0x40ab3aa { return; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ab3b7); f_0409b210(cpu); if cpu.returned_to != 0x40ab3b7 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40ab3c5 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x40ab428; continue; }
  pc = 0x40ab3ca; continue;
 }
 0x40ab3ca => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab428; continue; }
  pc = 0x40ab3d6; continue;
 }
 0x40ab3d6 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40ab3d8; continue;
 }
 0x40ab3d8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x40ab41a; continue; }
  pc = 0x40ab3e0; continue;
 }
 0x40ab3e0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x558u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x40ab41a; continue; }
  pc = 0x40ab3ec; continue;
 }
 0x40ab3ec => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab3f8); f_0409ab30(cpu); if cpu.returned_to != 0x40ab3f8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40ab41a; continue; }
  pc = 0x40ab400; continue;
 }
 0x40ab400 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab40c); f_0409b000(cpu); if cpu.returned_to != 0x40ab40c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x40ab41a => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40ab3d8; continue; }
  pc = 0x40ab428; continue;
 }
 0x40ab428 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab430); }
 let mut pc: u32 = 0x40ab430;
 loop { match pc {
 0x40ab430 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40ab44a); f_040ab5e0(cpu); if cpu.returned_to != 0x40ab44a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab460); }
 let mut pc: u32 = 0x40ab460;
 loop { match pc {
 0x40ab460 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab4b6; continue; }
  pc = 0x40ab4aa; continue;
 }
 0x40ab4aa => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1fffu32); cpu.eax = v; }
  pc = 0x40ab4cc; continue;
 }
 0x40ab4b6 => {
  if (cpu.sf == cpu.of) { pc = 0x40ab4c8; continue; }
  pc = 0x40ab4b8; continue;
 }
 0x40ab4b8 => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x40ab4cc; continue;
 }
 0x40ab4c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40ab4cc; continue;
 }
 0x40ab4cc => {
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40ab4de; continue; }
  pc = 0x40ab4dc; continue;
 }
 0x40ab4dc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40ab4de; continue;
 }
 0x40ab4de => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40ab4e8; continue; }
  pc = 0x40ab4e3; continue;
 }
 0x40ab4e3 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x40ab4e8; continue;
 }
 0x40ab4e8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x40aab70u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x40aab70u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x40aab70u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 13u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 13u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x40ab539; continue; }
  pc = 0x40ab522; continue;
 }
 0x40ab522 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40ab540; continue;
 }
 0x40ab539 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40ab540; continue;
 }
 0x40ab540 => {
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40ab559); f_0409af40(cpu); if cpu.returned_to != 0x40ab559 { return; }
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
pub fn f_040ab570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab570); }
 let mut pc: u32 = 0x40ab570;
 loop { match pc {
 0x40ab570 => {
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
  cpu.push(0x40ab5bd); f_0409ae80(cpu); if cpu.returned_to != 0x40ab5bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x314u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ab5d3); f_0409adc0(cpu); if cpu.returned_to != 0x40ab5d3 { return; }
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
pub fn f_040ab5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab5e0); }
 let mut pc: u32 = 0x40ab5e0;
 loop { match pc {
 0x40ab5e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40ab5e6; continue;
 }
 0x40ab5e6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40ab602; continue; }
  pc = 0x40ab5ed; continue;
 }
 0x40ab5ed => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x40ab5f6; continue; }
  pc = 0x40ab5f1; continue;
 }
 0x40ab5f1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40ab604; continue; }
  pc = 0x40ab5f6; continue;
 }
 0x40ab5f6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40ab5e6; continue; }
  pc = 0x40ab602; continue;
 }
 0x40ab602 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40ab604; continue;
 }
 0x40ab604 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab610); }
 let mut pc: u32 = 0x40ab610;
 loop { match pc {
 0x40ab610 => {
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
  if cpu.cf { pc = 0x40ab64d; continue; }
  pc = 0x40ab63f; continue;
 }
 0x40ab63f => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab659; continue; }
  pc = 0x40ab641; continue;
 }
 0x40ab641 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x40ab664; continue; }
  pc = 0x40ab649; continue;
 }
 0x40ab649 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab64d => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x40ab67a; continue; }
  pc = 0x40ab655; continue;
 }
 0x40ab655 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab659 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab664 => {
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
 0x40ab67a => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.cf { pc = 0x40ab6cb; continue; }
  pc = 0x40ab67f; continue;
 }
 0x40ab67f => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab7b1; continue; }
  pc = 0x40ab685; continue;
 }
 0x40ab685 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x40ab6b5; continue; }
  pc = 0x40ab68a; continue;
 }
 0x40ab68a => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab72f; continue; }
  pc = 0x40ab690; continue;
 }
 0x40ab690 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x40ab6a8; continue; }
  pc = 0x40ab695; continue;
 }
 0x40ab695 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab74b; continue; }
  pc = 0x40ab69b; continue;
 }
 0x40ab69b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x40ab759; continue; }
  pc = 0x40ab6a4; continue;
 }
 0x40ab6a4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab6a8 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x40ab73d; continue; }
  pc = 0x40ab6b1; continue;
 }
 0x40ab6b1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab6b5 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x40ab6be; continue; }
  pc = 0x40ab6ba; continue;
 }
 0x40ab6ba => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab713; continue; }
  pc = 0x40ab6bc; continue;
 }
 0x40ab6bc => {
  pc = 0x40ab721; continue;
 }
 0x40ab6be => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.zf { pc = 0x40ab7bf; continue; }
  pc = 0x40ab6c7; continue;
 }
 0x40ab6c7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab6cb => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.cf { pc = 0x40ab6fb; continue; }
  pc = 0x40ab6d0; continue;
 }
 0x40ab6d0 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab776; continue; }
  pc = 0x40ab6d6; continue;
 }
 0x40ab6d6 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.cf { pc = 0x40ab6ee; continue; }
  pc = 0x40ab6db; continue;
 }
 0x40ab6db => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab794; continue; }
  pc = 0x40ab6e1; continue;
 }
 0x40ab6e1 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x40ab7a3; continue; }
  pc = 0x40ab6ea; continue;
 }
 0x40ab6ea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab6ee => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x40ab785; continue; }
  pc = 0x40ab6f7; continue;
 }
 0x40ab6f7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab6fb => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40ab7d8; continue; }
  pc = 0x40ab704; continue;
 }
 0x40ab704 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ab767; continue; }
  pc = 0x40ab706; continue;
 }
 0x40ab706 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.zf { pc = 0x40ab7cd; continue; }
  pc = 0x40ab70f; continue;
 }
 0x40ab70f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab713 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab721 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab72f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab73d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab74b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab759 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab767 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab776 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab785 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab794 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab7a3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab7b1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab7bf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab7cd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  pc = 0x40ab7d8; continue;
 }
 0x40ab7d8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab7e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab7e0); }
 let mut pc: u32 = 0x40ab7e0;
 loop { match pc {
 0x40ab7e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c710u32), 0x0u32); }
  if !cpu.zf { pc = 0x40ab809; continue; }
  pc = 0x40ab7ec; continue;
 }
 0x40ab7ec => {
  { let v = 0x40aad70u32; cpu.push(v); }
  { let v = 0x40ab7e0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40ab800); f_040a30c0(cpu); if cpu.returned_to != 0x40ab800 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c710u32, v); }
  pc = 0x40ab809; continue;
 }
 0x40ab809 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab810); }
 let mut pc: u32 = 0x40ab810;
 loop { match pc {
 0x40ab810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ab81b); f_04096520(cpu); if cpu.returned_to != 0x40ab81b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x40ab83b; continue; }
  pc = 0x40ab823; continue;
 }
 0x40ab823 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40ab838); f_04095f10(cpu); if cpu.returned_to != 0x40ab838 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x40ab83b; continue;
 }
 0x40ab83b => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ab841); f_04096520(cpu); if cpu.returned_to != 0x40ab841 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab850); }
 let mut pc: u32 = 0x40ab850;
 loop { match pc {
 0x40ab850 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab85c); f_04096520(cpu); if cpu.returned_to != 0x40ab85c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40ab890; continue; }
  pc = 0x40ab864; continue;
 }
 0x40ab864 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab86a); f_04096130(cpu); if cpu.returned_to != 0x40ab86a { return; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40ab890); f_040ac420(cpu); if cpu.returned_to != 0x40ab890 { return; }
  pc = 0x40ab890; continue;
 }
 0x40ab890 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab896); f_04096520(cpu); if cpu.returned_to != 0x40ab896 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab8a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab8a0); }
 let mut pc: u32 = 0x40ab8a0;
 loop { match pc {
 0x40ab8a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab8ac); f_04096520(cpu); if cpu.returned_to != 0x40ab8ac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ab8ec; continue; }
  pc = 0x40ab8b3; continue;
 }
 0x40ab8b3 => {
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40ab8d3); f_040ac420(cpu); if cpu.returned_to != 0x40ab8d3 { return; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40ab8ec); f_040ac420(cpu); if cpu.returned_to != 0x40ab8ec { return; }
  pc = 0x40ab8ec; continue;
 }
 0x40ab8ec => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab8f2); f_04096520(cpu); if cpu.returned_to != 0x40ab8f2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ab900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ab900); }
 let mut pc: u32 = 0x40ab900;
 loop { match pc {
 0x40ab900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab912); f_04096520(cpu); if cpu.returned_to != 0x40ab912 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ab97f; continue; }
  pc = 0x40ab919; continue;
 }
 0x40ab919 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x40ab937; continue; }
  pc = 0x40ab91e; continue;
 }
 0x40ab91e => {
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40ab937); f_040ac420(cpu); if cpu.returned_to != 0x40ab937 { return; }
  pc = 0x40ab937; continue;
 }
 0x40ab937 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if cpu.zf { pc = 0x40ab97f; continue; }
  pc = 0x40ab93f; continue;
 }
 0x40ab93f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x84u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40ab963); f_040ac2b0(cpu); if cpu.returned_to != 0x40ab963 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40ab96d); f_040ac2b0(cpu); if cpu.returned_to != 0x40ab96d { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40ab97f); f_040ac420(cpu); if cpu.returned_to != 0x40ab97f { return; }
  pc = 0x40ab97f; continue;
 }
 0x40ab97f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ab985); f_04096520(cpu); if cpu.returned_to != 0x40ab985 { return; }
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
pub fn f_040ab990(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ab990); } r_040ab990(cpu, 0x40ab990); }
pub fn f_040ab9c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40ab9c0); } r_040ab990(cpu, 0x40ab9c0); }
fn r_040ab990(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40ab990 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab99a); f_04096520(cpu); if cpu.returned_to != 0x40ab99a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40ab9b2; continue; }
  pc = 0x40ab9a2; continue;
 }
 0x40ab9a2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40abe50; continue;
 }
 0x40ab9b2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ab9c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ab9ca); f_04096520(cpu); if cpu.returned_to != 0x40ab9ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40ab9df; continue; }
  pc = 0x40ab9d2; continue;
 }
 0x40ab9d2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40abe50; continue;
 }
 0x40ab9df => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abe50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u16 = 0xbu16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40abea9); f_040ac420(cpu); if cpu.returned_to != 0x40abea9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40abf42; continue; }
  pc = 0x40abeb1; continue;
 }
 0x40abeb1 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40abf16; continue; }
  pc = 0x40abeb5; continue;
 }
 0x40abeb5 => {
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
  pc = 0x40abf42; continue;
 }
 0x40abf16 => {
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
  pc = 0x40abf42; continue;
 }
 0x40abf42 => {
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
pub fn f_040abaa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abaa0); }
 let mut pc: u32 = 0x40abaa0;
 loop { match pc {
 0x40abaa0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ababa); f_04096520(cpu); if cpu.returned_to != 0x40ababa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40abacb; continue; }
  pc = 0x40abac1; continue;
 }
 0x40abac1 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.cf { pc = 0x40abacb; continue; }
  pc = 0x40abac6; continue;
 }
 0x40abac6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x40abae5; continue; }
  pc = 0x40abacb; continue;
 }
 0x40abacb => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40abad5; continue; }
  pc = 0x40abacf; continue;
 }
 0x40abacf => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40abad5; continue;
 }
 0x40abad5 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40abafb; continue; }
  pc = 0x40abad9; continue;
 }
 0x40abad9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abae5 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40abaef; continue; }
  pc = 0x40abae9; continue;
 }
 0x40abae9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40abaef; continue;
 }
 0x40abaef => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40abafb; continue; }
  pc = 0x40abaf3; continue;
 }
 0x40abaf3 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40abafb; continue;
 }
 0x40abafb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abb00); }
 let mut pc: u32 = 0x40abb00;
 loop { match pc {
 0x40abb00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40abb0a); f_04096520(cpu); if cpu.returned_to != 0x40abb0a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40abb12; continue; }
  pc = 0x40abb11; continue;
 }
 0x40abb11 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abb12 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abb20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abb20); }
 let mut pc: u32 = 0x40abb20;
 loop { match pc {
 0x40abb20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40abb79; continue; }
  pc = 0x40abb29; continue;
 }
 0x40abb29 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x40abb30); f_040abc90(cpu); if cpu.returned_to != 0x40abb30 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40abb6b; continue; }
  pc = 0x40abb36; continue;
 }
 0x40abb36 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40abb51; continue; }
  pc = 0x40abb3c; continue;
 }
 0x40abb3c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x40abb46); f_040ac150(cpu); if cpu.returned_to != 0x40abb46 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40abb6b; continue; }
  pc = 0x40abb4a; continue;
 }
 0x40abb4a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40abb51; continue;
 }
 0x40abb51 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x40abb5a; continue; }
  pc = 0x40abb56; continue;
 }
 0x40abb56 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abb5a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40abb6b => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40abb79; continue;
 }
 0x40abb79 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abb80); }
 let mut pc: u32 = 0x40abb80;
 loop { match pc {
 0x40abb80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40abb9c; continue; }
  pc = 0x40abb89; continue;
 }
 0x40abb89 => {
  { let v: u32 = 0x5u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x40abb95); f_040abf50(cpu); if cpu.returned_to != 0x40abb95 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40abb9c; continue;
 }
 0x40abb9c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abbc0); }
 let mut pc: u32 = 0x40abbc0;
 loop { match pc {
 0x40abbc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40abc0a; continue; }
  pc = 0x40abbca; continue;
 }
 0x40abbca => {
  { let v: u32 = cpu.r32(0x415c714u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40abbfd; continue; }
  pc = 0x40abbd4; continue;
 }
 0x40abbd4 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415c714u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40abbfd; continue; }
  pc = 0x40abbe1; continue;
 }
 0x40abbe1 => {
  { let v: u32 = cpu.r32(0x42c61c4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40abbfa); f_040a3160(cpu); if cpu.returned_to != 0x40abbfa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x40abbfd; continue;
 }
 0x40abbfd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40abc07); f_040a3230(cpu); if cpu.returned_to != 0x40abc07 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40abc0a; continue;
 }
 0x40abc0a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abc10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abc10); }
 let mut pc: u32 = 0x40abc10;
 loop { match pc {
 0x40abc10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0x1a8u32; cpu.push(v); }
  cpu.push(0x40abc22); f_040a3260(cpu); if cpu.returned_to != 0x40abc22 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40abc7a; continue; }
  pc = 0x40abc2b; continue;
 }
 0x40abc2b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x40abc32); f_040ac000(cpu); if cpu.returned_to != 0x40abc32 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if !cpu.zf { pc = 0x40abc48; continue; }
  pc = 0x40abc3b; continue;
 }
 0x40abc3b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40abc41); f_040a3230(cpu); if cpu.returned_to != 0x40abc41 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x40abc7a; continue;
 }
 0x40abc48 => {
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
  pc = 0x40abc7a; continue;
 }
 0x40abc7a => {
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
pub fn f_040abc90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abc90); }
 let mut pc: u32 = 0x40abc90;
 loop { match pc {
 0x40abc90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u16 = 0x5u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x40abce1); f_040ac420(cpu); if cpu.returned_to != 0x40abce1 { return; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x40abcef; continue; }
  pc = 0x40abce9; continue;
 }
 0x40abce9 => {
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x40abcf3; continue; }
  pc = 0x40abcef; continue;
 }
 0x40abcef => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40abcff; continue;
 }
 0x40abcf3 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40abcff; continue;
 }
 0x40abcff => {
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
pub fn f_040abd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abd10); }
 let mut pc: u32 = 0x40abd10;
 loop { match pc {
 0x40abd10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40abd2a); f_040ac420(cpu); if cpu.returned_to != 0x40abd2a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040abf50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40abf50); }
 let mut pc: u32 = 0x40abf50;
 loop { match pc {
 0x40abf50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40abfa5); f_040ac420(cpu); if cpu.returned_to != 0x40abfa5 { return; }
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
pub fn f_040ac000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac000); }
 let mut pc: u32 = 0x40ac000;
 loop { match pc {
 0x40ac000 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40ac079); cpu.call(t); if cpu.returned_to != 0x40ac079 { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x40ac089; continue; }
  pc = 0x40ac07f; continue;
 }
 0x40ac07f => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x40ac13c; continue;
 }
 0x40ac089 => {
  { cpu.sub32(cpu.r32(0x415c714u32), 0x0u32); }
  if !cpu.zf { pc = 0x40ac0c7; continue; }
  pc = 0x40ac092; continue;
 }
 0x40ac092 => {
  { let v = 0x42c61c4u32; cpu.push(v); }
  { let v = 0x42c61bcu32; cpu.push(v); }
  { let v = 0x42c61c0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x40ac0a8); f_040a31a0(cpu); if cpu.returned_to != 0x40ac0a8 { return; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40ac0c7; continue; }
  pc = 0x40ac0b5; continue;
 }
 0x40ac0b5 => {
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
 0x40ac0c7 => {
  { let v: u32 = 0x150du32; cpu.ecx = v; }
  { let v: u32 = 0x300u32; cpu.ebp = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.eax = v; }
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40ac122); cpu.call(t); if cpu.returned_to != 0x40ac122 { return; }
  { let v: u32 = cpu.r32(0x415c714u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c714u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40ac13c; continue;
 }
 0x40ac13c => {
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
pub fn f_040ac150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac150); }
 let mut pc: u32 = 0x40ac150;
 loop { match pc {
 0x40ac150 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c61c0u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x21u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x40ac1b1); f_040ac420(cpu); if cpu.returned_to != 0x40ac1b1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac273; continue; }
  pc = 0x40ac1b9; continue;
 }
 0x40ac1b9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40ac240; continue;
 }
 0x40ac1e1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x42c61bcu32); cpu.ebx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x40ac213); f_040ac420(cpu); if cpu.returned_to != 0x40ac213 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac273; continue; }
  pc = 0x40ac217; continue;
 }
 0x40ac217 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40ac229); f_040ac380(cpu); if cpu.returned_to != 0x40ac229 { return; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40ac240; continue;
 }
 0x40ac240 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40ac1e1; continue; }
  pc = 0x40ac24d; continue;
 }
 0x40ac24d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x40ac262); f_040ac380(cpu); if cpu.returned_to != 0x40ac262 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40ac275; continue;
 }
 0x40ac273 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40ac275; continue;
 }
 0x40ac275 => {
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
pub fn f_040ac280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac280); }
 let mut pc: u32 = 0x40ac280;
 loop { match pc {
 0x40ac280 => {
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
pub fn f_040ac2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac2b0); }
 let mut pc: u32 = 0x40ac2b0;
 loop { match pc {
 0x40ac2b0 => {
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
pub fn f_040ac320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac320); }
 let mut pc: u32 = 0x40ac320;
 loop { match pc {
 0x40ac320 => {
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
pub fn f_040ac380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac380); }
 let mut pc: u32 = 0x40ac380;
 loop { match pc {
 0x40ac380 => {
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
pub fn f_040ac3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac3f0); }
 let mut pc: u32 = 0x40ac3f0;
 loop { match pc {
 0x40ac3f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ac411); f_040ac420(cpu); if cpu.returned_to != 0x40ac411 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac420); }
 let mut pc: u32 = 0x40ac420;
 loop { match pc {
 0x40ac420 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40ac4c2); cpu.call(t); if cpu.returned_to != 0x40ac4c2 { return; }
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
pub fn f_040ac4e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac4e0); }
 let mut pc: u32 = 0x40ac4e0;
 loop { match pc {
 0x40ac4e0 => {
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
  let t = cpu.r32(0x415b914u32); cpu.push(0x40ac51a); cpu.call(t); if cpu.returned_to != 0x40ac51a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac520); }
 let mut pc: u32 = 0x40ac520;
 loop { match pc {
 0x40ac520 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c72cu32), 0x0u32); }
  if cpu.zf { pc = 0x40ac546; continue; }
  pc = 0x40ac52c; continue;
 }
 0x40ac52c => {
  { let v = 0x40ac520u32; cpu.push(v); }
  { let v = 0x40acc80u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40ac53d); f_040a3020(cpu); if cpu.returned_to != 0x40ac53d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c72cu32, v); }
  pc = 0x40ac546; continue;
 }
 0x40ac546 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac550); }
 let mut pc: u32 = 0x40ac550;
 loop { match pc {
 0x40ac550 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ac55e; continue; }
  pc = 0x40ac558; continue;
 }
 0x40ac558 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac55e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac570); }
 let mut pc: u32 = 0x40ac570;
 loop { match pc {
 0x40ac570 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac57c); f_04095a90(cpu); if cpu.returned_to != 0x40ac57c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac5f1; continue; }
  pc = 0x40ac58b; continue;
 }
 0x40ac58b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac596); f_040958d0(cpu); if cpu.returned_to != 0x40ac596 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ac5ab; continue; }
  pc = 0x40ac59d; continue;
 }
 0x40ac59d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40ac5a3); f_040959e0(cpu); if cpu.returned_to != 0x40ac5a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac5ab => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40ac5ef; continue; }
  pc = 0x40ac5b2; continue;
 }
 0x40ac5b2 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40ac5ef; continue; }
  pc = 0x40ac5b9; continue;
 }
 0x40ac5b9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40ac5ef; continue; }
  pc = 0x40ac5c1; continue;
 }
 0x40ac5c1 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ac5c8; continue; }
  pc = 0x40ac5c3; continue;
 }
 0x40ac5c3 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  pc = 0x40ac5ed; continue;
 }
 0x40ac5c8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac5d1); f_04098a10(cpu); if cpu.returned_to != 0x40ac5d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { cpu.poll(); pc = 0x40ac5c8; continue; }
  pc = 0x40ac5d9; continue;
 }
 0x40ac5d9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac5de => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac5e7); f_0409ab30(cpu); if cpu.returned_to != 0x40ac5e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  pc = 0x40ac5ed; continue;
 }
 0x40ac5ed => {
  if cpu.zf { cpu.poll(); pc = 0x40ac5de; continue; }
  pc = 0x40ac5ef; continue;
 }
 0x40ac5ef => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40ac5f1; continue;
 }
 0x40ac5f1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac600); }
 let mut pc: u32 = 0x40ac600;
 loop { match pc {
 0x40ac600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40ac643; continue; }
  pc = 0x40ac609; continue;
 }
 0x40ac609 => {
  cpu.push(0x40ac60e); f_0409cfd0(cpu); if cpu.returned_to != 0x40ac60e { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40ac63e; continue; }
  pc = 0x40ac615; continue;
 }
 0x40ac615 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40ac637; continue; }
  pc = 0x40ac61d; continue;
 }
 0x40ac61d => {
  if (cpu.cf || cpu.zf) { pc = 0x40ac626; continue; }
  pc = 0x40ac61f; continue;
 }
 0x40ac61f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40ac62e; continue; }
  pc = 0x40ac624; continue;
 }
 0x40ac624 => {
  pc = 0x40ac637; continue;
 }
 0x40ac626 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac62c); f_04098d70(cpu); if cpu.returned_to != 0x40ac62c { return; }
  pc = 0x40ac634; continue;
 }
 0x40ac62e => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac634); f_0409b000(cpu); if cpu.returned_to != 0x40ac634 { return; }
  pc = 0x40ac634; continue;
 }
 0x40ac634 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40ac637; continue;
 }
 0x40ac637 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40ac63e; continue;
 }
 0x40ac63e => {
  cpu.push(0x40ac643); f_0409cfc0(cpu); if cpu.returned_to != 0x40ac643 { return; }
  pc = 0x40ac643; continue;
 }
 0x40ac643 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac650); }
 let mut pc: u32 = 0x40ac650;
 loop { match pc {
 0x40ac650 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40ac7b2; continue; }
  pc = 0x40ac660; continue;
 }
 0x40ac660 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x40ac66f; continue; }
  pc = 0x40ac666; continue;
 }
 0x40ac666 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ac66c); f_04095820(cpu); if cpu.returned_to != 0x40ac66c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40ac66f; continue;
 }
 0x40ac66f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40ac7b2; continue; }
  pc = 0x40ac67b; continue;
 }
 0x40ac67b => {
  if (cpu.cf || cpu.zf) { pc = 0x40ac68b; continue; }
  pc = 0x40ac67d; continue;
 }
 0x40ac67d => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40ac719; continue; }
  pc = 0x40ac686; continue;
 }
 0x40ac686 => {
  pc = 0x40ac7b2; continue;
 }
 0x40ac68b => {
  { let v: u32 = cpu.r32(0x42c61ccu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac697); f_04099200(cpu); if cpu.returned_to != 0x40ac697 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac7b4; continue; }
  pc = 0x40ac6a5; continue;
 }
 0x40ac6a5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac6b0); f_04099030(cpu); if cpu.returned_to != 0x40ac6b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40ac6cb; continue; }
  pc = 0x40ac6b8; continue;
 }
 0x40ac6b8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40ac6c1); f_04099150(cpu); if cpu.returned_to != 0x40ac6c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac6cb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ac6d7); f_04097240(cpu); if cpu.returned_to != 0x40ac6d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x40acc40u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac6e8); f_04097640(cpu); if cpu.returned_to != 0x40ac6e8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40ac6f9); f_04098b10(cpu); if cpu.returned_to != 0x40ac6f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40ac70c); f_04098f80(cpu); if cpu.returned_to != 0x40ac70c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac719 => {
  { let v: u32 = cpu.r32(0x42c61c8u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40ac725); f_0409b800(cpu); if cpu.returned_to != 0x40ac725 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac7b4; continue; }
  pc = 0x40ac733; continue;
 }
 0x40ac733 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac739); f_0409b580(cpu); if cpu.returned_to != 0x40ac739 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac74a); f_0409b460(cpu); if cpu.returned_to != 0x40ac74a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ac764; continue; }
  pc = 0x40ac751; continue;
 }
 0x40ac751 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40ac75a); f_0409b630(cpu); if cpu.returned_to != 0x40ac75a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac764 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac770); f_040997d0(cpu); if cpu.returned_to != 0x40ac770 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x40acc00u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40ac781); f_040999b0(cpu); if cpu.returned_to != 0x40ac781 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40ac792); f_0409ad00(cpu); if cpu.returned_to != 0x40ac792 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40ac7a5); f_0409b210(cpu); if cpu.returned_to != 0x40ac7a5 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac7b2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40ac7b4; continue;
 }
 0x40ac7b4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac7c0); }
 let mut pc: u32 = 0x40ac7c0;
 loop { match pc {
 0x40ac7c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40ac85d; continue; }
  pc = 0x40ac7ce; continue;
 }
 0x40ac7ce => {
  cpu.push(0x40ac7d3); f_0409cfd0(cpu); if cpu.returned_to != 0x40ac7d3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40ac803; continue; }
  pc = 0x40ac7da; continue;
 }
 0x40ac7da => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x40ac7fc; continue; }
  pc = 0x40ac7e2; continue;
 }
 0x40ac7e2 => {
  if (cpu.cf || cpu.zf) { pc = 0x40ac7eb; continue; }
  pc = 0x40ac7e4; continue;
 }
 0x40ac7e4 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40ac7f3; continue; }
  pc = 0x40ac7e9; continue;
 }
 0x40ac7e9 => {
  pc = 0x40ac7fc; continue;
 }
 0x40ac7eb => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac7f1); f_04098d70(cpu); if cpu.returned_to != 0x40ac7f1 { return; }
  pc = 0x40ac7f9; continue;
 }
 0x40ac7f3 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40ac7f9); f_0409b000(cpu); if cpu.returned_to != 0x40ac7f9 { return; }
  pc = 0x40ac7f9; continue;
 }
 0x40ac7f9 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40ac7fc; continue;
 }
 0x40ac7fc => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40ac803; continue;
 }
 0x40ac803 => {
  cpu.push(0x40ac808); f_0409cfc0(cpu); if cpu.returned_to != 0x40ac808 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40ac81d; continue; }
  pc = 0x40ac80e; continue;
 }
 0x40ac80e => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40ac814); f_040a3230(cpu); if cpu.returned_to != 0x40ac814 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40ac81d; continue;
 }
 0x40ac81d => {
  { let v: u32 = cpu.r32(0x415c728u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x40ac830; continue; }
  pc = 0x40ac826; continue;
 }
 0x40ac826 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c728u32, v); }
  pc = 0x40ac854; continue;
 }
 0x40ac830 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ac854; continue; }
  pc = 0x40ac834; continue;
 }
 0x40ac834 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x40ac84d; continue; }
  pc = 0x40ac83b; continue;
 }
 0x40ac83b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ac847); f_040a3230(cpu); if cpu.returned_to != 0x40ac847 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac84d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40ac834; continue; }
  pc = 0x40ac854; continue;
 }
 0x40ac854 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40ac85a); f_040a3230(cpu); if cpu.returned_to != 0x40ac85a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40ac85d; continue;
 }
 0x40ac85d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ac860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ac860); }
 let mut pc: u32 = 0x40ac860;
 loop { match pc {
 0x40ac860 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x40ac86e); f_040a3260(cpu); if cpu.returned_to != 0x40ac86e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ac897; continue; }
  pc = 0x40ac87a; continue;
 }
 0x40ac87a => {
  { let v: u32 = 0x414b58cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40ac886; continue;
 }
 0x40ac886 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40ac886; continue; }
  pc = 0x40ac890; continue;
 }
 0x40ac890 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40acae0; continue;
 }
 0x40ac897 => {
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
  cpu.push(0x40ac8c5); f_040a2e70(cpu); if cpu.returned_to != 0x40ac8c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ac8f8; continue; }
  pc = 0x40ac8cf; continue;
 }
 0x40ac8cf => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ac8d5); f_040959e0(cpu); if cpu.returned_to != 0x40ac8d5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x414b59cu32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40ac8e4; continue;
 }
 0x40ac8e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40ac8e4; continue; }
  pc = 0x40ac8ee; continue;
 }
 0x40ac8ee => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40ac8f8 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x414b5acu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40ac904; continue;
 }
 0x40ac904 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac912; continue; }
  pc = 0x40ac90c; continue;
 }
 0x40ac90c => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac912; continue; }
  pc = 0x40ac910; continue;
 }
 0x40ac910 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac912; continue;
 }
 0x40ac912 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac91f; continue; }
  pc = 0x40ac917; continue;
 }
 0x40ac917 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac91f; continue; }
  pc = 0x40ac91c; continue;
 }
 0x40ac91c => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40ac91f; continue;
 }
 0x40ac91f => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40ac92e; continue; }
  pc = 0x40ac923; continue;
 }
 0x40ac923 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40ac92e; continue; }
  pc = 0x40ac927; continue;
 }
 0x40ac927 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40ac92e; continue; }
  pc = 0x40ac92a; continue;
 }
 0x40ac92a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40ac904; continue;
 }
 0x40ac92e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40ac935; continue; }
  pc = 0x40ac933; continue;
 }
 0x40ac933 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac935; continue;
 }
 0x40ac935 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40ac945; continue; }
  pc = 0x40ac939; continue;
 }
 0x40ac939 => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40aca50; continue;
 }
 0x40ac945 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x414b5b8u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x40ac955; continue;
 }
 0x40ac955 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac963; continue; }
  pc = 0x40ac95d; continue;
 }
 0x40ac95d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac963; continue; }
  pc = 0x40ac961; continue;
 }
 0x40ac961 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac963; continue;
 }
 0x40ac963 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac970; continue; }
  pc = 0x40ac968; continue;
 }
 0x40ac968 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac970; continue; }
  pc = 0x40ac96d; continue;
 }
 0x40ac96d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40ac970; continue;
 }
 0x40ac970 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40ac97f; continue; }
  pc = 0x40ac974; continue;
 }
 0x40ac974 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40ac97f; continue; }
  pc = 0x40ac978; continue;
 }
 0x40ac978 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40ac97f; continue; }
  pc = 0x40ac97b; continue;
 }
 0x40ac97b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40ac955; continue;
 }
 0x40ac97f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40ac986; continue; }
  pc = 0x40ac984; continue;
 }
 0x40ac984 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac986; continue;
 }
 0x40ac986 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40ac996; continue; }
  pc = 0x40ac98a; continue;
 }
 0x40ac98a => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40aca50; continue;
 }
 0x40ac996 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x414b5c0u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x40ac9a3; continue;
 }
 0x40ac9a3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac9b1; continue; }
  pc = 0x40ac9ab; continue;
 }
 0x40ac9ab => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac9b1; continue; }
  pc = 0x40ac9af; continue;
 }
 0x40ac9af => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac9b1; continue;
 }
 0x40ac9b1 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac9be; continue; }
  pc = 0x40ac9b6; continue;
 }
 0x40ac9b6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac9be; continue; }
  pc = 0x40ac9bb; continue;
 }
 0x40ac9bb => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40ac9be; continue;
 }
 0x40ac9be => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40ac9cd; continue; }
  pc = 0x40ac9c2; continue;
 }
 0x40ac9c2 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40ac9cd; continue; }
  pc = 0x40ac9c6; continue;
 }
 0x40ac9c6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40ac9cd; continue; }
  pc = 0x40ac9c9; continue;
 }
 0x40ac9c9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40ac9a3; continue;
 }
 0x40ac9cd => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40ac9d4; continue; }
  pc = 0x40ac9d2; continue;
 }
 0x40ac9d2 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac9d4; continue;
 }
 0x40ac9d4 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40aca1d; continue; }
  pc = 0x40ac9d8; continue;
 }
 0x40ac9d8 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x414b5c8u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x40ac9e8; continue;
 }
 0x40ac9e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x40ac9f6; continue; }
  pc = 0x40ac9f0; continue;
 }
 0x40ac9f0 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ac9f6; continue; }
  pc = 0x40ac9f4; continue;
 }
 0x40ac9f4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ac9f6; continue;
 }
 0x40ac9f6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x40aca03; continue; }
  pc = 0x40ac9fb; continue;
 }
 0x40ac9fb => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40aca03; continue; }
  pc = 0x40aca00; continue;
 }
 0x40aca00 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40aca03; continue;
 }
 0x40aca03 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40aca12; continue; }
  pc = 0x40aca07; continue;
 }
 0x40aca07 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40aca12; continue; }
  pc = 0x40aca0b; continue;
 }
 0x40aca0b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40aca12; continue; }
  pc = 0x40aca0e; continue;
 }
 0x40aca0e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x40ac9e8; continue;
 }
 0x40aca12 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40aca19; continue; }
  pc = 0x40aca17; continue;
 }
 0x40aca17 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40aca19; continue;
 }
 0x40aca19 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x40aca49; continue; }
  pc = 0x40aca1d; continue;
 }
 0x40aca1d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40aca26); f_040959e0(cpu); if cpu.returned_to != 0x40aca26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x414b5d0u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40aca35; continue;
 }
 0x40aca35 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40aca35; continue; }
  pc = 0x40aca3f; continue;
 }
 0x40aca3f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aca49 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40aca50; continue;
 }
 0x40aca50 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x40aca8b; continue; }
  pc = 0x40aca59; continue;
 }
 0x40aca59 => {
  { cpu.sub32(cpu.r32(0x42c61ccu32), 0x0u32); }
  if !cpu.zf { pc = 0x40aca8b; continue; }
  pc = 0x40aca62; continue;
 }
 0x40aca62 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40aca68); f_040959e0(cpu); if cpu.returned_to != 0x40aca68 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x414b5e4u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40aca77; continue;
 }
 0x40aca77 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40aca77; continue; }
  pc = 0x40aca81; continue;
 }
 0x40aca81 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40aca8b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x40acac9; continue; }
  pc = 0x40aca94; continue;
 }
 0x40aca94 => {
  { cpu.sub32(cpu.r32(0x42c61c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x40acac9; continue; }
  pc = 0x40aca9d; continue;
 }
 0x40aca9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40acaa6); f_040959e0(cpu); if cpu.returned_to != 0x40acaa6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x414b600u32; cpu.esi = v; }
  { let v: u32 = 0x42c5de0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40acab5; continue;
 }
 0x40acab5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40acab5; continue; }
  pc = 0x40acabf; continue;
 }
 0x40acabf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40acac9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x415c728u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40acae0; continue;
 }
 0x40acae0 => {
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
pub fn f_040acc00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acc00); }
 let mut pc: u32 = 0x40acc00;
 loop { match pc {
 0x40acc00 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40acc0c); f_040996c0(cpu); if cpu.returned_to != 0x40acc0c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40acc1e); f_040996c0(cpu); if cpu.returned_to != 0x40acc1e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40acc2e); f_0409b630(cpu); if cpu.returned_to != 0x40acc2e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040acc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acc40); }
 let mut pc: u32 = 0x40acc40;
 loop { match pc {
 0x40acc40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40acc4c); f_04097130(cpu); if cpu.returned_to != 0x40acc4c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40acc5e); f_04097130(cpu); if cpu.returned_to != 0x40acc5e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40acc6e); f_04099150(cpu); if cpu.returned_to != 0x40acc6e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040acc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acc80); }
 let mut pc: u32 = 0x40acc80;
 loop { match pc {
 0x40acc80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c72cu32), 0x0u32); }
  if !cpu.zf { pc = 0x40accd6; continue; }
  pc = 0x40acc8c; continue;
 }
 0x40acc8c => {
  { let v = 0x40ac520u32; cpu.push(v); }
  { let v = 0x40acc80u32; cpu.push(v); }
  cpu.push(0x40acc9b); f_040a30c0(cpu); if cpu.returned_to != 0x40acc9b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c61ccu32; cpu.push(v); }
  cpu.push(0x40accaa); f_040a3000(cpu); if cpu.returned_to != 0x40accaa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c61c8u32; cpu.push(v); }
  cpu.push(0x40accb9); f_040a3000(cpu); if cpu.returned_to != 0x40accb9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415c728u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x40acccd); f_040a3000(cpu); if cpu.returned_to != 0x40acccd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c72cu32, v); }
  pc = 0x40accd6; continue;
 }
 0x40accd6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040accda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40accda); }
 let mut pc: u32 = 0x40accda;
 loop { match pc {
 0x40accda => {
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
  if cpu.zf { pc = 0x40acd66; continue; }
  pc = 0x40accf0; continue;
 }
 0x40accf0 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd04; continue;
 }
 0x40acd04 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd0f; continue;
 }
 0x40acd0f => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  pc = 0x40acd14; continue;
 }
 0x40acd14 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd1c; continue;
 }
 0x40acd1c => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd24; continue;
 }
 0x40acd24 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40acd30); f_040b4135(cpu); if cpu.returned_to != 0x40acd30 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40acd44; continue; }
  pc = 0x40acd37; continue;
 }
 0x40acd37 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40acd60; continue;
 }
 0x40acd44 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40acd60; continue; }
  pc = 0x40acd48; continue;
 }
 0x40acd48 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x40acd52); f_0409ebfb(cpu); if cpu.returned_to != 0x40acd52 { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40acd60; continue;
 }
 0x40acd60 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x40acd14; continue;
 }
 0x40acd66 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd6c; continue;
 }
 0x40acd6c => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x20u8); }
  if !cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd76; continue;
 }
 0x40acd76 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40acd8e; continue; }
  pc = 0x40acd7d; continue;
 }
 0x40acd7d => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40acd8e); f_040a15dd(cpu); if cpu.returned_to != 0x40acd8e { return; }
  pc = 0x40acd8e; continue;
 }
 0x40acd8e => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40acd9e; continue; }
  pc = 0x40acd93; continue;
 }
 0x40acd93 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40acd9e; continue;
 }
 0x40acd9e => {
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
pub fn f_040acdb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acdb3); }
 let mut pc: u32 = 0x40acdb3;
 loop { match pc {
 0x40acdb3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415b810u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x40acdca); f_040ace74(cpu); if cpu.returned_to != 0x40acdca { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415b810u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x40acdec); f_040ad06c(cpu); if cpu.returned_to != 0x40acdec { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ace4d; continue; }
  pc = 0x40acdf2; continue;
 }
 0x40acdf2 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415b810u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415ac7cu32, v); }
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
  { let v: u32 = cpu.r32(0x415ac7cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x415b814u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x40ace34); f_040ace74(cpu); if cpu.returned_to != 0x40ace34 { return; }
  { let v: u32 = cpu.r32(0x415ac7cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415ac78u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40ace57; continue;
 }
 0x40ace4d => {
  { let v: u32 = cpu.eax; cpu.w32(0x415ac7cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415ac78u32, v); }
  pc = 0x40ace57; continue;
 }
 0x40ace57 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c6278u32, v); }
  { let v: u32 = cpu.r32(0x415ac7cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c627cu32, v); }
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
pub fn f_040ace74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ace74); }
 let mut pc: u32 = 0x40ace74;
 loop { match pc {
 0x40ace74 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c6280u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40ace8b; continue;
 }
 0x40ace8b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.zf { pc = 0x40ace99; continue; }
  pc = 0x40ace94; continue;
 }
 0x40ace94 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if !cpu.zf { pc = 0x40ace9c; continue; }
  pc = 0x40ace99; continue;
 }
 0x40ace99 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40ace8b; continue;
 }
 0x40ace9c => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x40acf6e; continue; }
  pc = 0x40acea4; continue;
 }
 0x40acea4 => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if !cpu.zf { pc = 0x40aceae; continue; }
  pc = 0x40aceab; continue;
 }
 0x40aceab => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40aceae; continue;
 }
 0x40aceae => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x40aceb3; continue;
 }
 0x40aceb3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x40acedc; continue; }
  pc = 0x40acebc; continue;
 }
 0x40acebc => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40acf3b; continue; }
  pc = 0x40acec5; continue;
 }
 0x40acec5 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40acedc; continue; }
  pc = 0x40acec9; continue;
 }
 0x40acec9 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x40aced2; continue; }
  pc = 0x40acecd; continue;
 }
 0x40acecd => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40acecf; continue;
 }
 0x40acecf => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40aceb3; continue;
 }
 0x40aced2 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x40acedc; continue; }
  pc = 0x40aced7; continue;
 }
 0x40aced7 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x40aceb3; continue;
 }
 0x40acedc => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.zf { pc = 0x40aceea; continue; }
  pc = 0x40acee5; continue;
 }
 0x40acee5 => {
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if !cpu.zf { pc = 0x40aceee; continue; }
  pc = 0x40aceea; continue;
 }
 0x40aceea => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40acf3b; continue; }
  pc = 0x40aceee; continue;
 }
 0x40aceee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x40acf3b; continue; }
  pc = 0x40acef4; continue;
 }
 0x40acef4 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  if !cpu.zf { pc = 0x40acf2c; continue; }
  pc = 0x40acefd; continue;
 }
 0x40acefd => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40acf0d; continue; }
  pc = 0x40acf01; continue;
 }
 0x40acf01 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  pc = 0x40acf09; continue;
 }
 0x40acf09 => {
  if !cpu.zf { pc = 0x40acf2c; continue; }
  pc = 0x40acf0b; continue;
 }
 0x40acf0b => {
  pc = 0x40acf2b; continue;
 }
 0x40acf0d => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x40acf21; continue; }
  pc = 0x40acf12; continue;
 }
 0x40acf12 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if cpu.zf { pc = 0x40acf2b; continue; }
  pc = 0x40acf1c; continue;
 }
 0x40acf1c => {
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  cpu.poll(); pc = 0x40acf09; continue;
 }
 0x40acf21 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x40acf2c; continue; }
  pc = 0x40acf2b; continue;
 }
 0x40acf2b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40acf2c; continue;
 }
 0x40acf2c => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x40acecf; continue; }
  pc = 0x40acf30; continue;
 }
 0x40acf30 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x40aceb3; continue;
 }
 0x40acf3b => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40acf61; continue; }
  pc = 0x40acf3f; continue;
 }
 0x40acf3f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x40acf58; continue; }
  pc = 0x40acf54; continue;
 }
 0x40acf54 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40acf6e; continue;
 }
 0x40acf58 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x40ace8b; continue;
 }
 0x40acf61 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40ace99; continue; }
  pc = 0x40acf6e; continue;
 }
 0x40acf6e => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c6280u32, v); }
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
pub fn f_040acf83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acf83); }
 let mut pc: u32 = 0x40acf83;
 loop { match pc {
 0x40acf83 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x415b824u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x40acf98); f_040b1989(cpu); if cpu.returned_to != 0x40acf98 { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x40acfa4; continue; }
  pc = 0x40acf9c; continue;
 }
 0x40acf9c => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x40acfa6; continue;
 }
 0x40acfa4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40acfa6; continue;
 }
 0x40acfa6 => {
  { let v: u32 = cpu.r32(0x415b824u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b828u32, v); }
  cpu.push(0x40acfb8); f_040b41a9(cpu); if cpu.returned_to != 0x40acfb8 { return; }
  { let v: u32 = cpu.r32(0x42c627cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c6278u32); cpu.eax = v; }
  cpu.push(0x40acfc8); f_0406e738(cpu); if cpu.returned_to != 0x40acfc8 { return; }
  cpu.push(0x40acfcd); f_040b4213(cpu); if cpu.returned_to != 0x40acfcd { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040acfd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40acfd2); }
 let mut pc: u32 = 0x40acfd2;
 loop { match pc {
 0x40acfd2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40acfd9; continue;
 }
 0x40acfd9 => {
  { let v: u32 = 0x415cc9au32; cpu.esi = v; }
  { let v: u32 = 0x415ccd6u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40acfe9; continue;
 }
 0x40acfe9 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x40ad001; continue; }
  pc = 0x40acfed; continue;
 }
 0x40acfed => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x40acffc; continue; }
  pc = 0x40acff2; continue;
 }
 0x40acff2 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40acffc; continue; }
  pc = 0x40acff7; continue;
 }
 0x40acff7 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40acffc; continue;
 }
 0x40acffc => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40acfe9; continue;
 }
 0x40ad001 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x40ad017; continue; }
  pc = 0x40ad005; continue;
 }
 0x40ad005 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40ad012; continue; }
  pc = 0x40ad00c; continue;
 }
 0x40ad00c => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x40ad00f); cpu.push(0x40ad011); cpu.call(t); if cpu.returned_to != 0x40ad011 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x40ad012; continue;
 }
 0x40ad012 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40acfd9; continue;
 }
 0x40ad017 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x40ad01c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad01d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad01d); }
 let mut pc: u32 = 0x40ad01d;
 loop { match pc {
 0x40ad01d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40ad025; continue;
 }
 0x40ad025 => {
  { let v: u32 = 0x415ccd6u32; cpu.esi = v; }
  { let v: u32 = 0x415cce8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ad035; continue;
 }
 0x40ad035 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x40ad04d; continue; }
  pc = 0x40ad039; continue;
 }
 0x40ad039 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x40ad048; continue; }
  pc = 0x40ad03e; continue;
 }
 0x40ad03e => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ad048; continue; }
  pc = 0x40ad043; continue;
 }
 0x40ad043 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40ad048; continue;
 }
 0x40ad048 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40ad035; continue;
 }
 0x40ad04d => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x40ad067; continue; }
  pc = 0x40ad051; continue;
 }
 0x40ad051 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ad062; continue; }
  pc = 0x40ad055; continue;
 }
 0x40ad055 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40ad062; continue; }
  pc = 0x40ad05c; continue;
 }
 0x40ad05c => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x40ad05f); cpu.push(0x40ad061); cpu.call(t); if cpu.returned_to != 0x40ad061 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x40ad062; continue;
 }
 0x40ad062 => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40ad025; continue;
 }
 0x40ad067 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x40ad06b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad06c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad06c); }
 let mut pc: u32 = 0x40ad06c;
 loop { match pc {
 0x40ad06c => {
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
  if cpu.zf { pc = 0x40ad085; continue; }
  pc = 0x40ad080; continue;
 }
 0x40ad080 => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40ad08c; continue; }
  pc = 0x40ad085; continue;
 }
 0x40ad085 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40ad140; continue;
 }
 0x40ad08c => {
  { let v: u32 = 0x415c730u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x40ad09f; continue;
 }
 0x40ad09f => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x40ad0ab; continue; }
  pc = 0x40ad0a6; continue;
 }
 0x40ad0a6 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x40ad0ab; continue;
 }
 0x40ad0ab => {
  { cpu.sub32(cpu.eax, cpu.r32(0x415c738u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x40ad0cb; continue; }
  pc = 0x40ad0b3; continue;
 }
 0x40ad0b3 => {
  { let v: u32 = cpu.r32(0x415c734u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40ad103; continue; }
  pc = 0x40ad0bd; continue;
 }
 0x40ad0bd => {
  { let v: u32 = cpu.ecx; cpu.w32(0x415c738u32, v); }
  { let v: u32 = cpu.r32(0x415c730u32); cpu.ecx = v; }
  pc = 0x40ad103; continue;
 }
 0x40ad0cb => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x40ad0bd; continue;
 }
 0x40ad0cf => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415c734u32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ad0e8); f_040b4242(cpu); if cpu.returned_to != 0x40ad0e8 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad136; continue; }
  pc = 0x40ad0ee; continue;
 }
 0x40ad0ee => {
  { let v: u32 = cpu.r32(0x415c738u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x40ad100; continue; }
  pc = 0x40ad0fb; continue;
 }
 0x40ad0fb => {
  { let v: u32 = cpu.eax; cpu.w32(0x415c738u32, v); }
  pc = 0x40ad100; continue;
 }
 0x40ad100 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x40ad103; continue;
 }
 0x40ad103 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x40ad0cf; continue; }
  pc = 0x40ad107; continue;
 }
 0x40ad107 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x40ad118; continue; }
  pc = 0x40ad10d; continue;
 }
 0x40ad10d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ad114); f_040b47e6(cpu); if cpu.returned_to != 0x40ad114 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad12d; continue; }
  pc = 0x40ad118; continue;
 }
 0x40ad118 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ad11f); f_040b49af(cpu); if cpu.returned_to != 0x40ad11f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ad136; continue; }
  pc = 0x40ad123; continue;
 }
 0x40ad123 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40ad09f; continue;
 }
 0x40ad12d => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40ad09f; continue;
 }
 0x40ad136 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x42c6284u32, v); }
  pc = 0x40ad140; continue;
 }
 0x40ad140 => {
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
pub fn f_040ad14f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad14f); }
 let mut pc: u32 = 0x40ad14f;
 loop { match pc {
 0x40ad14f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ad1cb; continue; }
  pc = 0x40ad15e; continue;
 }
 0x40ad15e => {
  { let v: u32 = cpu.r32(0x42c61d4u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40ad171; continue; }
  pc = 0x40ad168; continue;
 }
 0x40ad168 => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ad171; continue; }
  pc = 0x40ad16c; continue;
 }
 0x40ad16c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x40ad18b; continue; }
  pc = 0x40ad171; continue;
 }
 0x40ad171 => {
  { let v: u32 = cpu.r32(0x415c730u32); cpu.ecx = v; }
  pc = 0x40ad184; continue;
 }
 0x40ad179 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40ad181; continue; }
  pc = 0x40ad17d; continue;
 }
 0x40ad17d => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x40ad18b; continue; }
  pc = 0x40ad181; continue;
 }
 0x40ad181 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x40ad184; continue;
 }
 0x40ad184 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x40ad179; continue; }
  pc = 0x40ad18b; continue;
 }
 0x40ad18b => {
  { let v: u32 = 0x415c730u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40ad1a1); f_040b42ea(cpu); if cpu.returned_to != 0x40ad1a1 { return; }
  { let v: u32 = cpu.r32(0x415c734u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42c61d4u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x40ad1c3; continue; }
  pc = 0x40ad1b1; continue;
 }
 0x40ad1b1 => {
  { let v: u32 = cpu.r32(0x415c738u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x40ad1c3; continue; }
  pc = 0x40ad1bd; continue;
 }
 0x40ad1bd => {
  { let v: u32 = cpu.edx; cpu.w32(0x415c738u32, v); }
  pc = 0x40ad1c3; continue;
 }
 0x40ad1c3 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x42c6284u32, v); }
  pc = 0x40ad1cb; continue;
 }
 0x40ad1cb => {
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
pub fn f_040ad1d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad1d1); }
 let mut pc: u32 = 0x40ad1d1;
 loop { match pc {
 0x40ad1d1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40ad229; continue; }
  pc = 0x40ad1e3; continue;
 }
 0x40ad1e3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ad229; continue; }
  pc = 0x40ad1e7; continue;
 }
 0x40ad1e7 => {
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
  pc = 0x40ad223; continue;
 }
 0x40ad1fd => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40ad208); f_0409534e(cpu); if cpu.returned_to != 0x40ad208 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad220; continue; }
  pc = 0x40ad20c; continue;
 }
 0x40ad20c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x40ad220; continue; }
  pc = 0x40ad219; continue;
 }
 0x40ad219 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x40ad22b; continue;
 }
 0x40ad220 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x40ad223; continue;
 }
 0x40ad223 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x40ad1fd; continue; }
  pc = 0x40ad229; continue;
 }
 0x40ad229 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40ad22b; continue;
 }
 0x40ad22b => {
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
pub fn f_040ad232(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad232); }
 let mut pc: u32 = 0x40ad232;
 loop { match pc {
 0x40ad232 => {
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
pub fn f_040ad265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad265); }
 let mut pc: u32 = 0x40ad265;
 loop { match pc {
 0x40ad265 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b9c0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x414b9c7u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x414b9ceu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b99cu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x414b9a8u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x414b9b4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40ad2c2); f_040ad232(cpu); if cpu.returned_to != 0x40ad2c2 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x40ad2d0; continue; }
  pc = 0x40ad2cc; continue;
 }
 0x40ad2cc => {
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40ad2d0; continue;
 }
 0x40ad2d0 => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40ad2e3); f_040ad232(cpu); if cpu.returned_to != 0x40ad2e3 { return; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40ad2f6); f_040ad232(cpu); if cpu.returned_to != 0x40ad2f6 { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40ad308); f_040ad232(cpu); if cpu.returned_to != 0x40ad308 { return; }
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
  cpu.push(0x40ad330); f_040ad232(cpu); if cpu.returned_to != 0x40ad330 { return; }
  { let v: u32 = 0x16u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40ad340); f_040ad232(cpu); if cpu.returned_to != 0x40ad340 { return; }
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
pub fn f_040ad370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad370); }
 let mut pc: u32 = 0x40ad370;
 loop { match pc {
 0x40ad370 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x42c61d8u32; cpu.edx = v; }
  cpu.push(0x40ad37b); f_040ad265(cpu); if cpu.returned_to != 0x40ad37b { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad382(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad382); }
 let mut pc: u32 = 0x40ad382;
 loop { match pc {
 0x40ad382 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x40ad38f); f_040b49b2(cpu); if cpu.returned_to != 0x40ad38f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad39c; continue; }
  pc = 0x40ad393; continue;
 }
 0x40ad393 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40ad39c); f_0409db1d(cpu); if cpu.returned_to != 0x40ad39c { return; }
  pc = 0x40ad39c; continue;
 }
 0x40ad39c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad39f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad39f); }
 let mut pc: u32 = 0x40ad39f;
 loop { match pc {
 0x40ad39f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40ad3ab; continue; }
  pc = 0x40ad3a6; continue;
 }
 0x40ad3a6 => {
  { let v: u8 = 0x2fu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40ad3b9; continue;
 }
 0x40ad3ab => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x37u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40ad3b9; continue;
 }
 0x40ad3b9 => {
  { let v: u8 = 0x63u8; cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad3c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad3c5); }
 let mut pc: u32 = 0x40ad3c5;
 loop { match pc {
 0x40ad3c5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(44); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x40ad3d8); f_04092c82(cpu); if cpu.returned_to != 0x40ad3d8 { return; }
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
pub fn f_040ad3e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad3e4); }
 let mut pc: u32 = 0x40ad3e4;
 loop { match pc {
 0x40ad3e4 => {
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
  cpu.push(0x40ad3fe); f_040b4b1f(cpu); if cpu.returned_to != 0x40ad3fe { return; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x40ad40e); f_040b4b72(cpu); if cpu.returned_to != 0x40ad40e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad417(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad417); }
 let mut pc: u32 = 0x40ad417;
 loop { match pc {
 0x40ad417 => {
  { let v = 0xe4u32; cpu.push(v); }
  cpu.flat_ds(0x40ad41c); cpu.push(0x40ad421); f_0409deaa(cpu); if cpu.returned_to != 0x40ad421 { return; }
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
  if cpu.cf { pc = 0x40ad453; continue; }
  pc = 0x40ad43f; continue;
 }
 0x40ad43f => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.flat_ds(0x40ad444); cpu.push(0x40ad449); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad449 { return; }
  pc = 0x40ad449; continue;
 }
 0x40ad449 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40ad71f; continue;
 }
 0x40ad453 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x56u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x5au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5eu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad466); cpu.push(0x40ad46b); f_040b49e7(cpu); if cpu.returned_to != 0x40ad46b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40ad71f; continue; }
  pc = 0x40ad477; continue;
 }
 0x40ad477 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x40ad479); cpu.push(0x40ad47e); f_04094634(cpu); if cpu.returned_to != 0x40ad47e { return; }
  { let v: u32 = 0x9au32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.flat_ds(0x40ad486); cpu.push(0x40ad48b); f_040ad06c(cpu); if cpu.returned_to != 0x40ad48b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad4bc; continue; }
  pc = 0x40ad492; continue;
 }
 0x40ad492 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xfcu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x40ad498); cpu.push(0x40ad49d); f_040b1989(cpu); if cpu.returned_to != 0x40ad49d { return; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.cf { pc = 0x40ad4a9; continue; }
  pc = 0x40ad4a1; continue;
 }
 0x40ad4a1 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x40ad4ab; continue;
 }
 0x40ad4a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40ad4ab; continue;
 }
 0x40ad4ab => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad4bf; continue; }
  pc = 0x40ad4b2; continue;
 }
 0x40ad4b2 => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad4b5); cpu.push(0x40ad4ba); f_040ad14f(cpu); if cpu.returned_to != 0x40ad4ba { return; }
  cpu.poll(); pc = 0x40ad449; continue;
 }
 0x40ad4bc => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ad4bf; continue;
 }
 0x40ad4bf => {
  { let v: u32 = 0x42u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x4au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x46u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x93u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x40ad4d8); cpu.push(0x40ad4dd); f_040b4c92(cpu); if cpu.returned_to != 0x40ad4dd { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x415c7ccu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40ad4e0); cpu.push(0x40ad4e6); cpu.call(t); if cpu.returned_to != 0x40ad4e6 { return; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad4e9); cpu.push(0x40ad4ee); f_040ad06c(cpu); if cpu.returned_to != 0x40ad4ee { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad53e; continue; }
  pc = 0x40ad4f5; continue;
 }
 0x40ad4f5 => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x40ad4fe); cpu.push(0x40ad503); f_040b1989(cpu); if cpu.returned_to != 0x40ad503 { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x40ad515; continue; }
  pc = 0x40ad507; continue;
 }
 0x40ad507 => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x40ad517; continue;
 }
 0x40ad515 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40ad517; continue;
 }
 0x40ad517 => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad541; continue; }
  pc = 0x40ad51e; continue;
 }
 0x40ad51e => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.flat_ds(0x40ad523); cpu.push(0x40ad528); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad528 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ad534); cpu.push(0x40ad539); f_0409ec1e(cpu); if cpu.returned_to != 0x40ad539 { return; }
  pc = 0x40ad6fb; continue;
 }
 0x40ad53e => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ad541; continue;
 }
 0x40ad541 => {
  { let v: u32 = cpu.r8(0x415b843u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x40ad564; continue; }
  pc = 0x40ad54d; continue;
 }
 0x40ad54d => {
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x40ad564; continue; }
  pc = 0x40ad555; continue;
 }
 0x40ad555 => {
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x40ad564; continue; }
  pc = 0x40ad55d; continue;
 }
 0x40ad55d => {
  { let v: u32 = 0x414b620u32; cpu.w32(0x4au32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ad564; continue;
 }
 0x40ad564 => {
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x3eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad573); cpu.push(0x40ad578); f_040a17b0(cpu); if cpu.returned_to != 0x40ad578 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x40ad57d); cpu.push(0x40ad582); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad582 { return; }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x40ad5d4; continue; }
  pc = 0x40ad58a; continue;
 }
 0x40ad58a => {
  { let v: u32 = 0x414b623u32; cpu.edx = v; }
  cpu.flat_ds(0x40ad58f); cpu.push(0x40ad594); f_04092897(cpu); if cpu.returned_to != 0x40ad594 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad5b4; continue; }
  pc = 0x40ad598; continue;
 }
 0x40ad598 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ad5a2); cpu.push(0x40ad5a7); f_040ad3c5(cpu); if cpu.returned_to != 0x40ad5a7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40ad698; continue; }
  pc = 0x40ad5af; continue;
 }
 0x40ad5af => {
  pc = 0x40ad6fb; continue;
 }
 0x40ad5b4 => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x40ad5bc); cpu.push(0x40ad5c1); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad5c1 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad5ca); cpu.push(0x40ad5cf); f_040ad3e4(cpu); if cpu.returned_to != 0x40ad5cf { return; }
  pc = 0x40ad6f8; continue;
 }
 0x40ad5d4 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x40ad5da); cpu.push(0x40ad5df); f_04094634(cpu); if cpu.returned_to != 0x40ad5df { return; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x40ad5f6; continue; }
  pc = 0x40ad5ea; continue;
 }
 0x40ad5ea => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x40ad5ef); cpu.push(0x40ad5f4); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad5f4 { return; }
  pc = 0x40ad61f; continue;
 }
 0x40ad5f6 => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x414b628u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x40ad609); cpu.push(0x40ad60e); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad60e { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad617); cpu.push(0x40ad61c); f_040ad3e4(cpu); if cpu.returned_to != 0x40ad61c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ad61f; continue;
 }
 0x40ad61f => {
  cpu.flat_ds(0x40ad61f); cpu.push(0x40ad624); f_040a12e1(cpu); if cpu.returned_to != 0x40ad624 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x40ad637; continue; }
  pc = 0x40ad629; continue;
 }
 0x40ad629 => {
  cpu.flat_ds(0x40ad629); cpu.push(0x40ad62e); f_040a12e1(cpu); if cpu.returned_to != 0x40ad62e { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x40ad6fb; continue; }
  pc = 0x40ad637; continue;
 }
 0x40ad637 => {
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x40ad642); cpu.push(0x40ad647); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad647 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x414b62du32; cpu.esi = v; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.flat_ds(0x40ad659); cpu.push(0x40ad65e); f_040ad3e4(cpu); if cpu.returned_to != 0x40ad65e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ad661); cpu.push(0x40ad666); f_040a12e1(cpu); if cpu.returned_to != 0x40ad666 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x40ad679; continue; }
  pc = 0x40ad66b; continue;
 }
 0x40ad66b => {
  cpu.flat_ds(0x40ad66b); cpu.push(0x40ad670); f_040a12e1(cpu); if cpu.returned_to != 0x40ad670 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x40ad6fb; continue; }
  pc = 0x40ad679; continue;
 }
 0x40ad679 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.flat_ds(0x40ad67e); cpu.push(0x40ad683); f_0409ebfb(cpu); if cpu.returned_to != 0x40ad683 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x414b623u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x40ad68f); cpu.push(0x40ad694); f_040ad3c5(cpu); if cpu.returned_to != 0x40ad694 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40ad6fb; continue; }
  pc = 0x40ad698; continue;
 }
 0x40ad698 => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x40ad6a6); cpu.push(0x40ad6ab); f_040ad14f(cpu); if cpu.returned_to != 0x40ad6ab { return; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x40ad6b5); cpu.push(0x40ad6ba); f_040b4b1f(cpu); if cpu.returned_to != 0x40ad6ba { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.flat_ds(0x40ad6c9); cpu.push(0x40ad6ce); f_040ad39f(cpu); if cpu.returned_to != 0x40ad6ce { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x40ad6dc; continue; }
  pc = 0x40ad6d5; continue;
 }
 0x40ad6d5 => {
  { let v: u32 = 0x414b63au32; cpu.eax = v; }
  pc = 0x40ad6e1; continue;
 }
 0x40ad6dc => {
  { let v: u32 = 0x414b63eu32; cpu.eax = v; }
  pc = 0x40ad6e1; continue;
 }
 0x40ad6e1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x414b632u32; cpu.eax = v; }
  cpu.flat_ds(0x40ad6e7); cpu.push(0x40ad6ec); f_040ad1d1(cpu); if cpu.returned_to != 0x40ad6ec { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.flat_ds(0x40ad6f0); cpu.push(0x40ad6f5); f_040b4d6d(cpu); if cpu.returned_to != 0x40ad6f5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x40ad6f8; continue;
 }
 0x40ad6f8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40ad6fb; continue;
 }
 0x40ad6fb => {
  { let v: u32 = cpu.r32(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad6fe); cpu.push(0x40ad703); f_040ad14f(cpu); if cpu.returned_to != 0x40ad703 { return; }
  { let v: u32 = cpu.r32(0x66u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad706); cpu.push(0x40ad70b); f_040ad14f(cpu); if cpu.returned_to != 0x40ad70b { return; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x40ad70e); cpu.push(0x40ad713); f_040ad14f(cpu); if cpu.returned_to != 0x40ad713 { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x415c7d0u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40ad716); cpu.push(0x40ad71c); cpu.call(t); if cpu.returned_to != 0x40ad71c { return; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x40ad71f; continue;
 }
 0x40ad71f => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x40ad729); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad72a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad72a); }
 let mut pc: u32 = 0x40ad72a;
 loop { match pc {
 0x40ad72a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x40ad73d); f_040b4d8b(cpu); if cpu.returned_to != 0x40ad73d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad740); }
 let mut pc: u32 = 0x40ad740;
 loop { match pc {
 0x40ad740 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x40ad745); cpu.push(0x40ad74a); f_040ad778(cpu); if cpu.returned_to != 0x40ad74a { return; }
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
  cpu.flat_ds(0x40ad777); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad778); }
 let mut pc: u32 = 0x40ad778;
 loop { match pc {
 0x40ad778 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x40ad81eu32.wrapping_add(cpu.esi); cpu.eax = v; }
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
pub fn f_040ad7ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad7ea); }
 let mut pc: u32 = 0x40ad7ea;
 loop { match pc {
 0x40ad7ea => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x40ad81eu32.wrapping_add(cpu.eax); cpu.eax = v; }
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
pub fn f_040ad81e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad81e); }
 let mut pc: u32 = 0x40ad81e;
 loop { match pc {
 0x40ad81e => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad821(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad821); }
 let mut pc: u32 = 0x40ad821;
 loop { match pc {
 0x40ad821 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad82a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad82a); }
 let mut pc: u32 = 0x40ad82a;
 loop { match pc {
 0x40ad82a => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad82d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad82d); }
 let mut pc: u32 = 0x40ad82d;
 loop { match pc {
 0x40ad82d => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad830); }
 let mut pc: u32 = 0x40ad830;
 loop { match pc {
 0x40ad830 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad833(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad833); }
 let mut pc: u32 = 0x40ad833;
 loop { match pc {
 0x40ad833 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad836(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad836); }
 let mut pc: u32 = 0x40ad836;
 loop { match pc {
 0x40ad836 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad839(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad839); }
 let mut pc: u32 = 0x40ad839;
 loop { match pc {
 0x40ad839 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad83c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad83c); }
 let mut pc: u32 = 0x40ad83c;
 loop { match pc {
 0x40ad83c => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad83f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad83f); }
 let mut pc: u32 = 0x40ad83f;
 loop { match pc {
 0x40ad83f => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad842(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad842); }
 let mut pc: u32 = 0x40ad842;
 loop { match pc {
 0x40ad842 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad845(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad845); }
 let mut pc: u32 = 0x40ad845;
 loop { match pc {
 0x40ad845 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad848(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad848); }
 let mut pc: u32 = 0x40ad848;
 loop { match pc {
 0x40ad848 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad84b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad84b); }
 let mut pc: u32 = 0x40ad84b;
 loop { match pc {
 0x40ad84b => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad84e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad84e); }
 let mut pc: u32 = 0x40ad84e;
 loop { match pc {
 0x40ad84e => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad851(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad851); }
 let mut pc: u32 = 0x40ad851;
 loop { match pc {
 0x40ad851 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad854(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad854); }
 let mut pc: u32 = 0x40ad854;
 loop { match pc {
 0x40ad854 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad857(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad857); }
 let mut pc: u32 = 0x40ad857;
 loop { match pc {
 0x40ad857 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad85a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad85a); }
 let mut pc: u32 = 0x40ad85a;
 loop { match pc {
 0x40ad85a => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad85d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad85d); }
 let mut pc: u32 = 0x40ad85d;
 loop { match pc {
 0x40ad85d => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad860); }
 let mut pc: u32 = 0x40ad860;
 loop { match pc {
 0x40ad860 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad863(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad863); }
 let mut pc: u32 = 0x40ad863;
 loop { match pc {
 0x40ad863 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad866(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad866); }
 let mut pc: u32 = 0x40ad866;
 loop { match pc {
 0x40ad866 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad869(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad869); }
 let mut pc: u32 = 0x40ad869;
 loop { match pc {
 0x40ad869 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad86c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad86c); }
 let mut pc: u32 = 0x40ad86c;
 loop { match pc {
 0x40ad86c => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad86f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad86f); }
 let mut pc: u32 = 0x40ad86f;
 loop { match pc {
 0x40ad86f => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad872(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad872); }
 let mut pc: u32 = 0x40ad872;
 loop { match pc {
 0x40ad872 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad875(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad875); }
 let mut pc: u32 = 0x40ad875;
 loop { match pc {
 0x40ad875 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad878(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad878); }
 let mut pc: u32 = 0x40ad878;
 loop { match pc {
 0x40ad878 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad87b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad87b); }
 let mut pc: u32 = 0x40ad87b;
 loop { match pc {
 0x40ad87b => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad87e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad87e); }
 let mut pc: u32 = 0x40ad87e;
 loop { match pc {
 0x40ad87e => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad881(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad881); }
 let mut pc: u32 = 0x40ad881;
 loop { match pc {
 0x40ad881 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad884(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad884); }
 let mut pc: u32 = 0x40ad884;
 loop { match pc {
 0x40ad884 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad887(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad887); }
 let mut pc: u32 = 0x40ad887;
 loop { match pc {
 0x40ad887 => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad88a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad88a); }
 let mut pc: u32 = 0x40ad88a;
 loop { match pc {
 0x40ad88a => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad88d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad88d); }
 let mut pc: u32 = 0x40ad88d;
 loop { match pc {
 0x40ad88d => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad890); }
 let mut pc: u32 = 0x40ad890;
 loop { match pc {
 0x40ad890 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad893(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad893); }
 let mut pc: u32 = 0x40ad893;
 loop { match pc {
 0x40ad893 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad896(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad896); }
 let mut pc: u32 = 0x40ad896;
 loop { match pc {
 0x40ad896 => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad899(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad899); }
 let mut pc: u32 = 0x40ad899;
 loop { match pc {
 0x40ad899 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad89c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad89c); }
 let mut pc: u32 = 0x40ad89c;
 loop { match pc {
 0x40ad89c => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad89f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad89f); }
 let mut pc: u32 = 0x40ad89f;
 loop { match pc {
 0x40ad89f => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8a2); }
 let mut pc: u32 = 0x40ad8a2;
 loop { match pc {
 0x40ad8a2 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8a5); }
 let mut pc: u32 = 0x40ad8a5;
 loop { match pc {
 0x40ad8a5 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8a8); }
 let mut pc: u32 = 0x40ad8a8;
 loop { match pc {
 0x40ad8a8 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ab); }
 let mut pc: u32 = 0x40ad8ab;
 loop { match pc {
 0x40ad8ab => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ae); }
 let mut pc: u32 = 0x40ad8ae;
 loop { match pc {
 0x40ad8ae => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8b1); }
 let mut pc: u32 = 0x40ad8b1;
 loop { match pc {
 0x40ad8b1 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8b4); }
 let mut pc: u32 = 0x40ad8b4;
 loop { match pc {
 0x40ad8b4 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8b7); }
 let mut pc: u32 = 0x40ad8b7;
 loop { match pc {
 0x40ad8b7 => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8ba); }
 let mut pc: u32 = 0x40ad8ba;
 loop { match pc {
 0x40ad8ba => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8bd); }
 let mut pc: u32 = 0x40ad8bd;
 loop { match pc {
 0x40ad8bd => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8c0); }
 let mut pc: u32 = 0x40ad8c0;
 loop { match pc {
 0x40ad8c0 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ad8c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ad8c3); }
 let mut pc: u32 = 0x40ad8c3;
 loop { match pc {
 0x40ad8c3 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
