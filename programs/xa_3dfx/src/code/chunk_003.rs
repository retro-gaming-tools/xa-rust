#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010a2ded(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2ded); }
 let mut pc: u32 = 0x10a2ded;
 loop { match pc {
 0x10a2ded => {
  pc = 0x10b25df; continue;
 }
 0x10b25df => {
  { cpu.sub8(cpu.r8(0x117ba14u32), 0x1u8); }
  if cpu.zf { pc = 0x10b25e9; continue; }
  pc = 0x10b25e8; continue;
 }
 0x10b25e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b25e9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x117ba4cu32), 0x0u8); }
  if cpu.zf { pc = 0x10b2601; continue; }
  pc = 0x10b25fa; continue;
 }
 0x10b25fa => {
  cpu.push(0x10b25ff); f_010b2694(cpu); if cpu.returned_to != 0x10b25ff { return; }
  pc = 0x10b264e; continue;
 }
 0x10b2601 => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x10b261a; continue; }
  pc = 0x10b2605; continue;
 }
 0x10b2605 => {
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117b1dcu32); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx)); cpu.push(0x10b2618); cpu.call(t); if cpu.returned_to != 0x10b2618 { return; }
  pc = 0x10b264e; continue;
 }
 0x10b261a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { pc = 0x10b262a; continue; }
  pc = 0x10b261e; continue;
 }
 0x10b261e => {
  { let v: u16 = cpu.r16(0x117ba12u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x10b264e; continue;
 }
 0x10b262a => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x10b263e; continue; }
  pc = 0x10b262e; continue;
 }
 0x10b262e => {
  { let v: u16 = cpu.r16(0x117b1e0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.push(0x10b263c); f_010c342a(cpu); if cpu.returned_to != 0x10b263c { return; }
  pc = 0x10b264e; continue;
 }
 0x10b263e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  cpu.push(0x10b2645); f_010c342a(cpu); if cpu.returned_to != 0x10b2645 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b264e; continue; }
  pc = 0x10b2649; continue;
 }
 0x10b2649 => {
  cpu.push(0x10b264e); f_010b2659(cpu); if cpu.returned_to != 0x10b264e { return; }
  pc = 0x10b264e; continue;
 }
 0x10b264e => {
  { let v: u8 = 0x0u8; cpu.w8(0x117ba14u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2df2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2df2); }
 let mut pc: u32 = 0x10a2df2;
 loop { match pc {
 0x10a2df2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a2e0b; continue; }
  pc = 0x10a2dfd; continue;
 }
 0x10a2dfd => {
  { let v: u32 = cpu.r32(0x11742f0u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a2e81; continue; }
  pc = 0x10a2e0b; continue;
 }
 0x10a2e0b => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a2e12); f_010b2744(cpu); if cpu.returned_to != 0x10a2e12 { return; }
  pc = 0x10a2e34; continue;
 }
 0x10a2e14 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b7b0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.edx); }
  if cpu.zf { pc = 0x10a2e3c; continue; }
  pc = 0x10a2e33; continue;
 }
 0x10a2e33 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10a2e34; continue;
 }
 0x10a2e34 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2e14; continue; }
  pc = 0x10a2e3c; continue;
 }
 0x10a2e3c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a2e81; continue; }
  pc = 0x10a2e40; continue;
 }
 0x10a2e40 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10a2e71; continue;
 }
 0x10a2e44 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x116b7b0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.edx); }
  if cpu.zf { pc = 0x10a2e70; continue; }
  pc = 0x10a2e65; continue;
 }
 0x10a2e65 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11742f0u32, v); }
  pc = 0x10a2e7f; continue;
 }
 0x10a2e70 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a2e71; continue;
 }
 0x10a2e71 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10a2e44; continue; }
  pc = 0x10a2e79; continue;
 }
 0x10a2e79 => {
  { let v: u32 = cpu.edx; cpu.w32(0x11742f0u32, v); }
  pc = 0x10a2e7f; continue;
 }
 0x10a2e7f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10a2e81; continue;
 }
 0x10a2e81 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2e87); }
 let mut pc: u32 = 0x10a2e87;
 loop { match pc {
 0x10a2e87 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x10a2e8b; continue;
 }
 0x10a2e8b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a2e9f; continue; }
  pc = 0x10a2e98; continue;
 }
 0x10a2e98 => {
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a2e9f; continue; }
  pc = 0x10a2e9d; continue;
 }
 0x10a2e9d => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a2e9f; continue;
 }
 0x10a2e9f => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a2eb0; continue; }
  pc = 0x10a2ea8; continue;
 }
 0x10a2ea8 => {
  { cpu.sub32(cpu.ecx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a2eb0; continue; }
  pc = 0x10a2ead; continue;
 }
 0x10a2ead => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10a2eb0; continue;
 }
 0x10a2eb0 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10a2ebc; continue; }
  pc = 0x10a2eb4; continue;
 }
 0x10a2eb4 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10a2ebc; continue; }
  pc = 0x10a2eb8; continue;
 }
 0x10a2eb8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x10a2e8b; continue;
 }
 0x10a2ebc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2ed0); }
 let mut pc: u32 = 0x10a2ed0;
 loop { match pc {
 0x10a2ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.push(0x10a2edc); f_010a0d60(cpu); if cpu.returned_to != 0x10a2edc { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a2f09; continue; }
  pc = 0x10a2ee1; continue;
 }
 0x10a2ee1 => {
  { let v: u32 = 0x11690b8u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x11742f6u32; cpu.edx = v; }
  { let v: u32 = 0x10a30c8u32; cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a2f05); f_010a48c8(cpu); if cpu.returned_to != 0x10a2f05 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a2f3a; continue;
 }
 0x10a2f09 => {
  { cpu.sub16(cpu.r16(0x11742f4u32), 0x0u16); }
  if !cpu.zf { pc = 0x10a2f35; continue; }
  pc = 0x10a2f13; continue;
 }
 0x10a2f13 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1251ed8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1251ed4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1251ed0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11742f4u32, v); }
  pc = 0x10a2f35; continue;
 }
 0x10a2f35 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a2f3a; continue;
 }
 0x10a2f3a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2f44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2f44); }
 let mut pc: u32 = 0x10a2f44;
 loop { match pc {
 0x10a2f44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x11742f4u32), 0x0u16); }
  if cpu.zf { pc = 0x10a2f58; continue; }
  pc = 0x10a2f4f; continue;
 }
 0x10a2f4f => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11742f4u32, v); }
  pc = 0x10a2f58; continue;
 }
 0x10a2f58 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2f5c); }
 let mut pc: u32 = 0x10a2f5c;
 loop { match pc {
 0x10a2f5c => {
  { cpu.sub8(cpu.r8(0x1251ed8u32), 0x0u8); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a2f8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a2f8c); }
 let mut pc: u32 = 0x10a2f8c;
 loop { match pc {
 0x10a2f8c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251ed0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8(cpu.r8(0x1251ed8u32), 0x2u8); }
  if !cpu.zf { pc = 0x10a2fa5; continue; }
  pc = 0x10a2fa1; continue;
 }
 0x10a2fa1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a2ff3; continue;
 }
 0x10a2fa5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x1251bd0u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1251ed0u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if !cpu.zf { pc = 0x10a2fde; continue; }
  pc = 0x10a2fdc; continue;
 }
 0x10a2fdc => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a2fde; continue;
 }
 0x10a2fde => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x1251ed4u32)); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1251ed8u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a2ff3; continue;
 }
 0x10a2ff3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x1251ed0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3000); }
 let mut pc: u32 = 0x10a3000;
 loop { match pc {
 0x10a3000 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub8(cpu.r8(0x1251ed8u32), 0x0u8); }
  if !cpu.zf { pc = 0x10a3043; continue; }
  pc = 0x10a300f; continue;
 }
 0x10a300f => {
  { let v: u32 = cpu.r32(0x1174098u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10a3022); f_0109e944(cpu); if cpu.returned_to != 0x10a3022 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x12510a8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x12510a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3043 => {
  { let v: u32 = cpu.r32(0x1251ed4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1251bd0u32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1251ed4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1251ed4u32, v); }
  { cpu.sub32(cpu.edx, 0x20u32); }
  if !cpu.zf { pc = 0x10a308e; continue; }
  pc = 0x10a3086; continue;
 }
 0x10a3086 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1251ed4u32, v); }
  pc = 0x10a308e; continue;
 }
 0x10a308e => {
  { let v: u32 = cpu.r32(0x1251ed4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1251ed0u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1251ed8u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a30b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a30b0); }
 let mut pc: u32 = 0x10a30b0;
 loop { match pc {
 0x10a30b0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x12510a8u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x12510a4u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a30c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a30c8); }
 let mut pc: u32 = 0x10a30c8;
 loop { match pc {
 0x10a30c8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11690d4u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a30dc); f_0109d4ea(cpu); if cpu.returned_to != 0x10a30dc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a30e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a30e2); }
 let mut pc: u32 = 0x10a30e2;
 loop { match pc {
 0x10a30e2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x10a30fa; continue; }
  pc = 0x10a30f3; continue;
 }
 0x10a30f3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10a3197; continue;
 }
 0x10a30fa => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.edx)), 0x0u8); }
  if !cpu.zf { pc = 0x10a311c; continue; }
  pc = 0x10a3100; continue;
 }
 0x10a3100 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10a3102; continue;
 }
 0x10a3102 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10a311a; continue; }
  pc = 0x10a3108; continue;
 }
 0x10a3108 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3118; continue; }
  pc = 0x10a310c; continue;
 }
 0x10a310c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10a311a; continue; }
  pc = 0x10a3113; continue;
 }
 0x10a3113 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3102; continue; }
  pc = 0x10a3118; continue;
 }
 0x10a3118 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x10a311a; continue;
 }
 0x10a311a => {
  cpu.poll(); pc = 0x10a30f3; continue;
 }
 0x10a311c => {
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x10a3133; continue; }
  pc = 0x10a3128; continue;
 }
 0x10a3128 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x10a3133; continue; }
  pc = 0x10a3130; continue;
 }
 0x10a3130 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
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
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x10a314d; continue;
 }
 0x10a3133 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
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
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x10a314d; continue;
 }
 0x10a314d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if cpu.cf { pc = 0x10a3195; continue; }
  pc = 0x10a3156; continue;
 }
 0x10a3156 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.ecx == 0 { pc = 0x10a3168; continue; }
  pc = 0x10a315d; continue;
 }
 0x10a315d => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x10a3168; continue; }
  pc = 0x10a3165; continue;
 }
 0x10a3165 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a3195; continue; }
  pc = 0x10a3171; continue;
 }
 0x10a3168 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a3195; continue; }
  pc = 0x10a3171; continue;
 }
 0x10a3171 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x10a3187; continue; }
  pc = 0x10a3182; continue;
 }
 0x10a3182 => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  pc = 0x10a3187; continue;
 }
 0x10a3187 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a3190; continue; }
  pc = 0x10a318c; continue;
 }
 0x10a318c => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a3197; continue;
 }
 0x10a3190 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x10a314d; continue;
 }
 0x10a3195 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a3197; continue;
 }
 0x10a3197 => {
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
pub fn f_010a31a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a31a0); }
 let mut pc: u32 = 0x10a31a0;
 loop { match pc {
 0x10a31a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10a31a7; continue;
 }
 0x10a31a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10a31bf; continue; }
  pc = 0x10a31ad; continue;
 }
 0x10a31ad => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x61u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x19u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a31bc; continue; }
  pc = 0x10a31b8; continue;
 }
 0x10a31b8 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a31bc; continue;
 }
 0x10a31bc => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x10a31a7; continue;
 }
 0x10a31bf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a31d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a31d0); }
 let mut pc: u32 = 0x10a31d0;
 loop { match pc {
 0x10a31d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.zf { pc = 0x10a3244; continue; }
  pc = 0x10a31d8; continue;
 }
 0x10a31d8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10a3249; continue; }
  pc = 0x10a31e0; continue;
 }
 0x10a31e0 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a3244; continue; }
  pc = 0x10a31f0; continue;
 }
 0x10a31f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10a3249; continue; }
  pc = 0x10a31fa; continue;
 }
 0x10a31fa => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a3244; continue; }
  pc = 0x10a320a; continue;
 }
 0x10a320a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10a3249; continue; }
  pc = 0x10a3214; continue;
 }
 0x10a3214 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a3244; continue; }
  pc = 0x10a3224; continue;
 }
 0x10a3224 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x10a3249; continue; }
  pc = 0x10a322e; continue;
 }
 0x10a322e => {
  { let v: u32 = cpu.ebx.wrapping_add(0x10u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfefefeffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x80808080u32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x10a31d8; continue; }
  pc = 0x10a3244; continue;
 }
 0x10a3244 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3249 => {
  { cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x10a326a; continue; }
  pc = 0x10a324d; continue;
 }
 0x10a324d => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x10a3244; continue; }
  pc = 0x10a3251; continue;
 }
 0x10a3251 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if !cpu.zf { pc = 0x10a326a; continue; }
  pc = 0x10a3255; continue;
 }
 0x10a3255 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x10a3244; continue; }
  pc = 0x10a325a; continue;
 }
 0x10a325a => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.sub8((cpu.eax as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x10a326a; continue; }
  pc = 0x10a3264; continue;
 }
 0x10a3264 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { cpu.poll(); pc = 0x10a3244; continue; }
  pc = 0x10a3268; continue;
 }
 0x10a3268 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  pc = 0x10a326a; continue;
 }
 0x10a326a => {
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3272(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3272); }
 let mut pc: u32 = 0x10a3272;
 loop { match pc {
 0x10a3272 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x1au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = 0x4eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x10a3286); f_010b2782(cpu); if cpu.returned_to != 0x10a3286 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10a328d); f_010a32ae(cpu); if cpu.returned_to != 0x10a328d { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a328f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a328f); }
 let mut pc: u32 = 0x10a328f;
 loop { match pc {
 0x10a328f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x1au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x10a329b); f_010a32d1(cpu); if cpu.returned_to != 0x10a329b { return; }
  { let v: u8 = 0x4fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  cpu.push(0x10a32a4); f_010b2782(cpu); if cpu.returned_to != 0x10a32a4 { return; }
  cpu.push(0x10a32a9); f_010a32ae(cpu); if cpu.returned_to != 0x10a32a9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a32ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a32ae); }
 let mut pc: u32 = 0x10a32ae;
 loop { match pc {
 0x10a32ae => {
  { cpu.sub8(cpu.r8(0x117ba4au32), 0x9u8); }
  if !cpu.zf { pc = 0x10a32d0; continue; }
  pc = 0x10a32b7; continue;
 }
 0x10a32b7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x2fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10a32d0; continue;
 }
 0x10a32d0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a32d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a32d1); }
 let mut pc: u32 = 0x10a32d1;
 loop { match pc {
 0x10a32d1 => {
  { cpu.sub8(cpu.r8(0x117ba4au32), 0x9u8); }
  if !cpu.zf { pc = 0x10a32f3; continue; }
  pc = 0x10a32da; continue;
 }
 0x10a32da => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = 0x2fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x2bu32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10a32f3; continue;
 }
 0x10a32f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a32f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a32f4); }
 let mut pc: u32 = 0x10a32f4;
 loop { match pc {
 0x10a32f4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a32fa; continue;
 }
 0x10a32fa => {
  { cpu.sub8((cpu.ecx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10a3300; continue; }
  pc = 0x10a32fe; continue;
 }
 0x10a32fe => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x10a3300; continue;
 }
 0x10a3300 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a32fa; continue; }
  pc = 0x10a3309; continue;
 }
 0x10a3309 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a330e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a330e); }
 let mut pc: u32 = 0x10a330e;
 loop { match pc {
 0x10a330e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.sub16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3322(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3322); }
 let mut pc: u32 = 0x10a3322;
 loop { match pc {
 0x10a3322 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xeu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3336(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3336); }
 let mut pc: u32 = 0x10a3336;
 loop { match pc {
 0x10a3336 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x117430eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1174312u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x10a3372u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.cf { pc = 0x10a3367; continue; }
  pc = 0x10a3357; continue;
 }
 0x10a3357 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a3367; continue; }
  pc = 0x10a335b; continue;
 }
 0x10a335b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = 0x24u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x10a336d; continue;
 }
 0x10a3367 => {
  { let v: u8 = 0x24u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x10a336d; continue;
 }
 0x10a336d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x10a3371); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3372(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3372); }
 let mut pc: u32 = 0x10a3372;
 loop { match pc {
 0x10a3372 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.flat_ds(0x10a337d); cpu.push(0x10a3382); f_010af130(cpu); if cpu.returned_to != 0x10a3382 { return; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.esp; cpu.w32(0x1174308u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x117430cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x117ba2cu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1174314u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1251edcu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117ba2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.eax; cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x12522dcu32; cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x2u8); }
  if cpu.cf { pc = 0x10a33d9; continue; }
  pc = 0x10a33d1; continue;
 }
 0x10a33d1 => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10a33d9; continue; }
  pc = 0x10a33d5; continue;
 }
 0x10a33d5 => {
  { let v: u8 = 0x34u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10a33e4; continue;
 }
 0x10a33d9 => {
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x10a33e2; continue; }
  pc = 0x10a33dd; continue;
 }
 0x10a33dd => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10a33e4; continue;
 }
 0x10a33e2 => {
  { let v: u8 = 0x34u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x10a33e4; continue;
 }
 0x10a33e4 => {
  { let v: u32 = (cpu.edx as u16) as u32; cpu.eax = v; }
  { let v: u32 = (cpu.edi as u16) as u32; cpu.edx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  let t = cpu.r32(0x117430eu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10a33f0); cpu.push(cpu.segments[1] as u32); cpu.push(0x10a33f6); cpu.call(t); if cpu.returned_to != 0x10a33f6 { return; }
  { let at = 0x1174308u32.wrapping_add(cpu.bases[3]); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esp = v; cpu.set_segment(2, selector); }
  { let v: u32 = cpu.r32(0x1174314u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117ba2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x10a3412); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3420); }
 let mut pc: u32 = 0x10a3420;
 loop { match pc {
 0x10a3420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x90u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x1174318u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a34b4; continue; }
  pc = 0x10a3441; continue;
 }
 0x10a3441 => {
  cpu.push(0x10a3446); f_010a44a4(cpu); if cpu.returned_to != 0x10a3446 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a34b4; continue; }
  pc = 0x10a344b; continue;
 }
 0x10a344b => {
  { let v: u32 = cpu.r32(0x1254d38u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a3458); f_010a30e2(cpu); if cpu.returned_to != 0x10a3458 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a34b4; continue; }
  pc = 0x10a345e; continue;
 }
 0x10a345e => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1254d3cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3467; continue;
 }
 0x10a3467 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a347f; continue; }
  pc = 0x10a346f; continue;
 }
 0x10a346f => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3467; continue; }
  pc = 0x10a347f; continue;
 }
 0x10a347f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a348d; continue;
 }
 0x10a348d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a34a5; continue; }
  pc = 0x10a3495; continue;
 }
 0x10a3495 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a348d; continue; }
  pc = 0x10a34a5; continue;
 }
 0x10a34a5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x117431au32, v); }
  pc = 0x10a34b4; continue;
 }
 0x10a34b4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  cpu.push(0x10a34c5); f_010a3820(cpu); if cpu.returned_to != 0x10a34c5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a34d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a34d0); }
 let mut pc: u32 = 0x10a34d0;
 loop { match pc {
 0x10a34d0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1174318u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a355f; continue; }
  pc = 0x10a34ec; continue;
 }
 0x10a34ec => {
  cpu.push(0x10a34f1); f_010a44a4(cpu); if cpu.returned_to != 0x10a34f1 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a355f; continue; }
  pc = 0x10a34f6; continue;
 }
 0x10a34f6 => {
  { let v: u32 = cpu.r32(0x1254d38u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a3503); f_010a30e2(cpu); if cpu.returned_to != 0x10a3503 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a355f; continue; }
  pc = 0x10a3509; continue;
 }
 0x10a3509 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1254d3cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3512; continue;
 }
 0x10a3512 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a352a; continue; }
  pc = 0x10a351a; continue;
 }
 0x10a351a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3512; continue; }
  pc = 0x10a352a; continue;
 }
 0x10a352a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a3538; continue;
 }
 0x10a3538 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3550; continue; }
  pc = 0x10a3540; continue;
 }
 0x10a3540 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3538; continue; }
  pc = 0x10a3550; continue;
 }
 0x10a3550 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x117431au32, v); }
  pc = 0x10a355f; continue;
 }
 0x10a355f => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a3568); f_010a3de4(cpu); if cpu.returned_to != 0x10a3568 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3574); }
 let mut pc: u32 = 0x10a3574;
 loop { match pc {
 0x10a3574 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x1174318u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a3603; continue; }
  pc = 0x10a3590; continue;
 }
 0x10a3590 => {
  cpu.push(0x10a3595); f_010a44a4(cpu); if cpu.returned_to != 0x10a3595 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a3603; continue; }
  pc = 0x10a359a; continue;
 }
 0x10a359a => {
  { let v: u32 = cpu.r32(0x1254d38u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a35a7); f_010a30e2(cpu); if cpu.returned_to != 0x10a35a7 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a3603; continue; }
  pc = 0x10a35ad; continue;
 }
 0x10a35ad => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1254d3cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a35b6; continue;
 }
 0x10a35b6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a35ce; continue; }
  pc = 0x10a35be; continue;
 }
 0x10a35be => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a35b6; continue; }
  pc = 0x10a35ce; continue;
 }
 0x10a35ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a35dc; continue;
 }
 0x10a35dc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a35f4; continue; }
  pc = 0x10a35e4; continue;
 }
 0x10a35e4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a35dc; continue; }
  pc = 0x10a35f4; continue;
 }
 0x10a35f4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x117431au32, v); }
  pc = 0x10a3603; continue;
 }
 0x10a3603 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a360c); f_010a40c0(cpu); if cpu.returned_to != 0x10a360c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3618(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3618); }
 let mut pc: u32 = 0x10a3618;
 loop { match pc {
 0x10a3618 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1174318u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a36a6; continue; }
  pc = 0x10a3633; continue;
 }
 0x10a3633 => {
  cpu.push(0x10a3638); f_010a44a4(cpu); if cpu.returned_to != 0x10a3638 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a36a6; continue; }
  pc = 0x10a363d; continue;
 }
 0x10a363d => {
  { let v: u32 = cpu.r32(0x1254d38u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a364a); f_010a30e2(cpu); if cpu.returned_to != 0x10a364a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a36a6; continue; }
  pc = 0x10a3650; continue;
 }
 0x10a3650 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1254d3cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3659; continue;
 }
 0x10a3659 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3671; continue; }
  pc = 0x10a3661; continue;
 }
 0x10a3661 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3659; continue; }
  pc = 0x10a3671; continue;
 }
 0x10a3671 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a367f; continue;
 }
 0x10a367f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3697; continue; }
  pc = 0x10a3687; continue;
 }
 0x10a3687 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a367f; continue; }
  pc = 0x10a3697; continue;
 }
 0x10a3697 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x117431au32, v); }
  pc = 0x10a36a6; continue;
 }
 0x10a36a6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a36ad); f_010a432c(cpu); if cpu.returned_to != 0x10a36ad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
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
pub fn f_010a36bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a36bc); }
 let mut pc: u32 = 0x10a36bc;
 loop { match pc {
 0x10a36bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1174318u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a373e; continue; }
  pc = 0x10a36d7; continue;
 }
 0x10a36d7 => {
  cpu.push(0x10a36dc); f_010a44a4(cpu); if cpu.returned_to != 0x10a36dc { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a373e; continue; }
  pc = 0x10a36e1; continue;
 }
 0x10a36e1 => {
  { let v: u32 = cpu.r32(0x1254d38u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a36ee); f_010a30e2(cpu); if cpu.returned_to != 0x10a36ee { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a373e; continue; }
  pc = 0x10a36f4; continue;
 }
 0x10a36f4 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1254d3cu32); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a36fd; continue;
 }
 0x10a36fd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3715; continue; }
  pc = 0x10a3705; continue;
 }
 0x10a3705 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a36fd; continue; }
  pc = 0x10a3715; continue;
 }
 0x10a3715 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a3723; continue;
 }
 0x10a3723 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a373b; continue; }
  pc = 0x10a372b; continue;
 }
 0x10a372b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3723; continue; }
  pc = 0x10a373b; continue;
 }
 0x10a373b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  pc = 0x10a373e; continue;
 }
 0x10a373e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a3745); f_010a44a4(cpu); if cpu.returned_to != 0x10a3745 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
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
pub fn f_010a3754(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3754); }
 let mut pc: u32 = 0x10a3754;
 loop { match pc {
 0x10a3754 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117433cu32, v); }
  cpu.push(0x10a3768); f_0109b950(cpu); if cpu.returned_to != 0x10a3768 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a37a3; continue; }
  pc = 0x10a376d; continue;
 }
 0x10a376d => {
  { let v: u32 = 0x11690e4u32; cpu.ebx = v; }
  { let v: u32 = 0x1169104u32; cpu.ecx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a379f); f_010a48c8(cpu); if cpu.returned_to != 0x10a379f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a37de; continue;
 }
 0x10a37a3 => {
  { let v: u16 = cpu.r16(0x117431cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a37bc; continue; }
  pc = 0x10a37af; continue;
 }
 0x10a37af => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a37bc => {
  { let v: u32 = 0x1253c70u32; cpu.eax = v; }
  pc = 0x10a37c1; continue;
 }
 0x10a37c1 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x32u16); }
  if cpu.cf { cpu.poll(); pc = 0x10a37c1; continue; }
  pc = 0x10a37d0; continue;
 }
 0x10a37d0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x117431cu32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10a37de; continue;
 }
 0x10a37de => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a37e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a37e8); }
 let mut pc: u32 = 0x10a37e8;
 loop { match pc {
 0x10a37e8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x117431cu32), 0x0u16); }
  if cpu.zf { pc = 0x10a381b; continue; }
  pc = 0x10a37f4; continue;
 }
 0x10a37f4 => {
  { let v: u32 = 0x1253c70u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a37fb; continue;
 }
 0x10a37fb => {
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x10a3808; continue; }
  pc = 0x10a3800; continue;
 }
 0x10a3800 => {
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  cpu.push(0x10a3808); f_010a3a34(cpu); if cpu.returned_to != 0x10a3808 { return; }
  pc = 0x10a3808; continue;
 }
 0x10a3808 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x32u16); }
  if cpu.cf { cpu.poll(); pc = 0x10a37fb; continue; }
  pc = 0x10a3812; continue;
 }
 0x10a3812 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x117431cu32, v); }
  pc = 0x10a381b; continue;
 }
 0x10a381b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3820); }
 let mut pc: u32 = 0x10a3820;
 loop { match pc {
 0x10a3820 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x1253c70u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a3833; continue;
 }
 0x10a3833 => {
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.ebx)), 0x0u16); }
  if cpu.zf { pc = 0x10a3843; continue; }
  pc = 0x10a3839; continue;
 }
 0x10a3839 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub16((cpu.edx as u16), 0x32u16); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a3833; continue; }
  pc = 0x10a3843; continue;
 }
 0x10a3843 => {
  { cpu.sub16((cpu.edx as u16), 0x32u16); }
  if !cpu.zf { pc = 0x10a38db; continue; }
  pc = 0x10a384d; continue;
 }
 0x10a384d => {
  { let v: u32 = 0x1169108u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3864; continue;
 }
 0x10a3864 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a387c; continue; }
  pc = 0x10a386c; continue;
 }
 0x10a386c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3864; continue; }
  pc = 0x10a387c; continue;
 }
 0x10a387c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if cpu.cf { pc = 0x10a38aa; continue; }
  pc = 0x10a38a3; continue;
 }
 0x10a38a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  pc = 0x10a38aa; continue;
 }
 0x10a38aa => {
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  cpu.push(0x10a38cd); f_010a48c8(cpu); if cpu.returned_to != 0x10a38cd { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a38db => {
  { cpu.sub16((cpu.edi as u16), 0x4u16); }
  if cpu.cf { pc = 0x10a38f3; continue; }
  pc = 0x10a38e1; continue;
 }
 0x10a38e1 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3900; continue; }
  pc = 0x10a38e3; continue;
 }
 0x10a38e3 => {
  { cpu.sub16((cpu.edi as u16), 0x8u16); }
  if cpu.cf { pc = 0x10a3913; continue; }
  pc = 0x10a38e9; continue;
 }
 0x10a38e9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10a3907; continue; }
  pc = 0x10a38eb; continue;
 }
 0x10a38eb => {
  { cpu.sub16((cpu.edi as u16), 0x10u16); }
  if cpu.zf { pc = 0x10a390e; continue; }
  pc = 0x10a38f1; continue;
 }
 0x10a38f1 => {
  pc = 0x10a3913; continue;
 }
 0x10a38f3 => {
  { cpu.sub16((cpu.edi as u16), 0x2u16); }
  if !cpu.zf { pc = 0x10a3913; continue; }
  pc = 0x10a38f9; continue;
 }
 0x10a38f9 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x10a3913; continue;
 }
 0x10a3900 => {
  { let v: u32 = 0x261u32; cpu.ecx = v; }
  pc = 0x10a3913; continue;
 }
 0x10a3907 => {
  { let v: u32 = 0x202u32; cpu.ecx = v; }
  pc = 0x10a3913; continue;
 }
 0x10a390e => {
  { let v: u32 = 0x211u32; cpu.ecx = v; }
  pc = 0x10a3913; continue;
 }
 0x10a3913 => {
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10a392c; continue; }
  pc = 0x10a3918; continue;
 }
 0x10a3918 => {
  { let v = 0x1ffu32; cpu.push(v); }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a3927); f_010b2795(cpu); if cpu.returned_to != 0x10a3927 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10a3952; continue;
 }
 0x10a392c => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if cpu.zf { pc = 0x10a3945; continue; }
  pc = 0x10a3931; continue;
 }
 0x10a3931 => {
  { let v = 0x1ffu32; cpu.push(v); }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a3940); f_010b2795(cpu); if cpu.returned_to != 0x10a3940 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10a3952; continue;
 }
 0x10a3945 => {
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a394f); f_010b2795(cpu); if cpu.returned_to != 0x10a394f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10a3952; continue;
 }
 0x10a3952 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a39f7; continue; }
  pc = 0x10a395e; continue;
 }
 0x10a395e => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x1169120u32; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3977; continue;
 }
 0x10a3977 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a398f; continue; }
  pc = 0x10a397f; continue;
 }
 0x10a397f => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3977; continue; }
  pc = 0x10a398f; continue;
 }
 0x10a398f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a39be; continue; }
  pc = 0x10a39b6; continue;
 }
 0x10a39b6 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a39be; continue;
 }
 0x10a39be => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a39d2); f_010b2a07(cpu); if cpu.returned_to != 0x10a39d2 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a39e9); f_010a48c8(cpu); if cpu.returned_to != 0x10a39e9 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a39f7 => {
  { let v: u32 = cpu.edi | 0x1u32; cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1252370u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3a0f; continue;
 }
 0x10a3a0f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3a27; continue; }
  pc = 0x10a3a17; continue;
 }
 0x10a3a17 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3a0f; continue; }
  pc = 0x10a3a27; continue;
 }
 0x10a3a27 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
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
pub fn f_010a3a34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3a34); }
 let mut pc: u32 = 0x10a3a34;
 loop { match pc {
 0x10a3a34 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1253c70u32); cpu.edx = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x1u8); }
  if !cpu.zf { pc = 0x10a3a82; continue; }
  pc = 0x10a3a4b; continue;
 }
 0x10a3a4b => {
  { let v: u32 = 0x116913cu32; cpu.edx = v; }
  { let v: u32 = 0x1169104u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  cpu.push(0x10a3a7e); f_010a48c8(cpu); if cpu.returned_to != 0x10a3a7e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a3a96; continue;
 }
 0x10a3a82 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a3a8c); f_010b2a13(cpu); if cpu.returned_to != 0x10a3a8c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a3a96; continue;
 }
 0x10a3a96 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3aa0); }
 let mut pc: u32 = 0x10a3aa0;
 loop { match pc {
 0x10a3aa0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1253c70u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x10a3af4; continue; }
  pc = 0x10a3abf; continue;
 }
 0x10a3abf => {
  { let v: u32 = 0x1169158u32; cpu.edi = v; }
  { let v: u32 = 0x1169104u32; cpu.ebp = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a3b9a; continue;
 }
 0x10a3af4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a3b00); f_010b2a33(cpu); if cpu.returned_to != 0x10a3b00 { return; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10a3ba4; continue; }
  pc = 0x10a3b08; continue;
 }
 0x10a3b08 => {
  { let v: u32 = 0x1169170u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 7u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1252370u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3b2c; continue;
 }
 0x10a3b2c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3b44; continue; }
  pc = 0x10a3b34; continue;
 }
 0x10a3b34 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3b2c; continue; }
  pc = 0x10a3b44; continue;
 }
 0x10a3b44 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a3b73; continue; }
  pc = 0x10a3b6b; continue;
 }
 0x10a3b6b => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3b73; continue;
 }
 0x10a3b73 => {
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a3b89); f_010b2a07(cpu); if cpu.returned_to != 0x10a3b89 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  pc = 0x10a3b9a; continue;
 }
 0x10a3b9a => {
  cpu.push(0x10a3b9f); f_010a48c8(cpu); if cpu.returned_to != 0x10a3b9f { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10a3ba4; continue;
 }
 0x10a3ba4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3bac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3bac); }
 let mut pc: u32 = 0x10a3bac;
 loop { match pc {
 0x10a3bac => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1253c70u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x10a3bfe; continue; }
  pc = 0x10a3bc9; continue;
 }
 0x10a3bc9 => {
  { let v: u32 = 0x116918cu32; cpu.edi = v; }
  { let v: u32 = 0x1169104u32; cpu.ebp = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a3ca3; continue;
 }
 0x10a3bfe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a3c08); f_010b2b2e(cpu); if cpu.returned_to != 0x10a3c08 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a3cad; continue; }
  pc = 0x10a3c11; continue;
 }
 0x10a3c11 => {
  { let v: u32 = 0x11691a8u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 7u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1252370u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3c35; continue;
 }
 0x10a3c35 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3c4d; continue; }
  pc = 0x10a3c3d; continue;
 }
 0x10a3c3d => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3c35; continue; }
  pc = 0x10a3c4d; continue;
 }
 0x10a3c4d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a3c7c; continue; }
  pc = 0x10a3c74; continue;
 }
 0x10a3c74 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3c7c; continue;
 }
 0x10a3c7c => {
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a3c92); f_010b2a07(cpu); if cpu.returned_to != 0x10a3c92 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  pc = 0x10a3ca3; continue;
 }
 0x10a3ca3 => {
  cpu.push(0x10a3ca8); f_010a48c8(cpu); if cpu.returned_to != 0x10a3ca8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10a3cad; continue;
 }
 0x10a3cad => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3cb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a3cb8); }
 let mut pc: u32 = 0x10a3cb8;
 loop { match pc {
 0x10a3cb8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253c70u32); cpu.edi = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x1u8); }
  if !cpu.zf { pc = 0x10a3d19; continue; }
  pc = 0x10a3cda; continue;
 }
 0x10a3cda => {
  { let v: u32 = 0x11691c4u32; cpu.edi = v; }
  { let v: u32 = 0x1169104u32; cpu.ebp = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a3d12); f_010a48c8(cpu); if cpu.returned_to != 0x10a3d12 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a3dd9; continue;
 }
 0x10a3d19 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1174320u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a3d2f); f_010b2d03(cpu); if cpu.returned_to != 0x10a3d2f { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a3dd4; continue; }
  pc = 0x10a3d38; continue;
 }
 0x10a3d38 => {
  { let v: u32 = 0x11691dcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 7u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1252370u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3d59; continue;
 }
 0x10a3d59 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3d71; continue; }
  pc = 0x10a3d61; continue;
 }
 0x10a3d61 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3d59; continue; }
  pc = 0x10a3d71; continue;
 }
 0x10a3d71 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a3da0; continue; }
  pc = 0x10a3d98; continue;
 }
 0x10a3d98 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3da0; continue;
 }
 0x10a3da0 => {
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.ecx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a3db6); f_010b2a07(cpu); if cpu.returned_to != 0x10a3db6 { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a3dca); f_010a48c8(cpu); if cpu.returned_to != 0x10a3dca { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3dd4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a3dd9; continue;
 }
 0x10a3dd9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a3de4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a3de4); } r_010a3de4(cpu, 0x10a3de4); }
pub fn f_010a408b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a408b); } r_010a3de4(cpu, 0x10a408b); }
fn r_010a3de4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a3de4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x10a3df4); f_010a432c(cpu); if cpu.returned_to != 0x10a3df4 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x40u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a3e92; continue; }
  pc = 0x10a3e05; continue;
 }
 0x10a3e05 => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x11691f8u32; cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3e1e; continue;
 }
 0x10a3e1e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3e36; continue; }
  pc = 0x10a3e26; continue;
 }
 0x10a3e26 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3e1e; continue; }
  pc = 0x10a3e36; continue;
 }
 0x10a3e36 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x117433cu32), 0x20u32); }
  if cpu.cf { pc = 0x10a3e63; continue; }
  pc = 0x10a3e5b; continue;
 }
 0x10a3e5b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3e63; continue;
 }
 0x10a3e63 => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  cpu.push(0x10a3e88); f_010a48c8(cpu); if cpu.returned_to != 0x10a3e88 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3e92 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x10a3f42; continue; }
  pc = 0x10a3e9a; continue;
 }
 0x10a3e9a => {
  { let v: u32 = 0x101u32; cpu.edx = v; }
  cpu.push(0x10a3ea4); f_0109bdf4(cpu); if cpu.returned_to != 0x10a3ea4 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a3f3b; continue; }
  pc = 0x10a3eae; continue;
 }
 0x10a3eae => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x1169220u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3ec7; continue;
 }
 0x10a3ec7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3edf; continue; }
  pc = 0x10a3ecf; continue;
 }
 0x10a3ecf => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3ec7; continue; }
  pc = 0x10a3edf; continue;
 }
 0x10a3edf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.esi, 0x20u32); }
  if cpu.cf { pc = 0x10a3f0e; continue; }
  pc = 0x10a3f06; continue;
 }
 0x10a3f06 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3f0e; continue;
 }
 0x10a3f0e => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a3f31); f_010a48c8(cpu); if cpu.returned_to != 0x10a3f31 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3f3b => {
  { let v: u16 = 0x1u16; cpu.w16(0x40u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a3f42; continue;
 }
 0x10a3f42 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a3f4e); f_010a3820(cpu); if cpu.returned_to != 0x10a3f4e { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x44u32.wrapping_add(cpu.esp), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a3ffb; continue; }
  pc = 0x10a3f5c; continue;
 }
 0x10a3f5c => {
  { let v: u32 = 0x1169244u32; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a3f74; continue;
 }
 0x10a3f74 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a3f8c; continue; }
  pc = 0x10a3f7c; continue;
 }
 0x10a3f7c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a3f74; continue; }
  pc = 0x10a3f8c; continue;
 }
 0x10a3f8c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x117433cu32), 0x20u32); }
  if cpu.cf { pc = 0x10a3fb9; continue; }
  pc = 0x10a3fb1; continue;
 }
 0x10a3fb1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a3fb9; continue;
 }
 0x10a3fb9 => {
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a3fe3); f_010a48c8(cpu); if cpu.returned_to != 0x10a3fe3 { return; }
  { cpu.and8(cpu.r8(0x40u32.wrapping_add(cpu.esp)), 0x1u8); }
  if cpu.zf { pc = 0x10a3ff1; continue; }
  pc = 0x10a3fea; continue;
 }
 0x10a3fea => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10a3ff1); f_0109c4a8(cpu); if cpu.returned_to != 0x10a3ff1 { return; }
  pc = 0x10a3ff1; continue;
 }
 0x10a3ff1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a3ffb => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4007); f_010a3aa0(cpu); if cpu.returned_to != 0x10a4007 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a40ac; continue; }
  pc = 0x10a400f; continue;
 }
 0x10a400f => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x1169268u32; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a4027; continue;
 }
 0x10a4027 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a403f; continue; }
  pc = 0x10a402f; continue;
 }
 0x10a402f => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4027; continue; }
  pc = 0x10a403f; continue;
 }
 0x10a403f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a406e; continue; }
  pc = 0x10a4066; continue;
 }
 0x10a4066 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a406e; continue;
 }
 0x10a406e => {
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a4096); f_010a48c8(cpu); if cpu.returned_to != 0x10a4096 { return; }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a40a2); f_010a3a34(cpu); if cpu.returned_to != 0x10a40a2 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a408b => {
  { let v: u32 = cpu.add32(cpu.r32(0xffffff89u32.wrapping_add(cpu.esi)), cpu.esp); cpu.w32(0xffffff89u32.wrapping_add(cpu.esi), v); }
  if cpu.zf { pc = 0x10a40b4; continue; }
  pc = 0x10a4090; continue;
 }
 0x10a4090 => {
  { let v: u8 = (cpu.eax as u8) | 0xe8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a40a2); f_010a3a34(cpu); if cpu.returned_to != 0x10a40a2 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a40ac => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a40b3); f_010a3a34(cpu); if cpu.returned_to != 0x10a40b3 { return; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a40b4 => {
  cpu.push(0x10a40b9); cpu.call(0x5e53053c); if cpu.returned_to != 0x10a40b9 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a40c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a40c0); } r_010a40c0(cpu, 0x10a40c0); }
pub fn f_010a40f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a40f6); } r_010a40c0(cpu, 0x10a40f6); }
fn r_010a40c0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a40c0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  cpu.push(0x10a40d2); f_010a3820(cpu); if cpu.returned_to != 0x10a40d2 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a40f3; continue; }
  pc = 0x10a40d9; continue;
 }
 0x10a40d9 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169288u32; cpu.push(v); }
  { let v = 0x11692b0u32; cpu.push(v); }
  cpu.push(0x10a40e9); f_010a4b18(cpu); if cpu.returned_to != 0x10a40e9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a40f3 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.esi = v; }
  pc = 0x10a40f6; continue;
 }
 0x10a40f6 => {
  { let v: u32 = cpu.r32(0x1253c70u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a4105); f_010a4b94(cpu); if cpu.returned_to != 0x10a4105 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a412d; continue; }
  pc = 0x10a410c; continue;
 }
 0x10a410c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11692b8u32; cpu.push(v); }
  { let v = 0x11692b0u32; cpu.push(v); }
  cpu.push(0x10a411c); f_010a4b18(cpu); if cpu.returned_to != 0x10a411c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a4126); f_010a3a34(cpu); if cpu.returned_to != 0x10a4126 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a412d => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a415c; continue; }
  pc = 0x10a4132; continue;
 }
 0x10a4132 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x10a415c; continue; }
  pc = 0x10a4136; continue;
 }
 0x10a4136 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11692e4u32; cpu.push(v); }
  { let v = 0x11692b0u32; cpu.push(v); }
  cpu.push(0x10a4148); f_010a4b18(cpu); if cpu.returned_to != 0x10a4148 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a4152); f_010a3a34(cpu); if cpu.returned_to != 0x10a4152 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a415c => {
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a416a); f_010a3aa0(cpu); if cpu.returned_to != 0x10a416a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a418f; continue; }
  pc = 0x10a416e; continue;
 }
 0x10a416e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169318u32; cpu.push(v); }
  { let v = 0x11692b0u32; cpu.push(v); }
  cpu.push(0x10a417e); f_010a4b18(cpu); if cpu.returned_to != 0x10a417e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a4188); f_010a3a34(cpu); if cpu.returned_to != 0x10a4188 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a418f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a4196); f_010a3a34(cpu); if cpu.returned_to != 0x10a4196 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a41a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a41a0); }
 let mut pc: u32 = 0x10a41a0;
 loop { match pc {
 0x10a41a0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  cpu.push(0x10a41b5); f_010a3820(cpu); if cpu.returned_to != 0x10a41b5 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a4255; continue; }
  pc = 0x10a41c3; continue;
 }
 0x10a41c3 => {
  { let v: u32 = 0x1169340u32; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a41db; continue;
 }
 0x10a41db => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a41f3; continue; }
  pc = 0x10a41e3; continue;
 }
 0x10a41e3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a41db; continue; }
  pc = 0x10a41f3; continue;
 }
 0x10a41f3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if cpu.cf { pc = 0x10a4221; continue; }
  pc = 0x10a421a; continue;
 }
 0x10a421a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  pc = 0x10a4221; continue;
 }
 0x10a4221 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a424b); f_010a48c8(cpu); if cpu.returned_to != 0x10a424b { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4255 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4261); f_010a3bac(cpu); if cpu.returned_to != 0x10a4261 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a4306; continue; }
  pc = 0x10a4269; continue;
 }
 0x10a4269 => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x1169360u32; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a4281; continue;
 }
 0x10a4281 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4299; continue; }
  pc = 0x10a4289; continue;
 }
 0x10a4289 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4281; continue; }
  pc = 0x10a4299; continue;
 }
 0x10a4299 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a42c8; continue; }
  pc = 0x10a42c0; continue;
 }
 0x10a42c0 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a42c8; continue;
 }
 0x10a42c8 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a42f0); f_010a48c8(cpu); if cpu.returned_to != 0x10a42f0 { return; }
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a42fc); f_010a3a34(cpu); if cpu.returned_to != 0x10a42fc { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4306 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a430d); f_010a3a34(cpu); if cpu.returned_to != 0x10a430d { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a431c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a431c); }
 let mut pc: u32 = 0x10a431c;
 loop { match pc {
 0x10a431c => {
  cpu.push(0x10a4321); f_010b2d46(cpu); if cpu.returned_to != 0x10a4321 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a432c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a432c); }
 let mut pc: u32 = 0x10a432c;
 loop { match pc {
 0x10a432c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  cpu.push(0x10a4341); f_010a3820(cpu); if cpu.returned_to != 0x10a4341 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a43dc; continue; }
  pc = 0x10a434c; continue;
 }
 0x10a434c => {
  { let v: u32 = 0x1169384u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a4363; continue;
 }
 0x10a4363 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a437b; continue; }
  pc = 0x10a436b; continue;
 }
 0x10a436b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4363; continue; }
  pc = 0x10a437b; continue;
 }
 0x10a437b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if cpu.cf { pc = 0x10a43a9; continue; }
  pc = 0x10a43a2; continue;
 }
 0x10a43a2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  pc = 0x10a43a9; continue;
 }
 0x10a43a9 => {
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a43cd); f_010a48c8(cpu); if cpu.returned_to != 0x10a43cd { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a43dc => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1253c70u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a43ee); f_010a4b94(cpu); if cpu.returned_to != 0x10a43ee { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a448e; continue; }
  pc = 0x10a43f9; continue;
 }
 0x10a43f9 => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x11693a4u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a4412; continue;
 }
 0x10a4412 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a442a; continue; }
  pc = 0x10a441a; continue;
 }
 0x10a441a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4412; continue; }
  pc = 0x10a442a; continue;
 }
 0x10a442a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.cf { pc = 0x10a4459; continue; }
  pc = 0x10a4451; continue;
 }
 0x10a4451 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a4459; continue;
 }
 0x10a4459 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  cpu.push(0x10a447f); f_010a48c8(cpu); if cpu.returned_to != 0x10a447f { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a448e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4495); f_010a3a34(cpu); if cpu.returned_to != 0x10a4495 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
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
pub fn f_010a44a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a44a4); }
 let mut pc: u32 = 0x10a44a4;
 loop { match pc {
 0x10a44a4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = 0x200u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x10a44be); f_010b2795(cpu); if cpu.returned_to != 0x10a44be { return; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a4571; continue; }
  pc = 0x10a44cb; continue;
 }
 0x10a44cb => {
  cpu.push(0x10a44d0); f_010b2a07(cpu); if cpu.returned_to != 0x10a44d0 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x10a44dc; continue; }
  pc = 0x10a44d5; continue;
 }
 0x10a44d5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a4576; continue;
 }
 0x10a44dc => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x11693c8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a44f4; continue;
 }
 0x10a44f4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a450c; continue; }
  pc = 0x10a44fc; continue;
 }
 0x10a44fc => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a44f4; continue; }
  pc = 0x10a450c; continue;
 }
 0x10a450c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.cf { pc = 0x10a453b; continue; }
  pc = 0x10a4533; continue;
 }
 0x10a4533 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117433cu32, v); }
  pc = 0x10a453b; continue;
 }
 0x10a453b => {
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a454d); f_010b2a07(cpu); if cpu.returned_to != 0x10a454d { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a4564); f_010a48c8(cpu); if cpu.returned_to != 0x10a4564 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4571 => {
  cpu.push(0x10a4576); f_010b2a13(cpu); if cpu.returned_to != 0x10a4576 { return; }
  pc = 0x10a4576; continue;
 }
 0x10a4576 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
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
pub fn f_010a4584(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4584); }
 let mut pc: u32 = 0x10a4584;
 loop { match pc {
 0x10a4584 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x90u32; cpu.edx = v; }
  { let v: u32 = 0x12522e0u32; cpu.eax = v; }
  cpu.push(0x10a459a); f_010b2d4b(cpu); if cpu.returned_to != 0x10a459a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4613; continue; }
  pc = 0x10a459e; continue;
 }
 0x10a459e => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1169408u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1253d38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.eax = v; }
  { let v: u32 = 0x11693e4u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117433cu32, v); }
  { let v: u32 = 0x1253d38u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.cf { pc = 0x10a45e8; continue; }
  pc = 0x10a45e0; continue;
 }
 0x10a45e0 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a45e8; continue;
 }
 0x10a45e8 => {
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a45fa); f_010b2a07(cpu); if cpu.returned_to != 0x10a45fa { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a4611); f_010a48c8(cpu); if cpu.returned_to != 0x10a4611 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a4613; continue;
 }
 0x10a4613 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a461c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a461c); }
 let mut pc: u32 = 0x10a461c;
 loop { match pc {
 0x10a461c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10a4652; continue; }
  pc = 0x10a4637; continue;
 }
 0x10a4637 => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi)), 0x3au8); }
  if !cpu.zf { pc = 0x10a4652; continue; }
  pc = 0x10a463d; continue;
 }
 0x10a463d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10a464a); f_010a4864(cpu); if cpu.returned_to != 0x10a464a { return; }
  { let v: u32 = cpu.eax.wrapping_sub(0x40u32); cpu.eax = v; }
  cpu.push(0x10a4652); f_010a3322(cpu); if cpu.returned_to != 0x10a4652 { return; }
  pc = 0x10a4652; continue;
 }
 0x10a4652 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a4659); f_010b2dd3(cpu); if cpu.returned_to != 0x10a4659 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a46f1; continue; }
  pc = 0x10a4662; continue;
 }
 0x10a4662 => {
  { let v: u32 = cpu.r32(0x117433cu32); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = 0x116940cu32; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1253d38u32); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a467a; continue;
 }
 0x10a467a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4692; continue; }
  pc = 0x10a4682; continue;
 }
 0x10a4682 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a467a; continue; }
  pc = 0x10a4692; continue;
 }
 0x10a4692 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117433cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x1253d38u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.cf { pc = 0x10a46bd; continue; }
  pc = 0x10a46b5; continue;
 }
 0x10a46b5 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117433cu32, v); }
  pc = 0x10a46bd; continue;
 }
 0x10a46bd => {
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a46cf); f_010b2a07(cpu); if cpu.returned_to != 0x10a46cf { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a46e6); f_010a48c8(cpu); if cpu.returned_to != 0x10a46e6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a46f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
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
pub fn f_010a4700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4700); }
 let mut pc: u32 = 0x10a4700;
 loop { match pc {
 0x10a4700 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x20u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x1253c70u32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); }
  if !cpu.zf { pc = 0x10a4757; continue; }
  pc = 0x10a4718; continue;
 }
 0x10a4718 => {
  { let v: u32 = 0x1169430u32; cpu.esi = v; }
  { let v: u32 = 0x1169104u32; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a473c); f_010b2a07(cpu); if cpu.returned_to != 0x10a473c { return; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a4753); f_010a48c8(cpu); if cpu.returned_to != 0x10a4753 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a47a9; continue;
 }
 0x10a4757 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a4761); f_010b2df4(cpu); if cpu.returned_to != 0x10a4761 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a47a7; continue; }
  pc = 0x10a4769; continue;
 }
 0x10a4769 => {
  { let v: u32 = 0x1169448u32; cpu.ebx = v; }
  { let v: u32 = 0x1169104u32; cpu.ecx = v; }
  { let v: u32 = (cpu.edx as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10a478c); f_010b2a07(cpu); if cpu.returned_to != 0x10a478c { return; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  cpu.push(0x10a47a7); f_010a48c8(cpu); if cpu.returned_to != 0x10a47a7 { return; }
  pc = 0x10a47a7; continue;
 }
 0x10a47a7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10a47a9; continue;
 }
 0x10a47a9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x20u32); cpu.esp = v; }
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
pub fn f_010a47b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a47b4); }
 let mut pc: u32 = 0x10a47b4;
 loop { match pc {
 0x10a47b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1253c70u32); cpu.ecx = v; }
  { cpu.and8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if !cpu.zf { pc = 0x10a4802; continue; }
  pc = 0x10a47cb; continue;
 }
 0x10a47cb => {
  { let v: u32 = 0x1169468u32; cpu.edx = v; }
  { let v: u32 = 0x1169104u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xeu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x10a4814u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117432cu32; cpu.edx = v; }
  cpu.push(0x10a47fe); f_010a48c8(cpu); if cpu.returned_to != 0x10a47fe { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a480c; continue;
 }
 0x10a4802 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a480c); f_010b2e21(cpu); if cpu.returned_to != 0x10a480c { return; }
  pc = 0x10a480c; continue;
 }
 0x10a480c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4814(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4814); }
 let mut pc: u32 = 0x10a4814;
 loop { match pc {
 0x10a4814 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x116948cu32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4834); f_0109d4ea(cpu); if cpu.returned_to != 0x10a4834 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a483b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a483b); }
 let mut pc: u32 = 0x10a483b;
 loop { match pc {
 0x10a483b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a484e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a484e); }
 let mut pc: u32 = 0x10a484e;
 loop { match pc {
 0x10a484e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x10a483bu32; cpu.ecx = v; }
  cpu.push(0x10a485c); f_010b04be(cpu); if cpu.returned_to != 0x10a485c { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4864(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4864); }
 let mut pc: u32 = 0x10a4864;
 loop { match pc {
 0x10a4864 => {
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a4871; continue; }
  pc = 0x10a4869; continue;
 }
 0x10a4869 => {
  { cpu.sub32(cpu.eax, 0x7au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a4871; continue; }
  pc = 0x10a486e; continue;
 }
 0x10a486e => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x10a4871; continue;
 }
 0x10a4871 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4872(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4872); }
 let mut pc: u32 = 0x10a4872;
 loop { match pc {
 0x10a4872 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x117b22au32; cpu.eax = v; }
  cpu.push(0x10a488e); f_010b2e43(cpu); if cpu.returned_to != 0x10a488e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a48a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a48a0); }
 let mut pc: u32 = 0x10a48a0;
 loop { match pc {
 0x10a48a0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1255178u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x125517cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a48b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a48b4); }
 let mut pc: u32 = 0x10a48b4;
 loop { match pc {
 0x10a48b4 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x125517cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a48c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a48c8); }
 let mut pc: u32 = 0x10a48c8;
 loop { match pc {
 0x10a48c8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x125517cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0xau16); }
  if !cpu.zf { pc = 0x10a491a; continue; }
  pc = 0x10a48de; continue;
 }
 0x10a48de => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x1174350u32; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x1174340u32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x125517cu32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x10a4b84u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1174350u32; cpu.ecx = v; }
  cpu.push(0x10a4911); f_010a48c8(cpu); if cpu.returned_to != 0x10a4911 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a491a => {
  { cpu.sub16((cpu.ebx as u16), 0x50u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10a4952; continue; }
  pc = 0x10a4920; continue;
 }
 0x10a4920 => {
  { let v: u32 = 0x117435bu32; cpu.edi = v; }
  { let v: u32 = 0x1174340u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10a4b84u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x117435bu32; cpu.ecx = v; }
  cpu.push(0x10a4949); f_010a48c8(cpu); if cpu.returned_to != 0x10a4949 { return; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4952 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x1254d40u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edx), v); }
  { cpu.and16((cpu.ebx as u16), (cpu.ebx as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x10a498b; continue; }
  pc = 0x10a497e; continue;
 }
 0x10a497e => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edx), v); }
  { cpu.sub16((cpu.eax as u16), (cpu.edi as u16)); }
  if cpu.cf { cpu.poll(); pc = 0x10a497e; continue; }
  pc = 0x10a498b; continue;
 }
 0x10a498b => {
  { let v: u16 = cpu.r16(0x125517cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x125517cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a49a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a49a0); }
 let mut pc: u32 = 0x10a49a0;
 loop { match pc {
 0x10a49a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub16(cpu.r16(0x125517cu32), 0x0u16); }
  if cpu.zf { pc = 0x10a4ae7; continue; }
  pc = 0x10a49b9; continue;
 }
 0x10a49b9 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10a49d5; continue; }
  pc = 0x10a49bd; continue;
 }
 0x10a49bd => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x10a49d5; continue; }
  pc = 0x10a49c1; continue;
 }
 0x10a49c1 => {
  { cpu.sub32(cpu.r32(0x1255178u32), 0x0u32); }
  if cpu.zf { pc = 0x10a49d5; continue; }
  pc = 0x10a49ca; continue;
 }
 0x10a49ca => {
  { let v: u32 = 0x11694bcu32; cpu.eax = v; }
  let t = cpu.r32(0x1255178u32); cpu.push(0x10a49d5); cpu.call(t); if cpu.returned_to != 0x10a49d5 { return; }
  pc = 0x10a49d5; continue;
 }
 0x10a49d5 => {
  { let v: u32 = 0x1254d40u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x125517cu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  if (cpu.cf || cpu.zf) { pc = 0x10a4ac0; continue; }
  pc = 0x10a49f0; continue;
 }
 0x10a49f0 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  pc = 0x10a49f3; continue;
 }
 0x10a49f3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x12550b0u32, v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x10a4a2d; continue; }
  pc = 0x10a4a05; continue;
 }
 0x10a4a05 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a4a2d; continue; }
  pc = 0x10a4a09; continue;
 }
 0x10a4a09 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11694e8u32; cpu.push(v); }
  { let v = 0x12550b0u32; cpu.push(v); }
  cpu.push(0x10a4a19); f_0109d4ea(cpu); if cpu.returned_to != 0x10a4a19 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x12550b0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x10a4a2d; continue;
 }
 0x10a4a2d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10a4a5c; continue; }
  pc = 0x10a4a38; continue;
 }
 0x10a4a38 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a4a5c; continue; }
  pc = 0x10a4a3c; continue;
 }
 0x10a4a3c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x12550b0u32); cpu.eax = v; }
  { let v: u32 = 0x12550b0u32; cpu.edi = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x10a4a50); cpu.call(t); if cpu.returned_to != 0x10a4a50 { return; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x10a4a5c; continue;
 }
 0x10a4a5c => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10a4a88; continue; }
  pc = 0x10a4a61; continue;
 }
 0x10a4a61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x11694f0u32; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x12550b0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4a76); f_0109d4ea(cpu); if cpu.returned_to != 0x10a4a76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = 0x12550b0u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10a4a88; continue;
 }
 0x10a4a88 => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  if cpu.zf { pc = 0x10a4aa1; continue; }
  pc = 0x10a4a8d; continue;
 }
 0x10a4a8d => {
  { cpu.sub32(cpu.r32(0x1255178u32), 0x0u32); }
  if cpu.zf { pc = 0x10a4aa1; continue; }
  pc = 0x10a4a96; continue;
 }
 0x10a4a96 => {
  { let v: u32 = 0x12550b0u32; cpu.eax = v; }
  let t = cpu.r32(0x1255178u32); cpu.push(0x10a4aa1); cpu.call(t); if cpu.returned_to != 0x10a4aa1 { return; }
  pc = 0x10a4aa1; continue;
 }
 0x10a4aa1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x125517cu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x58u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x58u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub16((cpu.edi as u16), (cpu.edx as u16)); }
  if cpu.cf { cpu.poll(); pc = 0x10a49f3; continue; }
  pc = 0x10a4ac0; continue;
 }
 0x10a4ac0 => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  if cpu.zf { pc = 0x10a4ade; continue; }
  pc = 0x10a4ac5; continue;
 }
 0x10a4ac5 => {
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10a4ade; continue; }
  pc = 0x10a4aca; continue;
 }
 0x10a4aca => {
  { cpu.sub32(cpu.r32(0x1255178u32), 0x0u32); }
  if cpu.zf { pc = 0x10a4ade; continue; }
  pc = 0x10a4ad3; continue;
 }
 0x10a4ad3 => {
  { let v: u32 = 0x11694f4u32; cpu.eax = v; }
  let t = cpu.r32(0x1255178u32); cpu.push(0x10a4ade); cpu.call(t); if cpu.returned_to != 0x10a4ade { return; }
  pc = 0x10a4ade; continue;
 }
 0x10a4ade => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x125517cu32, v); }
  pc = 0x10a4ae7; continue;
 }
 0x10a4ae7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010a4af4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4af4); }
 let mut pc: u32 = 0x10a4af4;
 loop { match pc {
 0x10a4af4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a4afb; continue;
 }
 0x10a4afb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4b13; continue; }
  pc = 0x10a4b03; continue;
 }
 0x10a4b03 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4afb; continue; }
  pc = 0x10a4b13; continue;
 }
 0x10a4b13 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4b18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4b18); }
 let mut pc: u32 = 0x10a4b18;
 loop { match pc {
 0x10a4b18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xd0u32); cpu.esp = v; }
  { let v: u32 = 0xecu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0xccu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xe8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xccu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x10a4b47); f_010a484e(cpu); if cpu.returned_to != 0x10a4b47 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xccu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x4fu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xe4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x10a4af4u32; cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x10a4b78); f_010a48c8(cpu); if cpu.returned_to != 0x10a4b78 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xd0u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4b84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4b84); }
 let mut pc: u32 = 0x10a4b84;
 loop { match pc {
 0x10a4b84 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1169520u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a4b90); f_0109d4ea(cpu); if cpu.returned_to != 0x10a4b90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4b94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4b94); }
 let mut pc: u32 = 0x10a4b94;
 loop { match pc {
 0x10a4b94 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10a4ba7); f_010b2d03(cpu); if cpu.returned_to != 0x10a4ba7 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a4bcb; continue; }
  pc = 0x10a4bae; continue;
 }
 0x10a4bae => {
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10a4bbc); f_010b2d03(cpu); if cpu.returned_to != 0x10a4bbc { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4bc9); f_010b2d03(cpu); if cpu.returned_to != 0x10a4bc9 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10a4bcb; continue;
 }
 0x10a4bcb => {
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
pub fn f_010a4bd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4bd1); }
 let mut pc: u32 = 0x10a4bd1;
 loop { match pc {
 0x10a4bd1 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x10a4bd3; continue;
 }
 0x10a4bd3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4bed; continue; }
  pc = 0x10a4bdc; continue;
 }
 0x10a4bdc => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4bd3; continue; }
  pc = 0x10a4bed; continue;
 }
 0x10a4bed => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4bf0); }
 let mut pc: u32 = 0x10a4bf0;
 loop { match pc {
 0x10a4bf0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4c03; continue;
 }
 0x10a4c03 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4c1b; continue; }
  pc = 0x10a4c0b; continue;
 }
 0x10a4c0b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4c03; continue; }
  pc = 0x10a4c1b; continue;
 }
 0x10a4c1b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.flat_ds(0x10a4c22); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4c24); }
 let mut pc: u32 = 0x10a4c24;
 loop { match pc {
 0x10a4c24 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
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
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4c3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4c3d); }
 let mut pc: u32 = 0x10a4c3d;
 loop { match pc {
 0x10a4c3d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x10a4c40; continue;
 }
 0x10a4c40 => {
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10a4c4f; continue; }
  pc = 0x10a4c44; continue;
 }
 0x10a4c44 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4c40; continue; }
  pc = 0x10a4c4d; continue;
 }
 0x10a4c4d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a4c4f; continue;
 }
 0x10a4c4f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4c51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4c51); }
 let mut pc: u32 = 0x10a4c51;
 loop { match pc {
 0x10a4c51 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a4c57; continue; }
  pc = 0x10a4c55; continue;
 }
 0x10a4c55 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10a4c57; continue;
 }
 0x10a4c57 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4c60); }
 let mut pc: u32 = 0x10a4c60;
 loop { match pc {
 0x10a4c60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1177b6cu32.wrapping_add(cpu.edx.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x10a4c94; continue; }
  pc = 0x10a4c70; continue;
 }
 0x10a4c70 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169534u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  cpu.push(0x10a4c91); f_010a4b18(cpu); if cpu.returned_to != 0x10a4c91 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x10a4c94; continue;
 }
 0x10a4c94 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x1177b6cu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a4ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4ca0); }
 let mut pc: u32 = 0x10a4ca0;
 loop { match pc {
 0x10a4ca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x14u32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10a4cba; continue; }
  pc = 0x10a4cb3; continue;
 }
 0x10a4cb3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a50b9; continue;
 }
 0x10a4cba => {
  { cpu.sub32(cpu.r32(0x1174388u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a4d0f; continue; }
  pc = 0x10a4cc3; continue;
 }
 0x10a4cc3 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x7ef4u32; cpu.eax = v; }
  cpu.push(0x10a4cd2); f_0109bdf4(cpu); if cpu.returned_to != 0x10a4cd2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1174388u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4d0f; continue; }
  pc = 0x10a4cdb; continue;
 }
 0x10a4cdb => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x116957cu32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  cpu.push(0x10a4cfd); f_010a4b18(cpu); if cpu.returned_to != 0x10a4cfd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4d0f => {
  { let v = 0x11695bcu32; cpu.push(v); }
  { let v: u32 = 0x11695c0u32; cpu.ecx = v; }
  { let v: u32 = 0x11695bcu32; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x14u32); cpu.esi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10a4d2c); f_010b2ed6(cpu); if cpu.returned_to != 0x10a4d2c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11755d8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10a4d3b); f_010a34d0(cpu); if cpu.returned_to != 0x10a4d3b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x11755d8u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a4dc8; continue; }
  pc = 0x10a4d48; continue;
 }
 0x10a4d48 => {
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11695d0u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4d6b; continue;
 }
 0x10a4d6b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4d83; continue; }
  pc = 0x10a4d73; continue;
 }
 0x10a4d73 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4d6b; continue; }
  pc = 0x10a4d83; continue;
 }
 0x10a4d83 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4d92; continue;
 }
 0x10a4d92 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4daa; continue; }
  pc = 0x10a4d9a; continue;
 }
 0x10a4d9a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4d92; continue; }
  pc = 0x10a4daa; continue;
 }
 0x10a4daa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a4db6); f_010a4b18(cpu); if cpu.returned_to != 0x10a4db6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4dc8 => {
  { let v = 0x11695bcu32; cpu.push(v); }
  { let v: u32 = 0x11695fcu32; cpu.ecx = v; }
  { let v: u32 = 0x11695bcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10a4de2); f_010b2ed6(cpu); if cpu.returned_to != 0x10a4de2 { return; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a4dee); f_010a3420(cpu); if cpu.returned_to != 0x10a4dee { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x118u32.wrapping_add(cpu.esp), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a4e80; continue; }
  pc = 0x10a4dff; continue;
 }
 0x10a4dff => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x116960cu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4e23; continue;
 }
 0x10a4e23 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4e3b; continue; }
  pc = 0x10a4e2b; continue;
 }
 0x10a4e2b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4e23; continue; }
  pc = 0x10a4e3b; continue;
 }
 0x10a4e3b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4e4a; continue;
 }
 0x10a4e4a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4e62; continue; }
  pc = 0x10a4e52; continue;
 }
 0x10a4e52 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4e4a; continue; }
  pc = 0x10a4e62; continue;
 }
 0x10a4e62 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a4e6e); f_010a4b18(cpu); if cpu.returned_to != 0x10a4e6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4e80 => {
  { let v: u32 = 0x12u32; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a4e87; continue;
 }
 0x10a4e87 => {
  { let v: u32 = cpu.r32(0x116u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4e98); f_010a4700(cpu); if cpu.returned_to != 0x10a4e98 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a4f3f; continue; }
  pc = 0x10a4ea1; continue;
 }
 0x10a4ea1 => {
  { let v: u32 = 0x104u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a4eb1); f_010a3aa0(cpu); if cpu.returned_to != 0x10a4eb1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a4f39; continue; }
  pc = 0x10a4eb9; continue;
 }
 0x10a4eb9 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x116962cu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4edc; continue;
 }
 0x10a4edc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4ef4; continue; }
  pc = 0x10a4ee4; continue;
 }
 0x10a4ee4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4edc; continue; }
  pc = 0x10a4ef4; continue;
 }
 0x10a4ef4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4f03; continue;
 }
 0x10a4f03 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a4f1b; continue; }
  pc = 0x10a4f0b; continue;
 }
 0x10a4f0b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4f03; continue; }
  pc = 0x10a4f1b; continue;
 }
 0x10a4f1b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a4f27); f_010a4b18(cpu); if cpu.returned_to != 0x10a4f27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4f39 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a4e87; continue;
 }
 0x10a4f3f => {
  { let v: u32 = cpu.esi; cpu.w32(0x1174384u32, v); }
  { cpu.sub32(cpu.esi, 0x9c4u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a4f81; continue; }
  pc = 0x10a4f4d; continue;
 }
 0x10a4f4d => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169650u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  cpu.push(0x10a4f6f); f_010a4b18(cpu); if cpu.returned_to != 0x10a4f6f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a4f81 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10a4f8f); f_010a3cb8(cpu); if cpu.returned_to != 0x10a4f8f { return; }
  { let v: u32 = 0x2710u32; cpu.ebx = v; }
  { let v: u32 = 0x1177b6cu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10a4fa0); f_010afeb0(cpu); if cpu.returned_to != 0x10a4fa0 { return; }
  { let v: u32 = cpu.r32(0x1174384u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a50a5; continue; }
  pc = 0x10a4fb0; continue;
 }
 0x10a4fb0 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x10a4fb2; continue;
 }
 0x10a4fb2 => {
  { let v: u32 = cpu.r32(0x116u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x104u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a4fcd); f_010a3aa0(cpu); if cpu.returned_to != 0x10a4fcd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a5055; continue; }
  pc = 0x10a4fd5; continue;
 }
 0x10a4fd5 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169680u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a4ff8; continue;
 }
 0x10a4ff8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5010; continue; }
  pc = 0x10a5000; continue;
 }
 0x10a5000 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a4ff8; continue; }
  pc = 0x10a5010; continue;
 }
 0x10a5010 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a501f; continue;
 }
 0x10a501f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5037; continue; }
  pc = 0x10a5027; continue;
 }
 0x10a5027 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a501f; continue; }
  pc = 0x10a5037; continue;
 }
 0x10a5037 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a5043); f_010a4b18(cpu); if cpu.returned_to != 0x10a5043 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5055 => {
  { let v: u32 = cpu.r32(0x112u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11755d8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x10a506a); f_010a4c60(cpu); if cpu.returned_to != 0x10a506a { return; }
  { let v: u32 = cpu.r32(0x1174388u32); cpu.edi = v; }
  { let v: u32 = 0x104u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a507b; continue;
 }
 0x10a507b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5093; continue; }
  pc = 0x10a5083; continue;
 }
 0x10a5083 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a507b; continue; }
  pc = 0x10a5093; continue;
 }
 0x10a5093 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1174384u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xdu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.edi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a4fb2; continue; }
  pc = 0x10a50a5; continue;
 }
 0x10a50a5 => {
  { let v: u32 = cpu.r32(0x116u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a50b4); f_010a3a34(cpu); if cpu.returned_to != 0x10a50b4 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a50b9; continue;
 }
 0x10a50b9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x11cu32); cpu.esp = v; }
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
pub fn f_010a4e8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a4e8b); }
 let mut pc: u32 = 0x10a4e8b;
 loop { match pc {
 0x10a4e8b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.eax as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.cf = true; }
  { let v: u8 = cpu.adc8(cpu.r8(0xf868e8c8u32.wrapping_add(cpu.ecx)), (cpu.ecx as u8)); cpu.w8(0xf868e8c8u32.wrapping_add(cpu.ecx), v); }
  cpu.call(0x10a4e96); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a50c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a50c8); }
 let mut pc: u32 = 0x10a50c8;
 loop { match pc {
 0x10a50c8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x104u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x1174390u32), 0x0u16); }
  if !cpu.zf { pc = 0x10a524a; continue; }
  pc = 0x10a50e1; continue;
 }
 0x10a50e1 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v = 0x11695bcu32; cpu.push(v); }
  { let v: u32 = 0x11696a4u32; cpu.ecx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = 0x11695bcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1174390u32, v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10a510d); f_010b2ed6(cpu); if cpu.returned_to != 0x10a510d { return; }
  { let v: u32 = 0x125fba0u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10a511b); f_010a34d0(cpu); if cpu.returned_to != 0x10a511b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5198; continue; }
  pc = 0x10a511f; continue;
 }
 0x10a511f => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11696b0u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a5143; continue;
 }
 0x10a5143 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a515b; continue; }
  pc = 0x10a514b; continue;
 }
 0x10a514b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a5143; continue; }
  pc = 0x10a515b; continue;
 }
 0x10a515b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a516a; continue;
 }
 0x10a516a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5182; continue; }
  pc = 0x10a5172; continue;
 }
 0x10a5172 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a516a; continue; }
  pc = 0x10a5182; continue;
 }
 0x10a5182 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x10a5190); f_010a4b18(cpu); if cpu.returned_to != 0x10a5190 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10a524f; continue;
 }
 0x10a5198 => {
  { let v = 0x11695bcu32; cpu.push(v); }
  { let v: u32 = 0x11696d0u32; cpu.ecx = v; }
  { let v: u32 = 0x11695bcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10a51b2); f_010b2ed6(cpu); if cpu.returned_to != 0x10a51b2 { return; }
  { let v: u32 = 0x12623a0u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10a51c0); f_010a34d0(cpu); if cpu.returned_to != 0x10a51c0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a524a; continue; }
  pc = 0x10a51c8; continue;
 }
 0x10a51c8 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11696d8u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a51ec; continue;
 }
 0x10a51ec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5204; continue; }
  pc = 0x10a51f4; continue;
 }
 0x10a51f4 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a51ec; continue; }
  pc = 0x10a5204; continue;
 }
 0x10a5204 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a5213; continue;
 }
 0x10a5213 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a522b; continue; }
  pc = 0x10a521b; continue;
 }
 0x10a521b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a5213; continue; }
  pc = 0x10a522b; continue;
 }
 0x10a522b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x10a5239); f_010a4b18(cpu); if cpu.returned_to != 0x10a5239 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a524a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x10a524f; continue;
 }
 0x10a524f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x104u32); cpu.esp = v; }
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
pub fn f_010a5260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5260); }
 let mut pc: u32 = 0x10a5260;
 loop { match pc {
 0x10a5260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10a527a; continue; }
  pc = 0x10a5278; continue;
 }
 0x10a5278 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a527a; continue;
 }
 0x10a527a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5292; continue; }
  pc = 0x10a5283; continue;
 }
 0x10a5283 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  cpu.push(0x10a528a); f_010d6c20(cpu); if cpu.returned_to != 0x10a528a { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a5283; continue; }
  pc = 0x10a5292; continue;
 }
 0x10a5292 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_010a529c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a529c); }
 let mut pc: u32 = 0x10a529c;
 loop { match pc {
 0x10a529c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x11755d0u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12cbc1cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a52cc; continue; }
  pc = 0x10a52b9; continue;
 }
 0x10a52b9 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a52bd; continue;
 }
 0x10a52bd => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a52bd; continue; }
  pc = 0x10a52cc; continue;
 }
 0x10a52cc => {
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a52f8; continue; }
  pc = 0x10a52d6; continue;
 }
 0x10a52d6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  pc = 0x10a52e2; continue;
 }
 0x10a52e2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x11755dcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11755ccu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a52e2; continue; }
  pc = 0x10a52f8; continue;
 }
 0x10a52f8 => {
  { let v: u32 = cpu.esi; cpu.w32(0x11755d0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x12cbc24u32, v); }
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
pub fn f_010a530c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a530c); }
 let mut pc: u32 = 0x10a530c;
 loop { match pc {
 0x10a530c => {
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x11755ecu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5364(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5364); }
 let mut pc: u32 = 0x10a5364;
 loop { match pc {
 0x10a5364 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a53a4; continue; }
  pc = 0x10a537d; continue;
 }
 0x10a537d => {
  { let v: u32 = 0x11755ecu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x13u32); cpu.ebx = v; }
  pc = 0x10a5385; continue;
 }
 0x10a5385 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a538e); f_010a31d0(cpu); if cpu.returned_to != 0x10a538e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5396; continue; }
  pc = 0x10a5392; continue;
 }
 0x10a5392 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x10a53a4; continue;
 }
 0x10a5396 => {
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x20u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a5385; continue; }
  pc = 0x10a53a4; continue;
 }
 0x10a53a4 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x10a53af; continue; }
  pc = 0x10a53a8; continue;
 }
 0x10a53a8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a53af); f_010d6a81(cpu); if cpu.returned_to != 0x10a53af { return; }
  pc = 0x10a53af; continue;
 }
 0x10a53af => {
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
pub fn f_010a53b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a53b8); }
 let mut pc: u32 = 0x10a53b8;
 loop { match pc {
 0x10a53b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x11755ecu32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a53e0; continue; }
  pc = 0x10a53ce; continue;
 }
 0x10a53ce => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10a53d8; continue; }
  pc = 0x10a53d3; continue;
 }
 0x10a53d3 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x13u32); cpu.eax = v; }
  pc = 0x10a53e5; continue;
 }
 0x10a53d8 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a53ce; continue; }
  pc = 0x10a53e0; continue;
 }
 0x10a53e0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10a53e5; continue;
 }
 0x10a53e5 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x12cbc20u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a53f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a53f0); }
 let mut pc: u32 = 0x10a53f0;
 loop { match pc {
 0x10a53f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a54c4; continue; }
  pc = 0x10a5401; continue;
 }
 0x10a5401 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x12cbc24u32, v); }
  { cpu.sub32(cpu.ebp, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a543c; continue; }
  pc = 0x10a540f; continue;
 }
 0x10a540f => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11696f8u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  cpu.push(0x10a5432); f_010a4b18(cpu); if cpu.returned_to != 0x10a5432 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a54cf; continue;
 }
 0x10a543c => {
  { cpu.sub32(cpu.r32(0x11755d0u32), 0x0u32); }
  if !cpu.zf { pc = 0x10a5499; continue; }
  pc = 0x10a5445; continue;
 }
 0x10a5445 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  cpu.push(0x10a5456); f_0109bdf4(cpu); if cpu.returned_to != 0x10a5456 { return; }
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11755d0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5499; continue; }
  pc = 0x10a5465; continue;
 }
 0x10a5465 => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169730u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  cpu.push(0x10a5487); f_010a4b18(cpu); if cpu.returned_to != 0x10a5487 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5499 => {
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12cbc1cu32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a54c4; continue; }
  pc = 0x10a54ad; continue;
 }
 0x10a54ad => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10a54af; continue;
 }
 0x10a54af => {
  { let v: u32 = cpu.r32(0x11755d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a54af; continue; }
  pc = 0x10a54c4; continue;
 }
 0x10a54c4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12cbc24u32, v); }
  pc = 0x10a54cf; continue;
 }
 0x10a54cf => {
  { let v: u32 = cpu.r32(0x12cbc24u32); cpu.ebp = v; }
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
pub fn f_010a54dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a54dc); }
 let mut pc: u32 = 0x10a54dc;
 loop { match pc {
 0x10a54dc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x110u32); cpu.esp = v; }
  { let v = 0x11695bcu32; cpu.push(v); }
  { let v: u32 = 0x1169778u32; cpu.ecx = v; }
  { let v: u32 = 0x21cu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = 0x11695bcu32; cpu.edx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x10a5506); f_010b2ed6(cpu); if cpu.returned_to != 0x10a5506 { return; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x10a5514); f_010a3420(cpu); if cpu.returned_to != 0x10a5514 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x10a5599; continue; }
  pc = 0x10a5521; continue;
 }
 0x10a5521 => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169784u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a5544; continue;
 }
 0x10a5544 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a555c; continue; }
  pc = 0x10a554c; continue;
 }
 0x10a554c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a5544; continue; }
  pc = 0x10a555c; continue;
 }
 0x10a555c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a556b; continue;
 }
 0x10a556b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5583; continue; }
  pc = 0x10a5573; continue;
 }
 0x10a5573 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a556b; continue; }
  pc = 0x10a5583; continue;
 }
 0x10a5583 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a558f); f_010a4b18(cpu); if cpu.returned_to != 0x10a558f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a5821; continue;
 }
 0x10a5599 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  cpu.push(0x10a55ab); f_010a3aa0(cpu); if cpu.returned_to != 0x10a55ab { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a55b4; continue; }
  pc = 0x10a55af; continue;
 }
 0x10a55af => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a55b4; continue;
 }
 0x10a55b4 => {
  { cpu.sub32(cpu.r32(0x108u32.wrapping_add(cpu.esp)), 0x4e494244u32); }
  if cpu.zf { pc = 0x10a5644; continue; }
  pc = 0x10a55c5; continue;
 }
 0x10a55c5 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11697b4u32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a55e7; continue;
 }
 0x10a55e7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a55ff; continue; }
  pc = 0x10a55ef; continue;
 }
 0x10a55ef => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a55e7; continue; }
  pc = 0x10a55ff; continue;
 }
 0x10a55ff => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a560e; continue;
 }
 0x10a560e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a5626; continue; }
  pc = 0x10a5616; continue;
 }
 0x10a5616 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a560e; continue; }
  pc = 0x10a5626; continue;
 }
 0x10a5626 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a5632); f_010a4b18(cpu); if cpu.returned_to != 0x10a5632 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5644 => {
  { let v: u32 = cpu.r32(0x10au32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x12cbc20u32; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a565d); f_010a3aa0(cpu); if cpu.returned_to != 0x10a565d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a5666; continue; }
  pc = 0x10a5661; continue;
 }
 0x10a5661 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a5666; continue;
 }
 0x10a5666 => {
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a56a6; continue; }
  pc = 0x10a5674; continue;
 }
 0x10a5674 => {
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x11697e0u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10a5694); f_010a4b18(cpu); if cpu.returned_to != 0x10a5694 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a56a6 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5783; continue; }
  pc = 0x10a56b0; continue;
 }
 0x10a56b0 => {
  { let v: u32 = cpu.r32(0x10au32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = 0x104u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a56cd); f_010a3aa0(cpu); if cpu.returned_to != 0x10a56cd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a56db; continue; }
  pc = 0x10a56d1; continue;
 }
 0x10a56d1 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a5783; continue;
 }
 0x10a56db => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = 0xdu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x11755ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  cpu.push(0x10a56fa); f_010a3aa0(cpu); if cpu.returned_to != 0x10a56fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a5703; continue; }
  pc = 0x10a56fe; continue;
 }
 0x10a56fe => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a5703; continue;
 }
 0x10a5703 => {
  { let v: u32 = cpu.r32(0x10au32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a571e); f_010a3aa0(cpu); if cpu.returned_to != 0x10a571e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a5727; continue; }
  pc = 0x10a5722; continue;
 }
 0x10a5722 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a5727; continue;
 }
 0x10a5727 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x10au32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x108u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a574c); f_010a3aa0(cpu); if cpu.returned_to != 0x10a574c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10a5755; continue; }
  pc = 0x10a5750; continue;
 }
 0x10a5750 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10a5755; continue;
 }
 0x10a5755 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x12cbc20u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a56b0; continue; }
  pc = 0x10a5783; continue;
 }
 0x10a5783 => {
  { let v: u32 = cpu.r32(0x10au32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x10a5792); f_010a3a34(cpu); if cpu.returned_to != 0x10a5792 { return; }
  { cpu.sub16((cpu.edi as u16), 0x1u16); }
  if !cpu.zf { pc = 0x10a581c; continue; }
  pc = 0x10a579c; continue;
 }
 0x10a579c => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x116981cu32; cpu.esi = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a57bf; continue;
 }
 0x10a57bf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a57d7; continue; }
  pc = 0x10a57c7; continue;
 }
 0x10a57c7 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a57bf; continue; }
  pc = 0x10a57d7; continue;
 }
 0x10a57d7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x11695f8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10a57e6; continue;
 }
 0x10a57e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a57fe; continue; }
  pc = 0x10a57ee; continue;
 }
 0x10a57ee => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a57e6; continue; }
  pc = 0x10a57fe; continue;
 }
 0x10a57fe => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1169578u32; cpu.push(v); }
  cpu.push(0x10a580a); f_010a4b18(cpu); if cpu.returned_to != 0x10a580a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a581c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a5821; continue;
 }
 0x10a5821 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x110u32); cpu.esp = v; }
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
pub fn f_010a5830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5830); }
 let mut pc: u32 = 0x10a5830;
 loop { match pc {
 0x10a5830 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a586f; continue; }
  pc = 0x10a5845; continue;
 }
 0x10a5845 => {
  { let v: u32 = 0xdu32; cpu.ecx = v; }
  { let v = 0x12554a0u32; cpu.push(v); }
  { let v: u32 = 0x12554a0u32; cpu.edi = v; }
  { let v: u32 = 0x1169840u32; cpu.esi = v; }
  { let v = 0x1169578u32; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10a5865); f_010a4b18(cpu); if cpu.returned_to != 0x10a5865 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a586f => {
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12cbc20u32, v); }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = 0x11755ecu32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x13u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10a588b; continue;
 }
 0x10a588b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10a58a3; continue; }
  pc = 0x10a5893; continue;
 }
 0x10a5893 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a588b; continue; }
  pc = 0x10a58a3; continue;
 }
 0x10a58a3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x12u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x12cbc20u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a58d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a58d4); }
 let mut pc: u32 = 0x10a58d4;
 loop { match pc {
 0x10a58d4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1174388u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1174384u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5912; continue; }
  pc = 0x10a58f3; continue;
 }
 0x10a58f3 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a58fc); f_010a31d0(cpu); if cpu.returned_to != 0x10a58fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5904; continue; }
  pc = 0x10a5900; continue;
 }
 0x10a5900 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x10a5912; continue;
 }
 0x10a5904 => {
  { let v: u32 = cpu.r32(0x1174384u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xdu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a58f3; continue; }
  pc = 0x10a5912; continue;
 }
 0x10a5912 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
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
pub fn f_010a5934(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5934); }
 let mut pc: u32 = 0x10a5934;
 loop { match pc {
 0x10a5934 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10a5940); f_010a5e04(cpu); if cpu.returned_to != 0x10a5940 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a594f; continue; }
  pc = 0x10a5945; continue;
 }
 0x10a5945 => {
  { let v: u32 = 0x186a1u32; cpu.eax = v; }
  pc = 0x10a59cf; continue;
 }
 0x10a594f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a5956); f_010a50c8(cpu); if cpu.returned_to != 0x10a5956 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a5966; continue; }
  pc = 0x10a595b; continue;
 }
 0x10a595b => {
  { let v: u32 = 0x186a2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5966 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a596d); f_010a4ca0(cpu); if cpu.returned_to != 0x10a596d { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a597d; continue; }
  pc = 0x10a5972; continue;
 }
 0x10a5972 => {
  { let v: u32 = 0x186a3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a597d => {
  { let v: u32 = cpu.r32(0x320u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11755dcu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12cbc28u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a5997); f_010a53f0(cpu); if cpu.returned_to != 0x10a5997 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a59a7; continue; }
  pc = 0x10a599c; continue;
 }
 0x10a599c => {
  { let v: u32 = 0x186a4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a59a7 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a59ae); f_010a54dc(cpu); if cpu.returned_to != 0x10a59ae { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x10a59be; continue; }
  pc = 0x10a59b3; continue;
 }
 0x10a59b3 => {
  { let v: u32 = 0x186a4u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a59be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117553cu32, v); }
  cpu.push(0x10a59ca); f_010d1c46(cpu); if cpu.returned_to != 0x10a59ca { return; }
  { let v: u32 = 0x186a0u32; cpu.eax = v; }
  pc = 0x10a59cf; continue;
 }
 0x10a59cf => {
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
pub fn f_010a59d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a59d8); }
 let mut pc: u32 = 0x10a59d8;
 loop { match pc {
 0x10a59d8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1174388u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a59ea; continue; }
  pc = 0x10a59e3; continue;
 }
 0x10a59e3 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10a59ea); f_0109c4a8(cpu); if cpu.returned_to != 0x10a59ea { return; }
  pc = 0x10a59ea; continue;
 }
 0x10a59ea => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x11755d0u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1174388u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10a5a03; continue; }
  pc = 0x10a59fc; continue;
 }
 0x10a59fc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10a5a03); f_0109c4a8(cpu); if cpu.returned_to != 0x10a5a03 { return; }
  pc = 0x10a5a03; continue;
 }
 0x10a5a03 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11755d8u32); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11755d0u32, v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10a5a1c; continue; }
  pc = 0x10a5a15; continue;
 }
 0x10a5a15 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10a5a1c); f_0109c4a8(cpu); if cpu.returned_to != 0x10a5a1c { return; }
  pc = 0x10a5a1c; continue;
 }
 0x10a5a1c => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x11755d8u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5a26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5a26); }
 let mut pc: u32 = 0x10a5a26;
 loop { match pc {
 0x10a5a26 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x10a5a29; continue;
 }
 0x10a5a29 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10a5a47; continue; }
  pc = 0x10a5a44; continue;
 }
 0x10a5a44 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10a5a29; continue;
 }
 0x10a5a47 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2bu32); }
  if cpu.zf { pc = 0x10a5a55; continue; }
  pc = 0x10a5a50; continue;
 }
 0x10a5a50 => {
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x10a5a56; continue; }
  pc = 0x10a5a55; continue;
 }
 0x10a5a55 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10a5a56; continue;
 }
 0x10a5a56 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a5a58; continue;
 }
 0x10a5a58 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10a5a82; continue; }
  pc = 0x10a5a73; continue;
 }
 0x10a5a73 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x10a5a58; continue;
 }
 0x10a5a82 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x10a5a8d; continue; }
  pc = 0x10a5a8b; continue;
 }
 0x10a5a8b => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10a5a8d; continue;
 }
 0x10a5a8d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5a93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5a93); }
 let mut pc: u32 = 0x10a5a93;
 loop { match pc {
 0x10a5a93 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  pc = 0x10a5a99; continue;
 }
 0x10a5a99 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10a5aa2; continue; }
  pc = 0x10a5a9d; continue;
 }
 0x10a5a9d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5aa2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a5ab6; continue; }
  pc = 0x10a5aaf; continue;
 }
 0x10a5aaf => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a5ab6; continue; }
  pc = 0x10a5ab4; continue;
 }
 0x10a5ab4 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a5ab6; continue;
 }
 0x10a5ab6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a5ac7; continue; }
  pc = 0x10a5abf; continue;
 }
 0x10a5abf => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a5ac7; continue; }
  pc = 0x10a5ac4; continue;
 }
 0x10a5ac4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10a5ac7; continue;
 }
 0x10a5ac7 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10a5add; continue; }
  pc = 0x10a5acb; continue;
 }
 0x10a5acb => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5add => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x10a5a9d; continue; }
  pc = 0x10a5ae1; continue;
 }
 0x10a5ae1 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10a5a99; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5af0); }
 let mut pc: u32 = 0x10a5af0;
 loop { match pc {
 0x10a5af0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.push(0x10a5afe); f_010ca04e(cpu); if cpu.returned_to != 0x10a5afe { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x11750acu32, v); }
  cpu.push(0x10a5b0d); f_010ca247(cpu); if cpu.returned_to != 0x10a5b0d { return; }
  { cpu.sub32(cpu.r32(0x11750b0u32), 0x20000u32); }
  if !cpu.zf { pc = 0x10a5b40; continue; }
  pc = 0x10a5b19; continue;
 }
 0x10a5b19 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a5b68; continue;
 }
 0x10a5b40 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a5b68; continue;
 }
 0x10a5b68 => {
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1255e78u32, v); }
  cpu.push(0x10a5b7a); f_010ca189(cpu); if cpu.returned_to != 0x10a5b7a { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1255e68u32, v); }
  cpu.push(0x10a5b86); f_010ca184(cpu); if cpu.returned_to != 0x10a5b86 { return; }
  { let v: u32 = cpu.r32(0x1255e68u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11750b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1255dd0u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11750b4u32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5bb4; continue; }
  pc = 0x10a5baa; continue;
 }
 0x10a5baa => {
  { let v: u32 = 0x40u32; cpu.w32(0x11750b4u32, v); }
  pc = 0x10a5bb4; continue;
 }
 0x10a5bb4 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5c39; continue; }
  pc = 0x10a5bc4; continue;
 }
 0x10a5bc4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a5bc6; continue;
 }
 0x10a5bc6 => {
  { let v = 0x1255e6cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1255c80u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10a5bd8); f_010ca1a7(cpu); if cpu.returned_to != 0x10a5bd8 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1255e74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e70u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e6cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e78u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a5c15); f_010ca21a(cpu); if cpu.returned_to != 0x10a5c15 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12555e0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a5c29; continue; }
  pc = 0x10a5c20; continue;
 }
 0x10a5c20 => {
  cpu.push(0x10a5c25); f_010ca0ad(cpu); if cpu.returned_to != 0x10a5c25 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a5c8e; continue;
 }
 0x10a5c29 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a5bc6; continue; }
  pc = 0x10a5c39; continue;
 }
 0x10a5c39 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a5c6a; continue; }
  pc = 0x10a5c43; continue;
 }
 0x10a5c43 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x10a5c4f; continue;
 }
 0x10a5c4f => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750a8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11750b0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a5c4f; continue; }
  pc = 0x10a5c6a; continue;
 }
 0x10a5c6a => {
  { let v: u32 = 0x11755ecu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a5c71; continue;
 }
 0x10a5c71 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a5c71; continue; }
  pc = 0x10a5c89; continue;
 }
 0x10a5c89 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a5c8e; continue;
 }
 0x10a5c8e => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10a5df7; continue; }
  pc = 0x10a5c97; continue;
 }
 0x10a5c97 => {
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a5c9e); f_010ca0df(cpu); if cpu.returned_to != 0x10a5c9e { return; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a5ca5); f_010ca044(cpu); if cpu.returned_to != 0x10a5ca5 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a5cac); f_010ca03f(cpu); if cpu.returned_to != 0x10a5cac { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a5cb3); f_010ca049(cpu); if cpu.returned_to != 0x10a5cb3 { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a5cbc); f_010ca026(cpu); if cpu.returned_to != 0x10a5cbc { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a5cc3); f_010ca085(cpu); if cpu.returned_to != 0x10a5cc3 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1174396u32, v); }
  cpu.push(0x10a5cd3); f_010ca24c(cpu); if cpu.returned_to != 0x10a5cd3 { return; }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a5cda); f_010ca1c5(cpu); if cpu.returned_to != 0x10a5cda { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x10a658cu32; cpu.ebx = v; }
  { let v: u32 = 0x10a66bcu32; cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10a66d4u32; cpu.edi = v; }
  { let v: u32 = 0x10a66e0u32; cpu.ebp = v; }
  cpu.push(0x10a5cf7); f_010ca157(cpu); if cpu.returned_to != 0x10a5cf7 { return; }
  { let v: u32 = 0x10a6580u32; cpu.eax = v; }
  { let v: u32 = 0x10a666cu32; cpu.edx = v; }
  { let v: u32 = 0x10a66a4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11754d0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11754d8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11754dcu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11754e0u32, v); }
  { let v: u32 = 0x10a6848u32; cpu.ebx = v; }
  { let v: u32 = 0x10a6868u32; cpu.esi = v; }
  { let v: u32 = 0x10a6874u32; cpu.edi = v; }
  { let v: u32 = 0x10a689cu32; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11754c8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11754ccu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x11754d4u32, v); }
  { let v: u32 = 0x10a6824u32; cpu.eax = v; }
  { let v: u32 = 0x10a6834u32; cpu.edx = v; }
  { let v: u32 = 0x10a6854u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11754ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x11754f4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11754f8u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11754fcu32, v); }
  { let v: u32 = 0x10a68fcu32; cpu.ebx = v; }
  { let v: u32 = 0x10a694cu32; cpu.esi = v; }
  { let v: u32 = 0x10a6960u32; cpu.edi = v; }
  { let v: u32 = 0x10ca076u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x11754e4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11754e8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x11754f0u32, v); }
  { let v: u32 = 0x10a68c4u32; cpu.eax = v; }
  { let v: u32 = 0x10a68e0u32; cpu.edx = v; }
  { let v: u32 = 0x10a6924u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1175508u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1175510u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1175514u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1175518u32, v); }
  { let v: u32 = 0x10a6978u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1175500u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1175504u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x117550cu32, v); }
  { let v: u32 = 0x10ca071u32; cpu.eax = v; }
  { let v: u32 = 0x10a696cu32; cpu.edx = v; }
  { let v: u32 = 0x10a6804u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1175524u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x117551cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1175520u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1175528u32, v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a5df7; continue;
 }
 0x10a5df7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010a5e04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5e04); }
 let mut pc: u32 = 0x10a5e04;
 loop { match pc {
 0x10a5e04 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1174394u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x10a5e1a; continue; }
  pc = 0x10a5e14; continue;
 }
 0x10a5e14 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5e1a => {
  { let v = 0x1255dd4u32; cpu.push(v); }
  cpu.push(0x10a5e24); f_010ca10c(cpu); if cpu.returned_to != 0x10a5e24 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5e2e; continue; }
  pc = 0x10a5e28; continue;
 }
 0x10a5e28 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5e2e => {
  cpu.push(0x10a5e33); f_010ca09e(cpu); if cpu.returned_to != 0x10a5e33 { return; }
  { let v = 0x1255dd4u32; cpu.push(v); }
  cpu.push(0x10a5e3d); f_010ca111(cpu); if cpu.returned_to != 0x10a5e3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5e47; continue; }
  pc = 0x10a5e41; continue;
 }
 0x10a5e41 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10a5e47 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10a5e53); f_010ca125(cpu); if cpu.returned_to != 0x10a5e53 { return; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x1174394u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a5e60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a5e60); }
 let mut pc: u32 = 0x10a5e60;
 loop { match pc {
 0x10a5e60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x1174394u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x1u16); }
  if !cpu.zf { pc = 0x10a5e79; continue; }
  pc = 0x10a5e75; continue;
 }
 0x10a5e75 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10a5eb7; continue;
 }
 0x10a5e79 => {
  { let v = 0x1255dd4u32; cpu.push(v); }
  cpu.push(0x10a5e83); f_010ca10c(cpu); if cpu.returned_to != 0x10a5e83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5e8b; continue; }
  pc = 0x10a5e87; continue;
 }
 0x10a5e87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a5eb7; continue;
 }
 0x10a5e8b => {
  cpu.push(0x10a5e90); f_010ca09e(cpu); if cpu.returned_to != 0x10a5e90 { return; }
  { let v = 0x1255dd4u32; cpu.push(v); }
  cpu.push(0x10a5e9a); f_010ca111(cpu); if cpu.returned_to != 0x10a5e9a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5ea2; continue; }
  pc = 0x10a5e9e; continue;
 }
 0x10a5e9e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a5eb7; continue;
 }
 0x10a5ea2 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a5ea9); f_010ca125(cpu); if cpu.returned_to != 0x10a5ea9 { return; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1174394u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10a5eb7; continue;
 }
 0x10a5eb7 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10a6070; continue; }
  pc = 0x10a5ec0; continue;
 }
 0x10a5ec0 => {
  { cpu.sub32(cpu.r32(0x1255dd8u32), 0x1u32); }
  if cpu.zf { pc = 0x10a5ecb; continue; }
  pc = 0x10a5ec9; continue;
 }
 0x10a5ec9 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a5ecb; continue;
 }
 0x10a5ecb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a5edd); f_010ca143(cpu); if cpu.returned_to != 0x10a5edd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a5eed; continue; }
  pc = 0x10a5ee1; continue;
 }
 0x10a5ee1 => {
  cpu.push(0x10a5ee6); f_010ca0ad(cpu); if cpu.returned_to != 0x10a5ee6 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10a6070; continue;
 }
 0x10a5eed => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10a5ef7); f_010ca04e(cpu); if cpu.returned_to != 0x10a5ef7 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x11750acu32, v); }
  cpu.push(0x10a5f02); f_010a6224(cpu); if cpu.returned_to != 0x10a5f02 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10a6066; continue; }
  pc = 0x10a5f0b; continue;
 }
 0x10a5f0b => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a5f11); f_010ca0df(cpu); if cpu.returned_to != 0x10a5f11 { return; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a5f18); f_010ca044(cpu); if cpu.returned_to != 0x10a5f18 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a5f1e); f_010ca03f(cpu); if cpu.returned_to != 0x10a5f1e { return; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a5f24); f_010ca049(cpu); if cpu.returned_to != 0x10a5f24 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a5f2b); f_010ca026(cpu); if cpu.returned_to != 0x10a5f2b { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  cpu.push(0x10a5f34); f_010ca085(cpu); if cpu.returned_to != 0x10a5f34 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x1174396u32, v); }
  cpu.push(0x10a5f42); f_010ca24c(cpu); if cpu.returned_to != 0x10a5f42 { return; }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a5f49); f_010ca1c5(cpu); if cpu.returned_to != 0x10a5f49 { return; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x10a658cu32; cpu.edi = v; }
  { let v: u32 = 0x10a66a4u32; cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10a666cu32; cpu.esi = v; }
  { let v: u32 = 0x10a66e0u32; cpu.ebx = v; }
  cpu.push(0x10a5f65); f_010ca157(cpu); if cpu.returned_to != 0x10a5f65 { return; }
  { let v: u32 = 0x10a6580u32; cpu.ecx = v; }
  { let v: u32 = 0x10a66bcu32; cpu.eax = v; }
  { let v: u32 = 0x10a66d4u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11754ccu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11754d0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11754d4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11754e0u32, v); }
  { let v: u32 = 0x10a6834u32; cpu.esi = v; }
  { let v: u32 = 0x10a6848u32; cpu.edi = v; }
  { let v: u32 = 0x10a6854u32; cpu.ebp = v; }
  { let v: u32 = 0x10a689cu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11754c8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11754d8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11754dcu32, v); }
  { let v: u32 = 0x10a6824u32; cpu.ecx = v; }
  { let v: u32 = 0x10a6868u32; cpu.eax = v; }
  { let v: u32 = 0x10a6874u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11754e8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x11754ecu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11754f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11754fcu32, v); }
  { let v: u32 = 0x10a68e0u32; cpu.esi = v; }
  { let v: u32 = 0x10a68fcu32; cpu.edi = v; }
  { let v: u32 = 0x10a6924u32; cpu.ebp = v; }
  { let v: u32 = 0x10ca076u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11754e4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11754f4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11754f8u32, v); }
  { let v: u32 = 0x10a68c4u32; cpu.ecx = v; }
  { let v: u32 = 0x10a694cu32; cpu.eax = v; }
  { let v: u32 = 0x10a6960u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1175504u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1175508u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x117550cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1175518u32, v); }
  { let v: u32 = 0x10a696cu32; cpu.esi = v; }
  { let v: u32 = 0x10a6978u32; cpu.edi = v; }
  { let v: u32 = 0x10a6804u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1175500u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1175510u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1175514u32, v); }
  { let v: u32 = 0x10ca071u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1175520u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1175524u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1175528u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x117551cu32, v); }
  pc = 0x10a6066; continue;
 }
 0x10a6066 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x10a6070; continue; }
  pc = 0x10a606b; continue;
 }
 0x10a606b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6070; continue;
 }
 0x10a6070 => {
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
pub fn f_010a6078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6078); }
 let mut pc: u32 = 0x10a6078;
 loop { match pc {
 0x10a6078 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a608a); f_010ca044(cpu); if cpu.returned_to != 0x10a608a { return; }
  { cpu.sub16(cpu.r16(0x1174396u32), 0x1u16); }
  if !cpu.zf { pc = 0x10a609b; continue; }
  pc = 0x10a6094; continue;
 }
 0x10a6094 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a609b); f_010ca085(cpu); if cpu.returned_to != 0x10a609b { return; }
  pc = 0x10a609b; continue;
 }
 0x10a609b => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a60a2); f_010ca049(cpu); if cpu.returned_to != 0x10a60a2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a60b6); f_010ca0c1(cpu); if cpu.returned_to != 0x10a60b6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a60bc; continue; }
  pc = 0x10a60ba; continue;
 }
 0x10a60ba => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10a60bc; continue;
 }
 0x10a60bc => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10a60d7; continue; }
  pc = 0x10a60c0; continue;
 }
 0x10a60c0 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a60cf); f_010ca0c1(cpu); if cpu.returned_to != 0x10a60cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a60d7; continue; }
  pc = 0x10a60d3; continue;
 }
 0x10a60d3 => {
  { let v: u32 = cpu.eax ^ cpu.esi; cpu.eax = v; }
  pc = 0x10a60e8; continue;
 }
 0x10a60d7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a60e8; continue;
 }
 0x10a60e8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a60f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a60f0); }
 let mut pc: u32 = 0x10a60f0;
 loop { match pc {
 0x10a60f0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a60fb); f_010ca0d0(cpu); if cpu.returned_to != 0x10a60fb { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6102); f_010ca049(cpu); if cpu.returned_to != 0x10a6102 { return; }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a6109); f_010ca044(cpu); if cpu.returned_to != 0x10a6109 { return; }
  { cpu.sub16(cpu.r16(0x1174396u32), 0x1u16); }
  if !cpu.zf { pc = 0x10a611a; continue; }
  pc = 0x10a6113; continue;
 }
 0x10a6113 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a611a); f_010ca085(cpu); if cpu.returned_to != 0x10a611a { return; }
  pc = 0x10a611a; continue;
 }
 0x10a611a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6120); }
 let mut pc: u32 = 0x10a6120;
 loop { match pc {
 0x10a6120 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a613c); f_010ca0c1(cpu); if cpu.returned_to != 0x10a613c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6144; continue; }
  pc = 0x10a6140; continue;
 }
 0x10a6140 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6155; continue;
 }
 0x10a6144 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6155; continue;
 }
 0x10a6155 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a615c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a615c); }
 let mut pc: u32 = 0x10a615c;
 loop { match pc {
 0x10a615c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6167); f_010ca0d0(cpu); if cpu.returned_to != 0x10a6167 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a616c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a616c); }
 let mut pc: u32 = 0x10a616c;
 loop { match pc {
 0x10a616c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6188); f_010ca0c1(cpu); if cpu.returned_to != 0x10a6188 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6190; continue; }
  pc = 0x10a618c; continue;
 }
 0x10a618c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a61bb; continue;
 }
 0x10a6190 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1175540u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1175544u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a61ac; continue; }
  pc = 0x10a61a6; continue;
 }
 0x10a61a6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10a61ac; continue;
 }
 0x10a61ac => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10a61b6; continue; }
  pc = 0x10a61b0; continue;
 }
 0x10a61b0 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10a61b6; continue;
 }
 0x10a61b6 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a61bb; continue;
 }
 0x10a61bb => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a61c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a61c4); }
 let mut pc: u32 = 0x10a61c4;
 loop { match pc {
 0x10a61c4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a61cf); f_010ca0d0(cpu); if cpu.returned_to != 0x10a61cf { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a61d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a61d4); }
 let mut pc: u32 = 0x10a61d4;
 loop { match pc {
 0x10a61d4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a61dd); f_010ca0f3(cpu); if cpu.returned_to != 0x10a61dd { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a61ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a61ec); }
 let mut pc: u32 = 0x10a61ec;
 loop { match pc {
 0x10a61ec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10a6209); f_010ca0c1(cpu); if cpu.returned_to != 0x10a6209 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a620f; continue; }
  pc = 0x10a620d; continue;
 }
 0x10a620d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a620f; continue;
 }
 0x10a620f => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6218); f_010ca0d0(cpu); if cpu.returned_to != 0x10a6218 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6224(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6224); }
 let mut pc: u32 = 0x10a6224;
 loop { match pc {
 0x10a6224 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.push(0x10a6232); f_010ca247(cpu); if cpu.returned_to != 0x10a6232 { return; }
  { cpu.sub32(cpu.r32(0x11750b0u32), 0x20000u32); }
  if !cpu.zf { pc = 0x10a6263; continue; }
  pc = 0x10a623e; continue;
 }
 0x10a623e => {
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a628b; continue;
 }
 0x10a6263 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0x80u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a628b; continue;
 }
 0x10a628b => {
  { let v: u32 = 0xau32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1255e78u32, v); }
  cpu.push(0x10a629d); f_010ca189(cpu); if cpu.returned_to != 0x10a629d { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1255e68u32, v); }
  cpu.push(0x10a62a9); f_010ca184(cpu); if cpu.returned_to != 0x10a62a9 { return; }
  { let v: u32 = cpu.r32(0x1255e68u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11750b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1255dd0u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11750b4u32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a62d7; continue; }
  pc = 0x10a62cd; continue;
 }
 0x10a62cd => {
  { let v: u32 = 0x40u32; cpu.w32(0x11750b4u32, v); }
  pc = 0x10a62d7; continue;
 }
 0x10a62d7 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a635c; continue; }
  pc = 0x10a62e7; continue;
 }
 0x10a62e7 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a62e9; continue;
 }
 0x10a62e9 => {
  { let v = 0x1255e6cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1255c80u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10a62fb); f_010ca1a7(cpu); if cpu.returned_to != 0x10a62fb { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1255e74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e70u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e6cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e78u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6338); f_010ca21a(cpu); if cpu.returned_to != 0x10a6338 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12555e0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a634c; continue; }
  pc = 0x10a6343; continue;
 }
 0x10a6343 => {
  cpu.push(0x10a6348); f_010ca0ad(cpu); if cpu.returned_to != 0x10a6348 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a63b1; continue;
 }
 0x10a634c => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a62e9; continue; }
  pc = 0x10a635c; continue;
 }
 0x10a635c => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a638d; continue; }
  pc = 0x10a6366; continue;
 }
 0x10a6366 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x10a6372; continue;
 }
 0x10a6372 => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750a8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11750b0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a6372; continue; }
  pc = 0x10a638d; continue;
 }
 0x10a638d => {
  { let v: u32 = 0x11755ecu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a6394; continue;
 }
 0x10a6394 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a6394; continue; }
  pc = 0x10a63ac; continue;
 }
 0x10a63ac => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a63b1; continue;
 }
 0x10a63b1 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010a63bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a63bc); }
 let mut pc: u32 = 0x10a63bc;
 loop { match pc {
 0x10a63bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a63c6); f_010ca189(cpu); if cpu.returned_to != 0x10a63c6 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10a63cf); f_010ca184(cpu); if cpu.returned_to != 0x10a63cf { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf8u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a63dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a63dc); }
 let mut pc: u32 = 0x10a63dc;
 loop { match pc {
 0x10a63dc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x11750b0u32)); }
  if cpu.zf { pc = 0x10a6575; continue; }
  pc = 0x10a63f1; continue;
 }
 0x10a63f1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x11750b0u32, v); }
  cpu.push(0x10a63fb); f_010ca247(cpu); if cpu.returned_to != 0x10a63fb { return; }
  { cpu.sub32(cpu.r32(0x11750b0u32), 0x20000u32); }
  if !cpu.zf { pc = 0x10a642e; continue; }
  pc = 0x10a6407; continue;
 }
 0x10a6407 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a6456; continue;
 }
 0x10a642e => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e6cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1255e70u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x1255e74u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a6456; continue;
 }
 0x10a6456 => {
  { let v: u32 = 0xau32; cpu.ebp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x1255e78u32, v); }
  cpu.push(0x10a6468); f_010ca189(cpu); if cpu.returned_to != 0x10a6468 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x1255e68u32, v); }
  cpu.push(0x10a6474); f_010ca184(cpu); if cpu.returned_to != 0x10a6474 { return; }
  { let v: u32 = cpu.r32(0x1255e68u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x11750b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1255dd0u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x11750b4u32, v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a64a2; continue; }
  pc = 0x10a6498; continue;
 }
 0x10a6498 => {
  { let v: u32 = 0x40u32; cpu.w32(0x11750b4u32, v); }
  pc = 0x10a64a2; continue;
 }
 0x10a64a2 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a6525; continue; }
  pc = 0x10a64b2; continue;
 }
 0x10a64b2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a64b4; continue;
 }
 0x10a64b4 => {
  { let v = 0x1255e6cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1255c80u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10a64c6); f_010ca1a7(cpu); if cpu.returned_to != 0x10a64c6 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1255e74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e70u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e6cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255e78u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6503); f_010ca21a(cpu); if cpu.returned_to != 0x10a6503 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12555e0u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10a6515; continue; }
  pc = 0x10a650e; continue;
 }
 0x10a650e => {
  cpu.push(0x10a6513); f_010ca0ad(cpu); if cpu.returned_to != 0x10a6513 { return; }
  pc = 0x10a6575; continue;
 }
 0x10a6515 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebp); cpu.esi = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a64b4; continue; }
  pc = 0x10a6525; continue;
 }
 0x10a6525 => {
  { let v: u32 = cpu.r32(0x11750b4u32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10a6556; continue; }
  pc = 0x10a652f; continue;
 }
 0x10a652f => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  pc = 0x10a653b; continue;
 }
 0x10a653b => {
  { let v: u32 = cpu.eax.wrapping_add(0x10u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750a8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x11750acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x11750b0u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a653b; continue; }
  pc = 0x10a6556; continue;
 }
 0x10a6556 => {
  { let v: u32 = 0x11755ecu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10a655d; continue;
 }
 0x10a655d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x12cu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a655d; continue; }
  pc = 0x10a6575; continue;
 }
 0x10a6575 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010a6580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6580); }
 let mut pc: u32 = 0x10a6580;
 loop { match pc {
 0x10a6580 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a6587); f_010ca0f8(cpu); if cpu.returned_to != 0x10a6587 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a658c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a658c); }
 let mut pc: u32 = 0x10a658c;
 loop { match pc {
 0x10a658c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6597); f_010ca026(cpu); if cpu.returned_to != 0x10a6597 { return; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xffffffffu32; cpu.push(v); }
  cpu.push(0x10a65a5); f_010c9ffe(cpu); if cpu.returned_to != 0x10a65a5 { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a65ae); f_010ca026(cpu); if cpu.returned_to != 0x10a65ae { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a65b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a65b4); }
 let mut pc: u32 = 0x10a65b4;
 loop { match pc {
 0x10a65b4 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a65c3); f_010ca008(cpu); if cpu.returned_to != 0x10a65c3 { return; }
  { let v: u32 = cpu.r32(0x1175530u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1175568u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1175564u32); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x117552cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1175530u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x117552cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a65ef); f_010ca01c(cpu); if cpu.returned_to != 0x10a65ef { return; }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a660b); f_010ca026(cpu); if cpu.returned_to != 0x10a660b { return; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a6611); f_010ca049(cpu); if cpu.returned_to != 0x10a6611 { return; }
  { let v = 0xffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10a661e); f_010c9ffe(cpu); if cpu.returned_to != 0x10a661e { return; }
  { let v = 0x1e0u32; cpu.push(v); }
  { let v = 0x280u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6631); f_010ca01c(cpu); if cpu.returned_to != 0x10a6631 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6638); f_010ca049(cpu); if cpu.returned_to != 0x10a6638 { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6641); f_010ca026(cpu); if cpu.returned_to != 0x10a6641 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10a665f; continue; }
  pc = 0x10a664a; continue;
 }
 0x10a664a => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x1174396u32, v); }
  cpu.push(0x10a665d); f_010ca085(cpu); if cpu.returned_to != 0x10a665d { return; }
  pc = 0x10a6666; continue;
 }
 0x10a665f => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1174396u32, v); }
  pc = 0x10a6666; continue;
 }
 0x10a6666 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a666c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a666c); }
 let mut pc: u32 = 0x10a666c;
 loop { match pc {
 0x10a666c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6674); f_010ca008(cpu); if cpu.returned_to != 0x10a6674 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6678); }
 let mut pc: u32 = 0x10a6678;
 loop { match pc {
 0x10a6678 => {
  let t = cpu.r32(0x11754d4u32); cpu.push(0x10a667e); cpu.call(t); if cpu.returned_to != 0x10a667e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6680); }
 let mut pc: u32 = 0x10a6680;
 loop { match pc {
 0x10a6680 => {
  let t = cpu.r32(0x11754d8u32); cpu.push(0x10a6686); cpu.call(t); if cpu.returned_to != 0x10a6686 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6688); }
 let mut pc: u32 = 0x10a6688;
 loop { match pc {
 0x10a6688 => {
  let t = cpu.r32(0x11754dcu32); cpu.push(0x10a668e); cpu.call(t); if cpu.returned_to != 0x10a668e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66a4); }
 let mut pc: u32 = 0x10a66a4;
 loop { match pc {
 0x10a66a4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x1174396u32), 0x1u16); }
  if !cpu.zf { pc = 0x10a66b7; continue; }
  pc = 0x10a66b0; continue;
 }
 0x10a66b0 => {
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a66b7); f_010ca085(cpu); if cpu.returned_to != 0x10a66b7 { return; }
  pc = 0x10a66b7; continue;
 }
 0x10a66b7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66bc); }
 let mut pc: u32 = 0x10a66bc;
 loop { match pc {
 0x10a66bc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x1174396u32), 0x1u16); }
  if !cpu.zf { pc = 0x10a66cf; continue; }
  pc = 0x10a66c8; continue;
 }
 0x10a66c8 => {
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a66cf); f_010ca085(cpu); if cpu.returned_to != 0x10a66cf { return; }
  pc = 0x10a66cf; continue;
 }
 0x10a66cf => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66d4); }
 let mut pc: u32 = 0x10a66d4;
 loop { match pc {
 0x10a66d4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.push(0x10a66dd); f_010ca044(cpu); if cpu.returned_to != 0x10a66dd { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66e0); }
 let mut pc: u32 = 0x10a66e0;
 loop { match pc {
 0x10a66e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a66e9); f_010ca044(cpu); if cpu.returned_to != 0x10a66e9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a66ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a66ec); }
 let mut pc: u32 = 0x10a66ec;
 loop { match pc {
 0x10a66ec => {
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6714); f_010ca080(cpu); if cpu.returned_to != 0x10a6714 { return; }
  { let v = (cpu.r32(0x1175574u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x24u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf64(0x1169874u32); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x2cu32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(3) * cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = 0x12555a0u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  cpu.push(0x10a674f); f_010ca1ed(cpu); if cpu.returned_to != 0x10a674f { return; }
  { let v: u8 = cpu.r8(0x12555a0u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10a6761; continue; }
  pc = 0x10a675d; continue;
 }
 0x10a675d => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x10a6772; continue;
 }
 0x10a6761 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a6772; continue; }
  pc = 0x10a6767; continue;
 }
 0x10a6767 => {
  { cpu.sub8(cpu.r8(0x12555a0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a675d; continue; }
  pc = 0x10a6770; continue;
 }
 0x10a6770 => {
  cpu.poll(); pc = 0x10a6761; continue;
 }
 0x10a6772 => {
  { let v: u8 = cpu.r8(0x12555a0u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.sub8((cpu.edx as u8), 0xffu8); }
  if !cpu.zf { pc = 0x10a6783; continue; }
  pc = 0x10a677f; continue;
 }
 0x10a677f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x10a6794; continue;
 }
 0x10a6783 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10a6794; continue; }
  pc = 0x10a6789; continue;
 }
 0x10a6789 => {
  { cpu.sub8(cpu.r8(0x12555a0u32.wrapping_add(cpu.eax)), 0xffu8); }
  if cpu.zf { cpu.poll(); pc = 0x10a677f; continue; }
  pc = 0x10a6792; continue;
 }
 0x10a6792 => {
  cpu.poll(); pc = 0x10a6783; continue;
 }
 0x10a6794 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a67fc; continue; }
  pc = 0x10a6799; continue;
 }
 0x10a6799 => {
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10a67fc; continue; }
  pc = 0x10a679e; continue;
 }
 0x10a679e => {
  { let v: u32 = cpu.ecx.wrapping_sub(0x2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.esp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf64(0x116987cu32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.st(0) * cpu.rf64(0x1169884u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x10a67f2; continue; }
  pc = 0x10a67c6; continue;
 }
 0x10a67c6 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf64(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x10a67cd; continue;
 }
 0x10a67cd => {
  { let v = cpu.st(1); cpu.fpush(v); }
  cpu.push(0x10a67d4); f_0109d4ba(cpu); if cpu.returned_to != 0x10a67d4 { return; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x125559fu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10a67cd; continue; }
  pc = 0x10a67ed; continue;
 }
 0x10a67ed => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x10a67f2; continue;
 }
 0x10a67f2 => {
  { let v = 0x12555a0u32; cpu.push(v); }
  cpu.push(0x10a67fc); f_010ca08a(cpu); if cpu.returned_to != 0x10a67fc { return; }
  pc = 0x10a67fc; continue;
 }
 0x10a67fc => {
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6804(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6804); }
 let mut pc: u32 = 0x10a6804;
 loop { match pc {
 0x10a6804 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1255e6cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1255e7cu32, v); }
  { let v: u32 = cpu.r32(0x1255c80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6821); f_010ca161(cpu); if cpu.returned_to != 0x10a6821 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6824(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6824); }
 let mut pc: u32 = 0x10a6824;
 loop { match pc {
 0x10a6824 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a682f); f_010ca157(cpu); if cpu.returned_to != 0x10a682f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6834); }
 let mut pc: u32 = 0x10a6834;
 loop { match pc {
 0x10a6834 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a6845); f_010ca021(cpu); if cpu.returned_to != 0x10a6845 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6848(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6848); }
 let mut pc: u32 = 0x10a6848;
 loop { match pc {
 0x10a6848 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a6851); f_010ca1c5(cpu); if cpu.returned_to != 0x10a6851 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6854(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6854); }
 let mut pc: u32 = 0x10a6854;
 loop { match pc {
 0x10a6854 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6865); f_010ca021(cpu); if cpu.returned_to != 0x10a6865 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6868(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6868); }
 let mut pc: u32 = 0x10a6868;
 loop { match pc {
 0x10a6868 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a6871); f_010ca1c5(cpu); if cpu.returned_to != 0x10a6871 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6874(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6874); }
 let mut pc: u32 = 0x10a6874;
 loop { match pc {
 0x10a6874 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a6885); f_010ca021(cpu); if cpu.returned_to != 0x10a6885 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a6892); f_010c9fe5(cpu); if cpu.returned_to != 0x10a6892 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6899); f_010ca049(cpu); if cpu.returned_to != 0x10a6899 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a689c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a689c); }
 let mut pc: u32 = 0x10a689c;
 loop { match pc {
 0x10a689c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a68ab); f_010c9fe5(cpu); if cpu.returned_to != 0x10a68ab { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a68ba); f_010ca021(cpu); if cpu.returned_to != 0x10a68ba { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a68c1); f_010ca049(cpu); if cpu.returned_to != 0x10a68c1 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a68c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a68c4); }
 let mut pc: u32 = 0x10a68c4;
 loop { match pc {
 0x10a68c4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a68d3); f_010c9fe5(cpu); if cpu.returned_to != 0x10a68d3 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a68da); f_010ca049(cpu); if cpu.returned_to != 0x10a68da { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a68e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a68e0); }
 let mut pc: u32 = 0x10a68e0;
 loop { match pc {
 0x10a68e0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a68ef); f_010c9fe5(cpu); if cpu.returned_to != 0x10a68ef { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a68f6); f_010ca049(cpu); if cpu.returned_to != 0x10a68f6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a68fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a68fc); }
 let mut pc: u32 = 0x10a68fc;
 loop { match pc {
 0x10a68fc => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a690b); f_010c9fe5(cpu); if cpu.returned_to != 0x10a690b { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a691a); f_010ca021(cpu); if cpu.returned_to != 0x10a691a { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6921); f_010ca049(cpu); if cpu.returned_to != 0x10a6921 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6924(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6924); }
 let mut pc: u32 = 0x10a6924;
 loop { match pc {
 0x10a6924 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x10a6933); f_010c9fe5(cpu); if cpu.returned_to != 0x10a6933 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10a6942); f_010ca021(cpu); if cpu.returned_to != 0x10a6942 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6949); f_010ca049(cpu); if cpu.returned_to != 0x10a6949 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a694c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a694c); }
 let mut pc: u32 = 0x10a694c;
 loop { match pc {
 0x10a694c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x10a6955); f_010ca012(cpu); if cpu.returned_to != 0x10a6955 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a695c); f_010ca017(cpu); if cpu.returned_to != 0x10a695c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6960); }
 let mut pc: u32 = 0x10a6960;
 loop { match pc {
 0x10a6960 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6969); f_010ca012(cpu); if cpu.returned_to != 0x10a6969 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a696c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a696c); }
 let mut pc: u32 = 0x10a696c;
 loop { match pc {
 0x10a696c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6974); f_010ca030(cpu); if cpu.returned_to != 0x10a6974 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6978(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6978); }
 let mut pc: u32 = 0x10a6978;
 loop { match pc {
 0x10a6978 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x1255e6cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1255c80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10a6990); f_010ca1a2(cpu); if cpu.returned_to != 0x10a6990 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6994(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6994); }
 let mut pc: u32 = 0x10a6994;
 loop { match pc {
 0x10a6994 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a699b); f_010ca0ad(cpu); if cpu.returned_to != 0x10a699b { return; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1174394u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a69a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a69a8); }
 let mut pc: u32 = 0x10a69a8;
 loop { match pc {
 0x10a69a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a69b6); f_010b30cc(cpu); if cpu.returned_to != 0x10a69b6 { return; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a69c0; continue; }
  pc = 0x10a69bd; continue;
 }
 0x10a69bd => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10a69c0; continue;
 }
 0x10a69c0 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10a69c7); f_010b318f(cpu); if cpu.returned_to != 0x10a69c7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a69cf; continue; }
  pc = 0x10a69cd; continue;
 }
 0x10a69cd => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10a69cf; continue;
 }
 0x10a69cf => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a69d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a69d7); }
 let mut pc: u32 = 0x10a69d7;
 loop { match pc {
 0x10a69d7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x10a69e4); f_010b334f(cpu); if cpu.returned_to != 0x10a69e4 { return; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x10a69fa); f_010b36b3(cpu); if cpu.returned_to != 0x10a69fa { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a6a01); f_010b3947(cpu); if cpu.returned_to != 0x10a6a01 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6a21; continue; }
  pc = 0x10a6a05; continue;
 }
 0x10a6a05 => {
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117baa8u32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x10a6a21); f_010b36b3(cpu); if cpu.returned_to != 0x10a6a21 { return; }
  pc = 0x10a6a21; continue;
 }
 0x10a6a21 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6a28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6a28); }
 let mut pc: u32 = 0x10a6a28;
 loop { match pc {
 0x10a6a28 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1255f24u32; cpu.edx = v; }
  cpu.push(0x10a6a33); f_010a69d7(cpu); if cpu.returned_to != 0x10a6a33 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6a38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6a38); }
 let mut pc: u32 = 0x10a6a38;
 loop { match pc {
 0x10a6a38 => {
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x10a6a45; continue; }
  pc = 0x10a6a3d; continue;
 }
 0x10a6a3d => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10a6a45; continue; }
  pc = 0x10a6a42; continue;
 }
 0x10a6a42 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x10a6a45; continue;
 }
 0x10a6a45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6a48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6a48); }
 let mut pc: u32 = 0x10a6a48;
 loop { match pc {
 0x10a6a48 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x10a6a58); f_010b3c3c(cpu); if cpu.returned_to != 0x10a6a58 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10a6a5f); f_010afff1(cpu); if cpu.returned_to != 0x10a6a5f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6a68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6a68); }
 let mut pc: u32 = 0x10a6a68;
 loop { match pc {
 0x10a6a68 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x10a6a97; continue; }
  pc = 0x10a6a7d; continue;
 }
 0x10a6a7d => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10a6a7f; continue;
 }
 0x10a6a7f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10a6a8f); f_010b3c64(cpu); if cpu.returned_to != 0x10a6a8f { return; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x10a6b90; continue;
 }
 0x10a6a97 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x10a6ae2; continue; }
  pc = 0x10a6aa7; continue;
 }
 0x10a6aa7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { cpu.poll(); pc = 0x10a6a7f; continue; }
  pc = 0x10a6aac; continue;
 }
 0x10a6aac => {
  { let v: u16 = 0x2u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { cpu.fprem(); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.pf() { pc = 0x10a6ad9; continue; }
  pc = 0x10a6ac8; continue;
 }
 0x10a6ac8 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x10a6ad9; continue; }
  pc = 0x10a6ad7; continue;
 }
 0x10a6ad7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10a6ad9; continue;
 }
 0x10a6ad9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xfffffff9u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x10a6aea; continue;
 }
 0x10a6ae2 => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x10a6aea; continue; }
  pc = 0x10a6ae5; continue;
 }
 0x10a6ae5 => {
  pc = 0x10a6b71; continue;
 }
 0x10a6aea => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if !cpu.cf { pc = 0x10a6b71; continue; }
  pc = 0x10a6b02; continue;
 }
 0x10a6b02 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10a6b1c; continue; }
  pc = 0x10a6b11; continue;
 }
 0x10a6b11 => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10a6b1a); f_010a6b9b(cpu); if cpu.returned_to != 0x10a6b1a { return; }
  pc = 0x10a6b47; continue;
 }
 0x10a6b1c => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10a6b71; continue; }
  pc = 0x10a6b20; continue;
 }
 0x10a6b20 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10a6b71; continue; }
  pc = 0x10a6b26; continue;
 }
 0x10a6b26 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10a6b2e); f_010a6b9b(cpu); if cpu.returned_to != 0x10a6b2e { return; }
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x117bb1cu32), 0x1u8); }
  if !cpu.zf { pc = 0x10a6b3d; continue; }
  pc = 0x10a6b39; continue;
 }
 0x10a6b39 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x10a6b47; continue;
 }
 0x10a6b3d => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x10a6b47); f_010b3e7b(cpu); if cpu.returned_to != 0x10a6b47 { return; }
  pc = 0x10a6b47; continue;
 }
 0x10a6b47 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x10a6b6d; continue; }
  pc = 0x10a6b59; continue;
 }
 0x10a6b59 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if !cpu.zf { pc = 0x10a6b6d; continue; }
  pc = 0x10a6b66; continue;
 }
 0x10a6b66 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x10a6a7f; continue;
 }
 0x10a6b6d => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x10a6b90; continue;
 }
 0x10a6b71 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10a6b82); f_010b3cf8(cpu); if cpu.returned_to != 0x10a6b82 { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10a6b66; continue; }
  pc = 0x10a6b86; continue;
 }
 0x10a6b86 => {
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x10a6b8e; continue; }
  pc = 0x10a6b8c; continue;
 }
 0x10a6b8c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x10a6b8e; continue;
 }
 0x10a6b8e => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x10a6b90; continue;
 }
 0x10a6b90 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  {  }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6b9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6b9b); }
 let mut pc: u32 = 0x10a6b9b;
 loop { match pc {
 0x10a6b9b => {
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10a6ba4; continue; }
  pc = 0x10a6ba0; continue;
 }
 0x10a6ba0 => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  cpu.poll(); pc = 0x10a6b9b; continue;
 }
 0x10a6ba4 => {
  if !cpu.cf { pc = 0x10a6bb9; continue; }
  pc = 0x10a6ba6; continue;
 }
 0x10a6ba6 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  pc = 0x10a6ba8; continue;
 }
 0x10a6ba8 => {
  if cpu.zf { pc = 0x10a6bb5; continue; }
  pc = 0x10a6baa; continue;
 }
 0x10a6baa => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x10a6bb3; continue; }
  pc = 0x10a6bb1; continue;
 }
 0x10a6bb1 => {
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  pc = 0x10a6bb3; continue;
 }
 0x10a6bb3 => {
  cpu.poll(); pc = 0x10a6ba8; continue;
 }
 0x10a6bb5 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x10a6bbd; continue;
 }
 0x10a6bb9 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  pc = 0x10a6bbd; continue;
 }
 0x10a6bbd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6bd0); }
 let mut pc: u32 = 0x10a6bd0;
 loop { match pc {
 0x10a6bd0 => {
  { cpu.sub32(cpu.r32(0x117a5d4u32), 0x0u32); }
  if cpu.zf { pc = 0x10a6bf3; continue; }
  pc = 0x10a6bd9; continue;
 }
 0x10a6bd9 => {
  { let v = 0x10a6bd0u32; cpu.push(v); }
  { let v = 0x10aecc0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10a6bea); f_010b4750(cpu); if cpu.returned_to != 0x10a6bea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117a5d4u32, v); }
  pc = 0x10a6bf3; continue;
 }
 0x10a6bf3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6d8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6d8b); }
 let mut pc: u32 = 0x10a6d8b;
 loop { match pc {
 0x10a6d8b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a6d94); cpu.call(t); if cpu.returned_to != 0x10a6d94 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6d9c); f_010aec90(cpu); if cpu.returned_to != 0x10a6d9c { return; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a6da9); f_010bdc80(cpu); if cpu.returned_to != 0x10a6da9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a6df7; continue; }
  pc = 0x10a6db8; continue;
 }
 0x10a6db8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a6dca; continue; }
  pc = 0x10a6dc1; continue;
 }
 0x10a6dc1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a6df7; continue; }
  pc = 0x10a6dca; continue;
 }
 0x10a6dca => {
  cpu.push(0x10a6dcf); f_010b5291(cpu); if cpu.returned_to != 0x10a6dcf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6df7; continue; }
  pc = 0x10a6dd3; continue;
 }
 0x10a6dd3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a6ded); f_010b5523(cpu); if cpu.returned_to != 0x10a6ded { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a6df7); f_010aec40(cpu); if cpu.returned_to != 0x10a6df7 { return; }
  pc = 0x10a6df7; continue;
 }
 0x10a6df7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6e20); }
 let mut pc: u32 = 0x10a6e20;
 loop { match pc {
 0x10a6e20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6e67; continue; }
  pc = 0x10a6e40; continue;
 }
 0x10a6e40 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a6e4e; continue; }
  pc = 0x10a6e45; continue;
 }
 0x10a6e45 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a6e67; continue; }
  pc = 0x10a6e4e; continue;
 }
 0x10a6e4e => {
  cpu.push(0x10a6e53); f_010b5291(cpu); if cpu.returned_to != 0x10a6e53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6e67; continue; }
  pc = 0x10a6e57; continue;
 }
 0x10a6e57 => {
  cpu.push(0x10a6e5c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a6e5c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6e67; continue; }
  pc = 0x10a6e60; continue;
 }
 0x10a6e60 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6e69; continue;
 }
 0x10a6e67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6e69; continue;
 }
 0x10a6e69 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6e9b; continue; }
  pc = 0x10a6e6d; continue;
 }
 0x10a6e6d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116ab10u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a6e93); cpu.call(t); if cpu.returned_to != 0x10a6e93 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6e9b); f_010aec90(cpu); if cpu.returned_to != 0x10a6e9b { return; }
  pc = 0x10a6e9b; continue;
 }
 0x10a6e9b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a6ea8); f_010bdd30(cpu); if cpu.returned_to != 0x10a6ea8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6ed0); }
 let mut pc: u32 = 0x10a6ed0;
 loop { match pc {
 0x10a6ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a6f18; continue; }
  pc = 0x10a6ef1; continue;
 }
 0x10a6ef1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a6eff; continue; }
  pc = 0x10a6ef6; continue;
 }
 0x10a6ef6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a6f18; continue; }
  pc = 0x10a6eff; continue;
 }
 0x10a6eff => {
  cpu.push(0x10a6f04); f_010b5291(cpu); if cpu.returned_to != 0x10a6f04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6f18; continue; }
  pc = 0x10a6f08; continue;
 }
 0x10a6f08 => {
  cpu.push(0x10a6f0d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a6f0d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6f18; continue; }
  pc = 0x10a6f11; continue;
 }
 0x10a6f11 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a6f1a; continue;
 }
 0x10a6f18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a6f1a; continue;
 }
 0x10a6f1a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6f5a; continue; }
  pc = 0x10a6f1e; continue;
 }
 0x10a6f1e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aaf4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a6f52); cpu.call(t); if cpu.returned_to != 0x10a6f52 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a6f5a); f_010aec90(cpu); if cpu.returned_to != 0x10a6f5a { return; }
  pc = 0x10a6f5a; continue;
 }
 0x10a6f5a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a6f6f); f_010bdd80(cpu); if cpu.returned_to != 0x10a6f6f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a6fbc; continue; }
  pc = 0x10a6f7d; continue;
 }
 0x10a6f7d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a6f8f; continue; }
  pc = 0x10a6f86; continue;
 }
 0x10a6f86 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a6fbc; continue; }
  pc = 0x10a6f8f; continue;
 }
 0x10a6f8f => {
  cpu.push(0x10a6f94); f_010b5291(cpu); if cpu.returned_to != 0x10a6f94 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a6fbc; continue; }
  pc = 0x10a6f98; continue;
 }
 0x10a6f98 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a6fb2); f_010b5523(cpu); if cpu.returned_to != 0x10a6fb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a6fbc); f_010aec40(cpu); if cpu.returned_to != 0x10a6fbc { return; }
  pc = 0x10a6fbc; continue;
 }
 0x10a6fbc => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a6fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a6fe0); }
 let mut pc: u32 = 0x10a6fe0;
 loop { match pc {
 0x10a6fe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7027; continue; }
  pc = 0x10a7000; continue;
 }
 0x10a7000 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a700e; continue; }
  pc = 0x10a7005; continue;
 }
 0x10a7005 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7027; continue; }
  pc = 0x10a700e; continue;
 }
 0x10a700e => {
  cpu.push(0x10a7013); f_010b5291(cpu); if cpu.returned_to != 0x10a7013 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7027; continue; }
  pc = 0x10a7017; continue;
 }
 0x10a7017 => {
  cpu.push(0x10a701c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a701c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7027; continue; }
  pc = 0x10a7020; continue;
 }
 0x10a7020 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7029; continue;
 }
 0x10a7027 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7029; continue;
 }
 0x10a7029 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a705b; continue; }
  pc = 0x10a702d; continue;
 }
 0x10a702d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aad8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7053); cpu.call(t); if cpu.returned_to != 0x10a7053 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a705b); f_010aec90(cpu); if cpu.returned_to != 0x10a705b { return; }
  pc = 0x10a705b; continue;
 }
 0x10a705b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7068); f_010bdef0(cpu); if cpu.returned_to != 0x10a7068 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a708b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a708b); } r_010a708b(cpu, 0x10a708b); }
pub fn f_010a7090(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10a7090); } r_010a708b(cpu, 0x10a7090); }
fn r_010a708b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10a708b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x56u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x56u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a70d8; continue; }
  pc = 0x10a70b1; continue;
 }
 0x10a7090 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a70d8; continue; }
  pc = 0x10a70b1; continue;
 }
 0x10a70b1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a70bf; continue; }
  pc = 0x10a70b6; continue;
 }
 0x10a70b6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a70d8; continue; }
  pc = 0x10a70bf; continue;
 }
 0x10a70bf => {
  cpu.push(0x10a70c4); f_010b5291(cpu); if cpu.returned_to != 0x10a70c4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a70d8; continue; }
  pc = 0x10a70c8; continue;
 }
 0x10a70c8 => {
  cpu.push(0x10a70cd); f_010ae8c0(cpu); if cpu.returned_to != 0x10a70cd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a70d8; continue; }
  pc = 0x10a70d1; continue;
 }
 0x10a70d1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a70da; continue;
 }
 0x10a70d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a70da; continue;
 }
 0x10a70da => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a710c; continue; }
  pc = 0x10a70de; continue;
 }
 0x10a70de => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aac0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7104); cpu.call(t); if cpu.returned_to != 0x10a7104 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a710c); f_010aec90(cpu); if cpu.returned_to != 0x10a710c { return; }
  pc = 0x10a710c; continue;
 }
 0x10a710c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7119); f_010bdf90(cpu); if cpu.returned_to != 0x10a7119 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7167; continue; }
  pc = 0x10a7128; continue;
 }
 0x10a7128 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a713a; continue; }
  pc = 0x10a7131; continue;
 }
 0x10a7131 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7167; continue; }
  pc = 0x10a713a; continue;
 }
 0x10a713a => {
  cpu.push(0x10a713f); f_010b5291(cpu); if cpu.returned_to != 0x10a713f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7167; continue; }
  pc = 0x10a7143; continue;
 }
 0x10a7143 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a715d); f_010b5523(cpu); if cpu.returned_to != 0x10a715d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7167); f_010aebf0(cpu); if cpu.returned_to != 0x10a7167 { return; }
  pc = 0x10a7167; continue;
 }
 0x10a7167 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7190); }
 let mut pc: u32 = 0x10a7190;
 loop { match pc {
 0x10a7190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a71d0; continue; }
  pc = 0x10a71a9; continue;
 }
 0x10a71a9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a71b7; continue; }
  pc = 0x10a71ae; continue;
 }
 0x10a71ae => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a71d0; continue; }
  pc = 0x10a71b7; continue;
 }
 0x10a71b7 => {
  cpu.push(0x10a71bc); f_010b5291(cpu); if cpu.returned_to != 0x10a71bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a71d0; continue; }
  pc = 0x10a71c0; continue;
 }
 0x10a71c0 => {
  cpu.push(0x10a71c5); f_010ae8c0(cpu); if cpu.returned_to != 0x10a71c5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a71d0; continue; }
  pc = 0x10a71c9; continue;
 }
 0x10a71c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a71d2; continue;
 }
 0x10a71d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a71d2; continue;
 }
 0x10a71d2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a71fc; continue; }
  pc = 0x10a71d6; continue;
 }
 0x10a71d6 => {
  { let v: u32 = 0x116aaa8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x116aaa8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a71f9); f_010b5523(cpu); if cpu.returned_to != 0x10a71f9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x10a71fc; continue;
 }
 0x10a71fc => {
  cpu.push(0x10a7201); f_010be220(cpu); if cpu.returned_to != 0x10a7201 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7310); }
 let mut pc: u32 = 0x10a7310;
 loop { match pc {
 0x10a7310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7358; continue; }
  pc = 0x10a7331; continue;
 }
 0x10a7331 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a733f; continue; }
  pc = 0x10a7336; continue;
 }
 0x10a7336 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7358; continue; }
  pc = 0x10a733f; continue;
 }
 0x10a733f => {
  cpu.push(0x10a7344); f_010b5291(cpu); if cpu.returned_to != 0x10a7344 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7358; continue; }
  pc = 0x10a7348; continue;
 }
 0x10a7348 => {
  cpu.push(0x10a734d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a734d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7358; continue; }
  pc = 0x10a7351; continue;
 }
 0x10a7351 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a735a; continue;
 }
 0x10a7358 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a735a; continue;
 }
 0x10a735a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a738c; continue; }
  pc = 0x10a735e; continue;
 }
 0x10a735e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aa70u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7384); cpu.call(t); if cpu.returned_to != 0x10a7384 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a738c); f_010aec90(cpu); if cpu.returned_to != 0x10a738c { return; }
  pc = 0x10a738c; continue;
 }
 0x10a738c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7399); f_010bcf80(cpu); if cpu.returned_to != 0x10a7399 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a73e7; continue; }
  pc = 0x10a73a8; continue;
 }
 0x10a73a8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a73ba; continue; }
  pc = 0x10a73b1; continue;
 }
 0x10a73b1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a73e7; continue; }
  pc = 0x10a73ba; continue;
 }
 0x10a73ba => {
  cpu.push(0x10a73bf); f_010b5291(cpu); if cpu.returned_to != 0x10a73bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a73e7; continue; }
  pc = 0x10a73c3; continue;
 }
 0x10a73c3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a73dd); f_010b5523(cpu); if cpu.returned_to != 0x10a73dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a73e7); f_010aec40(cpu); if cpu.returned_to != 0x10a73e7 { return; }
  pc = 0x10a73e7; continue;
 }
 0x10a73e7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7410); }
 let mut pc: u32 = 0x10a7410;
 loop { match pc {
 0x10a7410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7458; continue; }
  pc = 0x10a7431; continue;
 }
 0x10a7431 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a743f; continue; }
  pc = 0x10a7436; continue;
 }
 0x10a7436 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7458; continue; }
  pc = 0x10a743f; continue;
 }
 0x10a743f => {
  cpu.push(0x10a7444); f_010b5291(cpu); if cpu.returned_to != 0x10a7444 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7458; continue; }
  pc = 0x10a7448; continue;
 }
 0x10a7448 => {
  cpu.push(0x10a744d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a744d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7458; continue; }
  pc = 0x10a7451; continue;
 }
 0x10a7451 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a745a; continue;
 }
 0x10a7458 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a745a; continue;
 }
 0x10a745a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a748c; continue; }
  pc = 0x10a745e; continue;
 }
 0x10a745e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aa54u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7484); cpu.call(t); if cpu.returned_to != 0x10a7484 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a748c); f_010aec90(cpu); if cpu.returned_to != 0x10a748c { return; }
  pc = 0x10a748c; continue;
 }
 0x10a748c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7499); f_010bcfd0(cpu); if cpu.returned_to != 0x10a7499 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a74e7; continue; }
  pc = 0x10a74a8; continue;
 }
 0x10a74a8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a74ba; continue; }
  pc = 0x10a74b1; continue;
 }
 0x10a74b1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a74e7; continue; }
  pc = 0x10a74ba; continue;
 }
 0x10a74ba => {
  cpu.push(0x10a74bf); f_010b5291(cpu); if cpu.returned_to != 0x10a74bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a74e7; continue; }
  pc = 0x10a74c3; continue;
 }
 0x10a74c3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a74dd); f_010b5523(cpu); if cpu.returned_to != 0x10a74dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a74e7); f_010aec40(cpu); if cpu.returned_to != 0x10a74e7 { return; }
  pc = 0x10a74e7; continue;
 }
 0x10a74e7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7510); }
 let mut pc: u32 = 0x10a7510;
 loop { match pc {
 0x10a7510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7557; continue; }
  pc = 0x10a7530; continue;
 }
 0x10a7530 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a753e; continue; }
  pc = 0x10a7535; continue;
 }
 0x10a7535 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7557; continue; }
  pc = 0x10a753e; continue;
 }
 0x10a753e => {
  cpu.push(0x10a7543); f_010b5291(cpu); if cpu.returned_to != 0x10a7543 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7557; continue; }
  pc = 0x10a7547; continue;
 }
 0x10a7547 => {
  cpu.push(0x10a754c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a754c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7557; continue; }
  pc = 0x10a7550; continue;
 }
 0x10a7550 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7559; continue;
 }
 0x10a7557 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7559; continue;
 }
 0x10a7559 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a75a7; continue; }
  pc = 0x10a755d; continue;
 }
 0x10a755d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116aa30u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a759f); cpu.call(t); if cpu.returned_to != 0x10a759f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a75a7); f_010aec90(cpu); if cpu.returned_to != 0x10a75a7 { return; }
  pc = 0x10a75a7; continue;
 }
 0x10a75a7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a75c4); f_010bd030(cpu); if cpu.returned_to != 0x10a75c4 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a7612; continue; }
  pc = 0x10a75d3; continue;
 }
 0x10a75d3 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a75e5; continue; }
  pc = 0x10a75dc; continue;
 }
 0x10a75dc => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7612; continue; }
  pc = 0x10a75e5; continue;
 }
 0x10a75e5 => {
  cpu.push(0x10a75ea); f_010b5291(cpu); if cpu.returned_to != 0x10a75ea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7612; continue; }
  pc = 0x10a75ee; continue;
 }
 0x10a75ee => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7608); f_010b5523(cpu); if cpu.returned_to != 0x10a7608 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7612); f_010aec40(cpu); if cpu.returned_to != 0x10a7612 { return; }
  pc = 0x10a7612; continue;
 }
 0x10a7612 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7730); }
 let mut pc: u32 = 0x10a7730;
 loop { match pc {
 0x10a7730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7778; continue; }
  pc = 0x10a7751; continue;
 }
 0x10a7751 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a775f; continue; }
  pc = 0x10a7756; continue;
 }
 0x10a7756 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7778; continue; }
  pc = 0x10a775f; continue;
 }
 0x10a775f => {
  cpu.push(0x10a7764); f_010b5291(cpu); if cpu.returned_to != 0x10a7764 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7778; continue; }
  pc = 0x10a7768; continue;
 }
 0x10a7768 => {
  cpu.push(0x10a776d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a776d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7778; continue; }
  pc = 0x10a7771; continue;
 }
 0x10a7771 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a777a; continue;
 }
 0x10a7778 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a777a; continue;
 }
 0x10a777a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a77ac; continue; }
  pc = 0x10a777e; continue;
 }
 0x10a777e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a9f4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a77a4); cpu.call(t); if cpu.returned_to != 0x10a77a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a77ac); f_010aec90(cpu); if cpu.returned_to != 0x10a77ac { return; }
  pc = 0x10a77ac; continue;
 }
 0x10a77ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a77b9); f_010bd0f0(cpu); if cpu.returned_to != 0x10a77b9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7807; continue; }
  pc = 0x10a77c8; continue;
 }
 0x10a77c8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a77da; continue; }
  pc = 0x10a77d1; continue;
 }
 0x10a77d1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7807; continue; }
  pc = 0x10a77da; continue;
 }
 0x10a77da => {
  cpu.push(0x10a77df); f_010b5291(cpu); if cpu.returned_to != 0x10a77df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7807; continue; }
  pc = 0x10a77e3; continue;
 }
 0x10a77e3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a77fd); f_010b5523(cpu); if cpu.returned_to != 0x10a77fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7807); f_010aec40(cpu); if cpu.returned_to != 0x10a7807 { return; }
  pc = 0x10a7807; continue;
 }
 0x10a7807 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a788b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a788b); }
 let mut pc: u32 = 0x10a788b;
 loop { match pc {
 0x10a788b => {
  { let v: u8 = cpu.r8(0xdc6850e0u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0xdc6850e0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.eax, 0x16a0116u32); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a78a4); cpu.call(t); if cpu.returned_to != 0x10a78a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a78ac); f_010aec90(cpu); if cpu.returned_to != 0x10a78ac { return; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a78b9); f_010bd120(cpu); if cpu.returned_to != 0x10a78b9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7907; continue; }
  pc = 0x10a78c8; continue;
 }
 0x10a78c8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a78da; continue; }
  pc = 0x10a78d1; continue;
 }
 0x10a78d1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7907; continue; }
  pc = 0x10a78da; continue;
 }
 0x10a78da => {
  cpu.push(0x10a78df); f_010b5291(cpu); if cpu.returned_to != 0x10a78df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7907; continue; }
  pc = 0x10a78e3; continue;
 }
 0x10a78e3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a78fd); f_010b5523(cpu); if cpu.returned_to != 0x10a78fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7907); f_010aec40(cpu); if cpu.returned_to != 0x10a7907 { return; }
  pc = 0x10a7907; continue;
 }
 0x10a7907 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7930); }
 let mut pc: u32 = 0x10a7930;
 loop { match pc {
 0x10a7930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7977; continue; }
  pc = 0x10a7950; continue;
 }
 0x10a7950 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a795e; continue; }
  pc = 0x10a7955; continue;
 }
 0x10a7955 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7977; continue; }
  pc = 0x10a795e; continue;
 }
 0x10a795e => {
  cpu.push(0x10a7963); f_010b5291(cpu); if cpu.returned_to != 0x10a7963 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7977; continue; }
  pc = 0x10a7967; continue;
 }
 0x10a7967 => {
  cpu.push(0x10a796c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a796c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7977; continue; }
  pc = 0x10a7970; continue;
 }
 0x10a7970 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7979; continue;
 }
 0x10a7977 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7979; continue;
 }
 0x10a7979 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a79d5; continue; }
  pc = 0x10a797d; continue;
 }
 0x10a797d => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a9a8u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a79cd); cpu.call(t); if cpu.returned_to != 0x10a79cd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a79d5); f_010aec90(cpu); if cpu.returned_to != 0x10a79d5 { return; }
  pc = 0x10a79d5; continue;
 }
 0x10a79d5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10a79fa); f_010bd1d0(cpu); if cpu.returned_to != 0x10a79fa { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7a20); }
 let mut pc: u32 = 0x10a7a20;
 loop { match pc {
 0x10a7a20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7a68; continue; }
  pc = 0x10a7a41; continue;
 }
 0x10a7a41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7a4f; continue; }
  pc = 0x10a7a46; continue;
 }
 0x10a7a46 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7a68; continue; }
  pc = 0x10a7a4f; continue;
 }
 0x10a7a4f => {
  cpu.push(0x10a7a54); f_010b5291(cpu); if cpu.returned_to != 0x10a7a54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7a68; continue; }
  pc = 0x10a7a58; continue;
 }
 0x10a7a58 => {
  cpu.push(0x10a7a5d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7a5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7a68; continue; }
  pc = 0x10a7a61; continue;
 }
 0x10a7a61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7a6a; continue;
 }
 0x10a7a68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7a6a; continue;
 }
 0x10a7a6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7a9c; continue; }
  pc = 0x10a7a6e; continue;
 }
 0x10a7a6e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a98cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7a94); cpu.call(t); if cpu.returned_to != 0x10a7a94 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7a9c); f_010aec90(cpu); if cpu.returned_to != 0x10a7a9c { return; }
  pc = 0x10a7a9c; continue;
 }
 0x10a7a9c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7aa9); f_010bd230(cpu); if cpu.returned_to != 0x10a7aa9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7af7; continue; }
  pc = 0x10a7ab8; continue;
 }
 0x10a7ab8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a7aca; continue; }
  pc = 0x10a7ac1; continue;
 }
 0x10a7ac1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7af7; continue; }
  pc = 0x10a7aca; continue;
 }
 0x10a7aca => {
  cpu.push(0x10a7acf); f_010b5291(cpu); if cpu.returned_to != 0x10a7acf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7af7; continue; }
  pc = 0x10a7ad3; continue;
 }
 0x10a7ad3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a7aed); f_010b5523(cpu); if cpu.returned_to != 0x10a7aed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7af7); f_010aec40(cpu); if cpu.returned_to != 0x10a7af7 { return; }
  pc = 0x10a7af7; continue;
 }
 0x10a7af7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7b20); }
 let mut pc: u32 = 0x10a7b20;
 loop { match pc {
 0x10a7b20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7b68; continue; }
  pc = 0x10a7b41; continue;
 }
 0x10a7b41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7b4f; continue; }
  pc = 0x10a7b46; continue;
 }
 0x10a7b46 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7b68; continue; }
  pc = 0x10a7b4f; continue;
 }
 0x10a7b4f => {
  cpu.push(0x10a7b54); f_010b5291(cpu); if cpu.returned_to != 0x10a7b54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7b68; continue; }
  pc = 0x10a7b58; continue;
 }
 0x10a7b58 => {
  cpu.push(0x10a7b5d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7b5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7b68; continue; }
  pc = 0x10a7b61; continue;
 }
 0x10a7b61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7b6a; continue;
 }
 0x10a7b68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7b6a; continue;
 }
 0x10a7b6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7b9c; continue; }
  pc = 0x10a7b6e; continue;
 }
 0x10a7b6e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a970u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7b94); cpu.call(t); if cpu.returned_to != 0x10a7b94 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7b9c); f_010aec90(cpu); if cpu.returned_to != 0x10a7b9c { return; }
  pc = 0x10a7b9c; continue;
 }
 0x10a7b9c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7ba9); f_010bd250(cpu); if cpu.returned_to != 0x10a7ba9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7bf7; continue; }
  pc = 0x10a7bb8; continue;
 }
 0x10a7bb8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a7bca; continue; }
  pc = 0x10a7bc1; continue;
 }
 0x10a7bc1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7bf7; continue; }
  pc = 0x10a7bca; continue;
 }
 0x10a7bca => {
  cpu.push(0x10a7bcf); f_010b5291(cpu); if cpu.returned_to != 0x10a7bcf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7bf7; continue; }
  pc = 0x10a7bd3; continue;
 }
 0x10a7bd3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a7bed); f_010b5523(cpu); if cpu.returned_to != 0x10a7bed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7bf7); f_010aec40(cpu); if cpu.returned_to != 0x10a7bf7 { return; }
  pc = 0x10a7bf7; continue;
 }
 0x10a7bf7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7c20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7c20); }
 let mut pc: u32 = 0x10a7c20;
 loop { match pc {
 0x10a7c20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7c67; continue; }
  pc = 0x10a7c40; continue;
 }
 0x10a7c40 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7c4e; continue; }
  pc = 0x10a7c45; continue;
 }
 0x10a7c45 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7c67; continue; }
  pc = 0x10a7c4e; continue;
 }
 0x10a7c4e => {
  cpu.push(0x10a7c53); f_010b5291(cpu); if cpu.returned_to != 0x10a7c53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7c67; continue; }
  pc = 0x10a7c57; continue;
 }
 0x10a7c57 => {
  cpu.push(0x10a7c5c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7c5c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7c67; continue; }
  pc = 0x10a7c60; continue;
 }
 0x10a7c60 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7c69; continue;
 }
 0x10a7c67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7c69; continue;
 }
 0x10a7c69 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7c9b; continue; }
  pc = 0x10a7c6d; continue;
 }
 0x10a7c6d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a950u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7c93); cpu.call(t); if cpu.returned_to != 0x10a7c93 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7c9b); f_010aec90(cpu); if cpu.returned_to != 0x10a7c9b { return; }
  pc = 0x10a7c9b; continue;
 }
 0x10a7c9b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7ca8); f_010bd2b0(cpu); if cpu.returned_to != 0x10a7ca8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7d80); }
 let mut pc: u32 = 0x10a7d80;
 loop { match pc {
 0x10a7d80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7dc7; continue; }
  pc = 0x10a7da0; continue;
 }
 0x10a7da0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7dae; continue; }
  pc = 0x10a7da5; continue;
 }
 0x10a7da5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7dc7; continue; }
  pc = 0x10a7dae; continue;
 }
 0x10a7dae => {
  cpu.push(0x10a7db3); f_010b5291(cpu); if cpu.returned_to != 0x10a7db3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7dc7; continue; }
  pc = 0x10a7db7; continue;
 }
 0x10a7db7 => {
  cpu.push(0x10a7dbc); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7dbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7dc7; continue; }
  pc = 0x10a7dc0; continue;
 }
 0x10a7dc0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7dc9; continue;
 }
 0x10a7dc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7dc9; continue;
 }
 0x10a7dc9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7dfb; continue; }
  pc = 0x10a7dcd; continue;
 }
 0x10a7dcd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a918u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7df3); cpu.call(t); if cpu.returned_to != 0x10a7df3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7dfb); f_010aec90(cpu); if cpu.returned_to != 0x10a7dfb { return; }
  pc = 0x10a7dfb; continue;
 }
 0x10a7dfb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7e08); f_010bd2f0(cpu); if cpu.returned_to != 0x10a7e08 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7e30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7e30); }
 let mut pc: u32 = 0x10a7e30;
 loop { match pc {
 0x10a7e30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7e78; continue; }
  pc = 0x10a7e51; continue;
 }
 0x10a7e51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7e5f; continue; }
  pc = 0x10a7e56; continue;
 }
 0x10a7e56 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7e78; continue; }
  pc = 0x10a7e5f; continue;
 }
 0x10a7e5f => {
  cpu.push(0x10a7e64); f_010b5291(cpu); if cpu.returned_to != 0x10a7e64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7e78; continue; }
  pc = 0x10a7e68; continue;
 }
 0x10a7e68 => {
  cpu.push(0x10a7e6d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7e6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7e78; continue; }
  pc = 0x10a7e71; continue;
 }
 0x10a7e71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7e7a; continue;
 }
 0x10a7e78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7e7a; continue;
 }
 0x10a7e7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7eac; continue; }
  pc = 0x10a7e7e; continue;
 }
 0x10a7e7e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a900u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7ea4); cpu.call(t); if cpu.returned_to != 0x10a7ea4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7eac); f_010aec90(cpu); if cpu.returned_to != 0x10a7eac { return; }
  pc = 0x10a7eac; continue;
 }
 0x10a7eac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7eb9); f_010bd340(cpu); if cpu.returned_to != 0x10a7eb9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a7f07; continue; }
  pc = 0x10a7ec8; continue;
 }
 0x10a7ec8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a7eda; continue; }
  pc = 0x10a7ed1; continue;
 }
 0x10a7ed1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7f07; continue; }
  pc = 0x10a7eda; continue;
 }
 0x10a7eda => {
  cpu.push(0x10a7edf); f_010b5291(cpu); if cpu.returned_to != 0x10a7edf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7f07; continue; }
  pc = 0x10a7ee3; continue;
 }
 0x10a7ee3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a7efd); f_010b5523(cpu); if cpu.returned_to != 0x10a7efd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a7f07); f_010aebf0(cpu); if cpu.returned_to != 0x10a7f07 { return; }
  pc = 0x10a7f07; continue;
 }
 0x10a7f07 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a7f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a7f30); }
 let mut pc: u32 = 0x10a7f30;
 loop { match pc {
 0x10a7f30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a7f77; continue; }
  pc = 0x10a7f50; continue;
 }
 0x10a7f50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a7f5e; continue; }
  pc = 0x10a7f55; continue;
 }
 0x10a7f55 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a7f77; continue; }
  pc = 0x10a7f5e; continue;
 }
 0x10a7f5e => {
  cpu.push(0x10a7f63); f_010b5291(cpu); if cpu.returned_to != 0x10a7f63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a7f77; continue; }
  pc = 0x10a7f67; continue;
 }
 0x10a7f67 => {
  cpu.push(0x10a7f6c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a7f6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7f77; continue; }
  pc = 0x10a7f70; continue;
 }
 0x10a7f70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a7f79; continue;
 }
 0x10a7f77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a7f79; continue;
 }
 0x10a7f79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a7fab; continue; }
  pc = 0x10a7f7d; continue;
 }
 0x10a7f7d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a8d8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a7fa3); cpu.call(t); if cpu.returned_to != 0x10a7fa3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a7fab); f_010aec90(cpu); if cpu.returned_to != 0x10a7fab { return; }
  pc = 0x10a7fab; continue;
 }
 0x10a7fab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a7fb8); f_010bc4d0(cpu); if cpu.returned_to != 0x10a7fb8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a83e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a83e0); }
 let mut pc: u32 = 0x10a83e0;
 loop { match pc {
 0x10a83e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8427; continue; }
  pc = 0x10a8400; continue;
 }
 0x10a8400 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a840e; continue; }
  pc = 0x10a8405; continue;
 }
 0x10a8405 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8427; continue; }
  pc = 0x10a840e; continue;
 }
 0x10a840e => {
  cpu.push(0x10a8413); f_010b5291(cpu); if cpu.returned_to != 0x10a8413 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8427; continue; }
  pc = 0x10a8417; continue;
 }
 0x10a8417 => {
  cpu.push(0x10a841c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a841c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8427; continue; }
  pc = 0x10a8420; continue;
 }
 0x10a8420 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a8429; continue;
 }
 0x10a8427 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a8429; continue;
 }
 0x10a8429 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8477; continue; }
  pc = 0x10a842d; continue;
 }
 0x10a842d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a7f4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a846f); cpu.call(t); if cpu.returned_to != 0x10a846f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8477); f_010aec90(cpu); if cpu.returned_to != 0x10a8477 { return; }
  pc = 0x10a8477; continue;
 }
 0x10a8477 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8494); f_010b8c60(cpu); if cpu.returned_to != 0x10a8494 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a84b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a84b0); }
 let mut pc: u32 = 0x10a84b0;
 loop { match pc {
 0x10a84b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a84f7; continue; }
  pc = 0x10a84d0; continue;
 }
 0x10a84d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a84de; continue; }
  pc = 0x10a84d5; continue;
 }
 0x10a84d5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a84f7; continue; }
  pc = 0x10a84de; continue;
 }
 0x10a84de => {
  cpu.push(0x10a84e3); f_010b5291(cpu); if cpu.returned_to != 0x10a84e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a84f7; continue; }
  pc = 0x10a84e7; continue;
 }
 0x10a84e7 => {
  cpu.push(0x10a84ec); f_010ae8c0(cpu); if cpu.returned_to != 0x10a84ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a84f7; continue; }
  pc = 0x10a84f0; continue;
 }
 0x10a84f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a84f9; continue;
 }
 0x10a84f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a84f9; continue;
 }
 0x10a84f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8539; continue; }
  pc = 0x10a84fd; continue;
 }
 0x10a84fd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a7d4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a8531); cpu.call(t); if cpu.returned_to != 0x10a8531 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8539); f_010aec90(cpu); if cpu.returned_to != 0x10a8539 { return; }
  pc = 0x10a8539; continue;
 }
 0x10a8539 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a854e); f_010b8ca0(cpu); if cpu.returned_to != 0x10a854e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8570); }
 let mut pc: u32 = 0x10a8570;
 loop { match pc {
 0x10a8570 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a85b8; continue; }
  pc = 0x10a8591; continue;
 }
 0x10a8591 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a859f; continue; }
  pc = 0x10a8596; continue;
 }
 0x10a8596 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a85b8; continue; }
  pc = 0x10a859f; continue;
 }
 0x10a859f => {
  cpu.push(0x10a85a4); f_010b5291(cpu); if cpu.returned_to != 0x10a85a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a85b8; continue; }
  pc = 0x10a85a8; continue;
 }
 0x10a85a8 => {
  cpu.push(0x10a85ad); f_010ae8c0(cpu); if cpu.returned_to != 0x10a85ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a85b8; continue; }
  pc = 0x10a85b1; continue;
 }
 0x10a85b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a85ba; continue;
 }
 0x10a85b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a85ba; continue;
 }
 0x10a85ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a85ec; continue; }
  pc = 0x10a85be; continue;
 }
 0x10a85be => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a7b8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a85e4); cpu.call(t); if cpu.returned_to != 0x10a85e4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a85ec); f_010aec90(cpu); if cpu.returned_to != 0x10a85ec { return; }
  pc = 0x10a85ec; continue;
 }
 0x10a85ec => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a85f9); f_010b8dc0(cpu); if cpu.returned_to != 0x10a85f9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a8647; continue; }
  pc = 0x10a8608; continue;
 }
 0x10a8608 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a861a; continue; }
  pc = 0x10a8611; continue;
 }
 0x10a8611 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8647; continue; }
  pc = 0x10a861a; continue;
 }
 0x10a861a => {
  cpu.push(0x10a861f); f_010b5291(cpu); if cpu.returned_to != 0x10a861f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8647; continue; }
  pc = 0x10a8623; continue;
 }
 0x10a8623 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a863d); f_010b5523(cpu); if cpu.returned_to != 0x10a863d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a8647); f_010aec40(cpu); if cpu.returned_to != 0x10a8647 { return; }
  pc = 0x10a8647; continue;
 }
 0x10a8647 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8670); }
 let mut pc: u32 = 0x10a8670;
 loop { match pc {
 0x10a8670 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a86b7; continue; }
  pc = 0x10a8690; continue;
 }
 0x10a8690 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a869e; continue; }
  pc = 0x10a8695; continue;
 }
 0x10a8695 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a86b7; continue; }
  pc = 0x10a869e; continue;
 }
 0x10a869e => {
  cpu.push(0x10a86a3); f_010b5291(cpu); if cpu.returned_to != 0x10a86a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a86b7; continue; }
  pc = 0x10a86a7; continue;
 }
 0x10a86a7 => {
  cpu.push(0x10a86ac); f_010ae8c0(cpu); if cpu.returned_to != 0x10a86ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a86b7; continue; }
  pc = 0x10a86b0; continue;
 }
 0x10a86b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a86b9; continue;
 }
 0x10a86b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a86b9; continue;
 }
 0x10a86b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a86f9; continue; }
  pc = 0x10a86bd; continue;
 }
 0x10a86bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a794u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a86f1); cpu.call(t); if cpu.returned_to != 0x10a86f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a86f9); f_010aec90(cpu); if cpu.returned_to != 0x10a86f9 { return; }
  pc = 0x10a86f9; continue;
 }
 0x10a86f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a870e); f_010b8f40(cpu); if cpu.returned_to != 0x10a870e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8730); }
 let mut pc: u32 = 0x10a8730;
 loop { match pc {
 0x10a8730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8778; continue; }
  pc = 0x10a8751; continue;
 }
 0x10a8751 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a875f; continue; }
  pc = 0x10a8756; continue;
 }
 0x10a8756 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8778; continue; }
  pc = 0x10a875f; continue;
 }
 0x10a875f => {
  cpu.push(0x10a8764); f_010b5291(cpu); if cpu.returned_to != 0x10a8764 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8778; continue; }
  pc = 0x10a8768; continue;
 }
 0x10a8768 => {
  cpu.push(0x10a876d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a876d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8778; continue; }
  pc = 0x10a8771; continue;
 }
 0x10a8771 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a877a; continue;
 }
 0x10a8778 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a877a; continue;
 }
 0x10a877a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a87ba; continue; }
  pc = 0x10a877e; continue;
 }
 0x10a877e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a770u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a87b2); cpu.call(t); if cpu.returned_to != 0x10a87b2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a87ba); f_010aec90(cpu); if cpu.returned_to != 0x10a87ba { return; }
  pc = 0x10a87ba; continue;
 }
 0x10a87ba => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a87cf); f_010b8f50(cpu); if cpu.returned_to != 0x10a87cf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a881c; continue; }
  pc = 0x10a87dd; continue;
 }
 0x10a87dd => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a87ef; continue; }
  pc = 0x10a87e6; continue;
 }
 0x10a87e6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a881c; continue; }
  pc = 0x10a87ef; continue;
 }
 0x10a87ef => {
  cpu.push(0x10a87f4); f_010b5291(cpu); if cpu.returned_to != 0x10a87f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a881c; continue; }
  pc = 0x10a87f8; continue;
 }
 0x10a87f8 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a8812); f_010b5523(cpu); if cpu.returned_to != 0x10a8812 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a881c); f_010aec40(cpu); if cpu.returned_to != 0x10a881c { return; }
  pc = 0x10a881c; continue;
 }
 0x10a881c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8840); }
 let mut pc: u32 = 0x10a8840;
 loop { match pc {
 0x10a8840 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8887; continue; }
  pc = 0x10a8860; continue;
 }
 0x10a8860 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a886e; continue; }
  pc = 0x10a8865; continue;
 }
 0x10a8865 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8887; continue; }
  pc = 0x10a886e; continue;
 }
 0x10a886e => {
  cpu.push(0x10a8873); f_010b5291(cpu); if cpu.returned_to != 0x10a8873 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8887; continue; }
  pc = 0x10a8877; continue;
 }
 0x10a8877 => {
  cpu.push(0x10a887c); f_010ae8c0(cpu); if cpu.returned_to != 0x10a887c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8887; continue; }
  pc = 0x10a8880; continue;
 }
 0x10a8880 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a8889; continue;
 }
 0x10a8887 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a8889; continue;
 }
 0x10a8889 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a88d7; continue; }
  pc = 0x10a888d; continue;
 }
 0x10a888d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a744u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a88cf); cpu.call(t); if cpu.returned_to != 0x10a88cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a88d7); f_010aec90(cpu); if cpu.returned_to != 0x10a88d7 { return; }
  pc = 0x10a88d7; continue;
 }
 0x10a88d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a88f4); f_010b8f70(cpu); if cpu.returned_to != 0x10a88f4 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8910); }
 let mut pc: u32 = 0x10a8910;
 loop { match pc {
 0x10a8910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8958; continue; }
  pc = 0x10a8931; continue;
 }
 0x10a8931 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a893f; continue; }
  pc = 0x10a8936; continue;
 }
 0x10a8936 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8958; continue; }
  pc = 0x10a893f; continue;
 }
 0x10a893f => {
  cpu.push(0x10a8944); f_010b5291(cpu); if cpu.returned_to != 0x10a8944 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8958; continue; }
  pc = 0x10a8948; continue;
 }
 0x10a8948 => {
  cpu.push(0x10a894d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a894d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8958; continue; }
  pc = 0x10a8951; continue;
 }
 0x10a8951 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a895a; continue;
 }
 0x10a8958 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a895a; continue;
 }
 0x10a895a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a899a; continue; }
  pc = 0x10a895e; continue;
 }
 0x10a895e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a718u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a8992); cpu.call(t); if cpu.returned_to != 0x10a8992 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a899a); f_010aec90(cpu); if cpu.returned_to != 0x10a899a { return; }
  pc = 0x10a899a; continue;
 }
 0x10a899a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a89af); f_010b8f90(cpu); if cpu.returned_to != 0x10a89af { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a89fc; continue; }
  pc = 0x10a89bd; continue;
 }
 0x10a89bd => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a89cf; continue; }
  pc = 0x10a89c6; continue;
 }
 0x10a89c6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a89fc; continue; }
  pc = 0x10a89cf; continue;
 }
 0x10a89cf => {
  cpu.push(0x10a89d4); f_010b5291(cpu); if cpu.returned_to != 0x10a89d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a89fc; continue; }
  pc = 0x10a89d8; continue;
 }
 0x10a89d8 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a89f2); f_010b5523(cpu); if cpu.returned_to != 0x10a89f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a89fc); f_010aebf0(cpu); if cpu.returned_to != 0x10a89fc { return; }
  pc = 0x10a89fc; continue;
 }
 0x10a89fc => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8a20); }
 let mut pc: u32 = 0x10a8a20;
 loop { match pc {
 0x10a8a20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8a68; continue; }
  pc = 0x10a8a41; continue;
 }
 0x10a8a41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a8a4f; continue; }
  pc = 0x10a8a46; continue;
 }
 0x10a8a46 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8a68; continue; }
  pc = 0x10a8a4f; continue;
 }
 0x10a8a4f => {
  cpu.push(0x10a8a54); f_010b5291(cpu); if cpu.returned_to != 0x10a8a54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8a68; continue; }
  pc = 0x10a8a58; continue;
 }
 0x10a8a58 => {
  cpu.push(0x10a8a5d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a8a5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8a68; continue; }
  pc = 0x10a8a61; continue;
 }
 0x10a8a61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a8a6a; continue;
 }
 0x10a8a68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a8a6a; continue;
 }
 0x10a8a6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8aaa; continue; }
  pc = 0x10a8a6e; continue;
 }
 0x10a8a6e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a6ecu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a8aa2); cpu.call(t); if cpu.returned_to != 0x10a8aa2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8aaa); f_010aec90(cpu); if cpu.returned_to != 0x10a8aaa { return; }
  pc = 0x10a8aaa; continue;
 }
 0x10a8aaa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a8abf); f_010b8fb0(cpu); if cpu.returned_to != 0x10a8abf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8b0c; continue; }
  pc = 0x10a8acd; continue;
 }
 0x10a8acd => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a8adf; continue; }
  pc = 0x10a8ad6; continue;
 }
 0x10a8ad6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8b0c; continue; }
  pc = 0x10a8adf; continue;
 }
 0x10a8adf => {
  cpu.push(0x10a8ae4); f_010b5291(cpu); if cpu.returned_to != 0x10a8ae4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8b0c; continue; }
  pc = 0x10a8ae8; continue;
 }
 0x10a8ae8 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a8b02); f_010b5523(cpu); if cpu.returned_to != 0x10a8b02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a8b0c); f_010aebf0(cpu); if cpu.returned_to != 0x10a8b0c { return; }
  pc = 0x10a8b0c; continue;
 }
 0x10a8b0c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8c40); }
 let mut pc: u32 = 0x10a8c40;
 loop { match pc {
 0x10a8c40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8c88; continue; }
  pc = 0x10a8c61; continue;
 }
 0x10a8c61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a8c6f; continue; }
  pc = 0x10a8c66; continue;
 }
 0x10a8c66 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8c88; continue; }
  pc = 0x10a8c6f; continue;
 }
 0x10a8c6f => {
  cpu.push(0x10a8c74); f_010b5291(cpu); if cpu.returned_to != 0x10a8c74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8c88; continue; }
  pc = 0x10a8c78; continue;
 }
 0x10a8c78 => {
  cpu.push(0x10a8c7d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a8c7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8c88; continue; }
  pc = 0x10a8c81; continue;
 }
 0x10a8c81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a8c8a; continue;
 }
 0x10a8c88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a8c8a; continue;
 }
 0x10a8c8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8cca; continue; }
  pc = 0x10a8c8e; continue;
 }
 0x10a8c8e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a698u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a8cc2); cpu.call(t); if cpu.returned_to != 0x10a8cc2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8cca); f_010aec90(cpu); if cpu.returned_to != 0x10a8cca { return; }
  pc = 0x10a8cca; continue;
 }
 0x10a8cca => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a8cdf); f_010b8ff0(cpu); if cpu.returned_to != 0x10a8cdf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8d2c; continue; }
  pc = 0x10a8ced; continue;
 }
 0x10a8ced => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a8cff; continue; }
  pc = 0x10a8cf6; continue;
 }
 0x10a8cf6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8d2c; continue; }
  pc = 0x10a8cff; continue;
 }
 0x10a8cff => {
  cpu.push(0x10a8d04); f_010b5291(cpu); if cpu.returned_to != 0x10a8d04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8d2c; continue; }
  pc = 0x10a8d08; continue;
 }
 0x10a8d08 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a8d22); f_010b5523(cpu); if cpu.returned_to != 0x10a8d22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a8d2c); f_010aebf0(cpu); if cpu.returned_to != 0x10a8d2c { return; }
  pc = 0x10a8d2c; continue;
 }
 0x10a8d2c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a8f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a8f70); }
 let mut pc: u32 = 0x10a8f70;
 loop { match pc {
 0x10a8f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a8fb7; continue; }
  pc = 0x10a8f90; continue;
 }
 0x10a8f90 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a8f9e; continue; }
  pc = 0x10a8f95; continue;
 }
 0x10a8f95 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a8fb7; continue; }
  pc = 0x10a8f9e; continue;
 }
 0x10a8f9e => {
  cpu.push(0x10a8fa3); f_010b5291(cpu); if cpu.returned_to != 0x10a8fa3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a8fb7; continue; }
  pc = 0x10a8fa7; continue;
 }
 0x10a8fa7 => {
  cpu.push(0x10a8fac); f_010ae8c0(cpu); if cpu.returned_to != 0x10a8fac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8fb7; continue; }
  pc = 0x10a8fb0; continue;
 }
 0x10a8fb0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a8fb9; continue;
 }
 0x10a8fb7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a8fb9; continue;
 }
 0x10a8fb9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a8ff9; continue; }
  pc = 0x10a8fbd; continue;
 }
 0x10a8fbd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a624u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a8ff1); cpu.call(t); if cpu.returned_to != 0x10a8ff1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a8ff9); f_010aec90(cpu); if cpu.returned_to != 0x10a8ff9 { return; }
  pc = 0x10a8ff9; continue;
 }
 0x10a8ff9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a900e); f_010b9050(cpu); if cpu.returned_to != 0x10a900e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a9030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a9030); }
 let mut pc: u32 = 0x10a9030;
 loop { match pc {
 0x10a9030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a9086; continue; }
  pc = 0x10a905f; continue;
 }
 0x10a905f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a906d; continue; }
  pc = 0x10a9064; continue;
 }
 0x10a9064 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a9086; continue; }
  pc = 0x10a906d; continue;
 }
 0x10a906d => {
  cpu.push(0x10a9072); f_010b5291(cpu); if cpu.returned_to != 0x10a9072 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a9086; continue; }
  pc = 0x10a9076; continue;
 }
 0x10a9076 => {
  cpu.push(0x10a907b); f_010ae8c0(cpu); if cpu.returned_to != 0x10a907b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a9086; continue; }
  pc = 0x10a907f; continue;
 }
 0x10a907f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a9088; continue;
 }
 0x10a9086 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a9088; continue;
 }
 0x10a9088 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a90c8; continue; }
  pc = 0x10a908c; continue;
 }
 0x10a908c => {
  { let v: u32 = cpu.r32(0x128u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a5e8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a90c0); cpu.call(t); if cpu.returned_to != 0x10a90c0 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a90c8); f_010aec90(cpu); if cpu.returned_to != 0x10a90c8 { return; }
  pc = 0x10a90c8; continue;
 }
 0x10a90c8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a90d7); f_010b90e0(cpu); if cpu.returned_to != 0x10a90d7 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10a915e; continue; }
  pc = 0x10a90e8; continue;
 }
 0x10a90e8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a90fa; continue; }
  pc = 0x10a90f1; continue;
 }
 0x10a90f1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a915e; continue; }
  pc = 0x10a90fa; continue;
 }
 0x10a90fa => {
  cpu.push(0x10a90ff); f_010b5291(cpu); if cpu.returned_to != 0x10a90ff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a915e; continue; }
  pc = 0x10a9103; continue;
 }
 0x10a9103 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a911d); f_010b5523(cpu); if cpu.returned_to != 0x10a911d { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a610u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a9151); cpu.call(t); if cpu.returned_to != 0x10a9151 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a915e); f_010aec90(cpu); if cpu.returned_to != 0x10a915e { return; }
  pc = 0x10a915e; continue;
 }
 0x10a915e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a9170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a9170); }
 let mut pc: u32 = 0x10a9170;
 loop { match pc {
 0x10a9170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a91b8; continue; }
  pc = 0x10a9191; continue;
 }
 0x10a9191 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a919f; continue; }
  pc = 0x10a9196; continue;
 }
 0x10a9196 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a91b8; continue; }
  pc = 0x10a919f; continue;
 }
 0x10a919f => {
  cpu.push(0x10a91a4); f_010b5291(cpu); if cpu.returned_to != 0x10a91a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a91b8; continue; }
  pc = 0x10a91a8; continue;
 }
 0x10a91a8 => {
  cpu.push(0x10a91ad); f_010ae8c0(cpu); if cpu.returned_to != 0x10a91ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a91b8; continue; }
  pc = 0x10a91b1; continue;
 }
 0x10a91b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a91ba; continue;
 }
 0x10a91b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a91ba; continue;
 }
 0x10a91ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a91fa; continue; }
  pc = 0x10a91be; continue;
 }
 0x10a91be => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a5c8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a91f2); cpu.call(t); if cpu.returned_to != 0x10a91f2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a91fa); f_010aec90(cpu); if cpu.returned_to != 0x10a91fa { return; }
  pc = 0x10a91fa; continue;
 }
 0x10a91fa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10a920f); f_010b9160(cpu); if cpu.returned_to != 0x10a920f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a925c; continue; }
  pc = 0x10a921d; continue;
 }
 0x10a921d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a922f; continue; }
  pc = 0x10a9226; continue;
 }
 0x10a9226 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a925c; continue; }
  pc = 0x10a922f; continue;
 }
 0x10a922f => {
  cpu.push(0x10a9234); f_010b5291(cpu); if cpu.returned_to != 0x10a9234 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a925c; continue; }
  pc = 0x10a9238; continue;
 }
 0x10a9238 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10a9252); f_010b5523(cpu); if cpu.returned_to != 0x10a9252 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a925c); f_010aec40(cpu); if cpu.returned_to != 0x10a925c { return; }
  pc = 0x10a925c; continue;
 }
 0x10a925c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010a9880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a9880); }
 let mut pc: u32 = 0x10a9880;
 loop { match pc {
 0x10a9880 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a98c7; continue; }
  pc = 0x10a98a0; continue;
 }
 0x10a98a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a98ae; continue; }
  pc = 0x10a98a5; continue;
 }
 0x10a98a5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a98c7; continue; }
  pc = 0x10a98ae; continue;
 }
 0x10a98ae => {
  cpu.push(0x10a98b3); f_010b5291(cpu); if cpu.returned_to != 0x10a98b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a98c7; continue; }
  pc = 0x10a98b7; continue;
 }
 0x10a98b7 => {
  cpu.push(0x10a98bc); f_010ae8c0(cpu); if cpu.returned_to != 0x10a98bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a98c7; continue; }
  pc = 0x10a98c0; continue;
 }
 0x10a98c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a98c9; continue;
 }
 0x10a98c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a98c9; continue;
 }
 0x10a98c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a98fb; continue; }
  pc = 0x10a98cd; continue;
 }
 0x10a98cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a4d0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a98f3); cpu.call(t); if cpu.returned_to != 0x10a98f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a98fb); f_010aec90(cpu); if cpu.returned_to != 0x10a98fb { return; }
  pc = 0x10a98fb; continue;
 }
 0x10a98fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a9908); f_010b9bf0(cpu); if cpu.returned_to != 0x10a9908 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010a9930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10a9930); }
 let mut pc: u32 = 0x10a9930;
 loop { match pc {
 0x10a9930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10a9978; continue; }
  pc = 0x10a9951; continue;
 }
 0x10a9951 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10a995f; continue; }
  pc = 0x10a9956; continue;
 }
 0x10a9956 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a9978; continue; }
  pc = 0x10a995f; continue;
 }
 0x10a995f => {
  cpu.push(0x10a9964); f_010b5291(cpu); if cpu.returned_to != 0x10a9964 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a9978; continue; }
  pc = 0x10a9968; continue;
 }
 0x10a9968 => {
  cpu.push(0x10a996d); f_010ae8c0(cpu); if cpu.returned_to != 0x10a996d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a9978; continue; }
  pc = 0x10a9971; continue;
 }
 0x10a9971 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10a997a; continue;
 }
 0x10a9978 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10a997a; continue;
 }
 0x10a997a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10a99ac; continue; }
  pc = 0x10a997e; continue;
 }
 0x10a997e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a4b4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10a99a4); cpu.call(t); if cpu.returned_to != 0x10a99a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10a99ac); f_010aec90(cpu); if cpu.returned_to != 0x10a99ac { return; }
  pc = 0x10a99ac; continue;
 }
 0x10a99ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10a99b9); f_010b9c70(cpu); if cpu.returned_to != 0x10a99b9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10a9a07; continue; }
  pc = 0x10a99c8; continue;
 }
 0x10a99c8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10a99da; continue; }
  pc = 0x10a99d1; continue;
 }
 0x10a99d1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10a9a07; continue; }
  pc = 0x10a99da; continue;
 }
 0x10a99da => {
  cpu.push(0x10a99df); f_010b5291(cpu); if cpu.returned_to != 0x10a99df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10a9a07; continue; }
  pc = 0x10a99e3; continue;
 }
 0x10a99e3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10a99fd); f_010b5523(cpu); if cpu.returned_to != 0x10a99fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10a9a07); f_010aec40(cpu); if cpu.returned_to != 0x10a9a07 { return; }
  pc = 0x10a9a07; continue;
 }
 0x10a9a07 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aa010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa010); }
 let mut pc: u32 = 0x10aa010;
 loop { match pc {
 0x10aa010 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa058; continue; }
  pc = 0x10aa031; continue;
 }
 0x10aa031 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa03f; continue; }
  pc = 0x10aa036; continue;
 }
 0x10aa036 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa058; continue; }
  pc = 0x10aa03f; continue;
 }
 0x10aa03f => {
  cpu.push(0x10aa044); f_010b5291(cpu); if cpu.returned_to != 0x10aa044 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa058; continue; }
  pc = 0x10aa048; continue;
 }
 0x10aa048 => {
  cpu.push(0x10aa04d); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa04d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa058; continue; }
  pc = 0x10aa051; continue;
 }
 0x10aa051 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa05a; continue;
 }
 0x10aa058 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa05a; continue;
 }
 0x10aa05a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa08c; continue; }
  pc = 0x10aa05e; continue;
 }
 0x10aa05e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a3c8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa084); cpu.call(t); if cpu.returned_to != 0x10aa084 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa08c); f_010aec90(cpu); if cpu.returned_to != 0x10aa08c { return; }
  pc = 0x10aa08c; continue;
 }
 0x10aa08c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa099); f_010b9450(cpu); if cpu.returned_to != 0x10aa099 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10aa0e7; continue; }
  pc = 0x10aa0a8; continue;
 }
 0x10aa0a8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aa0ba; continue; }
  pc = 0x10aa0b1; continue;
 }
 0x10aa0b1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa0e7; continue; }
  pc = 0x10aa0ba; continue;
 }
 0x10aa0ba => {
  cpu.push(0x10aa0bf); f_010b5291(cpu); if cpu.returned_to != 0x10aa0bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa0e7; continue; }
  pc = 0x10aa0c3; continue;
 }
 0x10aa0c3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10aa0dd); f_010b5523(cpu); if cpu.returned_to != 0x10aa0dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aa0e7); f_010aec40(cpu); if cpu.returned_to != 0x10aa0e7 { return; }
  pc = 0x10aa0e7; continue;
 }
 0x10aa0e7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa110); }
 let mut pc: u32 = 0x10aa110;
 loop { match pc {
 0x10aa110 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa157; continue; }
  pc = 0x10aa130; continue;
 }
 0x10aa130 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa13e; continue; }
  pc = 0x10aa135; continue;
 }
 0x10aa135 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa157; continue; }
  pc = 0x10aa13e; continue;
 }
 0x10aa13e => {
  cpu.push(0x10aa143); f_010b5291(cpu); if cpu.returned_to != 0x10aa143 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa157; continue; }
  pc = 0x10aa147; continue;
 }
 0x10aa147 => {
  cpu.push(0x10aa14c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa14c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa157; continue; }
  pc = 0x10aa150; continue;
 }
 0x10aa150 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa159; continue;
 }
 0x10aa157 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa159; continue;
 }
 0x10aa159 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa199; continue; }
  pc = 0x10aa15d; continue;
 }
 0x10aa15d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a3a0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa191); cpu.call(t); if cpu.returned_to != 0x10aa191 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa199); f_010aec90(cpu); if cpu.returned_to != 0x10aa199 { return; }
  pc = 0x10aa199; continue;
 }
 0x10aa199 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10aa1ae); f_010b9550(cpu); if cpu.returned_to != 0x10aa1ae { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa1d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa1d0); }
 let mut pc: u32 = 0x10aa1d0;
 loop { match pc {
 0x10aa1d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa217; continue; }
  pc = 0x10aa1f0; continue;
 }
 0x10aa1f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa1fe; continue; }
  pc = 0x10aa1f5; continue;
 }
 0x10aa1f5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa217; continue; }
  pc = 0x10aa1fe; continue;
 }
 0x10aa1fe => {
  cpu.push(0x10aa203); f_010b5291(cpu); if cpu.returned_to != 0x10aa203 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa217; continue; }
  pc = 0x10aa207; continue;
 }
 0x10aa207 => {
  cpu.push(0x10aa20c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa20c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa217; continue; }
  pc = 0x10aa210; continue;
 }
 0x10aa210 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa219; continue;
 }
 0x10aa217 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa219; continue;
 }
 0x10aa219 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa267; continue; }
  pc = 0x10aa21d; continue;
 }
 0x10aa21d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a378u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa25f); cpu.call(t); if cpu.returned_to != 0x10aa25f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa267); f_010aec90(cpu); if cpu.returned_to != 0x10aa267 { return; }
  pc = 0x10aa267; continue;
 }
 0x10aa267 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa284); f_010b9460(cpu); if cpu.returned_to != 0x10aa284 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aa2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa2a0); }
 let mut pc: u32 = 0x10aa2a0;
 loop { match pc {
 0x10aa2a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa2e7; continue; }
  pc = 0x10aa2c0; continue;
 }
 0x10aa2c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa2ce; continue; }
  pc = 0x10aa2c5; continue;
 }
 0x10aa2c5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa2e7; continue; }
  pc = 0x10aa2ce; continue;
 }
 0x10aa2ce => {
  cpu.push(0x10aa2d3); f_010b5291(cpu); if cpu.returned_to != 0x10aa2d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa2e7; continue; }
  pc = 0x10aa2d7; continue;
 }
 0x10aa2d7 => {
  cpu.push(0x10aa2dc); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa2dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa2e7; continue; }
  pc = 0x10aa2e0; continue;
 }
 0x10aa2e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa2e9; continue;
 }
 0x10aa2e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa2e9; continue;
 }
 0x10aa2e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa337; continue; }
  pc = 0x10aa2ed; continue;
 }
 0x10aa2ed => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a350u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa32f); cpu.call(t); if cpu.returned_to != 0x10aa32f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa337); f_010aec90(cpu); if cpu.returned_to != 0x10aa337 { return; }
  pc = 0x10aa337; continue;
 }
 0x10aa337 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa354); f_010b94e0(cpu); if cpu.returned_to != 0x10aa354 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa370); }
 let mut pc: u32 = 0x10aa370;
 loop { match pc {
 0x10aa370 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa3b7; continue; }
  pc = 0x10aa390; continue;
 }
 0x10aa390 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa39e; continue; }
  pc = 0x10aa395; continue;
 }
 0x10aa395 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa3b7; continue; }
  pc = 0x10aa39e; continue;
 }
 0x10aa39e => {
  cpu.push(0x10aa3a3); f_010b5291(cpu); if cpu.returned_to != 0x10aa3a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa3b7; continue; }
  pc = 0x10aa3a7; continue;
 }
 0x10aa3a7 => {
  cpu.push(0x10aa3ac); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa3ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa3b7; continue; }
  pc = 0x10aa3b0; continue;
 }
 0x10aa3b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa3b9; continue;
 }
 0x10aa3b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa3b9; continue;
 }
 0x10aa3b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa3eb; continue; }
  pc = 0x10aa3bd; continue;
 }
 0x10aa3bd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a334u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa3e3); cpu.call(t); if cpu.returned_to != 0x10aa3e3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa3eb); f_010aec90(cpu); if cpu.returned_to != 0x10aa3eb { return; }
  pc = 0x10aa3eb; continue;
 }
 0x10aa3eb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa3f8); f_010b9560(cpu); if cpu.returned_to != 0x10aa3f8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa420); }
 let mut pc: u32 = 0x10aa420;
 loop { match pc {
 0x10aa420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa467; continue; }
  pc = 0x10aa440; continue;
 }
 0x10aa440 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa44e; continue; }
  pc = 0x10aa445; continue;
 }
 0x10aa445 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa467; continue; }
  pc = 0x10aa44e; continue;
 }
 0x10aa44e => {
  cpu.push(0x10aa453); f_010b5291(cpu); if cpu.returned_to != 0x10aa453 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa467; continue; }
  pc = 0x10aa457; continue;
 }
 0x10aa457 => {
  cpu.push(0x10aa45c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa45c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa467; continue; }
  pc = 0x10aa460; continue;
 }
 0x10aa460 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa469; continue;
 }
 0x10aa467 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa469; continue;
 }
 0x10aa469 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa49b; continue; }
  pc = 0x10aa46d; continue;
 }
 0x10aa46d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a318u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa493); cpu.call(t); if cpu.returned_to != 0x10aa493 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa49b); f_010aec90(cpu); if cpu.returned_to != 0x10aa49b { return; }
  pc = 0x10aa49b; continue;
 }
 0x10aa49b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa4a8); f_010b95a0(cpu); if cpu.returned_to != 0x10aa4a8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa4d0); }
 let mut pc: u32 = 0x10aa4d0;
 loop { match pc {
 0x10aa4d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa517; continue; }
  pc = 0x10aa4f0; continue;
 }
 0x10aa4f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa4fe; continue; }
  pc = 0x10aa4f5; continue;
 }
 0x10aa4f5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa517; continue; }
  pc = 0x10aa4fe; continue;
 }
 0x10aa4fe => {
  cpu.push(0x10aa503); f_010b5291(cpu); if cpu.returned_to != 0x10aa503 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa517; continue; }
  pc = 0x10aa507; continue;
 }
 0x10aa507 => {
  cpu.push(0x10aa50c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa50c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa517; continue; }
  pc = 0x10aa510; continue;
 }
 0x10aa510 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa519; continue;
 }
 0x10aa517 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa519; continue;
 }
 0x10aa519 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa54b; continue; }
  pc = 0x10aa51d; continue;
 }
 0x10aa51d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a2fcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa543); cpu.call(t); if cpu.returned_to != 0x10aa543 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa54b); f_010aec90(cpu); if cpu.returned_to != 0x10aa54b { return; }
  pc = 0x10aa54b; continue;
 }
 0x10aa54b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa558); f_010b9600(cpu); if cpu.returned_to != 0x10aa558 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa580); }
 let mut pc: u32 = 0x10aa580;
 loop { match pc {
 0x10aa580 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa5c7; continue; }
  pc = 0x10aa5a0; continue;
 }
 0x10aa5a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa5ae; continue; }
  pc = 0x10aa5a5; continue;
 }
 0x10aa5a5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa5c7; continue; }
  pc = 0x10aa5ae; continue;
 }
 0x10aa5ae => {
  cpu.push(0x10aa5b3); f_010b5291(cpu); if cpu.returned_to != 0x10aa5b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa5c7; continue; }
  pc = 0x10aa5b7; continue;
 }
 0x10aa5b7 => {
  cpu.push(0x10aa5bc); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa5bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa5c7; continue; }
  pc = 0x10aa5c0; continue;
 }
 0x10aa5c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa5c9; continue;
 }
 0x10aa5c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa5c9; continue;
 }
 0x10aa5c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa5fb; continue; }
  pc = 0x10aa5cd; continue;
 }
 0x10aa5cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a2e0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa5f3); cpu.call(t); if cpu.returned_to != 0x10aa5f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa5fb); f_010aec90(cpu); if cpu.returned_to != 0x10aa5fb { return; }
  pc = 0x10aa5fb; continue;
 }
 0x10aa5fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa608); f_010b96b0(cpu); if cpu.returned_to != 0x10aa608 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa630); }
 let mut pc: u32 = 0x10aa630;
 loop { match pc {
 0x10aa630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa677; continue; }
  pc = 0x10aa650; continue;
 }
 0x10aa650 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa65e; continue; }
  pc = 0x10aa655; continue;
 }
 0x10aa655 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa677; continue; }
  pc = 0x10aa65e; continue;
 }
 0x10aa65e => {
  cpu.push(0x10aa663); f_010b5291(cpu); if cpu.returned_to != 0x10aa663 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa677; continue; }
  pc = 0x10aa667; continue;
 }
 0x10aa667 => {
  cpu.push(0x10aa66c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa66c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa677; continue; }
  pc = 0x10aa670; continue;
 }
 0x10aa670 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa679; continue;
 }
 0x10aa677 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa679; continue;
 }
 0x10aa679 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa6c7; continue; }
  pc = 0x10aa67d; continue;
 }
 0x10aa67d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a2bcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa6bf); cpu.call(t); if cpu.returned_to != 0x10aa6bf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa6c7); f_010aec90(cpu); if cpu.returned_to != 0x10aa6c7 { return; }
  pc = 0x10aa6c7; continue;
 }
 0x10aa6c7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa6e4); f_010b96f0(cpu); if cpu.returned_to != 0x10aa6e4 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10aa732; continue; }
  pc = 0x10aa6f3; continue;
 }
 0x10aa6f3 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aa705; continue; }
  pc = 0x10aa6fc; continue;
 }
 0x10aa6fc => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa732; continue; }
  pc = 0x10aa705; continue;
 }
 0x10aa705 => {
  cpu.push(0x10aa70a); f_010b5291(cpu); if cpu.returned_to != 0x10aa70a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa732; continue; }
  pc = 0x10aa70e; continue;
 }
 0x10aa70e => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa728); f_010b5523(cpu); if cpu.returned_to != 0x10aa728 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aa732); f_010aec40(cpu); if cpu.returned_to != 0x10aa732 { return; }
  pc = 0x10aa732; continue;
 }
 0x10aa732 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa750); }
 let mut pc: u32 = 0x10aa750;
 loop { match pc {
 0x10aa750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa797; continue; }
  pc = 0x10aa770; continue;
 }
 0x10aa770 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa77e; continue; }
  pc = 0x10aa775; continue;
 }
 0x10aa775 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa797; continue; }
  pc = 0x10aa77e; continue;
 }
 0x10aa77e => {
  cpu.push(0x10aa783); f_010b5291(cpu); if cpu.returned_to != 0x10aa783 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa797; continue; }
  pc = 0x10aa787; continue;
 }
 0x10aa787 => {
  cpu.push(0x10aa78c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa78c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa797; continue; }
  pc = 0x10aa790; continue;
 }
 0x10aa790 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa799; continue;
 }
 0x10aa797 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa799; continue;
 }
 0x10aa799 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa7cb; continue; }
  pc = 0x10aa79d; continue;
 }
 0x10aa79d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a298u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa7c3); cpu.call(t); if cpu.returned_to != 0x10aa7c3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa7cb); f_010aec90(cpu); if cpu.returned_to != 0x10aa7cb { return; }
  pc = 0x10aa7cb; continue;
 }
 0x10aa7cb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa7d8); f_010b9b40(cpu); if cpu.returned_to != 0x10aa7d8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa800); }
 let mut pc: u32 = 0x10aa800;
 loop { match pc {
 0x10aa800 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa848; continue; }
  pc = 0x10aa821; continue;
 }
 0x10aa821 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa82f; continue; }
  pc = 0x10aa826; continue;
 }
 0x10aa826 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa848; continue; }
  pc = 0x10aa82f; continue;
 }
 0x10aa82f => {
  cpu.push(0x10aa834); f_010b5291(cpu); if cpu.returned_to != 0x10aa834 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa848; continue; }
  pc = 0x10aa838; continue;
 }
 0x10aa838 => {
  cpu.push(0x10aa83d); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa83d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa848; continue; }
  pc = 0x10aa841; continue;
 }
 0x10aa841 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa84a; continue;
 }
 0x10aa848 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa84a; continue;
 }
 0x10aa84a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa87c; continue; }
  pc = 0x10aa84e; continue;
 }
 0x10aa84e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a270u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa874); cpu.call(t); if cpu.returned_to != 0x10aa874 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa87c); f_010aec90(cpu); if cpu.returned_to != 0x10aa87c { return; }
  pc = 0x10aa87c; continue;
 }
 0x10aa87c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa889); f_010b9b60(cpu); if cpu.returned_to != 0x10aa889 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10aa8d7; continue; }
  pc = 0x10aa898; continue;
 }
 0x10aa898 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aa8aa; continue; }
  pc = 0x10aa8a1; continue;
 }
 0x10aa8a1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa8d7; continue; }
  pc = 0x10aa8aa; continue;
 }
 0x10aa8aa => {
  cpu.push(0x10aa8af); f_010b5291(cpu); if cpu.returned_to != 0x10aa8af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa8d7; continue; }
  pc = 0x10aa8b3; continue;
 }
 0x10aa8b3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10aa8cd); f_010b5523(cpu); if cpu.returned_to != 0x10aa8cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aa8d7); f_010aebf0(cpu); if cpu.returned_to != 0x10aa8d7 { return; }
  pc = 0x10aa8d7; continue;
 }
 0x10aa8d7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aa900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa900); }
 let mut pc: u32 = 0x10aa900;
 loop { match pc {
 0x10aa900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa947; continue; }
  pc = 0x10aa920; continue;
 }
 0x10aa920 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa92e; continue; }
  pc = 0x10aa925; continue;
 }
 0x10aa925 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa947; continue; }
  pc = 0x10aa92e; continue;
 }
 0x10aa92e => {
  cpu.push(0x10aa933); f_010b5291(cpu); if cpu.returned_to != 0x10aa933 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa947; continue; }
  pc = 0x10aa937; continue;
 }
 0x10aa937 => {
  cpu.push(0x10aa93c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa93c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa947; continue; }
  pc = 0x10aa940; continue;
 }
 0x10aa940 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa949; continue;
 }
 0x10aa947 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa949; continue;
 }
 0x10aa949 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa97b; continue; }
  pc = 0x10aa94d; continue;
 }
 0x10aa94d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a24cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aa973); cpu.call(t); if cpu.returned_to != 0x10aa973 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aa97b); f_010aec90(cpu); if cpu.returned_to != 0x10aa97b { return; }
  pc = 0x10aa97b; continue;
 }
 0x10aa97b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aa988); f_010b9dd0(cpu); if cpu.returned_to != 0x10aa988 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aa9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aa9b0); }
 let mut pc: u32 = 0x10aa9b0;
 loop { match pc {
 0x10aa9b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aa9f8; continue; }
  pc = 0x10aa9d1; continue;
 }
 0x10aa9d1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aa9df; continue; }
  pc = 0x10aa9d6; continue;
 }
 0x10aa9d6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aa9f8; continue; }
  pc = 0x10aa9df; continue;
 }
 0x10aa9df => {
  cpu.push(0x10aa9e4); f_010b5291(cpu); if cpu.returned_to != 0x10aa9e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aa9f8; continue; }
  pc = 0x10aa9e8; continue;
 }
 0x10aa9e8 => {
  cpu.push(0x10aa9ed); f_010ae8c0(cpu); if cpu.returned_to != 0x10aa9ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aa9f8; continue; }
  pc = 0x10aa9f1; continue;
 }
 0x10aa9f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aa9fa; continue;
 }
 0x10aa9f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aa9fa; continue;
 }
 0x10aa9fa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aaa3a; continue; }
  pc = 0x10aa9fe; continue;
 }
 0x10aa9fe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a224u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aaa32); cpu.call(t); if cpu.returned_to != 0x10aaa32 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aaa3a); f_010aec90(cpu); if cpu.returned_to != 0x10aaa3a { return; }
  pc = 0x10aaa3a; continue;
 }
 0x10aaa3a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10aaa4f); f_010b9eb0(cpu); if cpu.returned_to != 0x10aaa4f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aaa9c; continue; }
  pc = 0x10aaa5d; continue;
 }
 0x10aaa5d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aaa6f; continue; }
  pc = 0x10aaa66; continue;
 }
 0x10aaa66 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aaa9c; continue; }
  pc = 0x10aaa6f; continue;
 }
 0x10aaa6f => {
  cpu.push(0x10aaa74); f_010b5291(cpu); if cpu.returned_to != 0x10aaa74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aaa9c; continue; }
  pc = 0x10aaa78; continue;
 }
 0x10aaa78 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10aaa92); f_010b5523(cpu); if cpu.returned_to != 0x10aaa92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aaa9c); f_010aebf0(cpu); if cpu.returned_to != 0x10aaa9c { return; }
  pc = 0x10aaa9c; continue;
 }
 0x10aaa9c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aaac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aaac0); }
 let mut pc: u32 = 0x10aaac0;
 loop { match pc {
 0x10aaac0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aab08; continue; }
  pc = 0x10aaae1; continue;
 }
 0x10aaae1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aaaef; continue; }
  pc = 0x10aaae6; continue;
 }
 0x10aaae6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aab08; continue; }
  pc = 0x10aaaef; continue;
 }
 0x10aaaef => {
  cpu.push(0x10aaaf4); f_010b5291(cpu); if cpu.returned_to != 0x10aaaf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aab08; continue; }
  pc = 0x10aaaf8; continue;
 }
 0x10aaaf8 => {
  cpu.push(0x10aaafd); f_010ae8c0(cpu); if cpu.returned_to != 0x10aaafd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aab08; continue; }
  pc = 0x10aab01; continue;
 }
 0x10aab01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aab0a; continue;
 }
 0x10aab08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aab0a; continue;
 }
 0x10aab0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aab3c; continue; }
  pc = 0x10aab0e; continue;
 }
 0x10aab0e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a208u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aab34); cpu.call(t); if cpu.returned_to != 0x10aab34 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aab3c); f_010aec90(cpu); if cpu.returned_to != 0x10aab3c { return; }
  pc = 0x10aab3c; continue;
 }
 0x10aab3c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aab49); f_010b9df0(cpu); if cpu.returned_to != 0x10aab49 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10aab97; continue; }
  pc = 0x10aab58; continue;
 }
 0x10aab58 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aab6a; continue; }
  pc = 0x10aab61; continue;
 }
 0x10aab61 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aab97; continue; }
  pc = 0x10aab6a; continue;
 }
 0x10aab6a => {
  cpu.push(0x10aab6f); f_010b5291(cpu); if cpu.returned_to != 0x10aab6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aab97; continue; }
  pc = 0x10aab73; continue;
 }
 0x10aab73 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10aab8d); f_010b5523(cpu); if cpu.returned_to != 0x10aab8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aab97); f_010aec40(cpu); if cpu.returned_to != 0x10aab97 { return; }
  pc = 0x10aab97; continue;
 }
 0x10aab97 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aabc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aabc0); }
 let mut pc: u32 = 0x10aabc0;
 loop { match pc {
 0x10aabc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aac08; continue; }
  pc = 0x10aabe1; continue;
 }
 0x10aabe1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aabef; continue; }
  pc = 0x10aabe6; continue;
 }
 0x10aabe6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aac08; continue; }
  pc = 0x10aabef; continue;
 }
 0x10aabef => {
  cpu.push(0x10aabf4); f_010b5291(cpu); if cpu.returned_to != 0x10aabf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aac08; continue; }
  pc = 0x10aabf8; continue;
 }
 0x10aabf8 => {
  cpu.push(0x10aabfd); f_010ae8c0(cpu); if cpu.returned_to != 0x10aabfd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aac08; continue; }
  pc = 0x10aac01; continue;
 }
 0x10aac01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aac0a; continue;
 }
 0x10aac08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aac0a; continue;
 }
 0x10aac0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aac3c; continue; }
  pc = 0x10aac0e; continue;
 }
 0x10aac0e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a1e8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aac34); cpu.call(t); if cpu.returned_to != 0x10aac34 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aac3c); f_010aec90(cpu); if cpu.returned_to != 0x10aac3c { return; }
  pc = 0x10aac3c; continue;
 }
 0x10aac3c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10aac49); f_010b6bb0(cpu); if cpu.returned_to != 0x10aac49 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10aac97; continue; }
  pc = 0x10aac58; continue;
 }
 0x10aac58 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aac6a; continue; }
  pc = 0x10aac61; continue;
 }
 0x10aac61 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aac97; continue; }
  pc = 0x10aac6a; continue;
 }
 0x10aac6a => {
  cpu.push(0x10aac6f); f_010b5291(cpu); if cpu.returned_to != 0x10aac6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aac97; continue; }
  pc = 0x10aac73; continue;
 }
 0x10aac73 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10aac8d); f_010b5523(cpu); if cpu.returned_to != 0x10aac8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aac97); f_010aec40(cpu); if cpu.returned_to != 0x10aac97 { return; }
  pc = 0x10aac97; continue;
 }
 0x10aac97 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aacc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aacc0); }
 let mut pc: u32 = 0x10aacc0;
 loop { match pc {
 0x10aacc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aad08; continue; }
  pc = 0x10aace1; continue;
 }
 0x10aace1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aacef; continue; }
  pc = 0x10aace6; continue;
 }
 0x10aace6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aad08; continue; }
  pc = 0x10aacef; continue;
 }
 0x10aacef => {
  cpu.push(0x10aacf4); f_010b5291(cpu); if cpu.returned_to != 0x10aacf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aad08; continue; }
  pc = 0x10aacf8; continue;
 }
 0x10aacf8 => {
  cpu.push(0x10aacfd); f_010ae8c0(cpu); if cpu.returned_to != 0x10aacfd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aad08; continue; }
  pc = 0x10aad01; continue;
 }
 0x10aad01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aad0a; continue;
 }
 0x10aad08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aad0a; continue;
 }
 0x10aad0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aad4a; continue; }
  pc = 0x10aad0e; continue;
 }
 0x10aad0e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a1c8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aad42); cpu.call(t); if cpu.returned_to != 0x10aad42 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aad4a); f_010aec90(cpu); if cpu.returned_to != 0x10aad4a { return; }
  pc = 0x10aad4a; continue;
 }
 0x10aad4a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10aad5f); f_010b66b0(cpu); if cpu.returned_to != 0x10aad5f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aadac; continue; }
  pc = 0x10aad6d; continue;
 }
 0x10aad6d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aad7f; continue; }
  pc = 0x10aad76; continue;
 }
 0x10aad76 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aadac; continue; }
  pc = 0x10aad7f; continue;
 }
 0x10aad7f => {
  cpu.push(0x10aad84); f_010b5291(cpu); if cpu.returned_to != 0x10aad84 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aadac; continue; }
  pc = 0x10aad88; continue;
 }
 0x10aad88 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10aada2); f_010b5523(cpu); if cpu.returned_to != 0x10aada2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aadac); f_010aec40(cpu); if cpu.returned_to != 0x10aadac { return; }
  pc = 0x10aadac; continue;
 }
 0x10aadac => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aadd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aadd0); }
 let mut pc: u32 = 0x10aadd0;
 loop { match pc {
 0x10aadd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aae17; continue; }
  pc = 0x10aadf0; continue;
 }
 0x10aadf0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aadfe; continue; }
  pc = 0x10aadf5; continue;
 }
 0x10aadf5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aae17; continue; }
  pc = 0x10aadfe; continue;
 }
 0x10aadfe => {
  cpu.push(0x10aae03); f_010b5291(cpu); if cpu.returned_to != 0x10aae03 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aae17; continue; }
  pc = 0x10aae07; continue;
 }
 0x10aae07 => {
  cpu.push(0x10aae0c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aae0c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aae17; continue; }
  pc = 0x10aae10; continue;
 }
 0x10aae10 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aae19; continue;
 }
 0x10aae17 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aae19; continue;
 }
 0x10aae19 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aae67; continue; }
  pc = 0x10aae1d; continue;
 }
 0x10aae1d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a1a0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aae5f); cpu.call(t); if cpu.returned_to != 0x10aae5f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aae67); f_010aec90(cpu); if cpu.returned_to != 0x10aae67 { return; }
  pc = 0x10aae67; continue;
 }
 0x10aae67 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aae84); f_010b66d0(cpu); if cpu.returned_to != 0x10aae84 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aafb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aafb0); }
 let mut pc: u32 = 0x10aafb0;
 loop { match pc {
 0x10aafb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10aaff8; continue; }
  pc = 0x10aafd1; continue;
 }
 0x10aafd1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aafdf; continue; }
  pc = 0x10aafd6; continue;
 }
 0x10aafd6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aaff8; continue; }
  pc = 0x10aafdf; continue;
 }
 0x10aafdf => {
  cpu.push(0x10aafe4); f_010b5291(cpu); if cpu.returned_to != 0x10aafe4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aaff8; continue; }
  pc = 0x10aafe8; continue;
 }
 0x10aafe8 => {
  cpu.push(0x10aafed); f_010ae8c0(cpu); if cpu.returned_to != 0x10aafed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aaff8; continue; }
  pc = 0x10aaff1; continue;
 }
 0x10aaff1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aaffa; continue;
 }
 0x10aaff8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10aaffa; continue;
 }
 0x10aaffa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ab03a; continue; }
  pc = 0x10aaffe; continue;
 }
 0x10aaffe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116a150u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ab032); cpu.call(t); if cpu.returned_to != 0x10ab032 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ab03a); f_010aec90(cpu); if cpu.returned_to != 0x10ab03a { return; }
  pc = 0x10ab03a; continue;
 }
 0x10ab03a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ab04f); f_010b66f0(cpu); if cpu.returned_to != 0x10ab04f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ab09c; continue; }
  pc = 0x10ab05d; continue;
 }
 0x10ab05d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ab06f; continue; }
  pc = 0x10ab066; continue;
 }
 0x10ab066 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ab09c; continue; }
  pc = 0x10ab06f; continue;
 }
 0x10ab06f => {
  cpu.push(0x10ab074); f_010b5291(cpu); if cpu.returned_to != 0x10ab074 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ab09c; continue; }
  pc = 0x10ab078; continue;
 }
 0x10ab078 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ab092); f_010b5523(cpu); if cpu.returned_to != 0x10ab092 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ab09c); f_010aebf0(cpu); if cpu.returned_to != 0x10ab09c { return; }
  pc = 0x10ab09c; continue;
 }
 0x10ab09c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010abc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10abc70); }
 let mut pc: u32 = 0x10abc70;
 loop { match pc {
 0x10abc70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10abcb7; continue; }
  pc = 0x10abc90; continue;
 }
 0x10abc90 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10abc9e; continue; }
  pc = 0x10abc95; continue;
 }
 0x10abc95 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10abcb7; continue; }
  pc = 0x10abc9e; continue;
 }
 0x10abc9e => {
  cpu.push(0x10abca3); f_010b5291(cpu); if cpu.returned_to != 0x10abca3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10abcb7; continue; }
  pc = 0x10abca7; continue;
 }
 0x10abca7 => {
  cpu.push(0x10abcac); f_010ae8c0(cpu); if cpu.returned_to != 0x10abcac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10abcb7; continue; }
  pc = 0x10abcb0; continue;
 }
 0x10abcb0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10abcb9; continue;
 }
 0x10abcb7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10abcb9; continue;
 }
 0x10abcb9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10abcf9; continue; }
  pc = 0x10abcbd; continue;
 }
 0x10abcbd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169f40u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10abcf1); cpu.call(t); if cpu.returned_to != 0x10abcf1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10abcf9); f_010aec90(cpu); if cpu.returned_to != 0x10abcf9 { return; }
  pc = 0x10abcf9; continue;
 }
 0x10abcf9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10abd0e); f_010b6670(cpu); if cpu.returned_to != 0x10abd0e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010abd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10abd30); }
 let mut pc: u32 = 0x10abd30;
 loop { match pc {
 0x10abd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10abd78; continue; }
  pc = 0x10abd51; continue;
 }
 0x10abd51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10abd5f; continue; }
  pc = 0x10abd56; continue;
 }
 0x10abd56 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10abd78; continue; }
  pc = 0x10abd5f; continue;
 }
 0x10abd5f => {
  cpu.push(0x10abd64); f_010b5291(cpu); if cpu.returned_to != 0x10abd64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10abd78; continue; }
  pc = 0x10abd68; continue;
 }
 0x10abd68 => {
  cpu.push(0x10abd6d); f_010ae8c0(cpu); if cpu.returned_to != 0x10abd6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10abd78; continue; }
  pc = 0x10abd71; continue;
 }
 0x10abd71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10abd7a; continue;
 }
 0x10abd78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10abd7a; continue;
 }
 0x10abd7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10abdac; continue; }
  pc = 0x10abd7e; continue;
 }
 0x10abd7e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169f20u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10abda4); cpu.call(t); if cpu.returned_to != 0x10abda4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10abdac); f_010aec90(cpu); if cpu.returned_to != 0x10abdac { return; }
  pc = 0x10abdac; continue;
 }
 0x10abdac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10abdb9); f_010b68e0(cpu); if cpu.returned_to != 0x10abdb9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10abe07; continue; }
  pc = 0x10abdc8; continue;
 }
 0x10abdc8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10abdda; continue; }
  pc = 0x10abdd1; continue;
 }
 0x10abdd1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10abe07; continue; }
  pc = 0x10abdda; continue;
 }
 0x10abdda => {
  cpu.push(0x10abddf); f_010b5291(cpu); if cpu.returned_to != 0x10abddf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10abe07; continue; }
  pc = 0x10abde3; continue;
 }
 0x10abde3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10abdfd); f_010b5523(cpu); if cpu.returned_to != 0x10abdfd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10abe07); f_010aec40(cpu); if cpu.returned_to != 0x10abe07 { return; }
  pc = 0x10abe07; continue;
 }
 0x10abe07 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac130); }
 let mut pc: u32 = 0x10ac130;
 loop { match pc {
 0x10ac130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac178; continue; }
  pc = 0x10ac151; continue;
 }
 0x10ac151 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac15f; continue; }
  pc = 0x10ac156; continue;
 }
 0x10ac156 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac178; continue; }
  pc = 0x10ac15f; continue;
 }
 0x10ac15f => {
  cpu.push(0x10ac164); f_010b5291(cpu); if cpu.returned_to != 0x10ac164 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac178; continue; }
  pc = 0x10ac168; continue;
 }
 0x10ac168 => {
  cpu.push(0x10ac16d); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac16d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac178; continue; }
  pc = 0x10ac171; continue;
 }
 0x10ac171 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac17a; continue;
 }
 0x10ac178 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac17a; continue;
 }
 0x10ac17a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac1ac; continue; }
  pc = 0x10ac17e; continue;
 }
 0x10ac17e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169eacu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac1a4); cpu.call(t); if cpu.returned_to != 0x10ac1a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac1ac); f_010aec90(cpu); if cpu.returned_to != 0x10ac1ac { return; }
  pc = 0x10ac1ac; continue;
 }
 0x10ac1ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ac1b9); f_010b6a20(cpu); if cpu.returned_to != 0x10ac1b9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ac207; continue; }
  pc = 0x10ac1c8; continue;
 }
 0x10ac1c8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ac1da; continue; }
  pc = 0x10ac1d1; continue;
 }
 0x10ac1d1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac207; continue; }
  pc = 0x10ac1da; continue;
 }
 0x10ac1da => {
  cpu.push(0x10ac1df); f_010b5291(cpu); if cpu.returned_to != 0x10ac1df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac207; continue; }
  pc = 0x10ac1e3; continue;
 }
 0x10ac1e3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ac1fd); f_010b5523(cpu); if cpu.returned_to != 0x10ac1fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ac207); f_010aec40(cpu); if cpu.returned_to != 0x10ac207 { return; }
  pc = 0x10ac207; continue;
 }
 0x10ac207 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ac230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac230); }
 let mut pc: u32 = 0x10ac230;
 loop { match pc {
 0x10ac230 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac277; continue; }
  pc = 0x10ac250; continue;
 }
 0x10ac250 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac25e; continue; }
  pc = 0x10ac255; continue;
 }
 0x10ac255 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac277; continue; }
  pc = 0x10ac25e; continue;
 }
 0x10ac25e => {
  cpu.push(0x10ac263); f_010b5291(cpu); if cpu.returned_to != 0x10ac263 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac277; continue; }
  pc = 0x10ac267; continue;
 }
 0x10ac267 => {
  cpu.push(0x10ac26c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac26c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac277; continue; }
  pc = 0x10ac270; continue;
 }
 0x10ac270 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac279; continue;
 }
 0x10ac277 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac279; continue;
 }
 0x10ac279 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac2c7; continue; }
  pc = 0x10ac27d; continue;
 }
 0x10ac27d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169e80u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac2bf); cpu.call(t); if cpu.returned_to != 0x10ac2bf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac2c7); f_010aec90(cpu); if cpu.returned_to != 0x10ac2c7 { return; }
  pc = 0x10ac2c7; continue;
 }
 0x10ac2c7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac2e4); f_010b68b0(cpu); if cpu.returned_to != 0x10ac2e4 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac300); }
 let mut pc: u32 = 0x10ac300;
 loop { match pc {
 0x10ac300 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac347; continue; }
  pc = 0x10ac320; continue;
 }
 0x10ac320 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac32e; continue; }
  pc = 0x10ac325; continue;
 }
 0x10ac325 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac347; continue; }
  pc = 0x10ac32e; continue;
 }
 0x10ac32e => {
  cpu.push(0x10ac333); f_010b5291(cpu); if cpu.returned_to != 0x10ac333 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac347; continue; }
  pc = 0x10ac337; continue;
 }
 0x10ac337 => {
  cpu.push(0x10ac33c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac33c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac347; continue; }
  pc = 0x10ac340; continue;
 }
 0x10ac340 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac349; continue;
 }
 0x10ac347 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac349; continue;
 }
 0x10ac349 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac389; continue; }
  pc = 0x10ac34d; continue;
 }
 0x10ac34d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169e58u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac381); cpu.call(t); if cpu.returned_to != 0x10ac381 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac389); f_010aec90(cpu); if cpu.returned_to != 0x10ac389 { return; }
  pc = 0x10ac389; continue;
 }
 0x10ac389 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ac39e); f_010b68d0(cpu); if cpu.returned_to != 0x10ac39e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac3c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac3c0); }
 let mut pc: u32 = 0x10ac3c0;
 loop { match pc {
 0x10ac3c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac407; continue; }
  pc = 0x10ac3e0; continue;
 }
 0x10ac3e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac3ee; continue; }
  pc = 0x10ac3e5; continue;
 }
 0x10ac3e5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac407; continue; }
  pc = 0x10ac3ee; continue;
 }
 0x10ac3ee => {
  cpu.push(0x10ac3f3); f_010b5291(cpu); if cpu.returned_to != 0x10ac3f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac407; continue; }
  pc = 0x10ac3f7; continue;
 }
 0x10ac3f7 => {
  cpu.push(0x10ac3fc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac3fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac407; continue; }
  pc = 0x10ac400; continue;
 }
 0x10ac400 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac409; continue;
 }
 0x10ac407 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac409; continue;
 }
 0x10ac409 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac449; continue; }
  pc = 0x10ac40d; continue;
 }
 0x10ac40d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169e38u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac441); cpu.call(t); if cpu.returned_to != 0x10ac441 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac449); f_010aec90(cpu); if cpu.returned_to != 0x10ac449 { return; }
  pc = 0x10ac449; continue;
 }
 0x10ac449 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ac45e); f_010b6940(cpu); if cpu.returned_to != 0x10ac45e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac480); }
 let mut pc: u32 = 0x10ac480;
 loop { match pc {
 0x10ac480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac4c7; continue; }
  pc = 0x10ac4a0; continue;
 }
 0x10ac4a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac4ae; continue; }
  pc = 0x10ac4a5; continue;
 }
 0x10ac4a5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac4c7; continue; }
  pc = 0x10ac4ae; continue;
 }
 0x10ac4ae => {
  cpu.push(0x10ac4b3); f_010b5291(cpu); if cpu.returned_to != 0x10ac4b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac4c7; continue; }
  pc = 0x10ac4b7; continue;
 }
 0x10ac4b7 => {
  cpu.push(0x10ac4bc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac4bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac4c7; continue; }
  pc = 0x10ac4c0; continue;
 }
 0x10ac4c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac4c9; continue;
 }
 0x10ac4c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac4c9; continue;
 }
 0x10ac4c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac509; continue; }
  pc = 0x10ac4cd; continue;
 }
 0x10ac4cd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169e14u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac501); cpu.call(t); if cpu.returned_to != 0x10ac501 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac509); f_010aec90(cpu); if cpu.returned_to != 0x10ac509 { return; }
  pc = 0x10ac509; continue;
 }
 0x10ac509 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ac51e); f_010b6970(cpu); if cpu.returned_to != 0x10ac51e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac540); }
 let mut pc: u32 = 0x10ac540;
 loop { match pc {
 0x10ac540 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac587; continue; }
  pc = 0x10ac560; continue;
 }
 0x10ac560 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac56e; continue; }
  pc = 0x10ac565; continue;
 }
 0x10ac565 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac587; continue; }
  pc = 0x10ac56e; continue;
 }
 0x10ac56e => {
  cpu.push(0x10ac573); f_010b5291(cpu); if cpu.returned_to != 0x10ac573 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac587; continue; }
  pc = 0x10ac577; continue;
 }
 0x10ac577 => {
  cpu.push(0x10ac57c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac57c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac587; continue; }
  pc = 0x10ac580; continue;
 }
 0x10ac580 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac589; continue;
 }
 0x10ac587 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac589; continue;
 }
 0x10ac589 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac5c9; continue; }
  pc = 0x10ac58d; continue;
 }
 0x10ac58d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169decu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac5c1); cpu.call(t); if cpu.returned_to != 0x10ac5c1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac5c9); f_010aec90(cpu); if cpu.returned_to != 0x10ac5c9 { return; }
  pc = 0x10ac5c9; continue;
 }
 0x10ac5c9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ac5de); f_010b69a0(cpu); if cpu.returned_to != 0x10ac5de { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac600); }
 let mut pc: u32 = 0x10ac600;
 loop { match pc {
 0x10ac600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac647; continue; }
  pc = 0x10ac620; continue;
 }
 0x10ac620 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac62e; continue; }
  pc = 0x10ac625; continue;
 }
 0x10ac625 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac647; continue; }
  pc = 0x10ac62e; continue;
 }
 0x10ac62e => {
  cpu.push(0x10ac633); f_010b5291(cpu); if cpu.returned_to != 0x10ac633 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac647; continue; }
  pc = 0x10ac637; continue;
 }
 0x10ac637 => {
  cpu.push(0x10ac63c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac63c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac647; continue; }
  pc = 0x10ac640; continue;
 }
 0x10ac640 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac649; continue;
 }
 0x10ac647 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac649; continue;
 }
 0x10ac649 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac67b; continue; }
  pc = 0x10ac64d; continue;
 }
 0x10ac64d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169dd4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac673); cpu.call(t); if cpu.returned_to != 0x10ac673 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac67b); f_010aec90(cpu); if cpu.returned_to != 0x10ac67b { return; }
  pc = 0x10ac67b; continue;
 }
 0x10ac67b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ac688); f_010b6750(cpu); if cpu.returned_to != 0x10ac688 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ac6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac6b0); }
 let mut pc: u32 = 0x10ac6b0;
 loop { match pc {
 0x10ac6b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac6f7; continue; }
  pc = 0x10ac6d0; continue;
 }
 0x10ac6d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac6de; continue; }
  pc = 0x10ac6d5; continue;
 }
 0x10ac6d5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac6f7; continue; }
  pc = 0x10ac6de; continue;
 }
 0x10ac6de => {
  cpu.push(0x10ac6e3); f_010b5291(cpu); if cpu.returned_to != 0x10ac6e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac6f7; continue; }
  pc = 0x10ac6e7; continue;
 }
 0x10ac6e7 => {
  cpu.push(0x10ac6ec); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac6ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac6f7; continue; }
  pc = 0x10ac6f0; continue;
 }
 0x10ac6f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac6f9; continue;
 }
 0x10ac6f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac6f9; continue;
 }
 0x10ac6f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac72b; continue; }
  pc = 0x10ac6fd; continue;
 }
 0x10ac6fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169db8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac723); cpu.call(t); if cpu.returned_to != 0x10ac723 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac72b); f_010aec90(cpu); if cpu.returned_to != 0x10ac72b { return; }
  pc = 0x10ac72b; continue;
 }
 0x10ac72b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ac738); f_010b67a0(cpu); if cpu.returned_to != 0x10ac738 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac810); }
 let mut pc: u32 = 0x10ac810;
 loop { match pc {
 0x10ac810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac857; continue; }
  pc = 0x10ac830; continue;
 }
 0x10ac830 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac83e; continue; }
  pc = 0x10ac835; continue;
 }
 0x10ac835 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac857; continue; }
  pc = 0x10ac83e; continue;
 }
 0x10ac83e => {
  cpu.push(0x10ac843); f_010b5291(cpu); if cpu.returned_to != 0x10ac843 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac857; continue; }
  pc = 0x10ac847; continue;
 }
 0x10ac847 => {
  cpu.push(0x10ac84c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac84c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac857; continue; }
  pc = 0x10ac850; continue;
 }
 0x10ac850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac859; continue;
 }
 0x10ac857 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac859; continue;
 }
 0x10ac859 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac88b; continue; }
  pc = 0x10ac85d; continue;
 }
 0x10ac85d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169d84u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac883); cpu.call(t); if cpu.returned_to != 0x10ac883 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac88b); f_010aec90(cpu); if cpu.returned_to != 0x10ac88b { return; }
  pc = 0x10ac88b; continue;
 }
 0x10ac88b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ac898); f_010b6820(cpu); if cpu.returned_to != 0x10ac898 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ac8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac8c0); }
 let mut pc: u32 = 0x10ac8c0;
 loop { match pc {
 0x10ac8c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac907; continue; }
  pc = 0x10ac8e0; continue;
 }
 0x10ac8e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac8ee; continue; }
  pc = 0x10ac8e5; continue;
 }
 0x10ac8e5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac907; continue; }
  pc = 0x10ac8ee; continue;
 }
 0x10ac8ee => {
  cpu.push(0x10ac8f3); f_010b5291(cpu); if cpu.returned_to != 0x10ac8f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac907; continue; }
  pc = 0x10ac8f7; continue;
 }
 0x10ac8f7 => {
  cpu.push(0x10ac8fc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac8fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac907; continue; }
  pc = 0x10ac900; continue;
 }
 0x10ac900 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac909; continue;
 }
 0x10ac907 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac909; continue;
 }
 0x10ac909 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac957; continue; }
  pc = 0x10ac90d; continue;
 }
 0x10ac90d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169d60u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ac94f); cpu.call(t); if cpu.returned_to != 0x10ac94f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac957); f_010aec90(cpu); if cpu.returned_to != 0x10ac957 { return; }
  pc = 0x10ac957; continue;
 }
 0x10ac957 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ac974); f_010b69c0(cpu); if cpu.returned_to != 0x10ac974 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ac990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ac990); }
 let mut pc: u32 = 0x10ac990;
 loop { match pc {
 0x10ac990 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ac9d7; continue; }
  pc = 0x10ac9b0; continue;
 }
 0x10ac9b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ac9be; continue; }
  pc = 0x10ac9b5; continue;
 }
 0x10ac9b5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ac9d7; continue; }
  pc = 0x10ac9be; continue;
 }
 0x10ac9be => {
  cpu.push(0x10ac9c3); f_010b5291(cpu); if cpu.returned_to != 0x10ac9c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ac9d7; continue; }
  pc = 0x10ac9c7; continue;
 }
 0x10ac9c7 => {
  cpu.push(0x10ac9cc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ac9cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ac9d7; continue; }
  pc = 0x10ac9d0; continue;
 }
 0x10ac9d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ac9d9; continue;
 }
 0x10ac9d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ac9d9; continue;
 }
 0x10ac9d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aca27; continue; }
  pc = 0x10ac9dd; continue;
 }
 0x10ac9dd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169d38u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aca1f); cpu.call(t); if cpu.returned_to != 0x10aca1f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aca27); f_010aec90(cpu); if cpu.returned_to != 0x10aca27 { return; }
  pc = 0x10aca27; continue;
 }
 0x10aca27 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10aca44); f_010b69f0(cpu); if cpu.returned_to != 0x10aca44 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010aca60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aca60); }
 let mut pc: u32 = 0x10aca60;
 loop { match pc {
 0x10aca60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acaa7; continue; }
  pc = 0x10aca80; continue;
 }
 0x10aca80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10aca8e; continue; }
  pc = 0x10aca85; continue;
 }
 0x10aca85 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acaa7; continue; }
  pc = 0x10aca8e; continue;
 }
 0x10aca8e => {
  cpu.push(0x10aca93); f_010b5291(cpu); if cpu.returned_to != 0x10aca93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acaa7; continue; }
  pc = 0x10aca97; continue;
 }
 0x10aca97 => {
  cpu.push(0x10aca9c); f_010ae8c0(cpu); if cpu.returned_to != 0x10aca9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acaa7; continue; }
  pc = 0x10acaa0; continue;
 }
 0x10acaa0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acaa9; continue;
 }
 0x10acaa7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acaa9; continue;
 }
 0x10acaa9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acaf7; continue; }
  pc = 0x10acaad; continue;
 }
 0x10acaad => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169d14u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10acaef); cpu.call(t); if cpu.returned_to != 0x10acaef { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acaf7); f_010aec90(cpu); if cpu.returned_to != 0x10acaf7 { return; }
  pc = 0x10acaf7; continue;
 }
 0x10acaf7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acb14); f_010b81e0(cpu); if cpu.returned_to != 0x10acb14 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10acb62; continue; }
  pc = 0x10acb23; continue;
 }
 0x10acb23 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10acb35; continue; }
  pc = 0x10acb2c; continue;
 }
 0x10acb2c => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acb62; continue; }
  pc = 0x10acb35; continue;
 }
 0x10acb35 => {
  cpu.push(0x10acb3a); f_010b5291(cpu); if cpu.returned_to != 0x10acb3a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acb62; continue; }
  pc = 0x10acb3e; continue;
 }
 0x10acb3e => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10acb58); f_010b5523(cpu); if cpu.returned_to != 0x10acb58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10acb62); f_010aec40(cpu); if cpu.returned_to != 0x10acb62 { return; }
  pc = 0x10acb62; continue;
 }
 0x10acb62 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010acb80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acb80); }
 let mut pc: u32 = 0x10acb80;
 loop { match pc {
 0x10acb80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acbc7; continue; }
  pc = 0x10acba0; continue;
 }
 0x10acba0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10acbae; continue; }
  pc = 0x10acba5; continue;
 }
 0x10acba5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acbc7; continue; }
  pc = 0x10acbae; continue;
 }
 0x10acbae => {
  cpu.push(0x10acbb3); f_010b5291(cpu); if cpu.returned_to != 0x10acbb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acbc7; continue; }
  pc = 0x10acbb7; continue;
 }
 0x10acbb7 => {
  cpu.push(0x10acbbc); f_010ae8c0(cpu); if cpu.returned_to != 0x10acbbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acbc7; continue; }
  pc = 0x10acbc0; continue;
 }
 0x10acbc0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acbc9; continue;
 }
 0x10acbc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acbc9; continue;
 }
 0x10acbc9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acbfb; continue; }
  pc = 0x10acbcd; continue;
 }
 0x10acbcd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169cfcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10acbf3); cpu.call(t); if cpu.returned_to != 0x10acbf3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acbfb); f_010aec90(cpu); if cpu.returned_to != 0x10acbfb { return; }
  pc = 0x10acbfb; continue;
 }
 0x10acbfb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10acc08); f_010b6a30(cpu); if cpu.returned_to != 0x10acc08 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010acc30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acc30); }
 let mut pc: u32 = 0x10acc30;
 loop { match pc {
 0x10acc30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acc77; continue; }
  pc = 0x10acc50; continue;
 }
 0x10acc50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10acc5e; continue; }
  pc = 0x10acc55; continue;
 }
 0x10acc55 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acc77; continue; }
  pc = 0x10acc5e; continue;
 }
 0x10acc5e => {
  cpu.push(0x10acc63); f_010b5291(cpu); if cpu.returned_to != 0x10acc63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acc77; continue; }
  pc = 0x10acc67; continue;
 }
 0x10acc67 => {
  cpu.push(0x10acc6c); f_010ae8c0(cpu); if cpu.returned_to != 0x10acc6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acc77; continue; }
  pc = 0x10acc70; continue;
 }
 0x10acc70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acc79; continue;
 }
 0x10acc77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acc79; continue;
 }
 0x10acc79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10accab; continue; }
  pc = 0x10acc7d; continue;
 }
 0x10acc7d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169cd8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10acca3); cpu.call(t); if cpu.returned_to != 0x10acca3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10accab); f_010aec90(cpu); if cpu.returned_to != 0x10accab { return; }
  pc = 0x10accab; continue;
 }
 0x10accab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10accb8); f_010b6b20(cpu); if cpu.returned_to != 0x10accb8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acce0); }
 let mut pc: u32 = 0x10acce0;
 loop { match pc {
 0x10acce0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acd27; continue; }
  pc = 0x10acd00; continue;
 }
 0x10acd00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10acd0e; continue; }
  pc = 0x10acd05; continue;
 }
 0x10acd05 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acd27; continue; }
  pc = 0x10acd0e; continue;
 }
 0x10acd0e => {
  cpu.push(0x10acd13); f_010b5291(cpu); if cpu.returned_to != 0x10acd13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acd27; continue; }
  pc = 0x10acd17; continue;
 }
 0x10acd17 => {
  cpu.push(0x10acd1c); f_010ae8c0(cpu); if cpu.returned_to != 0x10acd1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acd27; continue; }
  pc = 0x10acd20; continue;
 }
 0x10acd20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acd29; continue;
 }
 0x10acd27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acd29; continue;
 }
 0x10acd29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acd77; continue; }
  pc = 0x10acd2d; continue;
 }
 0x10acd2d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169cacu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10acd6f); cpu.call(t); if cpu.returned_to != 0x10acd6f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acd77); f_010aec90(cpu); if cpu.returned_to != 0x10acd77 { return; }
  pc = 0x10acd77; continue;
 }
 0x10acd77 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acd94); f_010b8330(cpu); if cpu.returned_to != 0x10acd94 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10acde2; continue; }
  pc = 0x10acda3; continue;
 }
 0x10acda3 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10acdb5; continue; }
  pc = 0x10acdac; continue;
 }
 0x10acdac => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acde2; continue; }
  pc = 0x10acdb5; continue;
 }
 0x10acdb5 => {
  cpu.push(0x10acdba); f_010b5291(cpu); if cpu.returned_to != 0x10acdba { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acde2; continue; }
  pc = 0x10acdbe; continue;
 }
 0x10acdbe => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10acdd8); f_010b5523(cpu); if cpu.returned_to != 0x10acdd8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10acde2); f_010aebf0(cpu); if cpu.returned_to != 0x10acde2 { return; }
  pc = 0x10acde2; continue;
 }
 0x10acde2 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ace00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ace00); }
 let mut pc: u32 = 0x10ace00;
 loop { match pc {
 0x10ace00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ace48; continue; }
  pc = 0x10ace21; continue;
 }
 0x10ace21 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ace2f; continue; }
  pc = 0x10ace26; continue;
 }
 0x10ace26 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ace48; continue; }
  pc = 0x10ace2f; continue;
 }
 0x10ace2f => {
  cpu.push(0x10ace34); f_010b5291(cpu); if cpu.returned_to != 0x10ace34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ace48; continue; }
  pc = 0x10ace38; continue;
 }
 0x10ace38 => {
  cpu.push(0x10ace3d); f_010ae8c0(cpu); if cpu.returned_to != 0x10ace3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ace48; continue; }
  pc = 0x10ace41; continue;
 }
 0x10ace41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ace4a; continue;
 }
 0x10ace48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ace4a; continue;
 }
 0x10ace4a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ace7c; continue; }
  pc = 0x10ace4e; continue;
 }
 0x10ace4e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169c88u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ace74); cpu.call(t); if cpu.returned_to != 0x10ace74 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ace7c); f_010aec90(cpu); if cpu.returned_to != 0x10ace7c { return; }
  pc = 0x10ace7c; continue;
 }
 0x10ace7c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ace89); f_010b6b30(cpu); if cpu.returned_to != 0x10ace89 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10aced7; continue; }
  pc = 0x10ace98; continue;
 }
 0x10ace98 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10aceaa; continue; }
  pc = 0x10acea1; continue;
 }
 0x10acea1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10aced7; continue; }
  pc = 0x10aceaa; continue;
 }
 0x10aceaa => {
  cpu.push(0x10aceaf); f_010b5291(cpu); if cpu.returned_to != 0x10aceaf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aced7; continue; }
  pc = 0x10aceb3; continue;
 }
 0x10aceb3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10acecd); f_010b5523(cpu); if cpu.returned_to != 0x10acecd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10aced7); f_010aebf0(cpu); if cpu.returned_to != 0x10aced7 { return; }
  pc = 0x10aced7; continue;
 }
 0x10aced7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010acf00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acf00); }
 let mut pc: u32 = 0x10acf00;
 loop { match pc {
 0x10acf00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acf47; continue; }
  pc = 0x10acf20; continue;
 }
 0x10acf20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10acf2e; continue; }
  pc = 0x10acf25; continue;
 }
 0x10acf25 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acf47; continue; }
  pc = 0x10acf2e; continue;
 }
 0x10acf2e => {
  cpu.push(0x10acf33); f_010b5291(cpu); if cpu.returned_to != 0x10acf33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acf47; continue; }
  pc = 0x10acf37; continue;
 }
 0x10acf37 => {
  cpu.push(0x10acf3c); f_010ae8c0(cpu); if cpu.returned_to != 0x10acf3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acf47; continue; }
  pc = 0x10acf40; continue;
 }
 0x10acf40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acf49; continue;
 }
 0x10acf47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acf49; continue;
 }
 0x10acf49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acf7b; continue; }
  pc = 0x10acf4d; continue;
 }
 0x10acf4d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169c64u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10acf73); cpu.call(t); if cpu.returned_to != 0x10acf73 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10acf7b); f_010aec90(cpu); if cpu.returned_to != 0x10acf7b { return; }
  pc = 0x10acf7b; continue;
 }
 0x10acf7b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10acf88); f_010b6be0(cpu); if cpu.returned_to != 0x10acf88 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010acfb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10acfb0); }
 let mut pc: u32 = 0x10acfb0;
 loop { match pc {
 0x10acfb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10acff8; continue; }
  pc = 0x10acfd1; continue;
 }
 0x10acfd1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10acfdf; continue; }
  pc = 0x10acfd6; continue;
 }
 0x10acfd6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10acff8; continue; }
  pc = 0x10acfdf; continue;
 }
 0x10acfdf => {
  cpu.push(0x10acfe4); f_010b5291(cpu); if cpu.returned_to != 0x10acfe4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10acff8; continue; }
  pc = 0x10acfe8; continue;
 }
 0x10acfe8 => {
  cpu.push(0x10acfed); f_010ae8c0(cpu); if cpu.returned_to != 0x10acfed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10acff8; continue; }
  pc = 0x10acff1; continue;
 }
 0x10acff1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10acffa; continue;
 }
 0x10acff8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10acffa; continue;
 }
 0x10acffa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad03a; continue; }
  pc = 0x10acffe; continue;
 }
 0x10acffe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169c3cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad032); cpu.call(t); if cpu.returned_to != 0x10ad032 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad03a); f_010aec90(cpu); if cpu.returned_to != 0x10ad03a { return; }
  pc = 0x10ad03a; continue;
 }
 0x10ad03a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ad04f); f_010b6cc0(cpu); if cpu.returned_to != 0x10ad04f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad09c; continue; }
  pc = 0x10ad05d; continue;
 }
 0x10ad05d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ad06f; continue; }
  pc = 0x10ad066; continue;
 }
 0x10ad066 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad09c; continue; }
  pc = 0x10ad06f; continue;
 }
 0x10ad06f => {
  cpu.push(0x10ad074); f_010b5291(cpu); if cpu.returned_to != 0x10ad074 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad09c; continue; }
  pc = 0x10ad078; continue;
 }
 0x10ad078 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ad092); f_010b5523(cpu); if cpu.returned_to != 0x10ad092 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ad09c); f_010aebf0(cpu); if cpu.returned_to != 0x10ad09c { return; }
  pc = 0x10ad09c; continue;
 }
 0x10ad09c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_010ad0c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad0c0); }
 let mut pc: u32 = 0x10ad0c0;
 loop { match pc {
 0x10ad0c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad108; continue; }
  pc = 0x10ad0e1; continue;
 }
 0x10ad0e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad0ef; continue; }
  pc = 0x10ad0e6; continue;
 }
 0x10ad0e6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad108; continue; }
  pc = 0x10ad0ef; continue;
 }
 0x10ad0ef => {
  cpu.push(0x10ad0f4); f_010b5291(cpu); if cpu.returned_to != 0x10ad0f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad108; continue; }
  pc = 0x10ad0f8; continue;
 }
 0x10ad0f8 => {
  cpu.push(0x10ad0fd); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad0fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad108; continue; }
  pc = 0x10ad101; continue;
 }
 0x10ad101 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad10a; continue;
 }
 0x10ad108 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad10a; continue;
 }
 0x10ad10a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad13c; continue; }
  pc = 0x10ad10e; continue;
 }
 0x10ad10e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169c20u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad134); cpu.call(t); if cpu.returned_to != 0x10ad134 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad13c); f_010aec90(cpu); if cpu.returned_to != 0x10ad13c { return; }
  pc = 0x10ad13c; continue;
 }
 0x10ad13c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad149); f_010b6c00(cpu); if cpu.returned_to != 0x10ad149 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ad197; continue; }
  pc = 0x10ad158; continue;
 }
 0x10ad158 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ad16a; continue; }
  pc = 0x10ad161; continue;
 }
 0x10ad161 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad197; continue; }
  pc = 0x10ad16a; continue;
 }
 0x10ad16a => {
  cpu.push(0x10ad16f); f_010b5291(cpu); if cpu.returned_to != 0x10ad16f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad197; continue; }
  pc = 0x10ad173; continue;
 }
 0x10ad173 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ad18d); f_010b5523(cpu); if cpu.returned_to != 0x10ad18d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ad197); f_010aec40(cpu); if cpu.returned_to != 0x10ad197 { return; }
  pc = 0x10ad197; continue;
 }
 0x10ad197 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad1c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad1c0); }
 let mut pc: u32 = 0x10ad1c0;
 loop { match pc {
 0x10ad1c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad207; continue; }
  pc = 0x10ad1e0; continue;
 }
 0x10ad1e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad1ee; continue; }
  pc = 0x10ad1e5; continue;
 }
 0x10ad1e5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad207; continue; }
  pc = 0x10ad1ee; continue;
 }
 0x10ad1ee => {
  cpu.push(0x10ad1f3); f_010b5291(cpu); if cpu.returned_to != 0x10ad1f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad207; continue; }
  pc = 0x10ad1f7; continue;
 }
 0x10ad1f7 => {
  cpu.push(0x10ad1fc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad1fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad207; continue; }
  pc = 0x10ad200; continue;
 }
 0x10ad200 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad209; continue;
 }
 0x10ad207 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad209; continue;
 }
 0x10ad209 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad23b; continue; }
  pc = 0x10ad20d; continue;
 }
 0x10ad20d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169c00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad233); cpu.call(t); if cpu.returned_to != 0x10ad233 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad23b); f_010aec90(cpu); if cpu.returned_to != 0x10ad23b { return; }
  pc = 0x10ad23b; continue;
 }
 0x10ad23b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad248); f_010b58f0(cpu); if cpu.returned_to != 0x10ad248 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad270); }
 let mut pc: u32 = 0x10ad270;
 loop { match pc {
 0x10ad270 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad2b8; continue; }
  pc = 0x10ad291; continue;
 }
 0x10ad291 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad29f; continue; }
  pc = 0x10ad296; continue;
 }
 0x10ad296 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad2b8; continue; }
  pc = 0x10ad29f; continue;
 }
 0x10ad29f => {
  cpu.push(0x10ad2a4); f_010b5291(cpu); if cpu.returned_to != 0x10ad2a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad2b8; continue; }
  pc = 0x10ad2a8; continue;
 }
 0x10ad2a8 => {
  cpu.push(0x10ad2ad); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad2ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad2b8; continue; }
  pc = 0x10ad2b1; continue;
 }
 0x10ad2b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad2ba; continue;
 }
 0x10ad2b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad2ba; continue;
 }
 0x10ad2ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad2fa; continue; }
  pc = 0x10ad2be; continue;
 }
 0x10ad2be => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169be0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad2f2); cpu.call(t); if cpu.returned_to != 0x10ad2f2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad2fa); f_010aec90(cpu); if cpu.returned_to != 0x10ad2fa { return; }
  pc = 0x10ad2fa; continue;
 }
 0x10ad2fa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ad30f); f_010b5990(cpu); if cpu.returned_to != 0x10ad30f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad35c; continue; }
  pc = 0x10ad31d; continue;
 }
 0x10ad31d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ad32f; continue; }
  pc = 0x10ad326; continue;
 }
 0x10ad326 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad35c; continue; }
  pc = 0x10ad32f; continue;
 }
 0x10ad32f => {
  cpu.push(0x10ad334); f_010b5291(cpu); if cpu.returned_to != 0x10ad334 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad35c; continue; }
  pc = 0x10ad338; continue;
 }
 0x10ad338 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ad352); f_010b5523(cpu); if cpu.returned_to != 0x10ad352 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ad35c); f_010aebf0(cpu); if cpu.returned_to != 0x10ad35c { return; }
  pc = 0x10ad35c; continue;
 }
 0x10ad35c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad380); }
 let mut pc: u32 = 0x10ad380;
 loop { match pc {
 0x10ad380 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad3c8; continue; }
  pc = 0x10ad3a1; continue;
 }
 0x10ad3a1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad3af; continue; }
  pc = 0x10ad3a6; continue;
 }
 0x10ad3a6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad3c8; continue; }
  pc = 0x10ad3af; continue;
 }
 0x10ad3af => {
  cpu.push(0x10ad3b4); f_010b5291(cpu); if cpu.returned_to != 0x10ad3b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad3c8; continue; }
  pc = 0x10ad3b8; continue;
 }
 0x10ad3b8 => {
  cpu.push(0x10ad3bd); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad3bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad3c8; continue; }
  pc = 0x10ad3c1; continue;
 }
 0x10ad3c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad3ca; continue;
 }
 0x10ad3c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad3ca; continue;
 }
 0x10ad3ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad3fc; continue; }
  pc = 0x10ad3ce; continue;
 }
 0x10ad3ce => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169bc0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad3f4); cpu.call(t); if cpu.returned_to != 0x10ad3f4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad3fc); f_010aec90(cpu); if cpu.returned_to != 0x10ad3fc { return; }
  pc = 0x10ad3fc; continue;
 }
 0x10ad3fc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad409); f_010b5840(cpu); if cpu.returned_to != 0x10ad409 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ad457; continue; }
  pc = 0x10ad418; continue;
 }
 0x10ad418 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ad42a; continue; }
  pc = 0x10ad421; continue;
 }
 0x10ad421 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad457; continue; }
  pc = 0x10ad42a; continue;
 }
 0x10ad42a => {
  cpu.push(0x10ad42f); f_010b5291(cpu); if cpu.returned_to != 0x10ad42f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad457; continue; }
  pc = 0x10ad433; continue;
 }
 0x10ad433 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ad44d); f_010b5523(cpu); if cpu.returned_to != 0x10ad44d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ad457); f_010aebf0(cpu); if cpu.returned_to != 0x10ad457 { return; }
  pc = 0x10ad457; continue;
 }
 0x10ad457 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad480); }
 let mut pc: u32 = 0x10ad480;
 loop { match pc {
 0x10ad480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad4c0; continue; }
  pc = 0x10ad499; continue;
 }
 0x10ad499 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad4a7; continue; }
  pc = 0x10ad49e; continue;
 }
 0x10ad49e => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad4c0; continue; }
  pc = 0x10ad4a7; continue;
 }
 0x10ad4a7 => {
  cpu.push(0x10ad4ac); f_010b5291(cpu); if cpu.returned_to != 0x10ad4ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad4c0; continue; }
  pc = 0x10ad4b0; continue;
 }
 0x10ad4b0 => {
  cpu.push(0x10ad4b5); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad4b5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad4c0; continue; }
  pc = 0x10ad4b9; continue;
 }
 0x10ad4b9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad4c2; continue;
 }
 0x10ad4c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad4c2; continue;
 }
 0x10ad4c2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad4ec; continue; }
  pc = 0x10ad4c6; continue;
 }
 0x10ad4c6 => {
  { let v: u32 = 0x1169ba4u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169ba4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad4e9); f_010b5523(cpu); if cpu.returned_to != 0x10ad4e9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x10ad4ec; continue;
 }
 0x10ad4ec => {
  cpu.push(0x10ad4f1); f_010b531b(cpu); if cpu.returned_to != 0x10ad4f1 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad500); }
 let mut pc: u32 = 0x10ad500;
 loop { match pc {
 0x10ad500 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad547; continue; }
  pc = 0x10ad520; continue;
 }
 0x10ad520 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad52e; continue; }
  pc = 0x10ad525; continue;
 }
 0x10ad525 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad547; continue; }
  pc = 0x10ad52e; continue;
 }
 0x10ad52e => {
  cpu.push(0x10ad533); f_010b5291(cpu); if cpu.returned_to != 0x10ad533 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad547; continue; }
  pc = 0x10ad537; continue;
 }
 0x10ad537 => {
  cpu.push(0x10ad53c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad53c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad547; continue; }
  pc = 0x10ad540; continue;
 }
 0x10ad540 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad549; continue;
 }
 0x10ad547 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad549; continue;
 }
 0x10ad549 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad57b; continue; }
  pc = 0x10ad54d; continue;
 }
 0x10ad54d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169b84u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad573); cpu.call(t); if cpu.returned_to != 0x10ad573 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad57b); f_010aec90(cpu); if cpu.returned_to != 0x10ad57b { return; }
  pc = 0x10ad57b; continue;
 }
 0x10ad57b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad588); f_010b52fa(cpu); if cpu.returned_to != 0x10ad588 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad5b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad5b0); }
 let mut pc: u32 = 0x10ad5b0;
 loop { match pc {
 0x10ad5b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad5f0; continue; }
  pc = 0x10ad5c9; continue;
 }
 0x10ad5c9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad5d7; continue; }
  pc = 0x10ad5ce; continue;
 }
 0x10ad5ce => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad5f0; continue; }
  pc = 0x10ad5d7; continue;
 }
 0x10ad5d7 => {
  cpu.push(0x10ad5dc); f_010b5291(cpu); if cpu.returned_to != 0x10ad5dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad5f0; continue; }
  pc = 0x10ad5e0; continue;
 }
 0x10ad5e0 => {
  cpu.push(0x10ad5e5); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad5e5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad5f0; continue; }
  pc = 0x10ad5e9; continue;
 }
 0x10ad5e9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad5f2; continue;
 }
 0x10ad5f0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad5f2; continue;
 }
 0x10ad5f2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad61c; continue; }
  pc = 0x10ad5f6; continue;
 }
 0x10ad5f6 => {
  { let v: u32 = 0x1169b6cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169b6cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad619); f_010b5523(cpu); if cpu.returned_to != 0x10ad619 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x10ad61c; continue;
 }
 0x10ad61c => {
  cpu.push(0x10ad621); f_010b53af(cpu); if cpu.returned_to != 0x10ad621 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad630); }
 let mut pc: u32 = 0x10ad630;
 loop { match pc {
 0x10ad630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad677; continue; }
  pc = 0x10ad650; continue;
 }
 0x10ad650 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad65e; continue; }
  pc = 0x10ad655; continue;
 }
 0x10ad655 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad677; continue; }
  pc = 0x10ad65e; continue;
 }
 0x10ad65e => {
  cpu.push(0x10ad663); f_010b5291(cpu); if cpu.returned_to != 0x10ad663 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad677; continue; }
  pc = 0x10ad667; continue;
 }
 0x10ad667 => {
  cpu.push(0x10ad66c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad66c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad677; continue; }
  pc = 0x10ad670; continue;
 }
 0x10ad670 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad679; continue;
 }
 0x10ad677 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad679; continue;
 }
 0x10ad679 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad6ab; continue; }
  pc = 0x10ad67d; continue;
 }
 0x10ad67d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169b54u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad6a3); cpu.call(t); if cpu.returned_to != 0x10ad6a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad6ab); f_010aec90(cpu); if cpu.returned_to != 0x10ad6ab { return; }
  pc = 0x10ad6ab; continue;
 }
 0x10ad6ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad6b8); f_010b5385(cpu); if cpu.returned_to != 0x10ad6b8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad6e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad6e8); }
 let mut pc: u32 = 0x10ad6e8;
 loop { match pc {
 0x10ad6e8 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad720; continue; }
  pc = 0x10ad6f9; continue;
 }
 0x10ad6f9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad707; continue; }
  pc = 0x10ad6fe; continue;
 }
 0x10ad6fe => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad720; continue; }
  pc = 0x10ad707; continue;
 }
 0x10ad707 => {
  cpu.push(0x10ad70c); f_010b5291(cpu); if cpu.returned_to != 0x10ad70c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad720; continue; }
  pc = 0x10ad710; continue;
 }
 0x10ad710 => {
  cpu.push(0x10ad715); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad715 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad720; continue; }
  pc = 0x10ad719; continue;
 }
 0x10ad719 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad722; continue;
 }
 0x10ad720 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad722; continue;
 }
 0x10ad722 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad74c; continue; }
  pc = 0x10ad726; continue;
 }
 0x10ad726 => {
  { let v: u32 = 0x1169b38u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169b38u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad749); f_010b5523(cpu); if cpu.returned_to != 0x10ad749 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x10ad74c; continue;
 }
 0x10ad74c => {
  cpu.push(0x10ad751); f_010b5365(cpu); if cpu.returned_to != 0x10ad751 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad760); }
 let mut pc: u32 = 0x10ad760;
 loop { match pc {
 0x10ad760 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad7a7; continue; }
  pc = 0x10ad780; continue;
 }
 0x10ad780 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad78e; continue; }
  pc = 0x10ad785; continue;
 }
 0x10ad785 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad7a7; continue; }
  pc = 0x10ad78e; continue;
 }
 0x10ad78e => {
  cpu.push(0x10ad793); f_010b5291(cpu); if cpu.returned_to != 0x10ad793 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad7a7; continue; }
  pc = 0x10ad797; continue;
 }
 0x10ad797 => {
  cpu.push(0x10ad79c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad79c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad7a7; continue; }
  pc = 0x10ad7a0; continue;
 }
 0x10ad7a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad7a9; continue;
 }
 0x10ad7a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad7a9; continue;
 }
 0x10ad7a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad7db; continue; }
  pc = 0x10ad7ad; continue;
 }
 0x10ad7ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169b20u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ad7d3); cpu.call(t); if cpu.returned_to != 0x10ad7d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ad7db); f_010aec90(cpu); if cpu.returned_to != 0x10ad7db { return; }
  pc = 0x10ad7db; continue;
 }
 0x10ad7db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad7e8); f_010b533b(cpu); if cpu.returned_to != 0x10ad7e8 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad810); }
 let mut pc: u32 = 0x10ad810;
 loop { match pc {
 0x10ad810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad852; continue; }
  pc = 0x10ad82b; continue;
 }
 0x10ad82b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad839; continue; }
  pc = 0x10ad830; continue;
 }
 0x10ad830 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad852; continue; }
  pc = 0x10ad839; continue;
 }
 0x10ad839 => {
  cpu.push(0x10ad83e); f_010b5291(cpu); if cpu.returned_to != 0x10ad83e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad852; continue; }
  pc = 0x10ad842; continue;
 }
 0x10ad842 => {
  cpu.push(0x10ad847); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad847 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad852; continue; }
  pc = 0x10ad84b; continue;
 }
 0x10ad84b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad854; continue;
 }
 0x10ad852 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad854; continue;
 }
 0x10ad854 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad87e; continue; }
  pc = 0x10ad858; continue;
 }
 0x10ad858 => {
  { let v: u32 = 0x1169b04u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x1169b04u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ad87b); f_010b5523(cpu); if cpu.returned_to != 0x10ad87b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10ad87e; continue;
 }
 0x10ad87e => {
  cpu.push(0x10ad883); f_010b5465(cpu); if cpu.returned_to != 0x10ad883 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ad8ce; continue; }
  pc = 0x10ad88f; continue;
 }
 0x10ad88f => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ad8a1; continue; }
  pc = 0x10ad898; continue;
 }
 0x10ad898 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad8ce; continue; }
  pc = 0x10ad8a1; continue;
 }
 0x10ad8a1 => {
  cpu.push(0x10ad8a6); f_010b5291(cpu); if cpu.returned_to != 0x10ad8a6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad8ce; continue; }
  pc = 0x10ad8aa; continue;
 }
 0x10ad8aa => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ad8c4); f_010b5523(cpu); if cpu.returned_to != 0x10ad8c4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ad8ce); f_010aec40(cpu); if cpu.returned_to != 0x10ad8ce { return; }
  pc = 0x10ad8ce; continue;
 }
 0x10ad8ce => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ad9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ad9b0); }
 let mut pc: u32 = 0x10ad9b0;
 loop { match pc {
 0x10ad9b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ad9f7; continue; }
  pc = 0x10ad9d0; continue;
 }
 0x10ad9d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ad9de; continue; }
  pc = 0x10ad9d5; continue;
 }
 0x10ad9d5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ad9f7; continue; }
  pc = 0x10ad9de; continue;
 }
 0x10ad9de => {
  cpu.push(0x10ad9e3); f_010b5291(cpu); if cpu.returned_to != 0x10ad9e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ad9f7; continue; }
  pc = 0x10ad9e7; continue;
 }
 0x10ad9e7 => {
  cpu.push(0x10ad9ec); f_010ae8c0(cpu); if cpu.returned_to != 0x10ad9ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ad9f7; continue; }
  pc = 0x10ad9f0; continue;
 }
 0x10ad9f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ad9f9; continue;
 }
 0x10ad9f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ad9f9; continue;
 }
 0x10ad9f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ada39; continue; }
  pc = 0x10ad9fd; continue;
 }
 0x10ad9fd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169ac0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ada31); cpu.call(t); if cpu.returned_to != 0x10ada31 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ada39); f_010aec90(cpu); if cpu.returned_to != 0x10ada39 { return; }
  pc = 0x10ada39; continue;
 }
 0x10ada39 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ada4e); f_010b53ff(cpu); if cpu.returned_to != 0x10ada4e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ada70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ada70); }
 let mut pc: u32 = 0x10ada70;
 loop { match pc {
 0x10ada70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10adab7; continue; }
  pc = 0x10ada90; continue;
 }
 0x10ada90 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ada9e; continue; }
  pc = 0x10ada95; continue;
 }
 0x10ada95 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10adab7; continue; }
  pc = 0x10ada9e; continue;
 }
 0x10ada9e => {
  cpu.push(0x10adaa3); f_010b5291(cpu); if cpu.returned_to != 0x10adaa3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10adab7; continue; }
  pc = 0x10adaa7; continue;
 }
 0x10adaa7 => {
  cpu.push(0x10adaac); f_010ae8c0(cpu); if cpu.returned_to != 0x10adaac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adab7; continue; }
  pc = 0x10adab0; continue;
 }
 0x10adab0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10adab9; continue;
 }
 0x10adab7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10adab9; continue;
 }
 0x10adab9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adaf9; continue; }
  pc = 0x10adabd; continue;
 }
 0x10adabd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169aa0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adaf1); cpu.call(t); if cpu.returned_to != 0x10adaf1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adaf9); f_010aec90(cpu); if cpu.returned_to != 0x10adaf9 { return; }
  pc = 0x10adaf9; continue;
 }
 0x10adaf9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10adb0e); f_010b53cf(cpu); if cpu.returned_to != 0x10adb0e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010adb30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10adb30); }
 let mut pc: u32 = 0x10adb30;
 loop { match pc {
 0x10adb30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10adb78; continue; }
  pc = 0x10adb51; continue;
 }
 0x10adb51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10adb5f; continue; }
  pc = 0x10adb56; continue;
 }
 0x10adb56 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10adb78; continue; }
  pc = 0x10adb5f; continue;
 }
 0x10adb5f => {
  cpu.push(0x10adb64); f_010b5291(cpu); if cpu.returned_to != 0x10adb64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10adb78; continue; }
  pc = 0x10adb68; continue;
 }
 0x10adb68 => {
  cpu.push(0x10adb6d); f_010ae8c0(cpu); if cpu.returned_to != 0x10adb6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adb78; continue; }
  pc = 0x10adb71; continue;
 }
 0x10adb71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10adb7a; continue;
 }
 0x10adb78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10adb7a; continue;
 }
 0x10adb7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adbba; continue; }
  pc = 0x10adb7e; continue;
 }
 0x10adb7e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a84u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adbb2); cpu.call(t); if cpu.returned_to != 0x10adbb2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adbba); f_010aec90(cpu); if cpu.returned_to != 0x10adbba { return; }
  pc = 0x10adbba; continue;
 }
 0x10adbba => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10adbcf); f_010b52d8(cpu); if cpu.returned_to != 0x10adbcf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adc1c; continue; }
  pc = 0x10adbdd; continue;
 }
 0x10adbdd => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10adbef; continue; }
  pc = 0x10adbe6; continue;
 }
 0x10adbe6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10adc1c; continue; }
  pc = 0x10adbef; continue;
 }
 0x10adbef => {
  cpu.push(0x10adbf4); f_010b5291(cpu); if cpu.returned_to != 0x10adbf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10adc1c; continue; }
  pc = 0x10adbf8; continue;
 }
 0x10adbf8 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10adc12); f_010b5523(cpu); if cpu.returned_to != 0x10adc12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10adc1c); f_010aec40(cpu); if cpu.returned_to != 0x10adc1c { return; }
  pc = 0x10adc1c; continue;
 }
 0x10adc1c => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010adc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10adc40); }
 let mut pc: u32 = 0x10adc40;
 loop { match pc {
 0x10adc40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10adc88; continue; }
  pc = 0x10adc61; continue;
 }
 0x10adc61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10adc6f; continue; }
  pc = 0x10adc66; continue;
 }
 0x10adc66 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10adc88; continue; }
  pc = 0x10adc6f; continue;
 }
 0x10adc6f => {
  cpu.push(0x10adc74); f_010b5291(cpu); if cpu.returned_to != 0x10adc74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10adc88; continue; }
  pc = 0x10adc78; continue;
 }
 0x10adc78 => {
  cpu.push(0x10adc7d); f_010ae8c0(cpu); if cpu.returned_to != 0x10adc7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adc88; continue; }
  pc = 0x10adc81; continue;
 }
 0x10adc81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10adc8a; continue;
 }
 0x10adc88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10adc8a; continue;
 }
 0x10adc8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10adcbc; continue; }
  pc = 0x10adc8e; continue;
 }
 0x10adc8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a68u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adcb4); cpu.call(t); if cpu.returned_to != 0x10adcb4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adcbc); f_010aec90(cpu); if cpu.returned_to != 0x10adcbc { return; }
  pc = 0x10adcbc; continue;
 }
 0x10adcbc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10adcc9); f_010b5297(cpu); if cpu.returned_to != 0x10adcc9 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10add17; continue; }
  pc = 0x10adcd8; continue;
 }
 0x10adcd8 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10adcea; continue; }
  pc = 0x10adce1; continue;
 }
 0x10adce1 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10add17; continue; }
  pc = 0x10adcea; continue;
 }
 0x10adcea => {
  cpu.push(0x10adcef); f_010b5291(cpu); if cpu.returned_to != 0x10adcef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10add17; continue; }
  pc = 0x10adcf3; continue;
 }
 0x10adcf3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10add0d); f_010b5523(cpu); if cpu.returned_to != 0x10add0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10add17); f_010aec40(cpu); if cpu.returned_to != 0x10add17 { return; }
  pc = 0x10add17; continue;
 }
 0x10add17 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010add40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10add40); }
 let mut pc: u32 = 0x10add40;
 loop { match pc {
 0x10add40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3b4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3b8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10add96; continue; }
  pc = 0x10add6f; continue;
 }
 0x10add6f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10add7d; continue; }
  pc = 0x10add74; continue;
 }
 0x10add74 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10add96; continue; }
  pc = 0x10add7d; continue;
 }
 0x10add7d => {
  cpu.push(0x10add82); f_010b5291(cpu); if cpu.returned_to != 0x10add82 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10add96; continue; }
  pc = 0x10add86; continue;
 }
 0x10add86 => {
  cpu.push(0x10add8b); f_010ae8c0(cpu); if cpu.returned_to != 0x10add8b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10add96; continue; }
  pc = 0x10add8f; continue;
 }
 0x10add8f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10add98; continue;
 }
 0x10add96 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10add98; continue;
 }
 0x10add98 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10addd4; continue; }
  pc = 0x10add9c; continue;
 }
 0x10add9c => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11699ecu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x38cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10addc7); cpu.call(t); if cpu.returned_to != 0x10addc7 { return; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10addd4); f_010aec90(cpu); if cpu.returned_to != 0x10addd4 { return; }
  pc = 0x10addd4; continue;
 }
 0x10addd4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10adddb); f_010b5ca0(cpu); if cpu.returned_to != 0x10adddb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae00d; continue; }
  pc = 0x10adde8; continue;
 }
 0x10adde8 => {
  { cpu.sub32(cpu.r32(0x12d1538u32), 0x0u32); }
  if cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10addf5; continue;
 }
 0x10addf5 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ade0b; continue; }
  pc = 0x10addfe; continue;
 }
 0x10addfe => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10ade0b; continue;
 }
 0x10ade0b => {
  cpu.push(0x10ade10); f_010b5291(cpu); if cpu.returned_to != 0x10ade10 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10ade18; continue;
 }
 0x10ade18 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ade32); f_010b5523(cpu); if cpu.returned_to != 0x10ade32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x388u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a08u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x394u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ade5f); cpu.call(t); if cpu.returned_to != 0x10ade5f { return; }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ade6c); f_010aec90(cpu); if cpu.returned_to != 0x10ade6c { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ade86); f_010b5523(cpu); if cpu.returned_to != 0x10ade86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a18u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3acu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adea8); cpu.call(t); if cpu.returned_to != 0x10adea8 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adeb0); f_010aec90(cpu); if cpu.returned_to != 0x10adeb0 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10adeca); f_010b5523(cpu); if cpu.returned_to != 0x10adeca { return; }
  { let v: u32 = cpu.r32(0xfeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a28u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x398u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adefa); cpu.call(t); if cpu.returned_to != 0x10adefa { return; }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adf07); f_010aec90(cpu); if cpu.returned_to != 0x10adf07 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adf20); f_010b5523(cpu); if cpu.returned_to != 0x10adf20 { return; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a38u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x39cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adf50); cpu.call(t); if cpu.returned_to != 0x10adf50 { return; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adf5d); f_010aec90(cpu); if cpu.returned_to != 0x10adf5d { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10adf77); f_010b5523(cpu); if cpu.returned_to != 0x10adf77 { return; }
  { let v: u32 = cpu.r32(0x102u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a48u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adfa7); cpu.call(t); if cpu.returned_to != 0x10adfa7 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10adfb4); f_010aec90(cpu); if cpu.returned_to != 0x10adfb4 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10adfce); f_010b5523(cpu); if cpu.returned_to != 0x10adfce { return; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x398u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169a58u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10adffe); cpu.call(t); if cpu.returned_to != 0x10adffe { return; }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae00b); f_010aec90(cpu); if cpu.returned_to != 0x10ae00b { return; }
  pc = 0x10ae055; continue;
 }
 0x10ae00d => {
  { cpu.sub32(cpu.r32(0x12d1538u32), 0x0u32); }
  if cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10ae016; continue;
 }
 0x10ae016 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae028; continue; }
  pc = 0x10ae01f; continue;
 }
 0x10ae01f => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10ae028; continue;
 }
 0x10ae028 => {
  cpu.push(0x10ae02d); f_010b5291(cpu); if cpu.returned_to != 0x10ae02d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae055; continue; }
  pc = 0x10ae031; continue;
 }
 0x10ae031 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ae04b); f_010b5523(cpu); if cpu.returned_to != 0x10ae04b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10ae055); f_010aec40(cpu); if cpu.returned_to != 0x10ae055 { return; }
  pc = 0x10ae055; continue;
 }
 0x10ae055 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae070); }
 let mut pc: u32 = 0x10ae070;
 loop { match pc {
 0x10ae070 => {
  f_010b5291(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae080); }
 let mut pc: u32 = 0x10ae080;
 loop { match pc {
 0x10ae080 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae0c7; continue; }
  pc = 0x10ae0a0; continue;
 }
 0x10ae0a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae0ae; continue; }
  pc = 0x10ae0a5; continue;
 }
 0x10ae0a5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae0c7; continue; }
  pc = 0x10ae0ae; continue;
 }
 0x10ae0ae => {
  cpu.push(0x10ae0b3); f_010b5291(cpu); if cpu.returned_to != 0x10ae0b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae0c7; continue; }
  pc = 0x10ae0b7; continue;
 }
 0x10ae0b7 => {
  cpu.push(0x10ae0bc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae0bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae0c7; continue; }
  pc = 0x10ae0c0; continue;
 }
 0x10ae0c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae0c9; continue;
 }
 0x10ae0c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae0c9; continue;
 }
 0x10ae0c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae0fb; continue; }
  pc = 0x10ae0cd; continue;
 }
 0x10ae0cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11699dcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae0f3); cpu.call(t); if cpu.returned_to != 0x10ae0f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae0fb); f_010aec90(cpu); if cpu.returned_to != 0x10ae0fb { return; }
  pc = 0x10ae0fb; continue;
 }
 0x10ae0fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ae108); f_010b5660(cpu); if cpu.returned_to != 0x10ae108 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae130); }
 let mut pc: u32 = 0x10ae130;
 loop { match pc {
 0x10ae130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae177; continue; }
  pc = 0x10ae150; continue;
 }
 0x10ae150 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae15e; continue; }
  pc = 0x10ae155; continue;
 }
 0x10ae155 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae177; continue; }
  pc = 0x10ae15e; continue;
 }
 0x10ae15e => {
  cpu.push(0x10ae163); f_010b5291(cpu); if cpu.returned_to != 0x10ae163 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae177; continue; }
  pc = 0x10ae167; continue;
 }
 0x10ae167 => {
  cpu.push(0x10ae16c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae16c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae177; continue; }
  pc = 0x10ae170; continue;
 }
 0x10ae170 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x10ae179; continue;
 }
 0x10ae177 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10ae179; continue;
 }
 0x10ae179 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae1d5; continue; }
  pc = 0x10ae17d; continue;
 }
 0x10ae17d => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11699b4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae1cd); cpu.call(t); if cpu.returned_to != 0x10ae1cd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae1d5); f_010aec90(cpu); if cpu.returned_to != 0x10ae1d5 { return; }
  pc = 0x10ae1d5; continue;
 }
 0x10ae1d5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10ae1fa); f_010b51b7(cpu); if cpu.returned_to != 0x10ae1fa { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10ae248; continue; }
  pc = 0x10ae209; continue;
 }
 0x10ae209 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae21b; continue; }
  pc = 0x10ae212; continue;
 }
 0x10ae212 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae248; continue; }
  pc = 0x10ae21b; continue;
 }
 0x10ae21b => {
  cpu.push(0x10ae220); f_010b5291(cpu); if cpu.returned_to != 0x10ae220 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae248; continue; }
  pc = 0x10ae224; continue;
 }
 0x10ae224 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ae23e); f_010b5523(cpu); if cpu.returned_to != 0x10ae23e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ae248); f_010aec40(cpu); if cpu.returned_to != 0x10ae248 { return; }
  pc = 0x10ae248; continue;
 }
 0x10ae248 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x12d1534u32)); cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae260); }
 let mut pc: u32 = 0x10ae260;
 loop { match pc {
 0x10ae260 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10ae26a); f_010b5150(cpu); if cpu.returned_to != 0x10ae26a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae270); }
 let mut pc: u32 = 0x10ae270;
 loop { match pc {
 0x10ae270 => {
  pc = 0x10b514c; continue;
 }
 0x10b514c => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae280); }
 let mut pc: u32 = 0x10ae280;
 loop { match pc {
 0x10ae280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae2c7; continue; }
  pc = 0x10ae2a0; continue;
 }
 0x10ae2a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae2ae; continue; }
  pc = 0x10ae2a5; continue;
 }
 0x10ae2a5 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae2c7; continue; }
  pc = 0x10ae2ae; continue;
 }
 0x10ae2ae => {
  cpu.push(0x10ae2b3); f_010b5291(cpu); if cpu.returned_to != 0x10ae2b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae2c7; continue; }
  pc = 0x10ae2b7; continue;
 }
 0x10ae2b7 => {
  cpu.push(0x10ae2bc); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae2bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae2c7; continue; }
  pc = 0x10ae2c0; continue;
 }
 0x10ae2c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae2c9; continue;
 }
 0x10ae2c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae2c9; continue;
 }
 0x10ae2c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae2fb; continue; }
  pc = 0x10ae2cd; continue;
 }
 0x10ae2cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169998u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae2f3); cpu.call(t); if cpu.returned_to != 0x10ae2f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae2fb); f_010aec90(cpu); if cpu.returned_to != 0x10ae2fb { return; }
  pc = 0x10ae2fb; continue;
 }
 0x10ae2fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ae308); f_010b5107(cpu); if cpu.returned_to != 0x10ae308 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae330); }
 let mut pc: u32 = 0x10ae330;
 loop { match pc {
 0x10ae330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae377; continue; }
  pc = 0x10ae350; continue;
 }
 0x10ae350 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae35e; continue; }
  pc = 0x10ae355; continue;
 }
 0x10ae355 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae377; continue; }
  pc = 0x10ae35e; continue;
 }
 0x10ae35e => {
  cpu.push(0x10ae363); f_010b5291(cpu); if cpu.returned_to != 0x10ae363 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae377; continue; }
  pc = 0x10ae367; continue;
 }
 0x10ae367 => {
  cpu.push(0x10ae36c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae36c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae377; continue; }
  pc = 0x10ae370; continue;
 }
 0x10ae370 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae379; continue;
 }
 0x10ae377 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae379; continue;
 }
 0x10ae379 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae3c7; continue; }
  pc = 0x10ae37d; continue;
 }
 0x10ae37d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169978u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae3bf); cpu.call(t); if cpu.returned_to != 0x10ae3bf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae3c7); f_010aec90(cpu); if cpu.returned_to != 0x10ae3c7 { return; }
  pc = 0x10ae3c7; continue;
 }
 0x10ae3c7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae3e4); f_010b4ef1(cpu); if cpu.returned_to != 0x10ae3e4 { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae400(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae400); }
 let mut pc: u32 = 0x10ae400;
 loop { match pc {
 0x10ae400 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae447; continue; }
  pc = 0x10ae420; continue;
 }
 0x10ae420 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae42e; continue; }
  pc = 0x10ae425; continue;
 }
 0x10ae425 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae447; continue; }
  pc = 0x10ae42e; continue;
 }
 0x10ae42e => {
  cpu.push(0x10ae433); f_010b5291(cpu); if cpu.returned_to != 0x10ae433 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae447; continue; }
  pc = 0x10ae437; continue;
 }
 0x10ae437 => {
  cpu.push(0x10ae43c); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae43c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae447; continue; }
  pc = 0x10ae440; continue;
 }
 0x10ae440 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae449; continue;
 }
 0x10ae447 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae449; continue;
 }
 0x10ae449 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae489; continue; }
  pc = 0x10ae44d; continue;
 }
 0x10ae44d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169958u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae481); cpu.call(t); if cpu.returned_to != 0x10ae481 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae489); f_010aec90(cpu); if cpu.returned_to != 0x10ae489 { return; }
  pc = 0x10ae489; continue;
 }
 0x10ae489 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ae49e); f_010b4e79(cpu); if cpu.returned_to != 0x10ae49e { return; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae4c0); }
 let mut pc: u32 = 0x10ae4c0;
 loop { match pc {
 0x10ae4c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae508; continue; }
  pc = 0x10ae4e1; continue;
 }
 0x10ae4e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae4ef; continue; }
  pc = 0x10ae4e6; continue;
 }
 0x10ae4e6 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae508; continue; }
  pc = 0x10ae4ef; continue;
 }
 0x10ae4ef => {
  cpu.push(0x10ae4f4); f_010b5291(cpu); if cpu.returned_to != 0x10ae4f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae508; continue; }
  pc = 0x10ae4f8; continue;
 }
 0x10ae4f8 => {
  cpu.push(0x10ae4fd); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae4fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae508; continue; }
  pc = 0x10ae501; continue;
 }
 0x10ae501 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae50a; continue;
 }
 0x10ae508 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae50a; continue;
 }
 0x10ae50a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae53c; continue; }
  pc = 0x10ae50e; continue;
 }
 0x10ae50e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x116993cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae534); cpu.call(t); if cpu.returned_to != 0x10ae534 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae53c); f_010aec90(cpu); if cpu.returned_to != 0x10ae53c { return; }
  pc = 0x10ae53c; continue;
 }
 0x10ae53c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ae549); f_010b4e51(cpu); if cpu.returned_to != 0x10ae549 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ae597; continue; }
  pc = 0x10ae558; continue;
 }
 0x10ae558 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae56a; continue; }
  pc = 0x10ae561; continue;
 }
 0x10ae561 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae597; continue; }
  pc = 0x10ae56a; continue;
 }
 0x10ae56a => {
  cpu.push(0x10ae56f); f_010b5291(cpu); if cpu.returned_to != 0x10ae56f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae597; continue; }
  pc = 0x10ae573; continue;
 }
 0x10ae573 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ae58d); f_010b5523(cpu); if cpu.returned_to != 0x10ae58d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ae597); f_010aebf0(cpu); if cpu.returned_to != 0x10ae597 { return; }
  pc = 0x10ae597; continue;
 }
 0x10ae597 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae5c0); }
 let mut pc: u32 = 0x10ae5c0;
 loop { match pc {
 0x10ae5c0 => {
  pc = 0x10b49e7; continue;
 }
 0x10b49e7 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x117bcf6u32)); cpu.w32(0x117bcf6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae5d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10ae5d0); } r_010ae5d0(cpu, 0x10ae5d0); }
pub fn f_010b49e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b49e0); } r_010ae5d0(cpu, 0x10b49e0); }
fn r_010ae5d0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10ae5d0 => {
  pc = 0x10b49e0; continue;
 }
 0x10b49e0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x117bcf6u32)); cpu.w32(0x117bcf6u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae5e0); }
 let mut pc: u32 = 0x10ae5e0;
 loop { match pc {
 0x10ae5e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae628; continue; }
  pc = 0x10ae601; continue;
 }
 0x10ae601 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae60f; continue; }
  pc = 0x10ae606; continue;
 }
 0x10ae606 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae628; continue; }
  pc = 0x10ae60f; continue;
 }
 0x10ae60f => {
  cpu.push(0x10ae614); f_010b5291(cpu); if cpu.returned_to != 0x10ae614 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae628; continue; }
  pc = 0x10ae618; continue;
 }
 0x10ae618 => {
  cpu.push(0x10ae61d); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae61d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae628; continue; }
  pc = 0x10ae621; continue;
 }
 0x10ae621 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae62a; continue;
 }
 0x10ae628 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae62a; continue;
 }
 0x10ae62a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae66a; continue; }
  pc = 0x10ae62e; continue;
 }
 0x10ae62e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169920u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae662); cpu.call(t); if cpu.returned_to != 0x10ae662 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae66a); f_010aec90(cpu); if cpu.returned_to != 0x10ae66a { return; }
  pc = 0x10ae66a; continue;
 }
 0x10ae66a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ae67f); f_010b5c80(cpu); if cpu.returned_to != 0x10ae67f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae6cc; continue; }
  pc = 0x10ae68d; continue;
 }
 0x10ae68d => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae69f; continue; }
  pc = 0x10ae696; continue;
 }
 0x10ae696 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae6cc; continue; }
  pc = 0x10ae69f; continue;
 }
 0x10ae69f => {
  cpu.push(0x10ae6a4); f_010b5291(cpu); if cpu.returned_to != 0x10ae6a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae6cc; continue; }
  pc = 0x10ae6a8; continue;
 }
 0x10ae6a8 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ae6c2); f_010b5523(cpu); if cpu.returned_to != 0x10ae6c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ae6cc); f_010aec40(cpu); if cpu.returned_to != 0x10ae6cc { return; }
  pc = 0x10ae6cc; continue;
 }
 0x10ae6cc => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae6f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae6f0); }
 let mut pc: u32 = 0x10ae6f0;
 loop { match pc {
 0x10ae6f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10ae738; continue; }
  pc = 0x10ae711; continue;
 }
 0x10ae711 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10ae71f; continue; }
  pc = 0x10ae716; continue;
 }
 0x10ae716 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae738; continue; }
  pc = 0x10ae71f; continue;
 }
 0x10ae71f => {
  cpu.push(0x10ae724); f_010b5291(cpu); if cpu.returned_to != 0x10ae724 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae738; continue; }
  pc = 0x10ae728; continue;
 }
 0x10ae728 => {
  cpu.push(0x10ae72d); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae72d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae738; continue; }
  pc = 0x10ae731; continue;
 }
 0x10ae731 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae73a; continue;
 }
 0x10ae738 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae73a; continue;
 }
 0x10ae73a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae76c; continue; }
  pc = 0x10ae73e; continue;
 }
 0x10ae73e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x1169900u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae764); cpu.call(t); if cpu.returned_to != 0x10ae764 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae76c); f_010aec90(cpu); if cpu.returned_to != 0x10ae76c { return; }
  pc = 0x10ae76c; continue;
 }
 0x10ae76c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ae779); f_010b5680(cpu); if cpu.returned_to != 0x10ae779 { return; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10ae7c7; continue; }
  pc = 0x10ae788; continue;
 }
 0x10ae788 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae79a; continue; }
  pc = 0x10ae791; continue;
 }
 0x10ae791 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae7c7; continue; }
  pc = 0x10ae79a; continue;
 }
 0x10ae79a => {
  cpu.push(0x10ae79f); f_010b5291(cpu); if cpu.returned_to != 0x10ae79f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae7c7; continue; }
  pc = 0x10ae7a3; continue;
 }
 0x10ae7a3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x117a5d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ae7bd); f_010b5523(cpu); if cpu.returned_to != 0x10ae7bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10ae7c7); f_010aebf0(cpu); if cpu.returned_to != 0x10ae7c7 { return; }
  pc = 0x10ae7c7; continue;
 }
 0x10ae7c7 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d1534u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae7f0); }
 let mut pc: u32 = 0x10ae7f0;
 loop { match pc {
 0x10ae7f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117a5d0u32), 0x0u32); }
  if cpu.zf { pc = 0x10ae8b8; continue; }
  pc = 0x10ae801; continue;
 }
 0x10ae801 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d1538u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d1534u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10ae83f; continue; }
  pc = 0x10ae818; continue;
 }
 0x10ae818 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x10ae826; continue; }
  pc = 0x10ae81d; continue;
 }
 0x10ae81d => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae83f; continue; }
  pc = 0x10ae826; continue;
 }
 0x10ae826 => {
  cpu.push(0x10ae82b); f_010b5291(cpu); if cpu.returned_to != 0x10ae82b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae83f; continue; }
  pc = 0x10ae82f; continue;
 }
 0x10ae82f => {
  cpu.push(0x10ae834); f_010ae8c0(cpu); if cpu.returned_to != 0x10ae834 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae83f; continue; }
  pc = 0x10ae838; continue;
 }
 0x10ae838 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10ae841; continue;
 }
 0x10ae83f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ae841; continue;
 }
 0x10ae841 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ae86b; continue; }
  pc = 0x10ae845; continue;
 }
 0x10ae845 => {
  { let v: u32 = 0x11698ecu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11698ecu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ae868); f_010b5523(cpu); if cpu.returned_to != 0x10ae868 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10ae86b; continue;
 }
 0x10ae86b => {
  cpu.push(0x10ae870); f_010b56f0(cpu); if cpu.returned_to != 0x10ae870 { return; }
  { cpu.sub32(cpu.r32(0x12d1538u32), 0x0u32); }
  if cpu.zf { pc = 0x10ae8a3; continue; }
  pc = 0x10ae879; continue;
 }
 0x10ae879 => {
  { cpu.sub32(cpu.r32(0x12d1534u32), 0x1u32); }
  if cpu.zf { pc = 0x10ae88b; continue; }
  pc = 0x10ae882; continue;
 }
 0x10ae882 => {
  { cpu.sub32(cpu.r32(0x12d153cu32), 0x0u32); }
  if cpu.zf { pc = 0x10ae8a3; continue; }
  pc = 0x10ae88b; continue;
 }
 0x10ae88b => {
  cpu.push(0x10ae890); f_010b5291(cpu); if cpu.returned_to != 0x10ae890 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ae8a3; continue; }
  pc = 0x10ae894; continue;
 }
 0x10ae894 => {
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ae8a0); f_010b5595(cpu); if cpu.returned_to != 0x10ae8a0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10ae8a3; continue;
 }
 0x10ae8a3 => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117a5d0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  pc = 0x10ae8b8; continue;
 }
 0x10ae8b8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae8c0); }
 let mut pc: u32 = 0x10ae8c0;
 loop { match pc {
 0x10ae8c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  cpu.push(0x10ae8ce); f_010b49ee(cpu); if cpu.returned_to != 0x10ae8ce { return; }
  { let v: u32 = cpu.r32(0x12d1540u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x36ee80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0xea60u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x90u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x10ae986; continue; }
  pc = 0x10ae92e; continue;
 }
 0x10ae92e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11698acu32; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10ae97c); cpu.call(t); if cpu.returned_to != 0x10ae97c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ae984); f_010aec90(cpu); if cpu.returned_to != 0x10ae984 { return; }
  pc = 0x10ae9ac; continue;
 }
 0x10ae986 => {
  { let v: u32 = 0x11698c8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x11698c8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ae9a9); f_010b5523(cpu); if cpu.returned_to != 0x10ae9a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10ae9ac; continue;
 }
 0x10ae9ac => {
  { let v: u32 = cpu.r32(0x12d1534u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10ae9cf; continue; }
  pc = 0x10ae9b7; continue;
 }
 0x10ae9b7 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x11698d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ae9cc); f_010b5523(cpu); if cpu.returned_to != 0x10ae9cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10ae9cf; continue;
 }
 0x10ae9cf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ae9e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ae9e0); }
 let mut pc: u32 = 0x10ae9e0;
 loop { match pc {
 0x10ae9e0 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x90u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x94u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aea30); cpu.call(t); if cpu.returned_to != 0x10aea30 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10aea4e); f_010b5523(cpu); if cpu.returned_to != 0x10aea4e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aebf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aebf0); }
 let mut pc: u32 = 0x10aebf0;
 loop { match pc {
 0x10aebf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x116989cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aec14); cpu.call(t); if cpu.returned_to != 0x10aec14 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10aec32); f_010b5523(cpu); if cpu.returned_to != 0x10aec32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aec40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aec40); }
 let mut pc: u32 = 0x10aec40;
 loop { match pc {
 0x10aec40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x116988cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10aec64); cpu.call(t); if cpu.returned_to != 0x10aec64 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10aec82); f_010b5523(cpu); if cpu.returned_to != 0x10aec82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aec90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aec90); }
 let mut pc: u32 = 0x10aec90;
 loop { match pc {
 0x10aec90 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d1544u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10aecb1); f_010b5523(cpu); if cpu.returned_to != 0x10aecb1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aecc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aecc0); }
 let mut pc: u32 = 0x10aecc0;
 loop { match pc {
 0x10aecc0 => {
  { cpu.sub32(cpu.r32(0x117a5d4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10aed40; continue; }
  pc = 0x10aecc9; continue;
 }
 0x10aecc9 => {
  { let v = 0x10a6bd0u32; cpu.push(v); }
  { let v = 0x10aecc0u32; cpu.push(v); }
  cpu.push(0x10aecd8); f_010b47f0(cpu); if cpu.returned_to != 0x10aecd8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d1534u32; cpu.push(v); }
  cpu.push(0x10aece7); f_010b4730(cpu); if cpu.returned_to != 0x10aece7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d1538u32; cpu.push(v); }
  cpu.push(0x10aecf6); f_010b4730(cpu); if cpu.returned_to != 0x10aecf6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d153cu32; cpu.push(v); }
  cpu.push(0x10aed05); f_010b4730(cpu); if cpu.returned_to != 0x10aed05 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d1544u32; cpu.push(v); }
  cpu.push(0x10aed14); f_010b4730(cpu); if cpu.returned_to != 0x10aed14 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117a5d0u32; cpu.push(v); }
  cpu.push(0x10aed23); f_010b4730(cpu); if cpu.returned_to != 0x10aed23 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d1540u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10aed37); f_010b4730(cpu); if cpu.returned_to != 0x10aed37 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117a5d4u32, v); }
  pc = 0x10aed40; continue;
 }
 0x10aed40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aed50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aed50); }
 let mut pc: u32 = 0x10aed50;
 loop { match pc {
 0x10aed50 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x10aed99; continue; }
  pc = 0x10aed59; continue;
 }
 0x10aed59 => {
  if !cpu.cf { pc = 0x10aed83; continue; }
  pc = 0x10aed5b; continue;
 }
 0x10aed5b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x10aed83; continue; }
  pc = 0x10aed62; continue;
 }
 0x10aed62 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.df = true; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10aed83 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  pc = 0x10aed99; continue;
 }
 0x10aed99 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aed9d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10aed9d); } r_010aed9d(cpu, 0x10aed9d); }
pub fn f_010aee2d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10aee2d); } r_010aed9d(cpu, 0x10aee2d); }
fn r_010aed9d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10aed9d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10aeda3, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x10aeda8, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x10aee29; continue; }
  pc = 0x10aedb8; continue;
 }
 0x10aedb8 => {
  if cpu.cf { pc = 0x10aee2d; continue; }
  pc = 0x10aedba; continue;
 }
 0x10aedba => {
  if cpu.zf { cpu.call(0x10aee31); return; }
  pc = 0x10aedbc; continue;
 }
 0x10aedbc => {
  if (cpu.cf || cpu.zf) { pc = 0x10aee35; continue; }
  pc = 0x10aedbe; continue;
 }
 0x10aedbe => {
  if cpu.sf { pc = 0x10aee39; continue; }
  pc = 0x10aedc0; continue;
 }
 0x10aedc0 => {
  if cpu.pf() { f_010aedc2(cpu); return; }
  f_010aedc2(cpu); return;
 }
 0x10aee29 => {
  { let v: u8 = 0x17u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10aee2d; continue;
 }
 0x10aee2d => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10aee33); f_010aee27(cpu); if cpu.returned_to != 0x10aee33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  pc = 0x10aee35; continue;
 }
 0x10aee35 => {
  if cpu.zf { pc = 0x10aee4f; continue; }
  pc = 0x10aee37; continue;
 }
 0x10aee37 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x41c64e6du32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3039u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x10aee4f; continue;
 }
 0x10aee39 => {
  cpu.unimplemented(0x10aee39, "Insd");
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0xc2u8; cpu.w8(0xffffff81u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x10aee4f; continue;
 }
 0x10aee4f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aedc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aedc2); }
 let mut pc: u32 = 0x10aedc2;
 loop { match pc {
 0x10aedc2 => {
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
  pc = 0x10aedd8; continue;
 }
 0x10aedd8 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10aed9du32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10aedd8; continue; }
  pc = 0x10aedf8; continue;
 }
 0x10aedf8 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10aedf8; continue; }
  pc = 0x10aee02; continue;
 }
 0x10aee02 => {
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
pub fn f_010aee0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aee0c); }
 let mut pc: u32 = 0x10aee0c;
 loop { match pc {
 0x10aee0c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x10aee1e; continue; }
  pc = 0x10aee14; continue;
 }
 0x10aee14 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10aee1e; continue; }
  pc = 0x10aee18; continue;
 }
 0x10aee18 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10aee1e; continue;
 }
 0x10aee1e => {
  cpu.push(0x10aee23); f_010aedc2(cpu); if cpu.returned_to != 0x10aee23 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aee27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aee27); }
 let mut pc: u32 = 0x10aee27;
 loop { match pc {
 0x10aee27 => {
  { let v: u32 = 0x117b1d0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aee51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aee51); }
 let mut pc: u32 = 0x10aee51;
 loop { match pc {
 0x10aee51 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10aee59); f_010aee27(cpu); if cpu.returned_to != 0x10aee59 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10aee5f; continue; }
  pc = 0x10aee5d; continue;
 }
 0x10aee5d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10aee5f; continue;
 }
 0x10aee5f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aee64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10aee64); } r_010aee64(cpu, 0x10aee64); }
pub fn f_010aeed8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10aeed8); } r_010aee64(cpu, 0x10aeed8); }
pub fn f_010af0f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af0f5); } r_010aee64(cpu, 0x10af0f5); }
pub fn f_010c5ea3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10c5ea3); } r_010aee64(cpu, 0x10c5ea3); }
fn r_010aee64(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10aee64 => {
  pc = 0x10aeedc; continue;
 }
 0x10aeed8 => {
  cpu.unimplemented(0x10aeed8, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba30u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba1cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x117ba28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x117ba53u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x117ba54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x10aef5e; continue; }
  pc = 0x10aef20; continue;
 }
 0x10aeedc => {
  {  }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba30u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba1cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x117ba28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x117ba53u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x117ba54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x10aef5e; continue; }
  pc = 0x10aef20; continue;
 }
 0x10aef20 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x117ba28u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba1cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10aef96; continue;
 }
 0x10aef5e => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x10aef98; continue; }
  pc = 0x10aef64; continue;
 }
 0x10aef64 => {
  { let v: u32 = cpu.edx; cpu.w32(0x117b1dcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x117ba28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10aef96; continue;
 }
 0x10aef96 => {
  pc = 0x10aeff1; continue;
 }
 0x10aef98 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10aefda; continue; }
  pc = 0x10aefa6; continue;
 }
 0x10aefa6 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x10aefb5; continue; }
  pc = 0x10aefaf; continue;
 }
 0x10aefaf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x117b1e0u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x10aefb5; continue;
 }
 0x10aefb5 => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x10aefc9; continue; }
  pc = 0x10aefc7; continue;
 }
 0x10aefc7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10aefc9; continue;
 }
 0x10aefc9 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x117ba28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x10aeff1; continue;
 }
 0x10aefda => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10aeff1; continue;
 }
 0x10aeff1 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x117ba4au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x117ba4bu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x10af139u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x117ba4du32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x117ba51u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x117ba28u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x12d29e0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xfu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.df = false; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 1); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  if cpu.zf { pc = 0x10af046; continue; }
  pc = 0x10af043; continue;
 }
 0x10af043 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x10af046; continue;
 }
 0x10af046 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x117ba51u32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x10af058; continue;
 }
 0x10af058 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x10af06d; continue; }
  pc = 0x10af066; continue;
 }
 0x10af066 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x10af06d; continue; }
  pc = 0x10af06c; continue;
 }
 0x10af06c => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10af06d; continue;
 }
 0x10af06d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x10af06d; continue; }
  pc = 0x10af073; continue;
 }
 0x10af073 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10af058; continue; }
  pc = 0x10af078; continue;
 }
 0x10af078 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10af07b; continue;
 }
 0x10af07b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x10af07b; continue; }
  pc = 0x10af081; continue;
 }
 0x10af081 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x117ba48u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x117ba2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba18u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x12d29e0u32; cpu.ecx = v; }
  { let v: u32 = 0x117d140u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x10af0bc; continue; }
  pc = 0x10af0af; continue;
 }
 0x10af0af => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10af0bc; continue; }
  pc = 0x10af0b7; continue;
 }
 0x10af0b7 => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x10af0bc; continue;
 }
 0x10af0bc => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x12d29e0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x117ba20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x117ba24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x10af0e7); cpu.push(0x10af0ec); f_010be62a(cpu); if cpu.returned_to != 0x10af0ec { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x10af0ee); cpu.push(0x10af0f3); f_010be5da(cpu); if cpu.returned_to != 0x10af0f3 { return; }
  pc = 0x10af0f3; continue;
 }
 0x10af0f3 => {
  pc = 0x10af118; continue;
 }
 0x10af0f5 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10aeed8u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x10af109; continue;
 }
 0x10af109 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10af109; continue; }
  pc = 0x10af10e; continue;
 }
 0x10af10e => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10af118; continue;
 }
 0x10af118 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x10af123); cpu.push(0x10af128); f_010be675(cpu); if cpu.returned_to != 0x10af128 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x10af12d); f_010af130(cpu); return;
 }
 0x10c5ea3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x117cc74u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10c5ea6); cpu.push(0x10c5eac); cpu.call(t); if cpu.returned_to != 0x10c5eac { return; }
  let t = cpu.r32(0x117cc78u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10c5eac); cpu.push(0x10c5eb2); cpu.call(t); if cpu.returned_to != 0x10c5eb2 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.poll(); pc = 0x10af0f3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af130); }
 let mut pc: u32 = 0x10af130;
 loop { match pc {
 0x10af130 => {
  { let v = cpu.r16(0x10af139u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10af138); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af139(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af139); } r_010af139(cpu, 0x10af139); }
pub fn f_010af13b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af13b); } r_010af139(cpu, 0x10af13b); }
pub fn f_010af140(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af140); } r_010af139(cpu, 0x10af140); }
fn r_010af139(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10af139 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10af13b; continue;
 }
 0x10af13b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117b1e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10af166; continue; }
  pc = 0x10af14c; continue;
 }
 0x10af140 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117b1e4u32), 0x0u32); }
  if cpu.zf { pc = 0x10af166; continue; }
  pc = 0x10af14c; continue;
 }
 0x10af14c => {
  { let v = 0x10af140u32; cpu.push(v); }
  { let v = 0x10af440u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10af15d); f_010b4750(cpu); if cpu.returned_to != 0x10af15d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117b1e4u32, v); }
  pc = 0x10af166; continue;
 }
 0x10af166 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af170); }
 let mut pc: u32 = 0x10af170;
 loop { match pc {
 0x10af170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x117a5d0u32), 0x0u32); }
  if !cpu.zf { pc = 0x10af2ff; continue; }
  pc = 0x10af184; continue;
 }
 0x10af184 => {
  { let v: u32 = 0x10af430u32; cpu.ebx = v; }
  { let v: u32 = 0x10af420u32; cpu.ecx = v; }
  { let v: u32 = 0x10af410u32; cpu.esi = v; }
  { let v: u32 = 0x10af3f0u32; cpu.edi = v; }
  { let v: u32 = 0x10af328u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x117b1e4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117bb34u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x117bb38u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x117bb3cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x117bb40u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x117bb44u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10af1e6; continue; }
  pc = 0x10af1c4; continue;
 }
 0x10af1c4 => {
  { let v = 0x10af140u32; cpu.push(v); }
  { let v = 0x10af440u32; cpu.push(v); }
  cpu.push(0x10af1d3); f_010b47f0(cpu); if cpu.returned_to != 0x10af1d3 { return; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117b1e4u32, v); }
  cpu.push(0x10af1e6); f_010aecc0(cpu); if cpu.returned_to != 0x10af1e6 { return; }
  pc = 0x10af1e6; continue;
 }
 0x10af1e6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x116ab68u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d1538u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d153cu32, v); }
  cpu.push(0x10af1fe); f_010be6c4(cpu); if cpu.returned_to != 0x10af1fe { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10af21a; continue; }
  pc = 0x10af206; continue;
 }
 0x10af206 => {
  cpu.push(0x10af20b); f_010b5720(cpu); if cpu.returned_to != 0x10af20b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x117a5d0u32, v); }
  pc = 0x10af2ff; continue;
 }
 0x10af21a => {
  { let v: u32 = 0x116ab74u32; cpu.eax = v; }
  cpu.push(0x10af224); f_010be6c4(cpu); if cpu.returned_to != 0x10af224 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10af234; continue; }
  pc = 0x10af22a; continue;
 }
 0x10af22a => {
  { let v: u32 = 0x1u32; cpu.w32(0x12d153cu32, v); }
  pc = 0x10af234; continue;
 }
 0x10af234 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10af23a); f_010b54a2(cpu); if cpu.returned_to != 0x10af23a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1544u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10af25f; continue; }
  pc = 0x10af247; continue;
 }
 0x10af247 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10af251); f_010b5720(cpu); if cpu.returned_to != 0x10af251 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117a5d0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10af25f => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10af266); f_010a69a8(cpu); if cpu.returned_to != 0x10af266 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x10af26d); f_010a6a28(cpu); if cpu.returned_to != 0x10af26d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x12d1570u32; cpu.edi = v; }
  cpu.push(0x10af27b); f_010be863(cpu); if cpu.returned_to != 0x10af27b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10af27f; continue;
 }
 0x10af27f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10af27f; continue; }
  pc = 0x10af289; continue;
 }
 0x10af289 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = 0x116ab84u32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x12d1588u32, v); }
  cpu.push(0x10af29b); f_010aec90(cpu); if cpu.returned_to != 0x10af29b { return; }
  { let v = 0x116abd8u32; cpu.push(v); }
  cpu.push(0x10af2a5); f_010aec90(cpu); if cpu.returned_to != 0x10af2a5 { return; }
  { let v = 0x116ac14u32; cpu.push(v); }
  cpu.push(0x10af2af); f_010aec90(cpu); if cpu.returned_to != 0x10af2af { return; }
  { let v = 0x12d1570u32; cpu.push(v); }
  cpu.push(0x10af2b9); f_010aec90(cpu); if cpu.returned_to != 0x10af2b9 { return; }
  { let v = 0x116ac24u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x10af2c8); f_010aec90(cpu); if cpu.returned_to != 0x10af2c8 { return; }
  cpu.push(0x10af2cd); f_010b5720(cpu); if cpu.returned_to != 0x10af2cd { return; }
  cpu.push(0x10af2d2); f_010b49ee(cpu); if cpu.returned_to != 0x10af2d2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1540u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1538u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1534u32, v); }
  cpu.push(0x10af2e8); f_010ae8c0(cpu); if cpu.returned_to != 0x10af2e8 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x116ac7cu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1534u32, v); }
  cpu.push(0x10af2f9); f_010aec90(cpu); if cpu.returned_to != 0x10af2f9 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x117a5d0u32, v); }
  pc = 0x10af2ff; continue;
 }
 0x10af2ff => {
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
pub fn f_010af310(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af310); } r_010af310(cpu, 0x10af310); }
pub fn f_010af328(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10af328); } r_010af310(cpu, 0x10af328); }
fn r_010af310(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10af310 => {
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x10af318, "Outsd");
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = (cpu.ebx as u8); let b = ((cpu.edx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xc0010af3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10af328; continue;
 }
 0x10af328 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10af3dd; continue; }
  pc = 0x10af344; continue;
 }
 0x10af344 => {
  cpu.poll(); pc = cpu.r32(0x10af310u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x10af34c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af353); f_0109d4ea(cpu); if cpu.returned_to != 0x10af353 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x10af35c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af366); f_0109d4ea(cpu); if cpu.returned_to != 0x10af366 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x10af36f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af37d); f_0109d4ea(cpu); if cpu.returned_to != 0x10af37d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x10af386 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af398); f_0109d4ea(cpu); if cpu.returned_to != 0x10af398 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x10af3a1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af3b7); f_0109d4ea(cpu); if cpu.returned_to != 0x10af3b7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x10af3c0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10af3da); f_0109d4ea(cpu); if cpu.returned_to != 0x10af3da { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x10af3dd; continue;
 }
 0x10af3dd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af3f0); }
 let mut pc: u32 = 0x10af3f0;
 loop { match pc {
 0x10af3f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10af401); f_010a6a48(cpu); if cpu.returned_to != 0x10af401 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af410); }
 let mut pc: u32 = 0x10af410;
 loop { match pc {
 0x10af410 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10af419); f_010be6c4(cpu); if cpu.returned_to != 0x10af419 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af420); }
 let mut pc: u32 = 0x10af420;
 loop { match pc {
 0x10af420 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10af429); f_010b00ed(cpu); if cpu.returned_to != 0x10af429 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af430); }
 let mut pc: u32 = 0x10af430;
 loop { match pc {
 0x10af430 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10af439); f_010b000a(cpu); if cpu.returned_to != 0x10af439 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af440); }
 let mut pc: u32 = 0x10af440;
 loop { match pc {
 0x10af440 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117b1e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10af46e; continue; }
  pc = 0x10af44c; continue;
 }
 0x10af44c => {
  { let v = 0x10af140u32; cpu.push(v); }
  { let v = 0x10af440u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10af460); f_010b47f0(cpu); if cpu.returned_to != 0x10af460 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117b1e4u32, v); }
  cpu.push(0x10af46e); f_010aecc0(cpu); if cpu.returned_to != 0x10af46e { return; }
  pc = 0x10af46e; continue;
 }
 0x10af46e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af472(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af472); }
 let mut pc: u32 = 0x10af472;
 loop { match pc {
 0x10af472 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x117b1e8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af47a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af47a); }
 let mut pc: u32 = 0x10af47a;
 loop { match pc {
 0x10af47a => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x10af483); f_010af48d(cpu); if cpu.returned_to != 0x10af483 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af48d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af48d); }
 let mut pc: u32 = 0x10af48d;
 loop { match pc {
 0x10af48d => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x10af49e; continue; }
  pc = 0x10af491; continue;
 }
 0x10af491 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x117ba2cu32)); }
  if (cpu.cf || cpu.zf) { pc = 0x10af49e; continue; }
  pc = 0x10af49d; continue;
 }
 0x10af49d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10af49e => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x117b1e8u32)); }
  if cpu.zf { f_010af4ab(cpu); return; }
  pc = 0x10af4aa; continue;
 }
 0x10af4aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af4ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af4ab); }
 let mut pc: u32 = 0x10af4ab;
 loop { match pc {
 0x10af4ab => {
  { let v: u32 = 0x117b1eau32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x10af4ba); f_010be875(cpu); if cpu.returned_to != 0x10af4ba { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_010af4c0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af4c0); }
 let mut pc: u32 = 0x10af4c0;
 loop { match pc {
 0x10af4c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d181cu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x12d181cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10af511); f_010a6a48(cpu); if cpu.returned_to != 0x10af511 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.r32(0x12d181cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af524(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af524); }
 let mut pc: u32 = 0x10af524;
 loop { match pc {
 0x10af524 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10af555); f_010a6a48(cpu); if cpu.returned_to != 0x10af555 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af564(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af564); }
 let mut pc: u32 = 0x10af564;
 loop { match pc {
 0x10af564 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x5u16; cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10af5ac); f_010a6a48(cpu); if cpu.returned_to != 0x10af5ac { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af5bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af5bc); }
 let mut pc: u32 = 0x10af5bc;
 loop { match pc {
 0x10af5bc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x6u16; cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10af604); f_010a6a48(cpu); if cpu.returned_to != 0x10af604 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af614); }
 let mut pc: u32 = 0x10af614;
 loop { match pc {
 0x10af614 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10af645); f_010a6a48(cpu); if cpu.returned_to != 0x10af645 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010af650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af650); }
 let mut pc: u32 = 0x10af650;
 loop { match pc {
 0x10af650 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, 0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10af723; continue;
 }
 0x10af66d => {
  { let v: u32 = cpu.r32(0x12d1808u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.ebp = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d1800u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x12d1800u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12d1818u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x117b1feu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x12u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  cpu.push(0x10af717); f_010af564(cpu); if cpu.returned_to != 0x10af717 { return; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1eu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12d183au32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10af723; continue;
 }
 0x10af723 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x117b1fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10af66d; continue; }
  pc = 0x10af733; continue;
 }
 0x10af733 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010af740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af740); }
 let mut pc: u32 = 0x10af740;
 loop { match pc {
 0x10af740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x12d1818u32); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x117b200u32), 0x0u16); }
  if !cpu.zf { pc = 0x10af9da; continue; }
  pc = 0x10af762; continue;
 }
 0x10af762 => {
  { let v: u32 = 0x11400b5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x1140000u32); cpu.ebx = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10af779); f_0109bdf4(cpu); if cpu.returned_to != 0x10af779 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1824u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10af79b; continue; }
  pc = 0x10af782; continue;
 }
 0x10af782 => {
  { let v = 0x116ac8cu32; cpu.push(v); }
  { let v = 0x116acc4u32; cpu.push(v); }
  cpu.push(0x10af791); f_010a4b18(cpu); if cpu.returned_to != 0x10af791 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10af9df; continue;
 }
 0x10af79b => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1140000u32; cpu.eax = v; }
  cpu.push(0x10af7a7); f_0109d010(cpu); if cpu.returned_to != 0x10af7a7 { return; }
  { let v: u32 = 0x1140008u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x1140000u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x60u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1828u32, v); }
  pc = 0x10af7c0; continue;
 }
 0x10af7c0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x12d1834u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1830u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10af7ee; continue; }
  pc = 0x10af7e9; continue;
 }
 0x10af7e9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10af7fd; continue; }
  pc = 0x10af7ee; continue;
 }
 0x10af7ee => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0xcfu8); }
  if cpu.zf { pc = 0x10af7fd; continue; }
  pc = 0x10af7f6; continue;
 }
 0x10af7f6 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x66u8); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x10af7c0; continue; }
  pc = 0x10af7fd; continue;
 }
 0x10af7fd => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x12d183bu32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x67u8); }
  if !cpu.zf { pc = 0x10af82b; continue; }
  pc = 0x10af808; continue;
 }
 0x10af808 => {
  { let v = 0x116acd0u32; cpu.push(v); }
  { let v = 0x116acc4u32; cpu.push(v); }
  cpu.push(0x10af817); f_010a4b18(cpu); if cpu.returned_to != 0x10af817 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10af82b => {
  { let v: u32 = 0x200u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10af845); f_010a6a48(cpu); if cpu.returned_to != 0x10af845 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12d1836u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12d1838u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d1828u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d1828u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x10af89f); f_010a6a48(cpu); if cpu.returned_to != 0x10af89f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x117b1fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x117b200u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x32u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x1eu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x117b1feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  cpu.push(0x10af8fe); f_0109bdf4(cpu); if cpu.returned_to != 0x10af8fe { return; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1820u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10af930; continue; }
  pc = 0x10af90d; continue;
 }
 0x10af90d => {
  { let v = 0x116ad10u32; cpu.push(v); }
  { let v = 0x116acc4u32; cpu.push(v); }
  cpu.push(0x10af91c); f_010a4b18(cpu); if cpu.returned_to != 0x10af91c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10af930 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x117b1fcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d178cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1808u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d180cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1800u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1804u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10af998; continue;
 }
 0x10af981 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x117b1feu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x10af998; continue;
 }
 0x10af998 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x117b1fcu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10af981; continue; }
  pc = 0x10af9a5; continue;
 }
 0x10af9a5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x117b1feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1814u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x12d183bu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d181cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x12d180cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x12d1818u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x12d17f3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x10af9da; continue;
 }
 0x10af9da => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10af9df; continue;
 }
 0x10af9df => {
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
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
pub fn f_010af9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10af9f0); }
 let mut pc: u32 = 0x10af9f0;
 loop { match pc {
 0x10af9f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x117b200u32), 0x0u16); }
  if cpu.zf { pc = 0x10afa5a; continue; }
  pc = 0x10af9ff; continue;
 }
 0x10af9ff => {
  { let v: u32 = cpu.r32(0x12d1820u32); cpu.eax = v; }
  cpu.push(0x10afa09); f_0109c4a8(cpu); if cpu.returned_to != 0x10afa09 { return; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  cpu.push(0x10afa13); f_0109c4a8(cpu); if cpu.returned_to != 0x10afa13 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x12d1836u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x12d1838u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x10afa4c); f_010a6a48(cpu); if cpu.returned_to != 0x10afa4c { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x117b200u32, v); }
  pc = 0x10afa5a; continue;
 }
 0x10afa5a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afa60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afa60); }
 let mut pc: u32 = 0x10afa60;
 loop { match pc {
 0x10afa60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10afa8e); f_010a6a48(cpu); if cpu.returned_to != 0x10afa8e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afa9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afa9c); }
 let mut pc: u32 = 0x10afa9c;
 loop { match pc {
 0x10afa9c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x12d178cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10afafd); f_010a6a48(cpu); if cpu.returned_to != 0x10afafd { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afb18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afb18); }
 let mut pc: u32 = 0x10afb18;
 loop { match pc {
 0x10afb18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x12d178cu32), 0x0u32); }
  if cpu.zf { pc = 0x10afb6c; continue; }
  pc = 0x10afb26; continue;
 }
 0x10afb26 => {
  { let v: u32 = cpu.r32(0x12d1824u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x12d183bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10afb5f); f_010a6a48(cpu); if cpu.returned_to != 0x10afb5f { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d1810u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d178cu32, v); }
  pc = 0x10afb6c; continue;
 }
 0x10afb6c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afb74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afb74); }
 let mut pc: u32 = 0x10afb74;
 loop { match pc {
 0x10afb74 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d180cu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d1814u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  cpu.push(0x10afb98); f_0109d010(cpu); if cpu.returned_to != 0x10afb98 { return; }
  { let v: u32 = cpu.r32(0x12d1814u32); cpu.esi = v; }
  cpu.push(0x10afba3); f_0109f860(cpu); if cpu.returned_to != 0x10afba3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x12d17f3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x12d180cu32); cpu.ebp = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x12d1804u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  pc = 0x10afbfa; continue;
 }
 0x10afbfa => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1bu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10afbfa; continue; }
  pc = 0x10afc05; continue;
 }
 0x10afc05 => {
  { let v: u32 = cpu.r32(0x12d1804u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x12d1814u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x12d1804u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x12d1814u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d180cu32, v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x12u32); cpu.eax = v; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0xfu8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x24u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10afc6f); f_010af5bc(cpu); if cpu.returned_to != 0x10afc6f { return; }
  { let v: u32 = cpu.r32(0x12d180cu32); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afc80); }
 let mut pc: u32 = 0x10afc80;
 loop { match pc {
 0x10afc80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d1818u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10afcd1; continue;
 }
 0x10afcab => {
  { let v: u32 = cpu.r32(0x12d1808u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x10afcca; continue; }
  pc = 0x10afcb8; continue;
 }
 0x10afcb8 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x10afcca; continue; }
  pc = 0x10afcc4; continue;
 }
 0x10afcc4 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10afcca; continue;
 }
 0x10afcca => {
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10afcd1; continue;
 }
 0x10afcd1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x117b1fcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10afcab; continue; }
  pc = 0x10afcde; continue;
 }
 0x10afcde => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10afdec; continue; }
  pc = 0x10afceb; continue;
 }
 0x10afceb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x12u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10afcff; continue; }
  pc = 0x10afcfd; continue;
 }
 0x10afcfd => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10afcff; continue;
 }
 0x10afcff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10afd1d; continue; }
  pc = 0x10afd06; continue;
 }
 0x10afd06 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d1800u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10afd1d; continue;
 }
 0x10afd1d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d1808u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x1eu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x12d1800u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d1800u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x26u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ ((cpu.ebx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0xfu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x2au32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x117b1feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1818u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10afdd0); f_010af564(cpu); if cpu.returned_to != 0x10afdd0 { return; }
  { let v: u32 = cpu.r32(0x12d1818u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x12d183au32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1818u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10afdec => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1818u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_010afe00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afe00); }
 let mut pc: u32 = 0x10afe00;
 loop { match pc {
 0x10afe00 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe30; continue; }
  pc = 0x10afe04; continue;
 }
 0x10afe04 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x10afe06; continue;
 }
 0x10afe06 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x10afe13; continue; }
  pc = 0x10afe0a; continue;
 }
 0x10afe0a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10afe06; continue; }
  pc = 0x10afe13; continue;
 }
 0x10afe13 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x10afe1c); f_010afe37(cpu); if cpu.returned_to != 0x10afe1c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe30; continue; }
  pc = 0x10afe22; continue;
 }
 0x10afe22 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe30; continue; }
  pc = 0x10afe27; continue;
 }
 0x10afe27 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe30; continue; }
  pc = 0x10afe2d; continue;
 }
 0x10afe2d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x10afe30; continue;
 }
 0x10afe30 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afe37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afe37); }
 let mut pc: u32 = 0x10afe37;
 loop { match pc {
 0x10afe37 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afea2; continue; }
  pc = 0x10afe3b; continue;
 }
 0x10afe3b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x10afe47; continue; }
  pc = 0x10afe3f; continue;
 }
 0x10afe3f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10afe3b; continue; }
  pc = 0x10afe47; continue;
 }
 0x10afe47 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe87; continue; }
  pc = 0x10afe4d; continue;
 }
 0x10afe4d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afe79; continue; }
  pc = 0x10afe50; continue;
 }
 0x10afe50 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10afe76; continue; }
  pc = 0x10afe5e; continue;
 }
 0x10afe5e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10afe50; continue; }
  pc = 0x10afe73; continue;
 }
 0x10afe73 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10afe76; continue;
 }
 0x10afe76 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10afe79; continue;
 }
 0x10afe79 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10afe87; continue;
 }
 0x10afe87 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afea2; continue; }
  pc = 0x10afe8d; continue;
 }
 0x10afe8d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afea2; continue; }
  pc = 0x10afe95; continue;
 }
 0x10afe95 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10afea2; continue; }
  pc = 0x10afe9d; continue;
 }
 0x10afe9d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x10afea2; continue;
 }
 0x10afea2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afeb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afeb0); }
 let mut pc: u32 = 0x10afeb0;
 loop { match pc {
 0x10afeb0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x10afec5); f_010afe00(cpu); if cpu.returned_to != 0x10afec5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afec8); }
 let mut pc: u32 = 0x10afec8;
 loop { match pc {
 0x10afec8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x10afede; continue; }
  pc = 0x10afed1; continue;
 }
 0x10afed1 => {
  { cpu.interrupt(0x1a); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10afef3; continue;
 }
 0x10afede => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  if !cpu.zf { pc = 0x10afef3; continue; }
  pc = 0x10afee6; continue;
 }
 0x10afee6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x1a); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10afef3; continue;
 }
 0x10afef3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afef7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10afef7); }
 let mut pc: u32 = 0x10afef7;
 loop { match pc {
 0x10afef7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x116ad40u32; cpu.eax = v; }
  cpu.push(0x10aff0c); f_010be6c4(cpu); if cpu.returned_to != 0x10aff0c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10aff19; continue; }
  pc = 0x10aff12; continue;
 }
 0x10aff12 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10aff50; continue;
 }
 0x10aff19 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10aff24; continue; }
  pc = 0x10aff1f; continue;
 }
 0x10aff1f => {
  { let v: u32 = 0x116ad50u32; cpu.ebx = v; }
  pc = 0x10aff24; continue;
 }
 0x10aff24 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10aff33); f_010be892(cpu); if cpu.returned_to != 0x10aff33 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10aff3f; continue; }
  pc = 0x10aff38; continue;
 }
 0x10aff38 => {
  { let v: u32 = 0x116ad5cu32; cpu.eax = v; }
  pc = 0x10aff44; continue;
 }
 0x10aff3f => {
  { let v: u32 = 0x116ad60u32; cpu.eax = v; }
  pc = 0x10aff44; continue;
 }
 0x10aff44 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x10aff4d); f_010bec1d(cpu); if cpu.returned_to != 0x10aff4d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  pc = 0x10aff50; continue;
 }
 0x10aff50 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aff57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aff57); }
 let mut pc: u32 = 0x10aff57;
 loop { match pc {
 0x10aff57 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
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
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010aff7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10aff7c); }
 let mut pc: u32 = 0x10aff7c;
 loop { match pc {
 0x10aff7c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x117ba4au32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10aff99; continue; }
  pc = 0x10aff8b; continue;
 }
 0x10aff8b => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10aff99; continue; }
  pc = 0x10aff90; continue;
 }
 0x10aff90 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x10affa3; continue;
 }
 0x10aff99 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10affa3; continue;
 }
 0x10affa3 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x21); }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010affae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10affae); }
 let mut pc: u32 = 0x10affae;
 loop { match pc {
 0x10affae => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x10affd1; continue; }
  pc = 0x10affbe; continue;
 }
 0x10affbe => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10affd1; continue; }
  pc = 0x10affc3; continue;
 }
 0x10affc3 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x10affd0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10affd1 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x10affda); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010affdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10affdb); }
 let mut pc: u32 = 0x10affdb;
 loop { match pc {
 0x10affdb => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010affdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10affdd); }
 let mut pc: u32 = 0x10affdd;
 loop { match pc {
 0x10affdd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010affe7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10affe7); }
 let mut pc: u32 = 0x10affe7;
 loop { match pc {
 0x10affe7 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010affef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10affef); }
 let mut pc: u32 = 0x10affef;
 loop { match pc {
 0x10affef => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010afff1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10afff1); } r_010afff1(cpu, 0x10afff1); }
pub fn f_010b0000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b0000); } r_010afff1(cpu, 0x10b0000); }
fn r_010afff1(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10afff1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x10b0003); f_010bec34(cpu); if cpu.returned_to != 0x10b0003 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0000 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b000a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b000a); }
 let mut pc: u32 = 0x10b000a;
 loop { match pc {
 0x10b000a => {
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
  if cpu.zf { pc = 0x10b0023; continue; }
  pc = 0x10b001e; continue;
 }
 0x10b001e => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b002a; continue; }
  pc = 0x10b0023; continue;
 }
 0x10b0023 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b00de; continue;
 }
 0x10b002a => {
  { let v: u32 = 0x117b204u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x10b003d; continue;
 }
 0x10b003d => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x10b0049; continue; }
  pc = 0x10b0044; continue;
 }
 0x10b0044 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x10b0049; continue;
 }
 0x10b0049 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x117b20cu32)); }
  if (cpu.cf || cpu.zf) { pc = 0x10b0069; continue; }
  pc = 0x10b0051; continue;
 }
 0x10b0051 => {
  { let v: u32 = cpu.r32(0x117b208u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b00a1; continue; }
  pc = 0x10b005b; continue;
 }
 0x10b005b => {
  { let v: u32 = cpu.ecx; cpu.w32(0x117b20cu32, v); }
  { let v: u32 = cpu.r32(0x117b204u32); cpu.ecx = v; }
  pc = 0x10b00a1; continue;
 }
 0x10b0069 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x10b005b; continue;
 }
 0x10b006d => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117b208u32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b0086); f_010bf012(cpu); if cpu.returned_to != 0x10b0086 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b00d4; continue; }
  pc = 0x10b008c; continue;
 }
 0x10b008c => {
  { let v: u32 = cpu.r32(0x117b20cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x10b009e; continue; }
  pc = 0x10b0099; continue;
 }
 0x10b0099 => {
  { let v: u32 = cpu.eax; cpu.w32(0x117b20cu32, v); }
  pc = 0x10b009e; continue;
 }
 0x10b009e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x10b00a1; continue;
 }
 0x10b00a1 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b006d; continue; }
  pc = 0x10b00a5; continue;
 }
 0x10b00a5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x10b00b6; continue; }
  pc = 0x10b00ab; continue;
 }
 0x10b00ab => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b00b2); f_010bf5b6(cpu); if cpu.returned_to != 0x10b00b2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b00cb; continue; }
  pc = 0x10b00b6; continue;
 }
 0x10b00b6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b00bd); f_010bf77f(cpu); if cpu.returned_to != 0x10b00bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b00d4; continue; }
  pc = 0x10b00c1; continue;
 }
 0x10b00c1 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b003d; continue;
 }
 0x10b00cb => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b003d; continue;
 }
 0x10b00d4 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x12d242cu32, v); }
  pc = 0x10b00de; continue;
 }
 0x10b00de => {
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
pub fn f_010b00ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b00ed); } r_010b00ed(cpu, 0x10b00ed); }
pub fn f_010b0111(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b0111); } r_010b00ed(cpu, 0x10b0111); }
fn r_010b00ed(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b00ed => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b0169; continue; }
  pc = 0x10b00fc; continue;
 }
 0x10b00fc => {
  { let v: u32 = cpu.r32(0x12d183cu32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b010f; continue; }
  pc = 0x10b0106; continue;
 }
 0x10b0106 => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b010f; continue; }
  pc = 0x10b010a; continue;
 }
 0x10b010a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x10b0129; continue; }
  pc = 0x10b010f; continue;
 }
 0x10b010f => {
  { let v: u32 = cpu.r32(0x117b204u32); cpu.ecx = v; }
  pc = 0x10b0122; continue;
 }
 0x10b0111 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xb2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.edi = v; }
  cpu.unimplemented(0x10b0118, "Int1");
  if (!cpu.cf && !cpu.zf) { pc = 0x10b011f; continue; }
  pc = 0x10b011b; continue;
 }
 0x10b0117 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b011f; continue; }
  pc = 0x10b011b; continue;
 }
 0x10b011b => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x10b0129; continue; }
  pc = 0x10b011f; continue;
 }
 0x10b011f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x10b0122; continue;
 }
 0x10b0122 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0117; continue; }
  pc = 0x10b0129; continue;
 }
 0x10b0129 => {
  { let v: u32 = 0x117b204u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b013f); f_010bf0ba(cpu); if cpu.returned_to != 0x10b013f { return; }
  { let v: u32 = cpu.r32(0x117b208u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d183cu32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x10b0161; continue; }
  pc = 0x10b014f; continue;
 }
 0x10b014f => {
  { let v: u32 = cpu.r32(0x117b20cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x10b0161; continue; }
  pc = 0x10b015b; continue;
 }
 0x10b015b => {
  { let v: u32 = cpu.edx; cpu.w32(0x117b20cu32, v); }
  pc = 0x10b0161; continue;
 }
 0x10b0161 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x12d242cu32, v); }
  pc = 0x10b0169; continue;
 }
 0x10b0169 => {
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
pub fn f_010b016f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b016f); }
 let mut pc: u32 = 0x10b016f;
 loop { match pc {
 0x10b016f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x10b018a; continue; }
  pc = 0x10b017e; continue;
 }
 0x10b017e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b018a; continue;
 }
 0x10b018a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b018e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b018e); }
 let mut pc: u32 = 0x10b018e;
 loop { match pc {
 0x10b018e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x10b016fu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10b01ae); f_010b04be(cpu); if cpu.returned_to != 0x10b01ae { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b01b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b01b7); }
 let mut pc: u32 = 0x10b01b7;
 loop { match pc {
 0x10b01b7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10b01d7); f_010b018e(cpu); if cpu.returned_to != 0x10b01d7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
