#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_040b3e04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b3e04); }
 let mut pc: u32 = 0x40b3e04;
 loop { match pc {
 0x40b3e04 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x40b3e23; continue; }
  pc = 0x40b3e1d; continue;
 }
 0x40b3e1d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x40b3e26; continue; }
  pc = 0x40b3e23; continue;
 }
 0x40b3e23 => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x40b3e26; continue;
 }
 0x40b3e26 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x40b3e32; continue; }
  pc = 0x40b3e2c; continue;
 }
 0x40b3e2c => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x40b3e41; continue; }
  pc = 0x40b3e32; continue;
 }
 0x40b3e32 => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x40b3e41; continue; }
  pc = 0x40b3e3e; continue;
 }
 0x40b3e3e => {
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  pc = 0x40b3e41; continue;
 }
 0x40b3e41 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x40b3e4d; continue; }
  pc = 0x40b3e47; continue;
 }
 0x40b3e47 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x40b3e50; continue; }
  pc = 0x40b3e4d; continue;
 }
 0x40b3e4d => {
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  pc = 0x40b3e50; continue;
 }
 0x40b3e50 => {
  { cpu.and32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x40b3e5e; continue; }
  pc = 0x40b3e57; continue;
 }
 0x40b3e57 => {
  { cpu.and32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40b3e70; continue; }
  pc = 0x40b3e5e; continue;
 }
 0x40b3e5e => {
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x40b3e70; continue; }
  pc = 0x40b3e64; continue;
 }
 0x40b3e64 => {
  { cpu.sub32(cpu.r32(0x6cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x40b3e70; continue; }
  pc = 0x40b3e6a; continue;
 }
 0x40b3e6a => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x40u32); }
  if cpu.zf { pc = 0x40b3e7a; continue; }
  pc = 0x40b3e70; continue;
 }
 0x40b3e70 => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x40b3e86; continue; }
  pc = 0x40b3e7a; continue;
 }
 0x40b3e7a => {
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x40b3e86; continue; }
  pc = 0x40b3e83; continue;
 }
 0x40b3e83 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x40b3e86; continue;
 }
 0x40b3e86 => {
  { let v: u32 = cpu.eax; cpu.w32(0x415c9a8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b3e9d; continue; }
  pc = 0x40b3e98; continue;
 }
 0x40b3e98 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x40b3e9d; continue;
 }
 0x40b3e9d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10000u32); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b3eb1; continue; }
  pc = 0x40b3eaf; continue;
 }
 0x40b3eaf => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x40b3eb1; continue;
 }
 0x40b3eb1 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x42c5ef8u32.wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x40b3efb; continue; }
  pc = 0x40b3eb9; continue;
 }
 0x40b3eb9 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9bcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x40b3ed9; continue; }
  pc = 0x40b3ed5; continue;
 }
 0x40b3ed5 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x40b3ed9; continue;
 }
 0x40b3ed9 => {
  { cpu.and32(cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x40b3ee9; continue; }
  pc = 0x40b3ee5; continue;
 }
 0x40b3ee5 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x40b3ee9; continue;
 }
 0x40b3ee9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x415c9c0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c9c4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])) | 0x20u32; cpu.w32(0x415c9a8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b3efb; continue;
 }
 0x40b3efb => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9b4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c9b8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9b0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  let t = cpu.r32(0x40b21d8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.flat_ds(0x40b3f41); cpu.push(0x40b3f49); cpu.call(t); if cpu.returned_to != 0x40b3f49 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x415c9b4u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x40b3f66); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b3f67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b3f67); }
 let mut pc: u32 = 0x40b3f67;
 loop { match pc {
 0x40b3f67 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x40b3f84; continue; }
  pc = 0x40b3f7e; continue;
 }
 0x40b3f7e => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x40b3f87; continue; }
  pc = 0x40b3f84; continue;
 }
 0x40b3f84 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x40b3f87; continue;
 }
 0x40b3f87 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x40b3f93; continue; }
  pc = 0x40b3f8d; continue;
 }
 0x40b3f8d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x40b3f96; continue; }
  pc = 0x40b3f93; continue;
 }
 0x40b3f93 => {
  { let v: u32 = cpu.eax | 0x20u32; cpu.eax = v; }
  pc = 0x40b3f96; continue;
 }
 0x40b3f96 => {
  { cpu.sub32(cpu.r32(0x68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40b3f9f; continue; }
  pc = 0x40b3f9c; continue;
 }
 0x40b3f9c => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  pc = 0x40b3f9f; continue;
 }
 0x40b3f9f => {
  { let v: u32 = cpu.eax; cpu.w32(0x415c9a8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x20u32); }
  if !cpu.zf { pc = 0x40b3fcd; continue; }
  pc = 0x40b3fb3; continue;
 }
 0x40b3fb3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x40b3fe6; continue; }
  pc = 0x40b3fc3; continue;
 }
 0x40b3fc3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x40b3fe6; continue;
 }
 0x40b3fcd => {
  { let v: u32 = cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])) | 0x8u32; cpu.w32(0x415c9a8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  pc = 0x40b3fe6; continue;
 }
 0x40b3fe6 => {
  { let v: u32 = cpu.r32(0x415c9a8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  let t = cpu.r32(0x40b1fd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.flat_ds(0x40b3feb); cpu.push(0x40b3ff3); cpu.call(t); if cpu.returned_to != 0x40b3ff3 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x40b3ff9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b3ffa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b3ffa); }
 let mut pc: u32 = 0x40b3ffa;
 loop { match pc {
 0x40b3ffa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.df = false; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.stos(1, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x40b4025); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4026(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4026); }
 let mut pc: u32 = 0x40b4026;
 loop { match pc {
 0x40b4026 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0x415c9c8u32; cpu.push(v); }
  { let v = 0x415c9a8u32; cpu.push(v); }
  cpu.flat_ds(0x40b4035); cpu.push(0x40b403a); f_040a30c0(cpu); if cpu.returned_to != 0x40b403a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40b4026u32; cpu.push(v); }
  { let v = 0x40b1fd8u32; cpu.push(v); }
  cpu.flat_ds(0x40b4047); cpu.push(0x40b404c); f_040a30c0(cpu); if cpu.returned_to != 0x40b404c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b4054); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4055(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4055); }
 let mut pc: u32 = 0x40b4055;
 loop { match pc {
 0x40b4055 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x415ace9u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x415ace9u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x415aca8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x415ace9u32, v); }
  pc = 0x40b40bb; continue;
 }
 0x40b4079 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x40b4083); f_040ad06c(cpu); if cpu.returned_to != 0x40b4083 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b40a8; continue; }
  pc = 0x40b4089; continue;
 }
 0x40b4089 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x40b4093); f_040ad06c(cpu); if cpu.returned_to != 0x40b4093 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b40a8; continue; }
  pc = 0x40b4099; continue;
 }
 0x40b4099 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x414b6f0u32; cpu.eax = v; }
  cpu.push(0x40b40a8); f_040ad382(cpu); if cpu.returned_to != 0x40b40a8 { return; }
  pc = 0x40b40a8; continue;
 }
 0x40b40a8 => {
  { let v: u32 = cpu.r32(0x42c61d0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c61d0u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b40bb; continue;
 }
 0x40b40bb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4079; continue; }
  pc = 0x40b40c2; continue;
 }
 0x40b40c2 => {
  { let v: u32 = cpu.edx; cpu.w32(0x42c569cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b40cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b40cc); }
 let mut pc: u32 = 0x40b40cc;
 loop { match pc {
 0x40b40cc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x40b40d3); f_040b40dd(cpu); if cpu.returned_to != 0x40b40d3 { return; }
  pc = 0x40b53fe; continue;
 }
 0x40b53fe => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x40b53ff; continue;
 }
 0x40b53ff => {
  { let v: u32 = cpu.r32(0x42c569cu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b541a; continue; }
  pc = 0x40b5409; continue;
 }
 0x40b5409 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x40b5412); f_040ad14f(cpu); if cpu.returned_to != 0x40b5412 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c569cu32, v); }
  cpu.poll(); pc = 0x40b53ff; continue;
 }
 0x40b541a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b40dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b40dd); }
 let mut pc: u32 = 0x40b40dd;
 loop { match pc {
 0x40b40dd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x415aca8u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42c61d0u32); cpu.eax = v; }
  pc = 0x40b412a; continue;
 }
 0x40b40ff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x40b4122; continue; }
  pc = 0x40b410f; continue;
 }
 0x40b410f => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x40b4122; continue; }
  pc = 0x40b4115; continue;
 }
 0x40b4115 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x40b4128; continue; }
  pc = 0x40b4119; continue;
 }
 0x40b4119 => {
  { cpu.sub32(cpu.eax, 0x415ad2au32); }
  if !cpu.cf { pc = 0x40b4122; continue; }
  pc = 0x40b4120; continue;
 }
 0x40b4120 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40b4122; continue;
 }
 0x40b4122 => {
  cpu.push(0x40b4127); f_040b5449(cpu); if cpu.returned_to != 0x40b4127 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b4128; continue;
 }
 0x40b4128 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40b412a; continue;
 }
 0x40b412a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b40ff; continue; }
  pc = 0x40b412e; continue;
 }
 0x40b412e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4135(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4135); }
 let mut pc: u32 = 0x40b4135;
 loop { match pc {
 0x40b4135 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x40b4147); f_040b18ea(cpu); if cpu.returned_to != 0x40b4147 { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x40b417a; continue; }
  pc = 0x40b414b; continue;
 }
 0x40b414b => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.rcl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b417a; continue; }
  pc = 0x40b416d; continue;
 }
 0x40b416d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40b4178); f_040b1852(cpu); if cpu.returned_to != 0x40b4178 { return; }
  pc = 0x40b41a1; continue;
 }
 0x40b417a => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40b416d; continue; }
  pc = 0x40b4191; continue;
 }
 0x40b4191 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x40b419f; continue; }
  pc = 0x40b4195; continue;
 }
 0x40b4195 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x40b419f); f_0409ebfb(cpu); if cpu.returned_to != 0x40b419f { return; }
  pc = 0x40b419f; continue;
 }
 0x40b419f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b41a1; continue;
 }
 0x40b41a1 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b41a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b41a9); }
 let mut pc: u32 = 0x40b41a9;
 loop { match pc {
 0x40b41a9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca10u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b41ca; continue; }
  pc = 0x40b41c5; continue;
 }
 0x40b41c5 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b41d1; continue; }
  pc = 0x40b41ca; continue;
 }
 0x40b41ca => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40b41d3; continue;
 }
 0x40b41d1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b41d3; continue;
 }
 0x40b41d3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b420c; continue; }
  pc = 0x40b41d7; continue;
 }
 0x40b41d7 => {
  cpu.push(0x40b41dc); f_040b566c(cpu); if cpu.returned_to != 0x40b41dc { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b420c; continue; }
  pc = 0x40b41e1; continue;
 }
 0x40b41e1 => {
  { let v: u32 = 0x409db63u32; cpu.edx = v; }
  { let v: u32 = 0x409db58u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x409db58u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b420c; continue;
 }
 0x40b420c => {
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
pub fn f_040b4212(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4212); }
 let mut pc: u32 = 0x40b4212;
 loop { match pc {
 0x40b4212 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4213); }
 let mut pc: u32 = 0x40b4213;
 loop { match pc {
 0x40b4213 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x415c9c8u32); cpu.push(0x40b421c); cpu.call(t); if cpu.returned_to != 0x40b421c { return; }
  let t = cpu.r32(0x415c9ccu32); cpu.push(0x40b4222); cpu.call(t); if cpu.returned_to != 0x40b4222 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b4229); f_040b422b(cpu); if cpu.returned_to != 0x40b4229 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4242(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4242); }
 let mut pc: u32 = 0x40b4242;
 loop { match pc {
 0x40b4242 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40b4291; continue; }
  pc = 0x40b424c; continue;
 }
 0x40b424c => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x40b4291; continue; }
  pc = 0x40b4251; continue;
 }
 0x40b4251 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b4291; continue; }
  pc = 0x40b4266; continue;
 }
 0x40b4266 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b4275; continue; }
  pc = 0x40b4270; continue;
 }
 0x40b4270 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x40b4275; continue;
 }
 0x40b4275 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x40b4295; continue; }
  pc = 0x40b427b; continue;
 }
 0x40b427b => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4275; continue; }
  pc = 0x40b428e; continue;
 }
 0x40b428e => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b4291; continue;
 }
 0x40b4291 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b42e0; continue;
 }
 0x40b4295 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x40b42c3; continue; }
  pc = 0x40b42a5; continue;
 }
 0x40b42a5 => {
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
  pc = 0x40b42d8; continue;
 }
 0x40b42c3 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b42d8; continue;
 }
 0x40b42d8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x40b42e0; continue;
 }
 0x40b42e0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x40b42e4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b42ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b42ea); }
 let mut pc: u32 = 0x40b42ea;
 loop { match pc {
 0x40b42e5 => {
  pc = 0x40b43f0; continue;
 }
 0x40b42ea => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x40b42e5; continue; }
  pc = 0x40b42f4; continue;
 }
 0x40b42f4 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x40b42e5; continue; }
  pc = 0x40b42ff; continue;
 }
 0x40b42ff => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x40b432c; continue; }
  pc = 0x40b430d; continue;
 }
 0x40b430d => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x40b4315; continue; }
  pc = 0x40b4312; continue;
 }
 0x40b4312 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b4315; continue;
 }
 0x40b4315 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b43a8; continue;
 }
 0x40b432c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x40b4343; continue; }
  pc = 0x40b4335; continue;
 }
 0x40b4335 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b43a8; continue; }
  pc = 0x40b433a; continue;
 }
 0x40b433a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x40b43a8; continue; }
  pc = 0x40b4341; continue;
 }
 0x40b4341 => {
  pc = 0x40b4352; continue;
 }
 0x40b4343 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x40b43a8; continue; }
  pc = 0x40b434a; continue;
 }
 0x40b434a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b43a8; continue; }
  pc = 0x40b4352; continue;
 }
 0x40b4352 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x40b4387; continue; }
  pc = 0x40b4362; continue;
 }
 0x40b4362 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b436f; continue; }
  pc = 0x40b436d; continue;
 }
 0x40b436d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x40b436f; continue;
 }
 0x40b436f => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x40b4373; continue;
 }
 0x40b4373 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b43a8; continue; }
  pc = 0x40b437a; continue;
 }
 0x40b437a => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40b4387; continue; }
  pc = 0x40b437f; continue;
 }
 0x40b437f => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b4373; continue; }
  pc = 0x40b4387; continue;
 }
 0x40b4387 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x40b4391; continue; }
  pc = 0x40b438e; continue;
 }
 0x40b438e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x40b4391; continue;
 }
 0x40b4391 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x40b43a8; continue; }
  pc = 0x40b4395; continue;
 }
 0x40b4395 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x40b43a8; continue; }
  pc = 0x40b439c; continue;
 }
 0x40b439c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x40b43a8; continue; }
  pc = 0x40b43a3; continue;
 }
 0x40b43a3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x40b4391; continue;
 }
 0x40b43a8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x40b43c7; continue; }
  pc = 0x40b43b7; continue;
 }
 0x40b43b7 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x40b43c3; continue; }
  pc = 0x40b43c0; continue;
 }
 0x40b43c0 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b43c3; continue;
 }
 0x40b43c3 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x40b43d8; continue;
 }
 0x40b43c7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b43d8; continue;
 }
 0x40b43d8 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x40b43e8; continue; }
  pc = 0x40b43e0; continue;
 }
 0x40b43e0 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x40b43e8; continue; }
  pc = 0x40b43e5; continue;
 }
 0x40b43e5 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b43e8; continue;
 }
 0x40b43e8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x40b43f0; continue; }
  pc = 0x40b43ed; continue;
 }
 0x40b43ed => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b43f0; continue;
 }
 0x40b43f0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x40b43f4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b43f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b43f5); }
 let mut pc: u32 = 0x40b43f5;
 loop { match pc {
 0x40b43f5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c734u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x40b440b; continue; }
  pc = 0x40b4402; continue;
 }
 0x40b4402 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415c734u32, v); }
  pc = 0x40b440b; continue;
 }
 0x40b440b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x42c61d4u32)); }
  if !cpu.zf { pc = 0x40b441b; continue; }
  pc = 0x40b4413; continue;
 }
 0x40b4413 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42c61d4u32, v); }
  pc = 0x40b441b; continue;
 }
 0x40b441b => {
  { let v: u32 = cpu.r32(0x415c730u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x40b442e; continue; }
  pc = 0x40b4425; continue;
 }
 0x40b4425 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415c730u32, v); }
  pc = 0x40b442e; continue;
 }
 0x40b442e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b443b; continue; }
  pc = 0x40b4438; continue;
 }
 0x40b4438 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b443b; continue;
 }
 0x40b443b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4442; continue; }
  pc = 0x40b443f; continue;
 }
 0x40b443f => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b4442; continue;
 }
 0x40b4442 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4446(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b4446); } r_040b4446(cpu, 0x40b4446); }
pub fn f_040b4665(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b4665); } r_040b4446(cpu, 0x40b4665); }
pub fn f_040b4736(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b4736); } r_040b4446(cpu, 0x40b4736); }
fn r_040b4446(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b4446 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c730u32); cpu.edx = v; }
  pc = 0x40b4451; continue;
 }
 0x40b4451 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b4730; continue; }
  pc = 0x40b4459; continue;
 }
 0x40b4459 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x40b449d; continue; }
  pc = 0x40b4467; continue;
 }
 0x40b4467 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x40b4474); f_040b43f5(cpu); if cpu.returned_to != 0x40b4474 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40b448d; continue; }
  pc = 0x40b447d; continue;
 }
 0x40b447d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x40b4451; continue;
 }
 0x40b448d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x40b4451; continue;
 }
 0x40b449d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x40b4451; continue;
 }
 0x40b4665 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x40b4679); f_040b4446(cpu); if cpu.returned_to != 0x40b4679 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40b4694; continue; }
  pc = 0x40b4684; continue;
 }
 0x40b4684 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b4694; continue;
 }
 0x40b4694 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b46b1; continue; }
  pc = 0x40b4698; continue;
 }
 0x40b4698 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b472d; continue;
 }
 0x40b46b1 => {
  { cpu.and16(cpu.r16(0x42c668eu32), 0xfff0u16); }
  if cpu.zf { pc = 0x40b46c6; continue; }
  pc = 0x40b46bc; continue;
 }
 0x40b46bc => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x42c668cu32, v); }
  pc = 0x40b46c6; continue;
 }
 0x40b46c6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b472b; continue; }
  pc = 0x40b46cf; continue;
 }
 0x40b46cf => {
  { let v: u32 = cpu.r32(0x42c668cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b472b; continue; }
  pc = 0x40b46e9; continue;
 }
 0x40b46e9 => {
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
  if (cpu.sf != cpu.of) { pc = 0x40b472b; continue; }
  pc = 0x40b470d; continue;
 }
 0x40b470d => {
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
  pc = 0x40b472d; continue;
 }
 0x40b472b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b472d; continue;
 }
 0x40b472d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x40b4730; continue;
 }
 0x40b4730 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4736 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ca30u32), 0x0u32); }
  if !cpu.zf { pc = 0x40b4749; continue; }
  pc = 0x40b4745; continue;
 }
 0x40b4745 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x40b472d; continue;
 }
 0x40b4749 => {
  { cpu.sub32(cpu.r32(0x415b80cu32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x40b4745; continue; }
  pc = 0x40b4752; continue;
 }
 0x40b4752 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x40b4759); f_040b4932(cpu); if cpu.returned_to != 0x40b4759 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x40b472d; continue; }
  pc = 0x40b475d; continue;
 }
 0x40b475d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b477e; continue; }
  pc = 0x40b4769; continue;
 }
 0x40b4769 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x40b4771); f_040b4665(cpu); if cpu.returned_to != 0x40b4771 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4799; continue; }
  pc = 0x40b4777; continue;
 }
 0x40b4777 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40b4799; continue;
 }
 0x40b477e => {
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
  pc = 0x40b4799; continue;
 }
 0x40b4799 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x40b4745; continue; }
  pc = 0x40b479d; continue;
 }
 0x40b479d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40b4745; continue; }
  pc = 0x40b47aa; continue;
 }
 0x40b47aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x40b4745; continue; }
  pc = 0x40b47b2; continue;
 }
 0x40b47b2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b47bb); f_040b45a1(cpu); if cpu.returned_to != 0x40b47bb { return; }
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
  cpu.push(0x40b47dc); f_040ad14f(cpu); if cpu.returned_to != 0x40b47dc { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x40b472d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b44a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b44a2); }
 let mut pc: u32 = 0x40b44a2;
 loop { match pc {
 0x40b44a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x40b44b0); f_040b4446(cpu); if cpu.returned_to != 0x40b44b0 { return; }
  { let v: u32 = cpu.r32(0x415c730u32); cpu.edx = v; }
  pc = 0x40b4591; continue;
 }
 0x40b44bb => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b4599; continue; }
  pc = 0x40b44c3; continue;
 }
 0x40b44c3 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x40b458e; continue; }
  pc = 0x40b44ce; continue;
 }
 0x40b44ce => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x40b458e; continue; }
  pc = 0x40b44d8; continue;
 }
 0x40b44d8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b44df); f_040b43f5(cpu); if cpu.returned_to != 0x40b44df { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40b4599; continue; }
  pc = 0x40b44ec; continue;
 }
 0x40b44ec => {
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
  if cpu.zf { pc = 0x40b4530; continue; }
  pc = 0x40b4520; continue;
 }
 0x40b4520 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b4530; continue;
 }
 0x40b4530 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b459b; continue; }
  pc = 0x40b4534; continue;
 }
 0x40b4534 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40b454b); f_040b45a1(cpu); if cpu.returned_to != 0x40b454b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x40b4587; continue; }
  pc = 0x40b4560; continue;
 }
 0x40b4560 => {
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
  cpu.push(0x40b4585); f_040ad14f(cpu); if cpu.returned_to != 0x40b4585 { return; }
  pc = 0x40b458a; continue;
 }
 0x40b4587 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b458a; continue;
 }
 0x40b458a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40b459b; continue;
 }
 0x40b458e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x40b4591; continue;
 }
 0x40b4591 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40b44bb; continue; }
  pc = 0x40b4599; continue;
 }
 0x40b4599 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b459b; continue;
 }
 0x40b459b => {
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
pub fn f_040b45a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b45a1); }
 let mut pc: u32 = 0x40b45a1;
 loop { match pc {
 0x40b45a1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415c730u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40b45b7; continue;
 }
 0x40b45ae => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x40b45bb; continue; }
  pc = 0x40b45b2; continue;
 }
 0x40b45b2 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40b45b7; continue;
 }
 0x40b45b7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b45ae; continue; }
  pc = 0x40b45bb; continue;
 }
 0x40b45bb => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b45ca; continue; }
  pc = 0x40b45c5; continue;
 }
 0x40b45c5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b45d0; continue;
 }
 0x40b45ca => {
  { let v: u32 = cpu.edx; cpu.w32(0x415c730u32, v); }
  pc = 0x40b45d0; continue;
 }
 0x40b45d0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b45d7; continue; }
  pc = 0x40b45d4; continue;
 }
 0x40b45d4 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b45d7; continue;
 }
 0x40b45d7 => {
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
pub fn f_040b4615(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4615); }
 let mut pc: u32 = 0x40b4615;
 loop { match pc {
 0x40b4615 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c730u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b4660; continue; }
  pc = 0x40b4621; continue;
 }
 0x40b4621 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b4653; continue; }
  pc = 0x40b4637; continue;
 }
 0x40b4637 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4653; continue; }
  pc = 0x40b463c; continue;
 }
 0x40b463c => {
  { cpu.sub8(cpu.r8(0x415b83cu32), 0x0u8); }
  if !cpu.zf { pc = 0x40b4653; continue; }
  pc = 0x40b4645; continue;
 }
 0x40b4645 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x40b464a, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b80cu32, v); }
  pc = 0x40b4653; continue;
 }
 0x40b4653 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x415b80cu32)); }
  if !cpu.zf { pc = 0x40b4660; continue; }
  pc = 0x40b465b; continue;
 }
 0x40b465b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4660 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b47e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b47e6); }
 let mut pc: u32 = 0x40b47e6;
 loop { match pc {
 0x40b47e6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b4801; continue; }
  pc = 0x40b47f8; continue;
 }
 0x40b47f8 => {
  { cpu.sub8(cpu.r8(0x415b83bu32), 0x0u8); }
  if cpu.zf { pc = 0x40b480d; continue; }
  pc = 0x40b4801; continue;
 }
 0x40b4801 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x40b481a; continue; }
  pc = 0x40b480d; continue;
 }
 0x40b480d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x40b4815); f_040b4736(cpu); if cpu.returned_to != 0x40b4815 { return; }
  pc = 0x40b4929; continue;
 }
 0x40b481a => {
  { cpu.sub32(cpu.r32(0x415ca30u32), 0x0u32); }
  if !cpu.zf { pc = 0x40b482a; continue; }
  pc = 0x40b4823; continue;
 }
 0x40b4823 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b4929; continue;
 }
 0x40b482a => {
  { cpu.sub32(cpu.r32(0x415b80cu32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b4833; continue;
 }
 0x40b4833 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x40b483a); f_040b4932(cpu); if cpu.returned_to != 0x40b483a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4929; continue; }
  pc = 0x40b4842; continue;
 }
 0x40b4842 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b486a; continue; }
  pc = 0x40b484e; continue;
 }
 0x40b484e => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b486a; continue; }
  pc = 0x40b4853; continue;
 }
 0x40b4853 => {
  { cpu.sub8(cpu.r8(0x415b83cu32), 0x0u8); }
  if !cpu.zf { pc = 0x40b486a; continue; }
  pc = 0x40b485c; continue;
 }
 0x40b485c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x40b4861, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b80cu32, v); }
  pc = 0x40b486a; continue;
 }
 0x40b486a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415b80cu32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x40b487e; continue; }
  pc = 0x40b4879; continue;
 }
 0x40b4879 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x40b487e; continue;
 }
 0x40b487e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b4885); f_040b5bfc(cpu); if cpu.returned_to != 0x40b4885 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b488a; continue;
 }
 0x40b488a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b4891; continue;
 }
 0x40b4891 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b4895; continue;
 }
 0x40b4895 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b48a1; continue;
 }
 0x40b48a1 => {
  { let v: u32 = cpu.r32(0x415c730u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40b48be; continue;
 }
 0x40b48ac => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40b48c2; continue; }
  pc = 0x40b48b3; continue;
 }
 0x40b48b3 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b48bb; continue; }
  pc = 0x40b48b7; continue;
 }
 0x40b48b7 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x40b48c2; continue; }
  pc = 0x40b48bb; continue;
 }
 0x40b48bb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x40b48be; continue;
 }
 0x40b48be => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x40b48ac; continue; }
  pc = 0x40b48c2; continue;
 }
 0x40b48c2 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b48ec; continue; }
  pc = 0x40b48c6; continue;
 }
 0x40b48c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x40b48ec; continue; }
  pc = 0x40b48d1; continue;
 }
 0x40b48d1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40b490b; continue;
 }
 0x40b48ec => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x40b4823; continue; }
  pc = 0x40b48f6; continue;
 }
 0x40b48f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x40b4904); f_040b45a1(cpu); if cpu.returned_to != 0x40b4904 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40b490b; continue;
 }
 0x40b490b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x40b4924); f_040ad14f(cpu); if cpu.returned_to != 0x40b4924 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40b4929; continue;
 }
 0x40b4929 => {
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
pub fn f_040b4932(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4932); }
 let mut pc: u32 = 0x40b4932;
 loop { match pc {
 0x40b4932 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40b4949; continue; }
  pc = 0x40b4943; continue;
 }
 0x40b4943 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4949 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b495e; continue; }
  pc = 0x40b4955; continue;
 }
 0x40b4955 => {
  { cpu.sub8(cpu.r8(0x415b83bu32), 0x0u8); }
  if cpu.zf { pc = 0x40b496a; continue; }
  pc = 0x40b495e; continue;
 }
 0x40b495e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x40b496f; continue; }
  pc = 0x40b496a; continue;
 }
 0x40b496a => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x40b4976; continue;
 }
 0x40b496f => {
  cpu.push(0x40b4974); f_040b4615(cpu); if cpu.returned_to != 0x40b4974 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x40b4976; continue;
 }
 0x40b4976 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x40b4943; continue; }
  pc = 0x40b497f; continue;
 }
 0x40b497f => {
  { let v: u32 = cpu.r32(0x415ca10u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x40b498e; continue; }
  pc = 0x40b4989; continue;
 }
 0x40b4989 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b498e; continue;
 }
 0x40b498e => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x40b4943; continue; }
  pc = 0x40b499a; continue;
 }
 0x40b499a => {
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
pub fn f_040b49af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b49af); }
 let mut pc: u32 = 0x40b49af;
 loop { match pc {
 0x40b49af => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b49b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b49b2); }
 let mut pc: u32 = 0x40b49b2;
 loop { match pc {
 0x40b49b2 => {
  { cpu.sub8(cpu.r8(0x415c9dcu32), 0x0u8); }
  if cpu.zf { pc = 0x40b49d5; continue; }
  pc = 0x40b49bb; continue;
 }
 0x40b49bb => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x40b49c3, "Int3");
  pc = 0x40b49cc; continue;
 }
 0x40b49cc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b49d5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b49d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b49d8); }
 let mut pc: u32 = 0x40b49d8;
 loop { match pc {
 0x40b49d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x40b49d9; continue;
 }
 0x40b49d9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x40b49e5; continue; }
  pc = 0x40b49e1; continue;
 }
 0x40b49e1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x40b49d9; continue;
 }
 0x40b49e5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b49e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b49e7); }
 let mut pc: u32 = 0x40b49e7;
 loop { match pc {
 0x40b49e7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(8); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40b49fe; continue; }
  pc = 0x40b49f8; continue;
 }
 0x40b49f8 => {
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.edi = v; }
  pc = 0x40b49fe; continue;
 }
 0x40b49fe => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b4a1a; continue; }
  pc = 0x40b4a04; continue;
 }
 0x40b4a04 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x40b4a15; continue;
 }
 0x40b4a08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x40b4a0f); f_04094634(cpu); if cpu.returned_to != 0x40b4a0f { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x40b4a15; continue;
 }
 0x40b4a15 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4a08; continue; }
  pc = 0x40b4a1a; continue;
 }
 0x40b4a1a => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x40b4a2d; continue; }
  pc = 0x40b4a21; continue;
 }
 0x40b4a21 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x40b4a28); f_04094634(cpu); if cpu.returned_to != 0x40b4a28 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x40b4a2d; continue;
 }
 0x40b4a2d => {
  { let v: u32 = cpu.r32(0x415ca10u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x415ca10u32, v); }
  cpu.push(0x40b4a49); f_040ad06c(cpu); if cpu.returned_to != 0x40b4a49 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b4a7e; continue; }
  pc = 0x40b4a4d; continue;
 }
 0x40b4a4d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b4a54); f_040ad06c(cpu); if cpu.returned_to != 0x40b4a54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b4a7e; continue; }
  pc = 0x40b4a58; continue;
 }
 0x40b4a58 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x40b4a62); f_0409ebfb(cpu); if cpu.returned_to != 0x40b4a62 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x40b4a6c); f_0409ec1e(cpu); if cpu.returned_to != 0x40b4a6c { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415ca10u32, v); }
  pc = 0x40b4a74; continue;
 }
 0x40b4a74 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40b4b19; continue;
 }
 0x40b4a7e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca10u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b4aaf; continue; }
  pc = 0x40b4a9b; continue;
 }
 0x40b4a9b => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x40b4aaa; continue;
 }
 0x40b4a9f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.push(0x40b4aa6); f_040b49d8(cpu); if cpu.returned_to != 0x40b4aa6 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  pc = 0x40b4aaa; continue;
 }
 0x40b4aaa => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4a9f; continue; }
  pc = 0x40b4aaf; continue;
 }
 0x40b4aaf => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x40b4ac2; continue; }
  pc = 0x40b4ab9; continue;
 }
 0x40b4ab9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.push(0x40b4ac2); f_040945e1(cpu); if cpu.returned_to != 0x40b4ac2 { return; }
  pc = 0x40b4ac2; continue;
 }
 0x40b4ac2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40b4ae3; continue; }
  pc = 0x40b4ac9; continue;
 }
 0x40b4ac9 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x40b4add; continue;
 }
 0x40b4ace => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b4ad3; continue; }
  pc = 0x40b4ad2; continue;
 }
 0x40b4ad2 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40b4ad3; continue;
 }
 0x40b4ad3 => {
  cpu.push(0x40b4ad8); f_04094634(cpu); if cpu.returned_to != 0x40b4ad8 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x40b4add; continue;
 }
 0x40b4add => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4ace; continue; }
  pc = 0x40b4ae3; continue;
 }
 0x40b4ae3 => {
  { cpu.sub32(cpu.edx, 0x7eu32); }
  if (cpu.cf || cpu.zf) { pc = 0x40b4b0b; continue; }
  pc = 0x40b4ae8; continue;
 }
 0x40b4ae8 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x40b4af2); f_0409ebfb(cpu); if cpu.returned_to != 0x40b4af2 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x40b4afc); f_0409ec1e(cpu); if cpu.returned_to != 0x40b4afc { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40b4b06); f_040ad14f(cpu); if cpu.returned_to != 0x40b4b06 { return; }
  cpu.poll(); pc = 0x40b4a74; continue;
 }
 0x40b4b0b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x40b4b19; continue;
 }
 0x40b4b19 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4b1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4b1f); }
 let mut pc: u32 = 0x40b4b1f;
 loop { match pc {
 0x40b4b1f => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40b4b2e; continue; }
  pc = 0x40b4b2d; continue;
 }
 0x40b4b2d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b4b2e; continue;
 }
 0x40b4b2e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40b4b56; continue; }
  pc = 0x40b4b33; continue;
 }
 0x40b4b33 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40b4b56; continue; }
  pc = 0x40b4b3b; continue;
 }
 0x40b4b3b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.push(0x40b4b47); f_040b49d8(cpu); if cpu.returned_to != 0x40b4b47 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40b4b56; continue; }
  pc = 0x40b4b4e; continue;
 }
 0x40b4b4e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x40b4b3b; continue;
 }
 0x40b4b56 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40b4b60; continue; }
  pc = 0x40b4b5a; continue;
 }
 0x40b4b5a => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4b60 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = cpu.sub8((cpu.eax as u8), cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4b72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4b72); }
 let mut pc: u32 = 0x40b4b72;
 loop { match pc {
 0x40b4b72 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9e0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415c9e4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9ecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9f2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x415c9f0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x415c9f6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9f8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9fcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415c9e6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415c9eau32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x415ca04u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x415ca00u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415ca06u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x415c9e0u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x40b4bec; continue; }
  pc = 0x40b4be6; continue;
 }
 0x40b4be6 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x40b4bee; continue;
 }
 0x40b4bec => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b4bee; continue;
 }
 0x40b4bee => {
  { cpu.cf = false; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x415b834u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x4bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.r16(0x415ca04u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x415ca00u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v = cpu.r16(0x415ca06u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x0u32; cpu.w32(0x415b834u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x40b4c2d; continue; }
  pc = 0x40b4c29; continue;
 }
 0x40b4c29 => {
  { let v: u8 = 0x4du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x40b4c2d; continue;
 }
 0x40b4c2d => {
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  cpu.flat_ds(0x40b4c32); cpu.push(0x40b4c37); f_040b184e(cpu); if cpu.returned_to != 0x40b4c37 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x40b4c3f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4c40); }
 let mut pc: u32 = 0x40b4c40;
 loop { match pc {
 0x40b4c40 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b4c4f; continue; }
  pc = 0x40b4c4a; continue;
 }
 0x40b4c4a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4c4f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x93u32); }
  if cpu.cf { pc = 0x40b4c62; continue; }
  pc = 0x40b4c5d; continue;
 }
 0x40b4c5d => {
  { let v: u32 = 0x92u32; cpu.edx = v; }
  pc = 0x40b4c62; continue;
 }
 0x40b4c62 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40b4c69); f_040b5d9b(cpu); if cpu.returned_to != 0x40b4c69 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b4c76); f_040b6068(cpu); if cpu.returned_to != 0x40b4c76 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b4c7f); f_040b5dd9(cpu); if cpu.returned_to != 0x40b4c7f { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b4c8c); f_040b5dd9(cpu); if cpu.returned_to != 0x40b4c8c { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4c92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4c92); }
 let mut pc: u32 = 0x40b4c92;
 loop { match pc {
 0x40b4c92 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40b4cd3; continue; }
  pc = 0x40b4ca4; continue;
 }
 0x40b4ca4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x40b4cd3; continue; }
  pc = 0x40b4cae; continue;
 }
 0x40b4cae => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b4cc3; continue; }
  pc = 0x40b4cb2; continue;
 }
 0x40b4cb2 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b4cc3; continue;
 }
 0x40b4cc3 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b4ccf); f_040b5e0a(cpu); if cpu.returned_to != 0x40b4ccf { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b4cde; continue;
 }
 0x40b4cd3 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b4cde; continue; }
  pc = 0x40b4cd7; continue;
 }
 0x40b4cd7 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40b4cde; continue;
 }
 0x40b4cde => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40b4ce4; continue;
 }
 0x40b4ce4 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40b4ce6; continue;
 }
 0x40b4ce6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b4ced); f_040b1ace(cpu); if cpu.returned_to != 0x40b4ced { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4d2e; continue; }
  pc = 0x40b4cf3; continue;
 }
 0x40b4cf3 => {
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x40b4cfd; continue; }
  pc = 0x40b4cf8; continue;
 }
 0x40b4cf8 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40b4ce6; continue;
 }
 0x40b4cfd => {
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b4d19; continue; }
  pc = 0x40b4d06; continue;
 }
 0x40b4d06 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40b4d1b; continue;
 }
 0x40b4d19 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b4d1b; continue;
 }
 0x40b4d1b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x40b4d28; continue; }
  pc = 0x40b4d23; continue;
 }
 0x40b4d23 => {
  { cpu.sub32(cpu.edx, 0x2fu32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4ce6; continue; }
  pc = 0x40b4d28; continue;
 }
 0x40b4d28 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40b4ce4; continue;
 }
 0x40b4d2e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x40b4d3c); f_040b4c40(cpu); if cpu.returned_to != 0x40b4d3c { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40b4d44; continue; }
  pc = 0x40b4d42; continue;
 }
 0x40b4d42 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x40b4d44; continue;
 }
 0x40b4d44 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x40b4d55); f_040b4c40(cpu); if cpu.returned_to != 0x40b4d55 { return; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x40b4d64); f_040b4c40(cpu); if cpu.returned_to != 0x40b4d64 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4d6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4d6d); }
 let mut pc: u32 = 0x40b4d6d;
 loop { match pc {
 0x40b4d6d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.ecx = v; }
  cpu.push(0x40b4d87); f_040ad417(cpu); if cpu.returned_to != 0x40b4d87 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4d8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4d8b); }
 let mut pc: u32 = 0x40b4d8b;
 loop { match pc {
 0x40b4d8b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.ecx = v; }
  cpu.push(0x40b4d97); f_040b613b(cpu); if cpu.returned_to != 0x40b4d97 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4d9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b4d9f); } r_040b4d9f(cpu, 0x40b4d9f); }
pub fn f_040b4e42(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b4e42); } r_040b4d9f(cpu, 0x40b4e42); }
fn r_040b4d9f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b4d9f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b4db8; continue; }
  pc = 0x40b4db2; continue;
 }
 0x40b4db2 => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x40b4dba; continue;
 }
 0x40b4db8 => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b4dba; continue;
 }
 0x40b4dba => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40b4ddd; continue; }
  pc = 0x40b4dc8; continue;
 }
 0x40b4dc8 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40b4ddd; continue; }
  pc = 0x40b4dd6; continue;
 }
 0x40b4dd6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x40b4ddd; continue;
 }
 0x40b4ddd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b4ded; continue; }
  pc = 0x40b4de1; continue;
 }
 0x40b4de1 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x40b4ded; continue; }
  pc = 0x40b4deb; continue;
 }
 0x40b4deb => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b4ded; continue;
 }
 0x40b4ded => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4e39; continue; }
  pc = 0x40b4df5; continue;
 }
 0x40b4df5 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4e36; continue; }
  pc = 0x40b4df9; continue;
 }
 0x40b4df9 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b4dfb; continue;
 }
 0x40b4dfb => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b4e06; continue; }
  pc = 0x40b4e00; continue;
 }
 0x40b4e00 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x40b4dfb; continue;
 }
 0x40b4e06 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4e06; continue; }
  pc = 0x40b4e34; continue;
 }
 0x40b4e34 => {
  pc = 0x40b4e39; continue;
 }
 0x40b4e36 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x40b4e39; continue;
 }
 0x40b4e39 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x40b4e40; continue;
 }
 0x40b4e40 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4e42 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40b4e57; continue; }
  pc = 0x40b4e49; continue;
 }
 0x40b4e49 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4e57; continue; }
  pc = 0x40b4e4d; continue;
 }
 0x40b4e4d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b4e57 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b4e68; continue; }
  pc = 0x40b4e5b; continue;
 }
 0x40b4e5b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x40b4e68; continue; }
  pc = 0x40b4e61; continue;
 }
 0x40b4e61 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40b4e57; continue;
 }
 0x40b4e68 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b4e75; continue; }
  pc = 0x40b4e6c; continue;
 }
 0x40b4e6c => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b4e6c; continue; }
  pc = 0x40b4e75; continue;
 }
 0x40b4e75 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b4e89; continue; }
  pc = 0x40b4e7d; continue;
 }
 0x40b4e7d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b4e89; continue; }
  pc = 0x40b4e81; continue;
 }
 0x40b4e81 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b4e7d; continue; }
  pc = 0x40b4e89; continue;
 }
 0x40b4e89 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40b4e40; continue; }
  pc = 0x40b4e8d; continue;
 }
 0x40b4e8d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x40b4e9a; continue; }
  pc = 0x40b4e93; continue;
 }
 0x40b4e93 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b4e8d; continue; }
  pc = 0x40b4e9a; continue;
 }
 0x40b4e9a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x40b4e40; continue; }
  pc = 0x40b4e9e; continue;
 }
 0x40b4e9e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x40b4e9a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4ea5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4ea5); }
 let mut pc: u32 = 0x40b4ea5;
 loop { match pc {
 0x40b4ea5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x40b4f0e; continue; }
  pc = 0x40b4eb6; continue;
 }
 0x40b4eb6 => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x40b4ec5; continue;
 }
 0x40b4ec5 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4ec5; continue; }
  pc = 0x40b4ed4; continue;
 }
 0x40b4ed4 => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x40b4ee4; continue; }
  pc = 0x40b4edd; continue;
 }
 0x40b4edd => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x40b4f02; continue;
 }
 0x40b4ee4 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x40b4eee; continue; }
  pc = 0x40b4ee9; continue;
 }
 0x40b4ee9 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x40b4f02; continue; }
  pc = 0x40b4eee; continue;
 }
 0x40b4eee => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x40b4f02; continue; }
  pc = 0x40b4ef8; continue;
 }
 0x40b4ef8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x40b4f02; continue;
 }
 0x40b4f02 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40b4f0e); f_0409e8e0(cpu); if cpu.returned_to != 0x40b4f0e { return; }
  pc = 0x40b4f0e; continue;
 }
 0x40b4f0e => {
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b4f16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b4f16); }
 let mut pc: u32 = 0x40b4f16;
 loop { match pc {
 0x40b4f16 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40b4f36); f_040b628b(cpu); if cpu.returned_to != 0x40b4f36 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4f56; continue; }
  pc = 0x40b4f3a; continue;
 }
 0x40b4f3a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40b4f49; continue;
 }
 0x40b4f41 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40b4f47; continue; }
  pc = 0x40b4f45; continue;
 }
 0x40b4f45 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b4f47; continue;
 }
 0x40b4f47 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x40b4f49; continue;
 }
 0x40b4f49 => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40b4f41; continue; }
  pc = 0x40b4f51; continue;
 }
 0x40b4f51 => {
  pc = 0x40b5133; continue;
 }
 0x40b4f56 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x40b4fcb; continue; }
  pc = 0x40b4f64; continue;
 }
 0x40b4f64 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b4f7d; continue; }
  pc = 0x40b4f79; continue;
 }
 0x40b4f79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b4fc4; continue; }
  pc = 0x40b4f7d; continue;
 }
 0x40b4f7d => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x40b4f85); f_040b6365(cpu); if cpu.returned_to != 0x40b4f85 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40b4f95); f_040b638d(cpu); if cpu.returned_to != 0x40b4f95 { return; }
  cpu.push(0x40b4f9a); f_0408cec2(cpu); if cpu.returned_to != 0x40b4f9a { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x40b4fa9; continue; }
  pc = 0x40b4fa5; continue;
 }
 0x40b4fa5 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4fad; continue; }
  pc = 0x40b4fa9; continue;
 }
 0x40b4fa9 => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b4fcb; continue;
 }
 0x40b4fad => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b4fba; continue; }
  pc = 0x40b4fb7; continue;
 }
 0x40b4fb7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b4fba; continue;
 }
 0x40b4fba => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40b4fc4; continue; }
  pc = 0x40b4fc3; continue;
 }
 0x40b4fc3 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b4fc4; continue;
 }
 0x40b4fc4 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b4fcb; continue;
 }
 0x40b4fcb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x40b4ffa; continue; }
  pc = 0x40b4fd4; continue;
 }
 0x40b4fd4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40b4fe6; continue; }
  pc = 0x40b4fdf; continue;
 }
 0x40b4fdf => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b4ffa; continue; }
  pc = 0x40b4fe6; continue;
 }
 0x40b4fe6 => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x40b4ff5); f_0409e8e0(cpu); if cpu.returned_to != 0x40b4ff5 { return; }
  pc = 0x40b5147; continue;
 }
 0x40b4ffa => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x40b5017; continue; }
  pc = 0x40b5006; continue;
 }
 0x40b5006 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b5012; continue; }
  pc = 0x40b500d; continue;
 }
 0x40b500d => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x40b5017; continue;
 }
 0x40b5012 => {
  if (cpu.sf == cpu.of) { pc = 0x40b5017; continue; }
  pc = 0x40b5014; continue;
 }
 0x40b5014 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x40b5017; continue;
 }
 0x40b5017 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b5030; continue; }
  pc = 0x40b502c; continue;
 }
 0x40b502c => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b5051; continue; }
  pc = 0x40b5030; continue;
 }
 0x40b5030 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x40b5051; continue; }
  pc = 0x40b5039; continue;
 }
 0x40b5039 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b5051; continue; }
  pc = 0x40b5040; continue;
 }
 0x40b5040 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40b504e); f_0409ec6b(cpu); if cpu.returned_to != 0x40b504e { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40b5051; continue;
 }
 0x40b5051 => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40b5070); f_040b63c9(cpu); if cpu.returned_to != 0x40b5070 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40b5089; continue; }
  pc = 0x40b507e; continue;
 }
 0x40b507e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b5096; continue;
 }
 0x40b5089 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x40b5096; continue; }
  pc = 0x40b508d; continue;
 }
 0x40b508d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x40b5096; continue;
 }
 0x40b5096 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x40b5108; continue; }
  pc = 0x40b50a2; continue;
 }
 0x40b50a2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x40b5131; continue; }
  pc = 0x40b50b1; continue;
 }
 0x40b50b1 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x40b50be); f_040b4e42(cpu); if cpu.returned_to != 0x40b50be { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40b50da; continue; }
  pc = 0x40b50cf; continue;
 }
 0x40b50cf => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40b50da; continue; }
  pc = 0x40b50d3; continue;
 }
 0x40b50d3 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b50da; continue;
 }
 0x40b50da => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b50ec; continue; }
  pc = 0x40b50e8; continue;
 }
 0x40b50e8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b50f2; continue; }
  pc = 0x40b50ec; continue;
 }
 0x40b50ec => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b50f2; continue;
 }
 0x40b50f2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x40b5104); f_040b4d9f(cpu); if cpu.returned_to != 0x40b5104 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x40b5133; continue;
 }
 0x40b5108 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40b5131; continue; }
  pc = 0x40b511c; continue;
 }
 0x40b511c => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x40b512f); f_040b4e42(cpu); if cpu.returned_to != 0x40b512f { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x40b5131; continue;
 }
 0x40b5131 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b5133; continue;
 }
 0x40b5133 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b4fe6; continue; }
  pc = 0x40b513b; continue;
 }
 0x40b513b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x40b5147); f_040b4ea5(cpu); if cpu.returned_to != 0x40b5147 { return; }
  pc = 0x40b5147; continue;
 }
 0x40b5147 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5150); }
 let mut pc: u32 = 0x40b5150;
 loop { match pc {
 0x40b5150 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x40b517d; continue; }
  pc = 0x40b515b; continue;
 }
 0x40b515b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x40b5163); f_040b18c3(cpu); if cpu.returned_to != 0x40b5163 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b517d; continue; }
  pc = 0x40b5167; continue;
 }
 0x40b5167 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x40b517d; continue; }
  pc = 0x40b5175; continue;
 }
 0x40b5175 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x40b517d; continue;
 }
 0x40b517d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5181); }
 let mut pc: u32 = 0x40b5181;
 loop { match pc {
 0x40ad79d => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x40ad7a5); cpu.push(0x40ad7aa); f_040ad7ea(cpu); if cpu.returned_to != 0x40ad7aa { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x1eu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[4] as u16); cpu.w16(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x22u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x40ad7e9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b5181 => {
  cpu.poll(); pc = 0x40ad79d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5186(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5186); }
 let mut pc: u32 = 0x40b5186;
 loop { match pc {
 0x40b5186 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x40b51ae; continue; }
  pc = 0x40b5195; continue;
 }
 0x40b5195 => {
  { cpu.sub8(cpu.r8(0x415b83cu32), 0x0u8); }
  if cpu.zf { pc = 0x40b51ae; continue; }
  pc = 0x40b519e; continue;
 }
 0x40b519e => {
  { let v: u16 = cpu.r16(0x415ac86u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b51a4; continue;
 }
 0x40b51a4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x42c6288u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b51ae => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b51bf; continue; }
  pc = 0x40b51ba; continue;
 }
 0x40b51ba => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b51c9; continue; }
  pc = 0x40b51bf; continue;
 }
 0x40b51bf => {
  { let v: u8 = cpu.r8(0x415b83au32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x40b51d4; continue; }
  pc = 0x40b51c9; continue;
 }
 0x40b51c9 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x34u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x40b51a4; continue;
 }
 0x40b51d4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b51e6; continue; }
  pc = 0x40b51dd; continue;
 }
 0x40b51dd => {
  { cpu.sub8(cpu.r8(0x415b83bu32), 0x0u8); }
  if cpu.zf { pc = 0x40b51f2; continue; }
  pc = 0x40b51e6; continue;
 }
 0x40b51e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x40b51f7; continue; }
  pc = 0x40b51f2; continue;
 }
 0x40b51f2 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40b51a4; continue;
 }
 0x40b51f7 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b526d; continue; }
  pc = 0x40b51fc; continue;
 }
 0x40b51fc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b526d; continue; }
  pc = 0x40b5208; continue;
 }
 0x40b5208 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40b5234; continue; }
  pc = 0x40b5225; continue;
 }
 0x40b5225 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x414b720u32; cpu.eax = v; }
  cpu.push(0x40b5234); f_040ad382(cpu); if cpu.returned_to != 0x40b5234 { return; }
  pc = 0x40b5234; continue;
 }
 0x40b5234 => {
  { let v: u32 = 0xfffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x42c6288u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x40b5276; continue; }
  pc = 0x40b525a; continue;
 }
 0x40b525a => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x414b746u32; cpu.eax = v; }
  cpu.push(0x40b5269); f_040ad382(cpu); if cpu.returned_to != 0x40b5269 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b526d => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x42c6288u32, v); }
  pc = 0x40b5276; continue;
 }
 0x40b5276 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b527a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b527a); }
 let mut pc: u32 = 0x40b527a;
 loop { match pc {
 0x40b527a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b52a0; continue; }
  pc = 0x40b5287; continue;
 }
 0x40b5287 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b52a0; continue; }
  pc = 0x40b5293; continue;
 }
 0x40b5293 => {
  { let v: u16 = cpu.r16(0x42c6288u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x40b52a0; continue;
 }
 0x40b52a0 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b52a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b52a8); }
 let mut pc: u32 = 0x40b52a8;
 loop { match pc {
 0x40b52a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x40b52b9); f_040b65ab(cpu); if cpu.returned_to != 0x40b52b9 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x414ba28u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x40b52cb); f_040b65ca(cpu); if cpu.returned_to != 0x40b52cb { return; }
  { let v: u32 = 0x414b7f5u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x40b52d7); f_040b65ca(cpu); if cpu.returned_to != 0x40b52d7 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b52e0); f_040b65ca(cpu); if cpu.returned_to != 0x40b52e0 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x40b52ec); f_040b19b3(cpu); if cpu.returned_to != 0x40b52ec { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b52f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b52f0); }
 let mut pc: u32 = 0x40b52f0;
 loop { match pc {
 0x40b52f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x415ca0cu32); cpu.push(0x40b52fa); cpu.call(t); if cpu.returned_to != 0x40b52fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b5319; continue; }
  pc = 0x40b52fe; continue;
 }
 0x40b52fe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x40b5308); f_040b52a8(cpu); if cpu.returned_to != 0x40b5308 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x40b5314; continue; }
  pc = 0x40b530d; continue;
 }
 0x40b530d => {
  cpu.push(0x40b5312); f_0409ebf6(cpu); if cpu.returned_to != 0x40b5312 { return; }
  pc = 0x40b5319; continue;
 }
 0x40b5314 => {
  cpu.push(0x40b5319); f_0409ec07(cpu); if cpu.returned_to != 0x40b5319 { return; }
  pc = 0x40b5319; continue;
 }
 0x40b5319 => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b531f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b531f); }
 let mut pc: u32 = 0x40b531f;
 loop { match pc {
 0x40b531f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c569cu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b5347; continue; }
  pc = 0x40b532e; continue;
 }
 0x40b532e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c569cu32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x40b538e; continue;
 }
 0x40b5347 => {
  { let v: u32 = 0x415aca8u32; cpu.ecx = v; }
  pc = 0x40b536e; continue;
 }
 0x40b534e => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x40b536b; continue; }
  pc = 0x40b5354; continue;
 }
 0x40b5354 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x40b535e); f_040ad06c(cpu); if cpu.returned_to != 0x40b535e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b53b3; continue; }
  pc = 0x40b5364; continue;
 }
 0x40b5364 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x40b538e; continue;
 }
 0x40b536b => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x40b536e; continue;
 }
 0x40b536e => {
  { cpu.sub32(cpu.ecx, 0x415aeb0u32); }
  if cpu.cf { cpu.poll(); pc = 0x40b534e; continue; }
  pc = 0x40b5376; continue;
 }
 0x40b5376 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x40b5385); f_040ad06c(cpu); if cpu.returned_to != 0x40b5385 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b53b3; continue; }
  pc = 0x40b538b; continue;
 }
 0x40b538b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x40b538e; continue;
 }
 0x40b538e => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x40b539c); f_0409e8e0(cpu); if cpu.returned_to != 0x40b539c { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x42c61d0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x42c61d0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40b53bf; continue;
 }
 0x40b53b3 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x40b53bd); f_0409ebfb(cpu); if cpu.returned_to != 0x40b53bd { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40b53bf; continue;
 }
 0x40b53bf => {
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
pub fn f_040b53c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b53c5); }
 let mut pc: u32 = 0x40b53c5;
 loop { match pc {
 0x40b53c5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x42c61d0u32; cpu.edx = v; }
  pc = 0x40b53cf; continue;
 }
 0x40b53cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b53fa; continue; }
  pc = 0x40b53d5; continue;
 }
 0x40b53d5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x40b53e0; continue; }
  pc = 0x40b53dc; continue;
 }
 0x40b53dc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x40b53cf; continue;
 }
 0x40b53e0 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x42c569cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c569cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x40b53fa; continue;
 }
 0x40b53fa => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b541c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b541c); }
 let mut pc: u32 = 0x40b541c;
 loop { match pc {
 0x40b541c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c61d0u32); cpu.eax = v; }
  pc = 0x40b5425; continue;
 }
 0x40b5425 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b5431; continue; }
  pc = 0x40b5429; continue;
 }
 0x40b5429 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b5431 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x40b543a; continue; }
  pc = 0x40b5436; continue;
 }
 0x40b5436 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x40b5425; continue;
 }
 0x40b543a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b5446); f_040b5449(cpu); if cpu.returned_to != 0x40b5446 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5449); }
 let mut pc: u32 = 0x40b5449;
 loop { match pc {
 0x40b5449 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x40b5451); f_040b54cf(cpu); if cpu.returned_to != 0x40b5451 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b545a); f_040b53c5(cpu); if cpu.returned_to != 0x40b545a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b545e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b545e); }
 let mut pc: u32 = 0x40b545e;
 loop { match pc {
 0x40b545e => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b5469; continue; }
  pc = 0x40b5466; continue;
 }
 0x40b5466 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x40b5469; continue;
 }
 0x40b5469 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b546a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b546a); }
 let mut pc: u32 = 0x40b546a;
 loop { match pc {
 0x40b546a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x40b5477); f_040b6645(cpu); if cpu.returned_to != 0x40b5477 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x40b5481; continue;
 }
 0x40b5481 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x40b548b); f_040b545e(cpu); if cpu.returned_to != 0x40b548b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b5481; continue; }
  pc = 0x40b5495; continue;
 }
 0x40b5495 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x40b54a6); f_040b545e(cpu); if cpu.returned_to != 0x40b54a6 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x40b54b3); f_040b545e(cpu); if cpu.returned_to != 0x40b54b3 { return; }
  { let v: u8 = 0x2eu8; cpu.w8(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x74u8; cpu.w8(0x9u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x6du8; cpu.w8(0xau32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x70u8; cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x7u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b54cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b54cf); }
 let mut pc: u32 = 0x40b54cf;
 loop { match pc {
 0x40b54cf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x40b54eb; continue; }
  pc = 0x40b54e1; continue;
 }
 0x40b54e1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40b5556; continue;
 }
 0x40b54eb => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x40b54fe; continue; }
  pc = 0x40b54f5; continue;
 }
 0x40b54f5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b54fc); f_040accda(cpu); if cpu.returned_to != 0x40b54fc { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b54fe; continue;
 }
 0x40b54fe => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b5505); f_040b664b(cpu); if cpu.returned_to != 0x40b5505 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40b5518; continue; }
  pc = 0x40b550a; continue;
 }
 0x40b550a => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x40b5518); f_040a15dd(cpu); if cpu.returned_to != 0x40b5518 { return; }
  pc = 0x40b5518; continue;
 }
 0x40b5518 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b5526; continue; }
  pc = 0x40b551c; continue;
 }
 0x40b551c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40b5524); f_040b1954(cpu); if cpu.returned_to != 0x40b5524 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x40b5526; continue;
 }
 0x40b5526 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x40b553b; continue; }
  pc = 0x40b552c; continue;
 }
 0x40b552c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40b5534); f_040ad14f(cpu); if cpu.returned_to != 0x40b5534 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40b553b; continue;
 }
 0x40b553b => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x40b5554; continue; }
  pc = 0x40b5541; continue;
 }
 0x40b5541 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x40b554d); f_040b546a(cpu); if cpu.returned_to != 0x40b554d { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x40b5554); f_040a1620(cpu); if cpu.returned_to != 0x40b5554 { return; }
  pc = 0x40b5554; continue;
 }
 0x40b5554 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40b5556; continue;
 }
 0x40b5556 => {
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
pub fn f_040b555f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b555f); } r_040b555f(cpu, 0x40b555f); }
pub fn f_040b55f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b55f4); } r_040b555f(cpu, 0x40b55f4); }
fn r_040b555f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b555f => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x40b5569); cpu.push(0x40b556e); f_0409db58(cpu); if cpu.returned_to != 0x40b556e { return; }
  { let v: u32 = 0x42c727cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415b81cu32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x415b834u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x42c628cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40b55c3; continue; }
  pc = 0x40b55a2; continue;
 }
 0x40b55a2 => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40b4212u32); }
  if cpu.zf { pc = 0x40b55c3; continue; }
  pc = 0x40b55ae; continue;
 }
 0x40b55ae => {
  { cpu.sub32(cpu.eax, 0x40b58b6u32); }
  if cpu.zf { pc = 0x40b55c3; continue; }
  pc = 0x40b55b5; continue;
 }
 0x40b55b5 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x40b55bc); cpu.push(0x40b55c1); f_040b6781(cpu); if cpu.returned_to != 0x40b55c1 { return; }
  pc = 0x40b55d4; continue;
 }
 0x40b55c3 => {
  { let v: u16 = cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x40b55cf; continue;
 }
 0x40b55cf => {
  cpu.flat_ds(0x40b55cf); cpu.push(0x40b55d4); f_0409f970(cpu); if cpu.returned_to != 0x40b55d4 { return; }
  pc = 0x40b55d4; continue;
 }
 0x40b55d4 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x40b55f3); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x40b55f4 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x40b55fe); cpu.push(0x40b5603); f_0409db58(cpu); if cpu.returned_to != 0x40b5603 { return; }
  { let v: u32 = 0x42c727cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415b81cu32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x415b834u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x42c628cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40b565b; continue; }
  pc = 0x40b5637; continue;
 }
 0x40b5637 => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40b4212u32); }
  if cpu.zf { pc = 0x40b565b; continue; }
  pc = 0x40b5643; continue;
 }
 0x40b5643 => {
  { cpu.sub32(cpu.eax, 0x40b5700u32); }
  if cpu.zf { pc = 0x40b565b; continue; }
  pc = 0x40b564a; continue;
 }
 0x40b564a => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x40b5651); cpu.push(0x40b5656); f_040b6781(cpu); if cpu.returned_to != 0x40b5656 { return; }
  cpu.poll(); pc = 0x40b55d4; continue;
 }
 0x40b565b => {
  { let v: u16 = cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x40b55cf; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b566c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b566c); }
 let mut pc: u32 = 0x40b566c;
 loop { match pc {
 0x40b566c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x415ca2cu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40b56f3; continue; }
  pc = 0x40b5680; continue;
 }
 0x40b5680 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40b5697; continue; }
  pc = 0x40b568c; continue;
 }
 0x40b568c => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x40b56e9; continue; }
  pc = 0x40b5697; continue;
 }
 0x40b5697 => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x400u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x6u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esp)) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b56e9; continue; }
  pc = 0x40b56dd; continue;
 }
 0x40b56dd => {
  { let v: u32 = 0x1u32; cpu.w32(0x415ca2cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b56f3; continue;
 }
 0x40b56e9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x415ca2cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b56f3; continue;
 }
 0x40b56f3 => {
  { let v: u32 = cpu.r32(0x415ca2cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b56ff); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5700); }
 let mut pc: u32 = 0x40b5700;
 loop { match pc {
 0x40b5700 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40b571b; continue; }
  pc = 0x40b570d; continue;
 }
 0x40b570d => {
  { cpu.sub16(cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x40b57ee; continue; }
  pc = 0x40b571b; continue;
 }
 0x40b571b => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40b58b1u32); }
  if !cpu.zf { pc = 0x40b5735; continue; }
  pc = 0x40b5729; continue;
 }
 0x40b5729 => {
  { let v: u32 = 0x40b58b6u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b5747; continue;
 }
 0x40b5735 => {
  { cpu.sub32(cpu.ebx, 0x40b5700u32); }
  if !cpu.zf { pc = 0x40b5747; continue; }
  pc = 0x40b573d; continue;
 }
 0x40b573d => {
  { let v: u32 = 0x40b4212u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b5747; continue;
 }
 0x40b5747 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b5790; continue; }
  pc = 0x40b5753; continue;
 }
 0x40b5753 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b5790; continue; }
  pc = 0x40b5758; continue;
 }
 0x40b5758 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca24u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca20u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x40b57dd; continue;
 }
 0x40b5790 => {
  cpu.flat_ds(0x40b5790); cpu.push(0x40b5795); f_040b566c(cpu); if cpu.returned_to != 0x40b5795 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b57c6; continue; }
  pc = 0x40b579a; continue;
 }
 0x40b579a => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca24u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca20u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x40b57dd; continue;
 }
 0x40b57c6 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x40b57d8); cpu.push(0x40b57dd); f_0409e9de(cpu); if cpu.returned_to != 0x40b57dd { return; }
  pc = 0x40b57dd; continue;
 }
 0x40b57dd => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x415ca18u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x415ca14u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b57ee; continue;
 }
 0x40b57ee => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b57f2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b58b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b58b1); }
 let mut pc: u32 = 0x40b58b1;
 loop { match pc {
 0x40b58b1 => {
  cpu.push(0x40b58b6); f_040b5700(cpu); if cpu.returned_to != 0x40b58b6 { return; }
  f_040b58b6(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b58b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b58b6); }
 let mut pc: u32 = 0x40b58b6;
 loop { match pc {
 0x40b57f3 => {
  { cpu.sub32(cpu.ebx, 0x40b58b6u32); }
  if !cpu.zf { pc = 0x40b5805; continue; }
  pc = 0x40b57fb; continue;
 }
 0x40b57fb => {
  { let v: u32 = 0x40b4212u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b5805; continue;
 }
 0x40b5805 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b584e; continue; }
  pc = 0x40b5811; continue;
 }
 0x40b5811 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b584e; continue; }
  pc = 0x40b5816; continue;
 }
 0x40b5816 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca2au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca26u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x40b589b; continue;
 }
 0x40b584e => {
  cpu.flat_ds(0x40b584e); cpu.push(0x40b5853); f_040b566c(cpu); if cpu.returned_to != 0x40b5853 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b5884; continue; }
  pc = 0x40b5858; continue;
 }
 0x40b5858 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x415ca2au32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca26u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x40b589b; continue;
 }
 0x40b5884 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x40b5896); cpu.push(0x40b589b); f_0409e9de(cpu); if cpu.returned_to != 0x40b589b { return; }
  pc = 0x40b589b; continue;
 }
 0x40b589b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x415ca1eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x415ca1au32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b58ac; continue;
 }
 0x40b58ac => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b58b0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b58b6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40b58cd; continue; }
  pc = 0x40b58c3; continue;
 }
 0x40b58c3 => {
  { cpu.sub16(cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x40b58ac; continue; }
  pc = 0x40b58cd; continue;
 }
 0x40b58cd => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40b58b1u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b57f3; continue; }
  pc = 0x40b58df; continue;
 }
 0x40b58df => {
  { let v: u32 = 0x40b5700u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x40b5805; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b58ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b58ee); }
 let mut pc: u32 = 0x40b58ee;
 loop { match pc {
 0x40b58ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ca14u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40b5a05; continue; }
  pc = 0x40b58fe; continue;
 }
 0x40b58fe => {
  { cpu.sub16(cpu.r16(0x415ca18u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x40b5a05; continue; }
  pc = 0x40b590c; continue;
 }
 0x40b590c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b596b; continue; }
  pc = 0x40b5918; continue;
 }
 0x40b5918 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b596b; continue; }
  pc = 0x40b591d; continue;
 }
 0x40b591d => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca18u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca14u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x40b555fu32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca20u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x40b59d7; continue;
 }
 0x40b596b => {
  cpu.flat_ds(0x40b596b); cpu.push(0x40b5970); f_040b566c(cpu); if cpu.returned_to != 0x40b5970 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b59b0; continue; }
  pc = 0x40b5975; continue;
 }
 0x40b5975 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca18u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca14u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x40b555fu32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x40b59d7; continue;
 }
 0x40b59b0 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x40b555fu32; cpu.ebx = v; }
  cpu.flat_ds(0x40b59ba); cpu.push(0x40b59bf); f_0409e9ac(cpu); if cpu.returned_to != 0x40b59bf { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415ca14u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x415ca18u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x40b59d2); cpu.push(0x40b59d7); f_0409e9de(cpu); if cpu.returned_to != 0x40b59d7 { return; }
  pc = 0x40b59d7; continue;
 }
 0x40b59d7 => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40b4212u32); }
  if !cpu.zf { pc = 0x40b59f3; continue; }
  pc = 0x40b59e5; continue;
 }
 0x40b59e5 => {
  { let v: u32 = 0x40b5700u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b59f2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b59f3 => {
  { cpu.sub32(cpu.ebx, 0x40b58b6u32); }
  if !cpu.zf { pc = 0x40b5a05; continue; }
  pc = 0x40b59fb; continue;
 }
 0x40b59fb => {
  { let v: u32 = 0x40b58b1u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b5a05; continue;
 }
 0x40b5a05 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b5a08); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5a09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5a09); }
 let mut pc: u32 = 0x40b5a09;
 loop { match pc {
 0x40b5a09 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ca1au32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x40b5b20; continue; }
  pc = 0x40b5a19; continue;
 }
 0x40b5a19 => {
  { cpu.sub16(cpu.r16(0x415ca1eu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x40b5b20; continue; }
  pc = 0x40b5a27; continue;
 }
 0x40b5a27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b5a86; continue; }
  pc = 0x40b5a33; continue;
 }
 0x40b5a33 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b5a86; continue; }
  pc = 0x40b5a38; continue;
 }
 0x40b5a38 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca1eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca1au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x40b55f4u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca2au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca26u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x40b5af2; continue;
 }
 0x40b5a86 => {
  cpu.flat_ds(0x40b5a86); cpu.push(0x40b5a8b); f_040b566c(cpu); if cpu.returned_to != 0x40b5a8b { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b5acb; continue; }
  pc = 0x40b5a90; continue;
 }
 0x40b5a90 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca1eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca1au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca2au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca26u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x40b55f4u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x40b5af2; continue;
 }
 0x40b5acb => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x40b55f4u32; cpu.ebx = v; }
  cpu.flat_ds(0x40b5ad5); cpu.push(0x40b5ada); f_0409e9ac(cpu); if cpu.returned_to != 0x40b5ada { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415ca1au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x415ca1eu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x40b5aed); cpu.push(0x40b5af2); f_0409e9de(cpu); if cpu.returned_to != 0x40b5af2 { return; }
  pc = 0x40b5af2; continue;
 }
 0x40b5af2 => {
  { let v: u32 = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x40b4212u32); }
  if !cpu.zf { pc = 0x40b5b0e; continue; }
  pc = 0x40b5b00; continue;
 }
 0x40b5b00 => {
  { let v: u32 = 0x40b58b6u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b5b0d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b5b0e => {
  { cpu.sub32(cpu.ebx, 0x40b5700u32); }
  if !cpu.zf { pc = 0x40b5b20; continue; }
  pc = 0x40b5b16; continue;
 }
 0x40b5b16 => {
  { let v: u32 = 0x40b58b1u32; cpu.w32(0x415c9ccu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b5b20; continue;
 }
 0x40b5b20 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x40b5b23); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5bfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5bfc); }
 let mut pc: u32 = 0x40b5bfc;
 loop { match pc {
 0x40b5bfc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x415b820u32)); }
  if cpu.cf { pc = 0x40b5c67; continue; }
  pc = 0x40b5c0c; continue;
 }
 0x40b5c0c => {
  { cpu.sub8(cpu.r8(0x415b83au32), 0x0u8); }
  if !cpu.zf { pc = 0x40b5c79; continue; }
  pc = 0x40b5c15; continue;
 }
 0x40b5c15 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfu32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40b5c27; continue; }
  pc = 0x40b5c22; continue;
 }
 0x40b5c22 => {
  { let v: u32 = 0xfffffffu32; cpu.ebp = v; }
  pc = 0x40b5c27; continue;
 }
 0x40b5c27 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0xedu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.shl32(cpu.eax, 31u32); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi | cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40b5c54; continue; }
  pc = 0x40b5c44; continue;
 }
 0x40b5c44 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.edi; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b5c67; continue; }
  pc = 0x40b5c54; continue;
 }
 0x40b5c54 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x40b5c59; continue;
 }
 0x40b5c59 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40b5cae; continue; }
  pc = 0x40b5c67; continue;
 }
 0x40b5c67 => {
  cpu.push(0x40b5c6c); f_040a12e1(cpu); if cpu.returned_to != 0x40b5c6c { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40b5cbc; continue;
 }
 0x40b5c79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40b5c8e; continue; }
  pc = 0x40b5c89; continue;
 }
 0x40b5c89 => {
  { let v: u32 = 0xfffffu32; cpu.ebx = v; }
  pc = 0x40b5c8e; continue;
 }
 0x40b5c8e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b5ca9; continue; }
  pc = 0x40b5c9a; continue;
 }
 0x40b5c9a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b83bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40b5ca9; continue; }
  pc = 0x40b5ca6; continue;
 }
 0x40b5ca6 => {
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x40b5ca9; continue;
 }
 0x40b5ca9 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40b5c59; continue;
 }
 0x40b5cae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415b80cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b80cu32, v); }
  pc = 0x40b5cbc; continue;
 }
 0x40b5cbc => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
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
pub fn f_040b5cc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5cc7); }
 let mut pc: u32 = 0x40b5cc7;
 loop { match pc {
 0x40b5cc7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x415c9d4u32), 0x0u32); }
  if !cpu.zf { pc = 0x40b5d92; continue; }
  pc = 0x40b5cdc; continue;
 }
 0x40b5cdc => {
  { let v = cpu.r16(0x415b841u32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415b83du32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40b5cf2; continue;
 }
 0x40b5cf2 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x40b5d0a; continue; }
  pc = 0x40b5cf7; continue;
 }
 0x40b5cf7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x40b5d05; continue; }
  pc = 0x40b5d01; continue;
 }
 0x40b5d01 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x40b5cf7; continue;
 }
 0x40b5d05 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x40b5cf2; continue;
 }
 0x40b5d0a => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b5d13; continue; }
  pc = 0x40b5d0e; continue;
 }
 0x40b5d0e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40b5d13; continue;
 }
 0x40b5d13 => {
  cpu.push(0x40b5d18); f_040ad06c(cpu); if cpu.returned_to != 0x40b5d18 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b5d92; continue; }
  pc = 0x40b5d24; continue;
 }
 0x40b5d24 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x40b5d33); f_040ad06c(cpu); if cpu.returned_to != 0x40b5d33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b5d8b; continue; }
  pc = 0x40b5d37; continue;
 }
 0x40b5d37 => {
  { let v: u32 = cpu.eax; cpu.w32(0x415c9d4u32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40b5d46; continue;
 }
 0x40b5d46 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x40b5d66; continue; }
  pc = 0x40b5d4c; continue;
 }
 0x40b5d4c => {
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x40b5d55; continue;
 }
 0x40b5d55 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40b5d55; continue; }
  pc = 0x40b5d60; continue;
 }
 0x40b5d60 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40b5d46; continue;
 }
 0x40b5d66 => {
  { let v: u32 = cpu.r32(0x415c9d4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415c9d8u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x40b5d89); f_0409e8e0(cpu); if cpu.returned_to != 0x40b5d89 { return; }
  pc = 0x40b5d92; continue;
 }
 0x40b5d8b => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b5d92); f_040ad14f(cpu); if cpu.returned_to != 0x40b5d92 { return; }
  pc = 0x40b5d92; continue;
 }
 0x40b5d92 => {
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
pub fn f_040b5d9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5d9b); }
 let mut pc: u32 = 0x40b5d9b;
 loop { match pc {
 0x40b5d9b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b5dcf; continue; }
  pc = 0x40b5dad; continue;
 }
 0x40b5dad => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x40b5db2; continue;
 }
 0x40b5db2 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b5dbb); f_040b1c95(cpu); if cpu.returned_to != 0x40b5dbb { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b5dcb; continue; }
  pc = 0x40b5dc4; continue;
 }
 0x40b5dc4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b5dcb; continue; }
  pc = 0x40b5dc8; continue;
 }
 0x40b5dc8 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x40b5db2; continue;
 }
 0x40b5dcb => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40b5dd1; continue;
 }
 0x40b5dcf => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b5dd1; continue;
 }
 0x40b5dd1 => {
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
pub fn f_040b5dd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5dd9); }
 let mut pc: u32 = 0x40b5dd9;
 loop { match pc {
 0x40b5dd9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b5e02; continue; }
  pc = 0x40b5deb; continue;
 }
 0x40b5deb => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40b5dfe; continue; }
  pc = 0x40b5df1; continue;
 }
 0x40b5df1 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  cpu.push(0x40b5df8); f_040b1c95(cpu); if cpu.returned_to != 0x40b5df8 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b5deb; continue; }
  pc = 0x40b5dfe; continue;
 }
 0x40b5dfe => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x40b5e04; continue;
 }
 0x40b5e02 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x40b5e04; continue;
 }
 0x40b5e04 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5e0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5e0a); }
 let mut pc: u32 = 0x40b5e0a;
 loop { match pc {
 0x40b5e0a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b5e1f; continue; }
  pc = 0x40b5e16; continue;
 }
 0x40b5e16 => {
  cpu.push(0x40b5e1b); f_040b5dd9(cpu); if cpu.returned_to != 0x40b5e1b { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b5e1f => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b5f76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b5f76); }
 let mut pc: u32 = 0x40b5f76;
 loop { match pc {
 0x40b5f76 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b6055; continue; }
  pc = 0x40b5f93; continue;
 }
 0x40b5f93 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40b601a; continue; }
  pc = 0x40b5f9c; continue;
 }
 0x40b5f9c => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b5fa0; continue;
 }
 0x40b5fa0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40b5fb8; continue; }
  pc = 0x40b5fa6; continue;
 }
 0x40b5fa6 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b5fb6; continue; }
  pc = 0x40b5faa; continue;
 }
 0x40b5faa => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40b5fb8; continue; }
  pc = 0x40b5fb1; continue;
 }
 0x40b5fb1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40b5fa0; continue; }
  pc = 0x40b5fb6; continue;
 }
 0x40b5fb6 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x40b5fb8; continue;
 }
 0x40b5fb8 => {
  { let v: u32 = cpu.r32(0x415c844u32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.esi = v; }
  pc = 0x40b5fc9; continue;
 }
 0x40b5fc9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x40b6013; continue; }
  pc = 0x40b5fce; continue;
 }
 0x40b5fce => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x40b5ff3; continue; }
  pc = 0x40b5fd2; continue;
 }
 0x40b5fd2 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40b5feb; continue; }
  pc = 0x40b5fd6; continue;
 }
 0x40b5fd6 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x40b5fed; continue;
 }
 0x40b5feb => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x40b5fed; continue;
 }
 0x40b5fed => {
  if cpu.zf { pc = 0x40b5ff3; continue; }
  pc = 0x40b5fef; continue;
 }
 0x40b5fef => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x40b6013; continue;
 }
 0x40b5ff3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b600c; continue; }
  pc = 0x40b5ff7; continue;
 }
 0x40b5ff7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x40b600e; continue;
 }
 0x40b600c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40b600e; continue;
 }
 0x40b600e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.poll(); pc = 0x40b5fc9; continue;
 }
 0x40b6013 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x40b601a; continue; }
  pc = 0x40b6017; continue;
 }
 0x40b6017 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x40b601a; continue;
 }
 0x40b601a => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
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
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40b605e; continue;
 }
 0x40b6055 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x40b605e); f_040b6843(cpu); if cpu.returned_to != 0x40b605e { return; }
  pc = 0x40b605e; continue;
 }
 0x40b605e => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
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
pub fn f_040b6068(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6068); }
 let mut pc: u32 = 0x40b6068;
 loop { match pc {
 0x40b6068 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b6077); f_040b5dd9(cpu); if cpu.returned_to != 0x40b6077 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b6082); f_040b6087(cpu); if cpu.returned_to != 0x40b6082 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6087(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6087); }
 let mut pc: u32 = 0x40b6087;
 loop { match pc {
 0x40b5f6f => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6087 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40b6131; continue; }
  pc = 0x40b609a; continue;
 }
 0x40b609a => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
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
  { let v: u32 = cpu.r32(0x415c844u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b60c5; continue; }
  pc = 0x40b60b6; continue;
 }
 0x40b60b6 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  pc = 0x40b60c5; continue;
 }
 0x40b60c5 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40b60ce; continue; }
  pc = 0x40b60c9; continue;
 }
 0x40b60c9 => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40b60ce; continue; }
  pc = 0x40b60cd; continue;
 }
 0x40b60cd => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b60ce; continue;
 }
 0x40b60ce => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40b60f6; continue; }
  pc = 0x40b60d5; continue;
 }
 0x40b60d5 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x40b5f6f; continue;
 }
 0x40b60f6 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x40b60fb; continue;
 }
 0x40b60fb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b6113; continue; }
  pc = 0x40b6103; continue;
 }
 0x40b6103 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40b60fb; continue; }
  pc = 0x40b6113; continue;
 }
 0x40b6113 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x40b5f6f; continue; }
  pc = 0x40b611c; continue;
 }
 0x40b611c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.push(0x40b612c); f_0409e8e0(cpu); if cpu.returned_to != 0x40b612c { return; }
  cpu.poll(); pc = 0x40b5f6f; continue;
 }
 0x40b6131 => {
  cpu.push(0x40b6136); f_0408f3b4(cpu); if cpu.returned_to != 0x40b6136 { return; }
  cpu.poll(); pc = 0x40b5f6f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b613b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b613b); }
 let mut pc: u32 = 0x40b613b;
 loop { match pc {
 0x40b613b => {
  { let v = 0xb8u32; cpu.push(v); }
  cpu.push(0x40b6145); f_0409deaa(cpu); if cpu.returned_to != 0x40b6145 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(164); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40b615f); f_040ad417(cpu); if cpu.returned_to != 0x40b615f { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b616a; continue;
 }
 0x40b616a => {
  cpu.push(0x40b616f); f_040a12e1(cpu); if cpu.returned_to != 0x40b616f { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40b6182; continue; }
  pc = 0x40b6174; continue;
 }
 0x40b6174 => {
  cpu.push(0x40b6179); f_040a12e1(cpu); if cpu.returned_to != 0x40b6179 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b6182; continue;
 }
 0x40b6182 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b618e; continue;
 }
 0x40b618e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b6197; continue;
 }
 0x40b6197 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b61a4; continue;
 }
 0x40b61a4 => {
  { let v: u32 = 0x414b7fcu32; cpu.eax = v; }
  cpu.push(0x40b61ae); f_040ad1d1(cpu); if cpu.returned_to != 0x40b61ae { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b61b8; continue;
 }
 0x40b61b8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40b61bf); f_04094634(cpu); if cpu.returned_to != 0x40b61bf { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b61ce; continue;
 }
 0x40b61ce => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b61d7; continue;
 }
 0x40b61d7 => {
  { let v: u32 = 0x3bu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b61e3); f_0409464d(cpu); if cpu.returned_to != 0x40b61e3 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40b61f3; continue; }
  pc = 0x40b61e9; continue;
 }
 0x40b61e9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40b61f0); f_04094634(cpu); if cpu.returned_to != 0x40b61f0 { return; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x40b61f3; continue;
 }
 0x40b61f3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x6eu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x40b621a; continue; }
  pc = 0x40b61ff; continue;
 }
 0x40b61ff => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x40b6209); f_0409ebfb(cpu); if cpu.returned_to != 0x40b6209 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x40b6213); f_0409ec1e(cpu); if cpu.returned_to != 0x40b6213 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40b6281; continue;
 }
 0x40b621a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x40b6226); f_0409e987(cpu); if cpu.returned_to != 0x40b6226 { return; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x7eu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x40b6239; continue; }
  pc = 0x40b6235; continue;
 }
 0x40b6235 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40b6239; continue;
 }
 0x40b6239 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x40b6246); f_0409e987(cpu); if cpu.returned_to != 0x40b6246 { return; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x40b6254); f_040ad417(cpu); if cpu.returned_to != 0x40b6254 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b625b; continue;
 }
 0x40b625b => {
  cpu.push(0x40b6260); f_040a12e1(cpu); if cpu.returned_to != 0x40b6260 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x40b626f; continue; }
  pc = 0x40b6265; continue;
 }
 0x40b6265 => {
  cpu.push(0x40b626a); f_040a12e1(cpu); if cpu.returned_to != 0x40b626a { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b626f; continue;
 }
 0x40b626f => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3bu32); }
  if !cpu.zf { pc = 0x40b627f; continue; }
  pc = 0x40b6277; continue;
 }
 0x40b6277 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.poll(); pc = 0x40b61ce; continue;
 }
 0x40b627f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b6281; continue;
 }
 0x40b6281 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b628b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b628b); }
 let mut pc: u32 = 0x40b628b;
 loop { match pc {
 0x40b628b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff0u32); }
  if !cpu.zf { pc = 0x40b6313; continue; }
  pc = 0x40b62b8; continue;
 }
 0x40b62b8 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x40b62d1; continue; }
  pc = 0x40b62cb; continue;
 }
 0x40b62cb => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b62d1; continue;
 }
 0x40b62d1 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx | cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.ebx); cpu.edx = v; }
  if !cpu.zf { pc = 0x40b6304; continue; }
  pc = 0x40b62fc; continue;
 }
 0x40b62fc => {
  { let v: u32 = cpu.r32(0x414b804u32); cpu.edx = v; }
  pc = 0x40b630a; continue;
 }
 0x40b6304 => {
  { let v: u32 = cpu.r32(0x414b808u32); cpu.edx = v; }
  pc = 0x40b630a; continue;
 }
 0x40b630a => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40b6315; continue;
 }
 0x40b6313 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b6315; continue;
 }
 0x40b6315 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b631e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b631e); } r_040b631e(cpu, 0x40b631e); }
pub fn f_040b6365(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b6365); } r_040b631e(cpu, 0x40b6365); }
fn r_040b631e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b631e => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b6320; continue;
 }
 0x40b6320 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b6345; continue; }
  pc = 0x40b6333; continue;
 }
 0x40b6333 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40b6341); f_040b6882(cpu); if cpu.returned_to != 0x40b6341 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b635d; continue;
 }
 0x40b6345 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x40b634d; continue; }
  pc = 0x40b6349; continue;
 }
 0x40b6349 => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x40b6357; continue;
 }
 0x40b634d => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x40b6355; continue; }
  pc = 0x40b6351; continue;
 }
 0x40b6351 => {
  { cpu.fpush(1.0); }
  pc = 0x40b6357; continue;
 }
 0x40b6355 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x40b6357; continue;
 }
 0x40b6357 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x40b635d; continue;
 }
 0x40b635d => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6365 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x40b6320; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6369(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6369); }
 let mut pc: u32 = 0x40b6369;
 loop { match pc {
 0x40b6369 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x40b6372); f_040b631e(cpu); if cpu.returned_to != 0x40b6372 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b638d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b638d); }
 let mut pc: u32 = 0x40b638d;
 loop { match pc {
 0x40b638d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40b63a7); f_040b68be(cpu); if cpu.returned_to != 0x40b63a7 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40b63bc; continue; }
  pc = 0x40b63b0; continue;
 }
 0x40b63b0 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x414b80cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40b63bc; continue;
 }
 0x40b63bc => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b63c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b63c9); }
 let mut pc: u32 = 0x40b63c9;
 loop { match pc {
 0x40b63c9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x40b63fa); f_040b628b(cpu); if cpu.returned_to != 0x40b63fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b6405; continue; }
  pc = 0x40b63fe; continue;
 }
 0x40b63fe => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b659a; continue;
 }
 0x40b6405 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b642f; continue; }
  pc = 0x40b6427; continue;
 }
 0x40b6427 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b6557; continue; }
  pc = 0x40b642f; continue;
 }
 0x40b642f => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40b6447; continue; }
  pc = 0x40b6439; continue;
 }
 0x40b6439 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40b6447; continue;
 }
 0x40b6447 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40b6455); f_040b68d7(cpu); if cpu.returned_to != 0x40b6455 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b645f; continue; }
  pc = 0x40b645d; continue;
 }
 0x40b645d => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x40b645f; continue;
 }
 0x40b645f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x5u32); cpu.edx = v; }
  { let v: u32 = 0xau32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40b647f; continue; }
  pc = 0x40b647d; continue;
 }
 0x40b647d => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x40b647f; continue;
 }
 0x40b647f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x40b648b; continue; }
  pc = 0x40b6489; continue;
 }
 0x40b6489 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x40b648b; continue;
 }
 0x40b648b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40b6525; continue; }
  pc = 0x40b6493; continue;
 }
 0x40b6493 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b649d; continue; }
  pc = 0x40b6498; continue;
 }
 0x40b6498 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x40b649d; continue;
 }
 0x40b649d => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40b64b3); f_040b68be(cpu); if cpu.returned_to != 0x40b64b3 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x40b64c7; continue; }
  pc = 0x40b64bc; continue;
 }
 0x40b64bc => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b64c7; continue; }
  pc = 0x40b64c0; continue;
 }
 0x40b64c0 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b64c7; continue;
 }
 0x40b64c7 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40b64db); f_0409ec2a(cpu); if cpu.returned_to != 0x40b64db { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b6525; continue; }
  pc = 0x40b64e1; continue;
 }
 0x40b64e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x40b64ef; continue; }
  pc = 0x40b64e9; continue;
 }
 0x40b64e9 => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x40b64f1; continue; }
  pc = 0x40b64ef; continue;
 }
 0x40b64ef => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x40b64f1; continue;
 }
 0x40b64f1 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40b64fd; continue; }
  pc = 0x40b64f6; continue;
 }
 0x40b64f6 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x40b6507; continue;
 }
 0x40b64fd => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b6507; continue; }
  pc = 0x40b6502; continue;
 }
 0x40b6502 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x40b6507; continue;
 }
 0x40b6507 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x40b651b); f_0409ec2a(cpu); if cpu.returned_to != 0x40b651b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b6525; continue; }
  pc = 0x40b651f; continue;
 }
 0x40b651f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40b6525; continue;
 }
 0x40b6525 => {
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffd2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd6u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 16u32); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x40b6557; continue; }
  pc = 0x40b654b; continue;
 }
 0x40b654b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x40b6557; continue; }
  pc = 0x40b6555; continue;
 }
 0x40b6555 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x40b6557; continue;
 }
 0x40b6557 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40b6563; continue; }
  pc = 0x40b655c; continue;
 }
 0x40b655c => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x40b656d; continue;
 }
 0x40b6563 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b656d; continue; }
  pc = 0x40b6568; continue;
 }
 0x40b6568 => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x40b656d; continue;
 }
 0x40b656d => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x40b657e); f_040adbc3(cpu); if cpu.returned_to != 0x40b657e { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x40b657f; continue;
 }
 0x40b657f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b6597; continue; }
  pc = 0x40b6587; continue;
 }
 0x40b6587 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40b657f; continue; }
  pc = 0x40b6597; continue;
 }
 0x40b6597 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40b659a; continue;
 }
 0x40b659a => {
  { let v: u32 = 0xffffffecu32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(32);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b65a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b65a6); }
 let mut pc: u32 = 0x40b65a6;
 loop { match pc {
 0x40b65a6 => {
  pc = 0x40b6946; continue;
 }
 0x40b6946 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b65ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b65ab); }
 let mut pc: u32 = 0x40b65ab;
 loop { match pc {
 0x40b65ab => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40b65b5; continue; }
  pc = 0x40b65b1; continue;
 }
 0x40b65b1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b65b5 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x415aca8u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b65ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b65ca); }
 let mut pc: u32 = 0x40b65ca;
 loop { match pc {
 0x40b65ca => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40b65e0; continue; }
  pc = 0x40b65d9; continue;
 }
 0x40b65d9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b65e0); f_040b1a57(cpu); if cpu.returned_to != 0x40b65e0 { return; }
  pc = 0x40b65e0; continue;
 }
 0x40b65e0 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b65fc; continue; }
  pc = 0x40b65ea; continue;
 }
 0x40b65ea => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x40b65fc; continue;
 }
 0x40b65fc => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40b6600; continue;
 }
 0x40b6600 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b6617; continue; }
  pc = 0x40b6608; continue;
 }
 0x40b6608 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x40b6610); f_040b19b3(cpu); if cpu.returned_to != 0x40b6610 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b6600; continue; }
  pc = 0x40b6615; continue;
 }
 0x40b6615 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b6617; continue;
 }
 0x40b6617 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40b6635; continue; }
  pc = 0x40b661b; continue;
 }
 0x40b661b => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40b6635; continue; }
  pc = 0x40b662c; continue;
 }
 0x40b662c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40b6633); f_040accda(cpu); if cpu.returned_to != 0x40b6633 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x40b6635; continue;
 }
 0x40b6635 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40b663d; continue; }
  pc = 0x40b6639; continue;
 }
 0x40b6639 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x40b663d; continue;
 }
 0x40b663d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
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
pub fn f_040b6645(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6645); }
 let mut pc: u32 = 0x40b6645;
 loop { match pc {
 0x40b6645 => {
  { let v: u32 = cpu.r32(0x415b818u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b664b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b664b); }
 let mut pc: u32 = 0x40b664b;
 loop { match pc {
 0x40b664b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x40b6657); f_040a16fb(cpu); if cpu.returned_to != 0x40b6657 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40b6688; continue; }
  pc = 0x40b665e; continue;
 }
 0x40b665e => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x40b6671; continue; }
  pc = 0x40b6664; continue;
 }
 0x40b6664 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x40b6671; continue; }
  pc = 0x40b666a; continue;
 }
 0x40b666a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40b6671); f_040b6949(cpu); if cpu.returned_to != 0x40b6671 { return; }
  pc = 0x40b6671; continue;
 }
 0x40b6671 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x40b6688; continue; }
  pc = 0x40b6677; continue;
 }
 0x40b6677 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x40b6685; continue; }
  pc = 0x40b667d; continue;
 }
 0x40b667d => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6685 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x40b6688; continue;
 }
 0x40b6688 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b668d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b668d); }
 let mut pc: u32 = 0x40b668d;
 loop { match pc {
 0x40b668d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_040b6781(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6697(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6697); }
 let mut pc: u32 = 0x40b6697;
 loop { match pc {
 0x40b6697 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x415ca3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40b66c6; continue; }
  pc = 0x40b66aa; continue;
 }
 0x40b66aa => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x40b66c6; continue; }
  pc = 0x40b66af; continue;
 }
 0x40b66af => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x40b66c6; continue; }
  pc = 0x40b66b4; continue;
 }
 0x40b66b4 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415ca3cu32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x40b66c6); cpu.call(t); if cpu.returned_to != 0x40b66c6 { return; }
  pc = 0x40b66c6; continue;
 }
 0x40b66c6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b66cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b66cc); }
 let mut pc: u32 = 0x40b66cc;
 loop { match pc {
 0x40b66cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b66d9; continue; }
  pc = 0x40b66d4; continue;
 }
 0x40b66d4 => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b66ea; continue; }
  pc = 0x40b66d9; continue;
 }
 0x40b66d9 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x40b66e3); f_0409ebfb(cpu); if cpu.returned_to != 0x40b66e3 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b66ea => {
  { let v: u32 = 0x40b668du32; cpu.w32(0x415ca68u32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x40b6711; continue; }
  pc = 0x40b66f9; continue;
 }
 0x40b66f9 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x40b6705; continue; }
  pc = 0x40b66fe; continue;
 }
 0x40b66fe => {
  cpu.push(0x40b6703); f_040b5700(cpu); if cpu.returned_to != 0x40b6703 { return; }
  pc = 0x40b6741; continue;
 }
 0x40b6705 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x40b6741; continue; }
  pc = 0x40b670a; continue;
 }
 0x40b670a => {
  cpu.push(0x40b670f); f_040b58ee(cpu); if cpu.returned_to != 0x40b670f { return; }
  pc = 0x40b6741; continue;
 }
 0x40b6711 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x40b672e; continue; }
  pc = 0x40b6716; continue;
 }
 0x40b6716 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x40b6722; continue; }
  pc = 0x40b671b; continue;
 }
 0x40b671b => {
  cpu.push(0x40b6720); f_040b58b6(cpu); if cpu.returned_to != 0x40b6720 { return; }
  pc = 0x40b6741; continue;
 }
 0x40b6722 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x40b6741; continue; }
  pc = 0x40b6727; continue;
 }
 0x40b6727 => {
  cpu.push(0x40b672c); f_040b5a09(cpu); if cpu.returned_to != 0x40b672c { return; }
  pc = 0x40b6741; continue;
 }
 0x40b672e => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x40b6741; continue; }
  pc = 0x40b6733; continue;
 }
 0x40b6733 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x40b6741; continue; }
  pc = 0x40b6737; continue;
 }
 0x40b6737 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x40b6741; continue; }
  pc = 0x40b673c; continue;
 }
 0x40b673c => {
  cpu.push(0x40b6741); f_040b6971(cpu); if cpu.returned_to != 0x40b6741 { return; }
  pc = 0x40b6741; continue;
 }
 0x40b6741 => {
  { let v: u32 = cpu.r32(0x415ca34u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca34u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6751(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b6751); } r_040b6751(cpu, 0x40b6751); }
pub fn f_040b6781(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b6781); } r_040b6751(cpu, 0x40b6781); }
fn r_040b6751(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b6751 => {
  { let v: u8 = cpu.shl8(cpu.r8(0xbu32.wrapping_add(cpu.edi)), 4u32); cpu.w8(0xbu32.wrapping_add(cpu.edi), v); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v = 0x67cb040bu32; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = v; }
  { let v = 0x6817040bu32; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.esi.wrapping_mul(8))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v = 0x6817040bu32; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = v; }
  { let v = 0x6817040bu32; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = v; }
  { let v = 0x5153040bu32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415ca34u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b6839; continue; }
  pc = 0x40b67a3; continue;
 }
 0x40b6781 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415ca34u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b6839; continue; }
  pc = 0x40b67a3; continue;
 }
 0x40b67a3 => {
  cpu.poll(); pc = cpu.r32(0x40b6751u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x40b67ab => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x40b67b5); f_040b6697(cpu); if cpu.returned_to != 0x40b67b5 { return; }
  pc = 0x40b67b5; continue;
 }
 0x40b67b5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40b67b7; continue;
 }
 0x40b67b7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b67c0 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x40b67cb; continue; }
  pc = 0x40b67c6; continue;
 }
 0x40b67c6 => {
  cpu.push(0x40b67cb); f_040b695f(cpu); if cpu.returned_to != 0x40b67cb { return; }
  pc = 0x40b67cb; continue;
 }
 0x40b67cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b67d3; continue;
 }
 0x40b67d3 => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b67d8; continue;
 }
 0x40b67d8 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b67dd; continue;
 }
 0x40b67dd => {
  { let v: u32 = 0x2u32; cpu.w32(0x415ca34u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x40b67ed); f_040b5700(cpu); if cpu.returned_to != 0x40b67ed { return; }
  pc = 0x40b67ed; continue;
 }
 0x40b67ed => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x40b67ef; continue;
 }
 0x40b67ef => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x40b67f2); cpu.call(t); if cpu.returned_to != 0x40b67f2 { return; }
  cpu.poll(); pc = 0x40b67b5; continue;
 }
 0x40b67f4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b67fc; continue;
 }
 0x40b67fc => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b6801; continue;
 }
 0x40b6801 => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b6806; continue;
 }
 0x40b6806 => {
  { let v: u32 = 0x2u32; cpu.w32(0x415ca34u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x40b6815); f_040b58b6(cpu); if cpu.returned_to != 0x40b6815 { return; }
  cpu.poll(); pc = 0x40b67ed; continue;
 }
 0x40b6817 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b681f; continue;
 }
 0x40b681f => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b6824; continue;
 }
 0x40b6824 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b67b5; continue; }
  pc = 0x40b6829; continue;
 }
 0x40b6829 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415ca34u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x40b67ef; continue;
 }
 0x40b6839 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x40b67b7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6843(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6843); }
 let mut pc: u32 = 0x40b6843;
 loop { match pc {
 0x40b6843 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x40b6862; continue; }
  pc = 0x40b6857; continue;
 }
 0x40b6857 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x40b6862; continue; }
  pc = 0x40b685f; continue;
 }
 0x40b685f => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40b6867; continue;
 }
 0x40b6862 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40b6867; continue;
 }
 0x40b6867 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40b6878; continue; }
  pc = 0x40b686b; continue;
 }
 0x40b686b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40b6878; continue; }
  pc = 0x40b6873; continue;
 }
 0x40b6873 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x40b6867; continue;
 }
 0x40b6878 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6882(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6882); }
 let mut pc: u32 = 0x40b6882;
 loop { match pc {
 0x40b6882 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x40b68ab; continue; }
  pc = 0x40b6893; continue;
 }
 0x40b6893 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b68ab; continue; }
  pc = 0x40b68a1; continue;
 }
 0x40b68a1 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x40b68ab; continue; }
  pc = 0x40b68a5; continue;
 }
 0x40b68a5 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b68af; continue;
 }
 0x40b68ab => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b68af; continue;
 }
 0x40b68af => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x40b68b7); f_040b1e75(cpu); if cpu.returned_to != 0x40b68b7 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b68be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b68be); }
 let mut pc: u32 = 0x40b68be;
 loop { match pc {
 0x40b68be => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x40b68ca); f_0408cec2(cpu); if cpu.returned_to != 0x40b68ca { return; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.eax), v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b68d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b68d7); }
 let mut pc: u32 = 0x40b68d7;
 loop { match pc {
 0x40b68d7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x40b68fe; continue; }
  pc = 0x40b68fa; continue;
 }
 0x40b68fa => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40b6933; continue; }
  pc = 0x40b68fe; continue;
 }
 0x40b68fe => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7ff0u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0x800fu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx | 0x3fe0u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3feu32); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x40b6933; continue;
 }
 0x40b6933 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6949(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6949); }
 let mut pc: u32 = 0x40b6949;
 loop { match pc {
 0x40b6949 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_040accda(cpu); return; }
  pc = 0x40b6951; continue;
 }
 0x40b6951 => {
  cpu.push(0x40b6956); f_0409d741(cpu); if cpu.returned_to != 0x40b6956 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b695f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b695f); }
 let mut pc: u32 = 0x40b695f;
 loop { match pc {
 0x40b695f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x414b814u32; cpu.eax = v; }
  cpu.push(0x40b696f); f_040ad382(cpu); if cpu.returned_to != 0x40b696f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6971(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6971); }
 let mut pc: u32 = 0x40b6971;
 loop { match pc {
 0x40b6971 => {
  pc = 0x40b6976; continue;
 }
 0x40b6976 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415c9d0u32), 0x40b6a05u32); }
  if cpu.zf { pc = 0x40b699f; continue; }
  pc = 0x40b6984; continue;
 }
 0x40b6984 => {
  { let v: u32 = 0x40b6697u32; cpu.ebx = v; }
  { let v: u32 = 0x40b6a05u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b845u32, v); }
  cpu.push(0x40b6999); f_040b69a8(cpu); if cpu.returned_to != 0x40b6999 { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415c9d0u32, v); }
  pc = 0x40b699f; continue;
 }
 0x40b699f => {
  cpu.push(0x40b69a4); f_040b6a62(cpu); if cpu.returned_to != 0x40b69a4 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b69a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b69a8); }
 let mut pc: u32 = 0x40b69a8;
 loop { match pc {
 0x40b69a8 => {
  { cpu.sub16(cpu.r16(0x415ca6cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x40b69bd; continue; }
  pc = 0x40b69b2; continue;
 }
 0x40b69b2 => {
  { cpu.sub8(cpu.r8(0x41544dcu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x40b69bd; continue; }
  pc = 0x40b69bb; continue;
 }
 0x40b69bb => {
  pc = 0x40b69be; continue;
 }
 0x40b69bd => {
  cpu.flat_ds(0x40b69bd); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b69be => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x40b69d4; continue; }
  pc = 0x40b69cd; continue;
 }
 0x40b69cd => {
  cpu.flat_ds(0x40b69cd); cpu.push(0x40b69d2); f_040b6df8(cpu); if cpu.returned_to != 0x40b69d2 { return; }
  pc = 0x40b69f9; continue;
 }
 0x40b69d4 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x40b69e4; continue; }
  pc = 0x40b69dd; continue;
 }
 0x40b69dd => {
  cpu.flat_ds(0x40b69dd); cpu.push(0x40b69e2); f_040b6db2(cpu); if cpu.returned_to != 0x40b69e2 { return; }
  pc = 0x40b69f9; continue;
 }
 0x40b69e4 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x40b69f4; continue; }
  pc = 0x40b69ed; continue;
 }
 0x40b69ed => {
  cpu.flat_ds(0x40b69ed); cpu.push(0x40b69f2); f_040b6e50(cpu); if cpu.returned_to != 0x40b69f2 { return; }
  pc = 0x40b69f9; continue;
 }
 0x40b69f4 => {
  cpu.flat_ds(0x40b69f4); cpu.push(0x40b69f9); f_040b6cc2(cpu); if cpu.returned_to != 0x40b69f9 { return; }
  pc = 0x40b69f9; continue;
 }
 0x40b69f9 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b69fa); cpu.push(0x40b69ff); f_040b6a62(cpu); if cpu.returned_to != 0x40b69ff { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x40b6a04); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6a05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6a05); }
 let mut pc: u32 = 0x40b6a05;
 loop { match pc {
 0x40b6a05 => {
  { cpu.sub16(cpu.r16(0x415ca6cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x40b6a10; continue; }
  pc = 0x40b6a0f; continue;
 }
 0x40b6a0f => {
  cpu.flat_ds(0x40b6a0f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6a10 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v: u8 = 0x7fu8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  {  }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x40b6a37; continue; }
  pc = 0x40b6a30; continue;
 }
 0x40b6a30 => {
  cpu.flat_ds(0x40b6a30); cpu.push(0x40b6a35); f_040b6e36(cpu); if cpu.returned_to != 0x40b6a35 { return; }
  pc = 0x40b6a5c; continue;
 }
 0x40b6a37 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x40b6a47; continue; }
  pc = 0x40b6a40; continue;
 }
 0x40b6a40 => {
  cpu.flat_ds(0x40b6a40); cpu.push(0x40b6a45); f_040b6ddf(cpu); if cpu.returned_to != 0x40b6a45 { return; }
  pc = 0x40b6a5c; continue;
 }
 0x40b6a47 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x40b6a57; continue; }
  pc = 0x40b6a50; continue;
 }
 0x40b6a50 => {
  cpu.flat_ds(0x40b6a50); cpu.push(0x40b6a55); f_040b6ea8(cpu); if cpu.returned_to != 0x40b6a55 { return; }
  pc = 0x40b6a5c; continue;
 }
 0x40b6a57 => {
  cpu.flat_ds(0x40b6a57); cpu.push(0x40b6a5c); f_040b6d85(cpu); if cpu.returned_to != 0x40b6a5c { return; }
  pc = 0x40b6a5c; continue;
 }
 0x40b6a5c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x40b6a60); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6a62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6a62); }
 let mut pc: u32 = 0x40b6a62;
 loop { match pc {
 0x40b6a62 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  {  }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  {  }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)) & 0xfff2u16; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.esp)); }
  {  }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6a77(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b6a77); } r_040b6a77(cpu, 0x40b6a77); }
pub fn f_040b6a84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b6a84); } r_040b6a77(cpu, 0x40b6a84); }
fn r_040b6a77(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b6a77 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.port_out(0xa0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out(0xf0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40b6a84; continue;
 }
 0x40b6a84 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x40b6a92, "Fnstenv");
  {  }
  {  }
  {  }
  {  }
  cpu.unimplemented(0x40b6a9a, "Fndisi");
  {  }
  cpu.flat_ds(0x40b6a9d); cpu.push(0x40b6aa2); f_0409db58(cpu); if cpu.returned_to != 0x40b6aa2 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x40b6ab2; continue;
 }
 0x40b6ab2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if cpu.cf { cpu.poll(); pc = 0x40b6ab2; continue; }
  pc = 0x40b6abc; continue;
 }
 0x40b6abc => {
  { cpu.sub8((cpu.ebx as u8), 0xdfu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40b6ab2; continue; }
  pc = 0x40b6ac1; continue;
 }
 0x40b6ac1 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let a = (cpu.ebx as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x40b6ad4; continue; }
  pc = 0x40b6ad0; continue;
 }
 0x40b6ad0 => {
  { let v: u8 = 0x86u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6b20; continue;
 }
 0x40b6ad4 => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x40b6ae8; continue; }
  pc = 0x40b6ad9; continue;
 }
 0x40b6ad9 => {
  { cpu.and16((cpu.edx as u16), 0x200u16); }
  if !cpu.zf { pc = 0x40b6ae4; continue; }
  pc = 0x40b6ae0; continue;
 }
 0x40b6ae0 => {
  { let v: u8 = 0x8bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6ae6; continue;
 }
 0x40b6ae4 => {
  { let v: u8 = 0x8au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6ae6; continue;
 }
 0x40b6ae6 => {
  pc = 0x40b6b20; continue;
 }
 0x40b6ae8 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40b6af4; continue; }
  pc = 0x40b6aed; continue;
 }
 0x40b6aed => {
  cpu.flat_ds(0x40b6aed); cpu.push(0x40b6af2); f_040b6b8a(cpu); if cpu.returned_to != 0x40b6af2 { return; }
  pc = 0x40b6b20; continue;
 }
 0x40b6af4 => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x40b6afd; continue; }
  pc = 0x40b6af9; continue;
 }
 0x40b6af9 => {
  { let v: u8 = 0x82u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6b20; continue;
 }
 0x40b6afd => {
  { cpu.and8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x40b6b0b; continue; }
  pc = 0x40b6b02; continue;
 }
 0x40b6b02 => {
  cpu.flat_ds(0x40b6b02); cpu.push(0x40b6b07); f_040b6c17(cpu); if cpu.returned_to != 0x40b6b07 { return; }
  { let v: u8 = 0x84u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6b20; continue;
 }
 0x40b6b0b => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x40b6b14; continue; }
  pc = 0x40b6b10; continue;
 }
 0x40b6b10 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6b20; continue;
 }
 0x40b6b14 => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b6b20; continue; }
  pc = 0x40b6b19; continue;
 }
 0x40b6b19 => {
  cpu.flat_ds(0x40b6b19); cpu.push(0x40b6b1e); f_040b6c69(cpu); if cpu.returned_to != 0x40b6b1e { return; }
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6b20; continue;
 }
 0x40b6b20 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x40b6b70; continue; }
  pc = 0x40b6b25; continue;
 }
 0x40b6b25 => {
  { let v = cpu.r32(0x415b81cu32.wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x415cc8eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x415cc90u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x415cc8eu32; cpu.esp = v; }
  { let v: u32 = 0x415ca8eu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x415b845u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40b6b4e); cpu.push(0x40b6b54); cpu.call(t); if cpu.returned_to != 0x40b6b54 { return; }
  { let v = cpu.r16(0x415cc8eu32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x415cc90u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x415cc8eu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x40b6b70; continue;
 }
 0x40b6b70 => {
  {  }
  {  }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)) & 0xff72u16; cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.fpu_control = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); }
  {  }
  { let v: u32 = cpu.esp.wrapping_add(0x1cu32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x40b6b89); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6b8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6b8a); }
 let mut pc: u32 = 0x40b6b8a;
 loop { match pc {
 0x40b6b8a => {
  { let v: u8 = 0x81u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xd9fau16); }
  if !cpu.zf { pc = 0x40b6b96; continue; }
  pc = 0x40b6b93; continue;
 }
 0x40b6b93 => {
  { let v: u8 = 0x88u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6b96 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f1u16); }
  if !cpu.zf { pc = 0x40b6ba0; continue; }
  pc = 0x40b6b9d; continue;
 }
 0x40b6b9d => {
  { let v: u8 = 0x8eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b6ba0 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f8u16); }
  if cpu.zf { pc = 0x40b6bac; continue; }
  pc = 0x40b6ba7; continue;
 }
 0x40b6ba7 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f5u16); }
  pc = 0x40b6bac; continue;
 }
 0x40b6bac => {
  if !cpu.zf { pc = 0x40b6bb2; continue; }
  pc = 0x40b6bae; continue;
 }
 0x40b6bae => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6c16; continue;
 }
 0x40b6bb2 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x310u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x310u16); }
  if !cpu.zf { pc = 0x40b6bc5; continue; }
  pc = 0x40b6bc1; continue;
 }
 0x40b6bc1 => {
  { let v: u8 = 0x8du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6c16; continue;
 }
 0x40b6bc5 => {
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x40b6bef; continue; }
  pc = 0x40b6bd1; continue;
 }
 0x40b6bd1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x40b6be4; continue; }
  pc = 0x40b6bdf; continue;
 }
 0x40b6bdf => {
  cpu.push(0x40b6be4); f_040b6cb4(cpu); if cpu.returned_to != 0x40b6be4 { return; }
  pc = 0x40b6be4; continue;
 }
 0x40b6be4 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x40b6beb; continue; }
  pc = 0x40b6be9; continue;
 }
 0x40b6be9 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40b6beb; continue;
 }
 0x40b6beb => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6c16; continue;
 }
 0x40b6bef => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x130u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x30u16); }
  if !cpu.zf { pc = 0x40b6c16; continue; }
  pc = 0x40b6bfd; continue;
 }
 0x40b6bfd => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x38u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x40b6c16; continue; }
  pc = 0x40b6c14; continue;
 }
 0x40b6c14 => {
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x40b6c16; continue;
 }
 0x40b6c16 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6c17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6c17); }
 let mut pc: u32 = 0x40b6c17;
 loop { match pc {
 0x40b6c17 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9fdu16); }
  if !cpu.zf { pc = 0x40b6c20; continue; }
  pc = 0x40b6c1e; continue;
 }
 0x40b6c1e => {
  pc = 0x40b6c68; continue;
 }
 0x40b6c20 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x40b6c4c; continue; }
  pc = 0x40b6c2f; continue;
 }
 0x40b6c2f => {
  cpu.push(0x40b6c34); f_040b6c69(cpu); if cpu.returned_to != 0x40b6c34 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x40b6c43; continue; }
  pc = 0x40b6c3e; continue;
 }
 0x40b6c3e => {
  cpu.push(0x40b6c43); f_040b6cb4(cpu); if cpu.returned_to != 0x40b6c43 { return; }
  pc = 0x40b6c43; continue;
 }
 0x40b6c43 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x40b6c4a; continue; }
  pc = 0x40b6c48; continue;
 }
 0x40b6c48 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40b6c4a; continue;
 }
 0x40b6c4a => {
  pc = 0x40b6c68; continue;
 }
 0x40b6c4c => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x40b6c63; continue; }
  pc = 0x40b6c56; continue;
 }
 0x40b6c56 => {
  { let v: u32 = 0x415ca78u32; cpu.esi = v; }
  cpu.push(0x40b6c61); f_040b6c8f(cpu); if cpu.returned_to != 0x40b6c61 { return; }
  pc = 0x40b6c68; continue;
 }
 0x40b6c63 => {
  cpu.push(0x40b6c68); f_040b6c69(cpu); if cpu.returned_to != 0x40b6c68 { return; }
  pc = 0x40b6c68; continue;
 }
 0x40b6c68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6c69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6c69); }
 let mut pc: u32 = 0x40b6c69;
 loop { match pc {
 0x40b6c69 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b6c7e; continue; }
  pc = 0x40b6c76; continue;
 }
 0x40b6c76 => {
  { let v = cpu.rf64(0x415ca82u32); cpu.fpush(v); }
  pc = 0x40b6c84; continue;
 }
 0x40b6c7e => {
  { let v = cpu.rf32(0x415ca8au32); cpu.fpush(v); }
  pc = 0x40b6c84; continue;
 }
 0x40b6c84 => {
  { cpu.and16(cpu.r16(0x14u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if cpu.zf { pc = 0x40b6c8e; continue; }
  pc = 0x40b6c8c; continue;
 }
 0x40b6c8c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x40b6c8e; continue;
 }
 0x40b6c8e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6c8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6c8f); }
 let mut pc: u32 = 0x40b6c8f;
 loop { match pc {
 0x40b6c8f => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if !cpu.zf { pc = 0x40b6c98; continue; }
  pc = 0x40b6c94; continue;
 }
 0x40b6c94 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b6c9d; continue;
 }
 0x40b6c98 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.edx as u8), 0x7u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40b6c9d; continue;
 }
 0x40b6c9d => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x40b6c9e; continue;
 }
 0x40b6c9e => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b6ca6; continue; }
  pc = 0x40b6ca2; continue;
 }
 0x40b6ca2 => {
  cpu.unimplemented(0x40b6ca2, "Fincstp");
  cpu.poll(); pc = 0x40b6c9e; continue;
 }
 0x40b6ca6 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x40b6cab; continue;
 }
 0x40b6cab => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x40b6cb3; continue; }
  pc = 0x40b6caf; continue;
 }
 0x40b6caf => {
  cpu.unimplemented(0x40b6caf, "Fdecstp");
  cpu.poll(); pc = 0x40b6cab; continue;
 }
 0x40b6cb3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6cb4); }
 let mut pc: u32 = 0x40b6cb4;
 loop { match pc {
 0x40b6cb4 => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40b6cbe; continue; }
  pc = 0x40b6cb9; continue;
 }
 0x40b6cb9 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40b6cc1; continue;
 }
 0x40b6cbe => {
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40b6cc1; continue;
 }
 0x40b6cc1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6cc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6cc2); }
 let mut pc: u32 = 0x40b6cc2;
 loop { match pc {
 0x40b6cc2 => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x415cc97u32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x40b6cd4; continue; }
  pc = 0x40b6ccc; continue;
 }
 0x40b6ccc => {
  { let v: u32 = 0x40b6a77u32; cpu.edx = v; }
  pc = 0x40b6cda; continue;
 }
 0x40b6cd4 => {
  { let v: u32 = 0x40b6a84u32; cpu.edx = v; }
  pc = 0x40b6cda; continue;
 }
 0x40b6cda => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x40b6d01; continue; }
  pc = 0x40b6ce3; continue;
 }
 0x40b6ce3 => {
  { let v: u8 = 0x50u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x40b6d01; continue;
 }
 0x40b6d01 => {
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x415cc95u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca6eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415ca6cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca72u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x40b6d78; continue; }
  pc = 0x40b6d3a; continue;
 }
 0x40b6d3a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x40b6d3d, "Sidt");
  { let v: u16 = 0x60u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x284u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = v; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = 0x50u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x40b6d83; continue;
 }
 0x40b6d78 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x40b6d83; continue;
 }
 0x40b6d83 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6d84); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6d85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6d85); }
 let mut pc: u32 = 0x40b6d85;
 loop { match pc {
 0x40b6d85 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x415cc95u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca72u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415ca6eu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x415ca6cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6db1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6db2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6db2); }
 let mut pc: u32 = 0x40b6db2;
 loop { match pc {
 0x40b6db2 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc94u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca6eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415ca6cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc94u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x40b6a84u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6dde); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6ddf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6ddf); }
 let mut pc: u32 = 0x40b6ddf;
 loop { match pc {
 0x40b6ddf => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc94u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca6eu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x415ca6cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6df7); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6df8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6df8); }
 let mut pc: u32 = 0x40b6df8;
 loop { match pc {
 0x40b6df8 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc96u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ca6eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415ca6cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc96u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x415cc97u32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x40b6e27; continue; }
  pc = 0x40b6e1f; continue;
 }
 0x40b6e1f => {
  { let v: u32 = 0x40b6a77u32; cpu.edx = v; }
  pc = 0x40b6e2d; continue;
 }
 0x40b6e27 => {
  { let v: u32 = 0x40b6a84u32; cpu.edx = v; }
  pc = 0x40b6e2d; continue;
 }
 0x40b6e2d => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6e35); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6e36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6e36); }
 let mut pc: u32 = 0x40b6e36;
 loop { match pc {
 0x40b6e36 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x415cc96u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x415ca6eu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x415ca6cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40b6e4e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6e50); }
 let mut pc: u32 = 0x40b6e50;
 loop { match pc {
 0x40b6e50 => {
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x415cc94u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.edx; cpu.w32(0x415ca6eu32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415ca6cu32, v); }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x415cc94u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415ca72u32, v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x415cc97u32), 0x20u8); }
  if !cpu.zf { pc = 0x40b6e95; continue; }
  pc = 0x40b6e8d; continue;
 }
 0x40b6e8d => {
  { let v: u32 = 0x40b6a77u32; cpu.edx = v; }
  pc = 0x40b6e9b; continue;
 }
 0x40b6e95 => {
  { let v: u32 = 0x40b6a84u32; cpu.edx = v; }
  pc = 0x40b6e9b; continue;
 }
 0x40b6e9b => {
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x415cc94u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6ea8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6ea8); }
 let mut pc: u32 = 0x40b6ea8;
 loop { match pc {
 0x40b6ea8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6eb0); }
 let mut pc: u32 = 0x40b6eb0;
 loop { match pc {
 0x40b6eb0 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x40b6eb5; continue;
 }
 0x40b6eb5 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x40b6ec3; continue; }
  pc = 0x40b6ebf; continue;
 }
 0x40b6ebf => {
  { let v: u16 = (cpu.eax as u16) | 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b6ec3; continue;
 }
 0x40b6ec3 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if cpu.zf { pc = 0x40b6ecf; continue; }
  pc = 0x40b6ecb; continue;
 }
 0x40b6ecb => {
  { let v: u16 = (cpu.eax as u16) | 0x7e0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b6ecf; continue;
 }
 0x40b6ecf => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if cpu.zf { pc = 0x40b6edb; continue; }
  pc = 0x40b6ed7; continue;
 }
 0x40b6ed7 => {
  { let v: u16 = (cpu.eax as u16) | 0xf800u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40b6edb; continue;
 }
 0x40b6edb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4223960u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x40b6ef2; continue; }
  pc = 0x40b6eef; continue;
 }
 0x40b6eef => {
  { let v: u32 = cpu.eax & 0xffffffe0u32; cpu.eax = v; }
  pc = 0x40b6ef2; continue;
 }
 0x40b6ef2 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if !cpu.zf { pc = 0x40b6eff; continue; }
  pc = 0x40b6efa; continue;
 }
 0x40b6efa => {
  { let v: u32 = cpu.eax & 0xfffff81fu32; cpu.eax = v; }
  pc = 0x40b6eff; continue;
 }
 0x40b6eff => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if !cpu.zf { pc = 0x40b6f0c; continue; }
  pc = 0x40b6f07; continue;
 }
 0x40b6f07 => {
  { let v: u32 = cpu.eax & 0xffff07ffu32; cpu.eax = v; }
  pc = 0x40b6f0c; continue;
 }
 0x40b6f0c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4225960u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40b6eb5; continue; }
  pc = 0x40b6f1f; continue;
 }
 0x40b6f1f => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6f21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6f21); }
 let mut pc: u32 = 0x40b6f21;
 loop { match pc {
 0x40b6f21 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b6f41; continue; }
  pc = 0x40b6f28; continue;
 }
 0x40b6f28 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0x4223960u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b6f41; continue;
 }
 0x40b6f41 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6f42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6f42); }
 let mut pc: u32 = 0x40b6f42;
 loop { match pc {
 0x40b6f42 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x40b6f6b; continue; }
  pc = 0x40b6f52; continue;
 }
 0x40b6f52 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.eax as u16), cpu.r16(0x4225960u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b6f6b; continue;
 }
 0x40b6f6b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6f6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6f6c); }
 let mut pc: u32 = 0x40b6f6c;
 loop { match pc {
 0x40b6f6c => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x40b6f87; continue; }
  pc = 0x40b6f70; continue;
 }
 0x40b6f70 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7bef7befu32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b6f87; continue;
 }
 0x40b6f87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6f88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6f88); }
 let mut pc: u32 = 0x40b6f88;
 loop { match pc {
 0x40b6f88 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b6fad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b6fad); }
 let mut pc: u32 = 0x40b6fad;
 loop { match pc {
 0x40b6fad => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x4153748u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4153744u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4153730u32, v); }
  pc = 0x40b7485; continue;
 }
 0x40b6fde => {
  { let v: u32 = cpu.r32(0x4153744u32); cpu.ecx = v; }
  pc = 0x40b71fc; continue;
 }
 0x40b6fe9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7007; continue; }
  pc = 0x40b6ff0; continue;
 }
 0x40b6ff0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7007; continue;
 }
 0x40b7007 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7028; continue; }
  pc = 0x40b700f; continue;
 }
 0x40b700f => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7028; continue;
 }
 0x40b7028 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7049; continue; }
  pc = 0x40b7030; continue;
 }
 0x40b7030 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7049; continue;
 }
 0x40b7049 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b706a; continue; }
  pc = 0x40b7051; continue;
 }
 0x40b7051 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b706a; continue;
 }
 0x40b706a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b708b; continue; }
  pc = 0x40b7072; continue;
 }
 0x40b7072 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b708b; continue;
 }
 0x40b708b => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b70ac; continue; }
  pc = 0x40b7093; continue;
 }
 0x40b7093 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x40b70ac; continue;
 }
 0x40b70ac => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b70cd; continue; }
  pc = 0x40b70b4; continue;
 }
 0x40b70b4 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x40b70cd; continue;
 }
 0x40b70cd => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b70ee; continue; }
  pc = 0x40b70d5; continue;
 }
 0x40b70d5 => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x40b70ee; continue;
 }
 0x40b70ee => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b710f; continue; }
  pc = 0x40b70f6; continue;
 }
 0x40b70f6 => {
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b710f; continue;
 }
 0x40b710f => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7130; continue; }
  pc = 0x40b7117; continue;
 }
 0x40b7117 => {
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7130; continue;
 }
 0x40b7130 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7151; continue; }
  pc = 0x40b7138; continue;
 }
 0x40b7138 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7151; continue;
 }
 0x40b7151 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7172; continue; }
  pc = 0x40b7159; continue;
 }
 0x40b7159 => {
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7172; continue;
 }
 0x40b7172 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7193; continue; }
  pc = 0x40b717a; continue;
 }
 0x40b717a => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7193; continue;
 }
 0x40b7193 => {
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b71b4; continue; }
  pc = 0x40b719b; continue;
 }
 0x40b719b => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x40b71b4; continue;
 }
 0x40b71b4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b71d5; continue; }
  pc = 0x40b71bc; continue;
 }
 0x40b71bc => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x40b71d5; continue;
 }
 0x40b71d5 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b71f6; continue; }
  pc = 0x40b71dd; continue;
 }
 0x40b71dd => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x40b71f6; continue;
 }
 0x40b71f6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x40b71fc; continue;
 }
 0x40b71fc => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b6fe9; continue; }
  pc = 0x40b7205; continue;
 }
 0x40b7205 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b7211; continue;
 }
 0x40b7211 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b722f; continue; }
  pc = 0x40b7218; continue;
 }
 0x40b7218 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b722f; continue;
 }
 0x40b722f => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b724f; continue; }
  pc = 0x40b7236; continue;
 }
 0x40b7236 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b724f; continue;
 }
 0x40b724f => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b725b; continue;
 }
 0x40b725b => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7279; continue; }
  pc = 0x40b7262; continue;
 }
 0x40b7262 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7279; continue;
 }
 0x40b7279 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7299; continue; }
  pc = 0x40b7280; continue;
 }
 0x40b7280 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7299; continue;
 }
 0x40b7299 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b72a5; continue;
 }
 0x40b72a5 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b72c3; continue; }
  pc = 0x40b72ac; continue;
 }
 0x40b72ac => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b72c3; continue;
 }
 0x40b72c3 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b72e3; continue; }
  pc = 0x40b72ca; continue;
 }
 0x40b72ca => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b72e3; continue;
 }
 0x40b72e3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b72ef; continue;
 }
 0x40b72ef => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b730d; continue; }
  pc = 0x40b72f6; continue;
 }
 0x40b72f6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b730d; continue;
 }
 0x40b730d => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b732d; continue; }
  pc = 0x40b7314; continue;
 }
 0x40b7314 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b732d; continue;
 }
 0x40b732d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b7339; continue;
 }
 0x40b7339 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7357; continue; }
  pc = 0x40b7340; continue;
 }
 0x40b7340 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7357; continue;
 }
 0x40b7357 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7377; continue; }
  pc = 0x40b735e; continue;
 }
 0x40b735e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7377; continue;
 }
 0x40b7377 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b7383; continue;
 }
 0x40b7383 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b73a1; continue; }
  pc = 0x40b738a; continue;
 }
 0x40b738a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b73a1; continue;
 }
 0x40b73a1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b73c1; continue; }
  pc = 0x40b73a8; continue;
 }
 0x40b73a8 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b73c1; continue;
 }
 0x40b73c1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b73cd; continue;
 }
 0x40b73cd => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b73eb; continue; }
  pc = 0x40b73d4; continue;
 }
 0x40b73d4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b73eb; continue;
 }
 0x40b73eb => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b740b; continue; }
  pc = 0x40b73f2; continue;
 }
 0x40b73f2 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b740b; continue;
 }
 0x40b740b => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7454; continue; }
  pc = 0x40b7413; continue;
 }
 0x40b7413 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7431; continue; }
  pc = 0x40b741a; continue;
 }
 0x40b741a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7431; continue;
 }
 0x40b7431 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b7451; continue; }
  pc = 0x40b7438; continue;
 }
 0x40b7438 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7451; continue;
 }
 0x40b7451 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7454; continue;
 }
 0x40b7454 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x40b747d; continue; }
  pc = 0x40b745c; continue;
 }
 0x40b745c => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b747a; continue; }
  pc = 0x40b7463; continue;
 }
 0x40b7463 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b747a; continue;
 }
 0x40b747a => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x40b747d; continue;
 }
 0x40b747d => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4153730u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x40b7485; continue;
 }
 0x40b7485 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153748u32)); cpu.w32(0x4153748u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x40b6fde; continue; }
  pc = 0x40b7491; continue;
 }
 0x40b7491 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7495(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7495); }
 let mut pc: u32 = 0x40b7495;
 loop { match pc {
 0x40b7495 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x4153748u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4153744u32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x41536d4u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x40b74e7; continue;
 }
 0x40b74c2 => {
  { let v: u32 = cpu.r32(0x4153744u32); cpu.ecx = v; }
  pc = 0x40b74c8; continue;
 }
 0x40b74c8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x40b74da; continue; }
  pc = 0x40b74d0; continue;
 }
 0x40b74d0 => {
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x41536d4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b74da; continue;
 }
 0x40b74da => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b74c8; continue; }
  pc = 0x40b74e3; continue;
 }
 0x40b74e3 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x40b74e7; continue;
 }
 0x40b74e7 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153748u32)); cpu.w32(0x4153748u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x40b74c2; continue; }
  pc = 0x40b74ef; continue;
 }
 0x40b74ef => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b74f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b74f3); }
 let mut pc: u32 = 0x40b74f3;
 loop { match pc {
 0x40b74f3 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x4153748u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4153744u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x41536d6u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4153730u32, v); }
  pc = 0x40b7562; continue;
 }
 0x40b7522 => {
  { let v: u32 = cpu.r32(0x4153744u32); cpu.ecx = v; }
  pc = 0x40b7528; continue;
 }
 0x40b7528 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x41536d6u32)); }
  if cpu.zf { pc = 0x40b7551; continue; }
  pc = 0x40b7536; continue;
 }
 0x40b7536 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x4223960u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7551; continue;
 }
 0x40b7551 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7528; continue; }
  pc = 0x40b755a; continue;
 }
 0x40b755a => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4153730u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x40b7562; continue;
 }
 0x40b7562 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153748u32)); cpu.w32(0x4153748u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x40b7522; continue; }
  pc = 0x40b756a; continue;
 }
 0x40b756a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b756e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b756e); } r_040b756e(cpu, 0x40b756e); }
pub fn f_040b794e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b794e); } r_040b756e(cpu, 0x40b794e); }
pub fn f_040b79a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b79a7); } r_040b756e(cpu, 0x40b79a7); }
pub fn f_040b7a00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7a00); } r_040b756e(cpu, 0x40b7a00); }
pub fn f_040b7a59(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7a59); } r_040b756e(cpu, 0x40b7a59); }
pub fn f_040b7aa1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7aa1); } r_040b756e(cpu, 0x40b7aa1); }
pub fn f_040b7afa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7afa); } r_040b756e(cpu, 0x40b7afa); }
pub fn f_040b7b53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7b53); } r_040b756e(cpu, 0x40b7b53); }
pub fn f_040b7bac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b7bac); } r_040b756e(cpu, 0x40b7bac); }
fn r_040b756e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40b756e => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536e0u32, v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536e4u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536ecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x41536f4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536f0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x41536f8u32, v); }
  { let v: u32 = cpu.r32(0x41536f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4153700u32, v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4153718u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x40b7605; continue; }
  pc = 0x40b75ce; continue;
 }
 0x40b75ce => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41536e0u32).wrapping_sub(cpu.eax); cpu.w32(0x41536e0u32, v); }
  { cpu.and32(cpu.r32(0x41536e0u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b7bc7; continue; }
  pc = 0x40b75e6; continue;
 }
 0x40b75e6 => {
  { let v = cpu.r32(0x41536f4u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x41536ecu32).wrapping_sub(cpu.eax); cpu.w32(0x41536ecu32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536fcu32, v); }
  { let v: u32 = cpu.r32(0x4153718u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b7605; continue;
 }
 0x40b7605 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x41536e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4153720u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b763c; continue; }
  pc = 0x40b7617; continue;
 }
 0x40b7617 => {
  { let v: u32 = cpu.r32(0x41536e0u32).wrapping_sub(cpu.eax); cpu.w32(0x41536e0u32, v); }
  { cpu.and32(cpu.r32(0x41536e0u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b7bc7; continue; }
  pc = 0x40b762d; continue;
 }
 0x40b762d => {
  { let v = cpu.r32(0x41536f4u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x41536ecu32).wrapping_sub(cpu.eax); cpu.w32(0x41536ecu32, v); }
  pc = 0x40b763c; continue;
 }
 0x40b763c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x415371cu32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x40b767e; continue; }
  pc = 0x40b7647; continue;
 }
 0x40b7647 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41536e4u32).wrapping_sub(cpu.eax); cpu.w32(0x41536e4u32, v); }
  { cpu.and32(cpu.r32(0x41536e4u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b7bc7; continue; }
  pc = 0x40b765f; continue;
 }
 0x40b765f => {
  { let v = cpu.r32(0x41536f8u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x41536f0u32).wrapping_sub(cpu.eax); cpu.w32(0x41536f0u32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153700u32, v); }
  { let v: u32 = cpu.r32(0x415371cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b767e; continue;
 }
 0x40b767e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x41536e4u32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x4153724u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b76b5; continue; }
  pc = 0x40b7690; continue;
 }
 0x40b7690 => {
  { let v: u32 = cpu.r32(0x41536e4u32).wrapping_sub(cpu.eax); cpu.w32(0x41536e4u32, v); }
  { cpu.and32(cpu.r32(0x41536e4u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b7bc7; continue; }
  pc = 0x40b76a6; continue;
 }
 0x40b76a6 => {
  { let v = cpu.r32(0x41536f8u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x41536f0u32).wrapping_sub(cpu.eax); cpu.w32(0x41536f0u32, v); }
  pc = 0x40b76b5; continue;
 }
 0x40b76b5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536e8u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x41536ecu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41536e0u32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153704u32, v); }
  { let v: u32 = cpu.r32(0x41536f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41536e4u32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153708u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153710u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x41536e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x41536e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153714u32, v); }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x40b7bca; continue; }
  pc = 0x40b7721; continue;
 }
 0x40b7721 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x40b7c67; continue; }
  pc = 0x40b772b; continue;
 }
 0x40b772b => {
  { let v: u32 = cpu.r32(0x4153700u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41536e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153728u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x4153728u32)); cpu.w32(0x4153728u32, v); }
  pc = 0x40b7bbb; continue;
 }
 0x40b7746 => {
  { let v: u32 = cpu.r32(0x41536e8u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x4153710u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x41536fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x41536e0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415372cu32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40b77bb; continue; }
  pc = 0x40b7778; continue;
 }
 0x40b7778 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b77b8; continue; }
  pc = 0x40b778d; continue;
 }
 0x40b778d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x10820u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b77b8; continue;
 }
 0x40b77b8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x40b77bb; continue;
 }
 0x40b77bb => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x415372cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b793d; continue; }
  pc = 0x40b77ca; continue;
 }
 0x40b77ca => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7823; continue; }
  pc = 0x40b77f7; continue;
 }
 0x40b77f7 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7823; continue;
 }
 0x40b7823 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b787c; continue; }
  pc = 0x40b7850; continue;
 }
 0x40b7850 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b787c; continue;
 }
 0x40b787c => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b78d5; continue; }
  pc = 0x40b78a9; continue;
 }
 0x40b78a9 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b78d5; continue;
 }
 0x40b78d5 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b792e; continue; }
  pc = 0x40b7902; continue;
 }
 0x40b7902 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x40b792e; continue;
 }
 0x40b792e => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x415372cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40b77ca; continue; }
  pc = 0x40b793d; continue;
 }
 0x40b793d => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x415372cu32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4153680u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40b794e => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b79a4; continue; }
  pc = 0x40b797b; continue;
 }
 0x40b797b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b79a4; continue;
 }
 0x40b79a4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b79a7; continue;
 }
 0x40b79a7 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b79fd; continue; }
  pc = 0x40b79d4; continue;
 }
 0x40b79d4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b79fd; continue;
 }
 0x40b79fd => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7a00; continue;
 }
 0x40b7a00 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7a56; continue; }
  pc = 0x40b7a2d; continue;
 }
 0x40b7a2d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7a56; continue;
 }
 0x40b7a56 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7a59; continue;
 }
 0x40b7a59 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7a99; continue; }
  pc = 0x40b7a6e; continue;
 }
 0x40b7a6e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x10820u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7a99; continue;
 }
 0x40b7a99 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x40b7bac; continue;
 }
 0x40b7aa1 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7af7; continue; }
  pc = 0x40b7ace; continue;
 }
 0x40b7ace => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7af7; continue;
 }
 0x40b7af7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7afa; continue;
 }
 0x40b7afa => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7b50; continue; }
  pc = 0x40b7b27; continue;
 }
 0x40b7b27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7b50; continue;
 }
 0x40b7b50 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7b53; continue;
 }
 0x40b7b53 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x40b7ba9; continue; }
  pc = 0x40b7b80; continue;
 }
 0x40b7b80 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x4281c14u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7ba9; continue;
 }
 0x40b7ba9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40b7bac; continue;
 }
 0x40b7bac => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x4153708u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4153714u32)); cpu.edi = v; }
  pc = 0x40b7bbb; continue;
 }
 0x40b7bbb => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153728u32)); cpu.w32(0x4153728u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7746; continue; }
  pc = 0x40b7bc7; continue;
 }
 0x40b7bc7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b7bca => {
  { let v: u32 = cpu.r32(0x4153700u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41536e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153728u32, v); }
  pc = 0x40b7c58; continue;
 }
 0x40b7bdc => {
  { let v: u32 = cpu.r32(0x41536e8u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x4153710u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x41536fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x41536e0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415372cu32, v); }
  pc = 0x40b7c06; continue;
 }
 0x40b7c06 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x40b7c3e; continue; }
  pc = 0x40b7c25; continue;
 }
 0x40b7c25 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x4225960u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7c3e; continue;
 }
 0x40b7c3e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x415372cu32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b7c06; continue; }
  pc = 0x40b7c49; continue;
 }
 0x40b7c49 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x4153708u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4153714u32)); cpu.edi = v; }
  pc = 0x40b7c58; continue;
 }
 0x40b7c58 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153728u32)); cpu.w32(0x4153728u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7bdc; continue; }
  pc = 0x40b7c64; continue;
 }
 0x40b7c64 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b7c67 => {
  { let v: u32 = cpu.r32(0x4153700u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41536e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153728u32, v); }
  pc = 0x40b7cd6; continue;
 }
 0x40b7c79 => {
  { let v: u32 = cpu.r32(0x41536e8u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x4153710u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x41536fcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x41536e0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415372cu32, v); }
  pc = 0x40b7ca3; continue;
 }
 0x40b7ca3 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x4153704u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x40b7cbc; continue; }
  pc = 0x40b7cb9; continue;
 }
 0x40b7cb9 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b7cbc; continue;
 }
 0x40b7cbc => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x415372cu32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b7ca3; continue; }
  pc = 0x40b7cc7; continue;
 }
 0x40b7cc7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x4153708u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4153714u32)); cpu.edi = v; }
  pc = 0x40b7cd6; continue;
 }
 0x40b7cd6 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x4153728u32)); cpu.w32(0x4153728u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7c79; continue; }
  pc = 0x40b7cde; continue;
 }
 0x40b7cde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7ce1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7ce1); }
 let mut pc: u32 = 0x40b7ce1;
 loop { match pc {
 0x40b7ce1 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7cf7; continue; }
  pc = 0x40b7ceb; continue;
 }
 0x40b7ceb => {
  if cpu.zf { pc = 0x40b7cfd; continue; }
  pc = 0x40b7ced; continue;
 }
 0x40b7ced => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x40b7ce1; continue; }
  pc = 0x40b7cf3; continue;
 }
 0x40b7cf3 => {
  { cpu.movs(2, false); }
  cpu.poll(); pc = 0x40b7ce1; continue;
 }
 0x40b7cf7 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40b7ce1; continue;
 }
 0x40b7cfd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7cfe); }
 let mut pc: u32 = 0x40b7cfe;
 loop { match pc {
 0x40b7cfe => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7d39; continue; }
  pc = 0x40b7d08; continue;
 }
 0x40b7d08 => {
  if cpu.zf { pc = 0x40b7d3f; continue; }
  pc = 0x40b7d0a; continue;
 }
 0x40b7d0a => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  pc = 0x40b7d1d; continue;
 }
 0x40b7d10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x40b7d1d; continue;
 }
 0x40b7d1d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7d10; continue; }
  pc = 0x40b7d20; continue;
 }
 0x40b7d20 => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x40b7cfe; continue; }
  pc = 0x40b7d28; continue;
 }
 0x40b7d28 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40b7cfe; continue;
 }
 0x40b7d39 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40b7cfe; continue;
 }
 0x40b7d3f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7d40); }
 let mut pc: u32 = 0x40b7d40;
 loop { match pc {
 0x40b7d40 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7d76; continue; }
  pc = 0x40b7d4a; continue;
 }
 0x40b7d4a => {
  if cpu.zf { pc = 0x40b7d7c; continue; }
  pc = 0x40b7d4c; continue;
 }
 0x40b7d4c => {
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x4225960u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b7d4c; continue; }
  pc = 0x40b7d74; continue;
 }
 0x40b7d74 => {
  cpu.poll(); pc = 0x40b7d40; continue;
 }
 0x40b7d76 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40b7d40; continue;
 }
 0x40b7d7c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7d7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7d7d); }
 let mut pc: u32 = 0x40b7d7d;
 loop { match pc {
 0x40b7d7d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x40b7d81; continue;
 }
 0x40b7d81 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b7dae; continue; }
  pc = 0x40b7d8b; continue;
 }
 0x40b7d8b => {
  if cpu.zf { pc = 0x40b7db4; continue; }
  pc = 0x40b7d8d; continue;
 }
 0x40b7d8d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b7d8d; continue; }
  pc = 0x40b7dac; continue;
 }
 0x40b7dac => {
  cpu.poll(); pc = 0x40b7d81; continue;
 }
 0x40b7dae => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40b7d81; continue;
 }
 0x40b7db4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7db5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7db5); }
 let mut pc: u32 = 0x40b7db5;
 loop { match pc {
 0x40b7db5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x41536d0u32, v); }
  { let v = cpu.rf32(0x41536c0u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x42a2828u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x41536ccu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x41536b8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x41536b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536c4u32, v); }
  { let v: u32 = cpu.r32(0x41536b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4153748u32, v); }
  { let v: u32 = cpu.r32(0x41536acu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415374cu32, v); }
  pc = 0x40b7df4; continue;
 }
 0x40b7df4 => {
  { let v: u32 = cpu.r32(0x41536d0u32); cpu.esi = v; }
  pc = 0x40b7dfa; continue;
 }
 0x40b7dfa => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x40b7fdd; continue; }
  pc = 0x40b7e07; continue;
 }
 0x40b7e07 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x40b7fcf; continue; }
  pc = 0x40b7e10; continue;
 }
 0x40b7e10 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.rf32(0x41536ccu32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x415374cu32); cpu.ebp = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x41536bcu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x41536c8u32, v); }
  { cpu.and32(cpu.ebp, 0x2u32); }
  if !cpu.zf { pc = 0x40b7eb6; continue; }
  pc = 0x40b7e54; continue;
 }
 0x40b7e54 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x41536c0u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4153738u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b7eb3; continue; }
  pc = 0x40b7e6a; continue;
 }
 0x40b7e6a => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536dcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4153738u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x40b7e8a; continue; }
  pc = 0x40b7e88; continue;
 }
 0x40b7e88 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40b7e8a; continue;
 }
 0x40b7e8a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x4227960u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x4247960u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b7eb3; continue;
 }
 0x40b7eb3 => {
  { let v: u32 = cpu.ebp.wrapping_sub(0x2u32); cpu.ebp = v; }
  pc = 0x40b7eb6; continue;
 }
 0x40b7eb6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x41536c8u32)); cpu.eax = v; }
  if cpu.zf { pc = 0x40b7f62; continue; }
  pc = 0x40b7ec4; continue;
 }
 0x40b7ec4 => {
  if cpu.sf { pc = 0x40b7fc1; continue; }
  pc = 0x40b7eca; continue;
 }
 0x40b7eca => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  pc = 0x40b7ecc; continue;
 }
 0x40b7ecc => {
  { let v: u32 = cpu.r32(0x41536c0u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4153738u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b7f4d; continue; }
  pc = 0x40b7ee0; continue;
 }
 0x40b7ee0 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536dcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4153738u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x40b7f00; continue; }
  pc = 0x40b7efe; continue;
 }
 0x40b7efe => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40b7f00; continue;
 }
 0x40b7f00 => {
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe00000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x422795eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1f0000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x424795eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4227960u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x4247960u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffeu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b7f4d; continue;
 }
 0x40b7f4d => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x41536c8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7ecc; continue; }
  pc = 0x40b7f5e; continue;
 }
 0x40b7f5e => {
  if !cpu.zf { pc = 0x40b7fc1; continue; }
  pc = 0x40b7f60; continue;
 }
 0x40b7f60 => {
  pc = 0x40b7f64; continue;
 }
 0x40b7f62 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  pc = 0x40b7f64; continue;
 }
 0x40b7f64 => {
  { let v: u32 = cpu.r32(0x41536c0u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4153738u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b7fc1; continue; }
  pc = 0x40b7f78; continue;
 }
 0x40b7f78 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536dcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4153738u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4153738u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x40b7f98; continue; }
  pc = 0x40b7f96; continue;
 }
 0x40b7f96 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40b7f98; continue;
 }
 0x40b7f98 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4227960u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x4247960u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40b7fc1; continue;
 }
 0x40b7fc1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.poll(); pc = 0x40b7dfa; continue;
 }
 0x40b7fcf => {
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7e10; continue; }
  pc = 0x40b7fd8; continue;
 }
 0x40b7fd8 => {
  cpu.poll(); pc = 0x40b7dfa; continue;
 }
 0x40b7fdd => {
  { let v: u32 = cpu.r32(0x41536d0u32).wrapping_add(0x1cu32); cpu.w32(0x41536d0u32, v); }
  { let v: u32 = cpu.r32(0x41536c4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x415374cu32), cpu.eax); cpu.w32(0x415374cu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x4153748u32)); cpu.w32(0x4153748u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b7df4; continue; }
  pc = 0x40b7ffb; continue;
 }
 0x40b7ffb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b7ffd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b7ffd); }
 let mut pc: u32 = 0x40b7ffd;
 loop { match pc {
 0x40b7ffd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4248160u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x40b8010; continue;
 }
 0x40b8010 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b806c; continue; }
  pc = 0x40b8014; continue;
 }
 0x40b8014 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4248168u32, v); }
  { let v: u32 = cpu.r32(0x4248160u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4248168u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42484c4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4248978u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x4248160u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4248164u32, v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4248168u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42484c4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x4248164u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4248978u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x40b8072; continue;
 }
 0x40b806c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40b8072; continue;
 }
 0x40b8072 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8010; continue; }
  pc = 0x40b8076; continue;
 }
 0x40b8076 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8078); }
 let mut pc: u32 = 0x40b8078;
 loop { match pc {
 0x40b8078 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x424816cu32, v); }
  { let v: u16 = cpu.r16(0x424816cu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp | 0xc00u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x424816eu32, v); }
  { cpu.fpu_control = cpu.r16(0x424816eu32); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42484b0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x42484b4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42484a4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x42484a8u32, v); }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42484acu32, v); }
  { let v = (cpu.r32(0x42484b4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x42484b8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x41536a8u32), cpu.ebx); }
  if cpu.zf { pc = 0x40b80f4; continue; }
  pc = 0x40b80da; continue;
 }
 0x40b80da => {
  { cpu.sub32(cpu.ebx, 0x96u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8369; continue; }
  pc = 0x40b80e6; continue;
 }
 0x40b80e6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536a8u32, v); }
  cpu.push(0x40b80f3); f_040b7ffd(cpu); if cpu.returned_to != 0x40b80f3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40b80f4; continue;
 }
 0x40b80f4 => {
  { let v: u32 = 0x424817cu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4248170u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x40b8104; continue;
 }
 0x40b8104 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x40b8107; continue;
 }
 0x40b8107 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b8117; continue; }
  pc = 0x40b8115; continue;
 }
 0x40b8115 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x40b8117; continue;
 }
 0x40b8117 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b81e3; continue; }
  pc = 0x40b8121; continue;
 }
 0x40b8121 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x42484b8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b81e3; continue; }
  pc = 0x40b8134; continue;
 }
 0x40b8134 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b818e; continue; }
  pc = 0x40b8149; continue;
 }
 0x40b8149 => {
  { let v = cpu.rf32(0x41536a0u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x41536a0u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b81e3; continue; }
  pc = 0x40b8176; continue;
 }
 0x40b8176 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b81df; continue;
 }
 0x40b818e => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b81e3; continue; }
  pc = 0x40b81c9; continue;
 }
 0x40b81c9 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b81df; continue;
 }
 0x40b81df => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x40b81e3; continue;
 }
 0x40b81e3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b8104; continue; }
  pc = 0x40b81ed; continue;
 }
 0x40b81ed => {
  { let v: u32 = cpu.r32(0x4248170u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x40b8107; continue; }
  pc = 0x40b81fb; continue;
 }
 0x40b81fb => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x40b8369; continue; }
  pc = 0x40b8203; continue;
 }
 0x40b8203 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4248174u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4248178u32, v); }
  pc = 0x40b8213; continue;
 }
 0x40b8213 => {
  { let v: u32 = 0x424817cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4248174u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b8225; continue;
 }
 0x40b8225 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b823e; continue; }
  pc = 0x40b822a; continue;
 }
 0x40b822a => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b823e; continue; }
  pc = 0x40b822f; continue;
 }
 0x40b822f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42483fcu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x40b823e; continue;
 }
 0x40b823e => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8225; continue; }
  pc = 0x40b8244; continue;
 }
 0x40b8244 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b8347; continue; }
  pc = 0x40b824d; continue;
 }
 0x40b824d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x42483fcu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x40b8259; continue;
 }
 0x40b8259 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x40b825d; continue;
 }
 0x40b825d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b826b; continue; }
  pc = 0x40b8266; continue;
 }
 0x40b8266 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  pc = 0x40b826b; continue;
 }
 0x40b826b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b825d; continue; }
  pc = 0x40b826e; continue;
 }
 0x40b826e => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8259; continue; }
  pc = 0x40b8271; continue;
 }
 0x40b8271 => {
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484c4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4248978u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42484bcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42484c0u32, v); }
  { let v: u32 = 0x42483fcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42484acu32); cpu.ebp = v; }
  pc = 0x40b829a; continue;
 }
 0x40b829a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8000u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8000u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42484c0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40b833e; continue; }
  pc = 0x40b82bf; continue;
 }
 0x40b82bf => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42484bcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b833e; continue; }
  pc = 0x40b82c7; continue;
 }
 0x40b82c7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x42484bcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b82d4; continue; }
  pc = 0x40b82cf; continue;
 }
 0x40b82cf => {
  { let v: u32 = cpu.r32(0x42484bcu32); cpu.eax = v; }
  pc = 0x40b82d4; continue;
 }
 0x40b82d4 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42484c0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b82e2; continue; }
  pc = 0x40b82dc; continue;
 }
 0x40b82dc => {
  { let v: u32 = cpu.r32(0x42484c0u32); cpu.ebx = v; }
  pc = 0x40b82e2; continue;
 }
 0x40b82e2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b833e; continue; }
  pc = 0x40b82e7; continue;
 }
 0x40b82e7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484a4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40b830e; continue; }
  pc = 0x40b82f9; continue;
 }
 0x40b82f9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b833e; continue; }
  pc = 0x40b830e; continue;
 }
 0x40b830e => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x40b8327; continue; }
  pc = 0x40b8314; continue;
 }
 0x40b8314 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8314; continue; }
  pc = 0x40b8327; continue;
 }
 0x40b8327 => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40b833e; continue; }
  pc = 0x40b832f; continue;
 }
 0x40b832f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b833e; continue;
 }
 0x40b833e => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b829a; continue; }
  pc = 0x40b8347; continue;
 }
 0x40b8347 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4248178u32)); cpu.w32(0x4248178u32, v); }
  { let v: u32 = cpu.r32(0x42484a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484a4u32).wrapping_add(cpu.eax); cpu.w32(0x42484a4u32, v); }
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42484b4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x40b8213; continue; }
  pc = 0x40b8369; continue;
 }
 0x40b8369 => {
  { cpu.fpu_control = cpu.r16(0x424816cu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8371(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8371); }
 let mut pc: u32 = 0x40b8371;
 loop { match pc {
 0x40b8371 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x424816cu32, v); }
  { let v: u16 = cpu.r16(0x424816cu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp | 0xc00u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x424816eu32, v); }
  { cpu.fpu_control = cpu.r16(0x424816eu32); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42484b0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x42484b4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42484a4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x42484a8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x42484acu32, v); }
  { let v = (cpu.r32(0x42484b4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x42484b8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x41536a8u32), cpu.ebx); }
  if cpu.zf { pc = 0x40b83e5; continue; }
  pc = 0x40b83cb; continue;
 }
 0x40b83cb => {
  { cpu.sub32(cpu.ebx, 0x96u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8633; continue; }
  pc = 0x40b83d7; continue;
 }
 0x40b83d7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x41536a8u32, v); }
  cpu.push(0x40b83e4); f_040b7ffd(cpu); if cpu.returned_to != 0x40b83e4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40b83e5; continue;
 }
 0x40b83e5 => {
  { let v: u32 = 0x424817cu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4248170u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x40b83f5; continue;
 }
 0x40b83f5 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x40b83f8; continue;
 }
 0x40b83f8 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b8408; continue; }
  pc = 0x40b8406; continue;
 }
 0x40b8406 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x40b8408; continue;
 }
 0x40b8408 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b84d4; continue; }
  pc = 0x40b8412; continue;
 }
 0x40b8412 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x42484b8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40b84d4; continue; }
  pc = 0x40b8425; continue;
 }
 0x40b8425 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b847f; continue; }
  pc = 0x40b843a; continue;
 }
 0x40b843a => {
  { let v = cpu.rf32(0x41536a0u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x41536a0u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b84d4; continue; }
  pc = 0x40b8467; continue;
 }
 0x40b8467 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b84d0; continue;
 }
 0x40b847f => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x41536a0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b84d4; continue; }
  pc = 0x40b84ba; continue;
 }
 0x40b84ba => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x41536a4u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b84d0; continue;
 }
 0x40b84d0 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x40b84d4; continue;
 }
 0x40b84d4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b83f5; continue; }
  pc = 0x40b84de; continue;
 }
 0x40b84de => {
  { let v: u32 = cpu.r32(0x4248170u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x40b83f8; continue; }
  pc = 0x40b84ec; continue;
 }
 0x40b84ec => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x40b8633; continue; }
  pc = 0x40b84f4; continue;
 }
 0x40b84f4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4248174u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4248178u32, v); }
  pc = 0x40b8504; continue;
 }
 0x40b8504 => {
  { let v: u32 = 0x424817cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4248174u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b8516; continue;
 }
 0x40b8516 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b852f; continue; }
  pc = 0x40b851b; continue;
 }
 0x40b851b => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b852f; continue; }
  pc = 0x40b8520; continue;
 }
 0x40b8520 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42483fcu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x40b852f; continue;
 }
 0x40b852f => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8516; continue; }
  pc = 0x40b8535; continue;
 }
 0x40b8535 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b8611; continue; }
  pc = 0x40b853e; continue;
 }
 0x40b853e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x42483fcu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x40b854a; continue;
 }
 0x40b854a => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x40b854e; continue;
 }
 0x40b854e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b855c; continue; }
  pc = 0x40b8557; continue;
 }
 0x40b8557 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  pc = 0x40b855c; continue;
 }
 0x40b855c => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b854e; continue; }
  pc = 0x40b855f; continue;
 }
 0x40b855f => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b854a; continue; }
  pc = 0x40b8562; continue;
 }
 0x40b8562 => {
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484c4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4248978u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42484bcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42484c0u32, v); }
  { let v: u32 = 0x42483fcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42484acu32); cpu.ebp = v; }
  pc = 0x40b858b; continue;
 }
 0x40b858b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8000u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8000u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42484c0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40b8608; continue; }
  pc = 0x40b85ac; continue;
 }
 0x40b85ac => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42484bcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b8608; continue; }
  pc = 0x40b85b4; continue;
 }
 0x40b85b4 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x42484bcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b85c1; continue; }
  pc = 0x40b85bc; continue;
 }
 0x40b85bc => {
  { let v: u32 = cpu.r32(0x42484bcu32); cpu.eax = v; }
  pc = 0x40b85c1; continue;
 }
 0x40b85c1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42484c0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b85cf; continue; }
  pc = 0x40b85c9; continue;
 }
 0x40b85c9 => {
  { let v: u32 = cpu.r32(0x42484c0u32); cpu.ebx = v; }
  pc = 0x40b85cf; continue;
 }
 0x40b85cf => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b8608; continue; }
  pc = 0x40b85d4; continue;
 }
 0x40b85d4 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484a4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40b85ef; continue; }
  pc = 0x40b85e6; continue;
 }
 0x40b85e6 => {
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b8608; continue; }
  pc = 0x40b85ef; continue;
 }
 0x40b85ef => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x40b85fd; continue; }
  pc = 0x40b85f5; continue;
 }
 0x40b85f5 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b85f5; continue; }
  pc = 0x40b85fd; continue;
 }
 0x40b85fd => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40b8608; continue; }
  pc = 0x40b8605; continue;
 }
 0x40b8605 => {
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b8608; continue;
 }
 0x40b8608 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b858b; continue; }
  pc = 0x40b8611; continue;
 }
 0x40b8611 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4248178u32)); cpu.w32(0x4248178u32, v); }
  { let v: u32 = cpu.r32(0x42484a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42484a4u32).wrapping_add(cpu.eax); cpu.w32(0x42484a4u32, v); }
  { let v: u32 = cpu.r32(0x4248178u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42484b4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x40b8504; continue; }
  pc = 0x40b8633; continue;
 }
 0x40b8633 => {
  { cpu.fpu_control = cpu.r16(0x424816cu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8640); }
 let mut pc: u32 = 0x40b8640;
 loop { match pc {
 0x40b8640 => {
  { let v: u32 = cpu.eax; cpu.w32(0x42555ecu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42555f0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x42555f4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x42555f8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42555fcu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x425560cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8668(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8668); }
 let mut pc: u32 = 0x40b8668;
 loop { match pc {
 0x40b8668 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42555f8u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255600u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255604u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255608u32, v); }
  { cpu.sub32(cpu.r32(0x42555f8u32), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b86a4; continue; }
  pc = 0x40b8698; continue;
 }
 0x40b8698 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40b869e); f_040ba685(cpu); if cpu.returned_to != 0x40b869e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42555f8u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x40b86a4; continue;
 }
 0x40b86a4 => {
  { let v: u32 = cpu.r32(0x4255600u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4255604u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255608u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42555ecu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42555f0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x42555f4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x425560cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b86da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b86da); }
 let mut pc: u32 = 0x40b86da;
 loop { match pc {
 0x40b86da => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4255784u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255760u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4255758u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255754u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x425575cu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b87d9; continue; }
  pc = 0x40b8705; continue;
 }
 0x40b8705 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b870a; continue;
 }
 0x40b870a => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 1u32); cpu.edi = v; }
  if cpu.zf { pc = 0x40b8730; continue; }
  pc = 0x40b8712; continue;
 }
 0x40b8712 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.r32(0x4255758u32); cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x42555f0u32)); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b872c; continue; }
  pc = 0x40b8728; continue;
 }
 0x40b8728 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.poll(); pc = 0x40b870a; continue;
 }
 0x40b872c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.poll(); pc = 0x40b870a; continue;
 }
 0x40b8730 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255768u32, v); }
  { let v = cpu.r32(0x4255758u32); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4255764u32, v); }
  pc = 0x40b874a; continue;
 }
 0x40b874a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x42555f0u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8757; continue; }
  pc = 0x40b8755; continue;
 }
 0x40b8755 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b8757; continue;
 }
 0x40b8757 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4255754u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8781; continue; }
  pc = 0x40b875f; continue;
 }
 0x40b875f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x425575cu32); cpu.ebx = v; }
  cpu.push(0x40b876c); f_040b887c(cpu); if cpu.returned_to != 0x40b876c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8778; continue; }
  pc = 0x40b8772; continue;
 }
 0x40b8772 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x425576cu32, v); }
  pc = 0x40b8778; continue;
 }
 0x40b8778 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x4255758u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b874a; continue; }
  pc = 0x40b8781; continue;
 }
 0x40b8781 => {
  { let v: u32 = cpu.r32(0x4255764u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255768u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4255758u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x4255760u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40b87d9; continue; }
  pc = 0x40b879c; continue;
 }
 0x40b879c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x42555f0u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b87a9; continue; }
  pc = 0x40b87a7; continue;
 }
 0x40b87a7 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b87a9; continue;
 }
 0x40b87a9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4255754u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b87d9; continue; }
  pc = 0x40b87b1; continue;
 }
 0x40b87b1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x425575cu32); cpu.ebx = v; }
  cpu.push(0x40b87be); f_040b887c(cpu); if cpu.returned_to != 0x40b87be { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b87ca; continue; }
  pc = 0x40b87c4; continue;
 }
 0x40b87c4 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x425576cu32, v); }
  pc = 0x40b87ca; continue;
 }
 0x40b87ca => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4255758u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x4255760u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40b879c; continue; }
  pc = 0x40b87d9; continue;
 }
 0x40b87d9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4255784u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x40b87ec; continue; }
  pc = 0x40b87e4; continue;
 }
 0x40b87e4 => {
  { let v: u32 = cpu.r32(0x425576cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x40b87ec; continue;
 }
 0x40b87ec => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b87ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b87ee); }
 let mut pc: u32 = 0x40b87ee;
 loop { match pc {
 0x40b87ee => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4255784u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b883e; continue; }
  pc = 0x40b87fe; continue;
 }
 0x40b87fe => {
  { let v: u32 = cpu.edx; cpu.w32(0x4255774u32, v); }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255770u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255778u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x425577cu32, v); }
  pc = 0x40b8819; continue;
 }
 0x40b8819 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4255778u32); cpu.ebx = v; }
  cpu.push(0x40b8825); f_040b887c(cpu); if cpu.returned_to != 0x40b8825 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8830; continue; }
  pc = 0x40b882a; continue;
 }
 0x40b882a => {
  { let v: u32 = cpu.esi; cpu.w32(0x4255780u32, v); }
  pc = 0x40b8830; continue;
 }
 0x40b8830 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4255774u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4255770u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40b8819; continue; }
  pc = 0x40b883e; continue;
 }
 0x40b883e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4255784u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x40b8863; continue; }
  pc = 0x40b8849; continue;
 }
 0x40b8849 => {
  { let v: u32 = cpu.r32(0x4255780u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x425577cu32)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x4255774u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4255784u32); cpu.eax = v; }
  pc = 0x40b8863; continue;
 }
 0x40b8863 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8865(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8865); }
 let mut pc: u32 = 0x40b8865;
 loop { match pc {
 0x40b8865 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4255784u32, v); }
  cpu.push(0x40b8875); f_040b887c(cpu); if cpu.returned_to != 0x40b8875 { return; }
  { let v: u32 = cpu.r32(0x4255784u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b887c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b887c); }
 let mut pc: u32 = 0x40b887c;
 loop { match pc {
 0x40b887c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40b8a12; continue; }
  pc = 0x40b888e; continue;
 }
 0x40b888e => {
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebp)), 0x4u16); }
  if cpu.zf { pc = 0x40b8926; continue; }
  pc = 0x40b889a; continue;
 }
 0x40b889a => {
  { let v: u32 = cpu.r32(0x42555f8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b88b6; continue;
 }
 0x40b88b6 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f4u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b88cc; continue;
 }
 0x40b88cc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f0u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b88e2; continue;
 }
 0x40b88e2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f0u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b88f8; continue;
 }
 0x40b88f8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555ecu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b890e; continue;
 }
 0x40b890e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555ecu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b8924; continue;
 }
 0x40b8924 => {
  pc = 0x40b897b; continue;
 }
 0x40b8926 => {
  { let v: u32 = cpu.r32(0x42555f8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42555f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b8943; continue;
 }
 0x40b8943 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b894b; continue;
 }
 0x40b894b => {
  { let v: u32 = cpu.r32(0x42555ecu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b895b; continue;
 }
 0x40b895b => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b8963; continue;
 }
 0x40b8963 => {
  { let v: u32 = cpu.r32(0x42555f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b8973; continue;
 }
 0x40b8973 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40b8a12; continue; }
  pc = 0x40b897b; continue;
 }
 0x40b897b => {
  { let v: u32 = cpu.r32(0x42555f8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4154780u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4154784u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42555e0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255610u32, v); }
  { cpu.sub32(cpu.r32(0x42555e0u32), 0x0u32); }
  if cpu.zf { pc = 0x40b8a18; continue; }
  pc = 0x40b89a5; continue;
 }
 0x40b89a5 => {
  { let v: u32 = cpu.r32(0x4154784u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255620u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255624u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255628u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4154764u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4154768u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x415476cu32, v); }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.esi = v; }
  { let v: u32 = 0x2au32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.esi = v; }
  pc = 0x40b89e1; continue;
 }
 0x40b89e1 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40b89fc; continue; }
  pc = 0x40b89eb; continue;
 }
 0x40b89eb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x415478cu32.wrapping_add(cpu.ebx)); cpu.push(0x40b89f6); cpu.call(t); if cpu.returned_to != 0x40b89f6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40b89e1; continue;
 }
 0x40b89fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8a12; continue; }
  pc = 0x40b8a02; continue;
 }
 0x40b8a02 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42555e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.push(0x40b8a11); f_040b8a23(cpu); if cpu.returned_to != 0x40b8a11 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8a12 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8a18 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255784u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8a23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8a23); }
 let mut pc: u32 = 0x40b8a23;
 loop { match pc {
 0x40b8a23 => {
  { let v: u32 = cpu.r32(0x42555e4u32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x425578cu32, v); }
  cpu.push(0x40b8a38); f_040b8a39(cpu); if cpu.returned_to != 0x40b8a38 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8a39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8a39); }
 let mut pc: u32 = 0x40b8a39;
 loop { match pc {
 0x40b8a39 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40b8a5a; continue; }
  pc = 0x40b8a43; continue;
 }
 0x40b8a43 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40b8a52); f_040b8a39(cpu); if cpu.returned_to != 0x40b8a52 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x40b8a58); f_040b8a39(cpu); if cpu.returned_to != 0x40b8a58 { return; }
  pc = 0x40b8a7f; continue;
 }
 0x40b8a5a => {
  { let v: u32 = cpu.eax; cpu.w32(0x4255788u32, v); }
  pc = 0x40b8a5f; continue;
 }
 0x40b8a5f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40b8a65); f_040b8a85(cpu); if cpu.returned_to != 0x40b8a65 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8a6f; continue; }
  pc = 0x40b8a6a; continue;
 }
 0x40b8a6a => {
  { let v: u32 = cpu.eax; cpu.w32(0x425578cu32, v); }
  pc = 0x40b8a6f; continue;
 }
 0x40b8a6f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x4255788u32)); cpu.w32(0x4255788u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40b8a5f; continue; }
  pc = 0x40b8a7f; continue;
 }
 0x40b8a7f => {
  { let v: u32 = cpu.r32(0x425578cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8a85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8a85); }
 let mut pc: u32 = 0x40b8a85;
 loop { match pc {
 0x40b8a85 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x42555e0u32), 0x2u32); }
  if cpu.zf { pc = 0x40b8ae0; continue; }
  pc = 0x40b8a8f; continue;
 }
 0x40b8a8f => {
  { let v: u32 = cpu.r32(0x42555ecu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42555f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255614u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255618u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x425561cu32, v); }
  cpu.push(0x40b8ab6); f_040b8caf(cpu); if cpu.returned_to != 0x40b8ab6 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42555f8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8b63; continue; }
  pc = 0x40b8ac2; continue;
 }
 0x40b8ac2 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x42555fcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8b63; continue; }
  pc = 0x40b8ace; continue;
 }
 0x40b8ace => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x40b8ad6); f_040b8d35(cpu); if cpu.returned_to != 0x40b8ad6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8b63; continue; }
  pc = 0x40b8ade; continue;
 }
 0x40b8ade => {
  pc = 0x40b8b26; continue;
 }
 0x40b8ae0 => {
  { let v: u32 = cpu.r32(0x42555ecu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42555f0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42555f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255614u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255618u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x425561cu32, v); }
  cpu.push(0x40b8b07); f_040b8b6a(cpu); if cpu.returned_to != 0x40b8b07 { return; }
  if cpu.zf { pc = 0x40b8b63; continue; }
  pc = 0x40b8b09; continue;
 }
 0x40b8b09 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4255614u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255618u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x425561cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x40b8b22); f_040b8d35(cpu); if cpu.returned_to != 0x40b8b22 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b8b63; continue; }
  pc = 0x40b8b26; continue;
 }
 0x40b8b26 => {
  { let v: u32 = cpu.r32(0x425560cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.sf == cpu.of) { pc = 0x40b8b57; continue; }
  pc = 0x40b8b30; continue;
 }
 0x40b8b30 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4255638u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v: u32 = 0x4255620u32; cpu.esi = v; }
  { let v: u32 = 0x425563cu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.inc32(cpu.r32(0x425560cu32)); cpu.w32(0x425560cu32, v); }
  pc = 0x40b8b57; continue;
 }
 0x40b8b57 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255784u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8b63 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8b6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8b6a); }
 let mut pc: u32 = 0x40b8b6a;
 loop { match pc {
 0x40b8b6a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8b77; continue; }
  pc = 0x40b8b75; continue;
 }
 0x40b8b75 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b8b77; continue;
 }
 0x40b8b77 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x425c830u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b8b99; continue; }
  pc = 0x40b8b93; continue;
 }
 0x40b8b93 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b8b99; continue;
 }
 0x40b8b99 => {
  { let v: u32 = cpu.eax; cpu.w32(0x425562cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255630u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255634u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255600u32); cpu.eax = v; }
  { let v = cpu.r32(0x425562cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255604u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255630u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255608u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255634u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.fpush(1.0); }
  { let v: u32 = cpu.eax; cpu.w32(0x425579cu32, v); }
  { let v = (cpu.r32(0x425579cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4255614u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255620u32)); cpu.eax = v; }
  { let v = cpu.r32(0x425562cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255618u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255624u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4255630u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x425561cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255628u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4255634u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x425579cu32, v); }
  { let v = (cpu.r32(0x425579cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x425579cu32, v); }
  { let v: u32 = cpu.r32(0x425579cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3f800000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8caa; continue; }
  pc = 0x40b8c57; continue;
 }
 0x40b8c57 => {
  { let v = (cpu.r32(0x4255600u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4255614u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4255790u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4255604u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4255618u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4255794u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4255608u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x425561cu32); cpu.ecx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4255798u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4255790u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4255794u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4255798u32)); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.or32(cpu.edx, 0xffffffffu32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8caa => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8caf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8caf); }
 let mut pc: u32 = 0x40b8caf;
 loop { match pc {
 0x40b8caf => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8cbb; continue; }
  pc = 0x40b8cb9; continue;
 }
 0x40b8cb9 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b8cbb; continue;
 }
 0x40b8cbb => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x425c830u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40b8cdd; continue; }
  pc = 0x40b8cd7; continue;
 }
 0x40b8cd7 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b8cdd; continue;
 }
 0x40b8cdd => {
  { let v: u32 = cpu.eax; cpu.w32(0x425562cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4255630u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4255634u32, v); }
  { let v: u32 = cpu.r32(0x4255614u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255620u32)); cpu.eax = v; }
  { let v = cpu.r32(0x425562cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255618u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255624u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4255630u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x425561cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255628u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4255634u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8d35); }
 let mut pc: u32 = 0x40b8d35;
 loop { match pc {
 0x40b8d35 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x42557c4u32; cpu.edi = v; }
  cpu.push(0x40b8d40); f_040b8ed8(cpu); if cpu.returned_to != 0x40b8d40 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255614u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557acu32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8d54; continue;
 }
 0x40b8d54 => {
  { let v: u32 = cpu.r32(0x4255614u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557b8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8d67; continue;
 }
 0x40b8d67 => {
  { let v: u32 = cpu.r32(0x4255618u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557b0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8d7a; continue;
 }
 0x40b8d7a => {
  { let v: u32 = cpu.r32(0x4255618u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557bcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8d8d; continue;
 }
 0x40b8d8d => {
  { let v: u32 = cpu.r32(0x425561cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8da0; continue;
 }
 0x40b8da0 => {
  { let v: u32 = cpu.r32(0x425561cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x42555f8u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557c0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8db9; continue; }
  pc = 0x40b8db3; continue;
 }
 0x40b8db3 => {
  cpu.push(0x40b8db8); f_040b8dbc(cpu); if cpu.returned_to != 0x40b8db8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8db9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8dbc); }
 let mut pc: u32 = 0x40b8dbc;
 loop { match pc {
 0x40b8dbc => {
  { let v: u32 = cpu.r32(0x425562cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4255630u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255634u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8dd3; continue; }
  pc = 0x40b8dd1; continue;
 }
 0x40b8dd1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40b8dd3; continue;
 }
 0x40b8dd3 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8dd9; continue; }
  pc = 0x40b8dd7; continue;
 }
 0x40b8dd7 => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40b8dd9; continue;
 }
 0x40b8dd9 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8ddf; continue; }
  pc = 0x40b8ddd; continue;
 }
 0x40b8ddd => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x40b8ddf; continue;
 }
 0x40b8ddf => {
  { let v: u32 = 0x42557c4u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b8e3e; continue; }
  pc = 0x40b8ded; continue;
 }
 0x40b8ded => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40b8e3e; continue; }
  pc = 0x40b8df1; continue;
 }
 0x40b8df1 => {
  { let v: u32 = cpu.r32(0x4255634u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x40b8dfa; continue;
 }
 0x40b8dfa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255614u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x4255618u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255614u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x4255618u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b8ed1; continue; }
  pc = 0x40b8e33; continue;
 }
 0x40b8e33 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8dfa; continue; }
  pc = 0x40b8e39; continue;
 }
 0x40b8e39 => {
  pc = 0x40b8ed4; continue;
 }
 0x40b8e3e => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b8e8a; continue; }
  pc = 0x40b8e42; continue;
 }
 0x40b8e42 => {
  { let v: u32 = cpu.r32(0x4255630u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  { let v: u32 = !cpu.edi; cpu.edi = v; }
  pc = 0x40b8e4d; continue;
 }
 0x40b8e4d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255614u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x425561cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255614u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x425561cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b8ed1; continue; }
  pc = 0x40b8e82; continue;
 }
 0x40b8e82 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8e4d; continue; }
  pc = 0x40b8e88; continue;
 }
 0x40b8e88 => {
  pc = 0x40b8ed4; continue;
 }
 0x40b8e8a => {
  { let v: u32 = cpu.r32(0x425562cu32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x40b8e93; continue;
 }
 0x40b8e93 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255618u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x425561cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4255618u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x425561cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b8ed1; continue; }
  pc = 0x40b8ec9; continue;
 }
 0x40b8ec9 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8e93; continue; }
  pc = 0x40b8ecf; continue;
 }
 0x40b8ecf => {
  pc = 0x40b8ed4; continue;
 }
 0x40b8ed1 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b8ed4 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8ed8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8ed8); }
 let mut pc: u32 = 0x40b8ed8;
 loop { match pc {
 0x40b8ed8 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xeu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42557acu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42557b0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42557b4u32, v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42557b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42557bcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42557c0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x40b8f0e; continue;
 }
 0x40b8f0e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x4257a08u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x42557acu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8f35; continue; }
  pc = 0x40b8f30; continue;
 }
 0x40b8f30 => {
  { let v: u32 = cpu.eax; cpu.w32(0x42557acu32, v); }
  pc = 0x40b8f35; continue;
 }
 0x40b8f35 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42557b0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8f43; continue; }
  pc = 0x40b8f3d; continue;
 }
 0x40b8f3d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x42557b0u32, v); }
  pc = 0x40b8f43; continue;
 }
 0x40b8f43 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x42557b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40b8f51; continue; }
  pc = 0x40b8f4b; continue;
 }
 0x40b8f4b => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42557b4u32, v); }
  pc = 0x40b8f51; continue;
 }
 0x40b8f51 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x42557b8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8f5e; continue; }
  pc = 0x40b8f59; continue;
 }
 0x40b8f59 => {
  { let v: u32 = cpu.eax; cpu.w32(0x42557b8u32, v); }
  pc = 0x40b8f5e; continue;
 }
 0x40b8f5e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x42557bcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8f6c; continue; }
  pc = 0x40b8f66; continue;
 }
 0x40b8f66 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x42557bcu32, v); }
  pc = 0x40b8f6c; continue;
 }
 0x40b8f6c => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x42557c0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40b8f7a; continue; }
  pc = 0x40b8f74; continue;
 }
 0x40b8f74 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42557c0u32, v); }
  pc = 0x40b8f7a; continue;
 }
 0x40b8f7a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8f0e; continue; }
  pc = 0x40b8f88; continue;
 }
 0x40b8f88 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8f93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8f93); }
 let mut pc: u32 = 0x40b8f93;
 loop { match pc {
 0x40b8f93 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x425563cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x425560cu32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40b8fdd; continue; }
  pc = 0x40b8fa3; continue;
 }
 0x40b8fa3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b8fa3; continue; }
  pc = 0x40b8fdd; continue;
 }
 0x40b8fdd => {
  { let v: u32 = cpu.r32(0x425560cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b8fe4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b8fe4); }
 let mut pc: u32 = 0x40b8fe4;
 loop { match pc {
 0x40b8fe4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9036(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9036); }
 let mut pc: u32 = 0x40b9036;
 loop { match pc {
 0x40b9036 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42558f4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42558f8u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x40b9048; continue;
 }
 0x40b9048 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x40b9053; continue;
 }
 0x40b9053 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b905a; continue; }
  pc = 0x40b9057; continue;
 }
 0x40b9057 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x40b905a; continue;
 }
 0x40b905a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x42558f8u32)); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b9092; continue; }
  pc = 0x40b9062; continue;
 }
 0x40b9062 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x42558f8u32)); cpu.edx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b9092; continue; }
  pc = 0x40b906a; continue;
 }
 0x40b906a => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x42558f4u32)); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b907c; continue; }
  pc = 0x40b9072; continue;
 }
 0x40b9072 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x42558f4u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b9084; continue; }
  pc = 0x40b907a; continue;
 }
 0x40b907a => {
  pc = 0x40b9092; continue;
 }
 0x40b907c => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x42558f4u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x40b9091; continue; }
  pc = 0x40b9084; continue;
 }
 0x40b9084 => {
  { let v = cpu.edx; cpu.imul32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if (cpu.sf != cpu.of) { pc = 0x40b9092; continue; }
  pc = 0x40b9091; continue;
 }
 0x40b9091 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x40b9092; continue;
 }
 0x40b9092 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40b9048; continue; }
  pc = 0x40b9098; continue;
 }
 0x40b9098 => {
  if (cpu.sf != cpu.of) { pc = 0x40b90a7; continue; }
  pc = 0x40b909a; continue;
 }
 0x40b909a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.poll(); pc = 0x40b9053; continue;
 }
 0x40b90a7 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b90b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b90b0); }
 let mut pc: u32 = 0x40b90b0;
 loop { match pc {
 0x40b90b0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4155364u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x428bf80u32, v); }
  { cpu.sub32(cpu.r32(0x4155384u32), cpu.eax); }
  if !cpu.zf { pc = 0x40b90e2; continue; }
  pc = 0x40b90c5; continue;
 }
 0x40b90c5 => {
  { cpu.sub32(cpu.r32(0x4155388u32), cpu.ebx); }
  if !cpu.zf { pc = 0x40b90e2; continue; }
  pc = 0x40b90cd; continue;
 }
 0x40b90cd => {
  { cpu.sub32(cpu.r32(0x4155368u32), cpu.ecx); }
  if !cpu.zf { pc = 0x40b90e2; continue; }
  pc = 0x40b90d5; continue;
 }
 0x40b90d5 => {
  { cpu.sub32(cpu.r32(0x4155394u32), cpu.edi); }
  if !cpu.zf { pc = 0x40b90e2; continue; }
  pc = 0x40b90dd; continue;
 }
 0x40b90dd => {
  pc = 0x40b924e; continue;
 }
 0x40b90e2 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x40b90ef; continue; }
  pc = 0x40b90e7; continue;
 }
 0x40b90e7 => {
  { cpu.sub32(cpu.edi, 0x400u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b90f4; continue; }
  pc = 0x40b90ef; continue;
 }
 0x40b90ef => {
  { let v: u32 = 0x40u32; cpu.edi = v; }
  pc = 0x40b90f4; continue;
 }
 0x40b90f4 => {
  cpu.push(0x40b90f9); f_040b92cf(cpu); if cpu.returned_to != 0x40b90f9 { return; }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x4154564u32); }
  { let v: u32 = cpu.edi; cpu.w32(0x4155394u32, v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4155398u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x4155398u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x415539cu32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x41553a8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x41553acu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4155398u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x41553a8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x41553a0u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x4155384u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4155388u32, v); }
  { let v = (cpu.r32(0x4155384u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x415538cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4155388u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4155390u32, v); cpu.fpop(); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4155368u32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415536cu32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4155370u32, v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4155374u32, v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4155378u32, v); }
  { let v = (cpu.r32(0x4155374u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x415537cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4155378u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4155380u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4155374u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x415455cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x41553c8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x41553c8u32)); cpu.w32(0x41553c8u32, v); }
  { let v = (cpu.r32(0x4155378u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x415455cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x41553ccu32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x41553ccu32)); cpu.w32(0x41553ccu32, v); }
  { let v = (cpu.r32(0x4155374u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x415455cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x41553d0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4155378u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x4155394u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x415455cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x41553d4u32, v); cpu.fpop(); }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.push(0x40b9245); f_040b9279(cpu); if cpu.returned_to != 0x40b9245 { return; }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x4154566u32); }
  pc = 0x40b924e; continue;
 }
 0x40b924e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9250); }
 let mut pc: u32 = 0x40b9250;
 loop { match pc {
 0x40b9250 => {
  { cpu.pushad(); }
  cpu.push(0x40b9256); f_040c1c74(cpu); if cpu.returned_to != 0x40b9256 { return; }
  cpu.push(0x40b925b); f_040c073e(cpu); if cpu.returned_to != 0x40b925b { return; }
  cpu.push(0x40b9260); f_040c1cfa(cpu); if cpu.returned_to != 0x40b9260 { return; }
  cpu.push(0x40b9265); f_040b92cf(cpu); if cpu.returned_to != 0x40b9265 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9267(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9267); }
 let mut pc: u32 = 0x40b9267;
 loop { match pc {
 0x40b9267 => {
  { let v: u32 = cpu.eax; cpu.w32(0x41553d8u32, v); }
  { let v = (cpu.r32(0x41553d8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x41553dcu32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9279(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9279); }
 let mut pc: u32 = 0x40b9279;
 loop { match pc {
 0x40b9279 => {
  { let v: u32 = cpu.eax; cpu.w32(0x41553a4u32, v); }
  { let v = (cpu.r32(0x41553a4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x41553a8u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x41553a8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x41553acu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4155398u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x41553a8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x41553a0u32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b92ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b92ab); }
 let mut pc: u32 = 0x40b92ab;
 loop { match pc {
 0x40b92ab => {
  { let v: u32 = cpu.eax; cpu.w32(0x41553bcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x41553c0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x41553c4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x41553b0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x41553b4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x41553b8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b92cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b92cf); }
 let mut pc: u32 = 0x40b92cf;
 loop { match pc {
 0x40b92cf => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x4154564u32, v); }
  { let v: u16 = cpu.r16(0x4154564u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfcffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xf3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0xc00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xffc0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4154566u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b92ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b92ff); }
 let mut pc: u32 = 0x40b92ff;
 loop { match pc {
 0x40b92ff => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42bfa48u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42bfa4cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x42bfa50u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b931a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b931a); }
 let mut pc: u32 = 0x40b931a;
 loop { match pc {
 0x40b931a => {
  { let v: u32 = cpu.r32(0x428bf7cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9330); }
 let mut pc: u32 = 0x40b9330;
 loop { match pc {
 0x40b9330 => {
  { let v: u32 = 0x4154590u32; cpu.edi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = 0x41545a8u32; cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9347(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9347); }
 let mut pc: u32 = 0x40b9347;
 loop { match pc {
 0x40b9347 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154758u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b941a; continue; }
  pc = 0x40b935c; continue;
 }
 0x40b935c => {
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4154744u32); cpu.ecx = v; }
  { let v: u32 = 0x425b88cu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a78u32, v); }
  pc = 0x40b9379; continue;
 }
 0x40b9379 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v = cpu.r32(0x415476cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x4154768u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x4154764u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.sub32(cpu.r32(0x4255a78u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b9379; continue; }
  pc = 0x40b941a; continue;
 }
 0x40b941a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b941b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b941b); }
 let mut pc: u32 = 0x40b941b;
 loop { match pc {
 0x40b941b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154750u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40b9499; continue; }
  pc = 0x40b942c; continue;
 }
 0x40b942c => {
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4154748u32); cpu.ecx = v; }
  { let v: u32 = 0x425dfa4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40b9446); f_040b9a7b(cpu); if cpu.returned_to != 0x40b9446 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x40b944c; continue;
 }
 0x40b944c => {
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x4154710u32); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x4154714u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x4154718u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x40b9484; continue; }
  pc = 0x40b9482; continue;
 }
 0x40b9482 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x40b9484; continue;
 }
 0x40b9484 => {
  { let v: u32 = cpu.eax.wrapping_add(0x300u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x3fffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b9494; continue; }
  pc = 0x40b948f; continue;
 }
 0x40b948f => {
  { let v: u32 = 0x3fffu32; cpu.eax = v; }
  pc = 0x40b9494; continue;
 }
 0x40b9494 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b944c; continue; }
  pc = 0x40b9499; continue;
 }
 0x40b9499 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b949a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b949a); }
 let mut pc: u32 = 0x40b949a;
 loop { match pc {
 0x40b949a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415475cu32, v); }
  { let v: u32 = cpu.r32(0x415474cu32); cpu.ecx = v; }
  { let v: u32 = 0x425e778u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a78u32, v); }
  pc = 0x40b94bc; continue;
 }
 0x40b94bc => {
  { let v: u32 = 0x4154614u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154578u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415457cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154580u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4154578u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415457cu32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x40b9546); f_040ba6df(cpu); if cpu.returned_to != 0x40b9546 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4154580u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415457cu32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x40b9566); f_040ba6df(cpu); if cpu.returned_to != 0x40b9566 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4255a78u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b94bc; continue; }
  pc = 0x40b9589; continue;
 }
 0x40b9589 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b958a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b958a); }
 let mut pc: u32 = 0x40b958a;
 loop { match pc {
 0x40b958a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b958b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b958b); }
 let mut pc: u32 = 0x40b958b;
 loop { match pc {
 0x40b958b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b960a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b960a); }
 let mut pc: u32 = 0x40b960a;
 loop { match pc {
 0x40b960a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9673(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9673); }
 let mut pc: u32 = 0x40b9673;
 loop { match pc {
 0x40b9673 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b96dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b96dc); }
 let mut pc: u32 = 0x40b96dc;
 loop { match pc {
 0x40b96dc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9749(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9749); }
 let mut pc: u32 = 0x40b9749;
 loop { match pc {
 0x40b9749 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42559fcu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a00u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a04u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a08u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a0cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a10u32, v); }
  { let v: u32 = 0x42559fcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4255a0cu32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40b9837; continue; }
  pc = 0x40b97f9; continue;
 }
 0x40b97f9 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4255a14u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4255a08u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4255a10u32)); cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4155378u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b9837 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b983e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b983e); }
 let mut pc: u32 = 0x40b983e;
 loop { match pc {
 0x40b983e => {
  { cpu.sub32(cpu.r32(0x4154760u32), 0x41545a8u32); }
  if cpu.zf { pc = 0x40b9954; continue; }
  pc = 0x40b984e; continue;
 }
 0x40b984e => {
  { let v: u32 = cpu.r32(0x4154760u32); cpu.eax = v; }
  { let v: u32 = 0x4255a24u32; cpu.ebx = v; }
  cpu.push(0x40b985d); f_040ba8eb(cpu); if cpu.returned_to != 0x40b985d { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154754u32, v); }
  { let v: u32 = cpu.r32(0x4154740u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x4257a08u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x4255ac4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x4255a24u32; cpu.ebx = v; }
  { let v = (cpu.r32(0x4154764u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4255a18u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x4154768u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4255a1cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x415476cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4255a20u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4155394u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154754u32); cpu.ecx = v; }
  pc = 0x40b98b4; continue;
 }
 0x40b98b4 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a18u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a1cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a20u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b9937; continue; }
  pc = 0x40b9913; continue;
 }
 0x40b9913 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x4155398u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x415537cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4155380u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b9937; continue;
 }
 0x40b9937 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b98b4; continue; }
  pc = 0x40b9953; continue;
 }
 0x40b9953 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b9954 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154754u32, v); }
  { let v: u32 = cpu.r32(0x4154740u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x4257a08u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x4255ac4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4154784u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4154590u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a48u32, v); }
  { let v = (cpu.r32(0x4255a48u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4154594u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4255a48u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a4cu32, v); }
  { let v = (cpu.r32(0x4255a4cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4154598u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x4255a4cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a50u32, v); }
  { let v = (cpu.r32(0x4255a50u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x4255a50u32, v); cpu.fpop(); }
  { let v: u32 = 0x41545ccu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4155394u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154754u32); cpu.ecx = v; }
  pc = 0x40b99db; continue;
 }
 0x40b99db => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a50u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a4cu32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4255a48u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40b9a5e; continue; }
  pc = 0x40b9a3a; continue;
 }
 0x40b9a3a => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x4155398u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x415537cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4155380u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x40b9a5e; continue;
 }
 0x40b9a5e => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40b99db; continue; }
  pc = 0x40b9a7a; continue;
 }
 0x40b9a7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9a7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9a7b); }
 let mut pc: u32 = 0x40b9a7b;
 loop { match pc {
 0x40b9a7b => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a78u32, v); }
  pc = 0x40b9a85; continue;
 }
 0x40b9a85 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4255a78u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b9a85; continue; }
  pc = 0x40b9b20; continue;
 }
 0x40b9b20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9b21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9b21); }
 let mut pc: u32 = 0x40b9b21;
 loop { match pc {
 0x40b9b21 => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a78u32, v); }
  pc = 0x40b9b2b; continue;
 }
 0x40b9b2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4255a78u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b9b2b; continue; }
  pc = 0x40b9b9e; continue;
 }
 0x40b9b9e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9d78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9d78); }
 let mut pc: u32 = 0x40b9d78;
 loop { match pc {
 0x40b9d78 => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a78u32, v); }
  pc = 0x40b9d82; continue;
 }
 0x40b9d82 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154768u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x415476cu32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x4255a78u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40b9d82; continue; }
  pc = 0x40b9e0b; continue;
 }
 0x40b9e0b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9ef9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9ef9); }
 let mut pc: u32 = 0x40b9ef9;
 loop { match pc {
 0x40b9ef9 => {
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3fffu32; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040b9f33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40b9f33); }
 let mut pc: u32 = 0x40b9f33;
 loop { match pc {
 0x40b9f33 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a60u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a6cu32, v); }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a64u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a70u32, v); }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a68u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a74u32, v); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba096(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba096); }
 let mut pc: u32 = 0x40ba096;
 loop { match pc {
 0x40ba096 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a60u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a6cu32, v); }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a64u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a70u32, v); }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a68u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a74u32, v); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a74u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a64u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a70u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba1ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba1ff); }
 let mut pc: u32 = 0x40ba1ff;
 loop { match pc {
 0x40ba1ff => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba338); }
 let mut pc: u32 = 0x40ba338;
 loop { match pc {
 0x40ba338 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4255a60u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a6cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a60u32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba414); }
 let mut pc: u32 = 0x40ba414;
 loop { match pc {
 0x40ba414 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4255a64u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a70u32, v); }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a64u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a70u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba4ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba4ef); }
 let mut pc: u32 = 0x40ba4ef;
 loop { match pc {
 0x40ba4ef => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4255a68u32, v); }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a74u32, v); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a68u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4255a74u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba5c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba5c4); }
 let mut pc: u32 = 0x40ba5c4;
 loop { match pc {
 0x40ba5c4 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40ba5ca); f_040ba61d(cpu); if cpu.returned_to != 0x40ba5ca { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x40ba5da); f_040ba61d(cpu); if cpu.returned_to != 0x40ba5da { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba61d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba61d); }
 let mut pc: u32 = 0x40ba61d;
 loop { match pc {
 0x40ba61d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40ba638; continue;
 }
 0x40ba638 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40ba643; continue; }
  pc = 0x40ba63c; continue;
 }
 0x40ba63c => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x40ba64d; continue; }
  pc = 0x40ba643; continue;
 }
 0x40ba643 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x40ba638; continue;
 }
 0x40ba64d => {
  cpu.push(0x40ba652); f_040ba6bc(cpu); if cpu.returned_to != 0x40ba652 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x40ba65f; continue; }
  pc = 0x40ba65a; continue;
 }
 0x40ba65a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x40ba65f; continue;
 }
 0x40ba65f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba685(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba685); }
 let mut pc: u32 = 0x40ba685;
 loop { match pc {
 0x40ba685 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40ba69e; continue;
 }
 0x40ba69e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40ba6a9; continue; }
  pc = 0x40ba6a2; continue;
 }
 0x40ba6a2 => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x40ba6b3; continue; }
  pc = 0x40ba6a9; continue;
 }
 0x40ba6a9 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x40ba69e; continue;
 }
 0x40ba6b3 => {
  cpu.push(0x40ba6b8); f_040ba6bc(cpu); if cpu.returned_to != 0x40ba6b8 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba6bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba6bc); }
 let mut pc: u32 = 0x40ba6bc;
 loop { match pc {
 0x40ba6bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10000000u32; cpu.ebx = v; }
  pc = 0x40ba6c6; continue;
 }
 0x40ba6c6 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40ba6d4; continue; }
  pc = 0x40ba6d0; continue;
 }
 0x40ba6d0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  pc = 0x40ba6d4; continue;
 }
 0x40ba6d4 => {
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40ba6c6; continue; }
  pc = 0x40ba6d9; continue;
 }
 0x40ba6d9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba6df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba6df); }
 let mut pc: u32 = 0x40ba6df;
 loop { match pc {
 0x40ba6df => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40ba6ea; continue; }
  pc = 0x40ba6e8; continue;
 }
 0x40ba6e8 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40ba6ea; continue;
 }
 0x40ba6ea => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40ba6f0; continue; }
  pc = 0x40ba6ee; continue;
 }
 0x40ba6ee => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x40ba6f0; continue;
 }
 0x40ba6f0 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u8 = ((cpu.zf || cpu.sf != cpu.of)) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40ba6f8; continue; }
  pc = 0x40ba6f7; continue;
 }
 0x40ba6f7 => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x40ba6f8; continue;
 }
 0x40ba6f8 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40ba72b; continue; }
  pc = 0x40ba6fc; continue;
 }
 0x40ba6fc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4261f1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x40ba71c; continue; }
  pc = 0x40ba715; continue;
 }
 0x40ba715 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x400u32); cpu.eax = v; }
  pc = 0x40ba71c; continue;
 }
 0x40ba71c => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.ebp); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x40ba722; continue; }
  pc = 0x40ba720; continue;
 }
 0x40ba720 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40ba722; continue;
 }
 0x40ba722 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x40ba72b; continue; }
  pc = 0x40ba726; continue;
 }
 0x40ba726 => {
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  pc = 0x40ba72b; continue;
 }
 0x40ba72b => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba732(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba732); }
 let mut pc: u32 = 0x40ba732;
 loop { match pc {
 0x40ba732 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a7cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a80u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a84u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a88u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a8cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a90u32, v); }
  { let v: u32 = cpu.r32(0x4255a7cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a84u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40ba78e); f_040ba6bc(cpu); if cpu.returned_to != 0x40ba78e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a7cu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a7cu32, v); }
  { let v: u32 = cpu.r32(0x4255a80u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a80u32, v); }
  { let v: u32 = cpu.r32(0x4255a84u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a84u32, v); }
  { let v: u32 = cpu.r32(0x4255a88u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a8cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a90u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40ba7fc); f_040ba6bc(cpu); if cpu.returned_to != 0x40ba7fc { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4255a88u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a88u32, v); }
  { let v: u32 = cpu.r32(0x4255a8cu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a8cu32, v); }
  { let v: u32 = cpu.r32(0x4255a90u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a90u32, v); }
  { let v: u32 = cpu.r32(0x4255a84u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a8cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a80u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a90u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a94u32, v); }
  { let v: u32 = cpu.r32(0x4255a7cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a90u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a84u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a88u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a98u32, v); }
  { let v: u32 = cpu.r32(0x4255a80u32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a88u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4255a7cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4255a8cu32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4255a9cu32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4255a7cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a88u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a94u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a80u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a8cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a98u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a84u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a90u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4255a9cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba8eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba8eb); }
 let mut pc: u32 = 0x40ba8eb;
 loop { match pc {
 0x40ba8eb => {
  { let v = cpu.rf32(0x415458cu32); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x18u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x14u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba93a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba93a); }
 let mut pc: u32 = 0x40ba93a;
 loop { match pc {
 0x40ba93a => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba948); }
 let mut pc: u32 = 0x40ba948;
 loop { match pc {
 0x40ba948 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x425f71cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba985(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba985); }
 let mut pc: u32 = 0x40ba985;
 loop { match pc {
 0x40ba985 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba993(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba993); }
 let mut pc: u32 = 0x40ba993;
 loop { match pc {
 0x40ba993 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x425ff1cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040ba9d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40ba9d0); }
 let mut pc: u32 = 0x40ba9d0;
 loop { match pc {
 0x40ba9d0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154764u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154768u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415476cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4154784u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2eu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x40baa00; continue;
 }
 0x40baa00 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40baa1d; continue; }
  pc = 0x40baa07; continue;
 }
 0x40baa07 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x415478cu32.wrapping_add(cpu.ebx)); cpu.push(0x40baa16); cpu.call(t); if cpu.returned_to != 0x40baa16 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40baa00; continue;
 }
 0x40baa1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040baa1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40baa1f); }
 let mut pc: u32 = 0x40baa1f;
 loop { match pc {
 0x40baa1f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040baa2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40baa2d); }
 let mut pc: u32 = 0x40baa2d;
 loop { match pc {
 0x40baa2d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040baa60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40baa60); }
 let mut pc: u32 = 0x40baa60;
 loop { match pc {
 0x40baa60 => {
  cpu.push(0x40baa65); f_040c07f7(cpu); if cpu.returned_to != 0x40baa65 { return; }
  { let v: u32 = 0x42a47b4u32; cpu.w32(0x42a47b0u32, v); }
  { let v: u32 = 0x42b40f0u32; cpu.w32(0x42b40ecu32, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x4154566u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa48u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa4cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x41553f0u32)); cpu.w32(0x41553f0u32, v); }
  { let v: u32 = 0x41545a8u32; cpu.eax = v; }
  { let v: u32 = 0x41545ccu32; cpu.ebx = v; }
  cpu.push(0x40baab5); f_040ba8eb(cpu); if cpu.returned_to != 0x40baab5 { return; }
  { let v: u32 = cpu.r32(0x41545acu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41545b8u32); cpu.ebx = v; }
  cpu.push(0x40baac5); f_040ba6df(cpu); if cpu.returned_to != 0x40baac5 { return; }
  { let v: u32 = 0x41545a8u32; cpu.ebx = v; }
  { let v: u32 = 0x415465cu32; cpu.edi = v; }
  cpu.push(0x40baad4); f_040ba4ef(cpu); if cpu.returned_to != 0x40baad4 { return; }
  { let v: u32 = cpu.r32(0x41545c0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x41545c8u32); cpu.ebx = v; }
  cpu.push(0x40baae4); f_040ba6df(cpu); if cpu.returned_to != 0x40baae4 { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x41545a8u32; cpu.ebx = v; }
  { let v: u32 = 0x41546a4u32; cpu.edi = v; }
  cpu.push(0x40baaf5); f_040ba414(cpu); if cpu.returned_to != 0x40baaf5 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040baaf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40baaf6); }
 let mut pc: u32 = 0x40baaf6;
 loop { match pc {
 0x40baaf6 => {
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x4154564u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bab00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bab00); }
 let mut pc: u32 = 0x40bab00;
 loop { match pc {
 0x40bab00 => {
  { let v: u32 = 0x426277cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154730u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4154734u32, v); }
  { let v: u32 = 0x42646bcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x427217cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154738u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x415473cu32, v); }
  { let v: u32 = 0x42740bcu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bab2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bab2b); }
 let mut pc: u32 = 0x40bab2b;
 loop { match pc {
 0x40bab2b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4154730u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154734u32); cpu.ebx = v; }
  cpu.push(0x40bab3c); f_040bafdc(cpu); if cpu.returned_to != 0x40bab3c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bab3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bab3e); }
 let mut pc: u32 = 0x40bab3e;
 loop { match pc {
 0x40bab3e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4154738u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  cpu.push(0x40bab4f); f_040baf10(cpu); if cpu.returned_to != 0x40bab4f { return; }
  { let v: u32 = cpu.r32(0x4154738u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  cpu.push(0x40bab5f); f_040bafdc(cpu); if cpu.returned_to != 0x40bab5f { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bab61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bab61); }
 let mut pc: u32 = 0x40bab61;
 loop { match pc {
 0x40bab61 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bab89; continue; }
  pc = 0x40bab65; continue;
 }
 0x40bab65 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4262744u32, v); }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4262748u32, v); }
  pc = 0x40bab74; continue;
 }
 0x40bab74 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40bab7a); f_040bab8a(cpu); if cpu.returned_to != 0x40bab7a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x4262744u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x4262748u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x40bab74; continue; }
  pc = 0x40bab89; continue;
 }
 0x40bab89 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bab8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bab8a); }
 let mut pc: u32 = 0x40bab8a;
 loop { match pc {
 0x40bab8a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4154780u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4154590u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x4154594u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x4154598u32)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553bcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40babc7; continue;
 }
 0x40babc7 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553c0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40babd6; continue;
 }
 0x40babd6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553c4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40babe5; continue;
 }
 0x40babe5 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553b0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40babf6; continue;
 }
 0x40babf6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bac05; continue;
 }
 0x40bac05 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x41553b8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bac14; continue;
 }
 0x40bac14 => {
  { let v: u32 = cpu.eax; cpu.w32(0x426274cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4262750u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4262754u32, v); }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.edx)), 0x1u16); }
  if cpu.zf { pc = 0x40bac39; continue; }
  pc = 0x40bac33; continue;
 }
 0x40bac33 => {
  { let v: u32 = cpu.r32(0x4154734u32); cpu.ebx = v; }
  pc = 0x40bac39; continue;
 }
 0x40bac39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x41545acu32); cpu.eax = v; }
  { let v = cpu.r32(0x426274cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545b8u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262750u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545c4u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262754u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x41553a4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bac86; continue;
 }
 0x40bac86 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x4155394u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bac97; continue;
 }
 0x40bac97 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x41545a8u32); cpu.eax = v; }
  { let v = cpu.r32(0x426274cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545b4u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262750u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545c0u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262754u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x41553d0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bace5; continue; }
  pc = 0x40bace3; continue;
 }
 0x40bace3 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40bace5; continue;
 }
 0x40bace5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x41553c8u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bacfb; continue;
 }
 0x40bacfb => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x40bad3d; continue; }
  pc = 0x40bad03; continue;
 }
 0x40bad03 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x41545b0u32); cpu.eax = v; }
  { let v = cpu.r32(0x426274cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545bcu32); cpu.eax = v; }
  { let v = cpu.r32(0x4262750u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545c8u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262754u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x40badab; continue;
 }
 0x40bad3d => {
  { let v: u32 = cpu.r32(0x41545b0u32); cpu.eax = v; }
  { let v = cpu.r32(0x426274cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545bcu32); cpu.eax = v; }
  { let v = cpu.r32(0x4262750u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x41545c8u32); cpu.eax = v; }
  { let v = cpu.r32(0x4262754u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x41553d4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bad89; continue; }
  pc = 0x40bad87; continue;
 }
 0x40bad87 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40bad89; continue;
 }
 0x40bad89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x41553ccu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bad9f; continue;
 }
 0x40bad9f => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x40badab; continue; }
  pc = 0x40bada7; continue;
 }
 0x40bada7 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x40badab; continue;
 }
 0x40badab => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x41553a4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40badc5; continue; }
  pc = 0x40badb8; continue;
 }
 0x40badb8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40badc9; continue; }
  pc = 0x40badc5; continue;
 }
 0x40badc5 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x40badc9; continue;
 }
 0x40badc9 => {
  { let v: u32 = cpu.r32(0x4154780u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebx)), 0x1u16); }
  if cpu.zf { pc = 0x40bae20; continue; }
  pc = 0x40bae05; continue;
 }
 0x40bae05 => {
  { let v: u32 = cpu.r32(0x4154734u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x42646bcu32); }
  if (cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bae16; continue;
 }
 0x40bae16 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4154734u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40bae39; continue;
 }
 0x40bae20 => {
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x42740bcu32); }
  if (cpu.sf == cpu.of) { pc = 0x40bae40; continue; }
  pc = 0x40bae31; continue;
 }
 0x40bae31 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x415473cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40bae39; continue;
 }
 0x40bae39 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40bae40 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bae44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bae44); } r_040bae44(cpu, 0x40bae44); }
pub fn f_040bae4d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bae4d); } r_040bae44(cpu, 0x40bae4d); }
fn r_040bae44(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40bae44 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40baebe; continue; }
  pc = 0x40bae4d; continue;
 }
 0x40bae4d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40baebf; continue; }
  pc = 0x40bae56; continue;
 }
 0x40bae56 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x40bae65; continue;
 }
 0x40bae65 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40bae7d; continue; }
  pc = 0x40bae6c; continue;
 }
 0x40bae6c => {
  if cpu.zf { pc = 0x40bae73; continue; }
  pc = 0x40bae6e; continue;
 }
 0x40bae6e => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x40bae65; continue;
 }
 0x40bae73 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x40bae7d; continue; }
  pc = 0x40bae78; continue;
 }
 0x40bae78 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x40bae65; continue;
 }
 0x40bae7d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x40bae95; continue; }
  pc = 0x40bae84; continue;
 }
 0x40bae84 => {
  if cpu.zf { pc = 0x40bae8b; continue; }
  pc = 0x40bae86; continue;
 }
 0x40bae86 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40bae7d; continue;
 }
 0x40bae8b => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if (cpu.cf || cpu.zf) { pc = 0x40bae95; continue; }
  pc = 0x40bae90; continue;
 }
 0x40bae90 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40bae7d; continue;
 }
 0x40bae95 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40baea3; continue; }
  pc = 0x40bae99; continue;
 }
 0x40bae99 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x40baea3; continue;
 }
 0x40baea3 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bae65; continue; }
  pc = 0x40baea7; continue;
 }
 0x40baea7 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40baeb4; continue; }
  pc = 0x40baeab; continue;
 }
 0x40baeab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x40baeb3); f_040bae4d(cpu); if cpu.returned_to != 0x40baeb3 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x40baeb4; continue;
 }
 0x40baeb4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40baebe; continue; }
  pc = 0x40baeb9; continue;
 }
 0x40baeb9 => {
  cpu.push(0x40baebe); f_040bae4d(cpu); if cpu.returned_to != 0x40baebe { return; }
  pc = 0x40baebe; continue;
 }
 0x40baebe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40baebf => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x40baec1; continue;
 }
 0x40baec1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40baee6; continue; }
  pc = 0x40baed3; continue;
 }
 0x40baed3 => {
  if cpu.zf { pc = 0x40baedb; continue; }
  pc = 0x40baed5; continue;
 }
 0x40baed5 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baec1; continue; }
  pc = 0x40baed9; continue;
 }
 0x40baed9 => {
  pc = 0x40baf0f; continue;
 }
 0x40baedb => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x40baee6; continue; }
  pc = 0x40baee0; continue;
 }
 0x40baee0 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baec1; continue; }
  pc = 0x40baee4; continue;
 }
 0x40baee4 => {
  pc = 0x40baf0f; continue;
 }
 0x40baee6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x40baeef; continue;
 }
 0x40baeef => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x40baf07; continue; }
  pc = 0x40baef6; continue;
 }
 0x40baef6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40baeef; continue; }
  pc = 0x40baf00; continue;
 }
 0x40baf00 => {
  if !cpu.zf { pc = 0x40baf07; continue; }
  pc = 0x40baf02; continue;
 }
 0x40baf02 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x40baeef; continue; }
  pc = 0x40baf07; continue;
 }
 0x40baf07 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baec1; continue; }
  pc = 0x40baf0f; continue;
 }
 0x40baf0f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040baf10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40baf10); } r_040baf10(cpu, 0x40baf10); }
pub fn f_040baf19(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40baf19); } r_040baf10(cpu, 0x40baf19); }
fn r_040baf10(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40baf10 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40baf8a; continue; }
  pc = 0x40baf19; continue;
 }
 0x40baf19 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40baf8b; continue; }
  pc = 0x40baf22; continue;
 }
 0x40baf22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x40baf31; continue;
 }
 0x40baf31 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40baf49; continue; }
  pc = 0x40baf38; continue;
 }
 0x40baf38 => {
  if cpu.zf { pc = 0x40baf3f; continue; }
  pc = 0x40baf3a; continue;
 }
 0x40baf3a => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x40baf31; continue;
 }
 0x40baf3f => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if !cpu.cf { pc = 0x40baf49; continue; }
  pc = 0x40baf44; continue;
 }
 0x40baf44 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x40baf31; continue;
 }
 0x40baf49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40baf61; continue; }
  pc = 0x40baf50; continue;
 }
 0x40baf50 => {
  if cpu.zf { pc = 0x40baf57; continue; }
  pc = 0x40baf52; continue;
 }
 0x40baf52 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40baf49; continue;
 }
 0x40baf57 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if !cpu.cf { pc = 0x40baf61; continue; }
  pc = 0x40baf5c; continue;
 }
 0x40baf5c => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40baf49; continue;
 }
 0x40baf61 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40baf6f; continue; }
  pc = 0x40baf65; continue;
 }
 0x40baf65 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x40baf6f; continue;
 }
 0x40baf6f => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40baf31; continue; }
  pc = 0x40baf73; continue;
 }
 0x40baf73 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40baf80; continue; }
  pc = 0x40baf77; continue;
 }
 0x40baf77 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x40baf7f); f_040baf19(cpu); if cpu.returned_to != 0x40baf7f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x40baf80; continue;
 }
 0x40baf80 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40baf8a; continue; }
  pc = 0x40baf85; continue;
 }
 0x40baf85 => {
  cpu.push(0x40baf8a); f_040baf19(cpu); if cpu.returned_to != 0x40baf8a { return; }
  pc = 0x40baf8a; continue;
 }
 0x40baf8a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40baf8b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x40baf8d; continue;
 }
 0x40baf8d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40bafb2; continue; }
  pc = 0x40baf9f; continue;
 }
 0x40baf9f => {
  if cpu.zf { pc = 0x40bafa7; continue; }
  pc = 0x40bafa1; continue;
 }
 0x40bafa1 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baf8d; continue; }
  pc = 0x40bafa5; continue;
 }
 0x40bafa5 => {
  pc = 0x40bafdb; continue;
 }
 0x40bafa7 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { pc = 0x40bafb2; continue; }
  pc = 0x40bafac; continue;
 }
 0x40bafac => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baf8d; continue; }
  pc = 0x40bafb0; continue;
 }
 0x40bafb0 => {
  pc = 0x40bafdb; continue;
 }
 0x40bafb2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x40bafbb; continue;
 }
 0x40bafbb => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x40bafd3; continue; }
  pc = 0x40bafc2; continue;
 }
 0x40bafc2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bafbb; continue; }
  pc = 0x40bafcc; continue;
 }
 0x40bafcc => {
  if !cpu.zf { pc = 0x40bafd3; continue; }
  pc = 0x40bafce; continue;
 }
 0x40bafce => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x40bafbb; continue; }
  pc = 0x40bafd3; continue;
 }
 0x40bafd3 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x40baf8d; continue; }
  pc = 0x40bafdb; continue;
 }
 0x40bafdb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bafdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bafdc); }
 let mut pc: u32 = 0x40bafdc;
 loop { match pc {
 0x40bafdc => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x40bb0a6; continue; }
  pc = 0x40bafe4; continue;
 }
 0x40bafe4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x4154770u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154764u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4154768u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x415476cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154774u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4154778u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4154784u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4157974u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40bb099; continue; }
  pc = 0x40bb031; continue;
 }
 0x40bb031 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4154780u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4154770u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x415482cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415472cu32, v); }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x4296798u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x5dc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb070; continue; }
  pc = 0x40bb05e; continue;
 }
 0x40bb05e => {
  { let v: u32 = 0x429a79cu32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x13880u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb070; continue; }
  pc = 0x40bb06b; continue;
 }
 0x40bb06b => {
  { let v: u32 = 0x429e7a0u32; cpu.eax = v; }
  pc = 0x40bb070; continue;
 }
 0x40bb070 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4296794u32, v); }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x32u32); cpu.esi = v; }
  pc = 0x40bb07e; continue;
 }
 0x40bb07e => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40bb099; continue; }
  pc = 0x40bb088; continue;
 }
 0x40bb088 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x415478cu32.wrapping_add(cpu.ebx)); cpu.push(0x40bb093); cpu.call(t); if cpu.returned_to != 0x40bb093 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40bb07e; continue;
 }
 0x40bb099 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x40bafe4; continue; }
  pc = 0x40bb0a6; continue;
 }
 0x40bb0a6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb0bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb0bb); }
 let mut pc: u32 = 0x40bb0bb;
 loop { match pc {
 0x40bb0bb => {
  { let v: u32 = cpu.add32(cpu.r32(0x415472cu32), 0xb0u32); cpu.w32(0x415472cu32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x4154788u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb0d0); }
 let mut pc: u32 = 0x40bb0d0;
 loop { match pc {
 0x40bb0d0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x428bf84u32)); cpu.w32(0x428bf84u32, v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  f_040bb0d8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb0d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb0d8); }
 let mut pc: u32 = 0x40bb0d8;
 loop { match pc {
 0x40bb0d8 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40bb0e9; continue; }
  pc = 0x40bb0e2; continue;
 }
 0x40bb0e2 => {
  cpu.push(0x40bb0e7); f_040bb133(cpu); if cpu.returned_to != 0x40bb0e7 { return; }
  pc = 0x40bb0ee; continue;
 }
 0x40bb0e9 => {
  cpu.push(0x40bb0ee); f_040bb0ef(cpu); if cpu.returned_to != 0x40bb0ee { return; }
  pc = 0x40bb0ee; continue;
 }
 0x40bb0ee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb0ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb0ef); }
 let mut pc: u32 = 0x40bb0ef;
 loop { match pc {
 0x40bb0ef => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40bb10f; continue; }
  pc = 0x40bb0fc; continue;
 }
 0x40bb0fc => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154744u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x425b888u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40bb11c; continue;
 }
 0x40bb10f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154744u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x425b888u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x40bb11c; continue;
 }
 0x40bb11c => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0xfffffffau32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x40bb126; continue; }
  pc = 0x40bb124; continue;
 }
 0x40bb124 => {
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x40bb126; continue;
 }
 0x40bb126 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40bb12c); f_040bb0d8(cpu); if cpu.returned_to != 0x40bb12c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x40bb132); f_040bb0d8(cpu); if cpu.returned_to != 0x40bb132 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb133(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb133); }
 let mut pc: u32 = 0x40bb133;
 loop { match pc {
 0x40bb133 => {
  { let v: u32 = cpu.r32(0x42bfa48u32).wrapping_add(cpu.eax); cpu.w32(0x42bfa48u32, v); }
  pc = 0x40bb139; continue;
 }
 0x40bb139 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bb157; continue; }
  pc = 0x40bb142; continue;
 }
 0x40bb142 => {
  if cpu.zf { pc = 0x40bb169; continue; }
  pc = 0x40bb144; continue;
 }
 0x40bb144 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154744u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x425b888u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40bb164; continue;
 }
 0x40bb157 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154744u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x425b888u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x40bb164; continue;
 }
 0x40bb164 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x40bb181; continue; }
  pc = 0x40bb169; continue;
 }
 0x40bb169 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa4cu32)); cpu.w32(0x42bfa4cu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x415472cu32)); cpu.ebx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.push(0x40bb180); cpu.call(t); if cpu.returned_to != 0x40bb180 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x40bb181; continue;
 }
 0x40bb181 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bb139; continue; }
  pc = 0x40bb18d; continue;
 }
 0x40bb18d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb18e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb18e); }
 let mut pc: u32 = 0x40bb18e;
 loop { match pc {
 0x40bb18e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { let v: u32 = 0x415472cu32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.esp = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4154734u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154730u32, v); }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4154738u32, v); }
  { let v: u32 = cpu.r32(0x4154754u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154740u32).wrapping_add(cpu.eax); cpu.w32(0x4154740u32, v); }
  { let v: u32 = cpu.r32(0x4154758u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154744u32).wrapping_add(cpu.eax); cpu.w32(0x4154744u32, v); }
  { let v: u32 = cpu.r32(0x4154750u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154748u32).wrapping_add(cpu.eax); cpu.w32(0x4154748u32, v); }
  pc = 0x40bb1d4; continue;
 }
 0x40bb1d4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40bb1da); f_040bab8a(cpu); if cpu.returned_to != 0x40bb1da { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x40bb1d4; continue; }
  pc = 0x40bb1e3; continue;
 }
 0x40bb1e3 => {
  { let v: u32 = cpu.r32(0x4154730u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4154734u32); cpu.ebx = v; }
  cpu.push(0x40bb1f3); f_040bafdc(cpu); if cpu.returned_to != 0x40bb1f3 { return; }
  { let v: u32 = cpu.r32(0x4154738u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  cpu.push(0x40bb203); f_040baf10(cpu); if cpu.returned_to != 0x40bb203 { return; }
  { let v: u32 = cpu.r32(0x4154738u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x415473cu32); cpu.ebx = v; }
  cpu.push(0x40bb213); f_040bafdc(cpu); if cpu.returned_to != 0x40bb213 { return; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x415472cu32; cpu.edi = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb225(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb225); }
 let mut pc: u32 = 0x40bb225;
 loop { match pc {
 0x40bb225 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb226(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb226); }
 let mut pc: u32 = 0x40bb226;
 loop { match pc {
 0x40bb226 => {
  cpu.push(0x40bb22b); f_040bb7df(cpu); if cpu.returned_to != 0x40bb22b { return; }
  if !cpu.zf { pc = 0x40bb7d6; continue; }
  pc = 0x40bb231; continue;
 }
 0x40bb231 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40bb7d6 => {
  { let v: u32 = 0x4154998u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bb7df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bb7df); }
 let mut pc: u32 = 0x40bb7df;
 loop { match pc {
 0x40bb7df => {
  { let v: u32 = 0x80000001u32; cpu.eax = v; }
  { let v: u32 = 0x7fffffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4281ba0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4281ba8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4281bb4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  { let v: u32 = cpu.r32(0x4154780u32); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bb834); f_040b960a(cpu); if cpu.returned_to != 0x40bb834 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb854; continue; }
  pc = 0x40bb84e; continue;
 }
 0x40bb84e => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bb854; continue;
 }
 0x40bb854 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb862; continue; }
  pc = 0x40bb85c; continue;
 }
 0x40bb85c => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bb862; continue;
 }
 0x40bb862 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb870; continue; }
  pc = 0x40bb86a; continue;
 }
 0x40bb86a => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bb870; continue;
 }
 0x40bb870 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb88b; continue; }
  pc = 0x40bb886; continue;
 }
 0x40bb886 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bb88b; continue;
 }
 0x40bb88b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb898; continue; }
  pc = 0x40bb893; continue;
 }
 0x40bb893 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bb898; continue;
 }
 0x40bb898 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb8b7; continue; }
  pc = 0x40bb8b2; continue;
 }
 0x40bb8b2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bb8b7; continue;
 }
 0x40bb8b7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb8c4; continue; }
  pc = 0x40bb8bf; continue;
 }
 0x40bb8bf => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bb8c4; continue;
 }
 0x40bb8c4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bb8e5); f_040b960a(cpu); if cpu.returned_to != 0x40bb8e5 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb905; continue; }
  pc = 0x40bb8ff; continue;
 }
 0x40bb8ff => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bb905; continue;
 }
 0x40bb905 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb913; continue; }
  pc = 0x40bb90d; continue;
 }
 0x40bb90d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bb913; continue;
 }
 0x40bb913 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb921; continue; }
  pc = 0x40bb91b; continue;
 }
 0x40bb91b => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bb921; continue;
 }
 0x40bb921 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb93c; continue; }
  pc = 0x40bb937; continue;
 }
 0x40bb937 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bb93c; continue;
 }
 0x40bb93c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb949; continue; }
  pc = 0x40bb944; continue;
 }
 0x40bb944 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bb949; continue;
 }
 0x40bb949 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb968; continue; }
  pc = 0x40bb963; continue;
 }
 0x40bb963 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bb968; continue;
 }
 0x40bb968 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb975; continue; }
  pc = 0x40bb970; continue;
 }
 0x40bb970 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bb975; continue;
 }
 0x40bb975 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bb996); f_040b960a(cpu); if cpu.returned_to != 0x40bb996 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb9b6; continue; }
  pc = 0x40bb9b0; continue;
 }
 0x40bb9b0 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bb9b6; continue;
 }
 0x40bb9b6 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb9c4; continue; }
  pc = 0x40bb9be; continue;
 }
 0x40bb9be => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bb9c4; continue;
 }
 0x40bb9c4 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb9d2; continue; }
  pc = 0x40bb9cc; continue;
 }
 0x40bb9cc => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bb9d2; continue;
 }
 0x40bb9d2 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bb9ed; continue; }
  pc = 0x40bb9e8; continue;
 }
 0x40bb9e8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bb9ed; continue;
 }
 0x40bb9ed => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bb9fa; continue; }
  pc = 0x40bb9f5; continue;
 }
 0x40bb9f5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bb9fa; continue;
 }
 0x40bb9fa => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bba19; continue; }
  pc = 0x40bba14; continue;
 }
 0x40bba14 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bba19; continue;
 }
 0x40bba19 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bba26; continue; }
  pc = 0x40bba21; continue;
 }
 0x40bba21 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bba26; continue;
 }
 0x40bba26 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bba47); f_040b960a(cpu); if cpu.returned_to != 0x40bba47 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bba67; continue; }
  pc = 0x40bba61; continue;
 }
 0x40bba61 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bba67; continue;
 }
 0x40bba67 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bba75; continue; }
  pc = 0x40bba6f; continue;
 }
 0x40bba6f => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bba75; continue;
 }
 0x40bba75 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bba83; continue; }
  pc = 0x40bba7d; continue;
 }
 0x40bba7d => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bba83; continue;
 }
 0x40bba83 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bba9e; continue; }
  pc = 0x40bba99; continue;
 }
 0x40bba99 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bba9e; continue;
 }
 0x40bba9e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbaab; continue; }
  pc = 0x40bbaa6; continue;
 }
 0x40bbaa6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bbaab; continue;
 }
 0x40bbaab => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbaca; continue; }
  pc = 0x40bbac5; continue;
 }
 0x40bbac5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bbaca; continue;
 }
 0x40bbaca => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbad7; continue; }
  pc = 0x40bbad2; continue;
 }
 0x40bbad2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bbad7; continue;
 }
 0x40bbad7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bbaf8); f_040b960a(cpu); if cpu.returned_to != 0x40bbaf8 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbb18; continue; }
  pc = 0x40bbb12; continue;
 }
 0x40bbb12 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bbb18; continue;
 }
 0x40bbb18 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbb26; continue; }
  pc = 0x40bbb20; continue;
 }
 0x40bbb20 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bbb26; continue;
 }
 0x40bbb26 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbb34; continue; }
  pc = 0x40bbb2e; continue;
 }
 0x40bbb2e => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bbb34; continue;
 }
 0x40bbb34 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbb4f; continue; }
  pc = 0x40bbb4a; continue;
 }
 0x40bbb4a => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bbb4f; continue;
 }
 0x40bbb4f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbb5c; continue; }
  pc = 0x40bbb57; continue;
 }
 0x40bbb57 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bbb5c; continue;
 }
 0x40bbb5c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbb7b; continue; }
  pc = 0x40bbb76; continue;
 }
 0x40bbb76 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bbb7b; continue;
 }
 0x40bbb7b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbb88; continue; }
  pc = 0x40bbb83; continue;
 }
 0x40bbb83 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bbb88; continue;
 }
 0x40bbb88 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bbba9); f_040b960a(cpu); if cpu.returned_to != 0x40bbba9 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbbc9; continue; }
  pc = 0x40bbbc3; continue;
 }
 0x40bbbc3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bbbc9; continue;
 }
 0x40bbbc9 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbbd7; continue; }
  pc = 0x40bbbd1; continue;
 }
 0x40bbbd1 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bbbd7; continue;
 }
 0x40bbbd7 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbbe5; continue; }
  pc = 0x40bbbdf; continue;
 }
 0x40bbbdf => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bbbe5; continue;
 }
 0x40bbbe5 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbc00; continue; }
  pc = 0x40bbbfb; continue;
 }
 0x40bbbfb => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bbc00; continue;
 }
 0x40bbc00 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbc0d; continue; }
  pc = 0x40bbc08; continue;
 }
 0x40bbc08 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bbc0d; continue;
 }
 0x40bbc0d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbc2c; continue; }
  pc = 0x40bbc27; continue;
 }
 0x40bbc27 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bbc2c; continue;
 }
 0x40bbc2c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbc39; continue; }
  pc = 0x40bbc34; continue;
 }
 0x40bbc34 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bbc39; continue;
 }
 0x40bbc39 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bbc5a); f_040b960a(cpu); if cpu.returned_to != 0x40bbc5a { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbc7a; continue; }
  pc = 0x40bbc74; continue;
 }
 0x40bbc74 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bbc7a; continue;
 }
 0x40bbc7a => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbc88; continue; }
  pc = 0x40bbc82; continue;
 }
 0x40bbc82 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bbc88; continue;
 }
 0x40bbc88 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbc96; continue; }
  pc = 0x40bbc90; continue;
 }
 0x40bbc90 => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bbc96; continue;
 }
 0x40bbc96 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbcb1; continue; }
  pc = 0x40bbcac; continue;
 }
 0x40bbcac => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bbcb1; continue;
 }
 0x40bbcb1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbcbe; continue; }
  pc = 0x40bbcb9; continue;
 }
 0x40bbcb9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bbcbe; continue;
 }
 0x40bbcbe => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbcdd; continue; }
  pc = 0x40bbcd8; continue;
 }
 0x40bbcd8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bbcdd; continue;
 }
 0x40bbcdd => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbcea; continue; }
  pc = 0x40bbce5; continue;
 }
 0x40bbce5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bbcea; continue;
 }
 0x40bbcea => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4154760u32); cpu.ebx = v; }
  cpu.push(0x40bbd0b); f_040b960a(cpu); if cpu.returned_to != 0x40bbd0b { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4154764u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4154768u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x415476cu32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbd2b; continue; }
  pc = 0x40bbd25; continue;
 }
 0x40bbd25 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb0u32, v); }
  pc = 0x40bbd2b; continue;
 }
 0x40bbd2b => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4281bb4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbd39; continue; }
  pc = 0x40bbd33; continue;
 }
 0x40bbd33 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4281bb4u32, v); }
  pc = 0x40bbd39; continue;
 }
 0x40bbd39 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155394u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbd47; continue; }
  pc = 0x40bbd41; continue;
 }
 0x40bbd41 => {
  { let v: u32 = cpu.r32(0x4155394u32); cpu.ebx = v; }
  pc = 0x40bbd47; continue;
 }
 0x40bbd47 => {
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155374u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbd62; continue; }
  pc = 0x40bbd5d; continue;
 }
 0x40bbd5d => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba0u32, v); }
  pc = 0x40bbd62; continue;
 }
 0x40bbd62 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbd6f; continue; }
  pc = 0x40bbd6a; continue;
 }
 0x40bbd6a => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba4u32, v); }
  pc = 0x40bbd6f; continue;
 }
 0x40bbd6f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x4155394u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4155378u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4281ba8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbd8e; continue; }
  pc = 0x40bbd89; continue;
 }
 0x40bbd89 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281ba8u32, v); }
  pc = 0x40bbd8e; continue;
 }
 0x40bbd8e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4281bacu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbd9b; continue; }
  pc = 0x40bbd96; continue;
 }
 0x40bbd96 => {
  { let v: u32 = cpu.eax; cpu.w32(0x4281bacu32, v); }
  pc = 0x40bbd9b; continue;
 }
 0x40bbd9b => {
  { let v: u32 = cpu.r32(0x4281bb4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4155394u32)); }
  if (cpu.sf != cpu.of) { pc = 0x40bbddc; continue; }
  pc = 0x40bbda8; continue;
 }
 0x40bbda8 => {
  { let v: u32 = cpu.r32(0x4281ba0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x415536cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bbddc; continue; }
  pc = 0x40bbdb5; continue;
 }
 0x40bbdb5 => {
  { let v: u32 = cpu.r32(0x4281ba8u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4155370u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bbddc; continue; }
  pc = 0x40bbdc3; continue;
 }
 0x40bbdc3 => {
  { let v: u32 = cpu.r32(0x4281ba4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40bbddc; continue; }
  pc = 0x40bbdcc; continue;
 }
 0x40bbdcc => {
  { let v: u32 = cpu.r32(0x4281bacu32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x40bbddc; continue; }
  pc = 0x40bbdd6; continue;
 }
 0x40bbdd6 => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40bbddc => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bbdf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bbdf0); }
 let mut pc: u32 = 0x40bbdf0;
 loop { match pc {
 0x40bbdf0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bbef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bbef0); }
 let mut pc: u32 = 0x40bbef0;
 loop { match pc {
 0x40bbef0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x40bbef9; continue;
 }
 0x40bbef9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bbef9; continue; }
  pc = 0x40bbf07; continue;
 }
 0x40bbf07 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x42b5e40u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bbf1b; continue;
 }
 0x40bbf1b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x42bb248u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bbf3c; continue; }
  pc = 0x40bbf2f; continue;
 }
 0x40bbf2f => {
  if cpu.zf { pc = 0x40bc098; continue; }
  pc = 0x40bbf35; continue;
 }
 0x40bbf35 => {
  { let v: u32 = 0x42b5e48u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x40bbf3c; continue;
 }
 0x40bbf3c => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bb8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bbcu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bbcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bb8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x42b5e44u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bbf74; continue; }
  pc = 0x40bbf6e; continue;
 }
 0x40bbf6e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bbf74; continue;
 }
 0x40bbf74 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x42b5e40u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bbf82; continue; }
  pc = 0x40bbf7c; continue;
 }
 0x40bbf7c => {
  { let v: u32 = cpu.edx; cpu.w32(0x42b5e40u32, v); }
  pc = 0x40bbf82; continue;
 }
 0x40bbf82 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc098; continue; }
  pc = 0x40bbf8a; continue;
 }
 0x40bbf8a => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4154a48u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42b5ad8u32, v); }
  { let v = (cpu.r32(0x4281bb8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x42b5ad8u32), 0x42bb248u32); }
  if (cpu.sf != cpu.of) { pc = 0x40bbfce; continue; }
  pc = 0x40bbfcc; continue;
 }
 0x40bbfcc => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40bbfce; continue;
 }
 0x40bbfce => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4281bc0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc07a; continue; }
  pc = 0x40bc019; continue;
 }
 0x40bc019 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc019; continue; }
  pc = 0x40bc07a; continue;
 }
 0x40bc07a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc07d; continue;
 }
 0x40bc07d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc07d; continue; }
  pc = 0x40bc091; continue;
 }
 0x40bc091 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40bc098; continue;
 }
 0x40bc098 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bbf1b; continue; }
  pc = 0x40bc0a4; continue;
 }
 0x40bc0a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bc0a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bc0a5); }
 let mut pc: u32 = 0x40bc0a5;
 loop { match pc {
 0x40bc0a5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x40bc0b5; continue;
 }
 0x40bc0b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bc0c2; continue; }
  pc = 0x40bc0bf; continue;
 }
 0x40bc0bf => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x40bc0c2; continue;
 }
 0x40bc0c2 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40bc0ca; continue; }
  pc = 0x40bc0c7; continue;
 }
 0x40bc0c7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x40bc0ca; continue;
 }
 0x40bc0ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc0b5; continue; }
  pc = 0x40bc0e1; continue;
 }
 0x40bc0e1 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4281c0cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4281c10u32, v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.rf32(0x4281c0cu32); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4154a4cu32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4281c10u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4281c0cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = ((!cpu.cf && !cpu.zf)) as u8; cpu.w8(0x4154a50u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x7fffu32; cpu.w32(0x42b5e40u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc129; continue;
 }
 0x40bc129 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x42bb248u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bc14a; continue; }
  pc = 0x40bc13d; continue;
 }
 0x40bc13d => {
  if cpu.zf { pc = 0x40bc48d; continue; }
  pc = 0x40bc143; continue;
 }
 0x40bc143 => {
  { let v: u32 = 0x42b5e48u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x40bc14a; continue;
 }
 0x40bc14a => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bb8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bbcu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bbcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bb8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x42b5e44u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc182; continue; }
  pc = 0x40bc17c; continue;
 }
 0x40bc17c => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc182; continue;
 }
 0x40bc182 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x42b5e40u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bc190; continue; }
  pc = 0x40bc18a; continue;
 }
 0x40bc18a => {
  { let v: u32 = cpu.edx; cpu.w32(0x42b5e40u32, v); }
  pc = 0x40bc190; continue;
 }
 0x40bc190 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc48d; continue; }
  pc = 0x40bc198; continue;
 }
 0x40bc198 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4154a48u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42b5ad8u32, v); }
  { let v = (cpu.r32(0x4281bb8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4281c04u32, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc8u32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4281bd8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bccu32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4281bdcu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x42b5ad8u32), 0x42bb248u32); }
  if (cpu.sf != cpu.of) { pc = 0x40bc20e; continue; }
  pc = 0x40bc209; continue;
 }
 0x40bc209 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  pc = 0x40bc20e; continue;
 }
 0x40bc20e => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4281bc0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc2be; continue; }
  pc = 0x40bc25d; continue;
 }
 0x40bc25d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc25d; continue; }
  pc = 0x40bc2be; continue;
 }
 0x40bc2be => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc2c1; continue;
 }
 0x40bc2c1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc2c1; continue; }
  pc = 0x40bc2d5; continue;
 }
 0x40bc2d5 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4281bdcu32); cpu.fpush(v); }
  { let v = cpu.rf32(0x4281bd8u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x4154a48u32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bc2ff; continue; }
  pc = 0x40bc2fd; continue;
 }
 0x40bc2fd => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40bc2ff; continue;
 }
 0x40bc2ff => {
  { cpu.sub8(cpu.r8(0x4154a50u32), 0x0u8); }
  if cpu.zf { pc = 0x40bc311; continue; }
  pc = 0x40bc308; continue;
 }
 0x40bc308 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x40bc33a; continue;
 }
 0x40bc311 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x30000u32); }
  if (cpu.sf == cpu.of) { pc = 0x40bc33a; continue; }
  pc = 0x40bc31f; continue;
 }
 0x40bc31f => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x10000u32); }
  if (cpu.sf == cpu.of) { pc = 0x40bc33a; continue; }
  pc = 0x40bc330; continue;
 }
 0x40bc330 => {
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x40bc33a; continue;
 }
 0x40bc33a => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if (cpu.sf != cpu.of) { pc = 0x40bc344; continue; }
  pc = 0x40bc340; continue;
 }
 0x40bc340 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x40bc344; continue;
 }
 0x40bc344 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4281bfcu32, v); }
  { let v: u32 = cpu.r32(0x42a27acu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4281bf8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4281c00u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4154a44u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x4281c04u32); cpu.fpush(v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4281bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4281bd0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4281bd4u32)); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4281bf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4281bf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4281bfcu32)); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc44a; continue; }
  pc = 0x40bc3e6; continue;
 }
 0x40bc3e6 => {
  { let v = cpu.st(0) + cpu.rf32(0x4281bf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x4281bf8u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281c00u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bd0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4281bd4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4281bfcu32); cpu.ecx = v; }
  pc = 0x40bc416; continue;
 }
 0x40bc416 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc416; continue; }
  pc = 0x40bc42c; continue;
 }
 0x40bc42c => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4281bfcu32)); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc3e6; continue; }
  pc = 0x40bc44a; continue;
 }
 0x40bc44a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x4281bfcu32)); cpu.ebp = v; }
  if cpu.zf { pc = 0x40bc47e; continue; }
  pc = 0x40bc452; continue;
 }
 0x40bc452 => {
  { let v: u32 = cpu.r32(0x4281c00u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bd0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4281bd4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  pc = 0x40bc468; continue;
 }
 0x40bc468 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc468; continue; }
  pc = 0x40bc47e; continue;
 }
 0x40bc47e => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40bc48d; continue;
 }
 0x40bc48d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc129; continue; }
  pc = 0x40bc499; continue;
 }
 0x40bc499 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bc49a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bc49a); }
 let mut pc: u32 = 0x40bc49a;
 loop { match pc {
 0x40bc49a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x40bc4a3; continue;
 }
 0x40bc4a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc4a3; continue; }
  pc = 0x40bc4bf; continue;
 }
 0x40bc4bf => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x42b5e40u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc4d3; continue;
 }
 0x40bc4d3 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x42bb248u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bc4f4; continue; }
  pc = 0x40bc4e7; continue;
 }
 0x40bc4e7 => {
  if cpu.zf { pc = 0x40bc738; continue; }
  pc = 0x40bc4ed; continue;
 }
 0x40bc4ed => {
  { let v: u32 = 0x42b5e48u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x40bc4f4; continue;
 }
 0x40bc4f4 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bb8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bbcu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bbcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bb8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x42b5e44u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc52c; continue; }
  pc = 0x40bc526; continue;
 }
 0x40bc526 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc52c; continue;
 }
 0x40bc52c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x42b5e40u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bc53a; continue; }
  pc = 0x40bc534; continue;
 }
 0x40bc534 => {
  { let v: u32 = cpu.edx; cpu.w32(0x42b5e40u32, v); }
  pc = 0x40bc53a; continue;
 }
 0x40bc53a => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc738; continue; }
  pc = 0x40bc542; continue;
 }
 0x40bc542 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4154a48u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42b5ad8u32, v); }
  { let v = (cpu.r32(0x4281bb8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc8u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc8u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd0u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bd0u32); cpu.ecx = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bccu32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bccu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bd4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc630; continue; }
  pc = 0x40bc5c8; continue;
 }
 0x40bc5c8 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc04u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1804u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc08u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1808u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x180cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1810u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1814u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc5c8; continue; }
  pc = 0x40bc630; continue;
 }
 0x40bc630 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc633; continue;
 }
 0x40bc633 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc633; continue; }
  pc = 0x40bc649; continue;
 }
 0x40bc649 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x42b5ad8u32), 0x42bb248u32); }
  if (cpu.sf != cpu.of) { pc = 0x40bc669; continue; }
  pc = 0x40bc667; continue;
 }
 0x40bc667 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40bc669; continue;
 }
 0x40bc669 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4281bc0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc719; continue; }
  pc = 0x40bc6b8; continue;
 }
 0x40bc6b8 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc6b8; continue; }
  pc = 0x40bc719; continue;
 }
 0x40bc719 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc71c; continue;
 }
 0x40bc71c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc71c; continue; }
  pc = 0x40bc730; continue;
 }
 0x40bc730 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40bc738; continue;
 }
 0x40bc738 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc4d3; continue; }
  pc = 0x40bc744; continue;
 }
 0x40bc744 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bc745(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bc745); }
 let mut pc: u32 = 0x40bc745;
 loop { match pc {
 0x40bc745 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x40bc74e; continue;
 }
 0x40bc74e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4000000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc74e; continue; }
  pc = 0x40bc763; continue;
 }
 0x40bc763 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x42b5e40u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc777; continue;
 }
 0x40bc777 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x42bb248u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bc798; continue; }
  pc = 0x40bc78b; continue;
 }
 0x40bc78b => {
  if cpu.zf { pc = 0x40bc972; continue; }
  pc = 0x40bc791; continue;
 }
 0x40bc791 => {
  { let v: u32 = 0x42b5e48u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x40bc798; continue;
 }
 0x40bc798 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bb8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bbcu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bbcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4281bb8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x42b5e44u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc7d0; continue; }
  pc = 0x40bc7ca; continue;
 }
 0x40bc7ca => {
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5e44u32, v); }
  pc = 0x40bc7d0; continue;
 }
 0x40bc7d0 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x42b5e40u32)); }
  if (cpu.sf == cpu.of) { pc = 0x40bc7de; continue; }
  pc = 0x40bc7d8; continue;
 }
 0x40bc7d8 => {
  { let v: u32 = cpu.edx; cpu.w32(0x42b5e40u32, v); }
  pc = 0x40bc7de; continue;
 }
 0x40bc7de => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc972; continue; }
  pc = 0x40bc7e6; continue;
 }
 0x40bc7e6 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4154a48u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42b5ad8u32, v); }
  { let v = (cpu.r32(0x4281bb8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154a40u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc8u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bd0u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4281bd0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc87a; continue; }
  pc = 0x40bc842; continue;
 }
 0x40bc842 => {
  { let v: u32 = cpu.eax; cpu.w32(0x2400u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2404u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2408u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x240cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2410u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2414u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc842; continue; }
  pc = 0x40bc87a; continue;
 }
 0x40bc87a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc87d; continue;
 }
 0x40bc87d => {
  { let v: u32 = cpu.eax; cpu.w32(0x2400u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc87d; continue; }
  pc = 0x40bc88b; continue;
 }
 0x40bc88b => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x42b5ad8u32), 0x42bb248u32); }
  if (cpu.sf != cpu.of) { pc = 0x40bc8a8; continue; }
  pc = 0x40bc8a6; continue;
 }
 0x40bc8a6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x40bc8a8; continue;
 }
 0x40bc8a8 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4281bc0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4281bc4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4281bc4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ad8u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bc954; continue; }
  pc = 0x40bc8f3; continue;
 }
 0x40bc8f3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bc8f3; continue; }
  pc = 0x40bc954; continue;
 }
 0x40bc954 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x40bc957; continue;
 }
 0x40bc957 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc957; continue; }
  pc = 0x40bc96b; continue;
 }
 0x40bc96b => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x40bc972; continue;
 }
 0x40bc972 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40bc777; continue; }
  pc = 0x40bc97e; continue;
 }
 0x40bc97e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bc97f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bc97f); }
 let mut pc: u32 = 0x40bc97f;
 loop { match pc {
 0x40bc97f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bca1d; continue; }
  pc = 0x40bc999; continue;
 }
 0x40bc999 => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42bfa58u32, v); }
  pc = 0x40bc9c8; continue;
 }
 0x40bc9c8 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bc9ce); f_040c08c7(cpu); if cpu.returned_to != 0x40bc9ce { return; }
  if cpu.zf { pc = 0x40bca0b; continue; }
  pc = 0x40bc9d0; continue;
 }
 0x40bc9d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.eax = v; }
  { let v: u32 = 0x428bc5cu32; cpu.edx = v; }
  pc = 0x40bc9e3; continue;
 }
 0x40bc9e3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40bca03; continue; }
  pc = 0x40bc9ec; continue;
 }
 0x40bc9ec => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40bc9f9; continue; }
  pc = 0x40bc9f6; continue;
 }
 0x40bc9f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x40bc9f9; continue;
 }
 0x40bc9f9 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x40bc9e3; continue; }
  pc = 0x40bc9ff; continue;
 }
 0x40bc9ff => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x40bc9e3; continue;
 }
 0x40bca03 => {
  if cpu.zf { pc = 0x40bca0b; continue; }
  pc = 0x40bca05; continue;
 }
 0x40bca05 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40bc9e3; continue;
 }
 0x40bca0b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bc9c8; continue; }
  pc = 0x40bca1d; continue;
 }
 0x40bca1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bca1e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bca1e); } r_040bca1e(cpu, 0x40bca1e); }
pub fn f_040bcb22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb22); } r_040bca1e(cpu, 0x40bcb22); }
pub fn f_040bcb32(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb32); } r_040bca1e(cpu, 0x40bcb32); }
pub fn f_040bcb42(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb42); } r_040bca1e(cpu, 0x40bcb42); }
pub fn f_040bcb52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb52); } r_040bca1e(cpu, 0x40bcb52); }
pub fn f_040bcb75(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb75); } r_040bca1e(cpu, 0x40bcb75); }
pub fn f_040bcb85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb85); } r_040bca1e(cpu, 0x40bcb85); }
pub fn f_040bcb95(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcb95); } r_040bca1e(cpu, 0x40bcb95); }
pub fn f_040bcba5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcba5); } r_040bca1e(cpu, 0x40bcba5); }
fn r_040bca1e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40bca1e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bcbdd; continue; }
  pc = 0x40bca38; continue;
 }
 0x40bca38 => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7bef7befu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42bfa58u32, v); }
  pc = 0x40bca6d; continue;
 }
 0x40bca6d => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bca73); f_040c113a(cpu); if cpu.returned_to != 0x40bca73 { return; }
  if cpu.zf { pc = 0x40bcbc7; continue; }
  pc = 0x40bca79; continue;
 }
 0x40bca79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.ebx = v; }
  { let v: u32 = 0x428bc5cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bcbb4; continue; }
  pc = 0x40bca97; continue;
 }
 0x40bca97 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40bcab4; continue; }
  pc = 0x40bca9f; continue;
 }
 0x40bca9f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40bcab4; continue;
 }
 0x40bcab4 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bcb11; continue; }
  pc = 0x40bcaca; continue;
 }
 0x40bcaca => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bcaca; continue; }
  pc = 0x40bcb11; continue;
 }
 0x40bcb11 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154a54u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bcb22 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcb32; continue;
 }
 0x40bcb32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcb42; continue;
 }
 0x40bcb42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcb52; continue;
 }
 0x40bcb52 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bca97; continue; }
  pc = 0x40bcb71; continue;
 }
 0x40bcb71 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bcbb4; continue; }
  pc = 0x40bcb73; continue;
 }
 0x40bcb73 => {
  if cpu.zf { pc = 0x40bcbc7; continue; }
  pc = 0x40bcb75; continue;
 }
 0x40bcb75 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcb85; continue;
 }
 0x40bcb85 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcb95; continue;
 }
 0x40bcb95 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bcba5; continue;
 }
 0x40bcba5 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bca97; continue; }
  pc = 0x40bcbb2; continue;
 }
 0x40bcbb2 => {
  if cpu.zf { pc = 0x40bcbc7; continue; }
  pc = 0x40bcbb4; continue;
 }
 0x40bcbb4 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bca97; continue; }
  pc = 0x40bcbc5; continue;
 }
 0x40bcbc5 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bcbb4; continue; }
  pc = 0x40bcbc7; continue;
 }
 0x40bcbc7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bca6d; continue; }
  pc = 0x40bcbdd; continue;
 }
 0x40bcbdd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bcbde(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bcbde); } r_040bcbde(cpu, 0x40bcbde); }
pub fn f_040bd13d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd13d); } r_040bcbde(cpu, 0x40bd13d); }
pub fn f_040bd15d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd15d); } r_040bcbde(cpu, 0x40bd15d); }
pub fn f_040bd17d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd17d); } r_040bcbde(cpu, 0x40bd17d); }
pub fn f_040bd19d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd19d); } r_040bcbde(cpu, 0x40bd19d); }
pub fn f_040bd1bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd1bd); } r_040bcbde(cpu, 0x40bd1bd); }
pub fn f_040bd1dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd1dd); } r_040bcbde(cpu, 0x40bd1dd); }
pub fn f_040bd1fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd1fd); } r_040bcbde(cpu, 0x40bd1fd); }
pub fn f_040bd21d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd21d); } r_040bcbde(cpu, 0x40bd21d); }
pub fn f_040bd24d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd24d); } r_040bcbde(cpu, 0x40bd24d); }
pub fn f_040bd26d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd26d); } r_040bcbde(cpu, 0x40bd26d); }
pub fn f_040bd28d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd28d); } r_040bcbde(cpu, 0x40bd28d); }
pub fn f_040bd2ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd2ad); } r_040bcbde(cpu, 0x40bd2ad); }
pub fn f_040bd2cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd2cd); } r_040bcbde(cpu, 0x40bd2cd); }
pub fn f_040bd2ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd2ed); } r_040bcbde(cpu, 0x40bd2ed); }
pub fn f_040bd30d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd30d); } r_040bcbde(cpu, 0x40bd30d); }
pub fn f_040bd32d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd32d); } r_040bcbde(cpu, 0x40bd32d); }
pub fn f_040bd34d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd34d); } r_040bcbde(cpu, 0x40bd34d); }
fn r_040bcbde(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40bcbde => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bd7c0; continue; }
  pc = 0x40bcbf8; continue;
 }
 0x40bcbf8 => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa68u32, v); }
  pc = 0x40bcc1e; continue;
 }
 0x40bcc1e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bcc25); f_040c08c7(cpu); if cpu.returned_to != 0x40bcc25 { return; }
  if cpu.zf { pc = 0x40bd7a9; continue; }
  pc = 0x40bcc2b; continue;
 }
 0x40bcc2b => {
  { cpu.sub32(cpu.r32(0x42bfa68u32), 0x0u32); }
  if !cpu.zf { pc = 0x40bcc41; continue; }
  pc = 0x40bcc34; continue;
 }
 0x40bcc34 => {
  cpu.push(0x40bcc39); f_040c1b67(cpu); if cpu.returned_to != 0x40bcc39 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40bd7a9; continue; }
  pc = 0x40bcc41; continue;
 }
 0x40bcc41 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5b14u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42b5b1cu32, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = (cpu.r32(0x42b5b14u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x42b5b1cu32) as i32 as f64); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf32(0x4154a48u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4154a44u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x4154a44u32); cpu.fpush(v); }
  { let v: u32 = 0x428bc5cu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bcd43; continue; }
  pc = 0x40bccc0; continue;
 }
 0x40bccc0 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x40bcccc; continue; }
  pc = 0x40bccc7; continue;
 }
 0x40bccc7 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x40bcccc; continue;
 }
 0x40bcccc => {
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa68u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x428bc5cu32; cpu.w32(0x4154a74u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x428bc5cu32); cpu.eax = v; }
  pc = 0x40bcdb6; continue;
 }
 0x40bcd43 => {
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa68u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x428bc5cu32; cpu.w32(0x4154a74u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x428bc5cu32); cpu.eax = v; }
  pc = 0x40bd6ff; continue;
 }
 0x40bcdb6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x40bd36b; continue; }
  pc = 0x40bcdcc; continue;
 }
 0x40bcdcc => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bcf34; continue; }
  pc = 0x40bcddb; continue;
 }
 0x40bcddb => {
  { let v = cpu.st(0) + cpu.rf32(0x42a27ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a37ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bcddb; continue; }
  pc = 0x40bcf34; continue;
 }
 0x40bcf34 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bd09c; continue; }
  pc = 0x40bcf43; continue;
 }
 0x40bcf43 => {
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x40bd09c; continue;
 }
 0x40bd09c => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40bd111; continue; }
  pc = 0x40bd0aa; continue;
 }
 0x40bd0aa => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154a78u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bd111 => {
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154a78u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bd13d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd15d; continue;
 }
 0x40bd15d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd17d; continue;
 }
 0x40bd17d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd19d; continue;
 }
 0x40bd19d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd1bd; continue;
 }
 0x40bd1bd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd1dd; continue;
 }
 0x40bd1dd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd1fd; continue;
 }
 0x40bd1fd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd21d; continue;
 }
 0x40bd21d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40bd79d; continue; }
  pc = 0x40bd23e; continue;
 }
 0x40bd23e => {
  { let v: u32 = cpu.r32(0x4154a74u32).wrapping_add(0x4u32); cpu.w32(0x4154a74u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40bd6ff; continue;
 }
 0x40bd24d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd26d; continue;
 }
 0x40bd26d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd28d; continue;
 }
 0x40bd28d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd2ad; continue;
 }
 0x40bd2ad => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd2cd; continue;
 }
 0x40bd2cd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd2ed; continue;
 }
 0x40bd2ed => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd30d; continue;
 }
 0x40bd30d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd32d; continue;
 }
 0x40bd32d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bd34d; continue;
 }
 0x40bd34d => {
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40bd79d; continue; }
  pc = 0x40bd35c; continue;
 }
 0x40bd35c => {
  { let v: u32 = cpu.r32(0x4154a74u32).wrapping_add(0x4u32); cpu.w32(0x4154a74u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40bd6ff; continue;
 }
 0x40bd36b => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bd4d5; continue; }
  pc = 0x40bd37a; continue;
 }
 0x40bd37a => {
  { let v = cpu.st(0) + cpu.rf32(0x42a27ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffcau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffceu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a37ecu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bd37a; continue; }
  pc = 0x40bd4d5; continue;
 }
 0x40bd4d5 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bd63f; continue; }
  pc = 0x40bd4e4; continue;
 }
 0x40bd4e4 => {
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x40bd63f; continue;
 }
 0x40bd63f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40bd6b4; continue; }
  pc = 0x40bd64d; continue;
 }
 0x40bd64d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154a78u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bd6b4 => {
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x40bd34d; continue; }
  pc = 0x40bd6df; continue;
 }
 0x40bd6df => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154a78u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bd6ff => {
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bd713; continue; }
  pc = 0x40bd70e; continue;
 }
 0x40bd70e => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x40bd713; continue;
 }
 0x40bd713 => {
  { let v = cpu.st(0) + cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x42a27acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4154a74u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x40bd79d; continue; }
  pc = 0x40bd78e; continue;
 }
 0x40bd78e => {
  { let v: u32 = cpu.r32(0x4154a74u32).wrapping_add(0x4u32); cpu.w32(0x4154a74u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x40bcdb6; continue;
 }
 0x40bd79d => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40bd7a9; continue;
 }
 0x40bd7a9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bcc1e; continue; }
  pc = 0x40bd7c0; continue;
 }
 0x40bd7c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bd7c1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bd7c1); } r_040bd7c1(cpu, 0x40bd7c1); }
pub fn f_040bda6e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bda6e); } r_040bd7c1(cpu, 0x40bda6e); }
pub fn f_040bda96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bda96); } r_040bd7c1(cpu, 0x40bda96); }
pub fn f_040bdabe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdabe); } r_040bd7c1(cpu, 0x40bdabe); }
pub fn f_040bdae6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdae6); } r_040bd7c1(cpu, 0x40bdae6); }
pub fn f_040bdb0e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdb0e); } r_040bd7c1(cpu, 0x40bdb0e); }
pub fn f_040bdb36(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdb36); } r_040bd7c1(cpu, 0x40bdb36); }
pub fn f_040bdb5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdb5e); } r_040bd7c1(cpu, 0x40bdb5e); }
pub fn f_040bdb86(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdb86); } r_040bd7c1(cpu, 0x40bdb86); }
pub fn f_040bdbb5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdbb5); } r_040bd7c1(cpu, 0x40bdbb5); }
pub fn f_040bdbdd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdbdd); } r_040bd7c1(cpu, 0x40bdbdd); }
pub fn f_040bdc05(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdc05); } r_040bd7c1(cpu, 0x40bdc05); }
pub fn f_040bdc2d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdc2d); } r_040bd7c1(cpu, 0x40bdc2d); }
pub fn f_040bdc55(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdc55); } r_040bd7c1(cpu, 0x40bdc55); }
pub fn f_040bdc7d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdc7d); } r_040bd7c1(cpu, 0x40bdc7d); }
pub fn f_040bdca5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdca5); } r_040bd7c1(cpu, 0x40bdca5); }
pub fn f_040bdccd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdccd); } r_040bd7c1(cpu, 0x40bdccd); }
pub fn f_040bdd28(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd28); } r_040bd7c1(cpu, 0x40bdd28); }
pub fn f_040bdd34(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd34); } r_040bd7c1(cpu, 0x40bdd34); }
pub fn f_040bdd40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd40); } r_040bd7c1(cpu, 0x40bdd40); }
pub fn f_040bdd4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd4c); } r_040bd7c1(cpu, 0x40bdd4c); }
pub fn f_040bdd58(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd58); } r_040bd7c1(cpu, 0x40bdd58); }
pub fn f_040bdd64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd64); } r_040bd7c1(cpu, 0x40bdd64); }
pub fn f_040bdd70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd70); } r_040bd7c1(cpu, 0x40bdd70); }
pub fn f_040bdd7c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd7c); } r_040bd7c1(cpu, 0x40bdd7c); }
pub fn f_040bdd88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd88); } r_040bd7c1(cpu, 0x40bdd88); }
pub fn f_040bdd94(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdd94); } r_040bd7c1(cpu, 0x40bdd94); }
pub fn f_040bdda0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdda0); } r_040bd7c1(cpu, 0x40bdda0); }
pub fn f_040bddac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bddac); } r_040bd7c1(cpu, 0x40bddac); }
pub fn f_040bddb8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bddb8); } r_040bd7c1(cpu, 0x40bddb8); }
pub fn f_040bddc4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bddc4); } r_040bd7c1(cpu, 0x40bddc4); }
pub fn f_040bddd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bddd0); } r_040bd7c1(cpu, 0x40bddd0); }
pub fn f_040bdddc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdddc); } r_040bd7c1(cpu, 0x40bdddc); }
fn r_040bd7c1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40bd7c1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bde0c; continue; }
  pc = 0x40bd7db; continue;
 }
 0x40bd7db => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa68u32, v); }
  pc = 0x40bd801; continue;
 }
 0x40bd801 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bd808); f_040c08c7(cpu); if cpu.returned_to != 0x40bd808 { return; }
  if cpu.zf { pc = 0x40bddf5; continue; }
  pc = 0x40bd80e; continue;
 }
 0x40bd80e => {
  { cpu.sub32(cpu.r32(0x42bfa68u32), 0x0u32); }
  if !cpu.zf { pc = 0x40bd824; continue; }
  pc = 0x40bd817; continue;
 }
 0x40bd817 => {
  cpu.push(0x40bd81c); f_040c1b67(cpu); if cpu.returned_to != 0x40bd81c { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40bddf5; continue; }
  pc = 0x40bd824; continue;
 }
 0x40bd824 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5b14u32, v); }
  { let v = (cpu.r32(0x42b5b14u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa68u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5b1cu32, v); }
  { let v = (cpu.r32(0x42b5b1cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42b5af8u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x428bc5cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bdce0; continue; }
  pc = 0x40bd8e3; continue;
 }
 0x40bd8e3 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40bd904; continue; }
  pc = 0x40bd8eb; continue;
 }
 0x40bd8eb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40bd904; continue;
 }
 0x40bd904 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bda5d; continue; }
  pc = 0x40bd91e; continue;
 }
 0x40bd91e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bd91e; continue; }
  pc = 0x40bda5d; continue;
 }
 0x40bda5d => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154abcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bda6e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bda96; continue;
 }
 0x40bda96 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdabe; continue;
 }
 0x40bdabe => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdae6; continue;
 }
 0x40bdae6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdb0e; continue;
 }
 0x40bdb0e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdb36; continue;
 }
 0x40bdb36 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdb5e; continue;
 }
 0x40bdb5e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdb86; continue;
 }
 0x40bdb86 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bd8e3; continue; }
  pc = 0x40bdba9; continue;
 }
 0x40bdba9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bdce0; continue; }
  pc = 0x40bdbaf; continue;
 }
 0x40bdbaf => {
  if cpu.zf { pc = 0x40bddf5; continue; }
  pc = 0x40bdbb5; continue;
 }
 0x40bdbb5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdbdd; continue;
 }
 0x40bdbdd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdc05; continue;
 }
 0x40bdc05 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdc2d; continue;
 }
 0x40bdc2d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdc55; continue;
 }
 0x40bdc55 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdc7d; continue;
 }
 0x40bdc7d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdca5; continue;
 }
 0x40bdca5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40bdccd; continue;
 }
 0x40bdccd => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bd8e3; continue; }
  pc = 0x40bdcda; continue;
 }
 0x40bdcda => {
  if cpu.zf { pc = 0x40bddf5; continue; }
  pc = 0x40bdce0; continue;
 }
 0x40bdce0 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bdd17; continue; }
  pc = 0x40bdcf4; continue;
 }
 0x40bdcf4 => {
  { let v: u32 = cpu.r32(0x42b5af8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x40bdd01; continue;
 }
 0x40bdd01 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bdd01; continue; }
  pc = 0x40bdd14; continue;
 }
 0x40bdd14 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x40bdd17; continue;
 }
 0x40bdd17 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154afcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40bdd28 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd34; continue;
 }
 0x40bdd34 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd40; continue;
 }
 0x40bdd40 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd4c; continue;
 }
 0x40bdd4c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd58; continue;
 }
 0x40bdd58 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd64; continue;
 }
 0x40bdd64 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd70; continue;
 }
 0x40bdd70 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd7c; continue;
 }
 0x40bdd7c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd88; continue;
 }
 0x40bdd88 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdd94; continue;
 }
 0x40bdd94 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdda0; continue;
 }
 0x40bdda0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bddac; continue;
 }
 0x40bddac => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bddb8; continue;
 }
 0x40bddb8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bddc4; continue;
 }
 0x40bddc4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bddd0; continue;
 }
 0x40bddd0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40bdddc; continue;
 }
 0x40bdddc => {
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bd8e3; continue; }
  pc = 0x40bddef; continue;
 }
 0x40bddef => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40bdce0; continue; }
  pc = 0x40bddf5; continue;
 }
 0x40bddf5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bd801; continue; }
  pc = 0x40bde0c; continue;
 }
 0x40bde0c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bde0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40bde0d); }
 let mut pc: u32 = 0x40bde0d;
 loop { match pc {
 0x40bde0d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40bdeb5; continue; }
  pc = 0x40bde27; continue;
 }
 0x40bde27 => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42bfa58u32, v); }
  pc = 0x40bde56; continue;
 }
 0x40bde56 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bde5c); f_040c1649(cpu); if cpu.returned_to != 0x40bde5c { return; }
  if cpu.zf { pc = 0x40bdea3; continue; }
  pc = 0x40bde5e; continue;
 }
 0x40bde5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa58u32); cpu.eax = v; }
  { let v: u32 = 0x428bc5cu32; cpu.edx = v; }
  pc = 0x40bde71; continue;
 }
 0x40bde71 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x40bde9b; continue; }
  pc = 0x40bde7a; continue;
 }
 0x40bde7a => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40bde86; continue; }
  pc = 0x40bde81; continue;
 }
 0x40bde81 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.poll(); pc = 0x40bde71; continue;
 }
 0x40bde86 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40bde91; continue; }
  pc = 0x40bde8e; continue;
 }
 0x40bde8e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x40bde91; continue;
 }
 0x40bde91 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x40bde71; continue; }
  pc = 0x40bde97; continue;
 }
 0x40bde97 => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x40bde71; continue;
 }
 0x40bde9b => {
  if cpu.zf { pc = 0x40bdea3; continue; }
  pc = 0x40bde9d; continue;
 }
 0x40bde9d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x40bde71; continue;
 }
 0x40bdea3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bde56; continue; }
  pc = 0x40bdeb5; continue;
 }
 0x40bdeb5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040bdeb6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40bdeb6); } r_040bdeb6(cpu, 0x40bdeb6); }
pub fn f_040be170(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be170); } r_040bdeb6(cpu, 0x40be170); }
pub fn f_040be198(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be198); } r_040bdeb6(cpu, 0x40be198); }
pub fn f_040be1c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be1c0); } r_040bdeb6(cpu, 0x40be1c0); }
pub fn f_040be1e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be1e8); } r_040bdeb6(cpu, 0x40be1e8); }
pub fn f_040be210(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be210); } r_040bdeb6(cpu, 0x40be210); }
pub fn f_040be238(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be238); } r_040bdeb6(cpu, 0x40be238); }
pub fn f_040be260(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be260); } r_040bdeb6(cpu, 0x40be260); }
pub fn f_040be288(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be288); } r_040bdeb6(cpu, 0x40be288); }
pub fn f_040be2b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be2b7); } r_040bdeb6(cpu, 0x40be2b7); }
pub fn f_040be2df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be2df); } r_040bdeb6(cpu, 0x40be2df); }
pub fn f_040be307(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be307); } r_040bdeb6(cpu, 0x40be307); }
pub fn f_040be32f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be32f); } r_040bdeb6(cpu, 0x40be32f); }
pub fn f_040be357(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be357); } r_040bdeb6(cpu, 0x40be357); }
pub fn f_040be37f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be37f); } r_040bdeb6(cpu, 0x40be37f); }
pub fn f_040be3a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be3a7); } r_040bdeb6(cpu, 0x40be3a7); }
pub fn f_040be3cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be3cf); } r_040bdeb6(cpu, 0x40be3cf); }
pub fn f_040be42a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be42a); } r_040bdeb6(cpu, 0x40be42a); }
pub fn f_040be436(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be436); } r_040bdeb6(cpu, 0x40be436); }
pub fn f_040be442(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be442); } r_040bdeb6(cpu, 0x40be442); }
pub fn f_040be44e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be44e); } r_040bdeb6(cpu, 0x40be44e); }
pub fn f_040be45a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be45a); } r_040bdeb6(cpu, 0x40be45a); }
pub fn f_040be466(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be466); } r_040bdeb6(cpu, 0x40be466); }
pub fn f_040be472(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be472); } r_040bdeb6(cpu, 0x40be472); }
pub fn f_040be47e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be47e); } r_040bdeb6(cpu, 0x40be47e); }
pub fn f_040be48a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be48a); } r_040bdeb6(cpu, 0x40be48a); }
pub fn f_040be496(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be496); } r_040bdeb6(cpu, 0x40be496); }
pub fn f_040be4a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4a2); } r_040bdeb6(cpu, 0x40be4a2); }
pub fn f_040be4ae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4ae); } r_040bdeb6(cpu, 0x40be4ae); }
pub fn f_040be4ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4ba); } r_040bdeb6(cpu, 0x40be4ba); }
pub fn f_040be4c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4c6); } r_040bdeb6(cpu, 0x40be4c6); }
pub fn f_040be4d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4d2); } r_040bdeb6(cpu, 0x40be4d2); }
pub fn f_040be4de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40be4de); } r_040bdeb6(cpu, 0x40be4de); }
fn r_040bdeb6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40bdeb6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x42bfa50u32)); cpu.w32(0x42bfa50u32, v); }
  { let v: u32 = cpu.r32(0x42b5e44u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42b5e40u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40be50e; continue; }
  pc = 0x40bded0; continue;
 }
 0x40bded0 => {
  { let v: u32 = 0x42b5e48u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42b5af0u32, v); }
  { let v: u32 = cpu.r32(0x4155368u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x4155364u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x42bfa68u32, v); }
  pc = 0x40bdef6; continue;
 }
 0x40bdef6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40bdefd); f_040c1649(cpu); if cpu.returned_to != 0x40bdefd { return; }
  if cpu.zf { pc = 0x40be4f7; continue; }
  pc = 0x40bdf03; continue;
 }
 0x40bdf03 => {
  { cpu.sub32(cpu.r32(0x42bfa68u32), 0x0u32); }
  if !cpu.zf { pc = 0x40bdf19; continue; }
  pc = 0x40bdf0c; continue;
 }
 0x40bdf0c => {
  cpu.push(0x40bdf11); f_040c1b67(cpu); if cpu.returned_to != 0x40bdf11 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40be4f7; continue; }
  pc = 0x40bdf19; continue;
 }
 0x40bdf19 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5b14u32, v); }
  { let v = (cpu.r32(0x42b5b14u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42bfa68u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b14u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5b1cu32, v); }
  { let v = (cpu.r32(0x42b5b1cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x42a37acu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x4154d80u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x42b5b14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x42b5b1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x42b5b1cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42b5af8u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x428bc5cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40be3e2; continue; }
  pc = 0x40bdfd8; continue;
 }
 0x40bdfd8 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if (cpu.sf != cpu.of) { pc = 0x40bdfe5; continue; }
  pc = 0x40bdfdd; continue;
 }
 0x40bdfdd => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x40be3cf; continue;
 }
 0x40bdfe5 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x40be006; continue; }
  pc = 0x40bdfed; continue;
 }
 0x40bdfed => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40be006; continue;
 }
 0x40be006 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40be15f; continue; }
  pc = 0x40be020; continue;
 }
 0x40be020 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40be020; continue; }
  pc = 0x40be15f; continue;
 }
 0x40be15f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154b3cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40be170 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be198; continue;
 }
 0x40be198 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be1c0; continue;
 }
 0x40be1c0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be1e8; continue;
 }
 0x40be1e8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be210; continue;
 }
 0x40be210 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be238; continue;
 }
 0x40be238 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be260; continue;
 }
 0x40be260 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be288; continue;
 }
 0x40be288 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bdfd8; continue; }
  pc = 0x40be2ab; continue;
 }
 0x40be2ab => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40be3e2; continue; }
  pc = 0x40be2b1; continue;
 }
 0x40be2b1 => {
  if cpu.zf { pc = 0x40be4f7; continue; }
  pc = 0x40be2b7; continue;
 }
 0x40be2b7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be2df; continue;
 }
 0x40be2df => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be307; continue;
 }
 0x40be307 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be32f; continue;
 }
 0x40be32f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be357; continue;
 }
 0x40be357 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be37f; continue;
 }
 0x40be37f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be3a7; continue;
 }
 0x40be3a7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x40be3cf; continue;
 }
 0x40be3cf => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bdfd8; continue; }
  pc = 0x40be3dc; continue;
 }
 0x40be3dc => {
  if cpu.zf { pc = 0x40be4f7; continue; }
  pc = 0x40be3e2; continue;
 }
 0x40be3e2 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42b5ae4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40be419; continue; }
  pc = 0x40be3f6; continue;
 }
 0x40be3f6 => {
  { let v: u32 = cpu.r32(0x42b5af8u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x40be403; continue;
 }
 0x40be403 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x42b5ae4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40be403; continue; }
  pc = 0x40be416; continue;
 }
 0x40be416 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x40be419; continue;
 }
 0x40be419 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x4154b7cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x40be42a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be436; continue;
 }
 0x40be436 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be442; continue;
 }
 0x40be442 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be44e; continue;
 }
 0x40be44e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be45a; continue;
 }
 0x40be45a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be466; continue;
 }
 0x40be466 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be472; continue;
 }
 0x40be472 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be47e; continue;
 }
 0x40be47e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be48a; continue;
 }
 0x40be48a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be496; continue;
 }
 0x40be496 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4a2; continue;
 }
 0x40be4a2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4ae; continue;
 }
 0x40be4ae => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4ba; continue;
 }
 0x40be4ba => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4c6; continue;
 }
 0x40be4c6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4d2; continue;
 }
 0x40be4d2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x42b5af8u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40be4de; continue;
 }
 0x40be4de => {
  { let v: u32 = cpu.r32(0x42b5ae4u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40bdfd8; continue; }
  pc = 0x40be4f1; continue;
 }
 0x40be4f1 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40be3e2; continue; }
  pc = 0x40be4f7; continue;
 }
 0x40be4f7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x4155368u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42b5af0u32)); cpu.w32(0x42b5af0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x40bdef6; continue; }
  pc = 0x40be50e; continue;
 }
 0x40be50e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
